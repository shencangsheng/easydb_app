# Official homebrew-cask pull request

Use this file when opening the first PR to `Homebrew/homebrew-cask`.

## Steps

1. Generate or verify the cask:

```bash
./scripts/generate-homebrew-cask.sh 2.9.0 \
  /path/to/EasyDB_2.9.0_aarch64.dmg \
  /path/to/EasyDB_2.9.0_x64.dmg
```

2. Run local checks:

```bash
brew style --fix homebrew/Casks/easydb.rb

# Create a temporary tap for install testing
brew tap-new /tmp/easydb-local-tap
mkdir -p "$(brew --repository)/Library/Taps/tmp/homebrew-easydb-local-tap/Casks"
cp homebrew/Casks/easydb.rb "$(brew --repository)/Library/Taps/tmp/homebrew-easydb-local-tap/Casks/easydb.rb"

HOMEBREW_NO_AUTO_UPDATE=1 HOMEBREW_NO_INSTALL_FROM_API=1 brew install --cask tmp/easydb-local-tap/easydb
brew uninstall --cask easydb

brew audit --cask --new easydb
brew audit --cask --online easydb
```

3. Open the PR:

```bash
./scripts/submit-homebrew-cask-pr.sh 2.9.0
```

Or fork manually and copy `homebrew/Casks/easydb.rb` to `Casks/e/easydb.rb`.

## PR title

`Add easydb 2.9.0`

## PR body

**Important**: This is a self-submission by the EasyDB author.

EasyDB is a lightweight desktop data query tool built with Rust and Tauri. It uses SQL to query local files directly with a built-in Apache DataFusion engine.

- **Homepage**: https://github.com/shencangsheng/easydb_app
- **Latest release**: https://github.com/shencangsheng/easydb_app/releases/tag/v2.9.0
- **License**: MIT
- **macOS support**: Apple Silicon and Intel DMG artifacts
- **Notability**: 618+ GitHub stars, active releases, public downloads via GitHub Releases

### Code signing

The current macOS release is **not code-signed or notarized**. This is documented in the project README. Users may see a Gatekeeper warning on first launch and can remove the quarantine attribute if needed:

```bash
xattr -r -d com.apple.quarantine /Applications/EasyDB.app
```

Local `brew audit --cask` may report `Signature verification failed` for this reason. Apple Developer ID signing and notarization are planned for a future release.

Local checks run before opening this PR:

- [x] `brew style --fix` (no offenses)
- [ ] `brew audit --cask --new easydb` (may fail on signature verification until signing is added)
- [ ] `brew audit --cask --online easydb`
- [ ] `HOMEBREW_NO_INSTALL_FROM_API=1 brew install --cask easydb`
- [ ] `brew uninstall --cask easydb`

After this cask is merged, future releases are bumped automatically from the EasyDB release workflow via `brew bump-cask-pr` when `HOMEBREW_GITHUB_API_TOKEN` is configured.
