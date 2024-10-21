# Suppress command echo output for all targets by default
.SILENT:

.PHONY: run
run:
	cp .env.development .env
	cargo watch -x run

.PHONY: setup
setup: run_middlewares db_migrate

.PHONY: run_middlewares
run_middlewares:
	docker compose up -d
	# wait for db to be ready
	sleep 1

.PHONY: db_migrate
db_migrate:
	if [ ! -e ./bin/psqldef ]; then make install_psqldef; fi
	echo '🐔 Migrate DB'
	./bin/psqldef -U=user -W=password -h=localhost -p=5432 todo_api_development --enable-drop-table < schema.sql

.PHONY: db_migrate_dryrun
db_migrate_dryrun:
	./bin/psqldef -U=user -W=password -h=localhost -p=5432 todo_api_development --enable-drop-table < schema.sql --dry-run

.PHONY: preview_openapi
preview_openapi:
	docker run --rm -p 8081:8080 -e SWAGGER_JSON="/openapi.yaml" -v "./docs/openapi.yaml:/openapi.yaml" swaggerapi/swagger-ui

SQLDEF_VERSION='v0.17.17'
PSQLDEF_URL="https://github.com/sqldef/sqldef/releases/download/${SQLDEF_VERSION}/psqldef_darwin_arm64.zip"
.PHONY: install_psqldef
install_psqldef:
	echo '🐔 Install psqldef'
	rm -rf ./bin/psqldef
	curl -L -o psqldef.zip "${PSQLDEF_URL}"
	unzip psqldef.zip -d ./bin/
	rm psqldef.zip
