//! Signed and shared recipe packs, safe automation graduation, and
//! preview-first replay.
//!
//! This module ships user-authored automation into one export-safe truth packet
//! whose unit of truth is a [`RecipePackRow`]: a single signed, shareable recipe
//! pack binding the signature and share-scope posture it is distributed under,
//! the automation authority it has graduated to, and a step disclosure that
//! says — for every effect the pack can produce — how the effect previews before
//! replay, what approval gate it carries, how it is audited, and whether it is
//! reversible. The packet is the canonical recipe-pack source for shell, docs,
//! support export, and release tooling; consumers project it instead of
//! re-deriving signature, authority, or replay posture by hand.
//!
//! The packet refuses to present a recipe pack greener than its disclosure
//! posture can back. Every pack carries a content-addressed manifest so a replay
//! can prove the exact bytes it rode, and a signed pack must carry a publisher
//! identity. A pack shared beyond a local workspace must be signed, and a pack on
//! the organization-managed channel must resolve to a managed or
//! enterprise-gateway mode. Automation graduation is held honest: a pack with no
//! authority may disclose no mutating effect, a pack that publishes irreversibly
//! must have graduated to an admin-gated or managed-template authority, and a
//! managed-only template authority must run on a managed or enterprise-gateway
//! mode. A mutating step is held to the same preview, policy, and audit bar as a
//! first-party command: every replay of a mutating step previews before it
//! applies (preview-first replay), carries a real approval gate, and is audited;
//! an irreversible external publish must be externally auditable; and a declared
//! reversibility must agree with the effect class. A blocked pack —
//! policy-blocked, trust-blocked, quarantined, or withdrawn — narrows its claim
//! instead of staying behind a Stable, Beta, or Preview label, and a pack still
//! awaiting first-use review may not claim Stable. Every pack carries a closed
//! set of downgrade rules — including the proof-stale and provider-unavailable
//! triggers — that narrow the claim instead of hiding the pack, reusing the
//! qualification, downgrade-trigger, and rollback-posture vocabularies frozen by
//! the M5 AI workflow matrix lane, the provider/locality mode vocabulary frozen
//! by the routing-policy lane, and the side-effect and approval vocabularies
//! frozen by the tool-gateway baseline, so no pack row may stay greener than its
//! evidence.
//!
//! Raw shell fragments, raw filesystem paths, raw endpoint URLs, credential
//! bodies, raw API keys, and OAuth tokens stay outside the support boundary; the
//! packet carries content addresses, classes, and review-safe labels only.
//!
//! The boundary schema is
//! [`schemas/ai/implement-signed-and-shared-recipe-packs-safe-automation-graduation-and-preview-first-replay.schema.json`](../../../../schemas/ai/implement-signed-and-shared-recipe-packs-safe-automation-graduation-and-preview-first-replay.schema.json).
//! The contract doc is
//! [`docs/automation/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay.md`](../../../../docs/automation/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay/`](../../../../fixtures/ai/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass, M5AiWorkflowRollbackPosture,
    M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};
use crate::implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains::{
    RoutePolicyModeClass, ROUTING_POLICY_SCHEMA_REF,
};
use crate::tool_gateway::{
    ToolApprovalPostureClass, ToolPublisherSourceClass, ToolSideEffectClass,
    TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`RecipePackGraduationPacket`].
pub const RECIPE_PACK_RECORD_KIND: &str =
    "implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay";

/// Schema version for recipe-pack graduation records.
pub const RECIPE_PACK_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const RECIPE_PACK_SCHEMA_REF: &str =
    "schemas/ai/implement-signed-and-shared-recipe-packs-safe-automation-graduation-and-preview-first-replay.schema.json";

/// Repo-relative path of the recipe-pack contract doc.
pub const RECIPE_PACK_DOC_REF: &str =
    "docs/automation/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay.md";

/// Repo-relative path of the recorded-macro / declarative-recipe contract this
/// lane projects from.
pub const RECIPE_PACK_AUTOMATION_CONTRACT_REF: &str =
    "docs/automation/recipe_and_macro_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const RECIPE_PACK_FIXTURE_DIR: &str =
    "fixtures/ai/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay";

/// Repo-relative path of the checked support-export artifact.
pub const RECIPE_PACK_ARTIFACT_REF: &str =
    "artifacts/ai/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const RECIPE_PACK_SUMMARY_REF: &str =
    "artifacts/ai/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay.md";

/// How a recipe pack is signed for distribution.
///
/// A pack shared beyond a local workspace must be signed; a managed-only channel
/// pack additionally carries organization or managed-channel authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipePackSignatureClass {
    /// Signed by the authoring user identity.
    AuthorSignature,
    /// Signed only by an organization identity.
    OrganizationSignatureOnly,
    /// Signed by the managed-only distribution channel.
    ManagedOnlyChannelSignature,
    /// Signed by both the author and an organization identity.
    AuthorAndOrganizationSignature,
    /// Unsigned; admissible only on a local scope.
    UnsignedLocalOnly,
}

impl RecipePackSignatureClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthorSignature => "author_signature",
            Self::OrganizationSignatureOnly => "organization_signature_only",
            Self::ManagedOnlyChannelSignature => "managed_only_channel_signature",
            Self::AuthorAndOrganizationSignature => "author_and_organization_signature",
            Self::UnsignedLocalOnly => "unsigned_local_only",
        }
    }

    /// Whether the pack carries any verifiable signature.
    pub const fn is_signed(self) -> bool {
        !matches!(self, Self::UnsignedLocalOnly)
    }

    /// Whether the signature carries organization or managed-channel authority.
    pub const fn carries_organization_authority(self) -> bool {
        matches!(
            self,
            Self::OrganizationSignatureOnly
                | Self::ManagedOnlyChannelSignature
                | Self::AuthorAndOrganizationSignature
        )
    }
}

/// Where a recipe pack is stored and how widely it is distributed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipePackShareScopeClass {
    /// Stored for one user, local only.
    UserScopeLocalOnly,
    /// Stored for one workspace, local only.
    WorkspaceScopeLocalOnly,
    /// Distributed on the organization-managed channel.
    OrganizationManagedChannel,
    /// Exported into a portable profile.
    PortableProfileExport,
    /// Exported into a support bundle.
    SupportBundleExport,
}

impl RecipePackShareScopeClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserScopeLocalOnly => "user_scope_local_only",
            Self::WorkspaceScopeLocalOnly => "workspace_scope_local_only",
            Self::OrganizationManagedChannel => "organization_managed_channel",
            Self::PortableProfileExport => "portable_profile_export",
            Self::SupportBundleExport => "support_bundle_export",
        }
    }

    /// Whether the scope keeps the pack on the user's local machine.
    pub const fn is_local_only(self) -> bool {
        matches!(
            self,
            Self::UserScopeLocalOnly | Self::WorkspaceScopeLocalOnly
        )
    }

    /// Whether the scope distributes the pack beyond the local workspace.
    pub const fn is_shared_beyond_workspace(self) -> bool {
        !self.is_local_only()
    }

    /// Whether the scope rides the organization-managed channel.
    pub const fn requires_managed_channel(self) -> bool {
        matches!(self, Self::OrganizationManagedChannel)
    }
}

/// Automation authority a recipe pack has graduated to.
///
/// The authority bounds the strongest effect class a pack may produce; it is the
/// safe-automation-graduation axis the shell, policy, and audit surfaces read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationAuthorityClass {
    /// Inspect-only; the pack may produce no mutating effect.
    InspectOnlyNoAuthority,
    /// Local reversible edits only.
    LocalReversibleOnly,
    /// Local edits behind an approval gate.
    LocalWithApproval,
    /// External reversible effects behind an approval gate.
    ExternalReversibleWithApproval,
    /// External irreversible publish behind an admin-approval gate.
    ExternalIrreversibleAdminGated,
    /// Managed-only template authority distributed on the managed channel.
    ManagedOnlyTemplateAuthority,
}

impl AutomationAuthorityClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectOnlyNoAuthority => "inspect_only_no_authority",
            Self::LocalReversibleOnly => "local_reversible_only",
            Self::LocalWithApproval => "local_with_approval",
            Self::ExternalReversibleWithApproval => "external_reversible_with_approval",
            Self::ExternalIrreversibleAdminGated => "external_irreversible_admin_gated",
            Self::ManagedOnlyTemplateAuthority => "managed_only_template_authority",
        }
    }

    /// Whether the authority permits any mutating effect.
    pub const fn grants_mutation(self) -> bool {
        !matches!(self, Self::InspectOnlyNoAuthority)
    }

    /// Whether the authority permits an irreversible external publish.
    pub const fn admits_irreversible_publish(self) -> bool {
        matches!(
            self,
            Self::ExternalIrreversibleAdminGated | Self::ManagedOnlyTemplateAuthority
        )
    }

    /// Whether the authority must run on a managed or enterprise-gateway mode.
    pub const fn requires_managed_mode(self) -> bool {
        matches!(self, Self::ManagedOnlyTemplateAuthority)
    }
}

/// How a recipe-pack step previews before it is replayed.
///
/// A mutating step must preview before it applies, so a replay never surprises
/// the user with an effect they did not see first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayPreviewClass {
    /// A full preview of the effect is shown and confirmed before replay.
    FullPreviewBeforeReplay,
    /// A reviewable diff of the effect is shown before replay.
    DiffPreviewBeforeReplay,
    /// A dry run of the effect is shown before replay.
    DryRunPreviewBeforeReplay,
    /// The step is inspect-only, so no replay-time preview is needed.
    InspectOnlyNoPreviewNeeded,
    /// No preview is available, so replay must block until one exists.
    PreviewUnavailableMustBlock,
}

impl ReplayPreviewClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullPreviewBeforeReplay => "full_preview_before_replay",
            Self::DiffPreviewBeforeReplay => "diff_preview_before_replay",
            Self::DryRunPreviewBeforeReplay => "dry_run_preview_before_replay",
            Self::InspectOnlyNoPreviewNeeded => "inspect_only_no_preview_needed",
            Self::PreviewUnavailableMustBlock => "preview_unavailable_must_block",
        }
    }

    /// Whether this class previews the effect before replay applies it.
    pub const fn previews_before_replay(self) -> bool {
        matches!(
            self,
            Self::FullPreviewBeforeReplay
                | Self::DiffPreviewBeforeReplay
                | Self::DryRunPreviewBeforeReplay
        )
    }
}

/// How a recipe-pack step is recorded for later audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipeStepAuditClass {
    /// Audited into the shared run-record timeline.
    AuditedToRunRecordTimeline,
    /// Audited into the support export.
    AuditedToSupportExport,
    /// Audited into local history only.
    AuditedLocalHistoryOnly,
    /// Not audited.
    NotAudited,
}

impl RecipeStepAuditClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuditedToRunRecordTimeline => "audited_to_run_record_timeline",
            Self::AuditedToSupportExport => "audited_to_support_export",
            Self::AuditedLocalHistoryOnly => "audited_local_history_only",
            Self::NotAudited => "not_audited",
        }
    }

    /// Whether the step is audited anywhere.
    pub const fn is_audited(self) -> bool {
        !matches!(self, Self::NotAudited)
    }

    /// Whether the step is audited to a durable, exportable surface.
    pub const fn is_externally_auditable(self) -> bool {
        matches!(
            self,
            Self::AuditedToRunRecordTimeline | Self::AuditedToSupportExport
        )
    }
}

/// Reversibility of a recipe-pack step's effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipeStepReversibilityClass {
    /// The step produces no observable change.
    NoSideEffect,
    /// The step can be reversed inside the workspace.
    ReversibleInWorkspace,
    /// The step can be reversed by restoring a checkpoint.
    CheckpointReversible,
    /// The step publishes externally and cannot be reversed.
    IrreversibleExternalPublish,
}

impl RecipeStepReversibilityClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSideEffect => "no_side_effect",
            Self::ReversibleInWorkspace => "reversible_in_workspace",
            Self::CheckpointReversible => "checkpoint_reversible",
            Self::IrreversibleExternalPublish => "irreversible_external_publish",
        }
    }
}

/// Operational state of a recipe pack at mint time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecipePackStateClass {
    /// Admitted and ready to replay.
    Admitted,
    /// Admitted but awaiting first-use review before any material replay.
    PendingFirstUseReview,
    /// Blocked by policy.
    PolicyBlocked,
    /// Blocked by workspace trust.
    TrustBlocked,
    /// Quarantined because its signature could not be verified.
    QuarantinedSignature,
    /// Withdrawn from distribution.
    Withdrawn,
}

impl RecipePackStateClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::PendingFirstUseReview => "pending_first_use_review",
            Self::PolicyBlocked => "policy_blocked",
            Self::TrustBlocked => "trust_blocked",
            Self::QuarantinedSignature => "quarantined_signature",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Whether the state admits a new material replay.
    pub const fn admits_replay(self) -> bool {
        matches!(self, Self::Admitted)
    }

    /// Whether downstream surfaces must display a typed block reason.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::PolicyBlocked | Self::TrustBlocked | Self::QuarantinedSignature | Self::Withdrawn
        )
    }
}

/// One disclosed effect a recipe pack's steps can produce.
///
/// Each disclosure binds the side-effect class to how it previews before replay,
/// is gated, is audited, and is reversed, so a replay can never produce an effect
/// the pack did not disclose under first-party command rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipeStepDisclosure {
    /// Side-effect class this disclosure covers.
    pub side_effect_class: ToolSideEffectClass,
    /// How the effect previews before replay applies it.
    pub replay_preview: ReplayPreviewClass,
    /// Approval gate required before the effect applies.
    pub approval_posture: ToolApprovalPostureClass,
    /// How the effect is audited.
    pub audit: RecipeStepAuditClass,
    /// Reversibility of the effect.
    pub reversibility: RecipeStepReversibilityClass,
    /// Review-safe disclosure label shown to the user before replay applies.
    pub disclosure_label: String,
}

impl RecipeStepDisclosure {
    /// Whether this disclosure covers a mutating effect held to the
    /// first-party command preview, policy, and audit bar.
    pub fn is_mutating(&self) -> bool {
        self.side_effect_class.requires_approval_gate()
    }

    /// Whether this disclosure carries a real approval gate.
    pub fn has_approval_gate(&self) -> bool {
        self.approval_posture.requires_approval_gate() || self.approval_posture.denies_dispatch()
    }
}

/// One downgrade rule that narrows a recipe pack's claim when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipePackDowngradeRule {
    /// Trigger that fires this rule.
    pub trigger: M5AiWorkflowDowngradeTrigger,
    /// Qualification the pack narrows to when the trigger fires.
    pub narrowed_to: M5AiWorkflowQualificationClass,
    /// Whether tooling enforces this rule automatically.
    pub auto_enforced: bool,
    /// Review-safe rationale for the narrowing.
    pub rationale: String,
}

/// One signed, shareable recipe pack binding signature, share scope, automation
/// authority, and preview-first replay disclosures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipePackRow {
    /// Stable recipe-pack id.
    pub pack_id: String,
    /// Human-readable pack label.
    pub pack_label: String,
    /// Pack family label.
    pub pack_family_label: String,
    /// Pack version.
    pub pack_version: String,
    /// Content address of the manifest bytes, proving the exact pack a replay
    /// rode.
    pub manifest_content_address: String,
    /// Opaque ref to the matching automation descriptor pack, when one exists.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub descriptor_pack_ref: String,
    /// Source/publisher class.
    pub publisher_source_class: ToolPublisherSourceClass,
    /// Opaque ref to the signed publisher identity record.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub publisher_identity_ref: String,
    /// Signature class the pack is distributed under.
    pub signature_class: RecipePackSignatureClass,
    /// Share scope the pack is stored and distributed under.
    pub share_scope_class: RecipePackShareScopeClass,
    /// Provider/locality mode the pack resolves to.
    pub resolved_mode: RoutePolicyModeClass,
    /// Automation authority the pack has graduated to.
    pub automation_authority_class: AutomationAuthorityClass,
    /// Operational state at mint time.
    pub state: RecipePackStateClass,
    /// Qualification class claimed for this pack.
    pub claimed_qualification: M5AiWorkflowQualificationClass,
    /// Disclosed effects the pack's steps can produce.
    pub step_disclosures: Vec<RecipeStepDisclosure>,
    /// Downgrade rules that narrow the claim.
    pub downgrade_rules: Vec<RecipePackDowngradeRule>,
    /// Rollback posture for a pack-policy change.
    pub rollback_posture: M5AiWorkflowRollbackPosture,
    /// True when the rollback path has been drilled and verified.
    pub rollback_verified: bool,
    /// Required evidence packet refs backing the claim.
    pub evidence_packet_refs: Vec<String>,
    /// Review-safe explanation of the pack posture.
    pub explanation_label: String,
}

impl RecipePackRow {
    /// Whether this pack carries a publicly claimed qualification.
    ///
    /// Stable, Beta, and Preview are claimed lanes; Experimental, Held, and
    /// Unavailable are not.
    pub fn is_claimed(&self) -> bool {
        matches!(
            self.claimed_qualification,
            M5AiWorkflowQualificationClass::Stable
                | M5AiWorkflowQualificationClass::Beta
                | M5AiWorkflowQualificationClass::Preview
        )
    }

    /// Whether the pack discloses any mutating effect.
    pub fn has_mutating_step(&self) -> bool {
        self.step_disclosures
            .iter()
            .any(RecipeStepDisclosure::is_mutating)
    }

    /// Whether the pack discloses an irreversible external publish.
    pub fn has_irreversible_publish(&self) -> bool {
        self.step_disclosures.iter().any(|disclosure| {
            disclosure.side_effect_class == ToolSideEffectClass::ExternalIrreversiblePublish
        })
    }

    /// Whether every mutating step previews before it is replayed.
    ///
    /// This is the preview-first replay guarantee consumers project rather than
    /// re-deriving per surface.
    pub fn is_preview_first(&self) -> bool {
        self.step_disclosures
            .iter()
            .filter(|disclosure| disclosure.is_mutating())
            .all(|disclosure| disclosure.replay_preview.previews_before_replay())
    }

    /// The disclosure for `side_effect_class`, if present.
    pub fn disclosure(
        &self,
        side_effect_class: ToolSideEffectClass,
    ) -> Option<&RecipeStepDisclosure> {
        self.step_disclosures
            .iter()
            .find(|disclosure| disclosure.side_effect_class == side_effect_class)
    }

    /// Qualification this pack narrows to when `trigger` fires.
    ///
    /// Returns the claimed qualification unchanged when no rule matches; this is
    /// the deterministic downgrade automation consumers and release tooling
    /// project instead of re-deriving narrowing locally.
    pub fn narrowed_qualification(
        &self,
        trigger: M5AiWorkflowDowngradeTrigger,
    ) -> M5AiWorkflowQualificationClass {
        self.downgrade_rules
            .iter()
            .find(|rule| rule.trigger == trigger)
            .map(|rule| rule.narrowed_to)
            .unwrap_or(self.claimed_qualification)
    }

    /// Renders a deterministic, review-safe inspector card for this pack.
    pub fn render_inspector(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("### Recipe pack `{}`\n", self.pack_id));
        out.push_str(&format!("- Pack: `{}`\n", self.pack_label));
        out.push_str(&format!(
            "- Qualification: `{}`\n",
            self.claimed_qualification.as_str()
        ));
        out.push_str(&format!(
            "- Signature: `{}` / share `{}`\n",
            self.signature_class.as_str(),
            self.share_scope_class.as_str()
        ));
        out.push_str(&format!(
            "- Authority: `{}` / mode `{}`\n",
            self.automation_authority_class.as_str(),
            self.resolved_mode.as_str()
        ));
        out.push_str(&format!("- State: `{}`\n", self.state.as_str()));
        out.push_str(&format!(
            "- Manifest content address: `{}`\n",
            self.manifest_content_address
        ));
        out.push_str("- Step disclosures:\n");
        for disclosure in &self.step_disclosures {
            out.push_str(&format!(
                "  - `{}` / replay-preview `{}` / approval `{}` / audit `{}` / reversibility `{}` ({})\n",
                disclosure.side_effect_class.as_str(),
                disclosure.replay_preview.as_str(),
                disclosure.approval_posture.as_str(),
                disclosure.audit.as_str(),
                disclosure.reversibility.as_str(),
                disclosure.disclosure_label
            ));
        }
        out
    }
}

/// Proof freshness block for the recipe-pack graduation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipePackProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows claimed packs.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`RecipePackGraduationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecipePackGraduationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Recipe pack rows.
    pub packs: Vec<RecipePackRow>,
    /// Proof freshness block.
    pub proof_freshness: RecipePackProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe signed/shared recipe-pack graduation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecipePackGraduationPacket {
    /// Record kind; must equal [`RECIPE_PACK_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`RECIPE_PACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Recipe pack rows.
    pub packs: Vec<RecipePackRow>,
    /// Proof freshness block.
    pub proof_freshness: RecipePackProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RecipePackGraduationPacket {
    /// Builds a recipe-pack graduation packet from stable-lane input.
    pub fn new(input: RecipePackGraduationPacketInput) -> Self {
        Self {
            record_kind: RECIPE_PACK_RECORD_KIND.to_owned(),
            schema_version: RECIPE_PACK_SCHEMA_VERSION,
            packet_id: input.packet_id,
            catalogue_label: input.catalogue_label,
            packs: input.packs,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the recipe-pack graduation invariants.
    pub fn validate(&self) -> Vec<RecipePackViolation> {
        let mut violations = Vec::new();

        if self.record_kind != RECIPE_PACK_RECORD_KIND {
            violations.push(RecipePackViolation::WrongRecordKind);
        }
        if self.schema_version != RECIPE_PACK_SCHEMA_VERSION {
            violations.push(RecipePackViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.catalogue_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RecipePackViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_packs_present(self, &mut violations);
        for pack in &self.packs {
            validate_pack(pack, &mut violations);
        }
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_automation_material(
            &serde_json::to_value(self).expect("recipe pack packet serializes"),
        ) {
            violations.push(RecipePackViolation::RawAutomationMaterialInExport);
        }

        violations
    }

    /// Count of packs carrying a publicly claimed qualification.
    pub fn claimed_pack_count(&self) -> usize {
        self.packs.iter().filter(|p| p.is_claimed()).count()
    }

    /// Count of packs in a blocked state.
    pub fn blocked_pack_count(&self) -> usize {
        self.packs.iter().filter(|p| p.state.is_blocked()).count()
    }

    /// Count of packs disclosing a mutating effect.
    pub fn mutating_pack_count(&self) -> usize {
        self.packs.iter().filter(|p| p.has_mutating_step()).count()
    }

    /// Count of signed packs.
    pub fn signed_pack_count(&self) -> usize {
        self.packs
            .iter()
            .filter(|p| p.signature_class.is_signed())
            .count()
    }

    /// Returns the pack row for `pack_id`, if present.
    pub fn pack(&self, pack_id: &str) -> Option<&RecipePackRow> {
        self.packs.iter().find(|p| p.pack_id == pack_id)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("recipe pack packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# Signed And Shared Recipe Packs, Safe Automation Graduation, And Preview-First Replay\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalogue_label));
        out.push_str(&format!(
            "- Packs: {} ({} claimed, {} signed, {} mutating, {} blocked)\n",
            self.packs.len(),
            self.claimed_pack_count(),
            self.signed_pack_count(),
            self.mutating_pack_count(),
            self.blocked_pack_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Recipe pack inspectors\n\n");
        for pack in &self.packs {
            out.push_str(&pack.render_inspector());
            out.push('\n');
        }
        out
    }
}

/// Errors emitted when reading the checked-in recipe-pack export.
#[derive(Debug)]
pub enum RecipePackArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RecipePackViolation>),
}

impl fmt::Display for RecipePackArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "recipe pack export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "recipe pack export failed validation: {tokens}")
            }
        }
    }
}

impl Error for RecipePackArtifactError {}

/// Validation failures emitted by [`RecipePackGraduationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecipePackViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The packet carries no packs.
    NoPacks,
    /// A pack id appears more than once.
    DuplicatePack,
    /// A pack row is missing a required identity or label field.
    PackRowIncomplete,
    /// A pack is missing its manifest content address.
    PackMissingContentAddress,
    /// A signed pack is missing a publisher identity ref.
    SignedPackMissingPublisherIdentity,
    /// A pack shared beyond a local workspace is unsigned.
    SharedPackMustBeSigned,
    /// A managed-channel pack does not resolve to a managed mode.
    ManagedChannelScopeRequiresManagedMode,
    /// A managed-only template authority does not run on a managed mode.
    ManagedTemplateAuthorityRequiresManagedMode,
    /// A pack discloses no steps.
    PackMissingStepDisclosures,
    /// A step disclosure is missing its disclosure label.
    StepDisclosureIncomplete,
    /// A side-effect class is disclosed more than once.
    DuplicateStepDisclosure,
    /// A mutating step does not preview before replay.
    MutatingStepWithoutPreviewFirstReplay,
    /// A mutating step carries no approval gate.
    MutatingStepWithoutApproval,
    /// A mutating step is not audited.
    MutatingStepWithoutAudit,
    /// A pack with no authority discloses a mutating step.
    InspectOnlyAuthorityHasMutatingStep,
    /// A pack discloses an irreversible publish without admin-gated authority.
    IrreversiblePublishWithoutAdminAuthority,
    /// An irreversible external publish is not externally auditable.
    IrreversiblePublishNotExternallyAudited,
    /// A disclosure's reversibility disagrees with its side-effect class.
    StepReversibilityMismatch,
    /// A blocked pack still claims a public qualification.
    BlockedPackClaimsQualification,
    /// A pack pending first-use review claims Stable.
    PendingReviewClaimsStable,
    /// A claimed pack is missing required evidence packet refs.
    ClaimedPackMissingEvidence,
    /// A claimed pack's reversing rollback path is not verified.
    ClaimedRollbackUnverified,
    /// A pack has no downgrade rules.
    DowngradeRulesMissing,
    /// A pack's downgrade rules omit the proof-stale trigger.
    DowngradeRuleMissingProofStale,
    /// A pack's downgrade rules omit the provider-unavailable trigger.
    DowngradeRuleMissingProviderUnavailable,
    /// A downgrade rule does not narrow below the claimed qualification.
    DowngradeRuleNotNarrowing,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw automation material.
    RawAutomationMaterialInExport,
}

impl RecipePackViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NoPacks => "no_packs",
            Self::DuplicatePack => "duplicate_pack",
            Self::PackRowIncomplete => "pack_row_incomplete",
            Self::PackMissingContentAddress => "pack_missing_content_address",
            Self::SignedPackMissingPublisherIdentity => "signed_pack_missing_publisher_identity",
            Self::SharedPackMustBeSigned => "shared_pack_must_be_signed",
            Self::ManagedChannelScopeRequiresManagedMode => {
                "managed_channel_scope_requires_managed_mode"
            }
            Self::ManagedTemplateAuthorityRequiresManagedMode => {
                "managed_template_authority_requires_managed_mode"
            }
            Self::PackMissingStepDisclosures => "pack_missing_step_disclosures",
            Self::StepDisclosureIncomplete => "step_disclosure_incomplete",
            Self::DuplicateStepDisclosure => "duplicate_step_disclosure",
            Self::MutatingStepWithoutPreviewFirstReplay => {
                "mutating_step_without_preview_first_replay"
            }
            Self::MutatingStepWithoutApproval => "mutating_step_without_approval",
            Self::MutatingStepWithoutAudit => "mutating_step_without_audit",
            Self::InspectOnlyAuthorityHasMutatingStep => "inspect_only_authority_has_mutating_step",
            Self::IrreversiblePublishWithoutAdminAuthority => {
                "irreversible_publish_without_admin_authority"
            }
            Self::IrreversiblePublishNotExternallyAudited => {
                "irreversible_publish_not_externally_audited"
            }
            Self::StepReversibilityMismatch => "step_reversibility_mismatch",
            Self::BlockedPackClaimsQualification => "blocked_pack_claims_qualification",
            Self::PendingReviewClaimsStable => "pending_review_claims_stable",
            Self::ClaimedPackMissingEvidence => "claimed_pack_missing_evidence",
            Self::ClaimedRollbackUnverified => "claimed_rollback_unverified",
            Self::DowngradeRulesMissing => "downgrade_rules_missing",
            Self::DowngradeRuleMissingProofStale => "downgrade_rule_missing_proof_stale",
            Self::DowngradeRuleMissingProviderUnavailable => {
                "downgrade_rule_missing_provider_unavailable"
            }
            Self::DowngradeRuleNotNarrowing => "downgrade_rule_not_narrowing",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawAutomationMaterialInExport => "raw_automation_material_in_export",
        }
    }
}

/// Reads and validates the checked-in recipe-pack export.
pub fn current_recipe_pack_export() -> Result<RecipePackGraduationPacket, RecipePackArtifactError> {
    let packet: RecipePackGraduationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay/support_export.json"
    )))
    .map_err(RecipePackArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RecipePackArtifactError::Validation(violations))
    }
}

/// Ordinal rank used to compare qualification severity for downgrade rules.
///
/// Higher means a stronger public claim, so a downgrade must move to a strictly
/// lower rank.
fn qualification_rank(class: M5AiWorkflowQualificationClass) -> u8 {
    match class {
        M5AiWorkflowQualificationClass::Unavailable => 0,
        M5AiWorkflowQualificationClass::Held => 1,
        M5AiWorkflowQualificationClass::Experimental => 2,
        M5AiWorkflowQualificationClass::Preview => 3,
        M5AiWorkflowQualificationClass::Beta => 4,
        M5AiWorkflowQualificationClass::Stable => 5,
    }
}

fn mode_is_managed(mode: RoutePolicyModeClass) -> bool {
    matches!(
        mode,
        RoutePolicyModeClass::Managed | RoutePolicyModeClass::EnterpriseGateway
    )
}

fn validate_source_contracts(
    packet: &RecipePackGraduationPacket,
    violations: &mut Vec<RecipePackViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        RECIPE_PACK_SCHEMA_REF,
        RECIPE_PACK_DOC_REF,
        RECIPE_PACK_AUTOMATION_CONTRACT_REF,
        TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF,
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
        ROUTING_POLICY_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(RecipePackViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_packs_present(
    packet: &RecipePackGraduationPacket,
    violations: &mut Vec<RecipePackViolation>,
) {
    if packet.packs.is_empty() {
        violations.push(RecipePackViolation::NoPacks);
        return;
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for pack in &packet.packs {
        if !seen.insert(pack.pack_id.as_str()) {
            violations.push(RecipePackViolation::DuplicatePack);
        }
    }
}

fn validate_pack(pack: &RecipePackRow, violations: &mut Vec<RecipePackViolation>) {
    if pack.pack_id.trim().is_empty()
        || pack.pack_label.trim().is_empty()
        || pack.pack_family_label.trim().is_empty()
        || pack.pack_version.trim().is_empty()
        || pack.explanation_label.trim().is_empty()
    {
        violations.push(RecipePackViolation::PackRowIncomplete);
    }

    if pack.manifest_content_address.trim().is_empty() {
        violations.push(RecipePackViolation::PackMissingContentAddress);
    }

    validate_signing_and_scope(pack, violations);
    validate_step_disclosures(pack, violations);
    validate_authority(pack, violations);
    validate_claim_state(pack, violations);
    validate_downgrade_rules(pack, violations);
}

fn validate_signing_and_scope(pack: &RecipePackRow, violations: &mut Vec<RecipePackViolation>) {
    // A signed pack — or one whose publisher class always demands an identity —
    // must carry a publisher identity ref.
    if (pack.signature_class.is_signed()
        || pack.publisher_source_class.requires_publisher_identity())
        && pack.publisher_identity_ref.trim().is_empty()
    {
        violations.push(RecipePackViolation::SignedPackMissingPublisherIdentity);
    }

    // A pack distributed beyond a local workspace must be signed.
    if pack.share_scope_class.is_shared_beyond_workspace() && !pack.signature_class.is_signed() {
        violations.push(RecipePackViolation::SharedPackMustBeSigned);
    }

    // A managed-channel pack must resolve to a managed or enterprise-gateway mode
    // and carry organization or managed-channel signing authority.
    if pack.share_scope_class.requires_managed_channel()
        && (!mode_is_managed(pack.resolved_mode)
            || !pack.signature_class.carries_organization_authority())
    {
        violations.push(RecipePackViolation::ManagedChannelScopeRequiresManagedMode);
    }
}

fn validate_step_disclosures(pack: &RecipePackRow, violations: &mut Vec<RecipePackViolation>) {
    if pack.step_disclosures.is_empty() {
        violations.push(RecipePackViolation::PackMissingStepDisclosures);
        return;
    }

    let mut seen: BTreeSet<ToolSideEffectClass> = BTreeSet::new();
    for disclosure in &pack.step_disclosures {
        if !seen.insert(disclosure.side_effect_class) {
            violations.push(RecipePackViolation::DuplicateStepDisclosure);
        }

        if disclosure.disclosure_label.trim().is_empty() {
            violations.push(RecipePackViolation::StepDisclosureIncomplete);
        }

        // A mutating step is held to the same preview-first replay, policy, and
        // audit bar as a first-party command.
        if disclosure.is_mutating() {
            if !disclosure.replay_preview.previews_before_replay() {
                violations.push(RecipePackViolation::MutatingStepWithoutPreviewFirstReplay);
            }
            if !disclosure.has_approval_gate() {
                violations.push(RecipePackViolation::MutatingStepWithoutApproval);
            }
            if !disclosure.audit.is_audited() {
                violations.push(RecipePackViolation::MutatingStepWithoutAudit);
            }
        }

        // An irreversible external publish must be durably, exportably audited.
        if disclosure.side_effect_class == ToolSideEffectClass::ExternalIrreversiblePublish
            && !disclosure.audit.is_externally_auditable()
        {
            violations.push(RecipePackViolation::IrreversiblePublishNotExternallyAudited);
        }

        // The disclosed reversibility must agree with the effect class for the
        // two unambiguous cases.
        let reversibility_ok = match disclosure.side_effect_class {
            ToolSideEffectClass::InspectOnly => {
                disclosure.reversibility == RecipeStepReversibilityClass::NoSideEffect
            }
            ToolSideEffectClass::ExternalIrreversiblePublish => {
                disclosure.reversibility
                    == RecipeStepReversibilityClass::IrreversibleExternalPublish
            }
            _ => true,
        };
        if !reversibility_ok {
            violations.push(RecipePackViolation::StepReversibilityMismatch);
        }
    }
}

fn validate_authority(pack: &RecipePackRow, violations: &mut Vec<RecipePackViolation>) {
    // A pack with no authority may disclose no mutating step.
    if !pack.automation_authority_class.grants_mutation() && pack.has_mutating_step() {
        violations.push(RecipePackViolation::InspectOnlyAuthorityHasMutatingStep);
    }

    // An irreversible publish requires an admin-gated or managed-template
    // authority to graduate at all.
    if pack.has_irreversible_publish()
        && !pack
            .automation_authority_class
            .admits_irreversible_publish()
    {
        violations.push(RecipePackViolation::IrreversiblePublishWithoutAdminAuthority);
    }

    // A managed-only template authority must run on a managed or
    // enterprise-gateway mode.
    if pack.automation_authority_class.requires_managed_mode()
        && !mode_is_managed(pack.resolved_mode)
    {
        violations.push(RecipePackViolation::ManagedTemplateAuthorityRequiresManagedMode);
    }
}

fn validate_claim_state(pack: &RecipePackRow, violations: &mut Vec<RecipePackViolation>) {
    // A blocked pack narrows its claim instead of staying behind a public
    // qualification.
    if pack.is_claimed() && pack.state.is_blocked() {
        violations.push(RecipePackViolation::BlockedPackClaimsQualification);
    }

    // A pack still awaiting first-use review may not claim Stable.
    if pack.state == RecipePackStateClass::PendingFirstUseReview
        && pack.claimed_qualification == M5AiWorkflowQualificationClass::Stable
    {
        violations.push(RecipePackViolation::PendingReviewClaimsStable);
    }

    if pack.is_claimed() && pack.evidence_packet_refs.is_empty() {
        violations.push(RecipePackViolation::ClaimedPackMissingEvidence);
    }

    // A claimed pack whose pack-policy change can be reversed must have drilled
    // that reversal; a non-applicable posture carries no reversal.
    if pack.is_claimed()
        && pack.rollback_posture != M5AiWorkflowRollbackPosture::NotApplicable
        && !pack.rollback_verified
    {
        violations.push(RecipePackViolation::ClaimedRollbackUnverified);
    }
}

fn validate_downgrade_rules(pack: &RecipePackRow, violations: &mut Vec<RecipePackViolation>) {
    if pack.downgrade_rules.is_empty() {
        violations.push(RecipePackViolation::DowngradeRulesMissing);
        return;
    }

    if !pack
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProofStale)
    {
        violations.push(RecipePackViolation::DowngradeRuleMissingProofStale);
    }

    // Provider outages and quota exhaustion narrow through the
    // provider-unavailable trigger, so every row must carry it.
    if !pack
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProviderUnavailable)
    {
        violations.push(RecipePackViolation::DowngradeRuleMissingProviderUnavailable);
    }

    let claimed_rank = qualification_rank(pack.claimed_qualification);
    for rule in &pack.downgrade_rules {
        if qualification_rank(rule.narrowed_to) >= claimed_rank {
            violations.push(RecipePackViolation::DowngradeRuleNotNarrowing);
            break;
        }
    }
}

fn validate_proof_freshness(
    packet: &RecipePackGraduationPacket,
    violations: &mut Vec<RecipePackViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(RecipePackViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// Recipe packs are declarative and content-addressed: the support boundary
/// carries content addresses, classes, and review-safe labels only, never raw
/// shell fragments, raw filesystem paths, raw endpoint URLs, or credential
/// material.
fn json_contains_forbidden_automation_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_automation_material(text),
        serde_json::Value::Array(values) => values
            .iter()
            .any(json_contains_forbidden_automation_material),
        serde_json::Value::Object(map) => map
            .values()
            .any(json_contains_forbidden_automation_material),
        _ => false,
    }
}

fn contains_forbidden_automation_material(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("api_key=")
        || lower.contains("api-key=")
        || lower.contains("raw_api_key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
        || lower.contains("$(")
        || lower.contains("&& ")
}
