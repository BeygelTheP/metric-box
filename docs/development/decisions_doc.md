# MetricBox Technical Decisions

## Architecture Decisions

### HTTP Framework: Axum vs Warp
**Decision:** Use Axum for Rust services

**Reasoning:**
- More modern (created by Tokio team)
- Simpler mental model than Warp's filter system
- Better ecosystem integration
- Growing adoption in community
- Easier to understand for new developers

**Trade-offs:**
- Warp has longer track record
- Axum is newer, fewer examples online

### Data Isolation: Shared vs Separate Tables
**Decision:** Table per tenant in single ClickHouse database

**Reasoning:**
- Complete data isolation between tenants
- Better performance per tenant
- Easier to implement tenant-specific optimizations
- Lower operational complexity than separate databases

**Trade-offs:**
- More complex table management
- Cross-tenant queries become impossible
- Schema migrations affect multiple tables

### Tenant Provisioning: Auto-create vs Admin API
**Decision:** Admin API for explicit tenant table creation

**Reasoning:**
- Consistent approach for creation and deletion
- No race conditions on first event
- Explicit provisioning process
- Better error handling and validation

**Trade-offs:**
- More complex service interactions
- User Management must call Event Ingestion
- Additional API endpoints to maintain

### Communication: HTTP vs gRPC
**Decision:** HTTP for all service communication initially

**Reasoning:**
- Simpler to implement and debug
- Easy to test with standard tools (curl, Postman)
- Good enough performance for initial implementation
- Lower learning curve

**Future consideration:**
- May upgrade to gRPC for high-frequency internal calls
- Keep HTTP for external APIs

## Technology Choices

### Time-Series Database: ClickHouse
**Decision:** Keep ClickHouse despite origin concerns

**Reasoning:**
- Excellent performance for analytical queries
- Good ecosystem and documentation
- Technical merits outweigh other considerations
- Industry-standard for time-series analytics

### Authentication: JWT
**Decision:** Use JWT tokens for authentication

**Reasoning:**
- Stateless authentication
- Easy to pass between microservices
- Standard format with good library support
- Contains tenant/user context

### Caching: Redis
**Decision:** Redis for caching and real-time counters

**Reasoning:**
- High performance for counter operations
- Good pub/sub support for real-time alerts
- Well-established caching patterns
- Easy integration with existing stack

## Development Decisions

### Repository Structure: Monorepo
**Decision:** Single repository for all services

**Reasoning:**
- Easier for portfolio demonstration
- Simplified dependency management
- Unified CI/CD pipeline
- Better for learning project scope

### Build System: Make + Docker
**Decision:** Makefile for development commands

**Reasoning:**
- Industry standard for development workflows
- Self-documenting with help targets
- Easy for reviewers to understand and use
- Good abstraction over Docker Compose complexity

## Future Considerations

### Scalability
- Consider event streaming (Kafka) for higher throughput
- Implement read replicas for Query Service
- Add connection pooling and caching layers

### Observability
- Implement distributed tracing with Jaeger
- Add custom metrics for business logic
- Create runbooks for common operational tasks

### Security
- Implement API rate limiting
- Add request validation and sanitization
- Consider encryption at rest for sensitive data