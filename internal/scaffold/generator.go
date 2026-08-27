package scaffold

import (
	"fmt"
	"path/filepath"
	"strings"
	"time"

	"github.com/jcvalenzo/cli-devforge/internal/prefix"
	"github.com/jcvalenzo/cli-devforge/internal/template"
)

// Options holds the parameters for project generation.
type Options struct {
	FullName    string
	Prefix      string
	ProjectName string
	Lang        string
	Author      string
	ModulePath  string
	License     string
	OutputDir   string
	Description string
}

// Generate creates a project scaffold based on the prefix and options.
func Generate(p *prefix.Prefix, opts Options) error {
	data := template.Data{
		FullName:    opts.FullName,
		Prefix:      opts.Prefix,
		ProjectName: opts.ProjectName,
		Lang:        opts.Lang,
		Author:      opts.Author,
		ModulePath:  opts.ModulePath,
		License:     opts.License,
		Date:        time.Now().Format("2006-01-02"),
		Description: opts.Description,
	}

	if opts.ModulePath == "" && opts.Lang == "go" {
		data.ModulePath = fmt.Sprintf("github.com/example/%s", opts.ProjectName)
	}

	files, err := template.TemplateDir(opts.Prefix, opts.Lang)
	if err != nil {
		return fmt.Errorf("listing templates: %w", err)
	}

	if len(files) == 0 {
		return fmt.Errorf("no templates found for %s/%s", opts.Prefix, opts.Lang)
	}

	projectDir := filepath.Join(opts.OutputDir, opts.FullName)
	prefixLangPrefix := filepath.Join(opts.Prefix, opts.Lang) + string(filepath.Separator)

	for _, f := range files {
		rendered, err := template.Render(f, data)
		if err != nil {
			return fmt.Errorf("rendering %s: %w", f, err)
		}

		// Convert template path to output path:
		// Remove prefix/lang directory prefix and .tmpl suffix
		outPath := strings.TrimPrefix(f, prefixLangPrefix)
		outPath = strings.TrimSuffix(outPath, ".tmpl")

		fullPath := filepath.Join(projectDir, outPath)

		if err := template.WriteFile(fullPath, rendered); err != nil {
			return fmt.Errorf("writing %s: %w", fullPath, err)
		}

		fmt.Printf("  created  %s\n", outPath)
	}

	return nil
}
