# Homebrew distribution

EasyDB publishes macOS builds to Homebrew automatically when a release tag is pushed.

## Install

After the tap is configured:

```bash
brew install --cask shencangsheng/easydb/easydb
```

Once the cask is accepted into official Homebrew:

```bash
brew install --cask easydb
```

## Release automation

The [`update-homebrew`](../.github/workflows/release.yml) job runs after each tag release and:

1. Downloads the arm64 and Intel DMG artifacts
2. Generates `homebrew/Casks/easydb.rb` from the template
3. Publishes checksums in the GitHub Actions job summary
4. Updates the personal tap repository when `HOMEBREW_TAP_TOKEN` is configured
5. Opens an official bump PR when `HOMEBREW_GITHUB_API_TOKEN` is configured and the cask already exists in `homebrew-cask`

## One-time setup

### 1. Create the tap repository

Create an empty public GitHub repository named `homebrew-easydb` under `shencangsheng`.

Initial layout:

```text
homebrew-easydb/
  Casks/
    easydb.rb
```

You can seed it by copying the generated file from a local release dry run:

```bash
./scripts/generate-homebrew-cask.sh 2.9.0 \
  path/to/EasyDB_2.9.0_aarch64.dmg \
  path/to/EasyDB_2.9.0_x64.dmg \
  /tmp/easydb.rb
```

### 2. Configure repository secrets

In `easydb_app` GitHub repository settings, add:

| Secret | Purpose |
|--------|---------|
| `HOMEBREW_TAP_TOKEN` | Fine-grained PAT with write access to `shencangsheng/homebrew-easydb` |
| `HOMEBREW_GITHUB_API_TOKEN` | Optional. PAT for `brew bump-cask-pr` to open official update PRs after the first cask merge |

### 3. Submit the first official cask PR

Official `homebrew-cask` cannot be updated directly by CI until the cask exists. Submit the first PR manually:

```bash
./scripts/generate-homebrew-cask.sh <version> <arm64.dmg> <x64.dmg> /tmp/easydb.rb

brew audit --cask --new easydb
brew audit --cask --online easydb
brew style --fix /tmp/easydb.rb
HOMEBREW_NO_INSTALL_FROM_API=1 brew install --cask easydb
brew uninstall --cask easydb
```

Then fork `Homebrew/homebrew-cask`, add `Casks/e/easydb.rb`, and open a PR titled `Add easydb <version>`.

Disclose self-submission in the PR body and include notability evidence:

- GitHub stars: 618+
- MIT license, active releases, macOS arm64 + Intel DMG support
- Latest release: https://github.com/shencangsheng/easydb_app/releases/latest

After merge, future tag releases can auto-open bump PRs via `HOMEBREW_GITHUB_API_TOKEN`.

## Regenerate cask locally

```bash
./scripts/generate-homebrew-cask.sh <version> <arm64.dmg> <x64.dmg>
```

The output defaults to `homebrew/Casks/easydb.rb`.
