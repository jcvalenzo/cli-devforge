# devforge

Project scaffold generator based on prefix-based naming conventions.

## Installation

```bash
go install github.com/jcvalenzo/cli-devforge@latest
```

## Usage

```bash
# Create a project
devforge create api-catalog --lang go --author "yourname"
devforge create ms-auth --lang java --author "yourname"
devforge create web-admin --lang node --author "yourname"
devforge create worker-email --lang python --author "yourname"

# List available prefixes
devforge list

# Show version
devforge version
```

## Available Prefixes

| Prefix | Description | Languages |
|--------|-------------|-----------|
| `app-` | Aplicacion completa | go, java |
| `api-` | API (monolito o API simple) | go, java |
| `ms-` | Microservicio | go, java |
| `web-` | Frontend web | node, go |
| `worker-` | Proceso asincrono | go, python |
| `scheduler-` | Tareas programadas | go |
| `lib-` | Libreria compartida | go, java |
| `cli-` | Herramienta CLI | go |
| `infra-` | Infraestructura | terraform |
| `k8s-` | Manifiestos Kubernetes | yaml |
| `helm-` | Helm Charts | yaml |
| `docker-` | Dockerfiles e imagenes | dockerfile |
| `monitoring-` | Observabilidad | prometheus |

## Development

```bash
# Build
go build -o bin/devforge .

# Run tests
go test ./...

# List commands
./bin/devforge --help
```

## Adding a New Prefix

1. Create template directory: `templates/<prefix>/<lang>/`
2. Add `.tmpl` files with Go template syntax
3. Register in `internal/prefix/registry.go`
4. Add embed directive in `templates/embed.go`

## License

MIT
