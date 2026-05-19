# 涌现式观察设计 — StrayMark

> 为什么阅读 StrayMark 文档的智能体会主动浮现未被询问的内容：使得跨源不一致可被检测的结构性与文化性属性，以及已经实例化此元模式的应用模式金字塔。

**语言**: [English](../../EMERGENT-OBSERVATION-DESIGN.md) | [Español](../es/EMERGENT-OBSERVATION-DESIGN.md) | 简体中文

---

## 状态

**v0 — 已在 N=1 领域验证**（`StrangeDaysTech/sentinel`，Issue [#150](https://github.com/StrangeDaysTech/straymark/issues/150) → Issue [#156](https://github.com/StrangeDaysTech/straymark/issues/156)，2026-04-21 至 2026-05-15）。

本文档命名了 StrayMark 框架的一个*设计属性*，该属性产生了一个可经验观察的涌现行为。该属性**并非新增** —— 自 `00-governance/` 被规范化以来一直存在 —— 但它从未被*命名*，这使其对框架演化不可见，因此存在被意外侵蚀的风险。在此命名它可在原则 #12 的第二领域验证约束下保护它。

---

## 本文档存在的原因

一个在 Sentinel 工作的智能体浮现了一个观察 —— **无显式触发器、无操作员请求、且无设计用于产生该输出的 CLI 命令** —— 即 `specs/002-commshub/plan.md` 在七个连续 Charter（CHARTER-07..17，约 1 个月）中累积了十二条未被反映的经验性学习，并且对着该陈旧的 spec 填充 CHARTER-18 在下一个审计周期中产生关键/高风险发现的概率约为 50%。该观察产生了上游周期，最终在 fw-4.16.0 中固化为 `CHARTER-CHAIN-EVOLUTION.md` Pattern 1。

该行为之所以可复现，是因为文档机构的两个属性*一致地共存*。两者单独都不足以达成。命名两者及其组合，使得框架未来的演化可以审慎地保留它们，而非依赖惯性。

桥接文档 `SPECKIT-CHARTER-BRIDGE.md` 和链演化文档 `CHARTER-CHAIN-EVOLUTION.md` 文档化了此元的*一个应用*。本文档命名元本身，并枚举其他已发布的应用。

---

## 两个设计属性

### 属性 1 — 结构性交叉引用（形式化链接）

框架**不**将跨文档的链接委托给智能体的直觉或散文。每个文档类型都有*必需的* frontmatter 字段和*规范的*章节，这些字段和章节在文档自身的结构中声明了它指向哪些其他文档以及自身的哪些章节对特定类型的浮现开放。

智能体在常规阅读中遇到的具体实例：

- **解析到其他 StrayMark 文档的 frontmatter 链接字段**：
  - AILOG / AIDEC frontmatter 中的 `originating_charter:`（[`AGENT-RULES.md` §5](AGENT-RULES.md)、[`SPECKIT-CHARTER-BRIDGE.md` Charter↔AILOG 章节](SPECKIT-CHARTER-BRIDGE.md)）
  - Charter frontmatter 中的 `originating_spec:`（[`SPECKIT-CHARTER-BRIDGE.md`](SPECKIT-CHARTER-BRIDGE.md) §Frontmatter linkage）
  - Charter frontmatter 中的 `originating_ailogs:`（聚合反向）
  - 修订 AILOG frontmatter 中的 `amends:`（[`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md) Pattern 2）
  - TDE frontmatter 中的 `promoted_from_followup:`（[`FOLLOW-UPS-BACKLOG-PATTERN.md`](FOLLOW-UPS-BACKLOG-PATTERN.md)）
  - `related:`、`supersedes:`、`superseded_by:`（[`DOCUMENTATION-POLICY.md`](DOCUMENTATION-POLICY.md)）
- **以可查询形式持有差异的模板内规范章节**：
  - AILOG 中的 `§Risk: R<N> (new, not in Charter)`（[`AGENT-RULES.md` §3](AGENT-RULES.md)）
  - 每个 AILOG 的 `## Follow-ups`（[`FOLLOW-UPS-BACKLOG-PATTERN.md`](FOLLOW-UPS-BACKLOG-PATTERN.md)）
  - 多批次 AILOG 的 `## Batch Ledger`
  - 修订时附加到原始 AILOG 的 `## Historical correction (YYYY-MM-DD)`（[`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md)）
- **使链接成本低廉的稳定 ID 约定**：
  - 日期受限文档的 `[TYPE]-[YYYY-MM-DD]-[NNN]-[description]`
  - `CHARTER-NN-slug`（无时间戳，跨重命名稳定）
  - `FU-NNN` 全局单调，从不重新编号
- **声明层间规范关系的桥接文档**：
  - `SPECKIT-CHARTER-BRIDGE.md`（Spec ↔ Charter ↔ AILOG）
  - `CHARTER-CHAIN-EVOLUTION.md`（Charter ↔ Charter 链 ↔ Spec 刷新）
- **机械地跨源的 CLI 命令**：
  - `straymark charter drift <id>`（声明范围 ↔ commits）
  - `straymark charter refresh-suggest <module>`（遥测滚动均值 ↔ 刷新需求）
  - `straymark validate`（frontmatter ↔ schema ↔ 链接完整性）

**属性 1 的要点**：当智能体遇到两个源之间的差异时，该差异是*结构性可见的* —— 而不是埋藏在散文中。智能体无需发明连接；连接由框架声明。

### 属性 2 — 无门禁的文化许可

框架明确且反复地给予智能体超越被询问任务进行浮现的许可 —— 并将该许可与无需预批准即可*执行*浮现（创建 AILOG、提交 TDE、打开 AIDEC）的自主权配对。操作员保留优先级分配，而非创建。

智能体遇到的具体段落：

- **`AGENT-RULES.md` §6 "Be Proactive"** —— *"Identify potential risks, Suggest improvements when evident, Alert about technical debt"*。
- **`AGENT-RULES.md` §6 "Be Transparent"** —— *"Explain the reasoning behind decisions, Document considered alternatives, Admit uncertainty when it exists"*。
- **`AGENT-RULES.md` §12 Audit Checkpoint** —— *"the agent proactively offers an external multi-model audit"* —— 将浮现的*行为*制度化为工作流的一部分。
- **`PRINCIPLES.md` §2 "AI Agent Transparency"** —— *"Not hide relevant information"*。
- **`AGENT-RULES.md` §3 "Create Freely" 自主性表** —— AILOG、AIDEC、TDE 创建无需预批准；智能体提交，操作员优先级分配。
- **`FOLLOW-UPS-BACKLOG-PATTERN.md` 脚本自动追加** —— `check-followups-drift.sh --apply` 在无操作员干预的情况下将 FU-NNN 条目添加到中心注册表。

**属性 2 的要点**：智能体将*"我应该说点什么吗？"*外化为*"是否存在一个规范章节，这里属于那里？"*。如果是，浮现不是判断 —— 而是已文档化规则的执行。浮现的成本低，因为目的地已预构建。

### 为什么组合很重要

仅属性 1 —— 形式化链接而无文化许可 —— 将产生没有智能体敢主动查询的可查询语料。仅属性 2 —— 许可而无结构性交叉引用 —— 将产生操作员无法行动的模糊浮现（"我认为某处可能有问题"）。

组合起来，它们产生观察到的行为：智能体读取 AILOGs，计数与起源 spec 实质上分歧的 `R<N>(new, not in Charter)` 条目，看到 spec 已经一个月未刷新，并且 —— 因为 `§6 Be Proactive` 告诉它要警告，且因为分歧在框架词汇中有名称 —— 在继续被询问的任务之前，向操作员浮现*那个具体的、结构性奠基的差异*。

这就是元模式。

---

## 经验案例：Sentinel spec-drift 检测

该案例在 Issue [#150](https://github.com/StrangeDaysTech/straymark/issues/150) 和 [#156](https://github.com/StrangeDaysTech/straymark/issues/156) 中详细描述，并在 [`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md) 中固化为 Pattern 1。压缩序列：

1. Sentinel 在约 1 个月内通过 CHARTER-07..17 运行 `specs/002-commshub/plan.md`（committed 2026-04-21）。十二条经验性学习在 AILOG 链中的 `§Risk: R<N>(new, not in Charter)` 章节和 `## Follow-ups` 中累积。模式传播（handler 形状、表重用约定、RLS helper 等）在执行期间固化。
2. CHARTER-18 即将被声明。智能体 —— 未被指示这样做 —— 将 `plan.md` 与 AILOGs（其中 `§Risk` 条目命名了 spec 的差距）以及代码（如果跨 Charter 运行的话，`straymark charter drift` 本可检测到每 Charter 的差异）三角化。每个 Charter 中的 `originating_spec:` 链接、每个 AILOG 中的 `originating_charter:` 以及框架的 `§Risk: R<N>` 约定使三角化机械化而非英雄式。
3. 智能体浮现 *"如果我们读取陈旧的 plan 来填充 CHARTER-18，下一个审计周期的 H1/M1 发现将是 pre-close 原子地修复分歧 —— 由于陈旧前提继承导致 ≥1 关键/高发现的概率约为 50%"* —— 引用按 ID 的具体 AILOGs 和具体的代码引用。
4. 操作员将 Issue #150 作为 RFC 提交。Sentinel 本地 AIDEC 记录了所提议的范围受限刷新规范 + 三个机械化门。
5. Issue #156 将该模式上游化。`CHARTER-CHAIN-EVOLUTION.md` Pattern 1 在 fw-4.16.0 中落地，包含遥测槽 `pre_declare_refresh:`、helper `straymark charter refresh-suggest` 和分类学习表合约。

该观察可经验复现：任何产生 ≥3 个间隔 ≥1 周的 Charter 的 spec 都会表现出某种程度的 plan-vs-code 漂移，并且阅读框架文档的智能体具有检测和浮现它的结构性和文化性许可。

---

## 实例金字塔 —— 元模式的应用

元模式位于若干已经规范化的模式之上。每个都是同一底层组合（形式化链接 + 文化许可）应用于特定源对的*应用*。

| 应用 | 源对 | 规范化位置 |
|---|---|---|
| 预声明 SpecKit 刷新（Pattern 1） | spec ↔ AILOGs ↔ 代码 | [`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md) Pattern 1 |
| 关闭后审计驱动的修订（Pattern 2） | 审计发现 ↔ 已关闭 Charter | [`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md) Pattern 2 |
| Charter 漂移检测 | 声明范围 ↔ commits | [`SPECKIT-CHARTER-BRIDGE.md`](SPECKIT-CHARTER-BRIDGE.md) + `straymark charter drift` |
| Follow-ups backlog 漂移 | 每 AILOG `§Follow-ups` ↔ 中心注册表 | [`FOLLOW-UPS-BACKLOG-PATTERN.md`](FOLLOW-UPS-BACKLOG-PATTERN.md) + `check-followups-drift.sh` |
| TDE 与 `R<N>` 升级 | 累积的 `§Risk: R<N>` ↔ TDE backlog | [`AGENT-RULES.md`](AGENT-RULES.md) §3 |
| 外部审计 checkpoint | implementation-complete 状态 ↔ 多模型审查 | [`AGENT-RULES.md`](AGENT-RULES.md) §12 |

这些不是临时约定。它们共享相同的形状：*通过 frontmatter 或章节链接连接的两个规范源，智能体被允许（有时被要求）浮现差异*。下一个应用轴 —— 无论它最终是什么 —— 都会在此表中认出自己。

---

## 反模式：元如何被破坏

元模式是脆弱的。以下每一项如果被引入，都会回退框架产生涌现观察的能力。

- **Frontmatter 链接作为可选**。如果新文档类型以 `related:` / `originating_*` 作为建议性而非必需性发布，则交叉引用图会出现盲点，智能体丢失通过该类型三角化的能力。
- **规范章节被收拢为散文**。如果 `§Risk: R<N>` 被替换为*"风险讨论"*，可查询性就蒸发了。智能体无法再计数 `R<N>` 条目以检测驱动 `refresh-suggest` 的饱和阈值。自由散文不可查询；结构化章节可以。
- **对智能体创建文档的门禁**。要求对填写 AILOG / AIDEC / TDE 进行预批准会扼杀属性 2。智能体回退到仅浮现被询问的内容，因为浮现成本超过了本地收益。
- **没有涌现信号的遥测**。如果 `.telemetry.yaml` schema 演化但没有保留像 `r_n_plus_one_emergent_count` 这样的信号，操作员就失去了对智能体浮现涌现风险的频率的可见性。反馈循环断裂；元对框架演化变得不可见。
- **绕过表面的 CLI 命令**。直接发出决策（不写 AILOG，不填充 `R<N>` 章节）的 CLI 绕过了结构化表面。智能体下游的三角化退化，因为源对不再通过文档连接。

---

## 开放的应用轴 —— 元可在何处复制

本文档底层的审计识别出四个位置，其中结构性基础设施*部分*存在但文化许可或应用模式尚未命名。这些是元的未来应用候选，而非交付承诺。

- **MCARD ↔ 部署的模型代码** —— `TEMPLATE-MCARD.md` 存在；Charter 遥测中没有 `model-version-at-close` 字段，没有 AILOG `deployed_mcard:` 链接字段，没有漂移检测模式。与归档 MCARD 分歧的模型部署目前不可见。
- **SBOM ↔ lockfiles** —— `AI-RISK-CATALOG.md` §RISK-004 提到 AI 组件的 SBOM 维护；没有链接到 SBOM 的规范 AILOG 字段，没有漂移脚本（类似于 `check-followups-drift.sh`）比较声明的 SBOM 与实际的 `package.lock` / `requirements.txt`，没有依赖变化事件的遥测信号。
- **生效中的 ADR ↔ 矛盾的实现** —— `.telemetry.yaml` schema 捕获 `decisions_contradicting_prior_adrs`，但没有协议告诉智能体在实现期间观察到矛盾时*何时*浮现。信号存在；浮现约定不存在。
- **Constitution Check ↔ 框架版本升级** —— `SPECKIT-CHARTER-BRIDGE.md §Constitution Check re-evaluation cadence` 口头规范化了节奏；在 `straymark update-framework` 上不会触发自动警报。Charter 之间的框架升级可以静默地更改 Constitution 的门。

这四项在单个上游 RFC issue 中跟踪（在本文档落地之后提交）。每个都需要经验性 N=1 采用者验证后才能固化为命名模式 —— 适用原则 #12。

---

## 命名新元应用的权限/接受流程

`CHARTER-CHAIN-EVOLUTION.md` 记录的相同上游接受流程递归地适用于此元。新应用轴（上述四个之一，或浮现的第五个）通过以下方式落地：

1. **采用者本地 RFC** 在 `.straymark/06-evolution/<axis>-rfc.md`，描述已存在（或正在添加）的结构连接以及智能体应遵循的文化许可规则。
2. **上游 Issue** 镜像 RFC，引用经验观察发生的 AILOGs/Charters/遥测。
3. **上游接受** 表现为：（a）更新相关模板/schema/治理文档以添加缺失的结构件（frontmatter 字段、规范章节、遥测信号）；（b）将该轴添加到本文档中的"实例金字塔"表；（c）可选的机械检测 CLI 脚手架。
4. **第二领域验证** 在该轴的 schema 添加从可选升级为推荐之前。

本文档本身为元实例化了步骤 3.b —— 认识到现有应用共享单个底层属性的上游接受输出。

---

## 开放问题

- **"实质性分歧"的运营化**。原则 #8 措辞（[`PRINCIPLES.md`](PRINCIPLES.md)）将"实质性"留给智能体判断。每应用阈值（Pattern 1 使用 `r_n_plus_one_emergent_count > 6` 滚动平均）经验性地校准。是否可以达到跨轴阈值，或者每个轴是否必须校准自己的，仍然开放。
- **遥测合并**。每个应用当前发出自己的遥测槽（`pre_declare_refresh:`、`post_close_amendment:`、`r_n_plus_one_emergent_count`）。合并的*"本 Charter 浮现的涌现观察"*计数器可能使元在度量级别可见。已推迟 —— 过早聚合可能丢失每轴信号粒度。
- **采用者引导**。首次阅读 `STRAYMARK.md` 的新采用者应在足够早的时刻遇到元，以便他们在体验该模式时识别它。它是否驻留在 `QUICK-REFERENCE.md`、`STRAYMARK.md` 本身或新的引导章节中，仍然开放。

---

## 相关

- [`PRINCIPLES.md`](PRINCIPLES.md) §8 —— *跨源不一致浮现*（凝聚的文化规则）。
- [`AGENT-RULES.md`](AGENT-RULES.md) §6 —— *Be Proactive*（运营授权）；§3 —— *TDE 与 `R<N>`*（一个应用表面）；§12 —— *审计 Checkpoint*（制度化浮现）。
- [`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md) —— Pattern 1, Pattern 2（两个最高级应用）。
- [`SPECKIT-CHARTER-BRIDGE.md`](SPECKIT-CHARTER-BRIDGE.md) —— Charter 作为属性 1 链接最密集的桥接层。
- [`FOLLOW-UPS-BACKLOG-PATTERN.md`](FOLLOW-UPS-BACKLOG-PATTERN.md) —— 每 AILOG ↔ 注册表表面的漂移检测。
- [`DOCUMENTATION-POLICY.md`](DOCUMENTATION-POLICY.md) —— frontmatter 和 `related:` 字段规范。
- [`../../STRAYMARK.md`](../../STRAYMARK.md) §15 —— 应用汇聚的有界单元 Charter。

---

*StrayMark fw-4.17.0 | [GitHub](https://github.com/StrangeDaysTech/straymark) | Issue [#150](https://github.com/StrangeDaysTech/straymark/issues/150) · [#156](https://github.com/StrangeDaysTech/straymark/issues/156)*
