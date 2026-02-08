# CrabTalk: Universal Mesh & Observability Plane 🦀🔭

![Rust](https://img.shields.io/badge/Language-Rust-orange)
![Observability](https://img.shields.io/badge/Stack-OpenTelemetry%20%2B%20Jaeger-blue)
![Crates](https://img.shields.io/badge/Crates-tracing--rs%20%7C%20opentelemetry-purple)

**CrabTalk** is a high-performance connectivity mesh designed to bridge disparate communication protocols (Real-time Web, IoT, Enterprise Streams).

Beyond just routing packets, CrabTalk implements **Observability by Design**. Instead of reinventing the wheel, it fully integrates the **OpenTelemetry (OTel)** ecosystem to provide production-grade visibility. The system solves the complex challenge of **Context Propagation** across heterogeneous protocols (e.g., passing a Trace ID from a WebSocket frame into a Kafka Record Header).

## 🏗️ Architecture & Integration

CrabTalk uses the standard Rust observability stack to instrument the core event loop:

* **Facade:** `tracing` (for lightweight instrumentation calls).
* **Backend:** `opentelemetry` & `opentelemetry-otlp` (for exporting data).
* **Visualization:** Jaeger (Traces) and Prometheus (Metrics).



### The Integration Challenge: Cross-Protocol Context
The main engineering feat is maintaining the "Trace Chain" when the transport layer changes.

1.  **Ingress (WebSocket):** The system extracts the `traceparent` from the WS handshake or custom header.
2.  **Internal Processing:** A `tracing::Span` is created for the duration of the event processing.
3.  **Egress (Kafka/gRPC):** The active OTel context is injected into the outgoing message metadata (Kafka Headers or gRPC Metadata).

## 📚 Observability Stack Implementation

### 1. Instrumentation (Rust `tracing` ecosystem)
We use idiomatic Rust macros to instrument async functions without polluting logic.
