// ducker 第二个二进制：MCP server（stdio）。
// 供 dsh 等 AI 助手通过 Model Context Protocol 读写 ducker 的任务与动作。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    app_lib::mcp::run();
}
