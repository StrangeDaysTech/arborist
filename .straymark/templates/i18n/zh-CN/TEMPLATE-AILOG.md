---
id: AILOG-YYYY-MM-DD-NNN
title: [操作的描述性标题]
status: accepted
created: YYYY-MM-DD
agent: [agent-name-v1.0]
confidence: high | medium | low
review_required: false
risk_level: low | medium | high | critical
eu_ai_act_risk: not_applicable  # unacceptable | high | limited | minimal | not_applicable
nist_genai_risks: []            # privacy | bias | confabulation | cbrn | dangerous_content | environmental | human_ai_config | information_integrity | information_security | intellectual_property | obscene_content | value_chain
iso_42001_clause: []            # 4 | 5 | 6 | 7 | 8 | 9 | 10
lines_changed: 0                # 可自动计算
files_modified: []              # 可自动计算
observability_scope: none        # none | basic | full — 当 OTel 监测相关时设置
tags: []
related: []
---

# AILOG: [标题]

## 摘要

[简要描述执行了什么操作及其原因]

## 背景

[促使执行此操作的情境]

## 执行的操作

1. [操作 1]
2. [操作 2]
3. [操作 3]

## 批次台账 (Batch Ledger)

> 对跨越 3+ 批次或 >1 天执行的 Charter 使用此章节。
> 每个批次的 commit 落地**立即**更新对应条目，使用
> `straymark charter batch-complete <CHARTER-ID> <N>`。Charter 关闭时
> 仍为 `(pending)` 的条目会导致 `straymark charter drift` 失败。
>
> 对于单批次或单会话的 AILOG，请完全省略此章节——上方的
> `## 执行的操作` 已足够。

### Batch 1 — [来自 Charter §Tasks 的名称]

(pending)

### Batch 2 — [来自 Charter §Tasks 的名称]

(pending)

## 修改的文件

| 文件 | 变更行数 (+/-) | 变更描述 |
|------|---------------|----------|
| `path/to/file.ts` | +N/-M | [变更描述] |

## 所做的决策

[如有相关决策，请简要记录或引用 AIDEC]

## 影响

- **功能性**: [描述]
- **性能**: [描述或不适用]
- **安全性**: [描述或不适用]
- **隐私**: [描述或不适用]
- **环境影响**: [描述或不适用]

## 验证

- [ ] 代码编译无错误
- [ ] 测试通过
- [ ] 已执行人工审查
- [ ] 安全扫描已通过（如 risk_level 为 high/critical）
- [ ] 隐私审查已完成（如涉及个人身份信息）

## EU AI Act 相关考量

> 仅当 `eu_ai_act_risk` 为 `high` 或 `limited` 时填写本节。

- **风险分类**: [high | limited]
- **附录 III 类别**: [如适用 — 指定类别]
- **是否需要合规评估**: [是/否]
- **透明度义务**: [适用义务的描述]

## NIST GenAI 风险评估

> 当变更涉及生成式 AI 组件时填写本节。
> 参考：NIST AI 600-1（生成式 AI 概况）。

| # | 类别 | 是否适用 | 描述 | 缓解措施 |
|---|------|---------|------|----------|
| 1 | CBRN 信息 | [是/否] | | |
| 2 | 虚构内容 | [是/否] | | |
| 3 | 危险/暴力/仇恨内容 | [是/否] | | |
| 4 | 数据隐私 | [是/否] | | |
| 5 | 环境影响 | [是/否] | | |
| 6 | 有害偏见/同质化 | [是/否] | | |
| 7 | 人机配置 | [是/否] | | |
| 8 | 信息完整性 | [是/否] | | |
| 9 | 信息安全 | [是/否] | | |
| 10 | 知识产权 | [是/否] | | |
| 11 | 淫秽/辱骂内容 | [是/否] | | |
| 12 | 价值链/组件集成 | [是/否] | | |

## 补充说明

[任何其他相关信息]

> **可观测性说明**：如果此变更修改了可观测性监测（新增 span、变更属性、管道配置），请描述可观测性影响并添加标签 `observabilidad`。

---

<!-- Template: StrayMark | https://strangedays.tech -->
