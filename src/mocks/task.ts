export const  allData = {
    today: [
      { id: 1, title: '完成项目提案', completed: false, create_at: new Date(), due_at: new Date() },
      { id: 2, title: '团队会议', completed: true ,create_at: new Date(), due_at: new Date()},
      { id: 3, title: '代码审查', completed: false ,create_at: new Date(), due_at: new Date()},
    ],
    weekly: {
      monday: [
        { id: 1, title: '周计划制定', completed: true,create_at: new Date(), due_at: new Date() },
        { id: 2, title: '客户会议', completed: true,create_at: new Date(), due_at: new Date() },
      ],
      tuesday: [
        { id: 3, title: '功能开发', completed: false,create_at: new Date(), due_at: new Date() },
        { id: 4, title: '文档编写', completed: false,create_at: new Date(), due_at: new Date() },
      ],
      wednesday: [
        { id: 5, title: '测试用例设计', completed: false,create_at: new Date(), due_at: new Date() },
      ],
      thursday: [],
      friday: [
        { id: 6, title: '项目演示准备', completed: false,create_at: new Date(), due_at: new Date() },
      ],
      saturday: [],
      sunday: [],
    },
    monthly: [
      { id: 1, title: '月度报告', completed: false,create_at: new Date(), due_at: new Date() },
      { id: 2, title: '预算规划', completed: false,create_at: new Date(), due_at: new Date() },
      { id: 3, title: '团队建设活动', completed: true,create_at: new Date(), due_at: new Date() },
    ]
  }