# 错误处理 — demo

- Demo 安装路径与生产应用相同：bundled theme/locale 使用 `expect`/`install` 辅助函数。
- 交互错误用 UI 状态展示（disabled、校验文案、notification），不要 `unwrap` 用户输入。
- 教学 demo（layers）里对内部不变量可用 `expect`，但需在注释说明前提。
