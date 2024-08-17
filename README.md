# ToDo API

## What is this?

The API for ToDo App implemented by Rust. For practice.

## Run

Run at <http://localhost:8080>.

```shell
cp .env.development .env
cargo watch -x run
```

## Run DB schema change

<details>
  <summary>Install psqldef</summary>

```shell
SQLDEF_VERSION="v0.17.17"
PSQLDEF_URL="https://github.com/sqldef/sqldef/releases/download/${SQLDEF_VERSION}/psqldef_darwin_arm64.zip"

curl -L -o psqldef.zip $PSQLDEF_URL
unzip psqldef.zip -d ./bin/
rm psqldef.zip
```

</details>

```shell
./bin/psqldef -U=user -W=password -h=localhost -p=5432 todo_api_development < schema.sql
```

## See OpenAPI docs at browser

Run at <http://localhost:8081>.

```shell
docker run --rm -p 8081:8080 -e SWAGGER_JSON="/openapi.yaml" -v "./docs/openapi.yaml:/openapi.yaml" swaggerapi/swagger-ui
```
