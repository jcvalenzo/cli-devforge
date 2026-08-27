package template

import (
	"testing"
)

func TestRender(t *testing.T) {
	data := Data{
		FullName:    "api-catalog",
		Prefix:      "api",
		ProjectName: "catalog",
		Lang:        "go",
		Author:      "test",
		ModulePath:  "github.com/test/catalog",
		License:     "MIT",
		Date:        "2026-01-01",
		Description: "Test API",
	}

	rendered, err := Render("api/go/go.mod.tmpl", data)
	if err != nil {
		t.Fatalf("Render() error = %v", err)
	}

	if rendered == "" {
		t.Error("Render() should not return empty string")
	}
	if !contains(rendered, "github.com/test/catalog") {
		t.Errorf("Render() should contain module path, got: %s", rendered)
	}
}

func TestRenderNotFound(t *testing.T) {
	data := Data{}
	_, err := Render("nonexistent/template.tmpl", data)
	if err == nil {
		t.Error("expected error for nonexistent template")
	}
}

func TestTemplateDir(t *testing.T) {
	files, err := TemplateDir("api", "go")
	if err != nil {
		t.Fatalf("TemplateDir() error = %v", err)
	}

	if len(files) == 0 {
		t.Error("TemplateDir() should return at least one file")
	}
}

func contains(s, substr string) bool {
	return len(s) >= len(substr) && (s == substr || len(s) > 0 && containsSubstring(s, substr))
}

func containsSubstring(s, substr string) bool {
	for i := 0; i <= len(s)-len(substr); i++ {
		if s[i:i+len(substr)] == substr {
			return true
		}
	}
	return false
}
