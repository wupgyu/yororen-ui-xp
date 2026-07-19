# 数据持久化规范 — xp_renderer

> 本 crate 无数据库；唯一资源是 `themes/xp-luna.json`（编译期 `include_str!` 嵌入）。

---

## 主题资源

`xp-luna.json` 沿用共享主题 schema（`surface.* / content.* / border.* / action.{neutral,primary,danger}.* / status.* / shadow.* / tokens.*`），路径与其他 renderer 保持一致，保证跨 renderer 换主题时读取不失效。

## `xp.*` 扩展路径契约

XP 专属值放 `xp.*` 前缀路径，renderer 读取时必须带 `style.rs` 常量 fallback（`xp_color` / `xp_number`）：

| 路径 | 值 | 消费方 |
|------|----|--------|
| `xp.titlebar.{from,to,text,height}` | 标题条蓝渐变 + 白字 + 28px | Modal |
| `xp.button.{face_from,face_to,border,default_border,hover_ring,primary_from,primary_to}` | 按钮面渐变与边色 | Button 族、Slider 拇指、Badge/Tag、步进器 |
| `xp.progress.{track_bg,track_border,chunk_from,chunk_to,chunk_border,segment_width,segment_gap,segment_radius}` | 分段进度条 | ProgressBar |
| `xp.bevel.{outer_light,inner_light,inner_dark,outer_dark}` | Win32 斜面四色 | 凹陷井、面板、轨道 |
| `xp.input.{bg,border,focus_border}` | 输入框白底 + 边色 | inputs 组 |
| `xp.selection.{bg,fg,hover_bg,hover_border}` | 选中 #316AC5 / 淡蓝 hover | 菜单、列表、树、下拉 |
| `xp.check.{glyph,box_bg,box_border}` | 蓝勾 / 白盒 / 边 | Checkbox、Radio |

修改上表任何路径时：先 `grep -r "<path>" crates/yororen-ui-xp-renderer/` 找全部消费方，并同步 `xp_luna_theme_parses_with_key_paths` 测试断言。

## 规则

| 做 | 不做 |
|----|------|
| 新增 XP 专属色放 `xp.*` 并在 style.rs 加同名 fallback | 把共享 schema 路径（`surface.*` 等）改成 XP 语义 |
| 几何尺寸放 `tokens.control.<widget>.*` | 在 JSON 里放位图 / 九宫格引用（纯矢量决策） |

---

## 常见错误

- 假设 theme JSON 有编译期 schema 校验（实际是开放路径 + renderer 默认回退）。
- 在 `xp.*` 与 `style.rs` 常量之间改了一边忘了另一边——测试只断言路径存在，不断言两边一致。
