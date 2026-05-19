# Charter 链演化 — StrayMark

> 两个互补的模式，用于保持多 Charter 模块的诚实性：在 Charter 被声明之前刷新 SpecKit 工件；以及当审计发现在 `status: closed` 之后到达时，对已关闭的 Charter 进行修订。

**语言**: [English](../../CHARTER-CHAIN-EVOLUTION.md) | [Español](../es/CHARTER-CHAIN-EVOLUTION.md) | 简体中文

---

## 状态

**v0 — 已在 N=1 领域验证**（`StrangeDaysTech/sentinel` CHARTER-18, 2026-05-15, Issue #156）。

两个模式都是作为框架规范指南记录在此的约定。CLI 提供只读和脚手架辅助工具（`straymark charter refresh-suggest`、`straymark charter amend`）；模式本身由操作员驱动。一旦第二个采用者验证，任一模式都可能演化 —— 在那之前，N=1 领域警告适用（原则 #12）。

---

## 本文档存在的原因

StrayMark 的 Charter 模式（`STRAYMARK.md` §15）假设单个 Charter 是有界的工作单元。对孤立的 Charter 而言这是有效的。对链中*第一个* Charter 也是有效的。但当一个模块在数月内累积许多用户故事 Charter 时，会浮现两种逐 Charter 模式无法解决的失败模式：

1. **链级别的规范漂移** —— SpecKit 工件（`plan.md`、`data-model.md`、`contracts/*`、`quickstart.md`、`research.md`）是针对链开始时模块的框架版本和现实编写的。在 3+ Charter 之后，累积的学习成果（提取的可重用模式、发现的代码缺口、演化的框架约定、批准的操作员决策）已使规范偏离了实现。直接从 Charter-N 关闭进入 Charter-(N+1) 声明会导致系统性的执行中作用域扩张和涌现的 `R<N+1> (new, not in Charter)` 条目。
2. **周期级别的审计发现** —— 外部审计周期在关闭后运行（审计员在关闭仪式后异步执行）。Critical 或 High 发现可在 Charter 被标记为 `status: closed` 之后到达。框架的选项是：(a) 为补救开新 Charter（重 —— 为 ~5 个文件编辑做完整声明 + Tasks + 仪式），或 (b) 将发现留在 `review.md` 中并失去"与 Charter 原子化"的属性。

模式 1 解决 (1)。模式 2 解决 (2)。两者组合 —— *接受过*模式 1 的 Charter 更可能*避免*模式 2，因为刷新吸收了审计本应在关闭后浮现的执行前风险。它们是互补的，而非可替代的。

---

## 模式 1 —— 预声明 SpecKit 刷新

### 何时适用此模式

当 SpecKit 驱动的模块满足**全部**以下条件时采用此模式：

- 模块有**3 个或更多已关闭的 Charter**（链长 ≥ 3）。
- 最近 3 个已关闭 Charter 的 `charter_telemetry.agent_quality.r_n_plus_one_emergent_count` 滚动均值**大于 6**。
- 自链最近的分支点以来，没有刷新 PR 落入 SpecKit 工件。

运行 `straymark charter refresh-suggest <module>` 对你的 `.telemetry.yaml` 历史评估启发式。CLI 读取所命名模块的最近关闭的 Charter 并打印建议；不会进行任何变更。

低于阈值时，仅逐 Charter 模式就足够 —— 过早采用刷新会增加一个 PR 的开销而没有回报。

### 形状

一个**专用的刷新 PR** 在 Charter-N 关闭和 Charter-(N+1) 声明之间落地。它只触及 SpecKit 工件的**非锁定部分**：

- `specs/<module>/plan.md` —— 阶段计划、依赖说明、排序。
- `specs/<module>/data-model.md` —— 实体、字段、约定。
- `specs/<module>/contracts/*.md` —— 接口契约、请求/响应形状。
- `specs/<module>/quickstart.md` —— 可运行场景。
- `specs/<module>/research.md` —— 累积的知识（见下方的"分类学习表"）。

`research.md` 承载着关键工件：一个**分类学习表**，整合链所学到的内容。最小桶：

| 桶 | 此处放什么 |
|---|---|
| 可重用模式 | 跨 Charter 涌现的、应向前继承的惯用法 / 实用程序 / 包装器（例如 `withRLS` 包装器、品牌缓存 LRU、去重表模式）。 |
| 代码缺口 | 链发现但未关闭的已识别但未修复的工作（例如未接线的表、桩实现、缺失的列）。每个缺口是一个带描述 + 拥有者 Charter（当前或未来）的 `Gn` 条目。 |
| 纪律模式 | 链批准的过程学习（例如跨家族审计对、batch-complete 纪律、每批次关闭节奏）。 |
| 经验校正 | 规范偏离实现的地方。`EC1...ECn` 条目：规范说 X，现实是 Y，选择的协调。 |

可选的**操作员决策（Dn）**在预声明时被批准，包含：决策、考虑的备选方案、选择的路径、理由。后续 Charter 将 Dn 作为契约继承。

### 机制

1. **刷新 PR** 在下一个 Charter 声明之前。可选的 AIDEC 记录刷新决策 + 考虑的备选方案。PR 标题应明确作用域（例如 `spec(<module>): US<n> plan refresh — LOCKED-aware Phase 7+8 redesign`）。
2. **分类学习表** 在 `research.md` 中，包含上述四个桶。每个条目有稳定的 id（Pn / Gn / DPn / ECn），以便后续 Charter 可按 id 引用。
3. **操作员决策（Dn）** 如适用 —— 明确列出，带备选方案 + 选择的路径 + 理由。
4. **下一个 Charter 的 `## Context` 部分** 按 id 引用每个模式、校正和决策。Charter 作用域基于刷新后的现实，而非链开始的规范。

### 遥测

在*下一个* Charter 的遥测中填充 `charter_telemetry.pre_declare_refresh:`（消费刷新的那个，而非刷新 PR 本身）：

```yaml
pre_declare_refresh:
  enabled: true
  refresh_pr: "owner/repo#76"
  refresh_aidec: "AIDEC-YYYY-MM-DD-NNN-speckit-refresh"
  reusable_patterns_integrated: 7
  code_gaps_integrated: 4
  discipline_patterns_integrated: 3
  empirical_corrections_integrated: 15
  operator_decisions_ratified: 3
```

若未发生刷新，则整个块省略 —— 不在场意味着"模式未使用"。

### 为何有效（经验性）

Sentinel CHARTER-18 是 7-Charter 链中第一个无需执行中补救 Charter 即可干净关闭的 Charter。`estimation_drift_factor: 1.0`、`pre_work.items_discovered_during_planning: 0`、`overall_satisfaction: 5/5`。操作员的漂移原因陈述：*"来自 PR #76 的 SpecKit 刷新 ... 消除了在先前 Charter 中驱动漂移的大部分歧义。无需执行中补救 Charter —— research.md 中的 EC1..EC15 经验校正清单将本会是执行前风险的东西吸收为执行中觉察。"*

---

## 模式 2 —— 关闭后审计驱动的修订（Batch N.4）

### 何时适用此模式

当 Charter 被标记 `status: closed` 之后满足**全部**以下条件时采用此模式：

- 在关闭后的 `review.md` 中浮现一个或多个被评定为 **Critical** 或 **High** 的外部审计发现。
- Charter 的 `closure_criterion` 由于未补救的发现而实质上未满足（即按现状发布将使关闭无效）。
- 修复表面适合**一个内聚的 PR**（~< 25 个文件，无架构重新打开 —— 无新抽象、无迁移、无 API 破坏）。

如果修复表面更大或属架构性，请改为开新 Charter。修订模式是为有界情形而存在的；它不是 Charter 规避机制。

### 形状

修订搭载在**与原 Charter 相同的 execute 分支**上（分支仍可合并到 `main`；修订提交落在其上）。一个**新 AILOG** 记录修订 —— 不是原 AILOG 的编辑。

```
charter-<N>-execute 分支
├── （原始提交 —— Charter execute 工作）
├── 提交 X：charter close（status: closed，写入 telemetry.yaml）
└── 提交 Y：charter-<N>(batch-7.4): audit-driven remediation —— <简短摘要>
    ↑
    AILOG-YYYY-MM-DD-MMM（新）记录此提交
    AILOG-YYYY-MM-DD-NNN（原）获得一个 `## Historical correction` 子节
                                  向前指向 AILOG-...-MMM
```

### 机制

1. **相同的 execute 分支** —— 不要从 `main` 分叉。原 Charter 的 execute 分支仍是单元；修订提交搭载在其上。
2. **新 AILOG** 在 `.straymark/07-ai-audit/agent-logs/` 下记录修订。约定：`risk_level: high` 且 `review_required: true`。新 AILOG 携带一个 `amends:` 字段指回原 AILOG id。
3. **原 AILOG 中的历史校正** —— 在原 AILOG 末尾追加一个 `## Historical correction (YYYY-MM-DD)` 子节，含指向新 AILOG 的前向指针。审计决策与执行决策不同；原始的主体保持完整作为历史记录。
4. **PR 评论** —— 如果 execute PR 尚未合并，将修订提交添加到其中并以"Batch N.4 amendment"子节更新 PR 描述，列出已关闭的发现。如果 PR 已合并，开一个后续 PR 引用原 PR 和 AILOG。
5. **遥测** —— 填充 `charter_telemetry.post_close_amendment:`（见下）。使用 `straymark charter audit <id> --merge-reports --merge-into <telemetry-yaml>` 将外部审计发现合并到同一文件；CLI 在 v0.2+ 容忍 `external_audit: []` 占位符重写。

`straymark charter amend <id>` 为步骤 2、3 和 5 做脚手架（创建新 AILOG 桩，编辑原 AILOG 加上 Historical correction 子节，打印 YAML 块）。不触碰 git —— 操作员决定何时提交。

### 遥测

在 Charter 的 `.telemetry.yaml` 中填充 `charter_telemetry.post_close_amendment:`：

```yaml
post_close_amendment:
  applied: true
  trigger: "external_audit"           # external_audit | production_incident | deferred_implementation
  ailog_id: "AILOG-YYYY-MM-DD-MMM"    # 新 AILOG，而非原始的
  findings_closed: 5
  files_modified: 19
  effort_hours: 6.0
```

若未发生修订，则整个块省略。

### 为何有效（经验性）

Sentinel CHARTER-18 在 2026-05-15 关闭，带 `external-audit-pending.yaml`。审计报告于 2026-05-15..05-17 到达。五个发现（来自 `gpt-5.3-codex` 的 4 个 Critical/High、来自 `gemini-2.5-pro` 的 1 个 Critical、校准员发现的 1 个 Medium）是代码级修复 —— DI 接线、重试 header 解析、多租户过滤器、超时默认值。Batch 7.4 修订在一个内聚提交中关闭了所有五个（19 个文件，+2257/-106 行）。新 Charter 将为 ~6 小时的聚焦工程创建多周治理开销。

---

## 跨模式组合

两个模式在链的不同层级运作并组合：

| 模式 | 层级 | 频率 | 吸收 |
|---|---|---|---|
| 预声明 SpecKit 刷新 | 链 / 模块 | 每 3+ Charter 一次 | 规范级漂移（架构假设、表命名、框架版本演化） |
| 关闭后审计驱动修订 | 周期 / Charter | 触发时按 Charter | 运行时级漂移（DI 接线、重试语义、多租户过滤器） |

*接受过*模式 1 的 Charter 更可能*避免*模式 2 —— 刷新吸收了否则将作为关闭后发现浮现的执行前风险。但 CHARTER-18 *两者*都需要 —— 刷新处理规范级漂移；修订处理刷新无法触及的运行时级漂移。在链层级鼓励模式 1；在周期层级容忍模式 2。

---

## 用于上游新模式的权威 / 接受流程

本文档本身是 Sentinel 为这两个模式走过的接受流程的输出（Issue [#156](https://github.com/StrangeDaysTech/straymark/issues/156)）。上游新 Charter-链模式的标准流程是：

1. **采用者本地 RFC** 位于采用者自己树中的 `.straymark/06-evolution/<name>-rfc.md`。采用者先在那里发布模式 —— N=1 证据是必要的但不充分的。
2. **上游 Issue** 在 `StrangeDaysTech/straymark` 镜像本地 RFC 主体，带遥测引用和 PR 链接。
3. **上游接受** 以以下形式落地：(a) 此处 `00-governance/` 中描述模式规范的文档，(b) 遥测 schema 添加（opt-in），(c) 操作员面向机制的可选 CLI 脚手架。N=1 领域警告携带至 v1 稳定化。
4. **第二领域验证** 在模式的 schema 字段从可选毕业到推荐之前。

`06-evolution/` 是飞行中 RFC 的标准采用者本地归宿。上游接受后，标准归宿是 `00-governance/<NAME>.md` —— 此文档实例化的约定。

---

## 开放问题

- **阈值调优** —— `r_n_plus_one_emergent_count` 滚动均值 6 的阈值源于 Sentinel。第二个领域可能移动它。CLI `straymark charter refresh-suggest` 暴露 `--threshold N` 供采用者校准。
- **模块启发式** —— `refresh-suggest <module>` 目前将 `<module>` 与 Charter 标题和 slug 匹配。SpecKit 约定模块（`specs/<NNN>-<module>/`）可在未来的 fw 升级中通过 Charter 的 `originating_spec` 字段提供更严格的绑定。
- **修订频率上限** —— 模式 2 受限于"一个内聚的 PR"。随时间接收两个或更多修订提交的 Charter 应被重新评估为原始关闭过早的信号。

---

## 相关

- [EMERGENT-OBSERVATION-DESIGN.md](EMERGENT-OBSERVATION-DESIGN.md) —— 元模式，Pattern 1 和 Pattern 2 都是其应用（形式化交叉引用 + 浮现的文化许可）。
- [STRAYMARK.md §15](../../../STRAYMARK.md) —— Charter 生命周期及本文档扩展的逐 Charter 模式。
- [SPECKIT-CHARTER-BRIDGE.md](SPECKIT-CHARTER-BRIDGE.md) —— SpecKit 工件如何映射到 Charter；模式 1 生活在此接缝上。
- [FOLLOW-UPS-BACKLOG-PATTERN.md](FOLLOW-UPS-BACKLOG-PATTERN.md) —— 跨许多 AILOG 累积的 `§Follow-ups` 的姐妹模式。
- [`.straymark/schemas/charter-telemetry.schema.v0.json`](../../schemas/charter-telemetry.schema.v0.json) —— `pre_declare_refresh` 和 `post_close_amendment` 在此定义。

---

*StrayMark fw-4.17.0 | [GitHub](https://github.com/StrangeDaysTech/straymark) | Issue [#156](https://github.com/StrangeDaysTech/straymark/issues/156)*
