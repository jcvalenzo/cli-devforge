package templates

import "embed"

//go:embed all:app all:api all:ms all:web all:worker all:scheduler all:lib all:cli all:infra all:k8s all:helm all:docker all:monitoring all:producer all:consumer all:stream all:connector all:bridge all:adapter all:lab
var FS embed.FS
