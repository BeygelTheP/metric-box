DOCKER_COMPOSE := docker compose

.PHONY: help setup demo test load-test build clean logs

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Targets:'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'

setup: ## Initial setup - install dependencies and configure MetricBox environment
	@echo "🚀 Setting up MetricBox..."
	@./scripts/setup-metricbox.sh

demo: ## Start the full MetricBox demo environment
	@echo "📦 Starting MetricBox demo..."
	@echo "🔨 Building services first..."
	@$(DOCKER_COMPOSE) build
	@echo "🚀 Starting services..."
	@$(DOCKER_COMPOSE) --env-file .env up -d
	@echo "⏳ Waiting for MetricBox services to start..."
	@sleep 10
	@./scripts/start-demo.sh
	@echo "✅ MetricBox demo ready!"
	@echo "📊 Grafana Dashboard: http://localhost:3000"
	@echo "📦 MetricBox API: http://localhost:8080"

build: ## Build all MetricBox services
	@echo "🔨 Building MetricBox services..."
	@$(DOCKER_COMPOSE) build

test: ## Run all MetricBox tests
	@echo "🧪 Running MetricBox tests..."
	@./scripts/run-tests.sh

load-test: ## Run MetricBox performance tests
	@echo "⚡ Running MetricBox load tests..."
	@./scripts/run-load-tests.sh

clean: ## Clean up MetricBox containers and volumes
	@echo "🧹 Cleaning up MetricBox..."
	@$(DOCKER_COMPOSE) down -v
	@docker system prune -f
	@docker volume prune -f
	@echo "✅ MetricBox cleanup complete"

logs: ## Show logs for all MetricBox services
	@$(DOCKER_COMPOSE) logs -f

status: ## Show status of all MetricBox services
	@$(DOCKER_COMPOSE) ps --format "table {{.Name}}\t{{.Image}}\t{{.Status}}\t{{.Ports}}"