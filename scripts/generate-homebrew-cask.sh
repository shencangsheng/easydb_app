#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Generate homebrew/Casks/easydb.rb from the template.

Usage:
  generate-homebrew-cask.sh <version> <arm64_dmg> <x64_dmg> [output_file]

Example:
  generate-homebrew-cask.sh 2.9.0 \
    EasyDB_2.9.0_aarch64.dmg \
    EasyDB_2.9.0_x64.dmg \
    homebrew/Casks/easydb.rb
EOF
}

if [[ $# -lt 3 ]]; then
  usage
  exit 1
fi

VERSION="$1"
ARM64_DMG="$2"
X64_DMG="$3"
OUTPUT="${4:-homebrew/Casks/easydb.rb}"
TEMPLATE="homebrew/Casks/easydb.rb.template"

if [[ ! -f "$TEMPLATE" ]]; then
  echo "Template not found: $TEMPLATE" >&2
  exit 1
fi

if [[ ! -f "$ARM64_DMG" ]]; then
  echo "ARM64 DMG not found: $ARM64_DMG" >&2
  exit 1
fi

if [[ ! -f "$X64_DMG" ]]; then
  echo "Intel DMG not found: $X64_DMG" >&2
  exit 1
fi

SHA256_ARM64="$(shasum -a 256 "$ARM64_DMG" | awk '{print $1}')"
SHA256_X64="$(shasum -a 256 "$X64_DMG" | awk '{print $1}')"

mkdir -p "$(dirname "$OUTPUT")"

export EASYDB_VERSION="$VERSION"
export EASYDB_SHA256_ARM64="$SHA256_ARM64"
export EASYDB_SHA256_X64="$SHA256_X64"

envsubst '${EASYDB_VERSION} ${EASYDB_SHA256_ARM64} ${EASYDB_SHA256_X64}' \
  < "$TEMPLATE" > "$OUTPUT"

echo "Generated $OUTPUT"
echo "  version: $VERSION"
echo "  sha256 arm64: $SHA256_ARM64"
echo "  sha256 x64:   $SHA256_X64"
