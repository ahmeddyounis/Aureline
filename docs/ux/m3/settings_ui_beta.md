# Settings UI Beta

This page describes the beta-grade settings UI projection that lives
in `aureline-settings`. It builds on the schema-backed inspector
records documented in [`docs/settings/inspector_alpha.md`](../../settings/inspector_alpha.md)
so user-facing UI rows, headless CLI inspection, and support exports
all report the same effective-value truth.

## Contract surface

The beta projection ships four record kinds, all under the shared
contract ref `settings:ui_beta:v1`:

- `settings_ui_beta_row` — one user-facing row carrying the value
  preview, source pill, badges (lock, sensitivity, restart, lifecycle,
  redaction), capability availability, help deep link, validation
  status, and a write affordance that names what editing the value
  requires.
- `settings_ui_beta_page` — rows grouped by setting-id prefix
  (Editor, Shell, Appearance, Security and trust, Files and watchers,
  AI, etc.) together with aggregate restart, policy-lock, and
  redaction banners.
- `settings_ui_beta_inspector_pane` — the expanded "open detail"
  view of one row. Renders the definition summary, a labelled
  source-chain table, the lock explanation block, the restart
  explanation, the policy-lock explanation when active, and the
  evidence refs from the canonical definition.
- `settings_ui_beta_write_composer` — scope-explicit write composer
  built by routing through the inspector's `preview_write` flow. The
  composer never mutates the live resolver and always names the
  destination artifact. Denied composers carry a typed
  `denial_explanation` with an owner label, a remediation label, a
  remediation action ref, and the policy source ref when an admin
  policy ceiling is the lock owner. The composer is the failure-drill
  proof surface; a denied write that omits owner or remediation is a
  contract bug.

A support-export wrapper (`settings_ui_beta_support_export`) carries
both the beta page and the canonical inspector records the rows were
built from, so a reviewer can pivot from a row to its inspector
record by the shared `source_record_ref`.

The boundary schema is
[`schemas/settings/effective_value.schema.json`](../../../schemas/settings/effective_value.schema.json).
The canonical effective-setting inspection record consumed by every
beta row is
[`schemas/settings/effective_setting.schema.json`](../../../schemas/settings/effective_setting.schema.json).

## Acceptance posture

The beta UI delivers the M3 settings authority acceptance gates:

- **Inspectable winning truth** — every row exposes the winning value,
  source label, ordered shadow chain, lock state, lock reason,
  restart posture, sensitivity class, redaction posture, and last
  applied revision across user, workspace, profile, and policy
  scopes. The same shape backs the inspector pane and the support
  export wrapper.
- **Scope-explicit writes with typed denials** — the write composer
  inherits the inspector's `preview_write` flow, so the destination
  preview always shows the scope the write targets (no broader-scope
  fan-out) and denials surface the owner and remediation route
  instead of failing silently.
- **UI / CLI / support-export parity** — rows, CLI projections, and
  support exports all carry the same `source_record_ref` per
  setting. The support export embeds the inspector records the page
  rows were derived from, so support reviewers and the user see the
  same effective-value truth.

## Headless consumers

The beta projection is exercised through the same
`aureline_settings_inspect` binary the alpha inspector ships:

```sh
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- ui-beta-page
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- ui-beta-inspector security.ai.egress_policy
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- ui-beta-write-composer
cargo run -q -p aureline-settings --bin aureline_settings_inspect -- ui-beta-support-export
```

The `ui-beta-write-composer` subcommand emits a denied composer for
the seeded `security.ai.egress_policy` policy lock so the
remediation copy is testable from CI without launching the live
shell.

## Fixtures

Protected fixtures live under
[`fixtures/settings/ui_beta/`](../../../fixtures/settings/ui_beta/):

- `page_all_settings.json` — beta page across the seeded catalog,
  with editor / shell / appearance / security / files groups.
- `inspector_pane_security_egress.json` — inspector pane for the
  policy-locked AI egress setting; the source chain shows the
  shadowed default, the capped user value, and the winning admin
  policy row.
- `write_composer_policy_denied.json` — denied write composer for the
  same setting at the user scope; the denial explanation names the
  admin policy bundle and the remediation route.
- `support_export_parity.json` — support export wrapper that quotes
  both the beta UI page and the inspector records the rows were
  built from.

## Verification

```sh
cargo test -p aureline-settings
cargo test -p aureline-settings --test ui_beta_fixtures
```

The integration test
[`crates/aureline-settings/tests/ui_beta_fixtures.rs`](../../../crates/aureline-settings/tests/ui_beta_fixtures.rs)
replays the fixtures through the Rust types and asserts the page
groups, restart and policy banners, source-chain table labels,
denial owner / remediation, and the shared `source_record_ref`
parity between UI rows and inspector records.
