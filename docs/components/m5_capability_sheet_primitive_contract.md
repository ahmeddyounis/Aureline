# M5 Capability-Sheet Primitive Contract — Consequence Grouping, Transitive Scope, and Re-consent

> Task: M05-758 · Batch B88 · Delivery class: high-trust component contract +
> reusable primitive implementation + support/export parity.

This contract implements Aureline's **one reusable capability-sheet primitive**
across every M5 trust lane that asks for meaningful access — extension install, AI
tool, provider route, remote connector, automation flow, and privileged helper —
so consequence grouping, transitive scope, reduced-mode choices, and the stable
revoke / re-consent paths stay consistent instead of collapsing into vague
per-feature "grant access?" prompts. It narrows the capability-sheet family named
by the frozen
[M5 trust-chronology component matrix](m5_trust_chronology_components_contract.md)
(M05-756) into a working primitive with a resolver and a per-lane parity matrix.

- **Boundary schema:** [`schemas/ui/m5-capability-sheet.schema.json`](../../schemas/ui/m5-capability-sheet.schema.json)
- **Rust source of truth:** `crates/aureline-shell/src/implement_the_m5_capability_sheet_consequence_grouping_transitive_scope_and_reconsent_primitive/`
- **Headless emitter:** `aureline_shell_m5_capability_sheet_primitive`
- **Checked support export:** [`artifacts/release/m5-capability-sheet-proof/support_export.json`](../../artifacts/release/m5-capability-sheet-proof/support_export.json)
- **Matrix CSV:** [`artifacts/release/m5-capability-sheet-proof/matrix.csv`](../../artifacts/release/m5-capability-sheet-proof/matrix.csv)
- **Report:** [`artifacts/components/m5-capability-sheet-primitive.md`](../../artifacts/components/m5-capability-sheet-primitive.md)
- **Narrowed fixtures:** [`fixtures/ui/m5-capability-sheet-primitive/`](../../fixtures/ui/m5-capability-sheet-primitive/)

The capability consequence classes, scope states, non-visual accessibility routes,
qualification classes, and downgrade triggers are reused verbatim from the frozen
[M5 trust-chronology component matrix](../../schemas/ui/m5-trust-chronology-components.schema.json);
the shell topology — zones, responsive classes, window classes, and consumer
surfaces — is reused verbatim from the frozen
[M5 shell-zone matrix](../../schemas/shell/m5-shell-zone.schema.json). This lane
mints new vocabulary only for what those matrices left implicit about the
capability sheet itself: its **trust-lane families**, its **anatomy parts**, its
**consent disclosures**, its **focus behaviors**, and its **export fields**. No M5
surface invents a second capability-prompt grammar.

The primitive also projects from the existing high-trust permission contracts:
[`schemas/trust/capability_sheet.schema.json`](../../schemas/trust/capability_sheet.schema.json),
[`schemas/extensions/effective_permission.schema.json`](../../schemas/extensions/effective_permission.schema.json),
and [`schemas/policy/permission_prompt_event.schema.json`](../../schemas/policy/permission_prompt_event.schema.json).

## Track invariant

One capability-sheet model groups requests by consequence, shows transitive scope,
and preserves reduced-mode and re-consent behavior. A request is never grouped by
an internal API name; a dependency that widens effective scope is always disclosed
before approval; policy pre-approve / pre-deny states and re-consent triggers are
preserved; and remembered approvals stay revocable from a stable trust surface with
the same capability vocabulary in the support export.

## The primitive: two halves

### 1. Resolver — `resolve_capability_sheet`

Given one actor's requested capabilities (each a `capability_token`, a
`consequence_class`, a `purpose_repr`, a `decision`, a `policy_predecision`, a
transitive-origin, a reduced-mode-availability flag, and re-consent / prior-grant
flags), the resolver produces one `M5ResolvedCapabilitySheet` carrying:

- one `scope_state` per request,
- the `consequence_groups` — requested capabilities grouped by consequence class in
  canonical order, never by request order or API name,
- `widens_effective_scope` (any transitive request),
- `reduced_mode_offered` (any request offering a narrower grant),
- `requires_re_consent` (any request needing new consent), and
- `revocable_from_settings` (any remembered grant revocable from a stable surface).

Per-request scope resolution order (first match wins): `revoke` →
`revoked_with_history`; a triggered re-consent on a standing grant →
`re_consent_required`; `approve_reduced` → `granted_reduced_scope`; `approve_full`
→ `granted_full_scope`; a transitive-but-ungranted request →
`transitive_scope_disclosed`; otherwise `requested_not_granted`. A grant is
revocable when it is currently held (`granted_full_scope`, `granted_reduced_scope`,
or `re_consent_required`); a not-yet-granted, transitive-disclosed, or
already-revoked request has nothing to revoke.

The resolver rejects malformed input: an empty actor identity, no requests, an
empty capability token or purpose, a duplicate token, a transitive request with no
origin, a reduced grant where reduced mode is unavailable, a **policy pre-denied
capability approved locally**, and any representation carrying URLs, credentials, or
other forbidden material.

### 2. Parity matrix — one row per trust lane

Each of the six trust lanes carries the same shared anatomy, the same consequence
classes and scope states, the same consent disclosures and focus behaviors, and the
same export fields, plus worked resolution cases proving the resolver on that lane.

| Trust lane | Zone | Worked resolution highlight |
| --- | --- | --- |
| `extension_install` | `transient_overlay` | full read + **reduced-scope** workspace edit |
| `ai_tool_request` | `transient_overlay` | execute requested + **transitive** network scope disclosed |
| `provider_route` | `transient_overlay` | remembered grant needs **re-consent**, stays revocable |
| `remote_connector` | `transient_overlay` | credential grant **revoked with history** |
| `automation_flow` | `transient_overlay` | policy **pre-approved** execute + **pre-denied** system-control |
| `privileged_helper` | `transient_overlay` | full system-control grant + transitive credential disclosure |

## Anatomy (shared sheet)

`actor_identity`, `purpose_text`, `consequence_grouped_requests`, `scope_choice`,
`reduced_mode_option`, `approve_action`, `deny_action`, and `detail_action` are
mandatory on every sheet. `transitive_scope_disclosure` is the conditional part
shown whenever a dependency widens effective scope.

## Consent disclosure

`remembered_grant_shown`, `revoke_path_shown`, and `no_silent_scope_widening` are
mandatory — a remembered approval always shows a stable revoke path and scope never
widens without an explicit new consent. `re_consent_reason_explained`,
`reduced_scope_disclosed`, and `transitive_origin_shown` complete the disclosure.

## Focus and approval

`approve_requires_explicit_focus` keeps approval deliberate (never default-focused);
`detail_side_sheet_escalation` and `return_focus_on_close` govern the detail escape;
`per_consequence_group_navigation` and `reduced_mode_toggle_reachable` keep the sheet
keyboard-operable; and `deep_link_to_revoke` addresses the stable revoke surface.

## Support / export reconstruction

The export fields `actor_identity_repr`, `consequence_class`, `capability_token`,
and `scope_state` are mandatory; `transitive_origin_repr`, `reduced_mode_offered`,
and `revocable_from_settings` complete the record. Each lane carries its worked
resolution cases in the export, and the validator re-runs the resolver on every
stored input and asserts it equals the stored output. Packet-level lints require at
least one worked case each that (a) discloses transitive scope before approval, (b)
grants a reduced scope, and (c) holds a remembered grant revocable from a stable
surface — so the support export reconstructs capability truth from the same shared
sheet model with the same vocabulary.

## Hard invariants (per surface row, all MUST be false)

- `drops_consequence_grouping` (a vague generic access prompt)
- `hides_transitive_scope`
- `skips_required_re_consent`
- `drops_export_or_audit_truth`

The Rust validator and resolver in `crates/aureline-shell` are the authoritative
gate; the schema and this doc document the shape. Regenerate the checked export,
CSV, report, and fixtures with the headless emitter subcommands (`support-export`,
`csv`, `report`, `fixture-automation-flow-beta-narrowed`,
`fixture-privileged-helper-preview-narrowed`).
