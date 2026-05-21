//! Beta coverage for the docs/help browser **content** surface.
//!
//! The docs browser already projects source / version / freshness /
//! client-scope / browser-handoff rows from a docs pack, and the cross-surface
//! `truth_wiring` module already binds the `DocsBrowser` surface to the same
//! release truth the migration center, Help/About, and service-health surfaces
//! read. This drill promotes the docs **content** rows themselves to beta: it
//! replays a frozen corpus of [`DocsBrowserRowCard`] projections from
//! `fixtures/help/docs_browser_content_beta/` and proves each docs entry is
//! wired to that release truth instead of asserting a private, more optimistic
//! story —
//!
//! 1. the entry **cites the current release truth** — the same claim manifest /
//!    compatibility report and the `docs_site` channel the `DocsBrowser` surface
//!    binding resolves to, plus a docs claim row the binding actually selected;
//! 2. the entry **does not over-claim** — its freshness badge is never fresher
//!    than the release truth's docs freshness badge, and only an exact build
//!    match against the running build counts as version-wired;
//! 3. degraded / stale entries are **labeled** with a snapshot-age label rather
//!    than silently downgrading.
//!
//! The corpus reuses the existing [`DocsBrowserRowCard`] type verbatim and the
//! live binding from [`seeded_truth_wiring_report`], so the docs content surface
//! shares one release truth instead of a divergent label set.
//! `ci/check_beta_docs_browser_content.py` re-derives the label vocabularies and
//! the release-truth binding from crate source and the release artifacts and
//! fails closed if either drifts.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use aureline_shell::docs_browser::state::DocsBrowserRowCard;
use aureline_shell::docs_browser::truth_wiring::{
    seeded_truth_wiring_report, ServiceContractState, SurfaceTruthBinding, TruthSurfaceClass,
};
use aureline_shell::service_health::{
    M3ClaimManifestSnapshot, ManifestChannelId, ServiceHealthBetaSurface,
};

const CORPUS_DIR: &str = "fixtures/help/docs_browser_content_beta";

/// Closed contract-state vocabulary the docs content surface shares with the
/// sibling public-truth surfaces. Mirrors [`ServiceContractState`]; the test
/// asserts the manifest mirror equals this set so the docs surface cannot fork
/// the boundary/visibility vocabulary.
const CONTRACT_STATES: [ServiceContractState; 7] = [
    ServiceContractState::Ready,
    ServiceContractState::Degraded,
    ServiceContractState::LocalOnly,
    ServiceContractState::Stale,
    ServiceContractState::ContractMismatch,
    ServiceContractState::PolicyBlocked,
    ServiceContractState::Unavailable,
];

fn repo_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../{relative}"))
}

#[derive(Debug, Clone, Deserialize)]
struct Manifest {
    record_kind: String,
    schema_version: u32,
    release_truth_binding: ReleaseTruthBinding,
    freshness_class_vocabulary: Vec<String>,
    version_match_state_vocabulary: Vec<String>,
    source_class_vocabulary: Vec<String>,
    identity_mode_vocabulary: Vec<String>,
    trust_state_vocabulary: Vec<String>,
    contract_state_vocabulary: Vec<String>,
    required_contract_states: Vec<String>,
    cases: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct ReleaseTruthBinding {
    surface_class: String,
    claim_manifest_ref: String,
    compatibility_report_ref: String,
    required_channel_id: String,
    running_build_identity_ref: String,
    release_freshness_badge: String,
    service_contract_state: String,
    freshness_state_tokens: Vec<String>,
    evidence_stale: bool,
    claim_downgraded: bool,
    docs_claim_row_refs: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct Case {
    record_kind: String,
    schema_version: u32,
    case_id: String,
    wiring: Wiring,
    expect: Expect,
    row_card: DocsBrowserRowCard,
}

#[derive(Debug, Clone, Deserialize)]
struct Wiring {
    claim_manifest_ref: String,
    compatibility_report_ref: String,
    required_channel_id: String,
    docs_claim_row_ref: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Expect {
    contract_state: String,
    freshness_wired: bool,
    freshness_degraded: bool,
    version_wired: bool,
    freshness_label_present: bool,
}

/// One thing a docs content entry can get wrong about release truth.
#[derive(Debug, Clone, PartialEq, Eq)]
enum EntryViolation {
    WrongClaimManifestRef,
    WrongCompatibilityReportRef,
    WrongChannel,
    UnknownDocsClaimRow,
    UnknownFreshnessClass,
    FreshnessOverClaimsReleaseTruth,
    FreshnessWiredMismatch,
    FreshnessDegradedMismatch,
    DegradedEntryMissingFreshnessLabel,
    VersionWiredMismatch,
    ExactVersionWrongRunningBuild,
    UnknownVersionState,
    UnknownSourceClass,
    UnknownIdentityMode,
    UnknownTrustState,
    UnknownContractState,
    ContractStateMismatch,
    FreshnessLabelPresentMismatch,
}

fn load_manifest() -> Manifest {
    let path = repo_path(&format!("{CORPUS_DIR}/manifest.json"));
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    let manifest: Manifest = serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()));
    assert_eq!(
        manifest.record_kind, "docs_browser_content_beta_manifest",
        "unexpected manifest record_kind"
    );
    assert_eq!(
        manifest.schema_version, 1,
        "unexpected manifest schema_version"
    );
    manifest
}

fn load_case(name: &str) -> Case {
    let path = repo_path(&format!("{CORPUS_DIR}/{name}"));
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    let case: Case = serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()));
    assert_eq!(
        case.record_kind, "docs_browser_content_beta_case",
        "unexpected record_kind in {name}"
    );
    assert_eq!(
        case.schema_version, 1,
        "unexpected schema_version in {name}"
    );
    case
}

fn load_cases(manifest: &Manifest) -> Vec<Case> {
    manifest.cases.iter().map(|name| load_case(name)).collect()
}

fn docs_binding() -> SurfaceTruthBinding {
    let report = seeded_truth_wiring_report();
    report
        .binding_for(TruthSurfaceClass::DocsBrowser)
        .expect("release truth wiring report must carry a docs browser binding")
        .clone()
}

/// Confidence ordering over the docs freshness badge vocabulary. A docs entry
/// may match or fall below the release-truth badge, never exceed it.
fn freshness_rank(token: &str) -> Option<u8> {
    Some(match token {
        "authoritative_live" => 4,
        "warm_cached" => 3,
        "degraded_cached" => 2,
        "stale" => 1,
        "unverified" => 0,
        _ => return None,
    })
}

/// The per-entry contract state, derived from the entry's freshness badge and
/// version match state. Stale freshness is stale; any other degraded freshness
/// or any non-exact build match degrades; an exact, warm/authoritative entry is
/// ready. An unknown freshness badge is a contract mismatch.
fn derive_entry_contract_state(card: &DocsBrowserRowCard) -> &'static str {
    match card.freshness_row.class_token.as_str() {
        "stale" => "stale",
        "degraded_cached" | "unverified" => "degraded",
        "authoritative_live" | "warm_cached" => {
            if card.version_row.state_token == "exact_build_match" {
                "ready"
            } else {
                "degraded"
            }
        }
        _ => "contract_mismatch",
    }
}

fn snapshot_label_present(card: &DocsBrowserRowCard) -> bool {
    card.source_row
        .snapshot_age_label
        .as_deref()
        .map(|label| !label.trim().is_empty())
        .unwrap_or(false)
}

/// Validate one docs content entry against the live release-truth binding and
/// the shared corpus vocabularies. Returns every violation found so the
/// negative drills can assert on the specific failure.
fn validate_entry(
    case: &Case,
    binding: &SurfaceTruthBinding,
    manifest: &Manifest,
) -> Vec<EntryViolation> {
    let mut out = Vec::new();
    let card = &case.row_card;

    if case.wiring.claim_manifest_ref != binding.claim_manifest_ref {
        out.push(EntryViolation::WrongClaimManifestRef);
    }
    if case.wiring.compatibility_report_ref != binding.compatibility_report_ref {
        out.push(EntryViolation::WrongCompatibilityReportRef);
    }
    if case.wiring.required_channel_id != binding.required_channel_id {
        out.push(EntryViolation::WrongChannel);
    }
    if !binding
        .claim_row_ids
        .iter()
        .any(|id| id == &case.wiring.docs_claim_row_ref)
    {
        out.push(EntryViolation::UnknownDocsClaimRow);
    }

    // Freshness wiring: an entry may never claim a fresher badge than release
    // truth provides for the docs surface.
    let release_rank = freshness_rank(&manifest.release_truth_binding.release_freshness_badge)
        .expect("release freshness badge must be a known freshness class");
    let entry_rank = match freshness_rank(&card.freshness_row.class_token) {
        Some(rank) => rank,
        None => {
            out.push(EntryViolation::UnknownFreshnessClass);
            release_rank
        }
    };
    if entry_rank > release_rank {
        out.push(EntryViolation::FreshnessOverClaimsReleaseTruth);
    }
    let freshness_wired = entry_rank == release_rank;
    if freshness_wired != case.expect.freshness_wired {
        out.push(EntryViolation::FreshnessWiredMismatch);
    }
    if card.freshness_row.degraded != case.expect.freshness_degraded {
        out.push(EntryViolation::FreshnessDegradedMismatch);
    }
    // A degraded entry must carry an explicit freshness (snapshot-age) label.
    if card.freshness_row.degraded && !snapshot_label_present(card) {
        out.push(EntryViolation::DegradedEntryMissingFreshnessLabel);
    }

    // Version wiring: only an exact build match counts as wired, and it must
    // cite the running build identity the release truth binds to.
    let version_wired = card.version_row.state_token == "exact_build_match";
    if version_wired != case.expect.version_wired {
        out.push(EntryViolation::VersionWiredMismatch);
    }
    if version_wired
        && card.version_row.running_build_identity_ref
            != manifest.release_truth_binding.running_build_identity_ref
    {
        out.push(EntryViolation::ExactVersionWrongRunningBuild);
    }

    // Boundary / visibility vocabulary parity with the shared closed sets.
    if !manifest
        .version_match_state_vocabulary
        .contains(&card.version_row.state_token)
    {
        out.push(EntryViolation::UnknownVersionState);
    }
    if !manifest
        .source_class_vocabulary
        .contains(&card.source_row.class_token)
    {
        out.push(EntryViolation::UnknownSourceClass);
    }
    if !manifest
        .identity_mode_vocabulary
        .contains(&card.client_scope_row.identity_mode_token)
    {
        out.push(EntryViolation::UnknownIdentityMode);
    }
    if !manifest
        .trust_state_vocabulary
        .contains(&card.client_scope_row.trust_state_token)
    {
        out.push(EntryViolation::UnknownTrustState);
    }

    // Contract state: drawn from the shared vocabulary and equal to the value
    // the entry's freshness/version derive.
    if !manifest
        .contract_state_vocabulary
        .contains(&case.expect.contract_state)
    {
        out.push(EntryViolation::UnknownContractState);
    }
    if derive_entry_contract_state(card) != case.expect.contract_state {
        out.push(EntryViolation::ContractStateMismatch);
    }

    if snapshot_label_present(card) != case.expect.freshness_label_present {
        out.push(EntryViolation::FreshnessLabelPresentMismatch);
    }

    out
}

// --------------------------------------------------------------------------- //
// Tests
// --------------------------------------------------------------------------- //

#[test]
fn corpus_binding_matches_live_release_truth() {
    let manifest = load_manifest();
    let declared = &manifest.release_truth_binding;
    let binding = docs_binding();

    assert_eq!(
        declared.surface_class, "docs_browser",
        "corpus binding must describe the docs browser surface"
    );
    assert_eq!(
        declared.claim_manifest_ref, binding.claim_manifest_ref,
        "claim manifest ref drifted from the live docs binding"
    );
    assert_eq!(
        declared.compatibility_report_ref, binding.compatibility_report_ref,
        "compatibility report ref drifted from the live docs binding"
    );
    assert_eq!(
        declared.required_channel_id, binding.required_channel_id,
        "required channel drifted from the live docs binding"
    );
    assert_eq!(
        declared.service_contract_state, binding.service_contract_state_token,
        "contract state drifted from the live docs binding"
    );
    assert_eq!(
        declared.evidence_stale, binding.evidence_stale,
        "evidence-stale flag drifted from the live docs binding"
    );
    assert_eq!(
        declared.claim_downgraded, binding.claim_downgraded,
        "claim-downgraded flag drifted from the live docs binding"
    );
    assert_eq!(
        declared
            .freshness_state_tokens
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>(),
        binding
            .freshness_state_tokens
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>(),
        "freshness state tokens drifted from the live docs binding"
    );
    assert_eq!(
        declared
            .docs_claim_row_refs
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>(),
        binding
            .claim_row_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>(),
        "selected docs claim rows drifted from the live docs binding"
    );

    // The release-truth docs freshness badge equals what the service-health
    // beta surface projects for the docs rows the binding selected, so the
    // corpus freshness ceiling tracks the live manifest.
    let snapshot = M3ClaimManifestSnapshot::load_from_path(repo_path(&declared.claim_manifest_ref))
        .expect("claim manifest must load");
    let surface = ServiceHealthBetaSurface::project_at_manifest_as_of(&snapshot);
    let docs_rows: Vec<_> = surface
        .rows
        .iter()
        .filter(|row| {
            row.claim_family == "docs_freshness"
                && row.channel_projections.iter().any(|projection| {
                    projection.channel_class == Some(ManifestChannelId::DocsSite)
                        && projection.binding_status == "required"
                })
        })
        .collect();
    assert!(
        !docs_rows.is_empty(),
        "service-health surface must project the docs freshness rows"
    );
    for row in &docs_rows {
        assert_eq!(
            row.freshness.badge_token, declared.release_freshness_badge,
            "docs row {} freshness badge drifted from the corpus binding",
            row.row_id
        );
    }
    let projected_refs: BTreeSet<String> = docs_rows.iter().map(|row| row.row_id.clone()).collect();
    assert_eq!(
        projected_refs,
        declared
            .docs_claim_row_refs
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>(),
        "service-health docs rows disagree with the corpus binding selection"
    );
}

#[test]
fn every_docs_entry_is_wired_to_release_truth() {
    let manifest = load_manifest();
    let binding = docs_binding();
    let cases = load_cases(&manifest);
    assert!(!cases.is_empty(), "corpus must declare at least one case");

    for case in &cases {
        let violations = validate_entry(case, &binding, &manifest);
        assert!(
            violations.is_empty(),
            "docs entry {} must be wired to release truth, found {violations:?}",
            case.case_id
        );

        // The render lines stay explicit about every truth axis, so the chrome
        // never blanks a degraded entry's provenance.
        let lines = case.row_card.render_lines();
        for prefix in ["Source: ", "Version: ", "Freshness: ", "Client scope: "] {
            assert!(
                lines.iter().any(|line| line.starts_with(prefix)),
                "entry {} must render an explicit {prefix:?} line: {lines:#?}",
                case.case_id
            );
        }
    }
}

#[test]
fn corpus_covers_ready_stale_and_degraded_contract_states() {
    let manifest = load_manifest();
    let cases = load_cases(&manifest);
    let covered: BTreeSet<&str> = cases
        .iter()
        .map(|case| case.expect.contract_state.as_str())
        .collect();
    for required in &manifest.required_contract_states {
        assert!(
            covered.contains(required.as_str()),
            "corpus is missing a case for required contract state {required}"
        );
    }
    for required in ["ready", "stale", "degraded"] {
        assert!(
            covered.contains(required),
            "corpus must cover contract state {required}"
        );
    }
}

#[test]
fn contract_state_vocabulary_matches_service_contract_state() {
    let manifest = load_manifest();
    let source: BTreeSet<String> = CONTRACT_STATES
        .iter()
        .map(|state| state.as_str().to_owned())
        .collect();
    let mirror: BTreeSet<String> = manifest.contract_state_vocabulary.iter().cloned().collect();
    assert_eq!(
        mirror, source,
        "manifest contract_state_vocabulary drifted from ServiceContractState"
    );
    // The freshness ceiling badge is itself a real freshness class.
    assert!(
        manifest
            .freshness_class_vocabulary
            .contains(&manifest.release_truth_binding.release_freshness_badge),
        "release freshness badge must be a known freshness class"
    );
}

#[test]
fn docs_entry_overclaiming_release_truth_freshness_fails() {
    let manifest = load_manifest();
    let binding = docs_binding();
    let mut case = load_case("project_docs_release_current.json");

    // Claim freshness fresher than the release-truth docs badge (warm cached).
    case.row_card.freshness_row.class_token = "authoritative_live".to_owned();
    case.row_card.freshness_row.label = "Authoritative (live)".to_owned();
    case.row_card.freshness_row.degraded = false;

    let violations = validate_entry(&case, &binding, &manifest);
    assert!(
        violations.contains(&EntryViolation::FreshnessOverClaimsReleaseTruth),
        "a docs entry fresher than release truth must fail; got {violations:?}"
    );
}

#[test]
fn stale_docs_entry_without_a_freshness_label_fails() {
    let manifest = load_manifest();
    let binding = docs_binding();
    let mut case = load_case("mirrored_docs_stale_labeled.json");

    // Drop the snapshot-age label from the stale entry — exactly the silent
    // downgrade the docs surface must refuse.
    case.row_card.source_row.snapshot_age_label = None;

    let violations = validate_entry(&case, &binding, &manifest);
    assert!(
        violations.contains(&EntryViolation::DegradedEntryMissingFreshnessLabel),
        "a stale entry without a freshness label must fail; got {violations:?}"
    );
}
