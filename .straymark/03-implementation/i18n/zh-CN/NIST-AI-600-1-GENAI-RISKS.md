# NIST AI 600-1 --- 生成式 AI 风险类别参考

> **标准**: NIST AI 600-1 --- Artificial Intelligence Risk Management Framework: Generative AI Profile
>
> NIST AI 600-1 定义了 12 个生成式 AI 系统特有的风险类别。本参考文档将每个类别映射到 StrayMark 模板、frontmatter 值和实际缓解措施。请结合 NIST AI RMF 功能指南使用本文档，以确保全面覆盖生成式 AI 风险。

---

## 1. CBRN Information

**标识符**: `cbrn`

生成式 AI 系统可能生成有助于制造、获取或部署化学、生物、放射性或核 (CBRN) 武器或材料的信息。

- 大型语言模型提供受管控化学物质的逐步合成说明
- 代码生成模型生成针对关键基础设施控制系统的功能性漏洞利用代码
- AI 助手以可操作的细节回应有关生物制剂培养的查询

### StrayMark 映射

| 方面 | 值 |
|--------|-------|
| 主要模板 | ETH |
| Frontmatter 值 | `nist_genai_risks: [cbrn]` |
| 关键部分 | 风险评估、建议 |
| 辅助文档 | SEC（输出过滤控制） |

### 建议缓解措施

- 实施输出过滤和内容安全分类器以检测 CBRN 相关内容
- 通过微调或系统提示限制模型在 CBRN 相邻领域的能力
- 为触发 CBRN 安全标记的查询建立人工审查工作流

---

## 2. Confabulation

**标识符**: `confabulation`

生成式 AI 系统可能产生事实错误、虚构的或与训练数据不一致的输出，同时以高度自信的方式呈现这些输出。

- 摘要模型编造源材料中不存在的引用
- 代码生成模型发明不属于引用库的 API 方法
- 对话式 AI 提供与既定临床指南相矛盾的医疗信息

### StrayMark 映射

| 方面 | 值 |
|--------|-------|
| 主要模板 | ETH |
| Frontmatter 值 | `nist_genai_risks: [confabulation]` |
| 关键部分 | 风险评估、透明度、建议 |
| 辅助文档 | MCARD（局限性部分）、TES（准确性测试） |

### 建议缓解措施

- 实施检索增强生成 (RAG) 以将输出建立在经过验证的来源基础上
- 为模型输出添加置信度指标和来源归属
- 创建包含事实准确性基准和幻觉检测测试的 TES 文档

---

## 3. Dangerous, Violent, or Hateful Content

**标识符**: `dangerous_content`

生成式 AI 系统可能生成宣传、美化或提供暴力、自残或针对个人或群体仇恨的指导性内容。

- 文本生成模型产生煽动对特定群体施暴的内容
- 图像生成模型根据请求创建逼真的暴力图形描述
- 聊天机器人在受到弱势用户提示时提供详细的自残指导

### StrayMark 映射

| 方面 | 值 |
|--------|-------|
| 主要模板 | ETH |
| Frontmatter 值 | `nist_genai_risks: [dangerous_content]` |
| 关键部分 | 风险评估、偏见评估、建议 |
| 辅助文档 | SEC（内容审核控制） |

### 建议缓解措施

- 在输入和输出端部署内容安全分类器
- 实施针对检测到的有害内容尝试的升级协议
- 通过对抗性红队测试验证内容过滤的有效性（记录在 TES 中）

---

## 4. Data Privacy

**标识符**: `privacy`

生成式 AI 系统可能记忆、泄露或无意中揭示训练数据或用户交互中的个人数据、敏感信息或隐私细节。

- 语言模型逐字复制训练数据中包含个人可识别信息的段落
- 对话式 AI 保留并在另一个用户的会话中显示某个用户的信息
- 在专有数据上训练的模型通过精心设计的提示泄露商业秘密

### StrayMark 映射

| 方面 | 值 |
|--------|-------|
| 主要模板 | ETH、DPIA |
| Frontmatter 值 | `nist_genai_risks: [privacy]` |
| 关键部分 | 数据隐私部分（ETH）、完整 DPIA |
| 辅助文档 | SEC（访问控制、数据隔离） |

### 建议缓解措施

- 在部署基于个人数据训练或处理个人数据的系统之前进行 DPIA
- 在训练管道中实施差分隐私技术或数据脱敏
- 测试记忆化和数据提取漏洞（记录在 TES 中）

---

## 5. Environmental Impacts

**标识符**: `environmental`

生成式 AI 模型的训练、微调和推理消耗大量计算资源，导致能源使用、碳排放和电子废弃物的增加。

- 训练大型基础模型消耗的能源相当于数百个家庭一年的用电量
- 频繁的重新训练周期成倍增加环境成本，而能力提升并不成比例
- 大规模部署推理在数据中心基础设施中产生持续的能源消耗

### StrayMark 映射

| 方面 | 值 |
|--------|-------|
| 主要模板 | ETH |
| Frontmatter 值 | `nist_genai_risks: [environmental]` |
| 关键部分 | 环境影响部分 |
| 辅助文档 | MCARD（计算需求）、AI-KPIS.md（效率指标） |

### 建议缓解措施

- 在 MCARD 中记录计算需求和能源估算
- 在 AI-KPIS.md 中跟踪和报告碳足迹指标
- 在 ADR 文档中评估模型效率替代方案（更小的模型、蒸馏、量化）

---

## 6. Harmful Bias and Homogenization

**标识符**: `bias`

生成式 AI 系统可能放大训练数据中存在的社会偏见，产生歧视或错误描述特定群体的输出。当 AI 被广泛使用时，同质化现象会减少思想和表达的多样性。

- 图像生成模型在专业场景中持续低估某些人口群体的代表性
- 文本模型将负面刻板印象与特定国籍或性别相关联
- 广泛使用单一 AI 写作助手使整个行业的沟通风格趋于同质化

### StrayMark 映射

| 方面 | 值 |
|--------|-------|
| 主要模板 | ETH |
| Frontmatter 值 | `nist_genai_risks: [bias]` |
| 关键部分 | 偏见评估、社会影响、建议 |
| 辅助文档 | TES（公平性测试）、MCARD（训练数据文档） |

### 建议缓解措施

- 在相关人口统计维度上进行偏见评估（记录在 ETH 偏见评估部分）
- 在 TES 文档中实施公平性指标并测试差异性影响
- 在 MCARD 中记录训练数据组成和已知的代表性差距

---

## 7. Human-AI Configuration

**标识符**: `human_ai_config`

由于人工监督水平不当、对 AI 输出的过度依赖、自动化偏见或人机交互边界配置不当而产生的风险。

- 操作员因自动化偏见在未经有意义审查的情况下盲目批准 AI 建议
- 系统部署时缺乏需要人类判断的情况下的明确升级路径
- 用户在长期交互中未遇到错误后对 AI 输出产生过度信任

### StrayMark 映射

| 方面 | 值 |
|--------|-------|
| 主要模板 | ETH |
| Frontmatter 值 | `nist_genai_risks: [human_ai_config]` |
| 关键部分 | 风险评估、建议 |
| 辅助文档 | AGENT-RULES.md（自主权限制）、MCARD（预期使用边界） |

### 建议缓解措施

- 在 AGENT-RULES.md 中为每个自主级别定义明确的人工监督要求
- 在 MCARD 的预期用途部分记录预期的人机交互模式
- 为高风险决策实施强制人工审查检查点（记录在 ADR 中）

---

## 8. Information Integrity

**标识符**: `information_integrity`

生成式 AI 可被用于创建或放大错误信息、虚假信息和篡改媒体，破坏公众信任和信息生态系统的完整性。

- 模型生成高度逼真但虚构的新闻文章，与合法新闻无法区分
- AI 生成的深度伪造媒体被用于冒充公众人物进行虚假信息运动
- 自动化内容生成大规模地向信息渠道注入低质量、误导性内容

### StrayMark 映射

| 方面 | 值 |
|--------|-------|
| 主要模板 | ETH |
| Frontmatter 值 | `nist_genai_risks: [information_integrity]` |
| 关键部分 | 风险评估、透明度、社会影响 |
| 辅助文档 | SEC（来源控制）、MCARD（输出水印） |

### 建议缓解措施

- 为 AI 生成的输出实施内容来源追溯和水印（在 ADR 中记录方法）
- 在 AI-GOVERNANCE-POLICY.md 中建立禁止欺骗性使用的使用策略并进行记录
- 在下游系统中部署篡改或 AI 生成内容的检测机制

---

## 9. Information Security

**标识符**: `information_security`

生成式 AI 系统引入了新型攻击面，包括提示注入、模型提取、训练数据投毒和绕过安全控制的对抗性输入。

- 攻击者使用提示注入覆盖系统指令并提取敏感配置
- 模型通过对抗性输入被操纵以产生绕过内容过滤器的输出
- 训练数据投毒导致模型在特定领域产生微妙错误的输出

### StrayMark 映射

| 方面 | 值 |
|--------|-------|
| 主要模板 | SEC |
| Frontmatter 值 | `nist_genai_risks: [information_security]` |
| 关键部分 | 威胁模型、已实施控制、漏洞 |
| 辅助文档 | ETH（风险评估）、TES（安全测试） |

### 建议缓解措施

- 创建包含针对生成式 AI 攻击向量（提示注入、提取、投毒）的威胁模型的 SEC 文档
- 实施在 SEC 中记录的输入验证和输出清理控制
- 进行对抗性红队演练并在 TES 中记录结果

---

## 10. Intellectual Property

**标识符**: `intellectual_property`

生成式 AI 可能通过复制受版权保护的材料、生成与受保护作品实质相似的输出或在未经授权的情况下使用专有数据来侵犯知识产权。

- 代码生成模型逐字复制训练数据中受版权保护的源代码片段
- 图像生成模型产生与在世艺术家独特风格高度相似的输出
- 在专有企业文档上训练的模型生成揭露受保护商业秘密的输出

### StrayMark 映射

| 方面 | 值 |
|--------|-------|
| 主要模板 | ETH |
| Frontmatter 值 | `nist_genai_risks: [intellectual_property]` |
| 关键部分 | 风险评估、建议 |
| 辅助文档 | SBOM（训练数据来源、许可证合规） |

### 建议缓解措施

- 在 SBOM 中记录训练数据来源及其许可条款
- 实施输出与已知受版权保护作品的相似性检测
- 为商业应用中使用的 AI 生成内容建立知识产权审查工作流（记录在 ADR 中）

---

## 11. Obscene or Degrading Content

**标识符**: `obscene_content`

生成式 AI 系统可能产生色情、淫秽或贬损性内容，无论是通过直接生成还是通过利用内容过滤器的弱点。

- 图像生成模型通过提示工程被操纵以绕过安全过滤器并生成露骨内容
- 文本模型基于个人的人口统计特征生成贬损性描述
- AI 系统通过将公开可用的照片与生成技术相结合来生成未经同意的私密图像

### StrayMark 映射

| 方面 | 值 |
|--------|-------|
| 主要模板 | ETH |
| Frontmatter 值 | `nist_genai_risks: [obscene_content]` |
| 关键部分 | 风险评估、建议 |
| 辅助文档 | SEC（内容过滤控制） |

### 建议缓解措施

- 在输入和输出端部署多层内容安全过滤器
- 实施能够抵抗常见越狱技术的强健内容分类器
- 进行专门针对内容过滤器绕过的红队演练（记录在 TES 中）

---

## 12. Value Chain and Component Integration

**标识符**: `value_chain`

由于将第三方 AI 组件、模型、数据集和服务集成到更大系统中而产生的风险，其中上游变更、漏洞或质量问题会传播到下游。

- 第三方嵌入模型引入微妙的偏见，传播到所有下游应用
- API 提供商在未通知的情况下更改模型行为，破坏依赖系统中的假设
- 微调模型继承了其基础模型中未公开的漏洞

### StrayMark 映射

| 方面 | 值 |
|--------|-------|
| 主要模板 | SBOM |
| Frontmatter 值 | `nist_genai_risks: [value_chain]` |
| 关键部分 | 第三方服务、组件、依赖项 |
| 辅助文档 | ETH（第三方风险）、SEC（供应链安全） |

### 建议缓解措施

- 维护覆盖价值链中所有 AI 组件的全面 SBOM 文档
- 在 MCARD 中固定模型版本并记录预期行为基线
- 与 AI 服务提供商建立变更通知的合同要求（记录在 ADR 中）

---

## 摘要：生成式 AI 风险类别到 StrayMark 映射

| 类别 | 标识符 | 主要 StrayMark 模板 | Frontmatter 值 |
|----------|-----------|---------------------------|-------------------|
| CBRN Information | `cbrn` | ETH | `nist_genai_risks: [cbrn]` |
| Confabulation | `confabulation` | ETH | `nist_genai_risks: [confabulation]` |
| Dangerous/Violent/Hateful Content | `dangerous_content` | ETH | `nist_genai_risks: [dangerous_content]` |
| Data Privacy | `privacy` | ETH、DPIA | `nist_genai_risks: [privacy]` |
| Environmental Impacts | `environmental` | ETH | `nist_genai_risks: [environmental]` |
| Harmful Bias and Homogenization | `bias` | ETH | `nist_genai_risks: [bias]` |
| Human-AI Configuration | `human_ai_config` | ETH | `nist_genai_risks: [human_ai_config]` |
| Information Integrity | `information_integrity` | ETH | `nist_genai_risks: [information_integrity]` |
| Information Security | `information_security` | SEC | `nist_genai_risks: [information_security]` |
| Intellectual Property | `intellectual_property` | ETH | `nist_genai_risks: [intellectual_property]` |
| Obscene/Degrading Content | `obscene_content` | ETH | `nist_genai_risks: [obscene_content]` |
| Value Chain and Component Integration | `value_chain` | SBOM | `nist_genai_risks: [value_chain]` |

---

*NIST AI 600-1 生成式 AI 风险类别参考 --- StrayMark Framework*

<!-- Template: StrayMark | https://strangedays.tech -->
