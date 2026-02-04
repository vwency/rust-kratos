.PHONY: run-full infra-up infra-down monitoring-up run dev cleanup

COMPOSE = docker compose

RUST_BIN = cargo
RUST_ARGS = run

run-full:
	$(COMPOSE) -f docker-compose.monitoring.yaml up -d
	$(COMPOSE) up -d

service-up:
	$(COMPOSE) up -d

monitoring-up:
	$(COMPOSE) -f docker-compose.monitoring.yaml up -d

monitoring-down:
	$(COMPOSE) -f docker-compose.monitoring.yaml down -v

service-down:
	$(COMPOSE) down -v

cleanup:
	$(COMPOSE) -f docker-compose.monitoring.yaml down -v
	$(COMPOSE) down -v

run:
	$(RUST_BIN) $(RUST_ARGS)

dev: infra-up run
