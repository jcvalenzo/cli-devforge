package cmd

import (
	"fmt"
	"os"
	"text/tabwriter"

	"github.com/jcvalenzo/cli-devforge/internal/prefix"
	"github.com/spf13/cobra"
)

var listCmd = &cobra.Command{
	Use:   "list",
	Short: "List all available prefixes",
	RunE: func(cmd *cobra.Command, args []string) error {
		w := tabwriter.NewWriter(os.Stdout, 0, 0, 2, ' ', 0)
		_, _ = fmt.Fprintln(w, "PREFIX\tDESCRIPTION\tLANGUAGES")
		_, _ = fmt.Fprintln(w, "------\t-----------\t---------")
		for _, p := range prefix.All() {
			_, _ = fmt.Fprintf(w, "%s\t%s\t%s\n", p.Name+"-", p.Description, joinLangs(p.Langs))
		}
		return w.Flush()
	},
}

func joinLangs(langs []string) string {
	result := ""
	for i, l := range langs {
		if i > 0 {
			result += ", "
		}
		result += l
	}
	return result
}
