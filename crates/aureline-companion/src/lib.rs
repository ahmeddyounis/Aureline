//! Frozen M5 companion, incident, sync, residency, and offboarding truth packet.
//!
//! This crate owns the single export-safe packet that locks the M5 depth
//! qualification for the companion, incident, managed-sync, residency, and
//! offboarding lanes into one frozen matrix with staged rollout lanes. Each lane
//! row binds a lane to its domain, its qualification class, its staged rollout
//! stage, an explicit locality disclosure of what stays local, what is staged,
//! and what requires provider or admin continuity, its required evidence packet
//! refs, the downgrade triggers that can narrow it, a rollback posture, its
//! source contracts, and the consumer surfaces that must project it.
//!
//! The matrix is the canonical M5 control source for this lane: later companion,
//! incident, support, diagnostics, and Help/About surfaces ingest it instead of
//! cloning status text. It keeps browser and mobile companions narrow, keeps
//! incident packets attributable, keeps managed sync inspectable, keeps
//! customer-managed and end-to-end-encryption residency claims provable, and
//! guarantees offboarding never strands user-owned local work. Credential bodies,
//! raw provider payloads, and raw sync record contents never cross this boundary.
//!
//! The crate also owns the concrete read-only companion triage surface in
//! [`companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff`],
//! which projects notification triage, review queues, and CI-status cards to
//! browser and mobile companions with an exact desktop handoff per item and
//! inherits its per-section qualification from the frozen matrix lanes.
//!
//! Building on those, it owns the session-follow and incident-awareness surface in
//! [`ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty`],
//! which lets browser and mobile companions follow an active desktop session and
//! stay aware of incidents with bounded read/write scope — the follow and
//! awareness surfaces are read-only, only a single bounded light-edit surface may
//! write through a host-approved relay, and stale state is always labeled rather
//! than shown as live.
//!
//! Building further, it owns the incident workspace surface in
//! [`add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets`],
//! which projects the incident workspace header card, the ordered evidence
//! timeline (including first-class missing spans), the read-only resource slices,
//! and the runbook packets to the incident workspace, desktop panel, diagnostics,
//! support exports, and Help/About — every section read-only and inheriting its
//! qualification from the frozen incident-workspace matrix lane, with attribution
//! preserved or honestly narrowed and an exact desktop handoff per item.
//!
//! Building on the incident workspace, it owns the runbook execution surface in
//! [`implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth`],
//! which projects the per-step runbook execution rows, the first-class deviation
//! notes that record every departure from the runbook, the export bundles that
//! package an incident for sharing, and the browser or vendor-console handoff to an
//! external surface — every section read-only and inheriting its qualification from
//! the frozen incident-workspace matrix lane, with an exact desktop handoff per item
//! and a local-first fallback that keeps every external handoff from stranding the
//! user when provider continuity is unavailable.
//!
//! Moving from the incident lanes to the managed-depth lanes, it owns the managed
//! sync maturity surface in
//! [`ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage`],
//! which projects the managed sync snapshot classes, the conflict review queue, the
//! device registry, and the end-to-end encrypted storage posture — binding the first
//! three sections to the frozen managed-sync matrix lane and the encrypted-storage
//! section to the residency-encryption matrix lane. Managed sync stays inspectable
//! and reconcilable to the authoritative local core, conflicts are reviewed by the
//! user rather than resolved silently in the server's favor, customer-managed-key and
//! end-to-end-encryption claims are proved where claimed or honestly labeled where
//! not, and stale state is always labeled rather than shown as live.
//!
//! Completing the managed-depth lanes, it owns the key/storage selection, residency,
//! and degraded managed-service continuity surface in
//! [`add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont`],
//! which projects the customer-managed-key and storage selection flows, the
//! region/residency cues, and the degraded managed-service continuity rows — binding
//! the first three sections to the frozen residency-encryption matrix lane and the
//! continuity section to the offboarding-continuity matrix lane. The surface is
//! read-only and only projects a selection — a key-custody, storage-location, or
//! residency change is applied by the local core, never authored from the surface — a
//! local-only key and local-first storage option are always offered as a fallback,
//! customer-managed-key and region-residency claims are proved where claimed or
//! honestly labeled where not, every continuity row says what stays local and what
//! requires provider or admin continuity so a degraded managed service never strands
//! user-owned local work, and stale state is always labeled rather than shown as live.
//!
//! Completing the offboarding-depth lane, it owns the usage-export and offboarding
//! surface in
//! [`implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho`],
//! which projects the usage-export packages, the full offboarding export bundles, the
//! grace-window state for scheduled deletions, and the org-switch semantics per data
//! class — binding every section to the frozen offboarding-continuity matrix lane. The
//! surface is read-only and only projects state — an export, a deletion, or an org
//! switch is applied by the local core, never authored from the surface — a local-first
//! usage-export and offboarding-package path is always offered as a fallback, export
//! completeness is proved where claimed or honestly labeled where not, an irreversible
//! (committed) deletion is labeled rather than shown as still reversible, user-owned
//! local work is never stranded by offboarding, deletion, or an org switch, and stale
//! state is always labeled rather than shown as live.
//!
//! Capping the M5 lane, it owns the companion-safe redaction, local-core continuity, and
//! offline packet-flow surface in
//! [`ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes`],
//! which ties the companion, incident, and support lanes together around three guarantees:
//! every record that crosses a companion, support, or incident boundary is redaction-safe
//! (no raw payload body crosses, and a redaction is proved where claimed or labeled where
//! not), the local core stays authoritative and its capabilities keep working offline, and
//! the support and incident packets that flow out assemble and replay offline from the local
//! core — binding the redaction section to the frozen companion-notification matrix lane, the
//! incident-packet section to the incident-workspace lane, and the continuity and
//! support-packet sections to the offboarding-continuity lane. The surface is read-only, a
//! local-first packet path is always offered so a degraded provider never strands the support
//! or incident workflow, incident packets stay attributable or are honestly labeled, and
//! stale state is always labeled rather than shown as live.
//!
//! Extending the companion lane into session continuity, it owns the remote-preview,
//! session-handoff, light-remote-edit, and scoped collaboration-follow surface in
//! [`add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio`],
//! which lets a browser or mobile companion remotely preview and hand off an active
//! desktop session, perform a single bounded light remote edit, and follow a
//! collaborator within a host-revocable shared scope — binding the remote-preview-handoff
//! surface to the frozen companion-session-follow matrix lane, the light-remote-edit
//! surface to the companion-light-edit lane, and the collaboration-follow surface to the
//! companion-review lane. The preview and collaboration-follow surfaces are read-only,
//! only the bounded light-remote-edit surface may write through a host-approved relay,
//! collaboration-follow is confined to a host-revocable scope, the local core stays
//! authoritative so a session handoff never strands user-owned local work, and stale
//! state is always labeled rather than shown as live.
//!
//! Capping the lane, it owns the M5 companion certification in
//! [`certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile`],
//! which certifies each of the eight frozen matrix lanes on every marketed M5
//! profile — local-solo, team-managed, enterprise-managed, browser-companion,
//! mobile-companion, and air-gapped-offline — binding every claim to the frozen
//! matrix as its ceiling so no surface stays greener than the certification. A
//! managed lane is never certified on a profile without a managed plane, a
//! relay-bound companion lane is never certified on an air-gapped profile, the
//! local-first incident and offboarding lanes stay certified even air-gapped, every
//! certified row discloses what stays local and what requires provider or admin
//! continuity, user-owned local work is never stranded, and the closed downgrade
//! rules narrow rather than hide on stale proof, an unverified residency or
//! encryption claim, an unavailable provider or admin plane, or a narrowed upstream
//! matrix lane.
//!
//! Opening a new component lane, it owns the frozen companion-component matrix in
//! [`freeze_the_m5_companion_component_matrix`], which freezes Aureline's reusable
//! companion-client components — the notification row, the mobile review card, the
//! CI-status card, the session-follow tile, the incident-snapshot card, and the
//! desktop-handoff sheet — into one export-safe matrix. Each component binds a stable
//! object identity, a workspace/repo client scope, a freshness class, one controlled
//! disposition vocabulary (`review_only`, `comment_capable`, `desktop_required`,
//! `cached`, `stale`, `policy_blocked`, `handoff_ready`), a severity where it applies,
//! and — for the desktop-handoff sheet — an exact handoff target, so later companion
//! rows reuse the matrix instead of feature-local companion chrome. Hard invariants keep
//! a component from masking its scope or freshness, hiding its companion-versus-desktop
//! capability boundary, inventing an alternate state label, or implying a desktop-required
//! action is companion-safe, and stale state is always labeled rather than shown as live.
//!
//! Implementing the first two of those frozen components, it owns the notification-row
//! and mobile-review-card controls in
//! [`implement_notification_rows_and_mobile_review_cards_with_object_identity_client_scope_freshness_severity_unread_and_desktop_handoff_truth`],
//! which narrows the frozen `notification_row` and `mobile_review_card` into one
//! export-safe packet with two co-equal control vectors so the first glance at a
//! companion event or review item is trustworthy. A notification row's delivery class
//! is derived from its freshness so a stale notification never reads as live, a review
//! card's capability class is derived from its disposition so a desktop-required or
//! policy-blocked review never reads as companion-completable, every quick triage verb
//! lands on one stable object rather than a generic activity page, and every widening
//! verb names one exact desktop-handoff target.
//!
//! Implementing the next two of those frozen components, it owns the CI-status-card and
//! session-follow-tile controls in
//! [`implement_ci_status_cards_and_session_follow_tiles_with_provider_source_run_or_session_identity_stale_state_labeling_and_follow_or_handoff_continuity`],
//! which narrows the frozen `ci_status_card` and `session_follow_tile` into one export-safe
//! packet with two co-equal control vectors so the companion stays honest about live versus
//! stale context. A CI-status card names its provider/source class and its stable run and
//! commit identity, and its result class is derived from the frozen CI status so a stale
//! status never reads as a live pass or fail and a desktop-only rerun is never implied
//! companion-safe. A session-follow tile preserves its presenter and session identity, and
//! its joinability class is derived from the frozen session-follow state so a diverged,
//! stale, host-inactive, or ended session degrades to an explicit read-only or not-joinable
//! state instead of an ambiguous empty card and never offers an ambiguous join into an
//! expired or narrowed session.
//!
//! Implementing the last two of those frozen components, it owns the incident-snapshot-card
//! and desktop-handoff-sheet controls in
//! [`implement_incident_snapshot_cards_and_desktop_handoff_sheets_with_service_run_identity_severity_status_target_identity_auth_tenant_reminder_and_open_on_desktop_truth`],
//! which narrows the frozen `incident_snapshot_card` and `desktop_handoff_sheet` into one
//! export-safe packet with two co-equal control vectors so exact incident and escalation
//! context is preserved when the task exceeds companion scope. An incident-snapshot card names
//! its service/source class, its stable service and run identity, its severity, and its latest
//! status, and its awareness class is derived from that status so a stale incident never reads
//! as a live one and the card stays awareness-only rather than overpromising remediation depth.
//! A desktop-handoff sheet names its target object, its stable target identity, exactly what
//! opens on desktop, and — where relevant — an auth or tenant reminder, and its open class is
//! derived from the frozen handoff target so a sheet with no resolvable target degrades to an
//! explicit not-openable state instead of implying a desktop client will open the intended
//! object without user archaeology.
//!
//! Capping the component lane, it owns the companion degraded-state continuity controls in
//! [`ship_cached_offline_auth_blocked_and_policy_blocked_companion_states_with_summary_first_object_continuity_safe_triage_verbs_and_no_blind_tap_routing`],
//! which governs the degraded states of all six frozen components across the notification and
//! handoff surfaces. Every surface binds one controlled availability state — live, cached,
//! offline, auth-blocked, policy-blocked, loading, or deleted-object — and derives its data-trust
//! class and its next-safe-action from that state, so a cached, offline, or stale surface never
//! reads as live and the copy that tells the user what to do next is never invented per surface.
//! Every surface preserves its object summary, its stable identity, and its safe triage verbs even
//! when full detail cannot be fetched or a publish path is no longer allowed; a surface whose path
//! is broken or over-privileged names an explicit desktop fallback and offers a resolvable desktop
//! handoff rather than routing blindly, and a surface whose object was deleted preserves its
//! summary and stops routing instead of opening a target that no longer exists.
//!
//! Closing the component lane, it owns the shared companion component-consumer adoption lane in
//! [`add_shared_inbox_review_ci_session_follow_incident_advisory_and_browser_or_desktop_handoff_consumers_so_companion_components_keep_scope_freshness_and_desktop_required_language_aligned_across_claimed_m5_profiles`],
//! which proves the six frozen components are reusable by binding every claimed M5 companion
//! consumer — the notification inbox, the review queue, CI status, session follow, incident
//! awareness, the advisory center, Help / docs, the support / export desk, the desktop-handoff
//! surface, and the export packet — to the same canonical component schemas and one shared
//! descriptor vocabulary (object identity, client scope, freshness, capability boundary,
//! severity, handoff target). Each consumer points at the primitive's canonical schema and
//! support-export artifact rather than re-wording those facts in local prose, every one of the
//! six families is adopted by at least two consumers, a cached / stale / desktop-required /
//! policy-blocked rendering auto-narrows the claim behind a self-contained banner naming the
//! exact reason and recovery action, and a stale, desktop-required, or policy-blocked component
//! never masquerades as a live, companion-safe one.
//!
//! Capping the component lane with an accessibility and auto-narrowing certification, it owns the
//! companion component accessibility parity capstone in
//! [`implement_keyboard_screen_reader_share_export_parity_and_automatic_narrowing_when_object_freshness_companion_authority_tenant_scope_or_handoff_validity_is_stale_limited_or_revoked_across_claimed_m5_companion_components`],
//! which certifies — per component family — that companion claims stay keyboard-complete,
//! screen-reader-reachable, and share/export-safe rather than presenting a stale object, a limited
//! companion authority, a narrowed tenant scope, or a revoked handoff as a still live, in-authority,
//! companion-safe surface. Every family reaches the same canonical object identity, client scope,
//! freshness, capability boundary, severity, and handoff target through a non-visual and headless
//! path; a hierarchy-heavy family (the incident-snapshot card) also binds its nested lineage to a
//! flat list / textual path; and when object freshness is stale, companion authority is limited,
//! tenant scope has narrowed, or handoff validity is revoked, the component's claim auto-narrows
//! from live-companion-safe / cached-continuity-safe to a stale-freshness / limited-authority /
//! narrowed-tenant / revoked-handoff projection that discloses the narrowing with a precise trigger
//! and binding dimension and preserves the canonical object lineage. A stale, limited, or revoked
//! state can never keep a live-companion-safe claim.
//!
//! Closing the component lane with a surface certification, it owns the companion component surface
//! certification capstone in
//! [`certify_companion_component_truth_on_every_claimed_m5_companion_and_handoff_surface`],
//! which certifies — per claimed surface — that the shared companion-component truth holds on every
//! claimed M5 companion and handoff surface (the notification inbox, the mobile review queue, the
//! CI-status dashboard, session follow, incident awareness, the desktop handoff, support / export,
//! and Help / docs) across six truth axes — visual, keyboard, screen-reader, share/export,
//! degraded-state, and companion-boundary provenance — and auto-narrows any surface that cannot
//! sustain it. A surface that keeps a live-companion-safe or cached-continuity-safe claim while its
//! object is stale, its companion authority is limited, its tenant scope has narrowed, or its
//! handoff validity is revoked is blocked; a surface that discloses the reduction by narrowing its
//! claim behind a bound reason and a frozen downgrade trigger is honestly yellow and never drops its
//! object-identity / client-scope / freshness / capability / severity / handoff continuity. The
//! always-on share/export axis stays certified on every surface so support and automation
//! reconstruct the same truth from the same object identity, and every certified surface cites the
//! one canonical companion component proof bundle rather than cloning per-surface evidence.

#![doc(html_root_url = "https://docs.rs/aureline-companion/0.0.0")]

pub mod add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont;
pub mod add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets;
pub mod add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio;
pub mod add_shared_inbox_review_ci_session_follow_incident_advisory_and_browser_or_desktop_handoff_consumers_so_companion_components_keep_scope_freshness_and_desktop_required_language_aligned_across_claimed_m5_profiles;
pub mod certify_companion_component_truth_on_every_claimed_m5_companion_and_handoff_surface;
pub mod certify_companion_incident_sync_residency_encryption_and_offboarding_lanes_on_every_marketed_m5_profile;
pub mod companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff;
pub mod freeze_the_m5_companion_component_matrix;
pub mod freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes;
pub mod implement_ci_status_cards_and_session_follow_tiles_with_provider_source_run_or_session_identity_stale_state_labeling_and_follow_or_handoff_continuity;
pub mod implement_incident_snapshot_cards_and_desktop_handoff_sheets_with_service_run_identity_severity_status_target_identity_auth_tenant_reminder_and_open_on_desktop_truth;
pub mod implement_keyboard_screen_reader_share_export_parity_and_automatic_narrowing_when_object_freshness_companion_authority_tenant_scope_or_handoff_validity_is_stale_limited_or_revoked_across_claimed_m5_companion_components;
pub mod implement_notification_rows_and_mobile_review_cards_with_object_identity_client_scope_freshness_severity_unread_and_desktop_handoff_truth;
pub mod implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth;
pub mod implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho;
pub mod ship_cached_offline_auth_blocked_and_policy_blocked_companion_states_with_summary_first_object_continuity_safe_triage_verbs_and_no_blind_tap_routing;
pub mod ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes;
pub mod ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage;
pub mod ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty;
