# ISO/IEC 25010:2023 — Software Quality Reference

> **Standard**: ISO/IEC 25010:2023 — Systems and software Quality Requirements and Evaluation (SQuaRE) — Product quality model
> **Replaces**: ISO/IEC 25010:2011
> **Purpose**: Reference document for DevTrail templates (ADR, REQ) that evaluate software quality characteristics.

---

## Quality Characteristics (2023 vs 2011)

The 2023 revision updates the model from 8 to **9 characteristics**, with significant renames and restructuring.

| # | Characteristic (2023) | Previous Name (2011) | Change |
|---|----------------------|---------------------|--------|
| 1 | **Functional Suitability** | Functional Suitability | No change |
| 2 | **Performance Efficiency** | Performance Efficiency | No change |
| 3 | **Compatibility** | Compatibility | No change |
| 4 | **Interaction Capability** | Usability | Renamed |
| 5 | **Reliability** | Reliability | Restructured |
| 6 | **Security** | Security | New sub: Resistance |
| 7 | **Maintainability** | Maintainability | No change |
| 8 | **Flexibility** | Portability | Renamed |
| 9 | **Safety** | *(new)* | New characteristic |

---

## Detailed Characteristics and Sub-characteristics

### 1. Functional Suitability

The degree to which a product provides functions that meet stated and implied needs.

| Sub-characteristic | Description |
|-------------------|-------------|
| Functional Completeness | Degree to which the set of functions covers all specified tasks and user objectives |
| Functional Correctness | Degree to which a product provides correct results with the needed degree of precision |
| Functional Appropriateness | Degree to which the functions facilitate the accomplishment of specified tasks and objectives |

### 2. Performance Efficiency

Performance relative to the amount of resources used under stated conditions.

| Sub-characteristic | Description |
|-------------------|-------------|
| Time Behaviour | Degree to which response, processing times, and throughput rates meet requirements |
| Resource Utilization | Degree to which amounts and types of resources used meet requirements |
| Capacity | Degree to which the maximum limits of a product parameter meet requirements |

### 3. Compatibility

Degree to which a product can exchange information and perform its required functions while sharing the same environment.

| Sub-characteristic | Description |
|-------------------|-------------|
| Co-existence | Degree to which a product can perform its functions efficiently while sharing a common environment and resources with other products |
| Interoperability | Degree to which two or more systems can exchange and use information |

### 4. Interaction Capability *(renamed from Usability)*

Degree to which a product can be used by specified users to achieve specified goals with effectiveness, efficiency, and satisfaction.

| Sub-characteristic | Description | Change from 2011 |
|-------------------|-------------|-------------------|
| Appropriateness Recognizability | Degree to which users can recognize whether a product is appropriate for their needs | No change |
| Learnability | Degree to which a product can be used to achieve specified learning goals with effectiveness, efficiency, freedom from risk, and satisfaction | No change |
| Operability | Degree to which a product has attributes that make it easy to operate and control | No change |
| User Error Protection | Degree to which a product protects users against making errors | No change |
| User Engagement | Degree to which a product provides an engaging and motivating interaction experience | Replaces "User Interface Aesthetics" |
| Inclusivity | Degree to which a product can be used by people with the widest range of characteristics and capabilities | Split from Accessibility |
| User Assistance | Degree to which a product provides appropriate help and guidance to users | Split from Accessibility |
| Self-descriptiveness | Degree to which a product presents information that makes its capabilities and use immediately obvious | New |

### 5. Reliability

Degree to which a system performs specified functions under specified conditions for a specified period of time.

| Sub-characteristic | Description | Change from 2011 |
|-------------------|-------------|-------------------|
| Faultlessness | Degree to which a system operates without faults under normal conditions | Replaces "Maturity" |
| Availability | Degree to which a system is operational and accessible when required for use | No change |
| Fault Tolerance | Degree to which a system operates as intended despite hardware or software faults | No change |
| Recoverability | Degree to which a product can recover data and re-establish the desired state after an interruption or failure | No change |

### 6. Security

Degree to which a product protects information and data.

| Sub-characteristic | Description | Change from 2011 |
|-------------------|-------------|-------------------|
| Confidentiality | Degree to which data is accessible only to those authorized to have access | No change |
| Integrity | Degree to which a system prevents unauthorized access to or modification of data | No change |
| Non-repudiation | Degree to which actions or events can be proven to have taken place | No change |
| Accountability | Degree to which actions of an entity can be traced uniquely to the entity | No change |
| Authenticity | Degree to which the identity of a subject or resource can be proved to be the one claimed | No change |
| Resistance | Degree to which a product resists attacks from unauthorized or malicious actors | New |

### 7. Maintainability

Degree of effectiveness and efficiency with which a product can be modified.

| Sub-characteristic | Description |
|-------------------|-------------|
| Modularity | Degree to which a system is composed of discrete components such that a change to one has minimal impact on others |
| Reusability | Degree to which an asset can be used in more than one system or in building other assets |
| Analysability | Degree of effectiveness and efficiency with which it is possible to assess the impact of a change |
| Modifiability | Degree to which a product can be effectively and efficiently modified without introducing defects |
| Testability | Degree of effectiveness and efficiency with which test criteria can be established and tests performed |

### 8. Flexibility *(renamed from Portability)*

Degree to which a product can be adapted for different or evolving hardware, software, or usage environments.

| Sub-characteristic | Description | Change from 2011 |
|-------------------|-------------|-------------------|
| Adaptability | Degree to which a product can be adapted for different or evolving environments | No change |
| Installability | Degree of effectiveness and efficiency with which a product can be successfully installed or uninstalled | No change |
| Replaceability | Degree to which a product can replace another specified product for the same purpose in the same environment | No change |
| Scalability | Degree to which a product can handle growing or shrinking workloads | New |

### 9. Safety *(new characteristic)*

Degree to which a product achieves acceptable levels of risk to people, business, software, property, or the environment.

| Sub-characteristic | Description |
|-------------------|-------------|
| Operational Constraint | Degree to which a product constrains its operation to within safe parameters or states |
| Risk Identification | Degree to which a product identifies risks that could affect safety |
| Fail Safe | Degree to which a product automatically places itself in a safe operating mode, or reverts to a safe condition in the event of a failure |
| Hazard Warning | Degree to which a product provides warnings of hazards |
| Safe Integration | Degree to which a product can maintain safety during and after integration with one or more components |

---

## Usage in DevTrail

- **TEMPLATE-REQ.md**: Non-Functional Requirements section uses these 9 characteristics as categories
- **TEMPLATE-ADR.md**: Consequences section evaluates decisions against relevant quality characteristics
- **TEMPLATE-TES.md**: Test planning considers quality characteristics as coverage dimensions

## Key Changes to Remember

When reviewing or creating DevTrail documents:

1. Use **"Interaction Capability"** instead of "Usability"
2. Use **"Flexibility"** instead of "Portability"
3. Always consider **"Safety"** as a quality dimension, especially for AI systems
4. **"Resistance"** (under Security) is relevant for threat assessments
5. **"Scalability"** (under Flexibility) is now a formal sub-characteristic

<!-- Reference: DevTrail | https://strangedays.tech -->
