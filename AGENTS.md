# AGENTS.md

- Language: Go (see `go.mod`).
- Module path: `github.com/jcvalenzo/cli-devforge`
- Build/Install: `make build`, `make install`, `make uninstall`, `make clean`
- Test: `make test` or `go test ./...`
- Run: `./bin/devforge <command>`
- Templates are embedded via `go:embed` in `templates/embed.go`. To add a new prefix, create the directory under `templates/` and add it to the embed directive.
- Prefix registry lives in `internal/prefix/registry.go`. Add new entries there.
- Template rendering uses Go's `text/template`. Variables: `{{.FullName}}`, `{{.Prefix}}`, `{{.ProjectName}}`, `{{.Lang}}`, `{{.Author}}`, `{{.ModulePath}}`, `{{.License}}`, `{{.Date}}`, `{{.Description}}`.
- Template file names are NOT rendered — only file contents. Use fixed directory names.

## Makefile Targets

| Target | Description |
|--------|-------------|
| `make build` | Compila binario en `bin/devforge` con ldflags |
| `make build-all` | Cross-compile para linux/darwin/windows (amd64/arm64) en `dist/` |
| `make test` | Ejecuta tests con coverage |
| `make vet` | Análisis estático con `go vet` |
| `make fmt` | Verifica y corrige formato con `gofmt` |
| `make lint` | Lint con `golangci-lint` |
| `make coverage` | Genera HTML de cobertura |
| `make install` | Instala binario en `~/go/bin/cli-devforge` |
| `make uninstall` | Elimina binario de `~/go/bin/` |
| `make clean` | Limpia `bin/`, `dist/`, `coverage.*` |
| `make all` | clean → fmt → vet → lint → build → test |

## Version Injection

La versión se inyecta via ldflags en `cmd.version` (NO `main.version`):

```makefile
LDFLAGS=-s -w -X github.com/jcvalenzo/cli-devforge/cmd.version=$(VERSION)
```

El valor de `VERSION` viene de `git describe --tags --always --dirty`.

## CI/CD

- **ci.yml**: Trigger en push/PR a cualquier branch. Jobs paralelos: build, test, vet, fmt, lint (usa `golangci-lint-action@v9` con `version: v2.12`).
- **auto-release.yml**: Trigger en push a `main`. Calcula la versión semántica desde los últimos commits, crea el tag `vX.Y.Z`, lo pushea y genera la GitHub Release con binarios multi-platform + checksums.
- **release.yml**: Trigger en push manual de tag `v*.*.*`. Cross-compile + checksums + GitHub Release (fallback manual).

## Gitflow

```
feature/* → PR → develop → PR → main  (auto-tag + release en main)
```

## Linter Config

Archivo `.golangci.yml` (v2) habilita:
- **Linters**: errcheck, govet, ineffassign, staticcheck, unused, misspell
- **Formatters**: gofmt, goimports
- Archivos de test excluidos de errcheck (`linters.exclusions.rules`)
