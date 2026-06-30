# M5 Post-Install Notice/Provenance/SBOM Disclosure Panels

This document is the contract for the in-product post-install disclosure panels
the M5 help lane ships for installed and generated artifact families. The panels
let a user inspect how a build or package arrived, whether its signature,
attestation, checksum, and revocation verify, what its notice / license / SBOM
inventory contains, and which provenance or notice data is **missing** — without
returning to the original download page or guessing from a channel color.

The panels project the frozen governance contract; they do not mint a parallel
truth:

- Panel record schema: [`schemas/governance/post_install_disclosure.schema.json`](../../schemas/governance/post_install_disclosure.schema.json)
- Panel-set bundle schema: [`schemas/help/m5-post-install-disclosure.schema.json`](../../schemas/help/m5-post-install-disclosure.schema.json)
- Governance contract: [`docs/governance/post_install_notice_and_provenance_contract.md`](../governance/post_install_notice_and_provenance_contract.md)
- Shared provenance vocabulary: [`docs/governance/provenance_badge_contract.md`](../governance/provenance_badge_contract.md)
- Publication gate: [`schemas/help/m5-public-handoff-matrix.schema.json`](../../schemas/help/m5-public-handoff-matrix.schema.json) — the frozen matrix whose `post_install_notice` / `provenance_disclosure` rows govern whether these panels may publish a claim.
- Producer: `aureline_shell::m5_post_install_disclosure`
- Headless emitter: `aureline_shell_m5_post_install_disclosure`

## Panels

Each [`PostInstallDisclosureRecord`] conforms to the governance schema. The
[`M5PostInstallDisclosurePanelSet`] bundles one record per governed M5 artifact
family:

| Family | Subject kind | Source class | Demonstrates |
| --- | --- | --- | --- |
| `desktop_build_installer` | `product_build` | `official` | Signed, verified build with complete notices and verified SPDX/CycloneDX SBOM. |
| `extension_framework_pack` | `extension_package` | `side_loaded` | Side-loaded pack with attestation, SBOM, license, notice, and revocation rendered as visible missing states; marketplace / package detail marks it side-loaded. |
| `mirrored_offline_artifact` | `mirrored_transport_artifact` | `mirrored` | Mirrored offline bundle whose origin verifies but whose revocation snapshot is stale and visibly refreshable. |
| `generated_export_artifact` | `generated_user_artifact` | `official` | Generated export with lineage refs, partial notices, and a redistribution-review hint. |

## What every panel carries

- **Source class and subject class stay separate.** `Official`, `Mirrored`,
  `Side-loaded`, and `Unknown provenance` describe transport posture; the subject
  (`product_build`, `extension_package`, `mirrored_transport_artifact`,
  `generated_user_artifact`, …) is named independently. The source label is pinned
  to its class — a mismatch fails validation.
- **Layered trust evidence.** Signature, attestation, checksum, revocation state,
  revocation freshness, SBOM state, SBOM formats, license state, and notice
  inventory are separate fields.
- **SBOM format labeling and scope stay explicit.** An attached SBOM must declare
  its formats; a non-attached SBOM must not claim formats it does not carry. The
  artifact subject and scope are always named.
- **Missing data is visible, never omitted.** When signature, attestation, SBOM,
  license, notice inventory, or revocation snapshot is missing, partial, unknown,
  stale, or policy-hidden, a typed `missing_or_partial_data` row names the affected
  `data_class`, a `Not provided` / `Partial` / `Unknown provenance` / `Stale` /
  `Policy hidden` cue, a disclosure sentence, and a resolution action where one
  exists. Silence never reads as "clean".
- **Post-install access survives.** Every record carries access points for
  `about`, `update_center`, `installed_state_inspector`, `diagnostics_export`, and
  `review_sheet`; packs add `marketplace_or_package_detail`. Exports preserve the
  same disclosure id, subject class, source class, stale/missing states, and
  omission reasons.

## Honesty invariants

The `honesty_invariants` block encodes the lane invariants as hard flags — all
must hold for the panel set to validate:

- `subject_kind_explicit`, `missing_data_visible_not_omitted`,
  `source_class_and_subject_separate`, `trust_evidence_layered`,
  `post_install_access_survives`, `sbom_format_and_scope_explicit`,
  `exports_preserve_caveats`,
  `provenance_states_distinguish_official_mirrored_side_loaded_unknown`, and
  `stale_or_revoked_never_reads_as_verified`.

The `consumer_projection` block binds About/help, installed-state inspectors,
diagnostics exports, and marketplace / package detail to the shared panel model,
and records that this lane **exposes** — does not replace — the release
publication artifacts.

## Narrowing without hiding

The fixtures under
[`fixtures/help/post-install-disclosure/`](../../fixtures/help/post-install-disclosure/)
show two narrowings that stay visible:

- `product_build_signature_revoked.json` — the official build whose signing key was
  revoked after install. Signature reads `signature_revoked`, revocation reads
  `revoked_or_yanked` with expired freshness, and a visible revocation row keeps the
  narrowing explicit. The build no longer reads as verified.
- `generated_export_sbom_not_provided.json` — the generated export with no SBOM,
  surfaced as a `Not provided` SBOM row rather than a blank cell.

## Companion artifacts

The seed builders in
`crates/aureline-shell/src/m5_post_install_disclosure/seed.rs` are the single
producer of the checked-in artifacts:

- Panel set: [`artifacts/help/m5-post-install-proof/panel_set.json`](../../artifacts/help/m5-post-install-proof/panel_set.json)
- Per-family panels: `artifacts/help/m5-post-install-proof/panel_*.json`
- Governance summary: [`artifacts/help/m5-post-install-disclosure-governance.md`](../../artifacts/help/m5-post-install-disclosure-governance.md)
- Panel CSV: [`artifacts/help/m5-post-install-disclosure-panels.csv`](../../artifacts/help/m5-post-install-disclosure-panels.csv)
- Narrowed fixtures: [`fixtures/help/post-install-disclosure/`](../../fixtures/help/post-install-disclosure/)

## Regeneration

```sh
BIN="cargo run -q -p aureline-shell --bin aureline_shell_m5_post_install_disclosure --"
$BIN panel-set > artifacts/help/m5-post-install-proof/panel_set.json
$BIN panel desktop_build_installer > artifacts/help/m5-post-install-proof/panel_product_build.json
$BIN panel extension_framework_pack > artifacts/help/m5-post-install-proof/panel_extension_pack.json
$BIN panel mirrored_offline_artifact > artifacts/help/m5-post-install-proof/panel_mirrored_offline.json
$BIN panel generated_export_artifact > artifacts/help/m5-post-install-proof/panel_generated_export.json
$BIN governance > artifacts/help/m5-post-install-disclosure-governance.md
$BIN csv > artifacts/help/m5-post-install-disclosure-panels.csv
$BIN fixture-signature-revoked > fixtures/help/post-install-disclosure/product_build_signature_revoked.json
$BIN fixture-generated-sbom-not-provided > fixtures/help/post-install-disclosure/generated_export_sbom_not_provided.json
```

The inline tests assert the checked-in panel set and fixtures match the seed
builders, so a drift between code and artifacts fails the build.
