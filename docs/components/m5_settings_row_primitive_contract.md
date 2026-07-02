# M5 Settings-Row Primitive Contract — Effective Value, Source Pill, and Lock State

> Task: M05-757 · Batch B88 · Delivery class: high-trust component contract +
> reusable primitive implementation + support/export parity.

This contract implements Aureline's **one reusable settings-row primitive** across
every M5 config-bearing surface — admin/enterprise, workspace trust, AI/model,
network/proxy, execution/runtime, extension, and update/config-channel — so
effective value, configured value, source scope, lock reason, and diff /
open-source-detail behavior stay consistent instead of drifting by screen. It
narrows the settings-row family named by the frozen
[M5 trust-chronology component matrix](m5_trust_chronology_components_contract.md)
(M05-756) into a working primitive with a resolver and a per-surface parity matrix.

- **Boundary schema:** [`schemas/ui/m5-settings-row.schema.json`](../../schemas/ui/m5-settings-row.schema.json)
- **Rust source of truth:** `crates/aureline-shell/src/implement_the_m5_settings_row_effective_value_source_pill_and_lock_state_primitive/`
- **Headless emitter:** `aureline_shell_m5_settings_row_primitive`
- **Checked support export:** [`artifacts/release/m5-settings-row-proof/support_export.json`](../../artifacts/release/m5-settings-row-proof/support_export.json)
- **Matrix CSV:** [`artifacts/release/m5-settings-row-proof/matrix.csv`](../../artifacts/release/m5-settings-row-proof/matrix.csv)
- **Report:** [`artifacts/components/m5-settings-row-primitive.md`](../../artifacts/components/m5-settings-row-primitive.md)
- **Narrowed fixtures:** [`fixtures/ui/m5-settings-row-primitive/`](../../fixtures/ui/m5-settings-row-primitive/)

The settings-row state vocabulary, source pills, non-visual accessibility routes,
qualification classes, and downgrade triggers are reused verbatim from the frozen
[M5 trust-chronology component matrix](../../schemas/ui/m5-trust-chronology-components.schema.json);
the shell topology — zones, responsive classes, window classes, and consumer
surfaces — is reused verbatim from the frozen
[M5 shell-zone matrix](../../schemas/shell/m5-shell-zone.schema.json). This lane
mints new vocabulary only for what those matrices left implicit about the settings
row itself: its **anatomy parts**, its **lock disclosures**, its **focus
behaviors**, and its **export fields**. No M5 surface invents a second settings-row
grammar.

## Track invariant

One settings-row model carries effective-versus-configured truth, source pills,
and lock-state explainability. A user-authored value is never confused with the
effective value; a locked or overridden value never hides what the user set; and
the support/export packet can reconstruct effective-value truth from the same
shared row model on every surface.

## The primitive: two halves

### 1. Resolver — `resolve_settings_row`

Given a setting's per-source contributions (each a `source_pill`, an opaque
`value_repr`, and an `enforces_lock` flag) plus the `pending_reload` and
`invalid_value_held` flags, the resolver produces one `M5ResolvedSettingsRow`
carrying:

- `effective_value_repr` and the `winning_source` that produced it,
- `configured_value_repr` — the `user_configured` value, **retained** even when a
  higher source or policy wins,
- the typed `row_state`,
- `is_locked` and the `lock_source`,
- `differs_from_configured` (drives the view-diff affordance), and
- the `shadow_chain` — every contributing source, highest precedence first.

The precedence ladder (highest wins) is
`policy_managed > environment_override > remote_profile > workspace_configured >
user_configured > default_value`. Only non-user sources may enforce a lock; a
lock claimed by a user or default source is rejected. Value representations
carrying URLs, credentials, or other forbidden material are rejected; managed
values are carried only as `redacted_managed_value`.

State resolution order: `invalid_value_held` → `pending_reload_to_apply` →
`redacted_managed_value` → `locked_by_policy` → `inherited_from_default`
(default wins) → `effective_matches_configured` (user value wins) →
`overridden_by_higher_source`.

### 2. Parity matrix — one row per config-bearing surface

Each of the seven config-bearing surfaces carries the same shared anatomy, the
same states and source pills, the same lock disclosures and focus behaviors, and
the same export fields, plus worked resolution cases proving the resolver on that
surface.

| Surface family | Zone | Worked resolution highlight |
| --- | --- | --- |
| `admin_enterprise` | `main_workspace` | policy-locked value keeps user value visible |
| `workspace_trust` | `main_workspace` | user value wins (`effective_matches_configured`) |
| `ai_model` | `main_workspace` | workspace override + redacted credential-managed value |
| `network_proxy` | `main_workspace` | environment override supersedes user value |
| `execution_runtime` | `main_workspace` | staged change is `pending_reload_to_apply` |
| `extension_settings` | `main_workspace` | unconfigured value is `inherited_from_default` |
| `update_channel` | `main_workspace` | invalid value held + remote-profile value |

## Anatomy (shared row)

`label`, `plain_language_description`, `value_control`, `source_pill`, and
`reset_action` are mandatory on every row. `view_diff_affordance`,
`source_detail_affordance`, and `open_in_json_affordance` are the escalation
affordances: view-diff compares effective versus configured, source-detail
escalates to a side sheet when inline explanation is insufficient, and open-in-JSON
jumps to the authoritative file.

## Lock disclosure

When a value is locked, `enforced_value_shown`, `lock_source_shown`, and
`user_configured_value_retained` are mandatory — the enforced value and the lock
source are shown together, and the user-configured value is never hidden.
`lock_reason_explained`, `override_request_path`, and `no_silent_value_hide` round
out the disclosure. The checked-in admin/enterprise worked case is the canonical
proof: policy enforces `disabled` while the user's `enabled` value stays visible
and the diff is exposed.

## Focus and search

`search_result_focus_landing` and `highlight_on_open` land and highlight the row
from search or a deep link; `source_detail_side_sheet_escalation` and
`inline_explanation_preferred` govern when detail escalates to a side sheet;
`return_focus_on_close` and `deep_link_anchor` keep focus and addressing stable.

## Support / export reconstruction

The export fields `setting_key`, `effective_value_repr`, `winning_source_pill`,
and `row_state` are mandatory; `configured_value_repr`, `lock_source_pill`, and
`shadow_chain` complete the record. Each surface carries its worked resolution
cases in the export, and the validator re-runs the resolver on every stored input
and asserts it equals the stored output — so the support export reconstructs
effective-value truth from the same shared row model, and at least one case must
prove a locked value that retains a differing user-configured value.

## Hard invariants (per surface row, all MUST be false)

- `conflates_effective_and_configured`
- `hides_user_configured_when_locked`
- `invents_private_row_grammar`
- `drops_export_or_audit_truth`

The Rust validator and resolver in `crates/aureline-shell` are the authoritative
gate; the schema and this doc document the shape. Regenerate the checked export,
CSV, report, and fixtures with the headless emitter subcommands
(`support-export`, `csv`, `report`, `fixture-admin-enterprise-beta-narrowed`,
`fixture-update-channel-preview-narrowed`).
