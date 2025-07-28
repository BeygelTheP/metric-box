# MetricBox System Design

## Overview
MetricBox is a distributed real-time metrics processing platform built with .NET and Rust microservices.

## Architecture Components

### Core Services
- **Event Ingestion** (Rust) - High-throughput event collection
- **Query Service** (Rust) - Fast metrics queries  
- **User Management** (.NET) - Authentication and user profiles
- **Dashboard Service** (.NET) - Dashboard configurations
- **Notification Service** (.NET) - Real-time alerts
- **API Gateway** (.NET) - Request routing and rate limiting

### Data Layer
- **PostgreSQL** - User data, dashboard configs
- **ClickHouse** - Time-series metrics data
- **Redis** - Caching and real-time data

### Monitoring
- **Prometheus** - Metrics collection
- **Grafana** - Visualization dashboards
- **Jaeger** - Distributed tracing
