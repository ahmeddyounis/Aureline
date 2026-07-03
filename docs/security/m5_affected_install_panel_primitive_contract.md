# M5 affected-install assessment panel primitive contract

One reusable affected-install assessment panel primitive for Aureline. Whenever a user,
admin, or support needs one precise answer to "am I affected?", this primitive renders
the same panel so the current build / channel / install-mode identity, the impacted
components, the current exposure, the mitigation status, the mirror freshness, and the
rollback / repin / help actions are visible inline — bound to the actual installed build
and the local install graph, never behind a generic "an update is available" banner and
never dependent on an external website lookup.

- **Module:** `crates/aureline-shell/src/implement_the_m5_affected_install_assessment_panel_primitive`
- **Boundary schema:** `schemas/security/m5-affected-install-panel.schema.json`
- **Support export:** `artifacts/release/m5-affected-install-panel-proof/support_export.json`
- **Matrix CSV:** `artifacts/release/m5-affected-install-panel-proof/matrix.csv`
- **Markdown report:** `artifacts/security/m5-affected-install-panel-primitive.md`
- **Narrowed fixtures:** `fixtures/security/m5-affected-install-panel-primitive/`
- **Emitter:** `cargo run -p aureline-shell --bin aureline_shell_m5_affected_install_panel_primitive -- <subcommand>`

This primitive *narrows* the affected-install-panel family of the frozen M5
advisory-component matrix
(`schemas/security/m5-advisory-component-matrix.schema.json`, minted by
`freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix`)
into a working assessment panel, and aligns its assessment vocabulary to the frozen
affected-install assessment record
(`schemas/security/affected_install_assessment.schema.json`), the install-row contract
(`schemas/release/install_row.schema.json`, whose `install_mode_class` vocabulary the
install-profile lanes align to field-for-field), and the advisory-identity / install
assessment contract (`docs/security/advisory_identity_and_install_assessment_contract.md`).
It reuses the matrix's severity classes, action states, required actions, continuity
claims, delivery profiles, mirror-freshness states, export fields, accessibility routes,
qualification classes, and downgrade triggers verbatim; reuses the advisory-row
primitive's install state and derived exposure state; and reuses the frozen shell-zone
matrix's zones, responsive classes, window classes, and consumer surfaces. The derived
assessment verdict is a resolver-side vocabulary, kept out of the frozen set.

## Resolver

`resolve_affected_install(&M5AffectedInstallResolutionInput) -> Result<M5ResolvedAffectedInstall, M5AffectedInstallResolutionError>`
takes one advisory on one install-profile lane — its copy-safe advisory id, severity,
affected object, exact build / channel identity, impacted components, install state,
mirror freshness, delivery profile, fixed build or mitigation, signer / source state,
action state, primary and help actions, and local-continuity claim — and produces one
resolved panel. The resolver:

- derives the current-**exposure state** from the install state
  (`installed_active` → `exposed`, `installed_mitigated` → `mitigated_in_place`,
  `installed_blocked` → `contained_by_block`, `installed_disabled` →
  `contained_by_disable`, `installed_awaiting_rollback` → `awaiting_rollback`,
  `not_installed` → `not_affected`, `superseded` → `resolved`), reusing the frozen
  advisory-row map;
- derives the **assessment verdict** — the "am I affected?" answer — from that exposure
  and the mirror freshness. A clean exposure (`not_affected`, `resolved`, or
  `mitigated_in_place`) over a **non-authoritative** mirror (`stale_past_grace`,
  `offline_expired`, or `unknown`) is auto-narrowed to `clean_pending_mirror_refresh` so
  mirror lag is disclosed instead of staying silently green; only an `up_to_date` or
  `stale_within_grace` mirror is authoritative enough to assert a clean verdict. An
  active exposure is never softened by mirror staleness;
- resolves against the **local install graph** — `resolved_from_local_graph` is true and
  `requires_external_website_lookup` is false by construction, so a claimed install
  profile answers "am I affected?" with no external website lookup;
- keeps the **mirror freshness and install mode visible** in the same surface
  (`mirror_freshness_visible`, `install_mode_visible`);
- keeps the **rollback / repin / help actions attached** to the panel
  (`actions_attached_to_panel`, `attached_actions`) instead of scattering them across
  separate surfaces;
- keeps `remains_visible` true by construction — the primitive structurally cannot hide
  the panel;
- projects the same assessment verdict, mirror freshness, install mode, and primary
  action into every channel (`update_center`, `help_about`, `support_bundle`,
  `admin_report`) so the assessment stays in parity across surfaces; and
- emits a copy-safe, export-safe summary carrying the mandatory export columns
  (`advisory_id`, `severity`, `action_state`, `affected_surface`, `mitigation_state`,
  `delivery_profile`, `freshness_state`, `continuity_note`) so the same assessment
  survives support bundles and admin reports.

Resolution rejects an empty advisory id, empty affected object, empty build identity,
empty impacted components, empty fixed-build-or-mitigation, empty signer / source state,
and any representation carrying forbidden material.

## Parity matrix

`M5AffectedInstallPanelPacket` binds one row per install-profile lane
(`per_user_installed`, `per_machine_installed`, `portable`, `offline_bundle`,
`managed_deployed`, `side_by_side_preview`) to the shared panel anatomy, the severity
vocabulary, every channel, the delivery profiles, the mirror-freshness states, the export
fields, and the accessibility routes. Every lane carries worked resolution cases whose
stored resolution must equal a fresh resolve of its input (`example_assessment_drift`).

### Panel anatomy (all mandatory — visible inline, no detail drawer)

`install_identity`, `impacted_components`, `current_exposure`, `mitigation_status`,
`mirror_freshness`, `primary_action`, `help_support_action`.

### Hard invariants (every row)

- never hides assessment truth behind a detail drawer,
- never degrades to a generic "update available" prompt,
- never requires an external website lookup to resolve,
- never lets a stale mirror stay silently green,
- never drops the copy-safe id or export summary.

### Acceptance-criterion lints (packet)

- **`local_graph_resolution_unproven`** — some worked resolution must resolve an
  installed-but-affected build against the local install graph with no website lookup
  and a complete export summary (AC1: claimed M5 install profiles resolve advisory state
  against the local install graph without an external website lookup).
- **`mirror_freshness_install_mode_unproven`** — every worked resolution must keep the
  mirror freshness and install mode visible, and some worked resolution must prove a
  stale / expired / unknown mirror auto-narrows a clean verdict to
  `clean_pending_mirror_refresh` (AC2: mirror freshness and install mode remain visible
  in the same assessment surface, and mirror lag never stays silently green).
- **`attached_actions_unproven`** — every worked resolution must keep its actions
  attached to the panel, and the union of attached actions must cover both a
  rollback / repin action and a help / support action (AC3: rollback / repin / help
  actions stay attached to the affected-install panel instead of being scattered).
- **`verdict_coverage_unproven`** / **`severity_coverage_unproven`** — the worked
  resolutions must exercise every assessment verdict and every severity class, including
  the mirror-refresh-pending verdict.

## Governance

Stale proof auto-narrows the primitive (`proof_freshness.auto_narrow_on_stale`). Narrowed
variants (managed deployed → Beta, offline bundle → Preview) hold a single lane below
Stable while keeping every lane visible. Raw hostnames, absolute paths, exploit payloads,
signatures, private registry URLs, and credentials never cross the boundary; only opaque,
export-safe reprs are carried. The Rust validator and resolver in `crates/aureline-shell`
are the authoritative gate.
