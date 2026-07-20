# 数据持久化规范 — xp_renderer

> 本 crate 无数据库；唯一资源是 `themes/xp-luna.json`（编译期 `include_str!` 嵌入）。

---

## 主题资源

`xp-luna.json` 沿用共享主题 schema（`surface.* / content.* / border.* / action.{neutral,primary,danger}.* / status.* / shadow.* / tokens.*`），路径与其他 renderer 保持一致，保证跨 renderer 换主题时读取不失效。

## `xp.*` 扩展路径契约

XP 专属值放 `xp.*` 前缀路径，renderer 读取时必须带 `style.rs` 常量 fallback（`xp_color` / `xp_number`）：

| 路径 | 值 | 消费方 |
|------|----|--------|
| `xp.titlebar.{from,mid_1,mid_2,mid_3,to}` | 标题条 5-stop 蓝渐变(`#0997FF→#0053EE→#0050EE→#0066FF→#0058EB`,4 band 叠带) | Modal 标题栏 |
| `xp.titlebar.{inactive_from,inactive_to}` | 失焦标题条 `#B8C4DC→#98A8C0` | Modal 标题栏 |
| `xp.titlebar.{text,height}` | 白字 + 26px | Modal |
| `xp.button.{face_from,face_mid,face_to}` | 按钮面 3-stop(`#FFF→#ECE9D8@45%→#DDD8C8`,双 band) | Button 族、Slider 拇指、Badge/Tag、步进器 |
| `xp.button.{border,default_border,hover_ring,primary_from,primary_to}` | 边 `#003C74` / hover 橙环 `#FFCF31` / 主蓝渐变 | 同上 |
| `xp.progress.{track_bg,track_border,chunk_from,chunk_to,chunk_border,segment_width,segment_gap,segment_radius}` | 分段进度条(css 绿 `#68D868→#189418`,track 边 `#7F9DB9`) | ProgressBar |
| `xp.bevel.{outer_light,inner_light,inner_dark,outer_dark}` | Win32 斜面四色 | 凹陷井、面板、轨道、divider 蚀刻双线 |
| `xp.input.{bg,border,focus_border}` | 输入框白底 + 边 `#7F9DB9` + focus `#316AC5`;`border` 兼作 listbox/tree/虚拟列表容器边 | inputs 组、lists 容器 |
| `xp.selection.{bg,fg,hover_bg,hover_border}` | 选中 `#316AC5` 白字;hover `#CFE0FA` | 列表、树、下拉 |
| `xp.menu.{hover_bg,hover_fg}` | 菜单项 hover `#316AC5` 蓝底白字 | Menu |
| `xp.toast.{bg,border}` | 气泡黄 `#FFFFE1` + 1px 黑边 | Toast、Notification |
| `xp.window.{border_active,border_inactive,body_border}` | 窗口边框 `#0058E6`/`#98A8C0`;body 内边框 `#A09C8C` | Modal |
| `xp.explorer.{toolbar_bg,toolbar_border,address_border,task_pane_bg_from,task_pane_bg_to,task_card_header_*,task_card_body_*,task_card_title,content_bg,link,group_header,group_rule_from}` | Explorer 场景:工具栏米底 + 蚀刻线 `#0000001A`、任务面板蓝渐变、任务卡三段渐变、分组标题 `#0C327D` + `#70BFFF` 渐变短尺 | XP Modal 全窗铬、ExplorerTaskCard、demo 直读 |
| `xp.caption.{from,to,close_from,close_to,border,size,radius}` | 标题按钮蓝渐变 `#3C8CFD→#1565E8`、close 红渐变 `#F08A6D→#D84A28`、半透明白边、21×21、radius 3 | Modal caption 按钮 |
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
