//! Seeded classroom-role-and-authority corpus, support export, and validation.
//!
//! Each case is one [`ClassroomProfile`] — a session's members and exercise
//! packets. The checked-in fixtures under
//! `fixtures/presentation/classroom-role-and-authority/` are a literal projection
//! of [`seeded_classroom_role_corpus`], so the JSON cannot drift from the Rust
//! types.
//!
//! The corpus deliberately covers a teaching session that expresses all five
//! roles with authority held only through separate external grants, a constrained-
//! client session where a moderator and a participant join as note-takers (and an
//! observer stays observing) rather than seeing broken controls, and an
//! exercise-packet session whose packets stay command-backed, target-constrained,
//! and authority-bounded — so role/authority separation, honest degradation, and
//! packet boundaries are proven across scenarios rather than asserted.

use serde::{Deserialize, Serialize};

use crate::presentation_mode::{
    PRESENTATION_MODE_BETA_SCHEMA_VERSION, PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF,
};

use super::roles::{
    project_classroom_profile, ClassroomMember, ClassroomProfile, ClassroomSupportExport,
    ClientClass, ExercisePacket, ExerciseTarget, ExerciseTargetKind, ExpectedAction,
    ProductAuthorityAttribution, TeachingRole,
};

/// Stable record kind for [`ClassroomRoleCase`] payloads.
pub const CLASSROOM_ROLE_CASE_RECORD_KIND: &str = "presentation_classroom_role_case";

/// Stable record kind for [`ClassroomRoleCorpus`] payloads.
pub const CLASSROOM_ROLE_CORPUS_RECORD_KIND: &str = "presentation_classroom_role_corpus";

/// One seeded scenario: a classroom profile plus its scenario label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassroomRoleCase {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable case id.
    pub case_id: String,
    /// Human-readable scenario label.
    pub scenario_label: String,
    /// The classroom profile for this scenario.
    pub profile: ClassroomProfile,
}

/// Aggregate coverage summary for the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassroomRoleSummary {
    /// Number of cases.
    pub case_count: u32,
    /// Total members across the corpus.
    pub member_count: u32,
    /// Total exercise packets across the corpus.
    pub packet_count: u32,
    /// Distinct roles covered across the corpus.
    pub roles_covered: Vec<TeachingRole>,
    /// Distinct client classes covered across the corpus.
    pub client_classes_covered: Vec<ClientClass>,
    /// True when no member's role grants any product authority.
    pub no_role_grants_product_authority: bool,
    /// True when no member's product authority is sourced from a classroom role.
    pub all_authority_externally_sourced: bool,
    /// True when every constrained-client member joins safely.
    pub all_constrained_clients_join_safely: bool,
    /// True when every exercise packet stays inside its authority boundary.
    pub all_packets_authority_bounded: bool,
    /// True when at least one case demonstrates a constrained client.
    pub constrained_client_demonstrated: bool,
    /// True when at least one case demonstrates an exercise packet.
    pub exercise_packet_demonstrated: bool,
}

/// The full seeded classroom-role-and-authority corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassroomRoleCorpus {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Mint timestamp.
    pub generated_at: String,
    /// Coverage summary.
    pub summary: ClassroomRoleSummary,
    /// Per-scenario cases.
    pub cases: Vec<ClassroomRoleCase>,
}

impl ClassroomRoleCorpus {
    /// Every profile across the corpus, in case order.
    pub fn all_profiles(&self) -> Vec<&ClassroomProfile> {
        self.cases.iter().map(|case| &case.profile).collect()
    }
}

/// Errors emitted by [`validate_classroom_role_corpus`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassroomRoleCorpusError {
    /// The corpus carried the wrong record kind or schema version.
    MalformedCorpus,
    /// A case carried the wrong record kind or schema version.
    MalformedCase {
        /// The offending case id.
        case_id: String,
    },
    /// A case's profile failed validation.
    ProfileInvalid {
        /// The offending case id.
        case_id: String,
        /// Stable tokens for the profile violations.
        violations: Vec<String>,
    },
    /// The summary did not match the cases it claims to summarize.
    SummaryMismatch,
    /// No case demonstrated a constrained client.
    ConstrainedClientNotDemonstrated,
    /// No case demonstrated an exercise packet.
    ExercisePacketNotDemonstrated,
}

/// Validate the seeded classroom-role-and-authority corpus.
pub fn validate_classroom_role_corpus(
    corpus: &ClassroomRoleCorpus,
) -> Result<(), ClassroomRoleCorpusError> {
    if corpus.record_kind != CLASSROOM_ROLE_CORPUS_RECORD_KIND
        || corpus.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
    {
        return Err(ClassroomRoleCorpusError::MalformedCorpus);
    }

    for case in &corpus.cases {
        if case.record_kind != CLASSROOM_ROLE_CASE_RECORD_KIND
            || case.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
        {
            return Err(ClassroomRoleCorpusError::MalformedCase {
                case_id: case.case_id.clone(),
            });
        }
        let violations = case.profile.validate();
        if !violations.is_empty() {
            return Err(ClassroomRoleCorpusError::ProfileInvalid {
                case_id: case.case_id.clone(),
                violations: violations.iter().map(|v| v.as_str().to_owned()).collect(),
            });
        }
    }

    let expected = summarize(&corpus.cases);
    if expected != corpus.summary {
        return Err(ClassroomRoleCorpusError::SummaryMismatch);
    }
    if !corpus.summary.constrained_client_demonstrated {
        return Err(ClassroomRoleCorpusError::ConstrainedClientNotDemonstrated);
    }
    if !corpus.summary.exercise_packet_demonstrated {
        return Err(ClassroomRoleCorpusError::ExercisePacketNotDemonstrated);
    }
    Ok(())
}

/// Project a corpus into a support-safe export over every profile.
pub fn classroom_role_support_export(
    export_id: impl Into<String>,
    generated_at: impl Into<String>,
    corpus: &ClassroomRoleCorpus,
) -> ClassroomSupportExport {
    ClassroomSupportExport::from_profiles(export_id, generated_at, corpus.all_profiles())
}

fn summarize(cases: &[ClassroomRoleCase]) -> ClassroomRoleSummary {
    use std::collections::BTreeSet;

    let mut member_count = 0u32;
    let mut packet_count = 0u32;
    let mut roles: BTreeSet<TeachingRole> = BTreeSet::new();
    let mut clients: BTreeSet<ClientClass> = BTreeSet::new();
    let mut no_role_grants_product_authority = true;
    let mut all_authority_externally_sourced = true;
    let mut all_constrained_clients_join_safely = true;
    let mut all_packets_authority_bounded = true;
    let mut constrained_client_demonstrated = false;
    let mut exercise_packet_demonstrated = false;

    for case in cases {
        let profile = &case.profile;
        for member in &profile.members {
            member_count += 1;
            roles.insert(member.role);
            clients.insert(member.client_class);
            if member.role.grants_terminal_or_debug_control()
                || member.role.implies_broader_authority()
                || member.role_grants_terminal_or_debug_control
                || member.role_implies_broader_authority
                || member.product_authority.granted_by_classroom_role
            {
                no_role_grants_product_authority = false;
            }
            if member.product_authority.granted_by_classroom_role {
                all_authority_externally_sourced = false;
            }
            if member.client_class.is_constrained() {
                constrained_client_demonstrated = true;
                if !member.degrades_honestly {
                    all_constrained_clients_join_safely = false;
                }
            }
        }
        for packet in &profile.exercise_packets {
            packet_count += 1;
            exercise_packet_demonstrated = true;
            if !packet.is_authority_bounded() {
                all_packets_authority_bounded = false;
            }
        }
    }

    ClassroomRoleSummary {
        case_count: cases.len() as u32,
        member_count,
        packet_count,
        roles_covered: roles.into_iter().collect(),
        client_classes_covered: clients.into_iter().collect(),
        no_role_grants_product_authority,
        all_authority_externally_sourced,
        all_constrained_clients_join_safely,
        all_packets_authority_bounded,
        constrained_client_demonstrated,
        exercise_packet_demonstrated,
    }
}

// ---- builders -------------------------------------------------------------

fn case(case_id: &str, scenario: &str, profile: ClassroomProfile) -> ClassroomRoleCase {
    ClassroomRoleCase {
        record_kind: CLASSROOM_ROLE_CASE_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        case_id: case_id.to_owned(),
        scenario_label: scenario.to_owned(),
        profile,
    }
}

/// A small, well-formed exercise packet constrained to a file and a symbol.
fn trace_packet(packet_id: &str) -> ExercisePacket {
    let targets = vec![
        ExerciseTarget::new(
            "file:crates/aureline-shell/src/presentation/classroom/roles.rs",
            ExerciseTargetKind::File,
        ),
        ExerciseTarget::new("symbol:fn for_seat", ExerciseTargetKind::Symbol),
    ];
    let actions = vec![
        ExpectedAction::new(
            "action.open-target",
            "Open the roles module",
            "cmd:editor.open_file",
            "key:editor.open_file",
            "file:crates/aureline-shell/src/presentation/classroom/roles.rs",
        ),
        ExpectedAction::new(
            "action.jump-symbol",
            "Jump to the capability-summary function",
            "cmd:editor.go_to_symbol",
            "key:editor.go_to_symbol",
            "symbol:fn for_seat",
        ),
    ];
    ExercisePacket::new(
        packet_id,
        "Trace the constrained-client capability summary",
        targets,
        actions,
    )
}

fn five_roles_case() -> ClassroomRoleCase {
    // Roles are expressed independently of authority. The approver holds product
    // approval authority only through a separate external grant; no role grants
    // terminal/debug control or implies broader authority by itself.
    let members = vec![
        ClassroomMember::new(
            "classroom:roles-01:moderator",
            TeachingRole::Moderator,
            ClientClass::Full,
            ProductAuthorityAttribution::none(),
        ),
        ClassroomMember::new(
            "classroom:roles-01:participant",
            TeachingRole::Participant,
            ClientClass::Full,
            ProductAuthorityAttribution::none(),
        ),
        ClassroomMember::new(
            "classroom:roles-01:observer",
            TeachingRole::Observer,
            ClientClass::Full,
            ProductAuthorityAttribution::none(),
        ),
        ClassroomMember::new(
            "classroom:roles-01:approver",
            TeachingRole::Approver,
            ClientClass::Full,
            ProductAuthorityAttribution::external("policy:grant:approval:role-lead"),
        ),
        ClassroomMember::new(
            "classroom:roles-01:scribe",
            TeachingRole::Scribe,
            ClientClass::Full,
            ProductAuthorityAttribution::none(),
        ),
    ];
    let profile = project_classroom_profile(
        "classroom-session:roles-01",
        members,
        vec![trace_packet("classroom:roles-01:packet-1")],
    );
    case(
        "classroom-case:five-roles-authority-separated",
        "Teaching session expressing moderator, participant, observer, approver, \
         and scribe roles on full clients. The approver's product approval \
         authority comes from a separate external grant; no role grants \
         terminal/debug control or implies broader authority by itself.",
        profile,
    )
}

fn constrained_clients_case() -> ClassroomRoleCase {
    // Constrained clients join honestly: a moderator on a low-bandwidth client
    // loses its drive controls (omitted, not broken) and joins as a note-taker; a
    // participant on a limited client joins as a note-taker; and a first-class
    // observer stays observing.
    let members = vec![
        ClassroomMember::new(
            "classroom:constrained-02:moderator",
            TeachingRole::Moderator,
            ClientClass::Full,
            ProductAuthorityAttribution::none(),
        ),
        ClassroomMember::new(
            "classroom:constrained-02:low-bandwidth-moderator",
            TeachingRole::Moderator,
            ClientClass::LowBandwidth,
            ProductAuthorityAttribution::none(),
        ),
        ClassroomMember::new(
            "classroom:constrained-02:limited-participant",
            TeachingRole::Participant,
            ClientClass::Limited,
            ProductAuthorityAttribution::none(),
        ),
        ClassroomMember::new(
            "classroom:constrained-02:observer",
            TeachingRole::Observer,
            ClientClass::Full,
            ProductAuthorityAttribution::none(),
        ),
    ];
    let profile =
        project_classroom_profile("classroom-session:constrained-02", members, Vec::new());
    case(
        "classroom-case:constrained-clients-join-as-observer-or-note-taker",
        "Constrained clients join honestly: a moderator on a low-bandwidth client \
         keeps no drive controls and joins as a note-taker, a participant on a \
         limited client joins as a note-taker, and a first-class observer stays \
         observing — never a seat staring at unusable controls.",
        profile,
    )
}

fn exercise_packet_case() -> ClassroomRoleCase {
    // Two packets, each command-backed and constrained to its declared targets,
    // posted by a moderator to a participant.
    let members = vec![
        ClassroomMember::new(
            "classroom:packets-03:moderator",
            TeachingRole::Moderator,
            ClientClass::Full,
            ProductAuthorityAttribution::none(),
        ),
        ClassroomMember::new(
            "classroom:packets-03:participant",
            TeachingRole::Participant,
            ClientClass::Full,
            ProductAuthorityAttribution::none(),
        ),
    ];
    let review_packet = ExercisePacket::new(
        "classroom:packets-03:packet-review",
        "Review the proposed change within the diff",
        vec![
            ExerciseTarget::new("diff:review:classroom-lane", ExerciseTargetKind::Diff),
            ExerciseTarget::new(
                "doc:docs/ux/classroom-and-teaching-roles.md",
                ExerciseTargetKind::Doc,
            ),
        ],
        vec![
            ExpectedAction::new(
                "action.open-diff",
                "Open the review diff",
                "cmd:review.open_diff",
                "key:review.open_diff",
                "diff:review:classroom-lane",
            ),
            ExpectedAction::new(
                "action.open-contract",
                "Open the teaching-roles contract",
                "cmd:docs.open",
                "key:docs.open",
                "doc:docs/ux/classroom-and-teaching-roles.md",
            ),
        ],
    );
    let profile = project_classroom_profile(
        "classroom-session:packets-03",
        members,
        vec![
            trace_packet("classroom:packets-03:packet-trace"),
            review_packet,
        ],
    );
    case(
        "classroom-case:exercise-packet-authority-bounded",
        "Exercise packets stay command-backed, inspectable, and authority-bounded: \
         every expected action invokes an existing command and stays inside the \
         packet's declared file, symbol, diff, and doc targets, opening no hidden \
         mutation path.",
        profile,
    )
}

/// Build the full seeded classroom-role-and-authority corpus.
pub fn seeded_classroom_role_corpus() -> ClassroomRoleCorpus {
    let cases = vec![
        five_roles_case(),
        constrained_clients_case(),
        exercise_packet_case(),
    ];
    let summary = summarize(&cases);
    ClassroomRoleCorpus {
        record_kind: CLASSROOM_ROLE_CORPUS_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        generated_at: "2026-06-21T00:00:00Z".to_owned(),
        summary,
        cases,
    }
}

/// A single canonical [`ClassroomProfile`] example, mirrored by
/// `artifacts/presentation/classroom-role.example.json`.
pub fn classroom_role_example() -> ClassroomProfile {
    five_roles_case().profile
}

/// A single canonical [`ExercisePacket`] example, mirrored by
/// `artifacts/presentation/exercise-packet.example.json`.
pub fn exercise_packet_example() -> ExercisePacket {
    trace_packet("classroom:example:packet-1")
}
