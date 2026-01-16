.PHONY: infra-up infra-down run dev

COMPOSE = docker compose

RUST_BIN = cargo
RUST_ARGS = run

infra-up:
	$(COMPOSE) up -d

infra-down:
	$(COMPOSE) down -v

run:
	$(RUST_BIN) $(RUST_ARGS)

dev: infra-up run
