.PHONY: run-full kratos-up hydra-up app-up infra-up monitoring-up monitoring-down kratos-down hydra-down app-down cleanup force-cleanup run dev

COMPOSE = docker compose
RUST_BIN = cargo
RUST_ARGS = run

run-full:
	$(COMPOSE) -f docker/docker-compose.kratos.yaml -f docker/docker-compose.hydra.yaml -f docker/docker-compose.app.yaml up -d

infra-up:
	$(COMPOSE) -f docker/docker-compose.kratos.yaml up -d
	$(COMPOSE) -f docker/docker-compose.hydra.yaml up -d

kratos-up:
	$(COMPOSE) -f docker/docker-compose.kratos.yaml up -d

hydra-up:
	$(COMPOSE) -f docker/docker-compose.hydra.yaml up -d

app-up:
	$(COMPOSE) -f docker/docker-compose.app.yaml up -d --build

monitoring-up:
	$(COMPOSE) -f docker/docker-compose.monitoring.yaml up -d

monitoring-down:
	$(COMPOSE) -f docker/docker-compose.monitoring.yaml down -v

kratos-down:
	$(COMPOSE) -f docker/docker-compose.kratos.yaml down -v

hydra-down:
	$(COMPOSE) -f docker/docker-compose.hydra.yaml down -v

app-down:
	$(COMPOSE) -f docker/docker-compose.app.yaml down -v

cleanup:
	$(COMPOSE) -f docker/docker-compose.kratos.yaml -f docker/docker-compose.hydra.yaml -f docker/docker-compose.app.yaml -f docker/docker-compose.monitoring.yaml down -v

run:
	$(RUST_BIN) $(RUST_ARGS)

dev: infra-up run
