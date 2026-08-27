# devforge / cli-devforge

Generador de scaffolds de proyecto basado en convenciones de nombres por prefijos.

Project scaffold generator based on prefix-based naming conventions.

---

## Prerrequisitos / Prerequisites

- **Go** >= 1.22 ([https://go.dev/dl/](https://go.dev/dl/))
- **Git**
- **GitHub CLI (gh)** (opcional / optional) — para `gh repo create`

---

## Instalación Rápida / Quick Install

```bash
git clone https://github.com/jcvalenzo/cli-devforge.git
cd cli-devforge
make install
```

**Verificar / Verify:**

```bash
~/go/bin/cli-devforge version
# → devforge v1.0.0 (si hay tag) o devforge dev
```

El binario se instala en `~/go/bin/cli-devforge`. Para usarlo sin ruta completa, agrega `~/go/bin` a tu PATH:

The binary is installed to `~/go/bin/cli-devforge`. To use it without full path, add `~/go/bin` to your PATH:

```bash
export PATH="$HOME/go/bin:$PATH"
```

Para instrucciones detalladas por sistema operativo (macOS, Linux, Windows), ver [INSTALL.md](INSTALL.md).

For detailed per-OS instructions (macOS, Linux, Windows), see [INSTALL.md](INSTALL.md).

---

## Uso / Usage

```bash
# Crear un proyecto / Create a project
cli-devforge create api-catalog --lang go --author "yourname"
cli-devforge create ms-auth --lang java --author "yourname"
cli-devforge create web-admin --lang node --author "yourname"
cli-devforge create worker-email --lang python --author "yourname"

# Listar prefijos / List prefixes
cli-devforge list

# Ver versión / Show version
cli-devforge version
```

**Salida post-creación / Post-creation output:**

```
  Project "api-catalog" created successfully in ./api-catalog

  Next steps:
    cd api-catalog
    git init
    git add .
    git commit -m "Initial commit"

  Push to GitHub:
    gh repo create api-catalog --public --source=. --remote=origin --push

  Or configure remote manually:
    git remote add origin git@github.com:YOUR_GITHUB_USERNAME/api-catalog.git
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
| `producer-` | Publicador de eventos / Event publisher | go |
| `consumer-` | Consumidor de eventos / Event consumer | go |
| `stream-` | Procesamiento de streams / Stream processing | go |
| `connector-` | Integration with third parties | go |
| `bridge-` | Puente entre sistemas/brokers / System/broker bridge | go |
| `adapter-` | Adaptador a servicio externo / External service adapter | go |

---

## Desarrollo / Development

```bash
# Compilar / Build
make build

# Compilar para todas las plataformas / Build for all platforms
make build-all

# Ejecutar tests / Run tests
make test

# Verificar formato / Check formatting
make fmt

# Verificar código / Vet code
make vet

# Lint / Lint code
make lint

# Cobertura de tests / Test coverage
make coverage

# Compilar + testear (como el CI) / Build + test (like CI)
make all

# Instalar globalmente / Install globally
make install

# Desinstalar / Uninstall
make uninstall

# Limpiar / Clean
make clean
```

La versión del binario se inyecta automáticamente desde git tags:

The binary version is injected automatically from git tags:

```bash
./bin/devforge version
# Con tag: → devforge v1.0.0
# Sin tag: → devforge v1.0.0-3-gabcdef
# En desarrollo: → devforge dev
```

---

## CI / GitHub Actions

El proyecto incluye workflows de GitHub Actions:

| Workflow | Trigger | Checks |
|----------|---------|--------|
| `ci.yml` | Push a cualquier branch, PR | build, test, vet, fmt, lint |
| `release.yml` | Push de tag `v*.*.*` | Build multi-platform + GitHub Release |

### Checks del CI

- **build** — Compila el binario
- **test** — Ejecuta tests con coverage
- **vet** — Análisis estático de Go
- **fmt** — Verifica formateo con gofmt
- **lint** — Análisis con golangci-lint

---

## Releases

Para crear un release:

```bash
git tag v1.0.0
git push origin v1.0.0
```

GitHub Actions automaticamente:
1. Compila binarios para linux/darwin/windows (amd64 + arm64)
2. Genera checksums
3. Crea una GitHub Release con los binarios y release notes

---

## Contribuir / Contributing

Ver [AGENTS.md](AGENTS.md) para guías de desarrollo y arquitectura del proyecto.

See [AGENTS.md](AGENTS.md) for development guidelines and project architecture.

---

## Licencia / License

MIT
