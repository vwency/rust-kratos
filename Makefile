.PHONY: run-full kratos-up monitoring-up monitoring-down kratos-down cleanup force-cleanup run dev
COMPOSE = docker compose
RUST_BIN = cargo
RUST_ARGS = run

run-full:
	$(COMPOSE) -f docker/docker-compose.kratos.yaml -f docker/docker-compose.monitoring.yaml up -d

kratos-up:
	$(COMPOSE) -f docker/docker-compose.kratos.yaml up -d

monitoring-up:
	$(COMPOSE) -f docker/docker-compose.monitoring.yaml up -d

monitoring-down:
	$(COMPOSE) -f docker/docker-compose.monitoring.yaml down -v

kratos-down:
	$(COMPOSE) -f docker/docker-compose.kratos.yaml down -v

cleanup:
	$(COMPOSE) -f docker/docker-compose.kratos.yaml -f docker/docker-compose.monitoring.yaml down -v

run:
	$(RUST_BIN) $(RUST_ARGS)

dev: run-full run
