package template

import (
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"
	"text/template"

	"github.com/jcvalenzo/cli-devforge/templates"
)

// Data holds the variables available to templates.
type Data struct {
	FullName    string
	Prefix      string
	ProjectName string
	Lang        string
	Author      string
	ModulePath  string
	License     string
	Date        string
	Description string
}

// Render reads a template file and renders it with the given data.
func Render(templatePath string, data Data) (string, error) {
	content, err := templates.FS.ReadFile(filepath.Join(templatePath))
	if err != nil {
		return "", fmt.Errorf("reading template %s: %w", templatePath, err)
	}

	tmpl, err := template.New(templatePath).Parse(string(content))
	if err != nil {
		return "", fmt.Errorf("parsing template %s: %w", templatePath, err)
	}

	var buf strings.Builder
	if err := tmpl.Execute(&buf, data); err != nil {
		return "", fmt.Errorf("executing template %s: %w", templatePath, err)
	}

	return buf.String(), nil
}

// TemplateDir returns the embedded template directory tree for a prefix/lang combo.
func TemplateDir(prefix, lang string) ([]string, error) {
	dirPath := filepath.Join(prefix, lang)

	var files []string
	err := fs.WalkDir(templates.FS, dirPath, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() {
			files = append(files, path)
		}
		return nil
	})
	if err != nil {
		return nil, fmt.Errorf("walking template dir %s/%s: %w", prefix, lang, err)
	}
	return files, nil
}

// WriteFile writes content to a file, creating directories as needed.
func WriteFile(path, content string) error {
	dir := filepath.Dir(path)
	if err := os.MkdirAll(dir, 0755); err != nil {
		return fmt.Errorf("creating directory %s: %w", dir, err)
	}

	if err := os.WriteFile(path, []byte(content), 0644); err != nil {
		return fmt.Errorf("writing file %s: %w", path, err)
	}
	return nil
}
