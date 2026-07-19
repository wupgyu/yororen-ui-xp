# 质量规范 — yororen_ui_locale_en

---

## 必需

1. 新增 UI 文案 key 时，**en / zh-CN / ar 同步**（至少 en 完整，其他可暂时回退但应跟踪缺口）。
2. key 使用点分嵌套：`select.placeholder`、`common.save`。
3. 复数用对象形式（`one` / `other`），不要在代码里拼英语复数。
4. RTL：`ar` 依赖 core 的 `text_direction()`；翻译本身避免写死左/右方向符号除非业务需要。
5. 保持 crate 只依赖 core（+ 必要序列化），不依赖 renderer。

---

## 测试

```bash
cargo test -p yororen_ui_locale_en
```

至少覆盖：bundled JSON 可解析 + 抽样 key 存在。
