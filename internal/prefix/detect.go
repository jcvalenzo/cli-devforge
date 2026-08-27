package prefix

import (
	"fmt"
	"strings"
)

// Detect extracts the prefix and project name from a full project name.
// Example: "api-catalog" -> prefix "api", project "catalog"
func Detect(fullName string) (*Prefix, string, error) {
	parts := strings.SplitN(fullName, "-", 2)
	if len(parts) < 2 {
		return nil, "", fmt.Errorf("invalid project name %q: must follow <prefix>-<name> convention", fullName)
	}

	prefixName := parts[0]
	projectName := parts[1]

	p := Find(prefixName)
	if p == nil {
		return nil, "", fmt.Errorf("unknown prefix %q in %q", prefixName, fullName)
	}

	return p, projectName, nil
}
