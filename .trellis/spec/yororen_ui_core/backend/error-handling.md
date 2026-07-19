# 错误处理

> UI 组件库的错误处理以类型、回退默认值与编译期诊断为主。

---

## 概览


**本包特点**：定义 Theme / Registry / i18n 运行时行为。渲染热路径避免 `Result` 爆炸；缺失配置用默认。


跨 crate 共性：

1. **Theme 路径缺失** → `Option` + renderer 侧 `unwrap_or` / `unwrap_or_default`，不 panic。
2. **安装期不变量**（内置 JSON 主题/翻译必须合法）→ `.expect("... is valid")`。
3. **可恢复解析**（`Theme::from_json`）→ `Result<Theme, serde_json::Error>`。
4. **i18n 缺 key** → 返回 key 本身（或约定回退）+ 一次性 `eprintln!`。
5. **XML** → 结构化 `XmlError`，宏入口转为编译错误。

---

## 错误类型（按层）

| 层 | 类型 / 策略 |
|----|-------------|
| Theme | `serde_json::Error`；路径 API 返回 `Option` |
| Renderer registry | 缺失 slot：`validate() -> Result<(), Vec<&'static str>>`；安装路径可能 panic 列出缺失项 |
| i18n | 缺 key 不返回 `Result`，避免渲染路径被错误处理淹没 |
| XML | `XmlError` + `XmlErrorKind`（ParseError / UnknownTag / UnknownAttribute / InvalidExpression / Unsupported） |

---

## 模式

```rust
// theme：可恢复
let theme = Theme::from_json(json)?;

// renderer：路径缺失用默认
let radius = theme
    .get_number("tokens.control.button.radius")
    .or_else(|| theme.get_number("tokens.radii.md"))
    .unwrap_or(6.0);

// 安装：内置资源视为不变量
Theme::from_json(SYSTEM_LIGHT).expect("system-light.json is valid");
```

---

## 常见错误

- 在 `.render(cx)` 热路径上到处 `?` 传播 `Result`，迫使每个组件处理“主题坏了”。
- 对缺失 theme path `unwrap()` 而不是默认值。
- 把 XML 运行时错误模型套到编译期宏上（XML 是 compile-time only）。
