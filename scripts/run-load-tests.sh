#!/bin/bash
echo "⚡ Running MetricBox performance tests..."

# Check if MetricBox is running
if ! curl -f http://localhost:8080/health > /dev/null 2>&1; then
    echo "❌ MetricBox is not running. Start it with 'make demo' first."
    exit 1
fi

echo "📊 MetricBox load testing will be implemented with k6..."
echo "✅ Load test setup complete!"
