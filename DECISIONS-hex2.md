# Hex 2.0 decisions (branch `custom`)

- Built as Hex's "test build" (HEX_TEST_BUILD=1) renamed Hex 2.0: gives its own bundle ID, data folder, permissions and no updater for free, with a tiny diff. Trade-off: dev tools show in Settings.
- `custom` follows upstream's newest stable tag (vX.Y.Z), not `dev`, so weekly updates are releases, not daily churn.
- Signed with the local "Z Mouse Map Signing" certificate (scripts/build-app.sh, test builds only), so the designated requirement is pinned to that cert and macOS keeps Microphone/Accessibility across rebuilds. Falls back to ad-hoc if the cert is missing.
- Vocabulary (39 terms) and Labs › Vocabulary biasing live in Hex 2.0's settings (~/Library/Application Support/hex-test), not in the repo.
- crates/capture/build.rs: helper placed via the `build/` folder's parent so it works with Cargo 1.100's new build-dir layout. Drop when upstream fixes it.
- Evergreen glow pill: drawn in rune_hud.wgsl as plate mode 2 (field.y); the dot field code stays untouched below it.
- Settings › General: Pill style (Hex dots + 10 catalog styles: 1, 3, 19, 22, 23, 24, 27, 28, 33, 35), Pill color (41: catalog 1–40 + Arcade), App theme (Hex standard / Arcade). Pill drawn by `pill_look` in rune_hud.wgsl (style = field.y − 1); colours ride in uniforms the pill leaves unused. Look catalog: https://claude.ai/artifact/43bSexayDGWBmz3R4e2tyN
- Defaults follow the Hex 2.0 arcade logo: Classic bars in Arcade colours, Arcade window theme (an override of the dark kit's palette in rune_ui/controls.rs).
