.PHONY: run-full service-up monitoring-up monitoring-down service-down cleanup force-cleanup run dev
COMPOSE = docker compose
RUST_BIN = cargo
RUST_ARGS = run

run-full:
	$(COMPOSE) -f docker/docker-compose.yaml -f docker/docker-compose.monitoring.yaml up -d

service-up:
	$(COMPOSE) -f docker/docker-compose.yaml up -d

monitoring-up:
	$(COMPOSE) -f docker/docker-compose.monitoring.yaml up -d

monitoring-down:
	$(COMPOSE) -f docker/docker-compose.monitoring.yaml down -v

service-down:
	$(COMPOSE) -f docker/docker-compose.yaml down -v

cleanup:
	$(COMPOSE) -f docker/docker-compose.yaml -f docker/docker-compose.monitoring.yaml down -v

run:
	$(RUST_BIN) $(RUST_ARGS)

dev: run-full run
