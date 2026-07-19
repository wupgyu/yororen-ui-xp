# 错误处理 — yororen_ui_xml_macro

- 参数解析失败 → `syn::Error`
- 用户 schema 读/解析失败 → `syn::Error`（带路径）
- XML/codegen 错误 → 底层 `XmlError` 转为编译诊断
- meta-crate 在 **未启用 xml feature** 时提供 stub 宏，直接 `compile_error!` 提示开启 feature
