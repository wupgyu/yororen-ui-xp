# 数据持久化规范

> 本项目是 **gpui UI 组件库**，没有传统数据库 / ORM / 迁移。

---

## 概览

- **不使用** SQL、ORM、迁移工具。
- 持久化相关能力仅出现在少数状态机里，且范围很窄。

---

## 实际存在的“持久化”

### NotificationCenter（`yororen-ui-core`）

- 仅 `sticky = true` 的通知可被持久化。
- **回调不持久化**。
- sticky 通知的 `payload` 可持久化。
- 队列默认 `max_queue_len = 5`。

### Theme / 翻译资源

- Theme：JSON 文件（`themes/*.json`），运行时 `Theme::from_json`。
- Locale：各 `yororen-ui-locale-*` crate 的 `translations/*.json`，通过 `include_str!` 或加载器装入。

### 应用侧状态

- 使用 `gpui::Entity<T>` + `cx.set_global` / `cx.global`。
- 复合组件状态（Modal / Popover / Select 等）由 headless 状态机持有，不落盘。

---

## 规则

| 做 | 不做 |
|----|------|
| 状态放 `Entity` / `Global` | 在 UI 库内引入数据库依赖 |
| 配置用 JSON theme / translation | 把业务数据写进组件库 crate |
| sticky 通知才考虑持久化 | 持久化闭包 / 回调 |

---

## 常见错误

- 把 NotificationCenter 当成通用 app 数据库。
- 假设 theme JSON 有编译期 schema 校验（实际是开放路径 + renderer 默认回退）。
