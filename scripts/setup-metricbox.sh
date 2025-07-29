#!/bin/bash
echo "🚀 Setting up MetricBox development environment..."

# Check prerequisites
command -v docker >/dev/null 2>&1 || { echo "❌ Docker is required but not installed."; exit 1; }
command -v docker compose >/dev/null 2>&1 || { echo "❌ Docker Compose is required but not installed."; exit 1; }

echo "✅ Prerequisites check passed"

# Create environment file
if [ ! -f .env ]; then
    cp configs/environments/.env.development .env
    echo "📄 Created .env file"
fi

echo "📦 MetricBox setup complete!"
echo "Run 'make demo' to start MetricBox"
