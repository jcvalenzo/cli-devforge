package prefix

// Prefix represents a project type prefix with its metadata.
type Prefix struct {
	Name        string
	Description string
	Langs       []string
	DefaultLang string
	TemplateDir string
}

// HasLang returns true if the prefix supports the given language.
func (p *Prefix) HasLang(lang string) bool {
	for _, l := range p.Langs {
		if l == lang {
			return true
		}
	}
	return false
}

var registry = []Prefix{
	{
		Name:        "app",
		Description: "Aplicacion completa",
		Langs:       []string{"go", "java"},
		DefaultLang: "go",
		TemplateDir: "app",
	},
	{
		Name:        "api",
		Description: "API (monolito o API simple)",
		Langs:       []string{"go", "java"},
		DefaultLang: "go",
		TemplateDir: "api",
	},
	{
		Name:        "ms",
		Description: "Microservicio",
		Langs:       []string{"go", "java"},
		DefaultLang: "go",
		TemplateDir: "ms",
	},
	{
		Name:        "web",
		Description: "Frontend web",
		Langs:       []string{"node", "go"},
		DefaultLang: "node",
		TemplateDir: "web",
	},
	{
		Name:        "worker",
		Description: "Proceso asincrono",
		Langs:       []string{"go", "python"},
		DefaultLang: "go",
		TemplateDir: "worker",
	},
	{
		Name:        "scheduler",
		Description: "Tareas programadas",
		Langs:       []string{"go"},
		DefaultLang: "go",
		TemplateDir: "scheduler",
	},
	{
		Name:        "lib",
		Description: "Libreria compartida",
		Langs:       []string{"go", "java"},
		DefaultLang: "go",
		TemplateDir: "lib",
	},
	{
		Name:        "cli",
		Description: "Herramienta CLI",
		Langs:       []string{"go"},
		DefaultLang: "go",
		TemplateDir: "cli",
	},
	{
		Name:        "infra",
		Description: "Infraestructura",
		Langs:       []string{"terraform"},
		DefaultLang: "terraform",
		TemplateDir: "infra",
	},
	{
		Name:        "k8s",
		Description: "Manifiestos Kubernetes",
		Langs:       []string{"yaml"},
		DefaultLang: "yaml",
		TemplateDir: "k8s",
	},
	{
		Name:        "helm",
		Description: "Helm Charts",
		Langs:       []string{"yaml"},
		DefaultLang: "yaml",
		TemplateDir: "helm",
	},
	{
		Name:        "docker",
		Description: "Dockerfiles e imagenes",
		Langs:       []string{"dockerfile"},
		DefaultLang: "dockerfile",
		TemplateDir: "docker",
	},
	{
		Name:        "monitoring",
		Description: "Observabilidad",
		Langs:       []string{"prometheus"},
		DefaultLang: "prometheus",
		TemplateDir: "monitoring",
	},
}

// All returns all registered prefixes.
func All() []Prefix {
	return registry
}

// Find returns a prefix by name, or nil if not found.
func Find(name string) *Prefix {
	for i := range registry {
		if registry[i].Name == name {
			return &registry[i]
		}
	}
	return nil
}
