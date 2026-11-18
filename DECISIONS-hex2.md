# Hex 2.0 decisions (branch `custom`)

- Built as Hex's "test build" (HEX_TEST_BUILD=1) renamed Hex 2.0: gives its own bundle ID, data folder, permissions and no updater for free, with a tiny diff. Trade-off: dev tools show in Settings.
- `custom` follows upstream's newest stable tag (vX.Y.Z), not `dev`, so weekly updates are releases, not daily churn.
- Ad-hoc signed (no Apple certificate yet): macOS asks for Microphone/Accessibility again after every rebuild. Fix: sign in to Xcode with an Apple ID and sign with "Apple Development".
- crates/capture/build.rs: helper placed via the `build/` folder's parent so it works with Cargo 1.100's new build-dir layout. Drop when upstream fixes it.
- Evergreen glow pill: drawn in rune_hud.wgsl as plate mode 2 (field.y); the dot field code stays untouched below it.
