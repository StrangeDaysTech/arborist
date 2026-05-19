# TC260 实施指南 — StrayMark

> 根据 **AI 安全治理框架 v2.0**(TC260,2025-09-15)填写 TC260RA 文档的实务指南。

**Languages**: [English](../../TC260-IMPLEMENTATION-GUIDE.md) | [Español](../es/TC260-IMPLEMENTATION-GUIDE.md) | 简体中文

---

## 何时创建 TC260RA

当 `regional_scope` 包含 `china` 且符合以下任一情况时,创建 TC260 风险评估:

- 您部署或修改的系统是或包含 AI 模型。
- 应用面向中国大陆用户,或运营者在中国大陆注册。
- 系统涉及触及中国大陆的跨境数据流。
- 预期发生新的模型版本、场景或规模跃升。

TC260RA 与 ETH 上的 EU AI Act 风险分类互补,通过相关 ETH / MCARD / AILOG 上的 `related: [TC260RA-...]` 引用。

---

## 三标准

TC260 v2.0 在三个正交维度上分级风险,然后组合为单一等级(low / medium / high / very_high / extremely_severe)。

### 1. 应用场景(`tc260_application_scenario`)

| 场景 | 例子 | 固有风险底线 |
|------|------|-----------|
| `public_service` | 政府聊天机器人、公共信息门户 | medium |
| `healthcare` | 临床决策支持、医学影像 | high |
| `finance` | 信贷评分、KYC、欺诈检测 | high |
| `safety_critical` | 自动驾驶、工业控制、能源 | very_high |
| `content_generation` | 文本/图像/视频合成 | medium |
| `social` | 推荐、排序、社交 | medium |
| `industrial_control` | OT 系统、机器人 | very_high |
| `other` | 简要记录 | — |

### 2. 智能等级(`tc260_intelligence_level`)

| 等级 | 定义 |
|------|------|
| `narrow` | 单一用途、确定性输出 |
| `foundation` | 通用基础模型(LLM、视觉语言),无工具使用 |
| `agentic` | 基础模型 + 自主工具使用,可在现实世界采取行动 |
| `general` | 接近通用人工智能 — 跨域广泛能力 |

### 3. 应用规模(`tc260_application_scale`)

| 规模 | 定义 |
|------|------|
| `individual` | 单一用户 / 小团队 |
| `organization` | 单一组织或企业 |
| `societal` | 公众的相当部分(≥ 100 万用户) |
| `cross_border` | 跨中国大陆与其他司法辖区运营 |

---

## 综合等级

无公开的数字公式。使用以下矩阵作为起点并记录推理:

| 场景 \ 智能 | Narrow | Foundation | Agentic | General |
|-------------|--------|-----------|---------|---------|
| public_service | low → medium | medium | high | very_high |
| healthcare / finance | medium | high | high | very_high |
| safety_critical | high | very_high | very_high | extremely_severe |
| content_generation | low | medium | high | very_high |
| social | low | medium | high | very_high |
| industrial_control | high | very_high | very_high | extremely_severe |

**规模修饰**:
- `individual`、`organization` → 不变。
- `societal` → 上调一级。
- `cross_border` → 上调一级 **并** 要求显式跨境数据分析(参见 PIPL-PIPIA-GUIDE)。

---

## 风险分类:如何填充

### 内生(`tc260_endogenous_risks`)

模型固有:`hallucination`、`bias`、`robustness`、`data_leakage`、`prompt_injection`、`model_extraction`。

### 应用(`tc260_application_risks`)

技术使用引发:`misuse`、`scope_creep`、`dependency`、`availability`、`integration_flaw`。

### 衍生(`tc260_derivative_risks`)

二阶社会影响:`labor_displacement`、`opinion_shaping`、`ecosystem_disruption`、`monoculture`、`loss_of_skill`。

对于 `very_high` 与 `extremely_severe` 等级,v2.0 明确要求 **灾难性风险监测**:在 TC260RA 第 5 节(治理措施)中记录。

---

## 从其他文档关联

当非 TC260RA 文档设置 `tc260_risk_level: high`(或更高)时,验证规则 **CROSS-004** 要求 `review_required: true`。TC260RA 自身应通过 `related:` 关联:

```yaml
related:
  - TC260RA-2026-04-25-001
  - MCARD-2026-04-25-001
  - PIPIA-2026-04-25-001
```

---

## 复审周期

| 触发 | 行动 |
|------|------|
| 模型版本变更 | 重新执行第 4 节(技术应对) |
| 场景扩展 | 重新分级:场景 × 智能 × 规模 |
| 跨越规模阈值(如 100 万用户) | 等级评估 |
| TC260 法规更新 | 完整重审 |

<!-- StrayMark | https://strangedays.tech -->
