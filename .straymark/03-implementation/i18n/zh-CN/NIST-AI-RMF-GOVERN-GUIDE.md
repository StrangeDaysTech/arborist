# NIST AI RMF --- GOVERN 功能实施指南

> **框架**: NIST AI Risk Management Framework (AI RMF 1.0)
> **功能**: GOVERN --- AI 组织治理结构
>
> GOVERN 功能建立并维护负责任 AI 风险管理所需的组织结构、策略、流程和文化。它是一个跨领域功能，为 MAP、MEASURE 和 MANAGE 功能提供信息，同时也从这些功能中获取信息。

---

## GV-1: AI 治理策略

建立、记录和传达组织策略，定义 AI 开发、部署和使用的期望。策略应当是动态文档，需要定期审查和更新。

> 策略设定组织基准。如果没有明确的策略，团队会默认依赖不一致的个人判断。

### StrayMark 映射

| NIST 要求 | StrayMark 文档 | 部分 / 字段 |
|------------------|-------------------|-----------------|
| AI 治理策略 | AI-GOVERNANCE-POLICY.md | 完整文档 |
| 文档标准 | DOCUMENTATION-POLICY.md | 完整文档 |
| Agent 自主权限制 | AGENT-RULES.md | 自主权表 |
| 伦理原则 | PRINCIPLES.md | 完整文档 |
| 法规合规策略 | AI-GOVERNANCE-POLICY.md | 第 1.3 节（法律和法规要求） |

### 实施检查清单

- [ ] 根据组织的背景、范围和法规环境定制 AI-GOVERNANCE-POLICY.md
- [ ] 采用 DOCUMENTATION-POLICY.md 作为所有 AI 相关文档的标准
- [ ] 配置 AGENT-RULES.md 以反映组织对 AI Agent 自主权的风险容忍度
- [ ] 至少每年或在发生重大变更时审查并更新所有治理策略

---

## GV-2: 问责与角色

为 AI 风险管理定义明确的角色、职责和问责结构。确保每个治理功能都有指定的负责人。

> 没有分配的问责是一种幻觉。每项风险管理活动都需要一个指定的责任人。

### StrayMark 映射

| NIST 要求 | StrayMark 文档 | 部分 / 字段 |
|------------------|-------------------|-----------------|
| 角色与职责 | AI-GOVERNANCE-POLICY.md | 第 2.2 节（角色与职责） |
| Agent 自主权边界 | AGENT-RULES.md | 自主权表、人工审查触发条件 |
| 决策权限 | ADR | 决策者部分 |
| 审查职责 | DOCUMENTATION-POLICY.md | 审查要求部分 |
| 事件问责 | INC | 负责人、响应人员字段 |

### 实施检查清单

- [ ] 完成 AI-GOVERNANCE-POLICY.md 第 2.2 节中的角色与职责表
- [ ] 在 AGENT-RULES.md 中为所有高风险活动定义人工审查触发条件
- [ ] 确保每个 ADR 都标识决策者及其权限
- [ ] 分配事件响应角色并在 INC 模板中记录

---

## GV-3: 劳动力多样性与包容性

确保 AI 开发和治理团队包含多样化的视角。多样化的团队更善于识别风险、偏见和盲点。

> 同质化的团队产生同质化的风险评估。视角的多样性是一种治理控制措施，而不仅仅是人力资源目标。

### StrayMark 映射

| NIST 要求 | StrayMark 文档 | 部分 / 字段 |
|------------------|-------------------|-----------------|
| 团队组成指导 | AI-GOVERNANCE-POLICY.md | 第 4 节（支持与资源） |
| 能力要求 | AI-GOVERNANCE-POLICY.md | 第 4.2 节（能力） |
| 包容性审查流程 | DOCUMENTATION-POLICY.md | 审查要求部分 |
| 利益相关者代表 | ETH | 利益相关者分析部分 |

### 实施检查清单

- [ ] 在 AI-GOVERNANCE-POLICY.md 第 4 节中记录团队组成期望
- [ ] 定义涵盖技术、伦理、法律和领域专业知识的能力要求
- [ ] 确保 ETH 利益相关者分析考虑受影响社区的视角
- [ ] 在高风险文档审查工作流中纳入多样化的审查人员

---

## GV-4: 组织文化

培养重视负责任 AI 开发、鼓励提出关切、支持持续学习 AI 风险和伦理的组织文化。

> 文化决定了策略在实践中是否被遵循，还是仅存在于纸面上。治理的力度取决于维系它的文化。

### StrayMark 映射

| NIST 要求 | StrayMark 文档 | 部分 / 字段 |
|------------------|-------------------|-----------------|
| 组织原则 | PRINCIPLES.md | 完整文档 |
| 伦理准则 | PRINCIPLES.md | 核心原则部分 |
| 透明度期望 | AGENT-RULES.md | 文档要求 |
| 从事件中学习 | INC | 经验教训部分 |

### 实施检查清单

- [ ] 采用并传达 PRINCIPLES.md 作为团队共同的伦理基础
- [ ] 通过将 INC 文档视为学习工具，建立无责文化以鼓励报告 AI 相关问题
- [ ] 使用 PRINCIPLES.md 和 AI-GOVERNANCE-POLICY.md 将 AI 伦理意识融入入职培训
- [ ] 表彰和分享记录在 AILOG 和 ETH 记录中的负责任 AI 实践案例

---

## GV-5: 利益相关者参与

定期让内部和外部利益相关者参与 AI 治理活动。利益相关者的参与有助于改善风险识别、建立信任，并确保治理反映多样化的需求。

> 在封闭环境中进行的治理缺乏预判现实世界影响所需的外部视角。

### StrayMark 映射

| NIST 要求 | StrayMark 文档 | 部分 / 字段 |
|------------------|-------------------|-----------------|
| 管理评审 | MANAGEMENT-REVIEW-TEMPLATE.md | 完整文档 |
| 利益相关者反馈 | MANAGEMENT-REVIEW-TEMPLATE.md | 反馈摘要部分 |
| 外部沟通 | AI-GOVERNANCE-POLICY.md | 第 4.4 节（沟通） |
| 公共透明度 | MCARD | 完整文档（面向公众的模型文档） |
| 评审结果 | MANAGEMENT-REVIEW-TEMPLATE.md | 行动项部分 |

### 实施检查清单

- [ ] 使用 MANAGEMENT-REVIEW-TEMPLATE.md 安排定期管理评审
- [ ] 将利益相关者反馈作为管理评审的常设议程项目
- [ ] 为外部部署的 AI 系统发布 MCARD 文档以支持公共透明度
- [ ] 记录利益相关者参与活动及其成果作为审计证据

---

## GV-6: AI 供应链治理

通过维护对 AI 系统中使用的第三方组件、模型、数据源和服务的透明度来治理 AI 供应链。

> 看不到的东西无法管理。供应链治理始于完整且最新的清单。

### StrayMark 映射

| NIST 要求 | StrayMark 文档 | 部分 / 字段 |
|------------------|-------------------|-----------------|
| 组件清单 | SBOM | 完整文档 |
| 第三方服务 | SBOM | 第三方服务部分 |
| 许可证合规 | SBOM | 许可证列 |
| 供应商风险评估 | ETH | 第三方风险部分 |
| 供应链策略 | AI-GOVERNANCE-POLICY.md | 第 5.3 节（第三方 AI 组件） |

### 实施检查清单

- [ ] 为每个 AI 系统维护最新的 SBOM，包括模型、库、API 和数据源
- [ ] 在 SBOM 许可证列中跟踪所有第三方组件的许可条款
- [ ] 对组件影响高风险功能的关键供应商进行 ETH 评估
- [ ] 在 AI-GOVERNANCE-POLICY.md 第 5.3 节中定义供应链治理要求

---

## 摘要：GOVERN 功能到 StrayMark 映射

| 类别 | 描述 | 主要 StrayMark 文档 | 关键字段 / 部分 |
|----------|-------------|---------------------------|----------------------|
| GV-1 | 策略 | AI-GOVERNANCE-POLICY.md、DOCUMENTATION-POLICY.md | 完整文档 |
| GV-2 | 问责 | AGENT-RULES.md、AI-GOVERNANCE-POLICY.md | 自主权表、第 2.2 节 |
| GV-3 | 劳动力多样性 | AI-GOVERNANCE-POLICY.md | 第 4 节 |
| GV-4 | 组织文化 | PRINCIPLES.md | 核心原则 |
| GV-5 | 利益相关者参与 | MANAGEMENT-REVIEW-TEMPLATE.md | 反馈摘要、行动项 |
| GV-6 | 供应链 | SBOM | 完整文档 |

---

*NIST AI RMF GOVERN 功能实施指南 --- StrayMark Framework*

<!-- Template: StrayMark | https://strangedays.tech -->
