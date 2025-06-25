export const allData = {
  today: [
    {
      id: "1", name: '完成项目提案', value:5, completed: false, created_at: new Date(), due_to: new Date(), children: [
        { id: "6", name: '功能开发', value:3,completed: false, created_at: new Date(), due_to: new Date() },
        { id: "7", name: '文档编写',value:1, completed: false, created_at: new Date(), due_to: new Date() },
      ]
    },
    { id: "2", name: '团队会议', value:1,completed: true, created_at: new Date(), due_to: new Date() },
    { id: "3", name: '代码审查',value:1, completed: false, created_at: new Date(), due_to: new Date() },
    { id: "15", name: '月度报告',value:1, completed: false, created_at: new Date(), due_to: new Date() },
    { id: "24", name: '预算规划',value:1, completed: false, created_at: new Date(), due_to: new Date() },
    { id: "37", name: '团队建设活动',value:1, completed: true, created_at: new Date(), due_to: new Date() },
    { id: "42", name: '月度报告',value:1, completed: false, created_at: new Date(), due_to: new Date() },
    { id: "25", name: '预算规划',value:1, completed: false, created_at: new Date(), due_to: new Date() },
    { id: "36", name: '团队建设活动',value:1, completed: true, created_at: new Date(), due_to: new Date() }
  ],
  weekly: {
    monday: [
      { id: "1", name: '周计划制定',value:1, completed: true, created_at: new Date(), due_to: new Date() },
      { id: "2", name: '客户会议', value:1,completed: true, created_at: new Date(), due_to: new Date() },
    ],
    tuesday: [
      { id: "3", name: '功能开发',value:1, completed: false, created_at: new Date(), due_to: new Date() },
      { id: "4", name: '文档编写',value:1, completed: false, created_at: new Date(), due_to: new Date() },
    ],
    wednesday: [
      { id: "5", name: '测试用例设计',value:1, completed: false, created_at: new Date(), due_to: new Date() },
    ],
    thursday: [],
    friday: [
      { id: "6", name: '项目演示准备',value:1, completed: false, created_at: new Date(), due_to: new Date() },
    ],
    saturday: [],
    sunday: [],
  },
  monthly: [
    { id: "1", name: '月度报告',value:1, completed: false, created_at: new Date(), due_to: new Date() },
    { id: "2", name: '预算规划', value:1,completed: false, created_at: new Date(), due_to: new Date() },
    { id: "3", name: '团队建设活动',value:1, completed: true, created_at: new Date(), due_to: new Date() },
  ]
}