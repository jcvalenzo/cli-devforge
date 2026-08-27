package scaffold

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/jcvalenzo/cli-devforge/internal/prefix"
)

func TestGenerate(t *testing.T) {
	tmpDir := t.TempDir()

	p := prefix.Find("api")
	if p == nil {
		t.Fatal("prefix 'api' not found")
	}

	opts := Options{
		FullName:    "api-test",
		Prefix:      "api",
		ProjectName: "test",
		Lang:        "go",
		Author:      "test",
		ModulePath:  "github.com/test/test",
		License:     "MIT",
		OutputDir:   tmpDir,
		Description: "Test API",
	}

	if err := Generate(p, opts); err != nil {
		t.Fatalf("Generate() error = %v", err)
	}

	// Verify files were created
	expectedFiles := []string{
		"api-test/go.mod",
		"api-test/cmd/server/main.go",
		"api-test/internal/handler/handler.go",
		"api-test/Makefile",
		"api-test/Dockerfile",
		"api-test/README.md",
		"api-test/.gitignore",
	}

	for _, f := range expectedFiles {
		path := filepath.Join(tmpDir, f)
		if _, err := os.Stat(path); os.IsNotExist(err) {
			t.Errorf("expected file %s to exist", f)
		}
	}

	// Verify content was rendered
	content, err := os.ReadFile(filepath.Join(tmpDir, "api-test/go.mod"))
	if err != nil {
		t.Fatalf("reading go.mod: %v", err)
	}
	if string(content) == "" {
		t.Error("go.mod should not be empty")
	}
}

func TestGenerateUnsupportedLang(t *testing.T) {
	tmpDir := t.TempDir()

	p := prefix.Find("api")
	if p == nil {
		t.Fatal("prefix 'api' not found")
	}

	opts := Options{
		FullName:    "api-test",
		Prefix:      "api",
		ProjectName: "test",
		Lang:        "python",
		Author:      "test",
		OutputDir:   tmpDir,
	}

	err := Generate(p, opts)
	if err == nil {
		t.Error("expected error for unsupported language")
	}
}
