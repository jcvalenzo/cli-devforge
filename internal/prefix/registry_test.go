package prefix

import "testing"

func TestGetCliSupportsJava(t *testing.T) {
	p := Find("cli")
	if p == nil {
		t.Fatal("Find(cli) should return a prefix")
	}
	if !p.HasLang("java") {
		t.Error("cli prefix should support java")
	}
	if !p.HasLang("go") {
		t.Error("cli prefix should support go")
	}
	if p.DefaultLang != "go" {
		t.Errorf("cli default lang should be go, got %s", p.DefaultLang)
	}
	if p.TemplateDir != "cli" {
		t.Errorf("cli TemplateDir should be cli, got %s", p.TemplateDir)
	}
}
