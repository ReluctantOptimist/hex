# Hex 2.0 decisions (branch `custom`)

- Built as Hex's "test build" (HEX_TEST_BUILD=1) renamed Hex 2.0: gives its own bundle ID, data folder, permissions and no updater for free, with a tiny diff. Trade-off: dev tools show in Settings.
- `custom` follows upstream's newest stable tag (vX.Y.Z), not `dev`, so weekly updates are releases, not daily churn.
- Signed with the local "Z Mouse Map Signing" certificate (scripts/build-app.sh, test builds only), so the designated requirement is pinned to that cert and macOS keeps Microphone/Accessibility across rebuilds. Falls back to ad-hoc if the cert is missing.
- Vocabulary (39 terms) and Labs › Vocabulary biasing live in Hex 2.0's settings (~/Library/Application Support/hex-test), not in the repo.
- crates/capture/build.rs: helper placed via the `build/` folder's parent so it works with Cargo 1.100's new build-dir layout. Drop when upstream fixes it.
- Evergreen glow pill: drawn in rune_hud.wgsl as plate mode 2 (field.y); the dot field code stays untouched below it.
