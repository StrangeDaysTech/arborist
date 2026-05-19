# C4 Model Diagram Guide

> This guide explains how to use the C4 Model with Mermaid syntax in DevTrail documents, particularly in ADR (Architecture Decision Record) documents.

**Languages**: English | [Español](i18n/es/C4-DIAGRAM-GUIDE.md)

---

## What is the C4 Model?

The C4 Model (Context, Containers, Components, Code) is a set of abstractions for visualizing software architecture at different zoom levels. Created by Simon Brown, it provides a consistent vocabulary for describing and communicating architecture.

Each level zooms into the previous one:

| Level | Shows | When to Use in DevTrail |
|-------|-------|------------------------|
| **1. Context** | System + users + external systems | ADR for system-level decisions, REQ for high-level requirements |
| **2. Container** | Applications, databases, services | ADR for service architecture, deployment decisions |
| **3. Component** | Internal modules within a container | ADR for module-level decisions, AILOG for significant refactors |
| **4. Code** | Classes, interfaces, functions | Rarely needed in DevTrail — use only for critical design patterns |

---

## Level 1: System Context

Shows who uses the system and what external systems it interacts with.

```mermaid
C4Context
    title System Context — E-Commerce Platform

    Person(customer, "Customer", "Browses products and places orders")
    Person(admin, "Admin", "Manages products and orders")

    System(ecommerce, "E-Commerce Platform", "Allows customers to browse and purchase products")

    System_Ext(payment, "Payment Gateway", "Processes credit card payments")
    System_Ext(email, "Email Service", "Sends transactional emails")
    System_Ext(shipping, "Shipping Provider", "Handles order fulfillment")

    Rel(customer, ecommerce, "Browses, orders", "HTTPS")
    Rel(admin, ecommerce, "Manages", "HTTPS")
    Rel(ecommerce, payment, "Processes payments", "HTTPS/REST")
    Rel(ecommerce, email, "Sends emails", "SMTP")
    Rel(ecommerce, shipping, "Creates shipments", "HTTPS/REST")
```

### Key Elements

| Element | Syntax | Description |
|---------|--------|-------------|
| Person | `Person(id, "Name", "Description")` | A user or role |
| System | `System(id, "Name", "Description")` | The system being documented |
| External System | `System_Ext(id, "Name", "Description")` | External dependency |
| Relationship | `Rel(from, to, "Label", "Technology")` | Communication flow |

---

## Level 2: Container

Zooms into the system to show the high-level technology choices: applications, data stores, and how they communicate.

```mermaid
C4Container
    title Container Diagram — E-Commerce Platform

    Person(customer, "Customer", "Browses and purchases products")

    System_Boundary(ecommerce, "E-Commerce Platform") {
        Container(webapp, "Web Application", "React, TypeScript", "Provides the shopping experience")
        Container(api, "API Service", "Rust, Actix-web", "Handles business logic and data access")
        Container(worker, "Background Worker", "Rust, Tokio", "Processes async tasks: emails, reports")
        ContainerDb(db, "Database", "PostgreSQL", "Stores products, orders, users")
        ContainerQueue(queue, "Message Queue", "RabbitMQ", "Async task distribution")
    }

    System_Ext(payment, "Payment Gateway", "External payment processor")

    Rel(customer, webapp, "Uses", "HTTPS")
    Rel(webapp, api, "Calls", "HTTPS/JSON")
    Rel(api, db, "Reads/Writes", "SQL")
    Rel(api, queue, "Publishes tasks", "AMQP")
    Rel(worker, queue, "Consumes tasks", "AMQP")
    Rel(api, payment, "Processes payments", "HTTPS/REST")
```

### Key Elements

| Element | Syntax | Description |
|---------|--------|-------------|
| Boundary | `System_Boundary(id, "Name") { ... }` | Groups containers |
| Container | `Container(id, "Name", "Tech", "Description")` | An application or service |
| Database | `ContainerDb(id, "Name", "Tech", "Description")` | A data store |
| Queue | `ContainerQueue(id, "Name", "Tech", "Description")` | A message queue |

---

## Level 3: Component

Zooms into a single container to show its internal components.

```mermaid
C4Component
    title Component Diagram — API Service

    Container_Boundary(api, "API Service") {
        Component(auth, "Auth Module", "Rust", "Handles authentication and authorization")
        Component(catalog, "Catalog Module", "Rust", "Product search and management")
        Component(orders, "Orders Module", "Rust", "Order processing and fulfillment")
        Component(payments, "Payments Module", "Rust", "Payment integration and reconciliation")
    }

    ContainerDb(db, "Database", "PostgreSQL")
    System_Ext(payment, "Payment Gateway")

    Rel(auth, db, "Reads users", "SQL")
    Rel(catalog, db, "Reads/Writes products", "SQL")
    Rel(orders, db, "Reads/Writes orders", "SQL")
    Rel(orders, payments, "Initiates payment")
    Rel(payments, payment, "Processes", "HTTPS/REST")
```

### Key Elements

| Element | Syntax | Description |
|---------|--------|-------------|
| Boundary | `Container_Boundary(id, "Name") { ... }` | Groups components within a container |
| Component | `Component(id, "Name", "Tech", "Description")` | An internal module or package |

---

## Level 4: Code

Shows classes, interfaces, and their relationships. **Rarely needed** in DevTrail — use only for critical design patterns that require documentation.

For code-level diagrams, use standard Mermaid class diagrams instead of C4:

```mermaid
classDiagram
    class OrderService {
        +create_order(cart: Cart) Order
        +cancel_order(id: OrderId) Result
        -validate_stock(items: Vec~Item~) bool
    }
    class PaymentService {
        +process_payment(order: Order) PaymentResult
        +refund(payment_id: PaymentId) Result
    }
    class Order {
        +id: OrderId
        +status: OrderStatus
        +items: Vec~OrderItem~
        +total: Decimal
    }

    OrderService --> PaymentService : uses
    OrderService --> Order : creates
```

---

## PlantUML Alternative

For teams that prefer PlantUML, equivalent syntax is available using the [C4-PlantUML](https://github.com/plantuml-stdlib/C4-PlantUML) library.

### Context (PlantUML)

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Context.puml

Person(customer, "Customer", "Browses and purchases")
System(ecommerce, "E-Commerce Platform", "Shopping platform")
System_Ext(payment, "Payment Gateway", "Processes payments")

Rel(customer, ecommerce, "Uses", "HTTPS")
Rel(ecommerce, payment, "Processes payments", "REST")
@enduml
```

### Container (PlantUML)

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Container.puml

Person(customer, "Customer")
System_Boundary(c1, "E-Commerce Platform") {
    Container(webapp, "Web App", "React", "UI")
    Container(api, "API", "Rust", "Business logic")
    ContainerDb(db, "Database", "PostgreSQL", "Data store")
}

Rel(customer, webapp, "Uses", "HTTPS")
Rel(webapp, api, "Calls", "JSON/HTTPS")
Rel(api, db, "Reads/Writes", "SQL")
@enduml
```

---

## Integration with DevTrail Documents

### In ADR Documents

Add a C4 diagram in the `## Architecture Diagram` section when the decision:
- Introduces or removes a system, service, or data store
- Changes inter-service communication patterns
- Modifies system boundaries or deployment topology

### In REQ Documents

Use a Level 1 (Context) diagram to illustrate:
- Who interacts with the system
- What external systems are involved
- High-level data flows

### Choosing the Right Level

| Decision Scope | C4 Level | Example |
|---------------|----------|---------|
| "We will integrate with Stripe for payments" | Context | New external system |
| "We will split the monolith into microservices" | Container | New service architecture |
| "We will extract auth into a separate module" | Component | Internal restructuring |
| "We will use the Strategy pattern for pricing" | Code (class diagram) | Design pattern |

---

## References

- [C4 Model — Simon Brown](https://c4model.com/)
- [Mermaid C4 Diagrams](https://mermaid.js.org/syntax/c4.html)
- [C4-PlantUML](https://github.com/plantuml-stdlib/C4-PlantUML)

---

*DevTrail v4.1.1 | [Strange Days Tech](https://strangedays.tech)*
