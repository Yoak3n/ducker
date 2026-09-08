//! 定时任务调度核心。
//!
//! 2026-09 重构背景：
//! 旧实现把每个「到点执行」注册成 delay_timer 0.11.6 时间轮的一次性任务，并每分钟
//! 重注册刷新。该时间轮在注册时按 `(剩余秒数 + 秒针 + 1)` 计算槽位与圈数：当剩余秒数
//! 位于本圈末尾、且与当前秒针位置叠加发生进位时，任务会整整多等一圈 —— 恰好 3600 秒，
//! 表现为定时任务固定晚一小时触发（例：2026-09-08 15:05:09 注册的 22:30 任务最终在
//! 23:30:01 触发：剩余 26691s、秒针 ≈3360，26691+1+3360 进位 → 实际 30292s 后才触发）。
//! 此外旧实现每分钟的刷新任务带 maximum_parallel_runnable_num(1)，一旦某次执行
//! panic/卡死，finish 事件不再发出、并行计数永久停在 1，后续刷新被静默跳过。
//!
//! 因此改为：一个独立的 1 秒 tokio tick —— 每分钟从数据库重建「调度表」（due_to →
//! 任务），tick 直接把计划时间与当前时间比对触发；每次 tick 跑在隔离的子任务里，
//! 个别异常只丢一拍，不会终止整个调度循环。delay_timer（时间轮）已从本模块移除。

use anyhow::Result;
use chrono::Local;
use parking_lot::RwLock;
use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicBool, AtomicI64, Ordering},
        Arc,
    },
    time::Duration,
};

use crate::{
    core::handle::Handle,
    logging, logging_error,
    service::{execute, hub::Hub},
    singleton,
    utils::{date::to_datetime_str, logging::Type},
};

/// 调度表（数据库 → Hub）的重建周期
const REFRESH_INTERVAL_SECS: i64 = 60;
/// 已触发记录保留时间：覆盖「触发后到调度表把该实例清掉」之间可能的最长间隔
const FIRED_RETENTION_SECS: i64 = 600;
/// 到点判定 tick 周期
const TICK_PERIOD: Duration = Duration::from_secs(1);

pub struct Timer {
    /// 当前跟踪的定时任务：uid -> 计划时间戳（调度表刷新时同步，用于变更日志）
    tracked: Arc<RwLock<HashMap<String, i64>>>,

    /// 已触发过的 (uid, 计划时间戳)：避免 tick 重复触发同一个实例
    fired: Arc<RwLock<HashSet<(String, i64)>>>,

    /// 上次重建调度表的 unix 秒
    last_refresh_ts: AtomicI64,

    /// Flag to mark if timer is initialized
    pub initialized: AtomicBool,
}

singleton!(Timer, TIMER_INSTANCE);

impl Timer {
    fn new() -> Self {
        Timer {
            tracked: Arc::new(RwLock::new(HashMap::new())),
            fired: Arc::new(RwLock::new(HashSet::new())),
            last_refresh_ts: AtomicI64::new(0),
            initialized: AtomicBool::new(false),
        }
    }

    /// 启动定时调度：一个 1 秒 tokio tick（跑在 Tauri 的 tokio 运行时上，与 UI 线程解耦）。
    pub fn init(&self) -> Result<()> {
        // 防止重复初始化
        if self
            .initialized
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            logging!(debug, Type::Timer, "Timer already initialized, skipping...");
            return Ok(());
        }

        logging!(info, Type::Timer, true, "Initializing timer...");

        tauri::async_runtime::spawn(async move {
            let mut ticker = tokio::time::interval(TICK_PERIOD);
            // tick 处理滞后时不必追赶，顺延即可
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            loop {
                ticker.tick().await;
                // 每个 tick 放进独立子任务：单次异常（含数据库/调度表构建错误）
                // 只丢掉这一拍，调度循环本身继续跑。
                let join = tauri::async_runtime::spawn(async move {
                    Timer::global().tick_once().await;
                });
                if let Err(e) = join.await {
                    logging!(error, Type::Timer, true, "定时调度 tick 异常: {:?}", e);
                }
            }
        });

        logging!(info, Type::Timer, "Timer initialization completed");
        Ok(())
    }

    /// 一拍 tick：必要时重建调度表，然后触发所有已到点的任务。
    async fn tick_once(&self) {
        let now = Local::now().timestamp();

        // 周期性重建调度表（默认每分钟一次；CAS 保证并发下只刷一次）
        let last = self.last_refresh_ts.load(Ordering::Relaxed);
        if now - last >= REFRESH_INTERVAL_SECS
            && self
                .last_refresh_ts
                .compare_exchange(last, now, Ordering::SeqCst, Ordering::Relaxed)
                .is_ok()
        {
            Hub::global().refresh().await;
            self.sync_tracked();
            self.prune_fired(now);
        }

        // 到点触发：调度表里计划时间已到的任务
        let Some(schedule) = Hub::global().latest_schedule() else {
            return;
        };
        for (ts, tasks) in schedule.iter() {
            if *ts > now {
                continue;
            }
            for task in tasks.iter() {
                let uid = task.id.clone();
                // 已触发过（含正在执行）的实例不再触发
                if self.fired.write().insert((uid.clone(), *ts)) {
                    self.fire(&uid, *ts);
                }
            }
        }
    }

    /// 触发一次定时任务（记录日志 + 异步执行，不阻塞 tick）
    fn fire(&self, uid: &str, ts: i64) {
        logging!(
            info,
            Type::Timer,
            true,
            "定时任务到点触发: uid={}, 计划时间={}, 当前时间={}",
            uid,
            to_datetime_str(ts),
            Local::now().format("%Y-%m-%d %H:%M:%S")
        );

        let id = uid.to_string();
        tauri::async_runtime::spawn(async move {
            let task_start = std::time::Instant::now();
            match execute::execute_tasks(&id, ts).await {
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
                    logging_error!(Type::Timer, "定时任务执行失败: id={}, err={}", id, e);
                    Handle::notice_message("Error", format!("定时任务执行失败:{}", e));
                }
            }
        });
    }

    /// 把 tracked（用于变更日志）与最新调度表对齐：新增/变更打「定时任务注册」，
    /// 已消失的打「定时任务已移除」。
    fn sync_tracked(&self) {
        let mut expect: HashMap<String, i64> = HashMap::new();
        if let Some(schedule) = Hub::global().latest_schedule() {
            for (ts, tasks) in schedule {
                for task in tasks {
                    expect
                        .entry(task.id.clone())
                        .and_modify(|e| {
                            if ts < *e {
                                *e = ts;
                            }
                        })
                        .or_insert(ts);
                }
            }
        }

        let mut tracked = self.tracked.write();

        let removed: Vec<String> = tracked
            .iter()
            .filter(|(uid, _)| !expect.contains_key(*uid))
            .map(|(uid, _)| (*uid).clone())
            .collect();
        for uid in removed {
            tracked.remove(&uid);
            logging!(info, Type::Timer, true, "定时任务已移除: uid={}", uid);
        }

        for (uid, ts) in expect {
            let unchanged = tracked
                .get(&uid)
                .is_some_and(|old| *old == ts);
            if unchanged {
                continue;
            }
            tracked.insert(uid.clone(), ts);
            logging!(
                info,
                Type::Timer,
                true,
                "定时任务注册: uid={}, 到点时间={}",
                uid,
                to_datetime_str(ts)
            );
        }
    }

    /// 清理过期的已触发记录
    fn prune_fired(&self, now: i64) {
        let cutoff = now - FIRED_RETENTION_SECS;
        self.fired.write().retain(|(_, ts)| *ts >= cutoff);
    }
}
