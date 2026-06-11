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

#![doc(html_root_url = "https://docs.rs/aureline-companion/0.0.0")]

pub mod add_customer_managed_key_or_storage_selection_flows_region_or_residency_cues_and_degraded_managed_service_cont;
pub mod add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets;
pub mod companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff;
pub mod freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes;
pub mod implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth;
pub mod implement_usage_export_and_offboarding_packages_grace_window_state_org_switch_semantics_and_deletion_export_ho;
pub mod ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes;
pub mod ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage;
pub mod ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty;
