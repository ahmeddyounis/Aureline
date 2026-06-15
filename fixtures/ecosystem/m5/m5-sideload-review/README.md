# Fixtures: M5 sideload review sheets

This directory contains fixture metadata for the `m5_sideload_review` packet.

The canonical full corpus is checked in at:

`artifacts/ecosystem/m5/m5-sideload-review.json`

## Coverage

- Eight review sheets cover the framework-pack, docs-pack, local-model-pack,
  recipe-pack, template, bridge-backed package, side-loaded package, and
  mirrored-registry families, so one reviewed-install model is proven across the
  marketed M5 artifact families that can be side-loaded as an unpacked directory or a
  content-addressed archive bundle.
- Each sheet carries a `governance_family_ref` that resolves to its row in
  `artifacts/ecosystem/m5/m5-ecosystem-install-governance-matrix.json`.
- Source identity covers both kinds (`unpacked_directory` and `archive_bundle`) and a
  spread of redacted location classes (`workspace_relative_path`,
  `user_home_relative_path`, `removable_media`, `network_mount`, `process_stream`).
- Install kind covers `first_sideload` and `reload_or_update`; the registry-binding
  decision covers `stay_local`, `bind_to_registry_later`, and
  `bound_to_registry_identity`; disposition covers `reviewed_install_ready`,
  `fresh_review_required`, and `blocked`.
- Every review trigger is exercised by at least one reload sheet:
  `permission_widening`, `runtime_class_changed`, `host_or_abi_rebound`,
  `external_executable_introduced`, `update_binding_changed`, and
  `release_channel_changed`.

## Guardrails proven by the corpus

- **No side-load inherits a trusted badge.** A `signed_verified` framework pack built
  locally and a `signed_verified` recipe pack both render `unsigned_local_only` because
  they stay local; only the `bound_to_registry_identity` template lifts the cap, and
  only as far as `registry_bound` — never `verified_publisher` or `enterprise_approved`.
- **Widening cannot apply through a silent hot reload.** The model-pack reload
  introduces an external executable, and the recipe-pack reload widens a permission and
  rebinds the runtime class and host; both recompute to `fresh_review_required` rather
  than installing silently.
- **Rebinding triggers a fresh review.** The bridge-pack reload changes its registry
  binding and release channel and recomputes to `fresh_review_required`.
- **Revoked signatures and anti-abuse quarantines block the install.** The side-loaded
  revoked package and the quarantined mirrored variant both recompute to `blocked` with
  the accept action disabled.
- **Limited-trust continuity is preserved on installed rows.** A reload that does not
  rebind to the registry never raises the installed row's rendered badge.

## Validation

`M5SideloadReview::validate()` is the CI-facing gate. It checks the closed
vocabularies, source-identity and signature consistency, the install-kind/baseline
contract, export-safety (no absolute paths), and — crucially — recomputes the rendered
trust tier, the review-trigger set, and the disposition from each sheet's facts and
flags any drift. The executable proof lives in
`crates/aureline-ecosystem/src/m5_sideload_review/tests.rs`.
