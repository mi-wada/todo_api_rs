# ToDo API

## What is this?

The API for ToDo App implemented by Rust. For practice.

## Run

Run at <http://localhost:8080>.

```shell
cargo run
```

## See OpenAPI docs at browser

Run at <http://localhost:8081>.

```shell
docker run --rm -p 8081:8080 -e SWAGGER_JSON="/openapi.yaml" -v "./docs/openapi.yaml:/openapi.yaml" swaggerapi/swagger-ui
```
