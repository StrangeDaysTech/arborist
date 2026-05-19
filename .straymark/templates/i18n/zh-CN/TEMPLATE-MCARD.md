---
id: MCARD-YYYY-MM-DD-NNN
title: "[模型名称] Card"
status: draft
created: YYYY-MM-DD
agent: [agent-name]
confidence: medium
review_required: true

# --- 审批工作流（可选，审批时填写）---
# reviewed_by: <审批人标识>             # 邮箱 | github 用户 | DID
# reviewed_at: YYYY-MM-DD
# review_outcome: approved             # approved | revisions_requested | rejected
risk_level: medium
eu_ai_act_risk: not_applicable  # unacceptable | high | limited | minimal | not_applicable
nist_genai_risks: []            # privacy | bias | confabulation | cbrn | dangerous_content | environmental | human_ai_config | information_integrity | information_security | intellectual_property | obscene_content | value_chain
iso_42001_clause: [8]
model_name: ""
model_type: LLM  # LLM | classifier | regressor | generator | recommender | other
model_version: ""
provider: ""
license: ""
tags: [ai-model]
related: []
---

# MCARD: [模型名称] Card

> **重要提示**：本文档是由 AI 智能体创建的草稿。
> 在继续之前，需要经过人工审查和批准。

## 模型详情

> 基于 Mitchell et al. (2019) -- "Model Cards for Model Reporting"。

| 字段 | 值 |
|------|-----|
| 开发者 | [开发该模型的组织或个人] |
| 模型日期 | [YYYY-MM-DD -- 模型训练或发布日期] |
| 模型版本 | [版本标识符] |
| 模型类型 | [LLM / classifier / regressor / generator / recommender / other] |
| 训练算法 | [用于训练的算法] |
| 基础模型 | [基础模型名称和版本（如为微调）；否则填 N/A] |
| 论文/资源 | [论文、博客文章或文档的 URL 或引用] |
| 引用格式 | [BibTeX 或纯文本引用] |
| 许可证 | [模型许可证 -- 例如 Apache 2.0、MIT、专有许可] |

## 预期用途

### 主要预期用途

- [主要用例 1]
- [主要用例 2]

### 主要预期用户

- [用户群体 1]
- [用户群体 2]

### 超出范围的用途

- [模型不适用的用例 1]
- [模型不适用的用例 2]

## 训练数据

> 为实现 SBOM 互操作性，建议与 CycloneDX `modelCard.modelParameters` 字段保持一致。

| 字段 | 值 |
|------|-----|
| 数据集名称 | [训练数据集名称] |
| 来源 | [数据获取来源] |
| 规模 | [样本数量、token 数量或存储大小] |
| 收集方法 | [数据收集方式] |
| 预处理 | [已应用的清洗、过滤、增强步骤] |
| 已知局限性 | [数据中的偏见、缺失或质量问题] |
| PII 评估 | [是否包含 PII 及其处理方式] |
| 许可证 | [训练数据适用的许可证] |

## 性能指标

| 指标 | 数值 | 测试数据集 | 置信区间 | 条件 |
|------|:----:|-----------|:--------:|------|
| [Accuracy / F1 / BLEU 等] | [数值] | [数据集名称和划分] | [95% CI 范围] | [条件或配置] |

### 分组评估

> 在适用时按相关子群体分别报告性能。

| 子群体 | 指标 | 数值 | 与整体基线比较 |
|--------|------|:----:|:-------------:|
| [子群体 1] | [指标] | [数值] | [+/- 相对于整体] |

## 偏见与公平性评估

| 人群分组 | 指标 | 性能 | 与基线差异 | 已应用的缓解措施 |
|----------|------|:----:|:---------:|-----------------|
| [分组 1 -- 例如年龄段、性别、种族] | [指标] | [数值] | [+/- 百分比或绝对值] | [缓解措施描述] |

## 环境影响

| 指标 | 数值 | 备注 |
|------|------|------|
| 训练能耗 (kWh) | [数值] | [计算方法或估算来源] |
| 二氧化碳当量（吨） | [数值] | [使用的电网碳排放强度] |
| 使用硬件 | [GPU/TPU 型号及数量] | [云服务商/区域] |
| 训练时长 | [小时/天] | [总计算时间] |
| 推理成本 | [每次请求或每千 token 成本] | [平均/峰值] |
| 区域/电网碳排放强度 | [区域名称] | [gCO2/kWh] |

## 安全性考量

| 关注点 | 评估 | 详情 |
|--------|:----:|------|
| 已知漏洞 | [无/描述] | [CVE 引用或已知问题描述] |
| 对抗鲁棒性 | [Low / Medium / High] | [评估方法和结果] |
| 提示注入风险 | [Low / Medium / High] | [对提示注入的敏感性评估] |
| 数据投毒风险 | [Low / Medium / High] | [训练数据完整性评估] |
| 模型提取风险 | [Low / Medium / High] | [模型权重或行为被提取的风险] |

## 伦理考量

- **是否使用敏感数据**：[训练中是否使用了敏感或个人数据及其处理方式]
- **训练中是否涉及人类受试者**：[数据收集中是否涉及人类受试者；IRB 或伦理委员会审查状态]
- **双重用途潜力**：[模型是否可能被挪用于有害用途；已采取的保障措施]
- **社会影响评估**：[更广泛的社会影响——正面和负面]

## 局限性与建议

### 已知局限性

- [局限性 1 -- 例如在特定语言或领域上性能不佳]
- [局限性 2 -- 例如上下文窗口限制]
- [局限性 3 -- 例如在特定场景下倾向于产生幻觉]

### 失败模式

- [失败模式 1 -- 模型可预测地失败的条件]
- [失败模式 2 -- 边界情况或对抗性输入]

### 部署者建议

- [建议 1 -- 例如实施输出过滤]
- [建议 2 -- 例如为高风险决策设置人工审核环节]
- [建议 3 -- 例如随时间监测模型漂移]

---

## 审批

| 字段 | 值 |
|------|-----|
| 审批人 | [姓名] |
| 日期 | [YYYY-MM-DD] |
| 决定 | [APPROVED / REJECTED / CONDITIONAL] |
| 条件 | [如适用] |

<!-- Template: StrayMark | https://strangedays.tech -->
