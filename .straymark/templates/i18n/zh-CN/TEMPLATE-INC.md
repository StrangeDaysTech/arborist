---
id: INC-YYYY-MM-DD-NNN
title: [事件标题]
status: draft
created: YYYY-MM-DD
agent: [agent-name-v1.0]
confidence: medium
review_required: true

# --- 审批工作流（可选，审批时填写）---
# reviewed_by: <审批人标识>             # 邮箱 | github 用户 | DID
# reviewed_at: YYYY-MM-DD
# review_outcome: approved             # approved | revisions_requested | rejected
risk_level: high | critical
severity: SEV1 | SEV2 | SEV3 | SEV4
eu_ai_act_applicable: false
incident_report_deadline: null  # YYYY-MM-DD — regulatory deadline if applicable
iso_42001_clause: []            # 4 | 5 | 6 | 7 | 8 | 9 | 10
observability_scope: none        # none | basic | full — set when OTel instrumentation is relevant
tags: []
related: []
incident_date: YYYY-MM-DD
resolved_date: null
---

# INC: [事件标题]

> **部分分析**：本文档包含 AI 智能体的分析内容。
> 最终结论和纠正措施需要人工审查。

## 事件概要

| 字段 | 值 |
|------|-----|
| 严重程度 | [SEV1/SEV2/SEV3/SEV4] |
| 开始日期/时间 | [YYYY-MM-DD HH:MM UTC] |
| 解决日期/时间 | [YYYY-MM-DD HH:MM UTC] |
| 持续时长 | [X 小时 Y 分钟] |
| 受影响服务 | [服务列表] |
| 受影响用户 | [估算数量] |
| 业务影响 | [描述] |

## 严重程度定义

| 严重程度 | 定义 |
|----------|------|
| SEV1 | 服务完全中断，业务受到严重影响 |
| SEV2 | 严重降级，主要功能受到影响 |
| SEV3 | 部分降级，存在替代方案 |
| SEV4 | 影响较小，仅少数用户受到影响 |

## 时间线

> 如果系统使用 OpenTelemetry，请包含 trace-id 作为关联证据。

| 时间 (UTC) | 事件 | Trace ID | Span ID | 仪表盘链接 |
|------------|------|----------|---------|-----------|
| HH:MM | [首次检测到症状] | [trace-id（如有）] | [span-id] | [链接] |
| HH:MM | [触发告警] | | | |
| HH:MM | [通知团队] | | | |
| HH:MM | [初步诊断] | | | |
| HH:MM | [实施缓解措施] | | | |
| HH:MM | [服务恢复] | | | |
| HH:MM | [事件关闭] | | | |

## 根因分析

### 直接原因
[直接导致故障的原因]

### 间接原因
1. [间接因素 1]
2. [间接因素 2]

### 根本原因（智能体分析）
[智能体对根本原因的分析]

> **注意**：此分析需要技术团队的验证。

## 影响

### 技术影响
- [技术影响 1]
- [技术影响 2]

### 业务影响
- [业务影响 1]
- [业务影响 2]

### 用户影响
- [用户影响 1]
- [用户影响 2]

## 已采取的缓解措施

1. [为解决事件所采取的措施]
2. [为解决事件所采取的措施]

## 建议纠正措施

> 这些建议需要人工审查和优先级排序。

| # | 措施 | 类型 | 优先级 | 负责人 | 截止日期 |
|---|------|------|--------|--------|----------|
| 1 | [措施] | 预防 | [High/Medium/Low] | [待定] | [待定] |
| 2 | [措施] | 检测 | [High/Medium/Low] | [待定] | [待定] |
| 3 | [措施] | 响应 | [High/Medium/Low] | [待定] | [待定] |

## 经验教训

### 做得好的方面
- [积极方面 1]
- [积极方面 2]

### 需要改进的方面
- [待改进方面 1]
- [待改进方面 2]

### 侥幸之处
- [本可能更严重的方面]

## EU AI Act 事件报告

> 对于 EU AI Act 下的高风险 AI 系统，事件必须在以下时限内向市场监管机构报告：
> - **15 天**（标准事件）
> - **10 天**（导致死亡的事件）
> - **2 天**（大范围或非常严重的事件）
>
> 参考：Article 73, EU AI Act。
>
> 仅当 `eu_ai_act_applicable` 为 `true` 时填写本节。

| 字段 | 值 |
|------|-----|
| 报告截止日期 | [YYYY-MM-DD] |
| 是否已通知主管机构 | [是/否/不适用] |
| 报告编号 | [已提交的参考编号] |

## 待解决问题

1. [需要进一步调查的问题]
2. [需要团队讨论的问题]

---

## 事后复盘

| 字段 | 值 |
|------|-----|
| 审查人 | [姓名] |
| 审查日期 | [YYYY-MM-DD] |
| 状态 | [Draft/Reviewed/Closed] |

<!-- Template: StrayMark | https://strangedays.tech -->
