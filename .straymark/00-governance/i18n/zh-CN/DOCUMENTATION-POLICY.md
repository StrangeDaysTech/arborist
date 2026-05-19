# 文档策略 - StrayMark

**语言**: [English](../../DOCUMENTATION-POLICY.md) | [Español](../es/DOCUMENTATION-POLICY.md) | 简体中文

## 本策略为何存在

StrayMark 将资深软件工程工作的认知纪律——明确的范围、声明的决策、命名的风险、记录的备选方案、可审计的轨迹——外化为与代码并存的版本化文件。本策略定义了使这种纪律可被审计的文档类型、元数据和治理规则。

作为产出这些文物的副作用，项目会积累与主流 AI 治理框架清晰映射的证据：

- **ISO/IEC 42001:2023** — AI 管理系统的核心标准
- **EU AI Act**（2026年8月生效）— 风险分类、透明度、事件报告
- **NIST AI RMF 1.0 + AI 600-1** — AI 风险管理功能和生成式 AI 配置文件
- **ISO/IEC 23894:2023** — AI 风险管理框架
- **GDPR** — 数据保护和隐私影响评估

本策略首先是为工程工作而写；当工作以纪律方式被记录时，合规便是自然而然的副产品。完整标准参考请见第 8 节，产品层面的论据则见上游仓库的 `Propuesta/straymark-design-principles.md`。

---

## 1. 文件命名约定

### 标准格式

```
[TYPE]-[YYYY-MM-DD]-[NNN]-[description].md
```

| 组成部分 | 说明 | 示例 |
|-----------|------|------|
| `TYPE` | 文档类型前缀 | `AILOG`, `AIDEC`, `ADR` |
| `YYYY-MM-DD` | 创建日期 | `2025-01-27` |
| `NNN` | 当天的序列号 | `001`, `002` |
| `description` | 简要描述（kebab-case 格式） | `implement-oauth` |

### 示例

```
AILOG-2025-01-27-001-implement-oauth.md
AIDEC-2025-01-27-001-testing-framework-selection.md
ADR-2025-01-27-001-microservices-architecture.md
REQ-2025-01-27-001-user-authentication.md
```

---

## 2. 必需的元数据（Frontmatter）

所有文档必须在开头包含 YAML 元数据：

```yaml
---
id: AILOG-2025-01-27-001
title: OAuth Authentication Implementation
status: draft | accepted | deprecated | superseded
created: 2025-01-27
updated: 2025-01-27
agent: claude-code-v1.0
confidence: high | medium | low
review_required: true | false
risk_level: low | medium | high | critical
tags:
  - auth
  - security
related:
  - ADR-2025-01-20-001
  - REQ-2025-01-15-003
---
```

### 必填字段

| 字段 | 说明 |
|------|------|
| `id` | 唯一标识符（与文件名相同，不含 .md） |
| `title` | 描述性标题 |
| `status` | 当前文档状态 |
| `created` | 创建日期 |
| `agent` | 创建文档的代理标识符 |
| `confidence` | 代理的置信度级别 |
| `review_required` | 是否需要人工审核 |
| `risk_level` | 变更风险级别 |

### 可选字段

| 字段 | 说明 | 使用时机 |
|------|------|----------|
| `updated` | 最后更新日期 | 任何更新时 |
| `tags` | 分类标签（参见以下约定） | 始终建议使用 |
| `related` | 关联文档的引用（参见以下约定） | 存在交叉引用时 |
| `supersedes` | 本文档替代的文档 ID | 替代其他文档时 |
| `superseded_by` | 替代本文档的文档 ID | 由替代文档设置 |
| `eu_ai_act_risk` | EU AI Act 风险分类：`unacceptable \| high \| limited \| minimal \| not_applicable` | 当变更涉及 EU AI Act 管辖的 AI 系统时 |
| `nist_genai_risks` | NIST AI 600-1 风险类别：`[privacy, bias, confabulation, ...]` | 当变更涉及生成式 AI 组件时 |
| `iso_42001_clause` | ISO 42001 条款：`[4, 5, 6, 7, 8, 9, 10]` | 映射到 ISO 42001 控制措施时 |
| `lines_changed` | 变更行数（可自动计算） | 在 AILOG 文档中 |
| `files_modified` | 修改的文件列表（可自动计算） | 在 AILOG 文档中 |
| `observability_scope` | OTel 埋点级别：`none \| basic \| full` | 当变更涉及可观测性埋点时 |
| `api_spec_path` | OpenAPI/AsyncAPI 规范文件路径 | 在 REQ 文档中，当需求涉及 API 接口时 |
| `api_changes` | 受影响的 API 端点列表 | 在 ADR 文档中，当决策修改公共 API 时 |
| `reviewed_by` | 人工审批人的身份（邮箱、GitHub 用户或 DID） | 由审批人在正式批准 `review_required: true` 文档时填写 |
| `reviewed_at` | 正式批准日期（`YYYY-MM-DD`，必须 ≥ `created`） | 与 `reviewed_by` 一同设置 |
| `review_outcome` | 闭环信号：`approved \| revisions_requested \| rejected` | 与 `reviewed_by` 一同设置。其存在即为"人工已审"的规范信号——见下方 §3.5 |

### 标签约定

标签是用于分类和搜索的**自由格式关键词**。它们有助于在项目中发现相关文档。

**格式规则：**
- 使用 **kebab-case**（小写，连字符分隔）：`gnome-integration`、`sqlite`、`api-design`
- 每个标签一个概念 — 避免复合标签，如 `auth-and-security`
- 建议范围：每个文档 **3 到 8 个标签**
- 标签应描述文档的**主题**、**技术**、**组件**或**关注点**

**示例：**
```yaml
tags: [sqlite, persistence, hexagonal-architecture, repository-pattern]
```

### 关联文档约定

关联引用将文档链接到同一项目中的其他 **StrayMark 文档**。它们支持在 `straymark explore` 等工具中进行交叉导航。

**格式规则：**
- 使用**文档文件名**（含 `.md` 扩展名）：`AILOG-2026-02-03-001-implement-sync-item.md`
- 对于治理文档或非类型化文档，直接使用文件名：`AGENT-RULES.md`、`DOCUMENTATION-POLICY.md`
- 路径相对于 `.straymark/` 解析 — 如果文档在子目录中，请包含从 `.straymark/` 开始的路径：`07-ai-audit/agent-logs/daemon/AILOG-2026-02-03-001-implement-sync-item.md`
- 当文件与引用文档在同一目录时，文件名即可
- **不要使用**外部任务 ID（`T001`、`US3`）、issue 编号或 URL — 这些属于文档正文，不属于 frontmatter
- **不要使用**不含描述的部分 ID（优先使用 `AILOG-2026-02-03-001-implement-sync-item.md` 而非 `AILOG-2026-02-03-001`）

**示例：**
```yaml
# 同一目录或已知位置 — 文件名即可
related:
  - AIDEC-2026-02-02-001-sqlite-bundled-vs-system.md
  - AGENT-RULES.md

# 特定子目录中的文档 — 包含从 .straymark/ 开始的路径
related:
  - 07-ai-audit/agent-logs/daemon/AILOG-2026-02-03-001-implement-sync-item.md
  - 02-design/decisions/ADR-2026-01-15-001-use-hexagonal-architecture.md
```

**解析规则：** CLI 按以下顺序解析引用：(1) 精确 ID 匹配，(2) `.straymark/` 中任意位置的文件名匹配，(3) 路径后缀匹配。使用完整文件名可提供最可靠的解析。

---

## 3. 文档状态

```
identified ──┐
             ├──► draft ──────► accepted ──────► deprecated
             │                       │                   │
             │                       │                   ▼
             │                       └──────► superseded
             │
             └──► (TDE 专用入口状态，见 §6)
                                      │
                                      ▼
                                  resolved
                                  (TDE 专用终态——债务已偿；见 §6)
```

| 状态 | 说明 |
|------|------|
| `identified` | 由代理驱动发现的类型的入口状态（今日仅 TDE）。在生命周期校验上等同于 `draft`——期望人工审阅者来排定优先级并推进。语义上有所区分，以便 adopter 的分析能够区分"代理发现了此债务"与"人工正在起草一份有意识的文档"。 |
| `draft` | 草稿中，待审核 |
| `accepted` | 已批准且为当前有效版本 |
| `resolved` | **TDE 专用终态**：本文档所述的技术债务已被解决；文档作为审计历史保留在磁盘上。区别于 `accepted`（"我们接受这笔债务继续存在"）、`superseded`（"另一份 TDE 取代了它"）和 `deprecated`（"TDE 这一概念本身已不再适用"）。规范的关闭引用（偿清债务的 Charter、PR 或 commit）写入 `## 解决记录` 章节。 |
| `deprecated` | 已废弃，但保留作为参考 |
| `superseded` | 已被其他文档替代 |

按类型的默认 status 映射位于 §6——大多数类型以 `draft` 或 `accepted` 进入，但 TDE 因代理自主权边界（代理识别、人工排序）以 `identified` 进入。TDE 是今日唯一拥有自定义终态（`resolved`）的类型；验证器全局接受 `resolved` 作为过渡安排。未来的按文档类型生命周期词汇表（issue #149 选项 B）将把 `resolved` 严格限定到 TDE；在此之前，将其用于非 TDE 文档可通过验证，但语义上不正确。

---

## 3.5 审批记录

`status` 记录文档的生命周期状态，`review_required: true` 记录*需要人工审核*。这两个字段都不记录人工审核*实际是否发生*。本节定义了需要正式审批的文档（AIDEC、ETH、MCARD、ADR、DPIA、INC、SEC 以及中国 scope 变体——触发条件见 AGENT-RULES.md §4）的规范闭环信号。

### 闭环信号

三个可选的 frontmatter 字段，由审批人在批准时填写：

```yaml
reviewed_by: pepe@example.com           # 邮箱 | github 用户 | DID
reviewed_at: 2026-05-02
review_outcome: approved                # approved | revisions_requested | rejected
```

语义：

- **`review_outcome` 的存在即为闭环信号。** 一份 `review_required: true` 且无 `review_outcome` 的文档处于*待审核*状态。
- 批准之后**不**将 `review_required: true` 改为 `false` —— 它作为"为何最初需要审核"的历史记录保留下来。
- `reviewed_at` 必须 `>= created`。如果 `reviewed_by` 已设置，`reviewed_at` 与 `review_outcome` 也必须设置（由 `straymark validate` 校验）。
- `review_outcome: revisions_requested` 支持迭代审核循环：文档被更新，审批人最终再次批准。约定是用最新一次的审批覆盖三个字段（frontmatter 仅保留最新状态）；下方 body 章节保留历史。

### Body 章节（规范的散文形式）

添加在文档正文的末位（例如：AIDEC/ADR 的 `## References` 之前；DPIA 的 `## Review Schedule` 之后；INC 的 `## Post-Mortem Review` 之后）。对于已包含 `## Approval` 表格形式的模板（ETH、MCARD、SEC、PIPIA、CACFILE、TC260RA、AILABEL），任一形式都是规范的；frontmatter 字段才是机器可读的真相源。

```markdown
## Approval

**Approved**: 2026-05-02 by `pepe@example.com`.

<可选的审批人备注——观察、条件、批准范围。如果没有需要补充于 frontmatter
之外的内容，可整节省略。>
```

### 多审批人流程（前瞻性）

对于需要多审批人的文档（如 ETH 同时需要法务和工程的 sign-off），v1 的规范是在 body 中按时间顺序追加多个 `## Approval` 块，frontmatter 反映*最新*一次的批准。结构化的 `review:` 数组形式（每个审批人一条）属于前瞻性，不在 v1 中——会等到至少一个真实采用项目以真实数据演练多审批人流程后再加入。

### CLI 工具

`straymark approve <doc-id> --outcome approved --reviewer <id> [--notes "..."] [--at YYYY-MM-DD]` 一次性写入 frontmatter 字段与 body 章节。`straymark validate --check-pending-reviews [--max-pending-days N]` 列出 `review_required: true` 且早于 `N` 天但仍无 `review_outcome` 的文档（warn-only，不报错）。详见 CLI-REFERENCE.md。

---

## 4. 风险级别

| 级别 | 使用场景 | 是否需要审核 |
|------|----------|-------------|
| `low` | 外观修改、文档更新 | 否 |
| `medium` | 新功能、代码重构 | 建议审核 |
| `high` | 安全、敏感数据、公共 API | 是 |
| `critical` | 不可逆变更、生产环境 | 强制审核 |

---

## 5. 置信度级别

| 级别 | 含义 | 操作 |
|------|------|------|
| `high` | 代理对决策确信无疑 | 继续执行 |
| `medium` | 代理有轻微疑虑 | 记录替代方案 |
| `low` | 代理需要验证 | 标记 `review_required: true` |

---

## 6. 目录结构

```
.straymark/
├── 00-governance/          # 策略和规则
├── 01-requirements/        # 系统需求
├── 02-design/              # 设计和架构
│   └── decisions/          # ADR
├── 03-implementation/      # 实施指南
├── 04-testing/             # 测试策略
├── 05-operations/          # 运维
│   └── incidents/          # 事后分析
├── 06-evolution/           # 系统演进
│   └── technical-debt/     # 技术债务
├── 07-ai-audit/            # AI 代理审计
│   ├── agent-logs/         # AILOG
│   ├── decisions/          # AIDEC
│   └── ethical-reviews/    # ETH
├── 08-security/            # SEC — 安全评估
├── 09-ai-models/           # MCARD — 模型/系统卡
└── templates/              # 模板
```

### 文档类型

| 类型 | 名称 | 目录 | 默认状态 | 需要审核 |
|------|------|------|----------|----------|
| `AILOG` | AI 操作日志 | `07-ai-audit/agent-logs/` | `accepted` | 否 |
| `AIDEC` | AI 决策 | `07-ai-audit/decisions/` | `accepted` | 否 |
| `ETH` | 伦理审查 | `07-ai-audit/ethical-reviews/` | `draft` | 是 |
| `ADR` | 架构决策记录 | `02-design/decisions/` | `draft` | 是 |
| `REQ` | 需求 | `01-requirements/` | `draft` | 是 |
| `TES` | 测试计划 | `04-testing/` | `draft` | 是 |
| `INC` | 事故事后分析 | `05-operations/incidents/` | `draft` | 是 |
| `TDE` | 技术债务 | `06-evolution/technical-debt/` | `identified`（在此进入；债务偿清时 `resolved` 为终态——仅 TDE） | 否 |
| `SEC` | 安全评估 | `08-security/` | `draft` | 是（始终） |
| `MCARD` | 模型/系统卡 | `09-ai-models/` | `draft` | 是（始终） |
| `SBOM` | 软件物料清单 | `07-ai-audit/` | `accepted` | 否 |
| `DPIA` | 数据保护影响评估 | `07-ai-audit/ethical-reviews/` | `draft` | 是（始终） |

---

## 7. 交叉引用

使用 `[TYPE-ID]` 格式进行引用：

```markdown
此决策基于 [REQ-2025-01-15-003] 中定义的需求。
另请参阅 [ADR-2025-01-20-001] 了解架构背景。
```

---

## 8. 参考标准

| 标准 | 版本 | 在 StrayMark 中的范围 |
|------|------|---------------------|
| ISO/IEC/IEEE 29148:2018 | 2018 | 需求工程 — TEMPLATE-REQ.md |
| ISO/IEC 25010:2023 | 2023 | 软件质量模型 — TEMPLATE-REQ.md, TEMPLATE-ADR.md |
| ISO/IEC/IEEE 29119-3:2021 | 2021 | 软件测试文档 — TEMPLATE-TES.md |
| ISO/IEC 42001:2023 | 2023 | AI 管理系统 — AI-GOVERNANCE-POLICY.md（核心标准） |
| EU AI Act | 2024（2026年8月生效） | AI 法规 — ETH, INC, AILOG 监管字段 |
| NIST AI RMF 1.0 | 2023 | AI 风险管理 — ETH, AILOG 风险类别 |
| NIST AI 600-1 | 2024 | 生成式 AI 配置文件 — ETH/AILOG 中的 12 个风险类别 |
| ISO/IEC 23894:2023 | 2023 | AI 风险管理 — AI-RISK-CATALOG |
| GDPR | 2016/679 | 数据保护 — ETH（数据隐私）, DPIA |
| OpenTelemetry | 当前 | 可观测性 — OBSERVABILITY-GUIDE，可选 |

---

*StrayMark fw-4.17.0 | [Strange Days Tech](https://strangedays.tech)*
