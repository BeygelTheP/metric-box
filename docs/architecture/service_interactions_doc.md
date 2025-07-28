# MetricBox Service Interactions

## Authentication Flow

### User Registration
```
Client → API Gateway → User Management → PostgreSQL
                    → Event Ingestion (create tenant table)
```

### Event Submission
```
Client → API Gateway (JWT auth) → Event Ingestion → ClickHouse + Redis
```

### Query Flow
```
Client → API Gateway (JWT auth) → Query Service → ClickHouse/Redis → Response
```

## Inter-Service Communication

### User Management ↔ Event Ingestion
**Tenant Creation:**
```http
POST /admin/tenant/{tenant_id}
Authorization: Bearer {admin_jwt}
```

**Tenant Deletion:**
```http
DELETE /admin/tenant/{tenant_id}
Authorization: Bearer {admin_jwt}
```

### API Gateway ↔ All Services
**JWT Token Validation:**
- API Gateway validates JWT
- Extracts `tenant_id` and `user_id`
- Passes to downstream services via headers

**Headers passed downstream:**
```
X-Tenant-ID: tenant_abc123
X-User-ID: user_456
X-Session-ID: session_789
```

### Event Ingestion ↔ Notification Service
**Alert Triggers:**
- Event Ingestion publishes to Redis pub/sub
- Notification Service subscribes to alert channels
```
PUBLISH alerts:tenant_abc123 '{"event": "high_error_rate", "count": 100}'
```

## Data Flow Patterns

### Real-time Event Processing
```
Event → Ingestion → ClickHouse (permanent)
                 → Redis (counters/cache)
                 → Redis pub/sub (alerts)
```

### Query Processing
```
Query → Query Service → Redis (check cache)
                     → ClickHouse (if cache miss)
                     → Redis (cache result)
                     → Response
```

### Dashboard Updates
```
Dashboard Service → Query Service → Cached results
                 → WebSocket (real-time updates)
```

## Error Handling

### Service Unavailable
- API Gateway returns 503 if downstream service down
- Implement circuit breaker pattern
- Graceful degradation where possible

### Data Consistency
- Event Ingestion: Best effort delivery
- Query Service: Eventually consistent reads
- User Management: Strong consistency required