# Beta community handoff

Help / About, the docs browser, migration center, and service-health surfaces
route issue handoff through the same release-truth wiring used by the beta
claim manifest and compatibility report. The handoff keeps the current object
ref and issue context ref attached before the user leaves the product or saves
a local support packet.

## Inputs

- Claim manifest: `artifacts/release/m3/claim_manifest.json`
- Compatibility report: `artifacts/compat/m3/compatibility_report.json`
- Wiring report: `artifacts/docs/m3/truth_wiring_report.md`
- Inspector: `cargo run -q -p aureline-shell --bin aureline_shell_truth_wiring -- markdown`

## Route classes

| Issue class | Route class | Trust class | Context carried |
|---|---|---|---|
| `docs_truth_mismatch` | `public_issue_tracker` | `official_public` | Docs object ref, claim row ref, claim manifest ref, compatibility report ref |
| `migration_compatibility_regression` | `public_issue_tracker` | `official_public` | Migration page/session ref, compatibility row refs, claim manifest ref |
| `design_proposal` | `public_rfc_forum` | `community` | Proposal object ref and manifest context only |
| `security_sensitive` | `private_security_channel` | `official_authenticated` | Security object ref and issue context, with private redaction profile |
| `private_workspace_support` | `private_support_channel` | `official_authenticated` | Redacted support context and compatibility report ref |

The only destination trust classes are `official_public`,
`official_authenticated`, `community`, and `local_only`. Public lanes never
attach raw diagnostics automatically. Private lanes require a local preview or
support export before navigation.

## Context preservation

Every `CommunityHandoffRequest` carries:

- `source_surface`
- `current_object_ref`
- `issue_context_ref`
- `claim_manifest_ref`
- `compatibility_report_ref`

The resolved `CommunityHandoffDecision` is valid only when both
`current_object_ref` and `issue_context_ref` survive routing. If either field is
empty, the truth-wiring report emits a `handoff_context_missing` finding.

## Public and private lanes

Use the public issue tracker for docs-truth mismatches and migration
compatibility regressions when the user can review the metadata-safe object refs
before submission. Use the public RFC forum for design proposals that do not
need local diagnostics.

Use the private security channel for exploit details, credentials, policy
bypass reports, or anything that would create disclosure risk in public. Use the
private support channel for workspace, tenant, account, or live-device context.
Those lanes are authenticated and require a redacted local packet before data
leaves the product boundary.

## Failure visibility

Truth-wiring failures are consolidated in
`artifacts/docs/m3/truth_wiring_report.md`. The report lists:

- surface bindings for docs browser, migration center, Help/About, and service
  health;
- claim row ids and compatibility row refs each surface resolves through;
- missing compatibility row refs, if any;
- handoff route decisions and whether object/context preservation succeeded.

Run `cargo run -q -p aureline-shell --bin aureline_shell_truth_wiring -- validate`
after refreshing the claim manifest or compatibility report.
