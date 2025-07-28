# MetricBox Data Model

## Event Structure

### Core Event Format
```json
{
  "event": "purchase",
  "timestamp": "2025-01-01T12:00:00Z",
  "user_id": "user_123",
  "session_id": "session_456",
  "properties": {
    "amount": 29.99,
    "product_id": "shoe_001",
    "currency": "USD"
  }
}
```

### Auto-added Metadata
```json
{
  "id": "uuid-generated",
  "tenant_id": "tenant_abc123",
  "received_at": "2025-01-01T12:00:01Z",
  "ip_address": "192.168.1.1",
  "user_agent": "Mozilla/5.0..."
}
```

## Data Storage Strategy

### Tenant Isolation
- **Approach:** Table per tenant
- **ClickHouse:** Single database, multiple tables
- **Table naming:** `events_tenant_{tenant_id}`

### ClickHouse Schema
```sql
CREATE TABLE events_tenant_abc123 (
    id UUID,
    event String,
    timestamp DateTime64(3),
    user_id String,
    session_id Nullable(String),
    properties String,  -- JSON
    received_at DateTime64(3),
    ip_address String,
    user_agent String
) ENGINE = MergeTree()
ORDER BY (timestamp, event)
PARTITION BY toYYYYMM(timestamp)
```

### Redis Usage
```
tenant:{tenant_id}:counters:{event_type} -> count
tenant:{tenant_id}:recent_events -> list of recent events
tenant:{tenant_id}:active_users -> set of active user_ids
```

### PostgreSQL Schema

#### Users Table
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email VARCHAR UNIQUE NOT NULL,
    password_hash VARCHAR NOT NULL,
    tenant_id UUID NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
```

#### Tenants Table
```sql
CREATE TABLE tenants (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    api_key VARCHAR UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
```

#### Dashboards Table
```sql
CREATE TABLE dashboards (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name VARCHAR NOT NULL,
    config JSONB NOT NULL,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMP DEFAULT NOW()
);
```