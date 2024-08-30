# ToDo API

## What is this?

The API for ToDo App implemented by Rust. For practice.

## Run

### 1. Setup

```shell
make setup
```

### 2. run

Run at <http://localhost:8080>.

```shell
make run
```

## Run DB schema change

<details>
  <summary>Install psqldef</summary>

```shell
make install_psqldef
```

</details>

```shell
make db_migrate
```

## See OpenAPI docs at browser

Run at <http://localhost:8081>.

```shell
make preview_openapi
```
