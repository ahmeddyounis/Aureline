//! Canonical seed builders for the release-note evidence set.
//!
//! These builders are the single producer of the checked-in evidence-set packet, the published
//! inventory, the release-grade parity proof (and its Markdown report), the machine-readable per-note
//! CSV export, and the what's-new / migration drill fixtures. The headless emitter and the inline tests
//! both call them so the in-code packet, the artifacts, and the fixtures never drift.
//!
//! The canonical packet is a representative release that carries **one note per change class** — every
//! behavior-changing or security-sensitive note is evidence-backed, every breaking / migration /
//! admin note links directly to a setting / import / rollback surface, and every what's-new card is
//! active, dismissible, and reopenable from the update center or Help. The drills perturb that set:
//!
//! - the **dismissed** drill dismisses every what's-new card and asserts each stays reopenable;
//! - the **docs-only** drill is a routine docs/compatibility release that leaves every consumer
//!   informational; and
//! - the **security-and-migration** drill is a focused set whose security, migration, and breaking
//!   notes each carry their required advisory / direct-action links and read as action-required.

use super::*;

/// Stable packet id for the canonical release-note evidence set.
pub const M5_RELEASE_NOTE_EVIDENCE_SET_PACKET_ID: &str = "m5-release-note-evidence:stable:0001";

/// Evaluation / mint timestamp for the canonical packet.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

fn both_profiles() -> Vec<DeploymentProfile> {
    vec![DeploymentProfile::Managed, DeploymentProfile::SelfHosted]
}

/// Builds an evidence link from a kind and a target ref slug.
fn link(note_id: &str, kind: EvidenceLinkKind, target_ref: &str) -> EvidenceLink {
    EvidenceLink::new(note_id, kind, target_ref)
}

/// Compactly builds a note from its parts; the what's-new card defaults to active.
fn note(
    note_id: &str,
    change_class: ChangeClass,
    channels: Vec<ChannelScope>,
    affected_artifact_classes: Vec<ArtifactClass>,
    from_version: Option<&str>,
    to_version: Option<&str>,
    evidence_links: Vec<EvidenceLink>,
) -> ReleaseNoteEvidenceRow {
    ReleaseNoteEvidenceRow::new(ReleaseNoteEvidenceRowInput {
        note_id: note_id.to_owned(),
        change_class,
        channels,
        affected_artifact_classes,
        affected_profiles: both_profiles(),
        from_version: from_version.map(str::to_owned),
        to_version: to_version.map(str::to_owned),
        evidence_links,
        whats_new_card: WhatsNewCard::active(note_id),
    })
}

// ---------------------------------------------------------------------------
// Canonical notes — one per change class
// ---------------------------------------------------------------------------

fn docs_only_note() -> ReleaseNoteEvidenceRow {
    let id = "docs_only_quickstart";
    note(
        id,
        ChangeClass::DocsOnly,
        vec![ChannelScope::Stable],
        vec![ArtifactClass::DocsHelpContent],
        None,
        None,
        vec![link(
            id,
            EvidenceLinkKind::DocsPage,
            "docs/help/whats-new/quickstart.md",
        )],
    )
}

fn compatibility_note() -> ReleaseNoteEvidenceRow {
    let id = "compatibility_window_shift";
    note(
        id,
        ChangeClass::Compatibility,
        vec![ChannelScope::Stable],
        vec![ArtifactClass::SchemaContracts],
        Some("12"),
        Some("13"),
        vec![
            link(
                id,
                EvidenceLinkKind::EvidencePacket,
                "artifacts/release/m5-release-note-proof/compatibility_window_shift.evidence",
            ),
            link(
                id,
                EvidenceLinkKind::CertificationDelta,
                "artifacts/release/m5-release-note-proof/compatibility_window_shift.certification-delta",
            ),
        ],
    )
}

fn behavioral_note() -> ReleaseNoteEvidenceRow {
    let id = "behavioral_default_layout";
    note(
        id,
        ChangeClass::Behavioral,
        vec![ChannelScope::Stable],
        vec![ArtifactClass::CoreRuntime, ArtifactClass::Configuration],
        Some("1.8.0"),
        Some("1.9.0"),
        vec![
            link(
                id,
                EvidenceLinkKind::EvidencePacket,
                "artifacts/release/m5-release-note-proof/behavioral_default_layout.evidence",
            ),
            link(
                id,
                EvidenceLinkKind::SettingSurface,
                "app://settings/layout/default",
            ),
        ],
    )
}

fn policy_note() -> ReleaseNoteEvidenceRow {
    let id = "policy_telemetry_consent";
    note(
        id,
        ChangeClass::Policy,
        vec![ChannelScope::Stable],
        vec![ArtifactClass::Configuration],
        None,
        None,
        vec![
            link(
                id,
                EvidenceLinkKind::EvidencePacket,
                "artifacts/release/m5-release-note-proof/policy_telemetry_consent.evidence",
            ),
            link(
                id,
                EvidenceLinkKind::DocsPage,
                "docs/help/whats-new/telemetry-consent.md",
            ),
        ],
    )
}

fn deprecated_note() -> ReleaseNoteEvidenceRow {
    let id = "deprecated_legacy_command";
    note(
        id,
        ChangeClass::Deprecated,
        vec![ChannelScope::Stable],
        vec![ArtifactClass::CoreRuntime],
        Some("1.8.0"),
        Some("1.9.0"),
        vec![
            link(
                id,
                EvidenceLinkKind::MigrationDoc,
                "docs/release/end_of_support_and_migration_contract.md#legacy-command",
            ),
            link(
                id,
                EvidenceLinkKind::DocsPage,
                "docs/help/whats-new/legacy-command.md",
            ),
        ],
    )
}

fn migration_note() -> ReleaseNoteEvidenceRow {
    let id = "migration_workspace_schema";
    note(
        id,
        ChangeClass::MigrationRequired,
        vec![ChannelScope::Stable, ChannelScope::Lts],
        vec![
            ArtifactClass::WorkspaceState,
            ArtifactClass::SchemaContracts,
        ],
        Some("12"),
        Some("13"),
        vec![
            link(
                id,
                EvidenceLinkKind::MigrationDoc,
                "docs/release/end_of_support_and_migration_contract.md#workspace-schema",
            ),
            link(
                id,
                EvidenceLinkKind::ImportSurface,
                "app://migration-assistant/workspace-schema",
            ),
            link(
                id,
                EvidenceLinkKind::RollbackControl,
                "app://update-center/rollback/workspace-schema",
            ),
        ],
    )
}

fn admin_note() -> ReleaseNoteEvidenceRow {
    let id = "admin_policy_bundle";
    note(
        id,
        ChangeClass::AdminActionRequired,
        vec![ChannelScope::Stable],
        vec![ArtifactClass::Configuration],
        None,
        None,
        vec![
            link(
                id,
                EvidenceLinkKind::EvidencePacket,
                "artifacts/release/m5-release-note-proof/admin_policy_bundle.evidence",
            ),
            link(
                id,
                EvidenceLinkKind::SettingSurface,
                "app://admin/policy-bundle",
            ),
        ],
    )
}

fn security_note() -> ReleaseNoteEvidenceRow {
    let id = "security_dependency_advisory";
    note(
        id,
        ChangeClass::Security,
        vec![ChannelScope::Stable, ChannelScope::Lts],
        vec![ArtifactClass::CoreRuntime],
        Some("1.8.0"),
        Some("1.9.0"),
        vec![
            link(
                id,
                EvidenceLinkKind::SecurityAdvisory,
                "docs/release/finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills.md#dependency-advisory",
            ),
            link(id, EvidenceLinkKind::RollbackControl, "app://update-center/rollback/dependency-advisory"),
        ],
    )
}

fn breaking_note() -> ReleaseNoteEvidenceRow {
    let id = "breaking_extension_api";
    note(
        id,
        ChangeClass::Breaking,
        vec![ChannelScope::Stable, ChannelScope::Lts],
        vec![
            ArtifactClass::ExtensionPacks,
            ArtifactClass::SchemaContracts,
        ],
        Some("3.2.1"),
        Some("4.0.0"),
        vec![
            link(
                id,
                EvidenceLinkKind::EvidencePacket,
                "artifacts/release/m5-release-note-proof/breaking_extension_api.evidence",
            ),
            link(
                id,
                EvidenceLinkKind::MigrationDoc,
                "docs/release/end_of_support_and_migration_contract.md#extension-api",
            ),
            link(
                id,
                EvidenceLinkKind::SettingSurface,
                "app://settings/extensions/api-compat",
            ),
            link(
                id,
                EvidenceLinkKind::RollbackControl,
                "app://update-center/rollback/extension-api",
            ),
        ],
    )
}

/// The canonical, representative note set: one note per change class.
fn canonical_notes() -> Vec<ReleaseNoteEvidenceRow> {
    vec![
        docs_only_note(),
        compatibility_note(),
        behavioral_note(),
        policy_note(),
        deprecated_note(),
        migration_note(),
        admin_note(),
        security_note(),
        breaking_note(),
    ]
}

/// The claimed consumer rows. Every consumer reads every published note so all of them surface one
/// vocabulary and one schema; their readiness and gaps are derived from the rows.
fn consumer_rows(note_ids: &[String]) -> Vec<ReleaseNoteConsumerRow> {
    ReleaseNoteConsumer::ALL
        .iter()
        .map(|consumer| ReleaseNoteConsumerRow::new(*consumer, note_ids))
        .collect()
}

/// The canonical release target.
fn canonical_target() -> ReleaseNoteTarget {
    ReleaseNoteTarget {
        channels: vec![ChannelScope::Stable, ChannelScope::Lts],
        profiles: both_profiles(),
        current_version: "1.8.0".to_owned(),
        target_version: "1.9.0".to_owned(),
    }
}

/// Assembles a packet from the given notes.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    notes: Vec<ReleaseNoteEvidenceRow>,
) -> ReleaseNoteEvidenceSet {
    let note_ids: Vec<String> = notes.iter().map(|n| n.note_id.clone()).collect();
    ReleaseNoteEvidenceSet::new(ReleaseNoteEvidenceSetInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        target: canonical_target(),
        notes,
        consumers: consumer_rows(&note_ids),
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, representative release-note evidence set: one evidence-backed, action-linked note per
/// change class, every what's-new card active and reopenable.
pub fn seeded_m5_release_note_evidence_set() -> ReleaseNoteEvidenceSet {
    assemble_packet(
        M5_RELEASE_NOTE_EVIDENCE_SET_PACKET_ID,
        "Aureline M5 release-note evidence",
        canonical_notes(),
    )
}

/// Drill: every what's-new card is dismissed, but each stays dismissible and reopenable from the update
/// center and Help — the reopenability acceptance criterion.
pub fn seeded_m5_release_note_evidence_set_dismissed() -> ReleaseNoteEvidenceSet {
    let notes: Vec<ReleaseNoteEvidenceRow> = canonical_notes()
        .into_iter()
        .map(|mut n| {
            n.whats_new_card = WhatsNewCard::dismissed(&n.note_id);
            n.recompute();
            n
        })
        .collect();
    assemble_packet(
        "m5-release-note-evidence:drill-dismissed:0001",
        "Aureline M5 release-note evidence — dismissed/reopenable drill",
        notes,
    )
}

/// Drill: a routine docs / compatibility release. Every note is informational, so every consumer reads
/// the set as informational and no action is required.
pub fn seeded_m5_release_note_evidence_set_docs_only() -> ReleaseNoteEvidenceSet {
    assemble_packet(
        "m5-release-note-evidence:drill-docs-only:0001",
        "Aureline M5 release-note evidence — docs-only drill",
        vec![docs_only_note(), compatibility_note()],
    )
}

/// Drill: a focused security / migration release. The security note carries an advisory, the migration
/// and breaking notes carry direct setting / import / rollback links, and all read as action-required —
/// the evidence-backed and direct-link acceptance criteria.
pub fn seeded_m5_release_note_evidence_set_security_and_migration() -> ReleaseNoteEvidenceSet {
    assemble_packet(
        "m5-release-note-evidence:drill-security-migration:0001",
        "Aureline M5 release-note evidence — security & migration drill",
        vec![security_note(), migration_note(), breaking_note()],
    )
}
