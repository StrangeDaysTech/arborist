# 中国法规框架 — StrayMark

> 在 `.straymark/config.yml` 中启用 `regional_scope: china` 时,StrayMark 涵盖的六项中国 AI / 数据法规的概览。

**Languages**: [English](../../CHINA-REGULATORY-FRAMEWORK.md) | [Español](../es/CHINA-REGULATORY-FRAMEWORK.md) | 简体中文

---

## 启用方式

中国法规检查为 **opt-in**。在 `.straymark/config.yml` 中启用:

```yaml
regional_scope:
  - global   # NIST + ISO 42001(始终可用)
  - eu       # EU AI Act + GDPR
  - china    # 增加下表的 6 个框架
```

启用 `china` 时:
- `straymark new` 暴露 4 种中国专属文档类型(PIPIA, CACFILE, TC260RA, AILABEL)。
- `straymark compliance --all` 包含 6 个中国检查器。
- `straymark validate` 强制执行 CROSS-004…CROSS-011 与 TYPE-003…TYPE-006。

不在 `regional_scope` 中包含 `china` 的项目不受影响。

---

## 覆盖矩阵

| # | 法规 | 类型 | 状态 | StrayMark 证据 |
|---|------|------|------|--------------|
| 1 | **TC260 人工智能安全治理框架 v2.0**(2025-09-15) | 推荐(待制定为 GB) | 生效 | TC260RA 模板;`tc260_risk_level`、`tc260_application_scenario`、`tc260_intelligence_level`、`tc260_application_scale` 字段 |
| 2 | **PIPL** + **PIPIA**(《个人信息保护法》第55-56条) | 强制 | 自 2021-11-01 起生效 | PIPIA 模板;`pipl_*` 字段;PIPIA 留存 ≥ 3 年 |
| 3 | **GB 45438-2025**《网络安全技术 人工智能生成合成内容标识方法》 | **强制** | 自 2025-09-01 起施行 | AILABEL 模板;MCARD 上的 `gb45438_*` 字段 |
| 4 | **CAC 算法备案**(《互联网信息服务算法推荐管理规定》;《生成式人工智能服务管理暂行办法》) | 范围内服务强制 | 生效 | CACFILE 模板;MCARD 上的 `cac_filing_required`、`cac_filing_number`、`cac_filing_status` 字段 |
| 5 | **GB/T 45652-2025** 预训练与微调数据安全 | 推荐 | 自 2025-11-01 起施行 | SBOM/MCARD 上的 `gb45652_training_data_compliance` 字段 |
| 6 | **CSL 2026** 网络安全法修订与《国家网络安全事件报告管理办法》 | 强制 | 自 2026-01-01 起生效 | INC 上的 "CSL 2026 Incident Reporting" 部分;`csl_severity_level`、`csl_report_deadline_hours` 字段 |

---

## 文档类型 → 框架映射

| 框架 | 主模板 | 交叉引用 | 其他可选字段 |
|------|--------|--------|------------|
| TC260 v2.0 | TC260RA | ETH, MCARD | AILOG / SEC 上的 `tc260_risk_level` |
| PIPL / PIPIA | PIPIA | DPIA(交叉引用) | ETH / MCARD 上的 `pipl_applicable` |
| GB 45438 | AILABEL | MCARD(生成式模型) | MCARD 上的 `gb45438_applicable` |
| CAC 算法备案 | CACFILE | MCARD, SBOM | MCARD 上的 `cac_filing_number` |
| GB/T 45652 | SBOM/MCARD 中的章节 | TC260RA | `gb45652_training_data_compliance` |
| CSL 2026 | INC(扩展) | (无) | INC 上的 `csl_severity_level` |

---

## 实施指南

| 框架 | 指南 |
|------|------|
| TC260 v2.0 风险分级 | [TC260-IMPLEMENTATION-GUIDE.md](TC260-IMPLEMENTATION-GUIDE.md) |
| PIPL 第55条触发 → PIPIA | [PIPL-PIPIA-GUIDE.md](PIPL-PIPIA-GUIDE.md) |
| 双重备案流程 | [CAC-FILING-GUIDE.md](CAC-FILING-GUIDE.md) |
| 显式 + 隐式标识 | [GB-45438-LABELING-GUIDE.md](GB-45438-LABELING-GUIDE.md) |

---

## 合规检查

启用 `china` 范围后,通过 `straymark compliance --standard <名称>` 暴露以下检查:

| `--standard` | 检查 ID | 验证内容 |
|--------------|--------|--------|
| `china-tc260` | TC260-001/002/003 | 至少存在一个 TC260RA;高风险等级要求人工审核;三个分级标准已填充 |
| `china-pipl` | PIPL-001/002/003/004 | 当 `pipl_applicable` 或敏感数据时存在 PIPIA;跨境传输已记录;留存 ≥ 3 年 |
| `china-gb45438` | GB45438-001/002/003 | MCARD 声明生成式内容时存在 AILABEL;声明显式 + 隐式标识策略;元数据字段已填充 |
| `china-cac` | CAC-001/002/003 | `cac_filing_required: true` 时存在 CACFILE;状态未滞留于 `pending` 超过 90 天;`*_approved` 时填写备案号 |
| `china-gb45652` | GB45652-001/002 | SBOM 声明每条训练数据合规;MCARD 描述数据安全控制 |
| `china-csl` | CSL-001/002/003 | INC 包含 `csl_severity_level`;时限与严重程度一致(1h ↔ particularly_serious、4h ↔ relatively_major);≥ relatively_major 提交 30 天事后审查 |

`straymark compliance --region china` 一次性运行全部 6 项。

---

## 资料来源

详细参考英文版本及英文版底部的链接列表。

<!-- StrayMark | https://strangedays.tech -->
