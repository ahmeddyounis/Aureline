//! Teaching / classroom roles, exercise-packet boundaries, observer-or-note-taker
//! degradation, and the separation of teaching roles from product authority.
//!
//! A claimed teaching or classroom presentation needs to express *who is doing
//! what* — a moderator driving the session, participants attempting exercises, an
//! approver signing exercises off, a scribe taking notes, and observers watching.
//! The trap the spec exists to avoid is letting a classroom metaphor smuggle in
//! hidden control: a "moderator" must not silently gain terminal/debug control,
//! approval authority over real product gates, or ordinary editing rights, and an
//! "approver" badge must not be confused with approving product changes.
//!
//! Rather than mint a second role vocabulary, this M5 presentation lane **reuses**
//! the canonical [`TeachingRole`] and [`ClientClass`] from
//! [`crate::teaching_session`] — the same five roles, client classes, and
//! observer-or-note-taker degradation rules whose role-aware affordance proof
//! already lives there. On top of them it adds the pieces the presentation
//! classroom contract still needs:
//!
//! - [`ProductAuthorityAttribution`] makes the separation explicit and auditable:
//!   a member's real product authority (if any) is always sourced from a separate
//!   external grant, never from the classroom badge
//!   ([`ProductAuthorityAttribution::granted_by_classroom_role`] is always false).
//! - [`MemberCapabilitySummary`] records, per seat, the honest capabilities a
//!   `(role, client)` pair resolves to — derived from the canonical role/client
//!   predicates — so a limited or low-bandwidth client is recorded as the
//!   observer or note-taker it actually is, never as a seat with controls it
//!   cannot use ([`ClassroomMember::degrades_honestly`]).
//! - [`ExercisePacket`] constrains an exercise to declared targets and
//!   command-backed expected actions, opening no mutation path outside the command
//!   and policy systems ([`ExercisePacket::is_authority_bounded`]).
//!
//! [`ClassroomProfile`] ties a session's members and packets into one inspectable
//! truth packet, and [`ClassroomProfile::validate`] re-derives every guardrail so
//! a hand-edited fixture cannot quietly grant a badge real authority. The
//! support-safe projection is [`ClassroomSupportExport`]: it records roles,
//! client classes, and posture honestly but carries no instructional prose,
//! member display name, or authority-grant ref.
//!
//! The canonical roster / role schema is
//! [`schemas/presentation/classroom-role.schema.json`](../../../../../schemas/presentation/classroom-role.schema.json);
//! the exercise-packet schema is
//! [`schemas/presentation/exercise-packet.schema.json`](../../../../../schemas/presentation/exercise-packet.schema.json);
//! the human-readable contract is `docs/ux/classroom-and-teaching-roles.md`.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::presentation_mode::{
    PRESENTATION_MODE_BETA_SCHEMA_VERSION, PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF,
};
pub use crate::teaching_session::{ClientClass, TeachingRole};

/// Stable record kind for [`ClassroomProfile`] payloads.
pub const CLASSROOM_PROFILE_RECORD_KIND: &str = "presentation_classroom_profile_record";

/// Stable record kind for [`ClassroomMember`] payloads.
pub const CLASSROOM_MEMBER_RECORD_KIND: &str = "presentation_classroom_member_record";

/// Stable record kind for [`ExercisePacket`] payloads.
pub const EXERCISE_PACKET_RECORD_KIND: &str = "presentation_exercise_packet_record";

/// Stable record kind for [`ClassroomSupportExport`] payloads.
pub const CLASSROOM_SUPPORT_EXPORT_RECORD_KIND: &str =
    "presentation_classroom_support_export_record";

/// Stable record kind for [`ClassroomMemberDiagnosticsRow`] payloads.
pub const CLASSROOM_MEMBER_DIAGNOSTICS_ROW_RECORD_KIND: &str =
    "presentation_classroom_member_diagnostics_row_record";

/// Stable record kind for [`ExercisePacketDiagnosticsRow`] payloads.
pub const EXERCISE_PACKET_DIAGNOSTICS_ROW_RECORD_KIND: &str =
    "presentation_exercise_packet_diagnostics_row_record";

/// Repo-relative path of the canonical classroom-role / roster schema.
pub const CLASSROOM_ROLE_SCHEMA_REF: &str = "schemas/presentation/classroom-role.schema.json";

/// Repo-relative path of the canonical exercise-packet schema.
pub const EXERCISE_PACKET_SCHEMA_REF: &str = "schemas/presentation/exercise-packet.schema.json";

/// Repo-relative path of the human-readable classroom / teaching-roles contract.
pub const CLASSROOM_AND_TEACHING_ROLES_DOC_REF: &str = "docs/ux/classroom-and-teaching-roles.md";

/// Directory holding the checked-in classroom-role-and-authority fixtures.
pub const CLASSROOM_ROLE_FIXTURE_DIR: &str = "fixtures/presentation/classroom-role-and-authority";

// ---------------------------------------------------------------------------
// Honest per-seat capability summary
// ---------------------------------------------------------------------------

/// The honest capabilities a `(role, client)` seat resolves to.
///
/// Derived from the canonical [`TeachingRole`] and [`ClientClass`] predicates via
/// [`Self::for_seat`], so it cannot drift from the role-aware affordance proof in
/// [`crate::teaching_session`]. Drive and mutation capabilities are gated off for
/// a constrained client — they are *omitted*, never claimed-then-broken — so a
/// limited or low-bandwidth seat is recorded as the observer or note-taker it
/// actually is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberCapabilitySummary {
    /// Whether the seat may drive the session (moderator on a full client).
    pub can_drive_session: bool,
    /// Whether the seat may take shared notes (note-taking is low-bandwidth safe).
    pub can_take_notes: bool,
    /// Whether the seat exposes a mutation affordance (still through the ordinary
    /// command / approval fence) — never on a constrained client.
    pub may_expose_mutation_affordance: bool,
    /// Whether the seat is a pure observer: no drive, mutation, or note control.
    pub observer_only: bool,
    /// Whether the seat joins safely: a constrained client is never recorded with
    /// a drive or mutation capability it cannot use. Always `true` by construction.
    pub joins_safely: bool,
}

impl MemberCapabilitySummary {
    /// Resolve the honest capabilities for a `(role, client)` seat.
    pub fn for_seat(role: TeachingRole, client: ClientClass) -> Self {
        let constrained = client.is_constrained();
        let can_drive_session = role.can_drive_session() && !constrained;
        let can_take_notes = role.can_take_notes();
        let may_expose_mutation_affordance = role.may_expose_mutation_affordance() && !constrained;
        let observer_only =
            !can_drive_session && !can_take_notes && !may_expose_mutation_affordance;
        // A constrained client is safe when it is never recorded with a drive or
        // mutation capability; a full client is always safe.
        let joins_safely = if constrained {
            !can_drive_session && !may_expose_mutation_affordance
        } else {
            true
        };
        Self {
            can_drive_session,
            can_take_notes,
            may_expose_mutation_affordance,
            observer_only,
            joins_safely,
        }
    }
}

// ---------------------------------------------------------------------------
// Product authority attribution (the separation axis)
// ---------------------------------------------------------------------------

/// Where a member's real product authority comes from — never the classroom.
///
/// This is the separation axis: a member may genuinely hold terminal/debug,
/// approval, or editing rights, but only ever through a separate, externally
/// recorded grant. [`Self::granted_by_classroom_role`] is fixed `false`, so the
/// badge can never be the source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductAuthorityAttribution {
    /// Whether the member holds any product authority from a separate grant.
    pub holds_external_authority: bool,
    /// The external grant ref, present exactly when authority is held. Minted by
    /// the permission system, never by this lane.
    pub external_authority_ref: Option<String>,
    /// Always `false`: no product authority was granted by the classroom role.
    pub granted_by_classroom_role: bool,
}

impl ProductAuthorityAttribution {
    /// A member who holds no product authority beyond their baseline; the
    /// classroom adds none.
    pub fn none() -> Self {
        Self {
            holds_external_authority: false,
            external_authority_ref: None,
            granted_by_classroom_role: false,
        }
    }

    /// A member whose product authority comes from a separate external grant.
    pub fn external(authority_ref: impl Into<String>) -> Self {
        Self {
            holds_external_authority: true,
            external_authority_ref: Some(authority_ref.into()),
            granted_by_classroom_role: false,
        }
    }

    /// Whether the attribution is internally honest: the classroom granted
    /// nothing, and the ref is present exactly when authority is held.
    pub fn is_consistent(&self) -> bool {
        !self.granted_by_classroom_role
            && self.holds_external_authority == self.external_authority_ref.is_some()
            && self
                .external_authority_ref
                .as_ref()
                .map(|r| !r.trim().is_empty())
                .unwrap_or(true)
    }
}

// ---------------------------------------------------------------------------
// Classroom member
// ---------------------------------------------------------------------------

/// One member of a teaching / classroom session.
///
/// Built through [`Self::new`], which resolves the honest capability summary from
/// the role and client and fixes the authority-separation guardrail flags to
/// their safe values. The teaching role and the product authority are
/// deliberately separate fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassroomMember {
    /// Record kind; must equal [`CLASSROOM_MEMBER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable member id (never a display name).
    pub participant_id: String,
    /// The member's teaching role (canonical [`TeachingRole`] vocabulary).
    pub role: TeachingRole,
    /// How capable the member's client is (canonical [`ClientClass`] vocabulary).
    pub client_class: ClientClass,
    /// The honest capabilities this seat resolves to.
    pub capability: MemberCapabilitySummary,
    /// Where the member's real product authority comes from (never the role).
    pub product_authority: ProductAuthorityAttribution,
    /// Always `false`: the role granted no terminal / debug control.
    pub role_grants_terminal_or_debug_control: bool,
    /// Always `false`: the role implies no broader authority than the workspace
    /// already permits (no approval authority and no editing rights from a badge).
    pub role_implies_broader_authority: bool,
    /// Always `true`: the seat is recorded with no capability its client cannot
    /// use, so a limited client joins honestly as an observer or note-taker.
    pub degrades_honestly: bool,
}

impl ClassroomMember {
    /// Build a member, resolving the honest capability summary and fixing the
    /// authority guardrails safe.
    pub fn new(
        participant_id: impl Into<String>,
        role: TeachingRole,
        client_class: ClientClass,
        product_authority: ProductAuthorityAttribution,
    ) -> Self {
        let capability = MemberCapabilitySummary::for_seat(role, client_class);
        let degrades_honestly = capability.joins_safely;
        Self {
            record_kind: CLASSROOM_MEMBER_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            participant_id: participant_id.into(),
            role,
            client_class,
            capability,
            product_authority,
            role_grants_terminal_or_debug_control: role.grants_terminal_or_debug_control(),
            role_implies_broader_authority: role.implies_broader_authority(),
            degrades_honestly,
        }
    }

    /// Whether the member is internally consistent: the role grants no authority,
    /// the authority attribution is honest, the capability summary matches the
    /// canonical derivation, and the seat joins safely.
    pub fn is_consistent(&self) -> bool {
        self.consistency_violation().is_none()
    }

    /// The first consistency violation for this member, if any.
    fn consistency_violation(&self) -> Option<ClassroomViolation> {
        let participant_id = self.participant_id.clone();
        if self.record_kind != CLASSROOM_MEMBER_RECORD_KIND
            || self.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
            || self.shared_contract_ref != PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF
            || self.participant_id.trim().is_empty()
        {
            return Some(ClassroomViolation::MemberMalformed { participant_id });
        }
        if self.role_grants_terminal_or_debug_control
            || self.role.grants_terminal_or_debug_control()
        {
            return Some(ClassroomViolation::RoleGrantsTerminalOrDebugControl { participant_id });
        }
        if self.role_implies_broader_authority || self.role.implies_broader_authority() {
            return Some(ClassroomViolation::RoleImpliesBroaderAuthority { participant_id });
        }
        if !self.product_authority.is_consistent()
            || self.product_authority.granted_by_classroom_role
        {
            return Some(ClassroomViolation::AuthorityAttributionInconsistent { participant_id });
        }
        // The capability summary must match the canonical derivation, so a hand-
        // edited fixture cannot claim a capability the seat does not actually have.
        let expected = MemberCapabilitySummary::for_seat(self.role, self.client_class);
        if self.capability != expected {
            return Some(ClassroomViolation::CapabilityDerivationMismatch { participant_id });
        }
        // A constrained client must never be recorded with a drive or mutation
        // capability, and the honesty flag must say so.
        if !self.capability.joins_safely || !self.degrades_honestly {
            return Some(ClassroomViolation::MemberDegradesDishonestly { participant_id });
        }
        None
    }
}

// ---------------------------------------------------------------------------
// Exercise packets
// ---------------------------------------------------------------------------

/// The kind of object an exercise target points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseTargetKind {
    /// A file in the workspace.
    File,
    /// A symbol anchor within a file.
    Symbol,
    /// A diff / review object.
    Diff,
    /// A docs / knowledge object.
    Doc,
    /// A topology / dependency graph object.
    Graph,
}

impl ExerciseTargetKind {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Symbol => "symbol",
            Self::Diff => "diff",
            Self::Doc => "doc",
            Self::Graph => "graph",
        }
    }
}

/// One target an exercise is constrained to: a stable ref plus its kind. The
/// `target_ref` is a stable id, never a file body or symbol source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseTarget {
    /// Stable id of the target object.
    pub target_ref: String,
    /// The kind of object the target addresses.
    pub kind: ExerciseTargetKind,
}

impl ExerciseTarget {
    /// Build a typed exercise target.
    pub fn new(target_ref: impl Into<String>, kind: ExerciseTargetKind) -> Self {
        Self {
            target_ref: target_ref.into(),
            kind,
        }
    }
}

/// One action an exercise expects, bound to an existing command.
///
/// An expected action never carries its own mutation path: it names a stable
/// command id (so the action runs through the command and policy systems) and a
/// declared target it operates on (so the exercise stays inside its boundary).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedAction {
    /// Stable action id within the packet.
    pub action_id: String,
    /// Short visible label.
    pub label: String,
    /// Stable command id this action invokes — the action is command-backed.
    pub command_id: String,
    /// Stable key-binding id so the action is keyboard reachable.
    pub key_binding_ref: String,
    /// The declared target this action operates on; must be one of the packet's
    /// [`ExercisePacket::targets`].
    pub target_ref: String,
}

impl ExpectedAction {
    /// Build an expected action.
    pub fn new(
        action_id: impl Into<String>,
        label: impl Into<String>,
        command_id: impl Into<String>,
        key_binding_ref: impl Into<String>,
        target_ref: impl Into<String>,
    ) -> Self {
        Self {
            action_id: action_id.into(),
            label: label.into(),
            command_id: command_id.into(),
            key_binding_ref: key_binding_ref.into(),
            target_ref: target_ref.into(),
        }
    }

    /// Whether the action is command-backed: it carries a non-empty `cmd:` id and
    /// a non-empty key-binding ref.
    pub fn is_command_backed(&self) -> bool {
        self.command_id.starts_with("cmd:")
            && !self.key_binding_ref.trim().is_empty()
            && !self.action_id.trim().is_empty()
    }
}

/// The authority boundary an exercise packet stays within.
///
/// Every flag is fixed to its safe value by [`ExercisePacket::new`] and re-checked
/// by [`ExercisePacket::is_authority_bounded`]. Together they assert the packet is
/// command-backed, stays inside its declared targets, opens no hidden mutation
/// path, and widens no role's product authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExercisePacketAuthorityBound {
    /// Whether every expected action invokes an existing command.
    pub all_actions_command_backed: bool,
    /// Whether every expected action stays within the declared targets.
    pub constrained_to_declared_targets: bool,
    /// Always `false`: the packet opens no mutation path outside the command and
    /// policy systems.
    pub opens_hidden_mutation_path: bool,
    /// Always `false`: the packet widens no role's product authority.
    pub widens_product_authority: bool,
}

/// An exercise packet: a teaching exercise constrained to declared targets, with
/// each expected action backed by an existing command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExercisePacket {
    /// Record kind; must equal [`EXERCISE_PACKET_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet title.
    pub title: String,
    /// The targets the exercise constrains itself to.
    pub targets: Vec<ExerciseTarget>,
    /// The actions the exercise expects, each command-backed and target-bound.
    pub expected_actions: Vec<ExpectedAction>,
    /// The authority boundary the packet stays within.
    pub authority_bound: ExercisePacketAuthorityBound,
}

impl ExercisePacket {
    /// Build a packet, deriving its authority boundary from its actions and
    /// targets. The hidden-mutation and authority-widening flags are fixed safe;
    /// the command-backed and target-constrained flags are computed from the
    /// actual actions so a malformed packet is detectable.
    pub fn new(
        packet_id: impl Into<String>,
        title: impl Into<String>,
        targets: Vec<ExerciseTarget>,
        expected_actions: Vec<ExpectedAction>,
    ) -> Self {
        let all_actions_command_backed =
            !expected_actions.is_empty() && expected_actions.iter().all(|a| a.is_command_backed());
        let declared: BTreeSet<&str> = targets.iter().map(|t| t.target_ref.as_str()).collect();
        let constrained_to_declared_targets = expected_actions
            .iter()
            .all(|a| declared.contains(a.target_ref.as_str()));
        Self {
            record_kind: EXERCISE_PACKET_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            packet_id: packet_id.into(),
            title: title.into(),
            targets,
            expected_actions,
            authority_bound: ExercisePacketAuthorityBound {
                all_actions_command_backed,
                constrained_to_declared_targets,
                opens_hidden_mutation_path: false,
                widens_product_authority: false,
            },
        }
    }

    /// The distinct target kinds the packet references, sorted.
    pub fn target_kinds(&self) -> Vec<ExerciseTargetKind> {
        let mut kinds: Vec<ExerciseTargetKind> = self.targets.iter().map(|t| t.kind).collect();
        kinds.sort();
        kinds.dedup();
        kinds
    }

    /// Whether the packet is well-formed: canonical identity, at least one target
    /// and one action, and a non-empty id and title.
    pub fn is_well_formed(&self) -> bool {
        self.record_kind == EXERCISE_PACKET_RECORD_KIND
            && self.schema_version == PRESENTATION_MODE_BETA_SCHEMA_VERSION
            && self.shared_contract_ref == PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF
            && !self.packet_id.trim().is_empty()
            && !self.title.trim().is_empty()
            && !self.targets.is_empty()
            && !self.expected_actions.is_empty()
    }

    /// Whether the packet stays inside its authority boundary: every action is
    /// command-backed and target-constrained, and it opens no hidden mutation
    /// path nor widens product authority.
    pub fn is_authority_bounded(&self) -> bool {
        let expected = Self::new(
            self.packet_id.clone(),
            self.title.clone(),
            self.targets.clone(),
            self.expected_actions.clone(),
        )
        .authority_bound;
        self.is_well_formed()
            && self.authority_bound == expected
            && self.authority_bound.all_actions_command_backed
            && self.authority_bound.constrained_to_declared_targets
            && !self.authority_bound.opens_hidden_mutation_path
            && !self.authority_bound.widens_product_authority
    }
}

// ---------------------------------------------------------------------------
// Classroom profile (the canonical truth packet)
// ---------------------------------------------------------------------------

/// The canonical classroom truth packet for one session: its members, exercise
/// packets, and the guardrail flags that keep teaching roles separate from
/// product authority and limited clients joining honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassroomProfile {
    /// Record kind; must equal [`CLASSROOM_PROFILE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Session id.
    pub session_id: String,
    /// The session's members.
    pub members: Vec<ClassroomMember>,
    /// The session's exercise packets.
    pub exercise_packets: Vec<ExercisePacket>,
    // ---- derived invariant flags (re-checked by validate) ----
    /// No member's role granted terminal / debug control.
    pub no_role_grants_terminal_or_debug_control: bool,
    /// No member's role implied broader authority (approval or editing rights).
    pub no_role_implies_broader_authority: bool,
    /// No member's product authority was sourced from a classroom role.
    pub no_authority_granted_by_classroom_role: bool,
    /// Every constrained-client member joins safely as an observer or note-taker.
    pub all_constrained_clients_join_safely: bool,
    /// Every exercise packet is command-backed.
    pub all_packets_command_backed: bool,
    /// Every exercise packet stays inside its authority boundary.
    pub all_packets_authority_bounded: bool,
    /// No exercise packet opens a mutation path outside the command system.
    pub no_packet_opens_hidden_mutation_path: bool,
    // ---- absolute guardrails (always false) ----
    /// Always `false`: the classroom opens no mutation shortcut.
    pub grants_mutation_authority: bool,
    /// Always `false`: the classroom grants no shared editing / debug control.
    pub grants_control_authority: bool,
}

impl ClassroomProfile {
    /// The members holding `role`.
    pub fn members_with_role(&self, role: TeachingRole) -> Vec<&ClassroomMember> {
        self.members.iter().filter(|m| m.role == role).collect()
    }

    /// The distinct roles present in the session, sorted.
    pub fn roles_present(&self) -> Vec<TeachingRole> {
        self.members
            .iter()
            .map(|m| m.role)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    /// Validate every invariant the packet claims. An empty result means the
    /// packet is internally honest: roles grant no product authority, constrained
    /// clients join honestly, and packets stay command-backed and bounded.
    pub fn validate(&self) -> Vec<ClassroomViolation> {
        let mut violations = Vec::new();

        if self.record_kind != CLASSROOM_PROFILE_RECORD_KIND
            || self.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
            || self.shared_contract_ref != PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF
            || self.session_id.trim().is_empty()
        {
            violations.push(ClassroomViolation::MalformedPacket);
        }

        if self.grants_mutation_authority || self.grants_control_authority {
            violations.push(ClassroomViolation::AuthorityWidened);
        }

        for member in &self.members {
            if let Some(v) = member.consistency_violation() {
                violations.push(v);
            }
        }

        for packet in &self.exercise_packets {
            if !packet.is_well_formed() {
                violations.push(ClassroomViolation::ExercisePacketMalformed {
                    packet_id: packet.packet_id.clone(),
                });
                continue;
            }
            if !packet.authority_bound.all_actions_command_backed {
                violations.push(ClassroomViolation::ExercisePacketNotCommandBacked {
                    packet_id: packet.packet_id.clone(),
                });
            }
            if !packet.authority_bound.constrained_to_declared_targets {
                violations.push(ClassroomViolation::ExercisePacketUnconstrained {
                    packet_id: packet.packet_id.clone(),
                });
            }
            if packet.authority_bound.opens_hidden_mutation_path {
                violations.push(ClassroomViolation::ExercisePacketOpensHiddenMutationPath {
                    packet_id: packet.packet_id.clone(),
                });
            }
            if packet.authority_bound.widens_product_authority {
                violations.push(ClassroomViolation::ExercisePacketWidensAuthority {
                    packet_id: packet.packet_id.clone(),
                });
            }
        }

        let expected = derive_profile_flags(&self.members, &self.exercise_packets);
        let claimed = ProfileFlags {
            no_role_grants_terminal_or_debug_control: self.no_role_grants_terminal_or_debug_control,
            no_role_implies_broader_authority: self.no_role_implies_broader_authority,
            no_authority_granted_by_classroom_role: self.no_authority_granted_by_classroom_role,
            all_constrained_clients_join_safely: self.all_constrained_clients_join_safely,
            all_packets_command_backed: self.all_packets_command_backed,
            all_packets_authority_bounded: self.all_packets_authority_bounded,
            no_packet_opens_hidden_mutation_path: self.no_packet_opens_hidden_mutation_path,
        };
        if expected != claimed {
            violations.push(ClassroomViolation::DerivedFlagsMismatch);
        }

        violations
    }
}

/// A reason a [`ClassroomProfile`] failed validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassroomViolation {
    /// The packet carried the wrong record kind, version, contract ref, or id.
    MalformedPacket,
    /// The packet claimed to widen mutation or control authority.
    AuthorityWidened,
    /// A member record was malformed.
    MemberMalformed {
        /// The offending member id.
        participant_id: String,
    },
    /// A member's role claimed terminal / debug control.
    RoleGrantsTerminalOrDebugControl {
        /// The offending member id.
        participant_id: String,
    },
    /// A member's role implied broader authority (approval / editing rights).
    RoleImpliesBroaderAuthority {
        /// The offending member id.
        participant_id: String,
    },
    /// A member's product authority attribution was inconsistent, or claimed the
    /// classroom role as its source.
    AuthorityAttributionInconsistent {
        /// The offending member id.
        participant_id: String,
    },
    /// A member's capability summary did not match the canonical derivation for
    /// its role and client.
    CapabilityDerivationMismatch {
        /// The offending member id.
        participant_id: String,
    },
    /// A constrained-client member was recorded with a capability it cannot use.
    MemberDegradesDishonestly {
        /// The offending member id.
        participant_id: String,
    },
    /// An exercise packet was malformed (empty id/title or no targets/actions).
    ExercisePacketMalformed {
        /// The offending packet id.
        packet_id: String,
    },
    /// An exercise packet had an action that is not command-backed.
    ExercisePacketNotCommandBacked {
        /// The offending packet id.
        packet_id: String,
    },
    /// An exercise packet had an action outside its declared targets.
    ExercisePacketUnconstrained {
        /// The offending packet id.
        packet_id: String,
    },
    /// An exercise packet claimed a hidden mutation path.
    ExercisePacketOpensHiddenMutationPath {
        /// The offending packet id.
        packet_id: String,
    },
    /// An exercise packet claimed to widen product authority.
    ExercisePacketWidensAuthority {
        /// The offending packet id.
        packet_id: String,
    },
    /// The packet's derived invariant flags did not match its members / packets.
    DerivedFlagsMismatch,
}

impl ClassroomViolation {
    /// Stable token used in tests and diagnostics.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MalformedPacket => "malformed_packet",
            Self::AuthorityWidened => "authority_widened",
            Self::MemberMalformed { .. } => "member_malformed",
            Self::RoleGrantsTerminalOrDebugControl { .. } => {
                "role_grants_terminal_or_debug_control"
            }
            Self::RoleImpliesBroaderAuthority { .. } => "role_implies_broader_authority",
            Self::AuthorityAttributionInconsistent { .. } => "authority_attribution_inconsistent",
            Self::CapabilityDerivationMismatch { .. } => "capability_derivation_mismatch",
            Self::MemberDegradesDishonestly { .. } => "member_degrades_dishonestly",
            Self::ExercisePacketMalformed { .. } => "exercise_packet_malformed",
            Self::ExercisePacketNotCommandBacked { .. } => "exercise_packet_not_command_backed",
            Self::ExercisePacketUnconstrained { .. } => "exercise_packet_unconstrained",
            Self::ExercisePacketOpensHiddenMutationPath { .. } => {
                "exercise_packet_opens_hidden_mutation_path"
            }
            Self::ExercisePacketWidensAuthority { .. } => "exercise_packet_widens_authority",
            Self::DerivedFlagsMismatch => "derived_flags_mismatch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProfileFlags {
    no_role_grants_terminal_or_debug_control: bool,
    no_role_implies_broader_authority: bool,
    no_authority_granted_by_classroom_role: bool,
    all_constrained_clients_join_safely: bool,
    all_packets_command_backed: bool,
    all_packets_authority_bounded: bool,
    no_packet_opens_hidden_mutation_path: bool,
}

fn derive_profile_flags(members: &[ClassroomMember], packets: &[ExercisePacket]) -> ProfileFlags {
    let mut no_terminal = true;
    let mut no_broader = true;
    let mut no_classroom_grant = true;
    let mut constrained_safe = true;

    for member in members {
        if member.role_grants_terminal_or_debug_control
            || member.role.grants_terminal_or_debug_control()
        {
            no_terminal = false;
        }
        if member.role_implies_broader_authority || member.role.implies_broader_authority() {
            no_broader = false;
        }
        if member.product_authority.granted_by_classroom_role {
            no_classroom_grant = false;
        }
        if member.client_class.is_constrained()
            && (!member.degrades_honestly || !member.capability.joins_safely)
        {
            constrained_safe = false;
        }
    }

    let mut all_command_backed = true;
    let mut all_bounded = true;
    let mut no_hidden_path = true;
    for packet in packets {
        if !packet.authority_bound.all_actions_command_backed {
            all_command_backed = false;
        }
        if !packet.is_authority_bounded() {
            all_bounded = false;
        }
        if packet.authority_bound.opens_hidden_mutation_path {
            no_hidden_path = false;
        }
    }

    ProfileFlags {
        no_role_grants_terminal_or_debug_control: no_terminal,
        no_role_implies_broader_authority: no_broader,
        no_authority_granted_by_classroom_role: no_classroom_grant,
        all_constrained_clients_join_safely: constrained_safe,
        all_packets_command_backed: all_command_backed,
        all_packets_authority_bounded: all_bounded,
        no_packet_opens_hidden_mutation_path: no_hidden_path,
    }
}

/// Project a [`ClassroomProfile`] for `session_id` from its members and packets,
/// deriving every guardrail flag. The result validates as long as the members
/// and packets are themselves coherent.
pub fn project_classroom_profile(
    session_id: impl Into<String>,
    members: Vec<ClassroomMember>,
    exercise_packets: Vec<ExercisePacket>,
) -> ClassroomProfile {
    let flags = derive_profile_flags(&members, &exercise_packets);
    ClassroomProfile {
        record_kind: CLASSROOM_PROFILE_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        session_id: session_id.into(),
        members,
        exercise_packets,
        no_role_grants_terminal_or_debug_control: flags.no_role_grants_terminal_or_debug_control,
        no_role_implies_broader_authority: flags.no_role_implies_broader_authority,
        no_authority_granted_by_classroom_role: flags.no_authority_granted_by_classroom_role,
        all_constrained_clients_join_safely: flags.all_constrained_clients_join_safely,
        all_packets_command_backed: flags.all_packets_command_backed,
        all_packets_authority_bounded: flags.all_packets_authority_bounded,
        no_packet_opens_hidden_mutation_path: flags.no_packet_opens_hidden_mutation_path,
        grants_mutation_authority: false,
        grants_control_authority: false,
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// One support-safe row per classroom member. Carries roles, client classes,
/// capability booleans, and posture booleans — never a display name or an
/// authority-grant ref.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassroomMemberDiagnosticsRow {
    /// Record kind; must equal [`CLASSROOM_MEMBER_DIAGNOSTICS_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Session id.
    pub session_id: String,
    /// Stable member id.
    pub participant_id: String,
    /// The member's teaching role.
    pub role: TeachingRole,
    /// How capable the member's client is.
    pub client_class: ClientClass,
    /// The honest capabilities this seat resolves to.
    pub capability: MemberCapabilitySummary,
    /// Whether the member holds product authority from a separate grant
    /// (presence only — the grant ref is never carried).
    pub holds_external_authority: bool,
    /// Always `false`: no authority was granted by the classroom role.
    pub authority_granted_by_classroom_role: bool,
    /// Always `false`: the role granted no terminal / debug control.
    pub role_grants_terminal_or_debug_control: bool,
    /// Always `false`: the role implied no broader authority.
    pub role_implies_broader_authority: bool,
    /// Whether the seat joins honestly with no unusable capability.
    pub degrades_honestly: bool,
}

impl ClassroomMemberDiagnosticsRow {
    fn from_member(session_id: &str, member: &ClassroomMember) -> Self {
        Self {
            record_kind: CLASSROOM_MEMBER_DIAGNOSTICS_ROW_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            session_id: session_id.to_owned(),
            participant_id: member.participant_id.clone(),
            role: member.role,
            client_class: member.client_class,
            capability: member.capability,
            holds_external_authority: member.product_authority.holds_external_authority,
            authority_granted_by_classroom_role: member.product_authority.granted_by_classroom_role,
            role_grants_terminal_or_debug_control: member.role_grants_terminal_or_debug_control,
            role_implies_broader_authority: member.role_implies_broader_authority,
            degrades_honestly: member.degrades_honestly,
        }
    }
}

/// One support-safe row per exercise packet. Carries counts, kinds, and boundary
/// booleans — never the title, action labels, command ids, or target refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExercisePacketDiagnosticsRow {
    /// Record kind; must equal [`EXERCISE_PACKET_DIAGNOSTICS_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Session id.
    pub session_id: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Number of declared targets.
    pub target_count: u32,
    /// Distinct target kinds, sorted.
    pub target_kinds: Vec<ExerciseTargetKind>,
    /// Number of expected actions.
    pub expected_action_count: u32,
    /// Whether every action is command-backed.
    pub all_actions_command_backed: bool,
    /// Whether every action stays within the declared targets.
    pub constrained_to_declared_targets: bool,
    /// Always `false`: the packet opens no hidden mutation path.
    pub opens_hidden_mutation_path: bool,
    /// Always `false`: the packet widens no product authority.
    pub widens_product_authority: bool,
}

impl ExercisePacketDiagnosticsRow {
    fn from_packet(session_id: &str, packet: &ExercisePacket) -> Self {
        Self {
            record_kind: EXERCISE_PACKET_DIAGNOSTICS_ROW_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            session_id: session_id.to_owned(),
            packet_id: packet.packet_id.clone(),
            target_count: packet.targets.len() as u32,
            target_kinds: packet.target_kinds(),
            expected_action_count: packet.expected_actions.len() as u32,
            all_actions_command_backed: packet.authority_bound.all_actions_command_backed,
            constrained_to_declared_targets: packet.authority_bound.constrained_to_declared_targets,
            opens_hidden_mutation_path: packet.authority_bound.opens_hidden_mutation_path,
            widens_product_authority: packet.authority_bound.widens_product_authority,
        }
    }
}

/// The support / diagnostics export for classroom roles and exercise packets.
///
/// Privacy-safe by construction: it carries roles, client classes, posture
/// booleans, counts, and kinds, but never instructional prose, a member display
/// name, or an authority-grant ref. Diagnostics, support-export, and telemetry
/// surfaces ingest this rather than cloning classroom state by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassroomSupportExport {
    /// Record kind; must equal [`CLASSROOM_SUPPORT_EXPORT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// RFC 3339 mint timestamp.
    pub generated_at: String,
    /// Support-safe per-member rows.
    pub member_rows: Vec<ClassroomMemberDiagnosticsRow>,
    /// Support-safe per-packet rows.
    pub packet_rows: Vec<ExercisePacketDiagnosticsRow>,
    /// Always `true`: no row claims a role granted product control authority.
    pub no_role_grants_control_authority: bool,
    /// Always `true`: no member's authority was granted by a classroom role.
    pub no_authority_granted_by_classroom_role: bool,
    /// Always `true`: every constrained-client member joins safely.
    pub all_constrained_clients_join_safely: bool,
    /// Always `true`: every packet stays inside its authority boundary.
    pub all_packets_authority_bounded: bool,
    /// Always `true`: instructional prose and labels are excluded.
    pub raw_instructional_prose_excluded: bool,
}

impl ClassroomSupportExport {
    /// Project a set of profiles into a support-safe export.
    pub fn from_profiles<'a>(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        profiles: impl IntoIterator<Item = &'a ClassroomProfile>,
    ) -> Self {
        let mut member_rows = Vec::new();
        let mut packet_rows = Vec::new();
        for profile in profiles {
            for member in &profile.members {
                member_rows.push(ClassroomMemberDiagnosticsRow::from_member(
                    &profile.session_id,
                    member,
                ));
            }
            for packet in &profile.exercise_packets {
                packet_rows.push(ExercisePacketDiagnosticsRow::from_packet(
                    &profile.session_id,
                    packet,
                ));
            }
        }
        let no_role_grants_control_authority = member_rows
            .iter()
            .all(|r| !r.role_grants_terminal_or_debug_control && !r.role_implies_broader_authority);
        let no_authority_granted_by_classroom_role = member_rows
            .iter()
            .all(|r| !r.authority_granted_by_classroom_role);
        let all_constrained_clients_join_safely = member_rows
            .iter()
            .filter(|r| r.client_class.is_constrained())
            .all(|r| r.degrades_honestly && r.capability.joins_safely);
        let all_packets_authority_bounded = packet_rows.iter().all(|r| {
            r.all_actions_command_backed
                && r.constrained_to_declared_targets
                && !r.opens_hidden_mutation_path
                && !r.widens_product_authority
        });
        Self {
            record_kind: CLASSROOM_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            member_rows,
            packet_rows,
            no_role_grants_control_authority,
            no_authority_granted_by_classroom_role,
            all_constrained_clients_join_safely,
            all_packets_authority_bounded,
            raw_instructional_prose_excluded: true,
        }
    }

    /// Validate the export's privacy and structural invariants.
    pub fn validate(&self) -> Vec<ClassroomSupportViolation> {
        let mut violations = Vec::new();
        if self.record_kind != CLASSROOM_SUPPORT_EXPORT_RECORD_KIND
            || self.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
            || self.shared_contract_ref != PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF
        {
            violations.push(ClassroomSupportViolation::WrongRecordKind);
        }
        if self.export_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            violations.push(ClassroomSupportViolation::MissingIdentity);
        }
        for row in &self.member_rows {
            if row.role_grants_terminal_or_debug_control
                || row.role_implies_broader_authority
                || row.authority_granted_by_classroom_role
            {
                violations.push(ClassroomSupportViolation::RowClaimsRoleAuthority);
            }
            if row.client_class.is_constrained() && !row.degrades_honestly {
                violations.push(ClassroomSupportViolation::RowDegradesDishonestly);
            }
        }
        for row in &self.packet_rows {
            if !row.all_actions_command_backed
                || !row.constrained_to_declared_targets
                || row.opens_hidden_mutation_path
                || row.widens_product_authority
            {
                violations.push(ClassroomSupportViolation::RowPacketUnbounded);
            }
        }
        if !self.raw_instructional_prose_excluded {
            violations.push(ClassroomSupportViolation::ProseNotExcluded);
        }
        if json_contains_classroom_prose(
            &serde_json::to_value(self).expect("classroom support export serializes"),
        ) {
            violations.push(ClassroomSupportViolation::RawProseInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("classroom support export serializes")
    }
}

/// Validation failures emitted by [`ClassroomSupportExport::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClassroomSupportViolation {
    /// Export record kind, version, or contract ref is wrong.
    WrongRecordKind,
    /// A required identity field is missing.
    MissingIdentity,
    /// A member row claimed a role granted product authority.
    RowClaimsRoleAuthority,
    /// A constrained-client member row joined dishonestly.
    RowDegradesDishonestly,
    /// A packet row claimed it was unbounded.
    RowPacketUnbounded,
    /// The prose-excluded flag is not set.
    ProseNotExcluded,
    /// The export contains a forbidden prose / label field.
    RawProseInExport,
}

impl ClassroomSupportViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::MissingIdentity => "missing_identity",
            Self::RowClaimsRoleAuthority => "row_claims_role_authority",
            Self::RowDegradesDishonestly => "row_degrades_dishonestly",
            Self::RowPacketUnbounded => "row_packet_unbounded",
            Self::ProseNotExcluded => "prose_not_excluded",
            Self::RawProseInExport => "raw_prose_in_export",
        }
    }
}

/// Whether a serialized export carries a forbidden prose / label field. A support
/// export must never carry a packet `title`, an action `label`, a `command_id`, a
/// `target_ref`, or an `external_authority_ref`; the metadata-only rows carry
/// counts, kinds, and booleans instead.
fn json_contains_classroom_prose(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => {
            if map.contains_key("title")
                || map.contains_key("label")
                || map.contains_key("command_id")
                || map.contains_key("target_ref")
                || map.contains_key("external_authority_ref")
            {
                return true;
            }
            map.values().any(json_contains_classroom_prose)
        }
        serde_json::Value::Array(items) => items.iter().any(json_contains_classroom_prose),
        _ => false,
    }
}
