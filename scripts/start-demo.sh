#!/bin/bash
echo "📦 Starting MetricBox demo..."

# Wait for core services
echo "⏳ Waiting for MetricBox services to be ready..."
timeout 60 bash -c 'until curl -f http://localhost:8080/health; do sleep 2; done'

if [ $? -eq 0 ]; then
    echo "✅ MetricBox services are ready!"
    echo "📊 Generating sample data..."
    # Trigger sample data generation
    curl -X POST http://localhost:8081/api/generate-sample || echo "⚠️  Sample data generation endpoint not ready yet"
else
    echo "⚠️  MetricBox services taking longer than expected to start"
fi

echo "📦 MetricBox demo is ready!"
echo "🌐 Access points:"
echo "  📊 Grafana Dashboard: http://localhost:3000 (admin/admin)"
echo "  📦 MetricBox API: http://localhost:8080"
echo "  📈 Prometheus: http://localhost:9090"
echo "  🔍 Jaeger Tracing: http://localhost:16686"
