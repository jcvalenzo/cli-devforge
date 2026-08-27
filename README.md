# devforge / cli-devforge

Generador de scaffolds de proyecto basado en convenciones de nombres por prefijos.

Project scaffold generator based on prefix-based naming conventions.

---

## Instalación Rápida / Quick Install

**macOS / Linux:**

```bash
git clone https://github.com/jcvalenzo/cli-devforge.git
cd cli-devforge
make install
```

**Windows (Git Bash / WSL):**

```bash
git clone https://github.com/jcvalenzo/cli-devforge.git
cd cli-devforge
make install
```

**Go directo / Direct Go:**

```bash
go install github.com/jcvalenzo/cli-devforge@latest
```

Para instrucciones detalladas por sistema operativo, ver [INSTALL.md](INSTALL.md).

For detailed per-OS instructions, see [INSTALL.md](INSTALL.md).

---

## Uso / Usage

```bash
# Crear un proyecto / Create a project
devforge create api-catalog --lang go --author "yourname"
devforge create ms-auth --lang java --author "yourname"
devforge create web-admin --lang node --author "yourname"
devforge create worker-email --lang python --author "yourname"

# Listar prefijos / List prefixes
devforge list

# Ver versión / Show version
devforge version
```

---

## Prefijos Disponibles / Available Prefixes

| Prefijo / Prefix | Significado / Meaning | Idiomas / Languages |
|------------------|-----------------------|---------------------|
| `app-` | Aplicación completa / Full application | go, java |
| `api-` | API (monolito o simple) / API (monolith or simple) | go, java |
| `ms-` | Microservicio / Microservice | go, java |
| `web-` | Frontend web | node, go |
| `worker-` | Proceso asíncrono / Async process | go, python |
| `scheduler-` | Tareas programadas / Scheduled tasks | go |
| `lib-` | Librería compartida / Shared library | go, java |
| `cli-` | Herramienta CLI / CLI tool | go |
| `infra-` | Infraestructura / Infrastructure | terraform |
| `k8s-` | Manifiestos Kubernetes / Kubernetes manifests | yaml |
| `helm-` | Helm Charts | yaml |
| `docker-` | Dockerfiles e imágenes / Dockerfiles & images | dockerfile |
| `monitoring-` | Observabilidad / Observability | prometheus |

---

## Desarrollo / Development

```bash
# Compilar / Build
make build

# Ejecutar tests / Run tests
make test

# Compilar + testear / Build + test
make all

# Limpiar / Clean
make clean
```

---

## Contribuir / Contributing

Ver [AGENTS.md](AGENTS.md) para guías de desarrollo y arquitectura del proyecto.

See [AGENTS.md](AGENTS.md) for development guidelines and project architecture.

---

## Licencia / License

MIT
