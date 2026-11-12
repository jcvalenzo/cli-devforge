#!/usr/bin/env bash
#
# Captures golden outputs from the legacy Go binary so the Rust migration can
# be validated for behavioural parity.
#
# This script is kept for reference. Run it from a checkout of the legacy Go
# version (for example, tag v0.4.0), not from the Rust branch:
#
#   make build
#   ./scripts/capture-golden.sh ./bin/devforge
#
# The generated fixtures live in tests/fixtures/golden/ and are committed.
# Do not run this against the Rust binary; it would overwrite the fixtures.
set -euo pipefail

BIN="${1:-./bin/devforge}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="$ROOT/tests/fixtures/golden"

if [ ! -x "$BIN" ]; then
  echo "error: binary not found or not executable: $BIN" >&2
  exit 1
fi

BIN="$(cd "$(dirname "$BIN")" && pwd)/$(basename "$BIN")"

rm -rf "$OUT"
mkdir -p "$OUT"

capture() {
  local case_name="$1"
  shift

  local dest="$OUT/$case_name"
  mkdir -p "$dest"

  printf '%s\n' "$@" >"$dest/args.txt"

  local tmp
  tmp="$(mktemp -d)"

  (
    cd "$tmp"
    set +e
    "$BIN" "$@" >"$dest/stdout.txt" 2>"$dest/stderr.txt"
    echo $? >"$dest/exit_code.txt"
    set -e
  )

  mkdir -p "$dest/tree"
  cp -R "$tmp/." "$dest/tree/"
  rm -rf "$tmp"
}

capture "list" list
capture "version" version

combos=(
  "app:go" "app:java"
  "api:go" "api:java"
  "ms:go" "ms:java"
  "web:node" "web:go"
  "worker:go" "worker:python"
  "scheduler:go"
  "lib:go" "lib:java"
  "cli:go" "cli:java"
  "infra:terraform"
  "k8s:yaml"
  "helm:yaml"
  "docker:dockerfile"
  "monitoring:prometheus"
  "producer:go"
  "consumer:go"
  "stream:go" "stream:java"
  "connector:go"
  "bridge:go"
  "adapter:go"
  "lab:go" "lab:java" "lab:python" "lab:react"
)

for combo in "${combos[@]}"; do
  prefix="${combo%%:*}"
  lang="${combo#*:}"
  capture "create-${prefix}-${lang}" create "${prefix}-sample" --lang "$lang" --author "Test Author"
done

capture "create-api-default" create api-sample --author "Test Author"
capture "create-custom-api-go" create api-catalog --lang go --author "Test Author" \
  --module github.com/acme/catalog --license Apache-2.0 -o out

capture "error-invalid-name" create apicatalog
capture "error-unknown-prefix" create unknown-name
capture "error-unsupported-lang" create api-catalog --lang python
capture "error-no-templates" create worker-email --lang python

echo "Golden fixtures written to $OUT"
