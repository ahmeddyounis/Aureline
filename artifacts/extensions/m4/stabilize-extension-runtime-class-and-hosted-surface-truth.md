# Stable Extension Runtime-Class And Hosted-Surface Truth Artifact

This artifact records the stable contract for runtime-class disclosure, active contribution inspectors, downgraded-host banners, hosted-surface boundary chrome, authoring-flow vocabulary parity, diagnostics, and support export.

## Canonical Packet

- Schema: `schemas/extensions/runtime-class.schema.json`
- Rust module: `crates/aureline-extensions/src/stabilize_extension_runtime_class_and_hosted_surface_truth/`
- Documentation: `docs/extensions/m4/stabilize-extension-runtime-class-and-hosted-surface-truth.md`
- Fixtures: `fixtures/extensions/m4/stabilize-extension-runtime-class-and-hosted-surface-truth/`

## Fixture Register

| Fixture | Expected tier | Purpose |
|---|---|---|
| `stable_full_truth_current.json` | `stable` | Runtime class, active inspector, hosted boundary, authoring flows, diagnostics, and support export all align. |
| `downgraded_bridge_with_banner_narrows_to_beta.json` | `beta` | A native or host-rendered contribution falls back to a bridge with explicit old/new runtime class, reason, feature loss, and recovery choices. |
| `hosted_surface_missing_chrome_withdrawn.json` | `withdrawn` | A hosted dashboard omits owner/origin and boundary-egress truth. |
| `inspector_missing_actions_narrows_to_preview.json` | `preview` | An active contribution inspector omits required restart and quarantine controls. |
| `local_dev_vocabulary_drift_narrows_to_preview.json` | `preview` | Local development uses a different trust vocabulary from public package review. |

## Stable Claim Guardrails

A row cannot keep a Stable runtime-class truth claim when any of these are true:

- runtime class is missing, unverified, or hidden behind generic extension wording;
- required consumers do not include marketplace result row, install review, active inspector, diagnostics, and support export;
- active inspectors omit package/version/signature/runtime/locus/trust/permission/host/event attribution or pause/restart/quarantine actions;
- a downgraded host is active without an explicit banner naming old and new runtime class, reason, feature loss, and migration or revert choices;
- a hosted surface lacks owner/origin chrome, boundary/egress summary, storage/cookie posture, accessibility/theming notes, or safe external handoff where safer;
- local development, sideload, or publish preview uses a different runtime, permission, rollback, or registry-binding vocabulary from public package flows;
- support export is missing.

The Rust packet derives `effective_tier` and `downgrade_reasons` from those facts, and the support export carries the same derived result.
