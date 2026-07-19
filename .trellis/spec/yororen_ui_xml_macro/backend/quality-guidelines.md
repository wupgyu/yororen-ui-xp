# 质量规范 — yororen_ui_xml_macro

- `#![forbid(unsafe_code)]`
- 保持 **thin wrapper**：逻辑回归 `yororen-ui-xml` 便于单测
- 发布顺序：`yororen_ui_xml` 先于 `yororen_ui_xml_macro`（见 `scripts/publish.sh`）
- 变更宏语法属于破坏性变更
