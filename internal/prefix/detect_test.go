package prefix

import (
	"testing"
)

func TestDetect(t *testing.T) {
	tests := []struct {
		name        string
		input       string
		wantPrefix  string
		wantProject string
		wantErr     bool
	}{
		{"valid api prefix", "api-catalog", "api", "catalog", false},
		{"valid ms prefix", "ms-auth", "ms", "auth", false},
		{"valid app prefix", "app-blog", "app", "blog", false},
		{"multi dash name", "api-user-service", "api", "user-service", false},
		{"no dash", "apicatalog", "", "", true},
		{"unknown prefix", "unknown-name", "", "", true},
		{"empty string", "", "", "", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			p, project, err := Detect(tt.input)
			if (err != nil) != tt.wantErr {
				t.Errorf("Detect(%q) error = %v, wantErr %v", tt.input, err, tt.wantErr)
				return
			}
			if err != nil {
				return
			}
			if p.Name != tt.wantPrefix {
				t.Errorf("Detect(%q) prefix = %q, want %q", tt.input, p.Name, tt.wantPrefix)
			}
			if project != tt.wantProject {
				t.Errorf("Detect(%q) project = %q, want %q", tt.input, project, tt.wantProject)
			}
		})
	}
}

func TestFind(t *testing.T) {
	p := Find("api")
	if p == nil {
		t.Fatal("Find(\"api\") returned nil")
	}
	if p.Name != "api" {
		t.Errorf("Find(\"api\") name = %q, want \"api\"", p.Name)
	}

	p = Find("nonexistent")
	if p != nil {
		t.Error("Find(\"nonexistent\") should return nil")
	}
}

func TestHasLang(t *testing.T) {
	p := Find("api")
	if p == nil {
		t.Fatal("Find(\"api\") returned nil")
	}
	if !p.HasLang("go") {
		t.Error("api should support go")
	}
	if !p.HasLang("java") {
		t.Error("api should support java")
	}
	if p.HasLang("python") {
		t.Error("api should not support python")
	}
}
