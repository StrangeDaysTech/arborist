# Follow-ups Backlog 模式 - StrayMark

> 用于跨多个 AILOG 和 Charter 管理累积的 `§Follow-ups` 与 `R<N> (new, not in Charter)` 条目的可复制约定。

**语言**: [English](../../FOLLOW-UPS-BACKLOG-PATTERN.md) | [Español](../es/FOLLOW-UPS-BACKLOG-PATTERN.md) | 简体中文

---

## 状态

**v0 — 在 N=1 域中已验证**（`StrangeDaysTech/sentinel` CHARTER-12,2026-05-06）。

这是一个**约定**,不是 CLI 功能。Adopter 使用 markdown + 一个可移植的 bash 脚本在本地复制此模式。在第二个 adopter 验证后,该模式可能演变为 `straymark followups` 子命令(参见[未决问题](#未决问题))。

---

## 何时适用此模式

StrayMark 的 per-AILOG `§Follow-ups` 约定在写入时有效 — 创建 AILOG 时,实施者记录推迟到后续 Charter 或操作触发器的内容。在累积列表超出操作员可凭记忆扫描范围之前,这种方式都能正常工作。

当满足**任一**以下条件时,采用此模式:

- 项目已累积 **约 20 个或更多 AILOG**,带有非平凡的 `§Follow-ups` 部分。
- 操作员反复要求代理"列出项目中所有待处理事项",答案需要多文件扫描。
- 一个"当 X 到来时执行"的 follow-up 几乎丢失,因为在 X 到来后从未重读过原始 AILOG。
- Charter 回顾揭示出本应在数周前被分类为 `closed`、但从未被索引的 follow-ups。

低于此规模时,仅 per-AILOG 约定就足够了 — 过早采用此模式只会增加维护开销而无回报。

---

## 形式

### 注册表文件

规范路径下的单个 markdown 文件:

```
.straymark/follow-ups-backlog.md
```

### Frontmatter (YAML)

```yaml
---
last_scan: 2026-05-06
last_scan_range: AILOG-NNNN-NN-NN-NNN..AILOG-NNNN-NN-NN-NNN  # 可选 —— 涵盖的首个..末尾 AILOG
schema_version: v0
total_open: 0           # 当前为 `open` 状态的条目计数
total_promoted: 0       # 当前为 `promoted` 状态的条目计数(schema v0.1 新增 —— 见"提升为 TDE")
total_closed_in_session: 0   # 上次会话以来 `closed` 条目计数(可选,operator-maintained)
total_phase_blocked: 0  # `phase-blocked` 条目计数(可选)
buckets:
  - ready
  - time-triggered
  - charter-triggered
  - phase-blocked
  - operational
fully_extracted_ailogs:
  - AILOG-2026-04-11-001
  - AILOG-2026-04-12-001
  # ... 每个 follow-ups 已被处理的 AILOG 一个条目
---
```

`total_*` 计数器是**operator-maintained 的元数据**。drift 脚本不会自动更新它们 —— 它们位于 header 之中,使会话开始时的扫视就能看到注册表的脉搏,无需在 bucket 中翻页。`total_promoted` 在 schema v0.1 中固化(Sentinel adopter 的经验信号,fw-4.13.1),与现有的 `total_open` / `total_closed_in_session` / `total_phase_blocked` 模式保持一致。

`fully_extracted_ailogs` 列表是 drift 检测的**承重元数据**。所有 `§Follow-ups` 和 `R<N>` 条目已被转移到注册表(或被显式分类为 superseded)的 AILOG 都属于此列表。Drift 检测将此列表与 repo 中具有 follow-up 内容的 AILOG 进行比对。

### Buckets

五个 bucket 按触发类型组织条目:

- `ready` — 现在可执行,无外部触发器依赖。
- `time-triggered` — 基于日历的触发器(审计周期、周期性审查)。
- `charter-triggered` — 由触及相关领域的未来 Charter 阻塞。
- `phase-blocked` — 由尚不存在的未来组件或阶段阻塞。
- `operational` — 操作员手动决策或外部系统操作。

### 条目 schema

bucket 内的每个条目遵循以下形式:

```markdown
### FU-NNN — <简短描述>
- **Origin**: AILOG-NNNN-NN-NN-NNN <指向源部分的指针>
- **Status**: open | in-progress | closed | superseded | promoted
- **Trigger**: ready | <日历日期> | when <X> | <其他>
- **Destination**: <Charter id、"operations"、未来阶段,或 TDE-YYYY-MM-DD-NNN>
- **Cost**: <工作量估计>
- **Notes**: <自由格式上下文>
- **Promoted to**: <TDE id,当 Status: promoted 时 — 见下方"提升为 TDE">
```

`FU-NNN` 在注册表整个生命周期内单调递增;条目关闭时不重新编号。

### Status 词汇表

- `open` — 待处理,尚未采取行动。
- `in-progress` — 已声明或正在执行的 Charter 处理此条目。
- `closed` — 条目已解决(Charter 已合并、操作任务已完成、时间已过且已审查)。
- `superseded` — 由其他工作处理,该工作未直接引用此条目。
- `promoted` — 条目因满足横向债务标准而被提升为 TDE 文档(见下方"提升为 TDE")。`Promoted to:` 字段携带 TDE id。

closed、superseded 和 promoted 条目保留在文件中(可审计的历史)。操作员可以将它们移到底部的 `## Bucket: closed` 部分以进行视觉整理,但绝不删除。

---

## 提升为 TDE

某些 FU 条目不仅仅是延期任务 —— 它们描述的是值得拥有自己治理文档的**横向技术债务**(TDE)。提升标准与 `AGENT-RULES.md §3` 中的 TDE-vs-`R<N>` 判定一致:

- 该条目是*先前 Charter 的遗留*(已经历 ≥1 次 Charter 关闭仍未修复)。
- 该条目*横跨多个模块或多个 Charter* —— 中央注册表已将其碎片化为共享同一根本原因的多个 bullet。
- 该条目*需要在当前 scope 包络之外的专用 Charter* 来修复。
- 该条目*需要人工决定优先级或分配*,操作员的周期性审查无法仅从 bullet 决定(impact × effort 矩阵、所有权)。

当上述任一条件成立时,将该 FU 条目提升为 `.straymark/06-evolution/technical-debt/` 下的 TDE 文档:

1. 通过 `/straymark-new tde`(或 `straymark new --type tde`)创建 TDE。从 FU 条目的上下文填入 `impact`、`effort`、`type` 与正文各节。
2. 在 TDE 的 frontmatter 中添加 `promoted_from_followup: FU-NNN` 以便溯源。
3. 在 FU 条目中,设置 `Status: promoted`、`Destination: TDE-YYYY-MM-DD-NNN`,并添加 `Promoted to: TDE-YYYY-MM-DD-NNN`。如果你维护 `## Bucket: closed` 节,则将条目移过去;否则保持原位并更新状态。

FU 条目在提升后**不会被删除** —— 它在注册表中的存在就是显示 TDE 来源的审计轨迹。

### 两种提升形态 —— 提升已存在的 vs 创建时即追溯提升

上述工作流涵盖**标准情况**:`open` 状态的 FU 条目已存在于注册表中,并在周期性审查期间被提升为 TDE。还有一种同样有效的情况,源自 Sentinel CHARTER-13 回顾的经验:

- **提升已存在条目** —— FU 数周或数个 Charter 之前已被(通常通过 `--apply`)登记为 `open`,经历过 ≥1 次 Charter 关闭仍未解决,并满足上述四项标准。标准流程。
- **创建时即追溯提升** —— 在回顾(Charter 关闭仪式、审计周期、RFC 撰写)*期间* 该债务被识别为值得作为 TDE,且从未作为 `open` FU 存在。先创建 TDE;在注册表中以 *`Status: promoted`* 状态新增一个 FU 条目,提供从 TDE 回溯到原始上下文(AILOG 中的某个 `R<N>`、calibrator 的 finding、被延期的分类)的审计轨迹。

两种形态在注册表中产生相同的终态:一个 `Status: promoted` 且具有 `Promoted to: TDE-YYYY-MM-DD-NNN` 指针的条目。区别在于条目是预先以 `open` 存在,还是天生即为 `promoted`。drift 脚本一视同仁(不按出生状态区分);统计 `total_promoted` 的 adopter 分析在两种情况下得到相同数字。

存疑时,优先创建 FU 条目 —— 即便是追溯创建 —— 因为它会把 TDE 交叉引用回触发该识别的 AILOG / R-号 / 源上下文。一个 `promoted_from_followup: FU-NNN` 指向 backlog 中实际存在的条目的 TDE,比指向一个虚构的 FU 更易导航。

### 何时提升

- **周期性审查** —— 当操作员做人工重新分类时,提升任何已经历 ≥2 次 Charter 关闭仍未解决且符合上述标准的条目。
- **Charter 关闭** —— 在审查刚关闭的 Charter 所解决的条目时,如果发现*未*被解决且符合上述标准的条目,则提升它们,而不是保留为 `open`。
- **Charter 声明前** —— 如果你即将声明一个 Charter,并注意到注册表中包含此 Charter 仅会*部分*处理的条目,那么未处理的部分可能应作为 TDE,而不是作为另一个被延期的 FU。

Drift 脚本(`scripts/check-followups-drift.sh`)在 v0 **不扩展**支持提升候选 —— 提升由操作员驱动。未来 v1 增强可标记符合"经历 ≥2 个 Charter"启发式的条目,但这等到第二个 adopter 验证模式后才会固化(与 v0 → v1 其余部分相同的门槛)。

---

## Drift 检测

一个小的 bash 脚本是验证层,使注册表与新 AILOG 保持同步。该脚本位于 adopter 的 repo 中(建议路径:`scripts/check-followups-drift.sh`),有三种模式。

### 模式

- **默认** — 扫描 `git diff origin/main..HEAD`(回退到 `HEAD~1..HEAD`)中修改的 AILOG。对任何具有 `§Follow-ups` / `R<N> (new)` 内容但不在 `fully_extracted_ailogs` 中的 AILOG 发出警告。drift 时退出 1。
- **`--apply`** — 相同的扫描,但自动在 `## Bucket: ready` 下追加新条目,使用自动生成的 `FU-NNN` id,并将 AILOG id 追加到 `fully_extracted_ailogs`。操作员稍后将其重新分类到正确的 bucket。
- **`--scan-all`** — 扫描项目中的每个 AILOG(周期性完整扫描)。

### Per-AILOG 与 per-bullet 粒度

跟踪是 **per-AILOG**,而非 per-bullet。AILOG 要么被完全提取(其 id 在 `fully_extracted_ailogs` 中 — 信任注册表),要么未被提取(提取所有内容)。Per-bullet 匹配将需要指纹识别(文本哈希或模糊比较),每当注册表条目对 AILOG 的 bullet 进行改写时,这都会产生误报 — 而经过整理的条目总是会改写。

Per-AILOG 粒度的成本:当在 Charter 关闭后向已提取的 AILOG 添加 follow-up 时,drift 检测无法发现。修复由操作员驱动 — 手动从 `fully_extracted_ailogs` 中移除该 AILOG 并使用 `--apply` 重新运行。这种权衡对 v0 是有意为之,因为大多数 AILOG 在 Charter 关闭后是 write-once。

### 脚本模板

参考实现(约 290 行 POSIX bash)位于 `StrangeDaysTech/sentinel` 的 `scripts/check-followups-drift.sh`。将其复制到你的 repo 并调整顶部常量:

```bash
BACKLOG_FILE=".straymark/follow-ups-backlog.md"
AILOG_DIR=".straymark/07-ai-audit/agent-logs"
```

脚本仅使用 `awk` 和 `grep` — 没有 jq、yq 或重型依赖。在 Linux 和 macOS 之间可移植。

---

## 代理集成

代理(Claude / Gemini 等)成为注册表的主要维护者。添加到你的 `CLAUDE.md` / `AGENT.md`:

```markdown
## Follow-ups backlog

- **会话开始**: 浏览 `.straymark/follow-ups-backlog.md` 以了解项目中所有待处理事项。
- **Pre-commit 检查**: 创建或修改了任何带有 `## Follow-ups` 或 `R<N> (new, not in Charter)` 条目的 AILOG 吗? → 在同一个 commit 中运行 `scripts/check-followups-drift.sh --apply` 以扩展 backlog。
- **Charter 关闭后**: 审查 Charter 解决的条目;将其标记为 `closed`(在 `Notes` 中带有关闭 Charter id)或 `superseded`。对于未解决但符合 TDE 标准的条目(先前 Charter 的遗留、横向、需要专用 Charter、需要人工优先级),将其提升为 TDE 文档(见本模式的"提升为 TDE"以及 `AGENT-RULES.md §3`)。
```

这使代理成为维护者,脚本成为验证层,操作员成为周期性审查者(重新分类、标记 closed、修剪 superseded、在符合标准时提升为 TDE)。

---

## 采用流程

对于从零开始的 adopter:

1. 创建 `.straymark/follow-ups-backlog.md`,使用上述 frontmatter(初始 `fully_extracted_ailogs:` 列表为空)和五个 `## Bucket: <name>` 标题。
2. 将参考脚本从 `StrangeDaysTech/sentinel` 复制到 `scripts/check-followups-drift.sh`。如果你的 AILOG 位于其他位置,调整 `AILOG_DIR`。
3. 运行 `scripts/check-followups-drift.sh --scan-all --apply` 从现有 AILOG 播种注册表。
4. 手动将自动生成的 `## Bucket: ready` 条目重新分类到正确的 bucket。这是一次性 triage,对于约 50 个条目的 backlog 通常需要 30-60 分钟。
5. 将代理集成块添加到 `CLAUDE.md` / `AGENT.md`。
6. 可选地将 `scripts/check-followups-drift.sh` 接入 pre-commit hook 以进行硬性 enforcement。

对于从临时跟踪迁移的 adopter:相同的流程,但步骤 4 可能需要决定哪些条目已经是 `closed` 或 `superseded` — 该分类是使注册表有用的关键。

---

## 参考实现

`StrangeDaysTech/sentinel` CHARTER-12,于 2026-05-06 合并:

- 实现 PR: [sentinel#53](https://github.com/StrangeDaysTech/sentinel/pull/53)(注册表 + 脚本 + CLAUDE.md 添加)。
- 关闭 PR: [sentinel#54](https://github.com/StrangeDaysTech/sentinel/pull/54)(合并后验证 + Charter 关闭)。

经验上下文: 47 个条目从 CHARTER-08 → CHARTER-11 回顾中播种(约 30 分钟的多代理 triage)。该链条展示了激发该模式的差距 — 没有注册表,操作员无法在不孤立地重新分类每个 Charter 的 follow-ups 的情况下查看"项目中所有待处理事项"。有了注册表,会话开始时的浏览就是一次文件读取。

---

## 未决问题

这些在 v0 中尚未解决。该模式的未来修订版本或 CLI helper 可能会处理它们:

- **Bucket 分类启发式**。今天 `--apply` 将每个新条目转储到 `## Bucket: ready`;操作员手动重新分类。使用 AILOG `tags` 和 Charter `effort_estimate` 的启发式可以自动建议 bucket。
- **Schema 验证**。注册表遵循一个隐式 schema;尚不存在 `.straymark/schemas/follow-ups-backlog.schema.v0.json`。今天的验证是人工审查。
- **与审计周期的集成**。当 `straymark charter audit --merge-reports` 产生未在关闭前原子修复的真实 debt findings 时,这些 findings 仅存在于 `.straymark/audits/<id>/review.md` 中。它们不会自动流入中央注册表。自动浮现它们将关闭一个已知差距。
- **`closed` 与 `superseded` 语义**。今天的差异在于解决工作是否显式引用了该条目。可能会出现更严格的约定。
- **作为 `straymark followups` CLI 的结晶化**。一旦第二个 adopter 验证了该模式,framework 可以发布一个子命令,镜像现有的 `straymark charter` 三件套:`list` / `close` / `drift`。处于 v0 的 adopter(此模式)通过删除其本地脚本并切换代理指令进行迁移。

---

## 致谢

通过 [issue #111](https://github.com/StrangeDaysTech/straymark/issues/111) 由 Sentinel adopter 贡献。经验基础:`StrangeDaysTech/sentinel` 中的 CHARTER-08 → CHARTER-11 链。作者:Claude Opus 4.7,代表操作员 José Villaseñor Montfort。

---

## 相关

- [EMERGENT-OBSERVATION-DESIGN.md](EMERGENT-OBSERVATION-DESIGN.md) —— 元模式，此漂移检测约定在每 AILOG ↔ 注册表表面实例化了它。
- [CHARTER-CHAIN-EVOLUTION.md](CHARTER-CHAIN-EVOLUTION.md) —— 在链级别（Pattern 1）和周期级别（Pattern 2）运作的姐妹模式。
- [AGENT-RULES.md §3](AGENT-RULES.md) —— 可将 follow-ups 升级为专用债务条目的 TDE-vs-`R<N>` 升级标准。

---

*StrayMark fw-4.17.0 | [Strange Days Tech](https://strangedays.tech)*
