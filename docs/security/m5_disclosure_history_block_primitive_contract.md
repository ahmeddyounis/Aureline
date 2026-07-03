# M5 disclosure / history block primitive contract

One reusable disclosure / history block primitive for Aureline. Whenever a user or support
needs to inspect an advisory's disclosure details and resolved-state history, this
primitive renders the same block so the current status, the affected versions / components,
the disclosure or learn-more path, the copy-safe reference ids (the Aureline advisory id
plus its CVE / GHSA aliases), the provenance, the resolved-versus-active history state, and
the open-doc / open-browser actions are visible inline — never flattened into a bare
"learn more" link to an external page, and never dropping a resolved advisory out of
inspectable history.

- **Module:** `crates/aureline-shell/src/implement_the_m5_disclosure_and_history_block_primitive`
- **Boundary schema:** `schemas/security/m5-disclosure-history-block.schema.json`
- **Support export:** `artifacts/release/m5-disclosure-history-block-proof/support_export.json`
- **Matrix CSV:** `artifacts/release/m5-disclosure-history-block-proof/matrix.csv`
- **Markdown report:** `artifacts/security/m5-disclosure-history-block-primitive.md`
- **Narrowed fixtures:** `fixtures/security/m5-disclosure-history-block-primitive/`
- **Emitter:** `cargo run -p aureline-shell --bin aureline_shell_m5_disclosure_history_block_primitive -- <subcommand>`

This primitive *narrows* the disclosure-block family of the frozen M5 advisory-component
matrix (`schemas/security/m5-advisory-component-matrix.schema.json`, minted by
`freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix`)
into a working disclosure / history block, and aligns its vocabulary to the frozen
advisory-identity record (`schemas/security/advisory_identity.schema.json`, whose Aureline /
CVE / GHSA id family the copy-safe reference kinds mirror), the advisory history and
resolution contract (`docs/security/advisory_history_and_resolution_contract.md`, whose
`entry_class` vocabulary the history states align to field-for-field, and whose
resolved-state downgrade rules the display posture honors), and the postmortem /
compensating-control contract
(`docs/security/postmortem_and_compensating_control_contract.md`). It reuses the matrix's
severity classes, action states, required actions, continuity claims, delivery profiles,
mirror-freshness states, disclosure fields, export fields, accessibility routes,
qualification classes, and downgrade triggers verbatim; and reuses the frozen shell-zone
matrix's zones, responsive classes, window classes, and consumer surfaces. The derived
display and handoff postures are resolver-side vocabularies, kept out of the frozen set.

## Resolver

`resolve_disclosure_block(&M5DisclosureBlockResolutionInput) -> Result<M5ResolvedDisclosureHistoryBlock, M5DisclosureBlockResolutionError>`
takes one advisory's disclosure state on one disclosure-source lane — its copy-safe
advisory id, optional CVE / GHSA aliases, severity, affected object, current status,
history state, delivery profile, mirror freshness, disclosure path, provenance, visibility
posture, action state, and local-continuity claim — and produces one resolved block. The
resolver:

- derives the **display posture** from the history state (`published` / `mitigated` →
  `full_weight`; `superseded` / `resolved` / `withdrawn` → `stepped_down_inspectable`;
  `draft` → `draft_restricted`), so a resolved advisory steps down its visual weight but
  never loses its current-status truth. `remains_inspectable` and `current_status_visible`
  are true by construction;
- derives the **handoff posture** from the disclosure-source lane (`first_party_signed` →
  `in_product_doc`; `mirrored` → `mirror_provenance_preserved`; `offline_imported` →
  `offline_import_provenance_preserved`; `externally_linked` / `community_postmortem` /
  `vendor_cross_reference` → `external_browser_provenance_preserved`), so a mirrored,
  offline-imported, or externally linked source keeps its provenance visible.
  `preserves_in_product_state_on_handoff` is true and `is_dead_end_link` is false by
  construction — an external handoff never replaces the in-product disclosure state with a
  dead-end link;
- assembles the **copy-safe reference ids** — the Aureline advisory id is always present;
  the CVE and GHSA aliases are added when non-empty — as copy-safe identifiers, never
  links (`reference_ids_copy_safe`);
- keeps the **open-doc / open-browser / copy-ids actions** attached
  (`open_in_product_doc`, `open_external_browser`, `copy_reference_ids`);
- keeps `remains_visible` true by construction — the primitive structurally cannot hide
  the block;
- projects the same history state, display posture, severity, primary reference id, and
  handoff posture into every channel (`help_about`, `update_center`, `support_bundle`) so
  the disclosure stays in parity across surfaces with one copy-safe id behavior; and
- emits a copy-safe, export-safe summary carrying the mandatory export columns
  (`advisory_id`, `severity`, `action_state`, `affected_surface`, `mitigation_state`,
  `disclosure_visibility`, `history_state`, `continuity_note`) so the same disclosure
  survives support bundles.

Resolution rejects an empty advisory id, empty affected object, empty current status,
empty disclosure path, empty provenance, empty visibility posture, and any representation
carrying forbidden material.

## Parity matrix

`M5DisclosureHistoryBlockPacket` binds one row per disclosure-source lane
(`first_party_signed`, `mirrored`, `offline_imported`, `externally_linked`,
`community_postmortem`, `vendor_cross_reference`) to the shared block anatomy, the severity
vocabulary, every channel, the disclosure fields, the history states, the export fields,
and the accessibility routes. Every lane carries worked resolution cases whose stored
resolution must equal a fresh resolve of its input (`example_disclosure_drift`).

### Block anatomy (all mandatory — visible inline, no detail drawer)

`current_status`, `affected_versions_components`, `reference_ids`, `disclosure_path`,
`provenance_source`, `history_state`, `open_actions`.

### Hard invariants (every row)

- never flattens the disclosure into a bare external link,
- never hides disclosure truth behind a detail drawer,
- never drops a resolved advisory out of inspectable history,
- never hides provenance when the source is mirrored, offline, or external,
- never drops the copy-safe id or export summary.

### Acceptance-criterion lints (packet)

- **`shared_primitive_parity_unproven`** — every worked resolution must project all three
  channels with identical core truth, and some worked resolution must carry a copy-safe
  reference set (the Aureline id plus at least one alias) with a full export summary (AC1:
  advisory detail in Help/About, update, and support lanes shares one disclosure / history
  primitive and one copy-safe id behavior).
- **`resolved_step_down_unproven`** — every worked resolution must remain inspectable with
  current-status truth, and some worked resolution must prove a resolved / superseded /
  withdrawn advisory steps down to `stepped_down_inspectable` but stays inspectable (AC2:
  resolved advisories step down visually but remain inspectable with current-status truth).
- **`provenance_handoff_unproven`** — every worked resolution must preserve the in-product
  disclosure state on handoff and keep provenance visible, and some worked resolution must
  exercise a remote / external provenance-preserved handoff (AC3: external handoff
  preserves provenance and does not replace the in-product disclosure state with a
  dead-end link).
- **`history_state_coverage_unproven`** / **`severity_coverage_unproven`** — the worked
  resolutions must exercise every history state and every severity class, including the
  resolved / superseded / withdrawn states that step down their visual weight.

## Governance

Stale proof auto-narrows the primitive (`proof_freshness.auto_narrow_on_stale`). Narrowed
variants (offline imported → Beta, externally linked → Preview) hold a single lane below
Stable while keeping every lane visible. Raw hostnames, absolute paths, exploit payloads,
signatures, private registry URLs, credentials, and raw disclosure bodies never cross the
boundary; only opaque, export-safe reprs and copy-safe reference ids are carried. The Rust
validator and resolver in `crates/aureline-shell` are the authoritative gate.
