# Rust Actix Web Template

Actix Web 4 template repository.

## Features

* Configuration through environment variables
* GitHub tests workflow
* Integration tests
* Tracing
* Workspace

## Environment Variables

| Name                   | Description                   | Example |
|------------------------|-------------------------------|---------|
| APP__HTTP_SERVER__HOST | Interface to bind HTTP server | 0.0.0.0 |
| APP__HTTP_SERVER__PORT | Port to bind HTTP server      | 0       |

## Workflow Secrets

| Name          | Value         |
|---------------|---------------|
| CODECOV_TOKEN | Codecov Token |
