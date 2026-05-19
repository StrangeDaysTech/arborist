---
charter_id: CHARTER-NN
status: declared
effort_estimate: M
trigger: "[一行：哪个具体信号 — 可观察的事件、声明的决策、指标阈值或基础设施里程碑 — 证明现在执行此 Charter 是合理的]"
# 当 Charter 有已知的来源时，以下两个字段应恰好设置其中一个。
# 对于在没有明确来源的情况下搭建的 Charter，两者都不存在是有效的（在状态
# 转为 in-progress 之前必须填写）。
# originating_ailogs: [AILOG-YYYY-MM-DD-NNN]
# originating_spec: specs/001-feature/spec.md
---

# Charter: [简短标题]

> **状态（从 frontmatter 映射 — 真实来源在上方）:** declared. 工作量: [XS | S | M | L] (~[N] 分钟).
>
> **来源:** [人类可读的摘要；机器可读的形式是 frontmatter 中的 `originating_ailogs` 或 `originating_spec`].

<!-- Charter 模板 — 6 个格式约定，提炼自 Sentinel /plan-audit 实验（6 个周期，
     2026-04-28）。请参阅本文件末尾的注释块，了解每个约定及其经验依据，以及
     straymark-cli-roadmap.md §3 加上 straymark-thesis-validation.md §3-§5 的源证据。 -->

## 背景

[1-2 段。此 Charter 解决什么问题，是什么操作或监管动机使其紧迫，之前尝试过什么（如果有）。
如果有助于读者理解为什么推迟该工作，也可在此引用原始 AILOGs。]

## 范围

**范围内:**

[要应用的具体变更的编号列表。每一项都必须可验证："X 文件
增加 Y 方法"、"Z 测试覆盖 W 情况"。避免诸如"改善性能"之类的模糊条目
— 那些是目标，不是范围。]

1. [项目 1]
2. [项目 2]
3. [...]

**范围外:**

[明确不被此 Charter 涵盖的内容列表。重要的是，这样外部审计员就不会将其
分类为差距。理想情况下，引用它们所属的 Charter 或倡议。]

- [项目 1] — 推迟到 [Charter/倡议].
- [项目 2] — 因 [原因] 不在范围内.

## 要修改的文件

| 文件 | 变更 |
|---|---|
| `path/to/file.ext` | [变更的具体描述] |
| `...` | `...` |
| `.straymark/07-ai-audit/agent-logs/AILOG-...md` | 新建, `risk_level: [low|medium|high]` |

## 验证

### 本地检查

可在干净 shell 中按字面执行的命令 — 包含依赖项的显式安装。
这些命令的任何失败都表明真实的债务。

```bash
# 构建与测试（适应你的技术栈）
<build-command>
<test-command>

# 带有显式安装的安全/漏洞扫描器
# （在 Sentinel PLAN-01..05 中验证的模式：隐式 PATH 查找产生
# 来自外部审计员的误报 'real_debt' 分类。）
<install-and-run-security-scanner>
<install-and-run-vulnerability-scanner>

# 其他本地命令。如果它们需要集成基础设施，请显式记录：
<integration-test-command>
```

### 生产环境冒烟测试（部署后）

**仅在部署到真实环境后才适用的命令**。在没有基础设施的干净 shell 中
**不可执行**。外部审计员应跳过本节 — 此处的失败**不是** `real_debt`。

```bash
# 示例：验证生产环境中新端点是否上线。
TOKEN="$(<auth-cli> print-identity-token)"
curl -X PUT "https://${SERVICE_HOST}/api/v1/.../..." \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"...": "..."}'

# 示例：在生产数据库中执行 SQL 查询以验证事件持久化。
<production-db-cli> connect <service-db> -- \
  -c "SELECT context FROM audit_records WHERE action='...' \
      ORDER BY timestamp DESC LIMIT 1"
```

## 风险

[R1, R2, ... 风险列表，实现承诺要缓解。每项都附有缓解措施文档。约定：
如果在执行过程中出现 Charter 中未预见的新风险，则在 AILOG 的 `## Risk`
下记录为 `R<N+1> (new, not in Charter)` — Gemini 和其他外部审计员
进行跨文档验证。

每项缓解措施都应说明：(a) 具体的触发条件或阈值（不要"最终"），
(b) 承诺的行动，(c) 如果缓解措施本身失败会发生什么，(d) 如果风险揭示
了未来周期的经验教训，则在何处记录后续洞察。]

- **R1 — [风险描述]**: [概率/严重性].
  缓解措施: [在实现中采取的具体行动].
- **R2 — ...**: ...
- [...]

## 任务

1. 同步 main，创建分支 `<branch-prefix>/[slug]`.
2. [实现任务 1].
3. [实现任务 2].
4. [...]
5. AILOG (`risk_level: [low|medium|high]`, `review_required: [true|false]`).
6. **对于多批次执行（3+ 批次或 >1 天）**：在 AILOG 中维护
   `## Batch Ledger`（批次台账）。每个批次的 commit 之后，运行
   `straymark charter batch-complete <CHARTER-ID> <N>` 在推送前更新
   台账。关闭时的 drift gate 将拒绝任何仍为 `(pending)` 的
   `### Batch N`。对于单批次 Charter，请跳过此步骤——AILOG 中的
   `## 执行的操作` 已足够。
7. 本地验证通过且干净.
8. **自动检查清单漂移**（当 CLI 路线图的第 2 阶段发布时）：
   `straymark charter drift CHARTER-NN <range>` 在提交**之前**检测声明的文件
   与修改的文件之间的漂移。如果它报告遗漏，请完成工作或在 AILOG 的
   `## Risk` 下记录为 `R<N+1> (new, not in Charter)`。如果它报告范围扩展，
   请在 AILOG 中记录原因（mock 更新、生成的文件、漂移修复预先存在等）。
   在第 2 阶段发布之前，手动运行 Sentinel 的 `check-plan-drift.sh` 以获得相同效果。
9. 提交 + 推送 + 打开 PR.

## Charter 关闭

关闭此 Charter 时：

1. **原子更新（格式 v4）**: 如果漂移检查（任务 #7）报告了任何
   AILOG 中尚未捕获的漂移，请在提交前在**同一提交/PR**中编辑
   `## 要修改的文件` 和/或添加 `## 关闭说明` 块。不要推迟到合并后的
   维护 PR。原子更新模式是保持 Charter 与执行一致的规范方式；推迟它
   会使 Charter 变得陈旧，并使未来的读者感到困惑（Sentinel 的 PLAN-07
   演示了此步骤所防止的失败模式）。

2. **合并后漂移检查**（第 2 阶段发布时自动化 + 手动审查）：
   - 运行 `straymark charter drift CHARTER-NN origin/main..HEAD`（第 2 阶段）或
     等效的 Sentinel 脚本，并验证输出是干净的，或所有
     漂移都已在 AILOG 中记录。
   - 这捕获了在合并后引入漂移的罕见情况（squash 改写、管理员修订等），
     而 #1 中的原子步骤无法应用。

3. **将 `.straymark/charters/README.md` 中的行移动**到 `## Closed` 并引用 PR.

4. **状态 frontmatter** 从 `in-progress` 移动到 `closed`（并可选地
   添加 `closed_at: YYYY-MM-DD` — schema 允许任意附加字段）.

5. **不要删除**此文件 — 规划历史与执行的 AILOG 一样重要.

## 关闭说明

> 仅当任务 #7 漂移检查报告了实现者选择原子地补救（而不是重做实现
> 以完全匹配 `## 要修改的文件`）的漂移时，才添加此部分。每个要点：
> 相对于声明发生了什么变化，为什么，引用记录决策的 AILOG。如果未检测到
> 漂移，则完全省略该部分 — 空的 `## 关闭说明` 是噪声。
>
> Sentinel 中的历史示例：PLAN-05 (`docs/plans/05-per-service-anomaly-thresholds.md`)
> §关闭说明 — 文件被移除，因为实现选择了不同的注入点；PLAN-07
> (`docs/plans/07-fix-distribution-aligner.md`) §关闭说明 — 文件被移除，因为
> 实时测试对此变更不敏感。两者都演示了在生产使用中的模式。

- `[path/file-from-declaration.ext]` [已移除 | 重定位到 X | 重新用途化]:
  [1-2 行解释实现做了什么以及为什么原始声明不再准确]。
  引用: AILOG-YYYY-MM-DD-NNN §[section].

---

<!--
格式约定 — 此模板中嵌入的 6 种模式，提炼自 6 个周期的 Sentinel /plan-audit
实验（2026-04-28）。出处是历史记录的一部分（用 StrayMark 的术语，这些只是
"约定"，而不是"v2 + v3 添加" — 该分区是 Sentinel 的迭代日志，而不是结构性的）。

1. 验证分为 `### 本地检查`（可在干净 shell 中按字面执行）
   和 `### 生产环境冒烟测试（部署后）`（没有基础设施无法执行）。
   原因：外部审计员将仅生产环境的命令失败分类为 `real_debt` —
   可避免的噪声。在约定被命名后，在 5/5 周期中得到验证。

2. 工作量以时间衡量（XS/S/M/L），而不是以 `~N 行` 衡量。原因：时间在
   4/5 周期中符合估计（1.0 倍）；行数因 AILOG/tests/mocks 而漂移 1.0 倍 → 3.1 倍 → 8.1 倍。
   行数不能预测认知工作量。

3. 像 `(可选)` 或 `(部署后)` 之类的修饰符作为结构化子部分存在，
   而不是作为内联括号注释。原因：Gemini 审计员一致地忽略了括号修饰符，
   并将标记为可选的命令分类为 `real_debt`。在适用模式的 2/2 周期中得到验证。

4. R<N> 风险在 Charter 中列举；执行期间出现的新风险在 AILOG 中
   记录为 `R<N+1> (new, not in Charter)`。原因：外部审计员可跨验证的
   信号 — 他们将 Charter 声明与 AILOG 出现进行三角测量。在出现新风险的
   4/4 周期中得到验证。

5. `## Charter 关闭` 部分要求实现者在任务 #7 检测到漂移时
   原子地更新 Charter 文档（与修复同一 PR），而不是在单独的
   合并后维护 PR 中。`## 关闭说明` 块是记录每个原子编辑的规范位置
   （相对于 `## 要修改的文件` 发生了什么变化，为什么，AILOG 引用）。
   原因：Sentinel 的 PLAN-07 演示了，如果没有明确的原子更新步骤，
   漂移补救可能落后于主 PR 数天，使 Charter 变得陈旧，并使未来的
   读者感到困惑 — Sentinel 2026-05-02-001 的 AIDEC 形式化了该差距
   并提出了格式 v4（此模板体现了它）。

6. 自动检查清单漂移（`straymark charter drift`，CLI 路线图的第 2 阶段；
   Sentinel 有 `scripts/check-plan-drift.sh`）在 pre-commit（任务 #7）和
   Charter 关闭时运行。检测 OMISSION 漂移（声明的文件未触及）和 SCOPE
   EXPANSION 漂移（触及的文件未声明）。原因：外部审计员捕获了实现者
   未在其 AILOG 中记录的 implementation-gap 和 hallucination 漂移。
   该脚本在提交**之前**捕获相同的漂移，将"已知且已记录"与"已忘记"分开。
   在针对规范 Sentinel 计划的 2/2 经验测试中零误报。
-->
