---
id: SBOM-YYYY-MM-DD-NNN
title: "[系统/组件] AI 软件物料清单"
status: accepted
created: YYYY-MM-DD
agent: [agent-name]
confidence: high
review_required: false  # 事实性清单
risk_level: low
iso_42001_clause: [8]
sbom_format_reference: SPDX-3.0 | CycloneDX-1.6 | custom
system_name: ""
tags: [sbom, supply-chain]
related: []
---

# SBOM: [系统/组件] AI 软件物料清单

## AI/ML 组件

> 本节对应 CycloneDX `component`，类型为 `type: machine-learning-model`。

| 组件名称 | 版本 | 供应商 | 类型 | 许可证 | 风险等级 | 漏洞状态 | 最近审计日期 |
|----------|------|--------|------|--------|----------|----------|-------------|
| [组件 1] | [x.y.z] | [供应商] | model | [许可证] | [Low/Med/High] | [Clean/Vulnerable] | [YYYY-MM-DD] |
| [组件 2] | [x.y.z] | [供应商] | library | [许可证] | [Low/Med/High] | [Clean/Vulnerable] | [YYYY-MM-DD] |
| [组件 3] | [x.y.z] | [供应商] | service | [许可证] | [Low/Med/High] | [Clean/Vulnerable] | [YYYY-MM-DD] |
| [组件 4] | [x.y.z] | [供应商] | dataset | [许可证] | [Low/Med/High] | [Clean/Vulnerable] | [YYYY-MM-DD] |

## 训练数据来源

> 符合 ISO 42001 附录 A.7（AI 系统数据）。

| 数据集 | 来源 | 许可证 | 包含个人信息 | 偏差评估摘要 | 数据溯源 | 保留策略 |
|--------|------|--------|-------------|-------------|----------|----------|
| [数据集 1] | [来源] | [许可证] | [是/否] | [摘要] | [溯源信息] | [策略] |
| [数据集 2] | [来源] | [许可证] | [是/否] | [摘要] | [溯源信息] | [策略] |

## 第三方 AI 服务

| 服务 | 供应商 | 用途 | 共享数据 | 已签署数据处理协议 | SLA | 区域 | 合规认证 |
|------|--------|------|----------|-------------------|-----|------|----------|
| [服务 1] | [供应商] | [用途] | [数据类型] | [是/否] | [SLA 条款] | [区域] | [SOC2, ISO 27001 等] |
| [服务 2] | [供应商] | [用途] | [数据类型] | [是/否] | [SLA 条款] | [区域] | [SOC2, ISO 27001 等] |

## 软件依赖

> 建议使用 `syft` 或 `trivy` 等工具自动生成本节内容。

| 软件包 | 版本 | 许可证 | 已知漏洞 | 最近更新 |
|--------|------|--------|----------|----------|
| [软件包 1] | [x.y.z] | [许可证] | [CVE-YYYY-NNNNN, ...] | [YYYY-MM-DD] |
| [软件包 2] | [x.y.z] | [许可证] | [无] | [YYYY-MM-DD] |
| [软件包 3] | [x.y.z] | [许可证] | [CVE-YYYY-NNNNN] | [YYYY-MM-DD] |

## 供应链风险评估

> 符合 NIST AI 600-1 第 12 类：价值链与组件集成。

- **总体风险等级**: [Low/Medium/High/Critical]

- **已识别的主要风险**:
  - [风险 1：描述]
  - [风险 2：描述]
  - [风险 3：描述]

- **缓解措施**:
  - [缓解措施 1：描述]
  - [缓解措施 2：描述]
  - [缓解措施 3：描述]

- **监控计划**:
  - [监控活动 1：频率和责任方]
  - [监控活动 2：频率和责任方]
  - [监控活动 3：频率和责任方]

<!-- Template: StrayMark | https://strangedays.tech -->
