# Homebrew distribution

EasyDB distributes macOS builds through the personal tap [shencangsheng/homebrew-easydb](https://github.com/shencangsheng/homebrew-easydb). Each release tag automatically updates the cask in that repository.

Official [homebrew-cask](https://github.com/Homebrew/homebrew-cask) requires Developer ID signing and notarization; EasyDB is not signed yet, so distribution stays on the personal tap only.

## Install

```bash
brew install --cask shencangsheng/easydb/easydb
```

### Unsigned app (Gatekeeper)

macOS may block the app on first launch. See the main README FAQ:

```bash
xattr -r -d com.apple.quarantine /Applications/EasyDB.app
```

## Release automation

The [`update-homebrew`](../.github/workflows/release.yml) job runs after each tag release and:

1. Downloads the arm64 and Intel DMG artifacts
2. Generates `homebrew/Casks/easydb.rb` from the template
3. Publishes checksums in the GitHub Actions job summary
4. Pushes the cask to `shencangsheng/homebrew-easydb` when `HOMEBREW_TAP_TOKEN` is configured

## One-time setup

### 1. Tap repository

Public repository: `shencangsheng/homebrew-easydb`

Layout:

```text
homebrew-easydb/
  Casks/
    easydb.rb
```

Seed from a local dry run or copy [`tap-seed/Casks/easydb.rb`](tap-seed/Casks/easydb.rb), then update checksums with:

```bash
./scripts/generate-homebrew-cask.sh <version> \
  path/to/EasyDB_<version>_aarch64.dmg \
  path/to/EasyDB_<version>_x64.dmg \
  /tmp/easydb.rb
```

### 2. Repository secret

In `easydb_app` GitHub repository settings:

| Secret | Purpose |
|--------|---------|
| `HOMEBREW_TAP_TOKEN` | Fine-grained PAT with write access to `shencangsheng/homebrew-easydb` |

## Regenerate cask locally

```bash
./scripts/generate-homebrew-cask.sh <version> <arm64.dmg> <x64.dmg>
```

The output defaults to `homebrew/Casks/easydb.rb` (generated artifact; not committed to the main repo).
