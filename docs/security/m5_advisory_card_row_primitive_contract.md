# M5 security-advisory card / row primitive contract

One reusable advisory card / row primitive for Aureline. Whenever a published
vulnerability, revocation, or security-impacting fix affects the desktop app, an
extension, a remote helper, a managed service, a docs artifact, or a signing / update
path, this primitive renders the same row so severity, affected scope, current
exposure, and the next action are visible inline — never hidden behind a generic
update banner and never dropped just because the affected item is blocked, disabled,
or awaiting rollback.

- **Module:** `crates/aureline-shell/src/implement_the_m5_advisory_card_and_row_primitive`
- **Boundary schema:** `schemas/security/m5-advisory-card-row.schema.json`
- **Support export:** `artifacts/release/m5-advisory-card-row-proof/support_export.json`
- **Matrix CSV:** `artifacts/release/m5-advisory-card-row-proof/matrix.csv`
- **Markdown report:** `artifacts/security/m5-advisory-card-row-primitive.md`
- **Narrowed fixtures:** `fixtures/security/m5-advisory-card-row-primitive/`
- **Emitter:** `cargo run -p aureline-shell --bin aureline_shell_m5_advisory_card_row_primitive -- <subcommand>`

This primitive *narrows* the frozen M5 advisory-component matrix
(`schemas/security/m5-advisory-component-matrix.schema.json`, minted by
`freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix`)
into a working advisory card / row. It reuses that matrix's severity classes, action
states, required actions, continuity claims, export fields, accessibility routes,
qualification classes, and downgrade triggers verbatim, and reuses the frozen
shell-zone matrix's zones, responsive classes, window classes, and consumer surfaces.
The install state and the derived exposure state are resolver-side vocabularies, kept
out of the frozen set.

## Resolver

`resolve_advisory_row(&M5AdvisoryRowResolutionInput) -> Result<M5ResolvedAdvisoryRow, M5AdvisoryRowResolutionError>`
takes one advisory affecting one surface lane — its copy-safe id, severity, affected
object, install state, fixed version or mitigation, signer / source state, action
state, primary action, and local-continuity claim — and produces one resolved row.
The resolver:

- derives the normalized **exposure state** from the install state
  (`installed_active` → `exposed`, `installed_mitigated` → `mitigated_in_place`,
  `installed_blocked` → `contained_by_block`, `installed_disabled` →
  `contained_by_disable`, `installed_awaiting_rollback` → `awaiting_rollback`,
  `not_installed` → `not_affected`, `superseded` → `resolved`);
- marks `installed_but_affected` for the active / blocked / disabled /
  awaiting-rollback states so those rows never disappear;
- keeps `remains_visible` true and `degrades_to_generic_prompt` false by
  construction — the primitive structurally cannot hide an advisory or collapse it
  into a generic update prompt;
- projects the same severity, exposure, and primary action into every channel
  (`update_center`, `marketplace`, `help_about`, `support_bundle`) so the row stays
  in parity across surfaces; and
- emits a copy-safe, export-safe summary carrying the mandatory export columns
  (`advisory_id`, `severity`, `action_state`, `affected_surface`, `mitigation_state`,
  `continuity_note`) for support and admin flows.

Resolution rejects an empty advisory id, empty affected object, empty
fixed-version-or-mitigation, empty signer / source state, and any representation
carrying forbidden material.

## Parity matrix

`M5AdvisoryRowPrimitivePacket` binds one row per affected-surface lane
(`desktop_app`, `extension`, `remote_helper`, `managed_service`, `docs_artifact`,
`signing_update_path`) to the shared advisory-row anatomy, the severity vocabulary,
every channel, the export fields, and the accessibility routes. Every lane carries
worked resolution cases whose stored resolution must equal a fresh resolve of its
input (`ExampleAdvisoryDrift`).

### Row anatomy (all mandatory — visible inline, no detail drawer)

`advisory_id`, `severity`, `affected_surface`, `current_exposure`,
`fixed_version_or_mitigation`, `signer_source_state`, `primary_action`.

### Hard invariants (every row)

- never hides advisory truth behind a detail drawer,
- never disappears for an installed-but-affected item,
- never degrades to a generic update prompt,
- never drops the copy-safe id or export summary.

### Acceptance-criterion lints (packet)

- **`channel_parity_unproven`** — some worked resolution must project every channel
  in parity (AC1: update, marketplace, Help / About, and support render the same
  row).
- **`inline_visibility_unproven`** — some worked resolution must render a full
  advisory row inline with a complete export summary (AC2: severity, scope, exposure,
  and next action visible without a secondary detail drawer).
- **`installed_but_affected_unproven`** — some worked resolution must keep a
  blocked / disabled / awaiting-rollback item's row visible without degrading (AC3:
  installed-but-affected items no longer disappear or degrade to generic prompts).
- **`severity_coverage_unproven`** / **`exposure_coverage_unproven`** — the worked
  resolutions must exercise every severity class and every exposure state.

## Governance

Stale proof auto-narrows the primitive (`proof_freshness.auto_narrow_on_stale`).
Narrowed variants (extension → Beta, signing / update path → Preview) hold a single
lane below Stable while keeping every lane visible. Raw reporter identities, exploit
payloads, signatures, hostnames, paths, private registry URLs, and credentials never
cross the boundary; only opaque, export-safe reprs are carried. The Rust validator and
resolver in `crates/aureline-shell` are the authoritative gate.
