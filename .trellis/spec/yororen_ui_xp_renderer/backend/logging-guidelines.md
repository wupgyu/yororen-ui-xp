# 日志规范 — xp_renderer

与 brutalism / default renderer 一致：**无运行时日志**。

- renderer compose 路径不写 `log::info!` / `println!` / `dbg!`。
- 主题解析失败直接 `expect`（编程错误），不写 warn。
- 诊断信息一律走测试与 demo 目检，不走日志。
