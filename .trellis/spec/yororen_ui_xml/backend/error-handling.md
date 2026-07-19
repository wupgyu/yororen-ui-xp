# 错误处理 — yororen_ui_xml

---

## XmlError

```text
XmlErrorKind:
  ParseError | UnknownTag | UnknownAttribute | InvalidExpression | Unsupported
```

- 带可选 `offset`（字节偏移）→ macro 侧渲染 `line:col` + snippet。
- include 的外部 XML 可用 `rendered` 保留自己的位置信息。
- `render()` / `render_with(LocationTracker)` 供宏转 `compile_error!`。

---

## 规则

1. 库侧收集结构化错误，**不**在库内直接依赖 `proc_macro::Diagnostic`。
2. 工具二进制对非致命问题 `eprintln!("warning: ...")`。
3. 这是 **compile-time only** 栈；不要设计运行时 XML 解析错误 API 给 app 热路径。
