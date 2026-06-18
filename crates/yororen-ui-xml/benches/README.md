# XML 引擎性能基准

```bash
cargo bench -p yororen_ui_xml --bench codegen_bench
```

基准覆盖：

- `codegen/simple_label` — 单叶子组件代码生成
- `codegen/button_with_event` — 带事件自动包装
- `codegen/nested_containers` — 嵌套容器
- `codegen/for_loop` — `<For>` 控制流
- `codegen/conditional` — `<If>` 控制流
- `parser/nested` — 仅 XML 解析

结果会输出到终端，并在 `target/criterion/` 生成 HTML 报告。
