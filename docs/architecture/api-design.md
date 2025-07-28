# MetricBox API Design

## Authentication
- JWT tokens passed via Authorization header
- Token contains `user_id`, `tenant_id`, and permissions

## Event Ingestion Service

### Event Endpoints
```
POST /api/events
POST /api/events/batch
```

### Admin Endpoints  
```
POST /admin/tenant/{tenant_id}
DELETE /admin/tenant/{tenant_id}
```

### Monitoring
```
GET /health
GET /metrics
```

## User Management Service

### Authentication
```
POST /api/auth/login
POST /api/auth/register
POST /api/auth/refresh
```

### User Operations
```
GET /api/users/profile
PUT /api/users/profile
DELETE /api/users/account
```

## Query Service

### Analytics Queries
```
GET /api/analytics/events/count
GET /api/analytics/events/breakdown
GET /api/analytics/events/top
```

## Dashboard Service

### Dashboard Management
```
GET /api/dashboards
POST /api/dashboards
PUT /api/dashboards/{id}
DELETE /api/dashboards/{id}
```

## Notification Service

### Alert Management
```
GET /api/alerts
POST /api/alerts
PUT /api/alerts/{id}
DELETE /api/alerts/{id}
```

## API Gateway

- Routes all external requests
- Handles authentication
- Rate limiting
- Request/response transformation