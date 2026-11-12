# devforge / cli-devforge

Generador de scaffolds de proyecto basado en convenciones de nombres por prefijos.

Project scaffold generator based on prefix-based naming conventions.

---

## Prerrequisitos / Prerequisites

- **Rust** stable ([https://rustup.rs/](https://rustup.rs/))
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
~/.cargo/bin/devforge version
# → devforge v0.5.0 (si hay tag) o devforge dev
```

El binario se instala en `~/.cargo/bin/devforge`. Para usarlo sin ruta completa, agrega `~/.cargo/bin` a tu PATH:

The binary is installed to `~/.cargo/bin/devforge`. To use it without full path, add `~/.cargo/bin` to your PATH:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

También puedes instalar directamente desde Git / You can also install directly from Git:

```bash
cargo install --git https://github.com/jcvalenzo/cli-devforge --locked
```

Para instrucciones detalladas por sistema operativo (macOS, Linux, Windows), ver [INSTALL.md](INSTALL.md).

For detailed per-OS instructions (macOS, Linux, Windows), see [INSTALL.md](INSTALL.md).

---

## Uso / Usage

```bash
# Pantalla de inicio / Home screen
devforge

# Crear un proyecto / Create a project
devforge create api-catalog --lang go --author "yourname"
devforge create ms-auth --lang java --author "yourname"
devforge create web-admin --lang node --author "yourname"
devforge create worker-email --lang go --author "yourname"

# Listar prefijos / List prefixes
devforge list

# Ver versión / Show version
devforge version
```

**Pantalla de inicio / Home screen:**

```
  ██████╗ ███████╗██╗   ██╗███████╗ ██████╗ ██████╗  ██████╗ ███████╗
  ██╔══██╗██╔════╝██║   ██║██╔════╝██╔═══██╗██╔══██╗██╔════╝ ██╔════╝
  ██║  ██║█████╗  ██║   ██║█████╗  ██║   ██║██████╔╝██║  ███╗█████╗
  ██║  ██║██╔══╝  ╚██╗ ██╔╝██╔══╝  ██║   ██║██╔══██╗██║   ██║██╔══╝
  ██████╔╝███████╗ ╚████╔╝ ██║     ╚██████╔╝██║  ██║╚██████╔╝███████╗
  ╚═════╝ ╚══════╝  ╚═══╝  ╚═╝      ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝

  devforge v0.5.0
  Generador de scaffolds de proyecto basado en convenciones de prefijos.
  20 prefijos disponibles.

  Comandos
    devforge list      Ver prefijos y lenguajes soportados
    devforge --help    Ver todas las opciones

  Ejemplo
    devforge create api-catalog --lang go
```

La pantalla de inicio se muestra solo al ejecutar `devforge` sin argumentos. Respeta `NO_COLOR` y elimina los colores cuando la salida no es una terminal.

The home screen is shown only when running `devforge` without arguments. It respects `NO_COLOR` and strips colors when output is not a terminal.

**Salida post-creación / Post-creation output:**

```
  Project "api-catalog" created successfully in ././api-catalog

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
| `cli-` | Herramienta CLI / CLI tool | go, java |
| `infra-` | Infraestructura / Infrastructure | terraform |
| `k8s-` | Manifiestos Kubernetes / Kubernetes manifests | yaml |
| `helm-` | Helm Charts | yaml |
| `docker-` | Dockerfiles e imágenes / Dockerfiles & images | dockerfile |
| `monitoring-` | Observabilidad / Observability | prometheus |
| `producer-` | Publicador de eventos / Event publisher | go |
| `consumer-` | Consumidor de eventos / Event consumer | go |
| `stream-` | Procesamiento de streams / Stream processing | go, java |
| `connector-` | Integration with third parties | go |
| `bridge-` | Puente entre sistemas/brokers / System/broker bridge | go |
| `adapter-` | Adaptador a servicio externo / External service adapter | go |
| `lab-` | Laboratorio / pruebas de cosas / Laboratory & experimentation | go, java, python, react |

> **Nota / Note:** `worker-` anuncia `python`, pero todavía no tiene plantillas. Usa `--lang go` hasta que se agregue el scaffold de Python. / `worker-` advertises `python`, but it has no templates yet. Use `--lang go` until the Python scaffold is added.

---

## Desarrollo / Development

```bash
# Compilar / Build
make build

# Ejecutar tests / Run tests
make test

# Formatear / Format
make fmt

# Verificar formato sin modificar / Check formatting without writing
make fmt-check

# Lint / Lint code
make lint

# Cobertura de tests / Test coverage (requiere cargo-llvm-cov)
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

Equivalentes con Cargo / Cargo equivalents:

```bash
cargo build --release --locked
cargo test --all-features --locked
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo fmt --all -- --check
```

La versión del binario se inyecta automáticamente desde git tags:

The binary version is injected automatically from git tags:

```bash
./bin/devforge version
# Con tag: → devforge v0.5.0
# Sin tag: → devforge v0.5.0-3-gabcdef
# En desarrollo: → devforge dev
```

Las plantillas se embeben en el binario en tiempo de compilación; recompila después de editarlas.

Templates are embedded into the binary at compile time; rebuild after editing them.

---

## CI / GitHub Actions

| Workflow | Trigger | Checks |
|----------|---------|--------|
| `ci.yml` | Push a cualquier branch, PR | fmt, clippy, tests en Linux/macOS/Windows |
| `auto-release.yml` | Push a `main` | Auto-tag + llama a `release-build.yml` |
| `release.yml` | Push de tag `v*.*.*` o manual | Llama a `release-build.yml` |
| `release-build.yml` | Reutilizable | Build multi-platform + checksums + GitHub Release |

### Checks del CI

- **fmt** — `cargo fmt --all -- --check`
- **clippy** — `cargo clippy -- -D warnings`
- **test** — `cargo test --all-features --locked` en Linux, macOS y Windows

---

## Releases

### Flujo automático (recomendado)

Los tags y releases se generan automáticamente al hacer merge a la rama `main`:

```
feature/* → PR → develop → PR → main (auto-tag + release)
```

Al push a `main`, GitHub Actions:
1. Calcula la siguiente versión semántica según los commits desde el último tag
2. Crea y pushea el tag `vX.Y.Z`
3. Compila binarios para linux/darwin/windows (amd64 + arm64)
4. Genera checksums
5. Crea la GitHub Release con los binarios y release notes

**Estrategia de versionado:**

| Tipo de commit | Bump |
|----------------|------|
| `BREAKING CHANGE:` o `MAJOR:` | major |
| `feature:` o `feat:` | minor |
| resto (`fix:`, `docs:`, etc.) | patch |
| sin tags previos | v0.1.0 |

### Release manual (fallback)

```bash
git tag v0.5.0
git push origin v0.5.0
```

También puedes ejecutar `release.yml` con `workflow_dispatch` indicando el tag.

---

## Actualizar la aplicación / Updating the application

### 1) Con `cargo install` (recomendado / recommended)

```bash
cargo install --git https://github.com/jcvalenzo/cli-devforge --locked
```

Asegúrate de tener `~/.cargo/bin` en tu `PATH`. / Make sure `~/.cargo/bin` is in your `PATH`.

### 2) Con `make install` desde el repositorio local

```bash
cd cli-devforge
git checkout main
git pull origin main
make install
```

### 3) Descargando el binario desde GitHub Releases

Descarga la última versión desde [GitHub Releases](https://github.com/jcvalenzo/cli-devforge/releases/latest):

| Sistema Operativo / OS | Arquitectura / Arch | Archivo a descargar / File |
|-------------------------|---------------------|----------------------------|
| macOS | Intel (x86_64) | `devforge-darwin-amd64` |
| macOS | Apple Silicon (arm64) | `devforge-darwin-arm64` |
| Linux | x86_64 | `devforge-linux-amd64` |
| Linux | ARM64 | `devforge-linux-arm64` |
| Windows | x86_64 | `devforge-windows-amd64.exe` |

**macOS / Linux:**

```bash
chmod +x devforge-<os>-<arch>
mv devforge-<os>-<arch> ~/.local/bin/devforge
```

> **Nota macOS (Gatekeeper):** Si el sistema bloquea la ejecución:
> ```bash
> xattr -d com.apple.quarantine ~/.local/bin/devforge
> ```

**Windows:**

1. Renombra `devforge-windows-amd64.exe` a `devforge.exe`.
2. Colócalo en un directorio incluido en tu `PATH` (ej. `C:\tools\`).
3. Si Windows SmartScreen aparece, selecciona *Más información → Ejecutar de todas formas*.

**Verificar integridad (opcional):**

```bash
sha256sum -c checksums.txt  # Linux/macOS
```

### Migración desde la versión Go / Migrating from the Go version

La versión anterior se instalaba como `cli-devforge` en `~/go/bin`. La versión Rust usa `devforge` en `~/.cargo/bin`. Para evitar conflictos:

```bash
rm -f ~/go/bin/cli-devforge
which -a devforge cli-devforge
```

---

## Contribuir / Contributing

Ver [AGENTS.md](AGENTS.md) para guías de desarrollo y arquitectura del proyecto.

See [AGENTS.md](AGENTS.md) for development guidelines and project architecture.

---

## Licencia / License

MIT
