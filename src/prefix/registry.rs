#[derive(Debug, Clone, Copy)]
pub struct Prefix {
    pub name: &'static str,
    pub description: &'static str,
    pub langs: &'static [&'static str],
    pub default_lang: &'static str,
    pub template_dir: &'static str,
}

impl Prefix {
    pub fn has_lang(&self, lang: &str) -> bool {
        self.langs.contains(&lang)
    }
}

pub const REGISTRY: &[Prefix] = &[
    Prefix {
        name: "app",
        description: "Aplicacion completa",
        langs: &["go", "java"],
        default_lang: "go",
        template_dir: "app",
    },
    Prefix {
        name: "api",
        description: "API (monolito o API simple)",
        langs: &["go", "java"],
        default_lang: "go",
        template_dir: "api",
    },
    Prefix {
        name: "ms",
        description: "Microservicio",
        langs: &["go", "java"],
        default_lang: "go",
        template_dir: "ms",
    },
    Prefix {
        name: "web",
        description: "Frontend web",
        langs: &["node", "go"],
        default_lang: "node",
        template_dir: "web",
    },
    Prefix {
        name: "worker",
        description: "Proceso asincrono",
        langs: &["go", "python"],
        default_lang: "go",
        template_dir: "worker",
    },
    Prefix {
        name: "scheduler",
        description: "Tareas programadas",
        langs: &["go"],
        default_lang: "go",
        template_dir: "scheduler",
    },
    Prefix {
        name: "lib",
        description: "Libreria compartida",
        langs: &["go", "java"],
        default_lang: "go",
        template_dir: "lib",
    },
    Prefix {
        name: "cli",
        description: "Herramienta CLI",
        langs: &["go", "java"],
        default_lang: "go",
        template_dir: "cli",
    },
    Prefix {
        name: "infra",
        description: "Infraestructura",
        langs: &["terraform"],
        default_lang: "terraform",
        template_dir: "infra",
    },
    Prefix {
        name: "k8s",
        description: "Manifiestos Kubernetes",
        langs: &["yaml"],
        default_lang: "yaml",
        template_dir: "k8s",
    },
    Prefix {
        name: "helm",
        description: "Helm Charts",
        langs: &["yaml"],
        default_lang: "yaml",
        template_dir: "helm",
    },
    Prefix {
        name: "docker",
        description: "Dockerfiles e imagenes",
        langs: &["dockerfile"],
        default_lang: "dockerfile",
        template_dir: "docker",
    },
    Prefix {
        name: "monitoring",
        description: "Observabilidad",
        langs: &["prometheus"],
        default_lang: "prometheus",
        template_dir: "monitoring",
    },
    Prefix {
        name: "producer",
        description: "Publicador de eventos",
        langs: &["go"],
        default_lang: "go",
        template_dir: "producer",
    },
    Prefix {
        name: "consumer",
        description: "Consumidor de eventos",
        langs: &["go"],
        default_lang: "go",
        template_dir: "consumer",
    },
    Prefix {
        name: "stream",
        description: "Procesamiento de streams",
        langs: &["go", "java"],
        default_lang: "go",
        template_dir: "stream",
    },
    Prefix {
        name: "connector",
        description: "Integration with third parties",
        langs: &["go"],
        default_lang: "go",
        template_dir: "connector",
    },
    Prefix {
        name: "bridge",
        description: "Puente entre sistemas/brokers",
        langs: &["go"],
        default_lang: "go",
        template_dir: "bridge",
    },
    Prefix {
        name: "adapter",
        description: "Adaptador a un servicio externo",
        langs: &["go"],
        default_lang: "go",
        template_dir: "adapter",
    },
    Prefix {
        name: "lab",
        description: "Laboratorio / pruebas de cosas",
        langs: &["go", "java", "python", "react"],
        default_lang: "go",
        template_dir: "lab",
    },
];

pub fn all() -> &'static [Prefix] {
    REGISTRY
}

pub fn find(name: &str) -> Option<&'static Prefix> {
    REGISTRY.iter().find(|prefix| prefix.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_known_prefix() {
        let prefix = find("api").expect("api prefix");
        assert_eq!(prefix.name, "api");
        assert_eq!(prefix.default_lang, "go");
        assert!(prefix.has_lang("go"));
        assert!(prefix.has_lang("java"));
        assert!(!prefix.has_lang("python"));
    }

    #[test]
    fn returns_none_for_unknown_prefix() {
        assert!(find("nonexistent").is_none());
    }

    #[test]
    fn cli_supports_go_and_java() {
        let prefix = find("cli").expect("cli prefix");
        assert!(prefix.has_lang("java"));
        assert!(prefix.has_lang("go"));
        assert_eq!(prefix.default_lang, "go");
        assert_eq!(prefix.template_dir, "cli");
    }

    #[test]
    fn registry_has_all_prefixes() {
        assert_eq!(all().len(), 20);
    }
}
