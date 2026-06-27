//! Canonical seed builders for the M5 runbook source register.
//!
//! These builders are the single producer of the checked-in source register, the
//! published inventory, the Markdown proof, and the source-descriptor fixtures.
//! The headless emitter and the inline tests both call them so the in-code
//! register, the artifacts, and the fixtures never drift. Each register derives
//! every source's effective authority posture, badge, and conformance review from
//! the same declared truth, so a reference-only browser doc can never present as a
//! first-party executable runbook, and a stale source narrows back to
//! reference-only without manual edits.

use super::*;

/// Stable register id for the canonical source register.
pub const M5_RUNBOOK_SOURCE_REGISTER_ID: &str = "m5-runbook-source-register:stable:0001";

/// Evaluation / mint timestamp for the canonical register.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

fn signer(
    signer_ref: &str,
    kind: RunbookProvenanceKind,
    signature_verified: bool,
    version: &str,
) -> RunbookSourceSigner {
    RunbookSourceSigner {
        signer_ref: signer_ref.to_owned(),
        provenance_kind: kind,
        signature_verified,
        attested_version: version.to_owned(),
    }
}

fn freshness(
    fresh_within_days: u32,
    stale_after_days: u32,
    days_since_verification: u32,
    provenance_verified: bool,
) -> FreshnessWindow {
    FreshnessWindow {
        fresh_within_days,
        stale_after_days,
        days_since_verification,
        provenance_verified,
    }
}

fn export_rights(exportable: bool, include_signer_in_export: bool) -> RunbookSourceExportRights {
    RunbookSourceExportRights {
        exportable,
        redaction_class: REDACTION_CLASS.to_owned(),
        include_signer_in_export,
        raw_body_exportable: false,
    }
}

/// Builds one governed runbook source and derives its effective posture.
#[allow(clippy::too_many_arguments)]
fn source(
    source_id: &str,
    label: &str,
    provenance_class: RunbookSourceProvenance,
    version_ref: &str,
    signer: RunbookSourceSigner,
    freshness: FreshnessWindow,
    owning_scope: &str,
    owner_role: &str,
    promotion: Option<RunbookSourcePromotion>,
    export_rights: RunbookSourceExportRights,
) -> GovernedRunbookSource {
    let mut src = GovernedRunbookSource {
        record_kind: M5_RUNBOOK_SOURCE_RECORD_KIND.to_owned(),
        schema_version: M5_RUNBOOK_SOURCE_SCHEMA_VERSION,
        source_id: source_id.to_owned(),
        source_label: label.to_owned(),
        provenance_class,
        version_ref: version_ref.to_owned(),
        signer,
        freshness,
        owning_scope: owning_scope.to_owned(),
        owner_role: owner_role.to_owned(),
        declared_authority_posture: provenance_class.default_posture(),
        effective_authority_posture: provenance_class.default_posture(),
        promotion,
        export_rights,
        detail_message_id: format!(
            "{}source.{}",
            M5_RUNBOOK_SOURCE_MESSAGE_ID_PREFIX, source_id
        ),
    };
    src.recompute();
    src
}

/// A first-party repo-local source: authored and signed in-repo, authoritative.
fn repo_local() -> GovernedRunbookSource {
    source(
        "src:repo-pipeline-restart",
        "First-party pipeline restart runbook",
        RunbookSourceProvenance::RepoLocal,
        "pipeline-restart@v7",
        signer(
            "release-signing-key:runbooks",
            RunbookProvenanceKind::SignedFirstParty,
            true,
            "pipeline-restart@v7",
        ),
        freshness(30, 120, 9, true),
        "org:aureline/team:incident-engineering",
        "runbook_authoring_owner",
        None,
        export_rights(true, true),
    )
}

/// A mirrored docs-pack source: a verified mirror of an upstream authoritative pack.
fn mirrored_docs_pack() -> GovernedRunbookSource {
    source(
        "src:mirror-observability-pack",
        "Mirrored observability runbook pack",
        RunbookSourceProvenance::MirroredDocsPack,
        "observability-pack@2026.05",
        signer(
            "mirror-digest:sha256:obs-pack",
            RunbookProvenanceKind::MirrorDigest,
            true,
            "observability-pack@2026.05",
        ),
        freshness(30, 90, 21, true),
        "org:aureline/team:platform-docs",
        "docs_help_owner",
        None,
        export_rights(true, true),
    )
}

/// A managed-catalog source: published through a managed catalog under a manifest.
fn managed_catalog() -> GovernedRunbookSource {
    source(
        "src:catalog-failover",
        "Managed catalog failover runbook",
        RunbookSourceProvenance::ManagedCatalog,
        "failover@catalog-2026.06",
        signer(
            "catalog-manifest:dr-catalog",
            RunbookProvenanceKind::CatalogManifest,
            true,
            "failover@catalog-2026.06",
        ),
        freshness(45, 120, 12, true),
        "org:aureline/catalog:disaster-recovery",
        "catalog_governance_owner",
        None,
        export_rights(true, true),
    )
}

/// A browser-reference source with no promotion: reference-only, never executable.
fn browser_reference_unpromoted() -> GovernedRunbookSource {
    source(
        "src:browser-vendor-scaling",
        "Vendor console scaling reference (browser capture)",
        RunbookSourceProvenance::BrowserReference,
        "vendor-scaling-doc@captured-2026.06",
        signer(
            "browser-capture:vendor-scaling",
            RunbookProvenanceKind::BrowserCapture,
            false,
            "vendor-scaling-doc@captured-2026.06",
        ),
        // The capture was re-confirmed current, but it carries no signature.
        freshness(30, 90, 4, true),
        "org:aureline/team:incident-engineering",
        "control_plane_boundary_owner",
        None,
        export_rights(true, false),
    )
}

/// A browser-reference source promoted into authoritative posture by a governed
/// first-party source. The promotion is what lets its step set execute.
fn browser_reference_promoted() -> GovernedRunbookSource {
    source(
        "src:browser-promoted-dr",
        "Vendor DR steps, promoted to first-party authority",
        RunbookSourceProvenance::BrowserReference,
        "vendor-dr-doc@captured-2026.06",
        signer(
            "browser-capture:vendor-dr",
            RunbookProvenanceKind::BrowserCapture,
            false,
            "vendor-dr-doc@captured-2026.06",
        ),
        freshness(30, 90, 6, true),
        "org:aureline/team:incident-engineering",
        "runbook_authoring_owner",
        Some(RunbookSourcePromotion {
            promotion_id: "promo:browser-dr-into-first-party".to_owned(),
            promoted_by_source_id: "src:repo-pipeline-restart".to_owned(),
            promotes_to: RunbookAuthorityPosture::Authoritative,
            approver_role: "runbook_authoring_owner".to_owned(),
            rationale_message_id: format!(
                "{}promotion.browser-promoted-dr",
                M5_RUNBOOK_SOURCE_MESSAGE_ID_PREFIX
            ),
        }),
        export_rights(true, false),
    )
}

/// The checked-in governed runbook sources demonstrating all four source classes.
pub fn seeded_runbook_sources() -> Vec<GovernedRunbookSource> {
    vec![
        repo_local(),
        mirrored_docs_pack(),
        managed_catalog(),
        browser_reference_unpromoted(),
        browser_reference_promoted(),
    ]
}

fn assemble(
    register_id: &str,
    report_label: &str,
    sources: Vec<GovernedRunbookSource>,
) -> M5RunbookSourceRegister {
    M5RunbookSourceRegister::new(M5RunbookSourceRegisterInput {
        register_id: register_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        sources,
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical runbook source register: one source per class, all fresh, with a
/// promoted and an unpromoted browser reference.
pub fn seeded_m5_runbook_source_register() -> M5RunbookSourceRegister {
    assemble(
        M5_RUNBOOK_SOURCE_REGISTER_ID,
        "M5 runbook source register",
        seeded_runbook_sources(),
    )
}

/// Drill: the mirrored docs-pack has gone stale (verified too long ago), so it
/// auto-narrows from `mirrored` back to `reference_only` and is no longer
/// executable — without any other source changing.
pub fn seeded_m5_runbook_source_register_stale_mirror_narrowed() -> M5RunbookSourceRegister {
    let mut sources = seeded_runbook_sources();
    for src in &mut sources {
        if src.source_id == "src:mirror-observability-pack" {
            // Push verification age past the stale threshold and re-derive.
            src.freshness.days_since_verification = src.freshness.stale_after_days + 30;
            src.recompute();
        }
    }
    assemble(
        "m5-runbook-source-register:drill-stale-mirror:0001",
        "M5 runbook source register — stale-mirror drill",
        sources,
    )
}
