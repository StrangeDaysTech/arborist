---
id: TC260RA-YYYY-MM-DD-NNN
title: "[系统] TC260 风险评估"
status: draft
created: YYYY-MM-DD
agent: [agent-name]
confidence: medium
review_required: true

# --- 审批工作流（可选，审批时填写）---
# reviewed_by: <审批人标识>             # 邮箱 | github 用户 | DID
# reviewed_at: YYYY-MM-DD
# review_outcome: approved             # approved | revisions_requested | rejected
risk_level: high
tc260_application_scenario: null  # 例如:public_service | healthcare | finance | content_generation | social | safety_critical
tc260_intelligence_level: null    # narrow | foundation | agentic | general
tc260_application_scale: null     # individual | organization | societal | cross_border
tc260_risk_level: not_applicable  # low | medium | high | very_high | extremely_severe | not_applicable
tc260_endogenous_risks: []
tc260_application_risks: []
tc260_derivative_risks: []
iso_42001_clause: [6]
tags: [china, tc260, risk-assessment]
related: []
---

# TC260RA: [系统] TC260 风险评估

> **重要提示**:此文档为 AI 代理创建的草稿,需要人工审核与批准。
>
> 与全国信息安全标准化技术委员会(TC260)于 2025 年 9 月 15 日发布的 **《人工智能安全治理框架 2.0》** 一致。

## 1. 四大支柱映射

说明本系统如何对应框架的四大支柱:

| 支柱 | 本系统的覆盖 |
|------|------------|
| **治理原则**(以人为本、AI 向善、安全可控) | [陈述] |
| **风险分类**(技术内生 / 应用 / 衍生) | [参见下方分类] |
| **技术应对** | [参见第4节] |
| **治理措施**(组织实施) | [参见第5节] |

## 2. 三标准风险分级(5.5节 / 附录1)

风险等级由 **应用场景 × 智能等级 × 应用规模** 组合得出。

### 2.1 应用场景

- **选定**:[public_service / healthcare / finance / content_generation / social / safety_critical / industrial_control / other]
- **理由**:[为何此场景适用]

### 2.2 智能等级

| 等级 | 定义 | 本系统 |
|------|------|------|
| Narrow | 单一用途模型 | [ ] |
| Foundation | 通用基础模型(LLM、视觉语言) | [ ] |
| Agentic | 基础模型 + 自主工具使用 | [ ] |
| General | 接近通用人工智能 | [ ] |

- **选定**:[narrow / foundation / agentic / general]

### 2.3 应用规模

| 规模 | 定义 | 本系统 |
|------|------|------|
| Individual | 单一用户 / 小团队 | [ ] |
| Organization | 单一组织部署 | [ ] |
| Societal | 影响公众的重要部分 | [ ] |
| Cross-border | 跨中国大陆与其他司法辖区运营 | [ ] |

- **选定**:[individual / organization / societal / cross_border]

### 2.4 计算所得风险等级

| 风险等级 | 描述 |
|---------|------|
| 低(Low) | 预期最小危害;标准控制即可 |
| 中(Medium) | 可预见、有限的危害;需复审与基本应对 |
| 高(High) | 对个人或特定群体有重大风险;需全面控制 |
| 很高(Very High) | 对社会稳定或大规模人群有风险;需行业级监督 |
| 极重(Extremely Severe) | 灾难性 / 系统性危害风险;失控 / 灾难性风险关切 |

- **计算等级**:[low / medium / high / very_high / extremely_severe]
- **理由**:[结合场景 × 智能 × 规模的推理]

## 3. 风险分类

### 3.1 内生技术风险

> 来源于 AI 模型自身:漏洞、偏见、幻觉、鲁棒性不足。

| 风险 | 描述 | 可能性 | 严重程度 | 缓解 |
|------|------|------|--------|------|
| [风险1] | [描述] | [低/中/高] | [低/中/高] | [措施] |
| [风险2] | [描述] | [低/中/高] | [低/中/高] | [措施] |

### 3.2 应用风险

> 来源于技术应用方式:误用、范围蔓延、依赖。

| 风险 | 描述 | 可能性 | 严重程度 | 缓解 |
|------|------|------|--------|------|
| [风险1] | [描述] | [低/中/高] | [低/中/高] | [措施] |
| [风险2] | [描述] | [低/中/高] | [低/中/高] | [措施] |

### 3.3 衍生风险

> 二阶社会影响:就业替代、舆论塑造、生态破坏。

| 风险 | 描述 | 可能性 | 严重程度 | 缓解 |
|------|------|------|--------|------|
| [风险1] | [描述] | [低/中/高] | [低/中/高] | [措施] |
| [风险2] | [描述] | [低/中/高] | [低/中/高] | [措施] |

## 4. 技术应对

将每项优先风险映射到一项或多项技术控制(红队、对齐、内容过滤、GB 45438 水印、模型评估套件、沙箱等)。

| 风险编号 | 应对措施 | 责任人 | 验证 |
|---------|---------|------|------|
| [E.1] | [控制] | [角色] | [测试方案 / 指标] |
| [A.1] | [控制] | [角色] | [测试方案 / 指标] |
| [D.1] | [控制] | [角色] | [测试方案 / 指标] |

## 5. 治理措施

- **指定负责人**:[角色 / 姓名]
- **内部报告频率**:[月度 / 季度]
- **升级路径**:[向谁升级及触发条件]
- **开源组件**:[如系统嵌入开源 AI:依据 v2.0 OSS 条款的治理]
- **灾难性风险监测**:[very_high / extremely_severe 等级所需:如何监测失控场景]

## 6. 监测与复审

- **下次复审日期**:[YYYY-MM-DD]
- **复审触发**:[模型版本变更 / 场景扩展 / 规模跃升 / 法规更新]
- **关联文档**:[ETH-..., MCARD-..., AILABEL-..., CACFILE-...]

## 批准

| 批准人 | 日期 | 决定 | 条件 |
|--------|------|------|------|
| [审核人] | [YYYY-MM-DD] | [批准 / 有条件批准 / 拒绝] | [条件(如有)] |

<!-- Template: StrayMark | https://strangedays.tech -->
