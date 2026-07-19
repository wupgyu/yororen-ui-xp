# yororen_ui（meta-crate）后端规范

> 门面 crate：再导出 core + default renderer + 内置 locales，并可选 brutalism / xml。

---

## 概览

应用通常只依赖 `yororen_ui` 一个包。本目录规范描述 **meta-crate 本身** 的职责边界与再导出约定。

---

## 规范索引

| 规范 | 说明 | 状态 |
|------|------|------|
| [目录结构](./directory-structure.md) | 模块组织与再导出布局 | 已补充 |
| [数据/资源](./database-guidelines.md) | 无 DB；主题/语言资源入口 | 已补充 |
| [错误处理](./error-handling.md) | feature 关闭时的编译期错误 | 已补充 |
| [质量规范](./quality-guidelines.md) | 再导出与 feature 边界 | 已补充 |
| [日志规范](./logging-guidelines.md) | 门面层日志策略 | 已补充 |

---

**语言要求**：所有文档默认使用**中文**编写。
