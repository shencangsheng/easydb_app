#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
CASK_SOURCE="${ROOT_DIR}/homebrew/Casks/easydb.rb"
VERSION="${1:-}"

usage() {
  cat <<'EOF'
Prepare an official homebrew-cask pull request for EasyDB.

Usage:
  submit-homebrew-cask-pr.sh <version>

Example:
  submit-homebrew-cask-pr.sh 2.9.0

Requirements:
  - gh CLI authenticated with fork access to Homebrew/homebrew-cask
  - Generated cask at homebrew/Casks/easydb.rb
  - Local brew audit/install checks already passed
EOF
}

if [[ -z "$VERSION" ]]; then
  usage
  exit 1
fi

if [[ ! -f "$CASK_SOURCE" ]]; then
  echo "Missing cask file: $CASK_SOURCE" >&2
  echo "Generate it first with scripts/generate-homebrew-cask.sh" >&2
  exit 1
fi

if ! command -v gh >/dev/null 2>&1; then
  echo "gh CLI is required to open the pull request." >&2
  exit 1
fi

WORKDIR="$(mktemp -d)"
trap 'rm -rf "$WORKDIR"' EXIT
cd "$WORKDIR"

gh repo fork Homebrew/homebrew-cask --clone --default-branch-only
cd homebrew-cask

git checkout -b "add-easydb-${VERSION}"
mkdir -p Casks/e
cp "$CASK_SOURCE" Casks/e/easydb.rb

git add Casks/e/easydb.rb
git commit -m "Add easydb ${VERSION}"

gh pr create \
  --repo Homebrew/homebrew-cask \
  --title "Add easydb ${VERSION}" \
  --body "$(cat <<EOF
**Important**: This is a self-submission by the EasyDB author.

EasyDB is a lightweight desktop data query tool built with Rust and Tauri. It uses SQL to query local files directly with a built-in Apache DataFusion engine.

- **Homepage**: https://github.com/shencangsheng/easydb_app
- **Latest release**: https://github.com/shencangsheng/easydb_app/releases/tag/v${VERSION}
- **License**: MIT
- **macOS support**: Apple Silicon and Intel DMG artifacts
- **Notability**: 618+ GitHub stars, active releases, public downloads via GitHub Releases

### Code signing

The current macOS release is **not code-signed or notarized**. This is documented in the project README. Users may see a Gatekeeper warning on first launch and can remove the quarantine attribute if needed:

\`\`\`bash
xattr -r -d com.apple.quarantine /Applications/EasyDB.app
\`\`\`

Local \`brew audit --cask\` may report \`Signature verification failed\` for this reason. Apple Developer ID signing and notarization are planned for a future release.

Local checks run before opening this PR:

- [ ] \`brew audit --cask --new easydb\` (may fail on signature verification until signing is added)
- [ ] \`brew audit --cask --online easydb\`
- [ ] \`brew style --fix Casks/e/easydb.rb\`
- [ ] \`HOMEBREW_NO_INSTALL_FROM_API=1 brew install --cask easydb\`
- [ ] \`brew uninstall --cask easydb\`

After this cask is merged, future releases are bumped automatically from the EasyDB release workflow via \`brew bump-cask-pr\`.
EOF
)"

echo "Opened homebrew-cask pull request for easydb ${VERSION}"
