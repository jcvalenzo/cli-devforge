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
