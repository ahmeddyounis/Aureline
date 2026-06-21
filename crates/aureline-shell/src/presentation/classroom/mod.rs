//! Teaching / classroom roles, exercise-packet boundaries, observer-or-note-taker
//! degradation, and authority separation from edit / debug / approval control.
//!
//! The canonical session object model — the
//! [`PresentationSession`](crate::presentation_mode::PresentationSession), its
//! waypoints, speaker notes, and audience participants — lives in
//! [`crate::presentation_mode`]. This module is the teaching-classroom layer the
//! spec calls for: it makes a session's *teaching* roles explicit while keeping
//! them strictly separate from product authority, and it keeps exercise packets a
//! thin, command-backed, authority-bounded contract rather than a hidden mutation
//! plane.
//!
//! - [`roles`] holds the model: [`ClassroomRole`] (moderator / participant /
//!   observer / approver / scribe), the [`ClientDeliveryClass`] +
//!   [`effective_classroom_mode`] honest-degradation rule that turns a limited
//!   client into an observer or note-taker instead of broken controls, the
//!   [`ProductAuthorityAttribution`] that sources real authority from a separate
//!   grant (never the badge), the [`ExercisePacket`] boundary object, and the
//!   [`ClassroomProfile`] truth packet with [`ClassroomProfile::validate`]. The
//!   support-safe projection is [`ClassroomSupportExport`].
//! - [`corpus`] is the mint-from-truth seed corpus, support export, and
//!   validation that the checked-in fixtures and headless inspectors share.
//!
//! The canonical roster / role schema is
//! [`schemas/presentation/classroom-role.schema.json`](../../../../../schemas/presentation/classroom-role.schema.json);
//! the exercise-packet schema is
//! [`schemas/presentation/exercise-packet.schema.json`](../../../../../schemas/presentation/exercise-packet.schema.json);
//! the human-readable contract is `docs/ux/classroom-and-teaching-roles.md`.

pub mod corpus;
pub mod roles;

pub use corpus::{
    classroom_role_example, classroom_role_support_export, exercise_packet_example,
    seeded_classroom_role_corpus, validate_classroom_role_corpus, ClassroomRoleCase,
    ClassroomRoleCorpus, ClassroomRoleCorpusError, ClassroomRoleSummary,
    CLASSROOM_ROLE_CASE_RECORD_KIND, CLASSROOM_ROLE_CORPUS_RECORD_KIND,
};
pub use roles::{
    project_classroom_profile, ClassroomMember, ClassroomMemberDiagnosticsRow, ClassroomProfile,
    ClassroomSupportExport, ClassroomSupportViolation, ClassroomViolation, ClientClass,
    ExercisePacket, ExercisePacketAuthorityBound, ExercisePacketDiagnosticsRow, ExerciseTarget,
    ExerciseTargetKind, ExpectedAction, MemberCapabilitySummary, ProductAuthorityAttribution,
    TeachingRole, CLASSROOM_AND_TEACHING_ROLES_DOC_REF,
    CLASSROOM_MEMBER_DIAGNOSTICS_ROW_RECORD_KIND, CLASSROOM_MEMBER_RECORD_KIND,
    CLASSROOM_PROFILE_RECORD_KIND, CLASSROOM_ROLE_FIXTURE_DIR, CLASSROOM_ROLE_SCHEMA_REF,
    CLASSROOM_SUPPORT_EXPORT_RECORD_KIND, EXERCISE_PACKET_DIAGNOSTICS_ROW_RECORD_KIND,
    EXERCISE_PACKET_RECORD_KIND, EXERCISE_PACKET_SCHEMA_REF,
};

#[cfg(test)]
mod tests;
