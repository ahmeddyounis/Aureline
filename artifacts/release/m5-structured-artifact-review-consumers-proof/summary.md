# Shared Artifact-Review Component Consumers: Mode, Risk, and Provenance Parity

- Packet: `artifact-review-component-consumer:stable:0001`
- Surface: `Shared artifact-review-component consumers`
- Consumer bindings: 18 (12 narrowed)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-08T00:00:00Z)

## Consumer bindings

- **config/app.toml** [`bind:ib-1:workspace`]: component `artifact_identity_bar` on `review_workspace`, mode `full_parity`
- **config/app.toml** [`bind:ib-1:diff`]: component `artifact_identity_bar` on `diff_toolbar`, mode `full_parity`
- **notebook.ipynb** [`bind:dm-2:diff`]: component `diff_mode_switcher` on `diff_toolbar`, mode `structured_fidelity_narrowed`
- **notebook.ipynb** [`bind:dm-2:export`]: component `diff_mode_switcher` on `exported_view`, mode `structured_fidelity_narrowed`
- **vendor.bin** [`bind:sr-3:diff`]: component `structure_row` on `diff_toolbar`, mode `raw_fallback_disclosed`
- **vendor.bin** [`bind:sr-3:workspace`]: component `structure_row` on `review_workspace`, mode `raw_fallback_disclosed`
- **manifest.json** [`bind:md-4:merge`]: component `merge_decision_row` on `merge_sheet`, mode `full_parity`
- **manifest.json** [`bind:md-4:workspace`]: component `merge_decision_row` on `review_workspace`, mode `full_parity`
- **Cargo.lock** [`bind:ga-5:merge`]: component `generated_artifact_notice` on `merge_sheet`, mode `structured_fidelity_narrowed`
- **Cargo.lock** [`bind:ga-5:support`]: component `generated_artifact_notice` on `support_packet`, mode `structured_fidelity_narrowed`
- **design-snapshot.png** [`bind:rc-6:diff`]: component `rendered_compare_viewer` on `diff_toolbar`, mode `structured_fidelity_narrowed`
- **design-snapshot.png** [`bind:rc-6:help`]: component `rendered_compare_viewer` on `help_surface`, mode `structured_fidelity_narrowed`
- **capture.webp** [`bind:mr-7:diff`]: component `media_metadata_rail` on `diff_toolbar`, mode `raw_fallback_disclosed`
- **capture.webp** [`bind:mr-7:export`]: component `media_metadata_rail` on `exported_view`, mode `raw_fallback_disclosed`
- **crash-adjunct.json** [`bind:rb-8:support`]: component `redaction_or_trust_badge_set` on `support_packet`, mode `redaction_narrowed`
- **crash-adjunct.json** [`bind:rb-8:export`]: component `redaction_or_trust_badge_set` on `exported_view`, mode `redaction_narrowed`
- **sbom.spdx.json** [`bind:cs-9:workspace`]: component `compare_summary_card` on `review_workspace`, mode `full_parity`
- **sbom.spdx.json** [`bind:cs-9:support`]: component `compare_summary_card` on `support_packet`, mode `full_parity`
