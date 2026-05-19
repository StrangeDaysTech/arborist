---
id: TDE-YYYY-MM-DD-NNN
title: [技术债务标题]
status: identified                # `identified` → `resolved` 当债务偿清时（仅 TDE 的终态）
created: YYYY-MM-DD
agent: [agent-name-v1.0]
confidence: high | medium | low
review_required: false
risk_level: low | medium | high
type: code | architecture | infrastructure | documentation | testing
impact: low | medium | high
effort: low | medium | high
iso_42001_clause: []            # 4 | 5 | 6 | 7 | 8 | 9 | 10
tags: []
related: []
priority: null
assigned_to: null
promoted_from_followup: null    # FU-NNN，如果从 .straymark/follow-ups-backlog.md 提升而来
---

# TDE: [技术债务标题]

> **由代理识别**：优先级排序和任务分配需要人工决策。
>
> **激活触发条件**（任意一条满足即可——若均不满足，请改在 AILOG 中以 `R<N> (new, not in Charter)` 记录）：先前 Charter 的遗留、横跨多个模块/Charter、需要当前 scope 包络之外的专用 Charter、或需要代理无法独自决定的人工优先级或分配。完整判定请参见 `.straymark/00-governance/AGENT-RULES.md` §3。

## 摘要

[已识别技术债务的简要描述]

## 债务类型

- [ ] **代码**：难以维护、重复或结构不良的代码
- [ ] **架构**：次优的架构决策
- [ ] **基础设施**：有问题的配置或依赖
- [ ] **文档**：缺失或过时的文档
- [ ] **测试**：覆盖率不足或脆弱的测试

## 位置

| 文件/组件 | 描述 |
|-----------|------|
| `path/to/file.ts` | [问题所在] |
| `path/to/component/` | [问题所在] |

## 问题描述

[详细描述为什么这属于技术债务]

### 观察到的症状
- [症状 1：例如"该文件超过 1000 行"]
- [症状 2：例如"有 5 个功能几乎相同的函数"]

### 原始原因
[该债务产生的原因——如果已知]

## 影响

### 对开发的影响
- [对开发团队的影响]

### 对维护的影响
- [对维护工作的阻碍]

### 对性能的影响（如适用）
- [性能影响]

### 对安全的影响（如适用）
- [安全风险]

## 建议的解决方案

[如何解决该问题的描述]

### 推荐方法
1. [步骤 1]
2. [步骤 2]
3. [步骤 3]

### 替代方案
- [替代方案 1]：[简要描述]
- [替代方案 2]：[简要描述]

## 估算

| 方面 | 值 | 理由 |
|------|-----|------|
| 工作量 | [Low/Medium/High] | [原因] |
| 解决后的影响 | [Low/Medium/High] | [原因] |
| 不解决的风险 | [Low/Medium/High] | [原因] |
| 紧迫性 | [Low/Medium/High] | [原因] |

## 优先级矩阵（供人工参考）

```
         │ 低工作量    │ 高工作量    │
─────────┼─────────────┼─────────────┤
高       │   立即执行   │    规划     │
影响     │             │             │
─────────┼─────────────┼─────────────┤
低       │   快速解决   │    考虑     │
影响     │             │             │
```

## 依赖关系

- [应先解决的其他债务]
- [可能受影响的功能]

## 代理备注

[补充上下文、观察或建议]

---

## 优先级决策

| 字段 | 值 |
|------|-----|
| 优先级决策人 | [姓名] |
| 日期 | [YYYY-MM-DD] |
| 分配的优先级 | [P1/P2/P3/Backlog/Will not resolve] |
| 迭代/里程碑 | [如适用] |
| 分配给 | [团队/个人] |
| 备注 | [说明] |

---

## 解决记录 (Resolution)

> 当此处所述的债务被解决时，填写此章节**并**将 frontmatter 的 `status: identified` 翻转为 `resolved`。
> 保留磁盘上的文档——`resolved` 是 TDE 的规范终态；文件成为审计历史而非被删除。
> 关于生命周期语义，请见 DOCUMENTATION-POLICY.md §3。
>
> 当债务仍为 `identified` / `accepted` / superseded 时，请完全省略此章节——
> 它仅在终态转换时有意义。

| 字段 | 值 |
|------|-----|
| 解决者 | [偿清债务的 Charter ID / PR / commit] |
| 日期 | [YYYY-MM-DD] |
| 验证方式 | [如何验证已解决——测试、drift 检查、审计等] |
| 备注 | [未来读者应当知晓的事项，例如部分解决的范围] |

<!-- Template: StrayMark | https://strangedays.tech -->
