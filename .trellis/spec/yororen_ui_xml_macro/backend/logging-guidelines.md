# 日志规范

> 本仓库几乎不使用结构化日志库；以文档注释 + 有限的 `eprintln!` 诊断为主。

---

## 概览

- **库代码默认静默**：正常路径不打日志。
- **缺失翻译键**：`yororen-ui-core` i18n 对同一 key 只 `eprintln!` 一次（`OnceLock` + `HashSet` 去重），格式：
  ```text
  [yororen-ui-core::i18n] missing translation for key: {key}
  ```
- **CLI / 工具二进制**（如 `gen-schema`）：用 `eprintln!` 输出 warning，不引入 `log` / `tracing` 依赖。
- **XML 宏错误**：编译期 `compile_error!` / diagnostic，不是运行时日志。

---

## 级别约定（实际用法）

| 场景 | 做法 |
|------|------|
| 开发期缺失 i18n key | 一次性 `eprintln!` warning |
| 代码生成 / schema 工具异常输入 | `eprintln!("warning: ...")` 后尽量继续或失败退出 |
| 不可恢复的安装期不变量（bundled JSON 必须合法） | `.expect("... is valid")` |
| 用户可见运行时错误 | 优先 UI 状态 / `NotificationCenter`，不要刷 stderr |

---

## 应该记录

- 缺失的 translation key（已实现去重）。
- schema 生成器遇到未知 control-flow 变体等工具侧警告。

---

## 不应记录

- 密钥、token、用户私密输入。
- 每个 frame / 每次 hover 的高频 UI 事件。
- 已用类型系统 / `Result` 表达清楚的预期失败（优先返回错误，不要既返回又刷屏）。

---

## 新增日志时的规则

1. 先问：是否可用类型 / `Result` / UI 反馈替代？
2. 若必须 stderr：带 crate 前缀，并考虑去重。
3. 不要为了“完善可观测性”给 UI 库强行引入 `tracing` 全家桶，除非全 workspace 明确采用。
