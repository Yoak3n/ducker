use anyhow::{Context, Result};
use chrono::{Local, SecondsFormat};
use delay_timer::prelude::{DelayTimer, DelayTimerBuilder, TaskBuilder};
use parking_lot::RwLock;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Arc,
    },
};

use crate::{
    core::handle::Handle,
    logging, logging_error,
    service::{execute, hub::Hub},
    singleton,
    utils::logging::Type,
};

type TaskID = u64;
// const AUTO_REFRESH_ID: &str = "auto_refresh_task";

#[derive(Debug, Clone)]
pub struct TimerTask {
    pub task_id: TaskID,
    pub interval_seconds: i64,
    #[allow(unused)]
    // 不知道这个字段有什么用
    pub last_period: i64,
}

pub struct Timer {
    /// cron manager
    pub delay_timer: Arc<RwLock<DelayTimer>>,

    /// save the current state - using RwLock for better read concurrency
    pub timer_map: Arc<RwLock<HashMap<String, TimerTask>>>,

    /// increment id - atomic counter for better performance
    pub timer_count: AtomicU64,

    /// Flag to mark if timer is initialized - atomic for better performance
    pub initialized: AtomicBool,
}

// Use singleton macro
singleton!(Timer, TIMER_INSTANCE);

impl Timer {
    fn new() -> Self {
        Timer {
            delay_timer: Arc::new(RwLock::new(DelayTimerBuilder::default().build())),
            timer_map: Arc::new(RwLock::new(HashMap::new())),
            timer_count: AtomicU64::new(1),
            initialized: AtomicBool::new(false),
        }
    }

    /// Initialize timer with better error handling and atomic operations
    pub fn init(&self) -> Result<()> {
        // Use compare_exchange for thread-safe initialization check
        if self
            .initialized
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            logging!(debug, Type::Timer, "Timer already initialized, skipping...");
            return Ok(());
        }

        logging!(info, Type::Timer, true, "Initializing timer...");

        // Initialize timer tasks
        if let Err(e) = self.refresh() {
            // Reset initialization flag on error
            self.initialized.store(false, Ordering::SeqCst);
            logging_error!(Type::Timer, "Failed to initialize timer: {}", e);
            return Err(e);
        }

        // 定时每一分钟刷新待办动作
        let auto_refrsh_task_id = self.timer_count.fetch_add(1, Ordering::Relaxed);
        let auto_refresh_task = TaskBuilder::default()
            .set_task_id(auto_refrsh_task_id)
            .set_maximum_parallel_runnable_num(1)
            .set_frequency_repeated_by_minutes(1)
            .spawn_async_routine(move || async move {
                Hub::global().refresh().await;
                let _ = Self::global().refresh();
            })
            .context("failed to create auto_refresh_task")?;
        let delay_timer = self.delay_timer.write();
        delay_timer.add_task(auto_refresh_task)?;

        logging!(info, Type::Timer, "Timer initialization completed");
        Ok(())
    }

    /// Refresh timer tasks with better error handling
    pub fn refresh(&self) -> Result<()> {
        // Generate diff outside of lock to minimize lock contention
        // Hub::global().refresh();
        let diff_map = self.gen_diff();
        logging!(info, Type::Timer,true, "Timer refresh at {}",Local::now().to_rfc3339_opts(SecondsFormat::Secs, true));
        if diff_map.is_empty() {
            logging!(debug, Type::Timer, "No timer changes needed");
            return Ok(());
        }

        // Apply changes while holding locks
        let mut timer_map = self.timer_map.write();
        let mut delay_timer = self.delay_timer.write();

        for (uid, diff) in diff_map {
            match diff {
                DiffFlag::Del(tid) => {
                    timer_map.remove(&uid);
                    if let Err(e) = delay_timer.remove_task(tid) {
                        logging!(
                            warn,
                            Type::Timer,
                            true,
                            "Failed to remove task {} for uid {}: {}",
                            tid,
                            uid,
                            e
                        );
                    } else {
                        logging!(debug, Type::Timer, true, "Removed task {} for uid {}", tid, uid);
                    }
                }
                DiffFlag::Add(tid, interval) => {
                    let now = Local::now().timestamp();
                    let task = TimerTask {
                        task_id: tid,
                        interval_seconds: interval,
                        last_period: now,
                    };

                    timer_map.insert(uid.clone(), task);
                    if let Err(e) =
                        self.add_task(&mut delay_timer, uid.clone(), tid, interval, now + interval)
                    {
                        logging_error!(Type::Timer, "Failed to add task for uid {}: {}", uid, e);
                        timer_map.remove(&uid); // Rollback on failure
                    } else {
                        logging!(
                            debug,
                            Type::Timer,
                            true,
                            "Added task {} for uid {} at {}",
                            tid,
                            uid,
                            now + interval
                        );
                    }
                }
                DiffFlag::Mod(tid, interval) => {
                    // Remove old task first
                    if let Err(e) = delay_timer.remove_task(tid) {
                        logging!(
                            warn,
                            Type::Timer,
                            true,
                            "Failed to remove old task {} for uid {}: {}",
                            tid,
                            uid,
                            e
                        );
                    }
                    let now = Local::now().timestamp();
                    // Then add the new one
                    let task = TimerTask {
                        task_id: tid,
                        interval_seconds: interval,
                        last_period: now,
                    };

                    timer_map.insert(uid.clone(), task);
                    // 这样时间戳加间隔可靠吗？
                    if let Err(e) =
                        self.add_task(&mut delay_timer, uid.clone(), tid, interval, now + interval)
                    {
                        logging_error!(Type::Timer, "Failed to update task for uid {}: {}", uid, e);
                        timer_map.remove(&uid); // Rollback on failure
                    } else {
                        logging!(debug, Type::Timer, "Updated task {} for uid {}", tid, uid);
                    }
                }
            }
        }

        Ok(())
    }

    /// Generate map of profile UIDs to update intervals
    fn gen_map(&self) -> HashMap<String, i64> {
        let mut new_map = HashMap::new();
        let cur_time = Local::now().timestamp();
        if let Some(items) = Hub::global().latest_schedule() {
            for (timestamp, tasks) in items.iter() {
                let interval = timestamp - cur_time;
                for task in tasks.iter() {
                    let id = task.id.clone();
                    if interval > 0 {
                        logging!(
                            debug,
                            Type::Timer,
                            "找到定时更新配置: id={}, interval={}seconds",
                            id,
                            interval
                        );
                        // 新的配置的时间间隔大于当前的时间间隔，说明不需要更新
                        if new_map.contains_key(&id) && new_map.get(&id).unwrap() < &interval {
                            logging!(
                                debug,
                                Type::Timer,
                                "定时更新配置已存在: id={}, interval={}seconds",
                                task.id,
                                interval
                            );
                            continue;
                        }
                        // TODO 这样就不支持在同一个时间戳执行同一个action
                        new_map.insert(id, interval);
                    }
                }
            }
        }
        logging!(
            debug,
            Type::Timer,
            "生成的定时更新配置数量: {}",
            new_map.len()
        );
        new_map
    }

    // Generate differences between current and new timer configuration
    fn gen_diff(&self) -> HashMap<String, DiffFlag> {
        let mut diff_map = HashMap::new();
        let new_map = self.gen_map();

        // Read lock for comparing current state
        let timer_map = self.timer_map.read();
        logging!(
            debug,
            Type::Timer,
            "当前 timer_map 大小: {}",
            timer_map.len()
        );

        // Find tasks to modify or delete
        for (uid, timer_task) in timer_map.iter() {
            match new_map.get(uid) {
                // 由于delay_timer内部会更新task的interval_seconds，所以这里应该会不断发送ModFlag
                Some(&interval) if interval != timer_task.interval_seconds => {
                    // Task exists but interval changed
                    logging!(
                        debug,
                        Type::Timer,
                        "定时任务间隔变更: uid={}, 旧={}, 新={}",
                        uid,
                        timer_task.interval_seconds,
                        interval
                    );
                    diff_map.insert(uid.clone(), DiffFlag::Mod(timer_task.task_id, interval));
                }
                None => {
                    // Task no longer needed
                    logging!(debug, Type::Timer, true, "定时任务已删除: uid={}", uid);
                    diff_map.insert(uid.clone(), DiffFlag::Del(timer_task.task_id));
                }
                _ => {
                    // Task exists with same interval, no change needed
                    logging!(debug, Type::Timer, true, "定时任务保持不变: uid={}", uid);
                }
            }
        }

        // Find new tasks to add
        // 我去，你这task_id竟然是自增的吗
        let mut next_id = self.timer_count.load(Ordering::Relaxed);
        let original_id = next_id;

        for (uid, &interval) in new_map.iter() {
            if !timer_map.contains_key(uid) {
                logging!(
                    debug,
                    Type::Timer,
                    true,
                    "新增定时任务: uid={}, interval={}sec",
                    uid,
                    interval
                );
                diff_map.insert(uid.clone(), DiffFlag::Add(next_id, interval));
                next_id += 1;
            }
        }

        // Update counter only if we added new tasks
        if next_id > original_id {
            self.timer_count.store(next_id, Ordering::Relaxed);
        }

        diff_map
    }

    /// Add a timer task with better error handling
    fn add_task(
        &self,
        delay_timer: &mut DelayTimer,
        uid: String,
        tid: TaskID,
        seconds: i64,
        timestamp: i64,
    ) -> Result<()> {
        logging!(
            info,
            Type::Timer,
            "Adding task: uid={}, id={}, interval={}sec",
            uid,
            tid,
            seconds
        );

        // Create a task with reasonable retries and backoff
        let task = TaskBuilder::default()
            .set_task_id(tid)
            .set_maximum_parallel_runnable_num(1)
            .set_frequency_once_by_seconds(seconds as u64)
            .spawn_async_routine(move || {
                let uid = uid.clone();
                async move {
                    Self::async_task(uid, timestamp).await;
                }
            })
            .context("failed to create timer task")?;

        delay_timer
            .add_task(task)
            .context("failed to add timer task")?;
        Ok(())
    }

    // Async task with better error handling and logging
    async fn async_task(id: String, timestamp: i64) {
        let task_start = std::time::Instant::now();
        // let task_start = Local::now();

        logging!(
            info,
            Type::Timer,
            "Running timer task for action: {}, timestamp: {}",
            id,
            timestamp
        );
        match tokio::time::timeout(std::time::Duration::from_secs(40), async {
            // feat::update_profile(uid.clone(), None, Some(is_current)).await
            execute::execute_tasks(&id, timestamp).await
        })
        .await
        {
            Ok(result) => match result {
                Ok(_) => {
                    let duration = task_start.elapsed().as_millis();
                    logging!(
                        info,
                        Type::Timer,
                        "Timer task completed successfully for id: {} (took {}ms)",
                        id,
                        duration
                    );
                }
                Err(e) => {
                    logging_error!(Type::Timer, "Failed to update profile uid {}: {}", id, e);
                    Handle::notice_message("Error", format!("定时任务执行失败:{}", e));
                }
            },
            Err(_) => {
                logging_error!(Type::Timer, "Timer task timed out for uid: {}", id);
            }
        }
    }
}

#[derive(Debug)]
enum DiffFlag {
    Del(TaskID),
    Add(TaskID, i64),
    Mod(TaskID, i64),
}
