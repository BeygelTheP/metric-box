#!/bin/bash
echo "🧪 Running MetricBox test suite..."

echo "🦀 Testing Rust services..."
for service in event-ingestion query-service data-generator; do
    if [ -f "services/$service/Cargo.toml" ]; then
        echo "  Testing metricbox-$service..."
        (cd services/$service && cargo test)
    fi
done

echo "🔧 Testing .NET services..."  
for service in user-management dashboard-service notification-service api-gateway; do
    if [ -f "services/$service"/*.csproj ]; then
        echo "  Testing MetricBox $service..."
        (cd services/$service && dotnet test)
    fi
done

echo "✅ MetricBox tests complete!"
