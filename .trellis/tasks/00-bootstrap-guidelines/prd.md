# 启动任务：补全项目开发规范

**你（AI）正在执行这个任务，开发者不会直接阅读这个文件。**

开发者刚刚第一次在这个项目里运行了 `trellis init`。
现在 `.trellis/` 已创建，里面包含待补全的 spec 脚手架，这个启动任务也已经
出现在 `.trellis/tasks/` 下。当他们准备处理它时，应当在提供 Trellis 会话身份
的会话里启动这个任务。

**你的目标**：帮助他们把团队真实的编码规范补充进 `.trellis/spec/`。
未来这个项目里的每次 AI 会话——包括 `trellis-implement` 和 `trellis-check`
子代理——都会自动加载每个任务 jsonl 清单里列出的 spec 文件。spec 为空，
子代理就会写出泛化代码；spec 真实完整，子代理才会贴近团队现有风格。

不要一上来倾倒说明。先用一句简短欢迎语开场，确认仓库里是否已有约定文档
（如 CLAUDE.md、.cursorrules 等），再以对话方式推进。

---

## 当前状态（完成后更新下面复选框）

- [ ] 补充 yororen_ui 的开发规范
- [ ] 补充 yororen_ui_core 的开发规范
- [ ] 补充 yororen_ui_default_renderer 的开发规范
- [ ] 补充 yororen_ui_brutalism_renderer 的开发规范
- [ ] 补充 yororen_ui_xml 的开发规范
- [ ] 补充 yororen_ui_xml_macro 的开发规范
- [ ] 补充 yororen_ui_locale_en 的开发规范
- [ ] 补充 yororen_ui_locale_zh_cn 的开发规范
- [ ] 补充 yororen_ui_locale_ar 的开发规范
- [ ] 补充 counter-demo 的开发规范
- [ ] 补充 showcase-xml-demo 的开发规范
- [ ] 补充 gallery-demo 的开发规范
- [ ] 补充 gallery-xml-demo 的开发规范
- [ ] 补充 inputs-demo 的开发规范
- [ ] 补充 layers-demo 的开发规范
- [ ] 补充 theme-showcase-demo 的开发规范
- [ ] 补充 variant-showcase-demo 的开发规范
- [ ] 补充代码示例

---

## 需要补充的 Spec 文件

### Package: yororen_ui (`spec/yororen_ui/`)

- Backend guidelines: `.trellis/spec/yororen_ui/backend/`

### Package: yororen_ui_core (`spec/yororen_ui_core/`)

- Backend guidelines: `.trellis/spec/yororen_ui_core/backend/`

### Package: yororen_ui_default_renderer (`spec/yororen_ui_default_renderer/`)

- Backend guidelines: `.trellis/spec/yororen_ui_default_renderer/backend/`

### Package: yororen_ui_brutalism_renderer (`spec/yororen_ui_brutalism_renderer/`)

- Backend guidelines: `.trellis/spec/yororen_ui_brutalism_renderer/backend/`

### Package: yororen_ui_xml (`spec/yororen_ui_xml/`)

- Backend guidelines: `.trellis/spec/yororen_ui_xml/backend/`

### Package: yororen_ui_xml_macro (`spec/yororen_ui_xml_macro/`)

- Backend guidelines: `.trellis/spec/yororen_ui_xml_macro/backend/`

### Package: yororen_ui_locale_en (`spec/yororen_ui_locale_en/`)

- Backend guidelines: `.trellis/spec/yororen_ui_locale_en/backend/`

### Package: yororen_ui_locale_zh_cn (`spec/yororen_ui_locale_zh_cn/`)

- Backend guidelines: `.trellis/spec/yororen_ui_locale_zh_cn/backend/`

### Package: yororen_ui_locale_ar (`spec/yororen_ui_locale_ar/`)

- Backend guidelines: `.trellis/spec/yororen_ui_locale_ar/backend/`

### Package: counter-demo (`spec/counter-demo/`)

- Backend guidelines: `.trellis/spec/counter-demo/backend/`

### Package: showcase-xml-demo (`spec/showcase-xml-demo/`)

- Backend guidelines: `.trellis/spec/showcase-xml-demo/backend/`

### Package: gallery-demo (`spec/gallery-demo/`)

- Backend guidelines: `.trellis/spec/gallery-demo/backend/`

### Package: gallery-xml-demo (`spec/gallery-xml-demo/`)

- Backend guidelines: `.trellis/spec/gallery-xml-demo/backend/`

### Package: inputs-demo (`spec/inputs-demo/`)

- Backend guidelines: `.trellis/spec/inputs-demo/backend/`

### Package: layers-demo (`spec/layers-demo/`)

- Backend guidelines: `.trellis/spec/layers-demo/backend/`

### Package: theme-showcase-demo (`spec/theme-showcase-demo/`)

- Backend guidelines: `.trellis/spec/theme-showcase-demo/backend/`

### Package: variant-showcase-demo (`spec/variant-showcase-demo/`)

- Backend guidelines: `.trellis/spec/variant-showcase-demo/backend/`


### 思考指南（已预填）

`.trellis/spec/guides/` 中已预置通用思考指南。
只有当其中内容明显不适合当前项目时，才需要调整。

---

## 如何补充 Spec

### 第一步：优先导入已有约定文档（推荐）

先在仓库中搜索已有的约定文档。如果存在，先读这些文件，再把相关规则整理到对应
的 `.trellis/spec/` 文件里——这通常比从零开始快得多。

| 文件 / 目录 | 工具 |
|------|------|
| `CLAUDE.md` / `CLAUDE.local.md` | Claude Code |
| `AGENTS.md` | Codex / Claude Code / 兼容 agent 的工具 |
| `.cursorrules` | Cursor |
| `.cursor/rules/*.mdc` | Cursor（规则目录） |
| `.windsurfrules` | Windsurf |
| `.clinerules` | Cline |
| `.roomodes` | Roo Code |
| `.github/copilot-instructions.md` | GitHub Copilot |
| `.vscode/settings.json` → `github.copilot.chat.codeGeneration.instructions` | VS Code Copilot |
| `CONVENTIONS.md` / `.aider.conf.yml` | aider |
| `CONTRIBUTING.md` | 通用项目约定 |
| `.editorconfig` | 编辑器格式规则 |

### 第二步：分析代码库中未被文档覆盖的部分

从真实代码里归纳模式。写每个 spec 文件前：
- 先找到 2-3 个真实示例。
- 记录真实文件路径，不要写假想路径。
- 记下团队明确避免的反模式。

### 第三步：记录现实，而不是理想状态

**关键**：写代码库现在**实际上怎样做**，而不是“应该怎样做”。
子代理会按 spec 来实现；如果 spec 写的是不存在的理想模式，后续生成的代码就会和
仓库现状脱节。

如果团队存在已知技术债，请记录当前现状——如何改进是后续话题，不属于这次启动任务。

---

## 运行时机制速览（当开发者问“为什么需要 spec”时再解释）

- 每个 AI 编码任务都会派生两个子代理：`trellis-implement`（负责写代码）和
  `trellis-check`（负责验证质量）。
- 每个任务都有 `implement.jsonl` / `check.jsonl` 清单，用于列出需要加载的 spec 文件。
- 平台 hook 会自动把这些 spec 文件以及任务的 `prd.md` 注入到每个子代理的 prompt 中，
  这样它们能按团队约定编码或评审，而不需要人工反复粘贴。
- 唯一事实来源是 `.trellis/spec/`。这也是为什么现在把它补好，会长期持续收益。

---

## 完成方式

当开发者确认上面的清单都已结合真实示例补完后，引导他们执行：

```bash
python ./.trellis/scripts/task.py finish
python ./.trellis/scripts/task.py archive 00-bootstrap-guidelines
```

归档后，后续每位新加入这个项目的开发者拿到的将不再是这个启动任务，而是
`00-join-<slug>` 入项引导任务。

---

## 建议开场白

“欢迎使用 Trellis！刚才的初始化已经让我可以帮助你补齐项目 spec。这是一次性设置，
做好之后，后续每次 AI 会话都会按团队规范工作，而不是产出泛化代码。开始前你手头有
现成的约定文档（如 CLAUDE.md、.cursorrules、CONTRIBUTING.md）可以先让我读取吗？
如果没有，我就从代码库里开始归纳。”
