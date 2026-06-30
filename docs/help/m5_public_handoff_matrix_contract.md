# M5 Post-Install Notice/Provenance, Community-Handoff, Reproduction-Packet, and Device-Permission/Auth-Boundary Matrix

This document is the contract for the frozen M5 matrix that names the canonical
Aureline public-handoff and capture-boundary object model. The matrix is the single
M5 source of truth for whether claimed help, support, ecosystem, and voice/capture
surfaces may publish a handoff or boundary promise: Help/About, the marketplace,
update/service-health, community handoff, reproduction packets, and capture/auth
surfaces ingest the checked-in packet rather than maintaining parallel dialogs.

- Record kind: `freeze_m5_public_handoff_and_capture_boundary_matrix`
- Schema: [`schemas/help/m5-public-handoff-matrix.schema.json`](../../schemas/help/m5-public-handoff-matrix.schema.json)
- Canonical support export: [`artifacts/help/m5-public-handoff/support_export.json`](../../artifacts/help/m5-public-handoff/support_export.json)
- Governance summary: [`artifacts/help/m5-public-handoff-governance.md`](../../artifacts/help/m5-public-handoff-governance.md)
- Matrix CSV: [`artifacts/help/m5-public-handoff-matrix.csv`](../../artifacts/help/m5-public-handoff-matrix.csv)
- Fixtures: [`fixtures/help/m5-public-handoff/`](../../fixtures/help/m5-public-handoff/)
- Producer: `aureline_shell::freeze_the_m5_public_handoff_and_capture_boundary_matrix::current_stable_m5_public_handoff_matrix_export`
- Headless emitter: `aureline_shell_m5_public_handoff_matrix`

## Governed objects

| Object | Qualification | Owner | State vocabularies | Source contract |
| --- | --- | --- | --- | --- |
| `post_install_notice` | Stable | Help/About owner | provenance_class / notice_freshness_state | [`schemas/help/provenance_badge_vocabulary.schema.json`](../../schemas/help/provenance_badge_vocabulary.schema.json) |
| `provenance_disclosure` | Stable | Help/About owner | provenance_class / notice_freshness_state | [`schemas/help/provenance_badge_vocabulary.schema.json`](../../schemas/help/provenance_badge_vocabulary.schema.json) |
| `community_handoff_route` | Stable | Ecosystem owner | route_trust_class / continuity_state | [`schemas/help/community-handoff-packet.schema.json`](../../schemas/help/community-handoff-packet.schema.json) |
| `reproduction_packet` | Stable | Supportability owner | redaction_state / continuity_state | [`schemas/public/repro_packet_preview.schema.json`](../../schemas/public/repro_packet_preview.schema.json) |
| `offline_capture_continuity` | Stable | Supportability owner | continuity_state / redaction_state | [`schemas/public/repro_packet_preview.schema.json`](../../schemas/public/repro_packet_preview.schema.json) |
| `device_permission_boundary` | Beta | Voice/capture owner | capture_permission_state / boundary_chrome_honesty | [`schemas/help/service-health-destination.schema.json`](../../schemas/help/service-health-destination.schema.json) |
| `embedded_auth_boundary` | Beta | Browser/auth boundary owner | boundary_chrome_honesty / route_trust_class | [`schemas/help/community-handoff-packet.schema.json`](../../schemas/help/community-handoff-packet.schema.json) |
| `service_health_notice` | Stable | Service-health owner | route_trust_class / notice_freshness_state | [`schemas/help/service-health-destination.schema.json`](../../schemas/help/service-health-destination.schema.json) |

Each object row binds a qualification class to its required fields, the controlled
state vocabularies it carries, the concrete vocabulary tokens it admits, its
evidence requirement, the proof packet refs that keep it current, its downgrade
triggers, its rollback posture, its source contracts, and the consumer surfaces
that must project its qualification truth. An object kind's required state
vocabularies must appear in `state_vocabularies`, and a declared vocabulary must
carry concrete tokens while an undeclared vocabulary must carry none — so the
matrix is exact about which truth each object speaks.

## Controlled vocabulary

The matrix freezes one self-describing `vocabulary_set` block, mapped onto the
canonical tokens already owned by the community-handoff packet, the provenance
badge vocabulary, the service-health destination contract, and the M3
handoff-target / repro-packet contracts rather than minting parallel tokens:

- **Provenance class** — `official`, `mirrored`, `side_loaded`, `unknown`. A
  side-loaded or unknown source is never softened into an implied official one.
- **Route trust class** — `official`, `community`, `private`, `local_only`. A
  community destination is never presented as an official authenticated one; a
  `local_only` posture means nothing leaves the device.
- **Capture permission state** — `granted`, `scope_limited`, `not_requested`,
  `denied`, `revoked`. A capture surface never acts beyond its granted permission
  and capability scope.
- **Redaction state** — `preview_required`, `previewed_redacted`,
  `no_sensitive_material`, `unredacted_blocked`. Raw sensitive material never leaves
  implicitly; share stays blocked until the preview is confirmed.
- **Continuity state** — `ready_to_launch`, `launch_failed_retained`,
  `blocked_retained`, `offline_saved_local`. A failed or blocked launch retains
  drafted material and falls back to a durable local save.
- **Boundary chrome-honesty** — `native_trusted_chrome`, `clearly_embedded`,
  `labeled_external_surface`, `unattributed_impersonation_blocked`. A webview or
  auth surface never impersonates native trusted product chrome.
- **Notice freshness** — `proven_current`, `cached`, `warming`, `stale`,
  `unverified`. `stale` keeps one reserved meaning across help, About,
  service-health, support, and release surfaces; a cached or stale notice never
  implies current authority.

The `vocabulary_set` block must match these canonical token lists exactly; any
drift fails validation with `vocabulary_set_drift`.

## Track invariant

Help and community handoff stay boundary-honest and privacy-bounded. The
`trust_review` block encodes the lane invariants as hard flags — all must hold for
the matrix to validate:

- `post_install_provenance_inspectable_after_install` — post-install provenance and
  notice states remain inspectable after install.
- `outbound_routes_declare_visibility_and_support_class_before_launch` — outbound
  public/community routes declare visibility and support class before launch.
- `repro_packets_previewed_and_redacted_before_share` — reproduction packets are
  previewed and redacted before share.
- `offline_capture_survives_failed_handoff` — offline capture survives a failed or
  blocked handoff.
- `device_mic_auth_webview_never_impersonates_native_chrome` — device, mic, auth,
  and webview boundaries never impersonate native trusted product chrome.
- `provenance_states_distinguish_official_mirrored_side_loaded_unknown` — provenance
  states stay distinct.
- `capture_stays_within_granted_permission_and_capability_limit` — capture stays
  within its granted permission and capability scope.
- `one_handoff_object_model_not_parallel_dialogs` — every surface resolves to one
  handoff object model.
- `no_new_community_programs_or_capture_modalities` — no new community programs or
  capture modalities are invented.
- `redaction_default_excludes_raw_sensitive_material` — redaction default excludes
  raw sensitive material.
- `downgrade_narrows_instead_of_hides` and
  `stale_or_underqualified_blocks_promotion`.

## Consumer projection and release posture

`consumer_projection` binds every consumer surface to the shared object model:
Help/About, marketplace, update/service-health, community handoff, reproduction
packets, capture/auth surfaces, support export, docs, and release notes all read the
same packet, and Preview/Labs surfaces are visibly labeled when not covered. The
`release_posture` block binds the supporting release packet
(`evidence:public-handoff-release-packet:m5`) and the mirror/offline packet
(`evidence:public-handoff-mirror-offline-packet:m5`) and requires support/export and
mirror/offline parity for every object.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and the last-refresh timestamp; when
proof goes stale `auto_narrow_on_stale` narrows the affected object. The supported
downgrade triggers — each naming a gap the matrix fails or narrows on rather than
leaving implied — are `notice_stale`, `provenance_unverified`,
`route_visibility_undeclared`, `redaction_preview_missing`, `offline_continuity_lost`,
`capture_scope_exceeded`, `native_chrome_impersonation`, `policy_blocked`,
`proof_stale`, and `upstream_dependency_narrowed`. The
[fixtures](../../fixtures/help/m5-public-handoff/) show a held reproduction packet
(after a missing-redaction-preview finding) and a preview-narrowed provenance
disclosure (after an unverified-provenance finding); both remain valid packets
because narrowing is explicit, not hidden.

Stable promotion of any claimed M5 help/support/ecosystem/capture-boundary row that
maps to a governed object fails while that object lacks a current matrix entry and
mapped proof packet: `current_stable_m5_public_handoff_matrix_export` revalidates the
checked-in packet, and a missing object, drifted vocabulary, missing proof ref, or
unsatisfied trust invariant blocks the packet.

## Boundary

Raw URLs, raw local paths, raw usernames, raw hostnames, tokens, raw diagnostics,
private endpoints, credentials, and user text bodies never cross this boundary. The
packet carries only metadata, qualification truth, controlled-vocabulary tokens, and
contract references.

## Regeneration

The seed builders in
`crates/aureline-shell/src/freeze_the_m5_public_handoff_and_capture_boundary_matrix/seed.rs`
are the single producer of the checked-in artifacts. To regenerate after a change:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_public_handoff_matrix -- support-export > artifacts/help/m5-public-handoff/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_public_handoff_matrix -- governance > artifacts/help/m5-public-handoff-governance.md
cargo run -q -p aureline-shell --bin aureline_shell_m5_public_handoff_matrix -- csv > artifacts/help/m5-public-handoff-matrix.csv
cargo run -q -p aureline-shell --bin aureline_shell_m5_public_handoff_matrix -- fixture-repro-redaction-held > fixtures/help/m5-public-handoff/repro_redaction_held.json
cargo run -q -p aureline-shell --bin aureline_shell_m5_public_handoff_matrix -- fixture-provenance-unverified-narrowed > fixtures/help/m5-public-handoff/provenance_unverified_narrowed.json
```

The inline tests assert the checked-in support export and fixtures match the seed
builders, so a drift between code and artifacts fails the build.
