package cmd

import (
	"fmt"

	"github.com/jcvalenzo/cli-devforge/internal/prefix"
	"github.com/jcvalenzo/cli-devforge/internal/scaffold"
	"github.com/spf13/cobra"
)

var (
	lang       string
	author     string
	modulePath string
	license    string
	outputDir  string
)

var createCmd = &cobra.Command{
	Use:   "create <prefix-name>",
	Short: "Create a new project scaffold",
	Long: `Create a new project scaffold based on prefix naming convention.

Examples:
  devforge create api-catalog --lang go
  devforge create ms-auth --lang java
  devforge create web-admin --lang node
  devforge create worker-email --lang python`,
	Args: cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		name := args[0]

		p, projectName, err := prefix.Detect(name)
		if err != nil {
			return err
		}

		if lang == "" {
			lang = p.DefaultLang
		}

		if !p.HasLang(lang) {
			return fmt.Errorf("language %q not supported for prefix %q (available: %v)", lang, p.Name, p.Langs)
		}

		opts := scaffold.Options{
			FullName:    name,
			Prefix:      p.Name,
			ProjectName: projectName,
			Lang:        lang,
			Author:      author,
			ModulePath:  modulePath,
			License:     license,
			OutputDir:   outputDir,
			Description: p.Description,
		}

		if err := scaffold.Generate(p, opts); err != nil {
			return err
		}

		fmt.Printf("\nProject %q created successfully in ./%s/%s\n\n", name, outputDir, name)
		fmt.Println("Next steps:")
		fmt.Printf("  cd %s\n", name)
		fmt.Println("  git init")
		fmt.Println("  git add .")
		fmt.Println("  git commit -m \"Initial commit\"")
		fmt.Println()
		fmt.Println("Push to GitHub:")
		fmt.Printf("  gh repo create %s --public --source=. --remote=origin --push\n", name)
		fmt.Println()
		fmt.Println("Or configure remote manually:")
		fmt.Printf("  git remote add origin git@github.com:YOUR_GITHUB_USERNAME/%s.git\n", name)
		fmt.Println()
		return nil
	},
}

func init() {
	createCmd.Flags().StringVarP(&lang, "lang", "l", "", "language for the project (default: prefix default)")
	createCmd.Flags().StringVarP(&author, "author", "a", "", "author name")
	createCmd.Flags().StringVar(&modulePath, "module", "", "Go module path (for Go projects)")
	createCmd.Flags().StringVar(&license, "license", "MIT", "project license")
	createCmd.Flags().StringVarP(&outputDir, "output", "o", ".", "output directory")
}
