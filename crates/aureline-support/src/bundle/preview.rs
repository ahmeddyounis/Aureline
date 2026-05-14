//! Local preview builder for the support-bundle manifest.
//!
//! The shell consumes [`SupportBundlePreview`] before exposing any
//! share / upload step. The protected walk is:
//!
//! 1. Open support export.
//! 2. Preview bundle contents and redactions locally — read the
//!    [`SupportBundlePreview::manifest`] record and inspect the rows.
//! 3. Verify exact-build identity is captured before export.
//!
//! [`SupportBundlePreviewBuilder::write_preview_snapshot`] persists the
//! preview manifest to disk so a reviewer can reopen the same review
//! after a process restart without contacting a support service.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::exact_build::ExactBuildCapture;
use super::manifest::{
    ActionPolicySourceContext, ActionReconstructionContext, ActionabilityImpact,
    ActionabilityWarning, CollectionContext, ExcludedClass, FileSectionIdentity, HighRiskItemEntry,
    ParityBinding, PolicyContext, PolicyLock, PolicyNote, PreviewClassificationSummary,
    PreviewExportParity, Redaction, RedactionControl, RedactionReport, ReopenAfterExportPath,
    ReviewDecision, SecretScanSummary, SizeEstimate, SupportBundleManifest,
    SupportBundlePreviewItem, COLLECTION_SCHEMA_VERSION, SUPPORT_BUNDLE_MANIFEST_RECORD_KIND,
    SUPPORT_BUNDLE_PREVIEW_ITEM_RECORD_KIND, SUPPORT_BUNDLE_PREVIEW_ITEM_SCHEMA_VERSION,
};
use super::redaction::LocalFirstDefaults;
use super::vocabulary::{
    ActionabilityImpactClass, ActorClass, DiagnosticDataClass, HighRiskContentClass,
    PolicyNoteSeverity, RedactionState, ReviewDecidedByClass, ReviewDecisionClass, SecretScanState,
    TrustState,
};

/// Stable record-kind tag carried on every preview projection.
pub const SUPPORT_BUNDLE_PREVIEW_RECORD_KIND: &str = "support_bundle_preview_record";

/// Reviewer-facing notice quoted on every preview snapshot so the lane's
/// scope is not overstated.
pub const SUPPORT_BUNDLE_PREVIEW_SEED_SCOPE_NOTICE: &str =
    "Support-bundle preview seed: rows are minted from the local-first redaction profile, the \
     exact-build identity captured at preview time, and the canonical manifest schema. Live byte-\
     level redaction, upload transport, hosted intake, and ticket routing are reserved for a \
     later milestone.";

/// Caller-supplied seed for one preview row. The builder applies the
/// local-first defaults to derive redaction state, decision class,
/// excluded entries, and high-risk markers.
#[derive(Debug, Clone)]
pub struct PreviewItemSeed {
    pub support_pack_item_id: String,
    pub title: String,
    pub data_class: DiagnosticDataClass,
    pub high_risk_content_class: HighRiskContentClass,
    pub bundle_section_class: String,
    pub artifact_kind_class: String,
    pub manifest_path_ref: String,
    pub bundle_member_path_ref: Option<String>,
    pub source_refs: Vec<String>,
    pub size_estimate: SizeEstimate,
    pub impact_class: ActionabilityImpactClass,
    pub impact_summary: String,
    pub notes: String,
}

impl PreviewItemSeed {
    /// Stable preview-item id derived from the support-pack item id. The
    /// schema requires the id to start with `support.preview.item.` and
    /// the support-pack id to start with `support.item.`, so this swap
    /// keeps the preview row joinable to the matrix verbatim.
    fn preview_item_id(&self) -> String {
        let suffix = self
            .support_pack_item_id
            .strip_prefix("support.item.")
            .unwrap_or(self.support_pack_item_id.as_str());
        format!("support.preview.item.{suffix}")
    }

    /// Reviewer-visible high-risk label required by the schema whenever
    /// `data_class == high_risk`. Returns `None` for lower-risk rows.
    fn visible_high_risk_label(&self) -> Option<String> {
        if matches!(self.data_class, DiagnosticDataClass::HighRisk) {
            Some(format!(
                "High risk — {}",
                match self.high_risk_content_class {
                    HighRiskContentClass::SecretBearing => "secret-bearing material",
                    HighRiskContentClass::FullShellHistory => "full shell history",
                    HighRiskContentClass::RawDumpOrMemory => "raw dump or memory",
                    HighRiskContentClass::RawTraceOrTranscript => "raw trace or transcript",
                    HighRiskContentClass::PolicyProhibitedUnknown =>
                        "policy-prohibited unknown content",
                    HighRiskContentClass::NotApplicable => "high-risk content",
                }
            ))
        } else {
            None
        }
    }
}

/// Caller-supplied route and command reconstruction data for one preview row.
#[derive(Debug, Clone)]
pub struct ActionReconstructionSeed {
    /// Support-pack item id for the preview row that carries this context.
    pub support_pack_item_id: String,
    /// Command Aureline believed it was running.
    pub command_id: String,
    /// Descriptor or revision ref used by the command.
    pub command_descriptor_ref: String,
    /// Invocation session id recorded by the command lane.
    pub invocation_session_id: String,
    /// Target identity ref or typed absent/unknown token.
    pub target_identity_ref: String,
    /// Optional route-truth packet ref.
    pub action_route_packet_ref: Option<String>,
    /// Origin class from the action-route taxonomy.
    pub action_origin_class: String,
    /// Target class from the action-route taxonomy.
    pub action_target_class: String,
    /// Route class from the action-route taxonomy.
    pub action_route_class: String,
    /// Exposure class from the action-route taxonomy.
    pub action_exposure_class: String,
    /// Policy source that governed the action.
    pub policy_source: ActionPolicySourceContext,
    /// Redaction-safe summary for support/incident readers.
    pub route_summary: String,
    /// Optional link to the reviewed-command enforcement row.
    pub reviewed_enforcement_ref: Option<String>,
    /// Redaction class for the reconstruction context.
    pub redaction_class: String,
}

/// Read-only projection of a built support-bundle preview. The shell
/// renders rows and the export writer emits the manifest verbatim, so
/// both surfaces agree on what would leave the machine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportBundlePreview {
    pub record_kind: String,
    pub seed_scope_notice: String,
    pub manifest: SupportBundleManifest,
    pub honesty_marker_present: bool,
    pub preview_snapshot_ref: String,
}

impl SupportBundlePreview {
    /// True when the preview contains at least one row whose redaction
    /// state forced the seed to mint a `prohibited` chip and an
    /// excluded-class entry, or whose size estimate could not be
    /// computed. Drives the chrome's honesty banner.
    pub fn honesty_marker_present(&self) -> bool {
        self.honesty_marker_present
    }

    /// Convenience accessor for the rows the chrome should render in the
    /// preview pane.
    pub fn preview_items(&self) -> &[SupportBundlePreviewItem] {
        &self.manifest.preview_items
    }
}

/// Errors raised by the preview builder.
#[derive(Debug)]
pub enum SupportBundlePreviewError {
    /// At least one queued row carried [`DiagnosticDataClass::HighRisk`]
    /// without a non-`NotApplicable` high-risk subtype. The schema
    /// requires the subtype to be set explicitly so the manifest can
    /// label the row honestly.
    MissingHighRiskSubtype { support_pack_item_id: String },
    /// An action reconstruction seed cited a support-pack item id that
    /// was not queued as a preview row.
    ActionContextMissingPreviewItem { support_pack_item_id: String },
    /// The caller queued zero rows. The schema requires at least one
    /// preview item per manifest so the seed refuses to mint an empty
    /// bundle.
    EmptyPreview,
    /// I/O failed when persisting the preview snapshot.
    Io(io::Error),
    /// Serialization failed when persisting the preview snapshot.
    Serde(serde_json::Error),
}

impl std::fmt::Display for SupportBundlePreviewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingHighRiskSubtype {
                support_pack_item_id,
            } => write!(
                f,
                "preview row {support_pack_item_id} queued as high_risk but did not name a \
                 high_risk_content_class subtype"
            ),
            Self::ActionContextMissingPreviewItem {
                support_pack_item_id,
            } => write!(
                f,
                "action reconstruction context references {support_pack_item_id}, but no preview \
                 row with that support-pack item id was queued"
            ),
            Self::EmptyPreview => write!(f, "support-bundle preview must contain at least one row"),
            Self::Io(err) => write!(f, "support-bundle preview io error: {err}"),
            Self::Serde(err) => write!(f, "support-bundle preview serialization error: {err}"),
        }
    }
}

impl std::error::Error for SupportBundlePreviewError {}

impl From<io::Error> for SupportBundlePreviewError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for SupportBundlePreviewError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serde(err)
    }
}

/// Builder for a [`SupportBundlePreview`]. The chrome calls
/// [`SupportBundlePreviewBuilder::add_item`] once per queued row, then
/// [`SupportBundlePreviewBuilder::build`] to mint the manifest.
pub struct SupportBundlePreviewBuilder {
    bundle_id: String,
    title: String,
    generated_at: String,
    actor_class: ActorClass,
    policy_context: PolicyContext,
    collection_intent: String,
    exact_build: ExactBuildCapture,
    seeds: Vec<PreviewItemSeed>,
    action_contexts: Vec<ActionReconstructionSeed>,
}

impl SupportBundlePreviewBuilder {
    /// Build a new preview builder. `bundle_id` becomes the manifest's
    /// `support_bundle_id`. `title` is the reviewer-visible heading.
    /// `generated_at` is the RFC 3339 UTC timestamp the manifest records;
    /// the caller passes it in so tests can pin a deterministic value.
    pub fn new(
        bundle_id: impl Into<String>,
        title: impl Into<String>,
        generated_at: impl Into<String>,
        exact_build: ExactBuildCapture,
    ) -> Self {
        Self {
            bundle_id: bundle_id.into(),
            title: title.into(),
            generated_at: generated_at.into(),
            actor_class: ActorClass::SupportCenterPreview,
            policy_context: PolicyContext {
                policy_epoch: 1,
                trust_state: TrustState::Trusted,
                policy_bundle_ref: Some("policy-bundle:local-default:0001".into()),
            },
            collection_intent: "Preview a local support bundle before any share or upload step."
                .into(),
            exact_build,
            seeds: Vec::new(),
            action_contexts: Vec::new(),
        }
    }

    /// Override the actor class. Defaults to
    /// [`ActorClass::SupportCenterPreview`].
    pub fn with_actor_class(mut self, actor_class: ActorClass) -> Self {
        self.actor_class = actor_class;
        self
    }

    /// Override the policy context. Defaults to
    /// `policy_epoch=1, trust_state=trusted, policy_bundle_ref=policy-bundle:local-default:0001`.
    pub fn with_policy_context(mut self, policy_context: PolicyContext) -> Self {
        self.policy_context = policy_context;
        self
    }

    /// Override the collection intent string.
    pub fn with_collection_intent(mut self, collection_intent: impl Into<String>) -> Self {
        self.collection_intent = collection_intent.into();
        self
    }

    /// Queue one preview row.
    pub fn add_item(&mut self, seed: PreviewItemSeed) -> &mut Self {
        self.seeds.push(seed);
        self
    }

    /// Queue one action reconstruction context. The context must cite a
    /// support-pack item id already queued through [`Self::add_item`].
    pub fn add_action_reconstruction_context(
        &mut self,
        seed: ActionReconstructionSeed,
    ) -> &mut Self {
        self.action_contexts.push(seed);
        self
    }

    /// Mint the manifest and the read-only preview projection. Applies
    /// the local-first defaults, derives excluded-class entries for any
    /// row the defaults push out of export, and pins
    /// `prohibited_items_confirmed_absent` to the support-pack ids of
    /// every row marked as a prohibited high-risk class.
    pub fn build(self) -> Result<SupportBundlePreview, SupportBundlePreviewError> {
        if self.seeds.is_empty() {
            return Err(SupportBundlePreviewError::EmptyPreview);
        }

        for seed in &self.seeds {
            if matches!(seed.data_class, DiagnosticDataClass::HighRisk)
                && matches!(
                    seed.high_risk_content_class,
                    HighRiskContentClass::NotApplicable
                )
            {
                return Err(SupportBundlePreviewError::MissingHighRiskSubtype {
                    support_pack_item_id: seed.support_pack_item_id.clone(),
                });
            }
        }

        let mut preview_items: Vec<SupportBundlePreviewItem> = Vec::with_capacity(self.seeds.len());
        let mut review_decisions: Vec<ReviewDecision> = Vec::with_capacity(self.seeds.len());
        let mut excluded_classes: Vec<ExcludedClass> = Vec::new();
        let mut item_decision_refs: Vec<String> = Vec::with_capacity(self.seeds.len());
        let mut redaction_states_present: Vec<RedactionState> = Vec::new();
        let mut applied_rule_refs: Vec<String> = Vec::new();
        let mut high_risk_items: Vec<HighRiskItemEntry> = Vec::new();
        let mut prohibited_items: Vec<String> = Vec::new();
        let mut redaction_controls: Vec<RedactionControl> = Vec::with_capacity(self.seeds.len());
        let mut data_classes_present: Vec<DiagnosticDataClass> = Vec::new();
        let mut included_support_pack_item_ids: Vec<String> = Vec::new();
        let mut excluded_support_pack_item_ids: Vec<String> = Vec::new();
        let mut any_prohibited_row = false;

        for (index, seed) in self.seeds.iter().enumerate() {
            let posture =
                LocalFirstDefaults::posture_for(seed.data_class, seed.high_risk_content_class);

            // Track unique redaction states present and rule refs applied
            // so the manifest's redaction_report stays consistent with the
            // rows it summarizes.
            if !redaction_states_present.contains(&posture.redaction_state) {
                redaction_states_present.push(posture.redaction_state);
            }
            if !data_classes_present.contains(&seed.data_class) {
                data_classes_present.push(seed.data_class);
            }
            for rule_ref in &posture.rule_refs {
                let owned = (*rule_ref).to_owned();
                if !applied_rule_refs.contains(&owned) {
                    applied_rule_refs.push(owned);
                }
            }

            let preview_item_id = seed.preview_item_id();
            let review_decision_ref = format!("review_decisions[{index}]");
            let manifest_field_ref = format!("preview_items[{index}]");
            let item_digest_ref = format!("digest:preview-item:{}", seed.support_pack_item_id);

            let visible_high_risk_label = seed.visible_high_risk_label();
            let warning_required = matches!(seed.impact_class, ActionabilityImpactClass::High)
                || matches!(
                    seed.impact_class,
                    ActionabilityImpactClass::BlocksFirstActionableDiagnosis
                );

            let actionability = ActionabilityImpact {
                impact_class: seed.impact_class,
                affects_first_actionable_diagnosis: matches!(
                    seed.impact_class,
                    ActionabilityImpactClass::BlocksFirstActionableDiagnosis
                ),
                warning_required: false,
                warning_text: if warning_required {
                    Some(format!(
                        "Removing or further redacting {} reduces the chance of first actionable diagnosis.",
                        seed.title
                    ))
                } else {
                    None
                },
                impact_summary: seed.impact_summary.clone(),
            };

            let parity_binding = ParityBinding {
                support_pack_item_id: seed.support_pack_item_id.clone(),
                inclusion_rule_ref: format!("{}.included_by_default", seed.support_pack_item_id),
                export_manifest_field_ref: manifest_field_ref.clone(),
                preview_decision_ref: review_decision_ref.clone(),
                item_digest_ref: Some(item_digest_ref.clone()),
                exact_build_identity_refs: self.exact_build.exact_build_refs.clone(),
                post_export_reconstruction_fields: vec![
                    "preview_item_id".into(),
                    "parity_binding.support_pack_item_id".into(),
                    "redaction.data_class".into(),
                    "redaction.redaction_state".into(),
                    "materialization.collection_source_class".into(),
                    "materialization.digest_ref".into(),
                ],
            };

            let policy_lock = PolicyLock {
                locked_by_policy: false,
                reason_class: "not_locked".into(),
                policy_ref: None,
                reason_summary: "No policy lock applies in the local-first default profile.".into(),
            };

            let materialization_label = match posture.redaction_state {
                RedactionState::NotRequiredMetadata => "metadata manifest",
                RedactionState::OmittedPendingOptIn => "omitted pending opt-in",
                RedactionState::Prohibited => "prohibited — never exported",
                RedactionState::RetainedLocalOnly => "retained on local machine only",
                RedactionState::PolicyLocked => "policy locked",
                RedactionState::RedactedSummary => "redacted summary",
                RedactionState::SanitizedSnapshot => "sanitized snapshot",
            };
            let body_available_in_preview = matches!(
                posture.redaction_state,
                RedactionState::NotRequiredMetadata
                    | RedactionState::RedactedSummary
                    | RedactionState::SanitizedSnapshot
            );
            let storage_mode = match posture.redaction_state {
                RedactionState::NotRequiredMetadata => "embedded_export_copy",
                RedactionState::RetainedLocalOnly => "local_only_copy_retained",
                RedactionState::Prohibited | RedactionState::OmittedPendingOptIn => {
                    "intentionally_excluded"
                }
                RedactionState::PolicyLocked => "intentionally_excluded",
                RedactionState::RedactedSummary | RedactionState::SanitizedSnapshot => {
                    "embedded_export_copy"
                }
            };
            let embedding_state = match posture.redaction_state {
                RedactionState::NotRequiredMetadata
                | RedactionState::RedactedSummary
                | RedactionState::SanitizedSnapshot => "embedded",
                RedactionState::RetainedLocalOnly => "by_reference",
                RedactionState::Prohibited
                | RedactionState::OmittedPendingOptIn
                | RedactionState::PolicyLocked => "omitted",
            };

            let materialization = serde_json::json!({
                "support_export_posture": match posture.redaction_state {
                    RedactionState::NotRequiredMetadata => "included_by_default",
                    RedactionState::OmittedPendingOptIn => "opt_in_only",
                    RedactionState::Prohibited => "excluded_always",
                    RedactionState::RetainedLocalOnly => "excluded_by_default",
                    RedactionState::PolicyLocked => "excluded_by_default",
                    _ => "included_metadata_only",
                },
                "storage_mode": storage_mode,
                "embedding_state": embedding_state,
                "collection_source_class": "generated_by_exporter",
                "body_available_in_preview": body_available_in_preview,
                "representation_label": materialization_label,
                "digest_ref": item_digest_ref,
            });

            let deselectability = serde_json::json!({
                "can_deselect": !matches!(
                    posture.redaction_state,
                    RedactionState::Prohibited | RedactionState::PolicyLocked
                ),
                "deselection_rule_class": match posture.redaction_state {
                    RedactionState::Prohibited => "blocked_forbidden_marker_required",
                    RedactionState::PolicyLocked => "blocked_by_policy",
                    _ => "deselect_allowed_without_warning",
                },
                "stronger_redaction_available": false,
                "allowed_stronger_redaction_states": Vec::<&str>::new(),
                "locked_reason": match posture.redaction_state {
                    RedactionState::Prohibited => Some(
                        "Prohibited content stays out of the export under the local-first default profile."
                    ),
                    RedactionState::PolicyLocked => Some(
                        "Active policy locks this row out of the export."
                    ),
                    _ => None,
                },
                "warning_required_before_deselect": false,
            });

            let redaction = Redaction {
                data_class: seed.data_class,
                high_risk_content_class: seed.high_risk_content_class,
                redaction_class: match posture.redaction_state {
                    RedactionState::NotRequiredMetadata => "metadata_safe_default".into(),
                    _ => "internal_support_restricted".into(),
                },
                redaction_state: posture.redaction_state,
                visible_high_risk_label: visible_high_risk_label.clone(),
                redaction_rule_refs: posture.rule_refs.iter().map(|s| (*s).to_owned()).collect(),
                redaction_summary_ref: format!("redaction_report:{}", self.bundle_id),
            };

            redaction_controls.push(RedactionControl {
                control_id: format!("redaction-control:{}", seed.support_pack_item_id),
                preview_item_id: preview_item_id.clone(),
                support_pack_item_id: seed.support_pack_item_id.clone(),
                default_redaction_state: posture.redaction_state,
                selected_redaction_state: posture.redaction_state,
                allowed_narrower_states: allowed_narrower_states(posture.redaction_state),
                broadening_requires_review: true,
                raw_content_export_allowed: false,
                policy_locked: matches!(posture.redaction_state, RedactionState::PolicyLocked),
                control_note: control_note_for_state(posture.redaction_state).to_owned(),
            });

            let preview_item = SupportBundlePreviewItem {
                support_bundle_preview_item_schema_version:
                    SUPPORT_BUNDLE_PREVIEW_ITEM_SCHEMA_VERSION,
                record_kind: SUPPORT_BUNDLE_PREVIEW_ITEM_RECORD_KIND.to_owned(),
                preview_item_id: preview_item_id.clone(),
                title: seed.title.clone(),
                file_section_identity: FileSectionIdentity {
                    section_id: format!("support.preview.section.{}", seed.support_pack_item_id),
                    bundle_section_class: seed.bundle_section_class.clone(),
                    artifact_kind_class: seed.artifact_kind_class.clone(),
                    preview_label: seed.title.clone(),
                    manifest_path_ref: manifest_field_ref.clone(),
                    bundle_member_path_ref: seed.bundle_member_path_ref.clone(),
                    source_refs: seed.source_refs.clone(),
                },
                size_estimate: seed.size_estimate.clone(),
                redaction,
                materialization,
                deselectability,
                actionability_impact: actionability,
                parity_binding,
                policy_lock,
                notes: seed.notes.clone(),
            };

            preview_items.push(preview_item);

            let review_decision = ReviewDecision {
                preview_item_id: preview_item_id.clone(),
                support_pack_item_id: seed.support_pack_item_id.clone(),
                decision_class: posture.decision_class,
                selected_redaction_state: posture.redaction_state,
                decided_by_class: match posture.decision_class {
                    ReviewDecisionClass::OmittedProhibited => {
                        ReviewDecidedByClass::ProhibitedClassRule
                    }
                    _ => ReviewDecidedByClass::DefaultRule,
                },
                decision_reason: match posture.redaction_state {
                    RedactionState::NotRequiredMetadata => {
                        "Metadata-only row included by default.".into()
                    }
                    RedactionState::OmittedPendingOptIn => {
                        "Code-adjacent row omitted until reviewer opts in.".into()
                    }
                    RedactionState::Prohibited => {
                        "Prohibited row removed from export under the local-first default profile."
                            .into()
                    }
                    RedactionState::RetainedLocalOnly => {
                        "High-risk capture retained on the local machine only.".into()
                    }
                    RedactionState::PolicyLocked => {
                        "Row locked out of export by active policy.".into()
                    }
                    RedactionState::RedactedSummary => "Row exported as a redacted summary.".into(),
                    RedactionState::SanitizedSnapshot => {
                        "Row exported as a sanitized snapshot.".into()
                    }
                },
                actionability_warning_ack_ref: None,
            };
            review_decisions.push(review_decision);
            item_decision_refs.push(review_decision_ref);

            // Honesty drill: every row whose default posture pushes it
            // out of the export must show up as an excluded-class entry
            // and (for prohibited rows) on the prohibited-confirmed-
            // absent list with a high-risk-items annotation.
            if posture.is_excluded_from_export {
                if !excluded_support_pack_item_ids.contains(&seed.support_pack_item_id) {
                    excluded_support_pack_item_ids.push(seed.support_pack_item_id.clone());
                }
                if let Some(reason) = posture.exclusion_reason {
                    excluded_classes.push(ExcludedClass {
                        data_class: seed.data_class,
                        high_risk_content_class: seed.high_risk_content_class,
                        support_pack_item_id: Some(seed.support_pack_item_id.clone()),
                        artifact_kind_class: seed.artifact_kind_class.clone(),
                        exclusion_reason_class: reason,
                        explicit_reason: LocalFirstDefaults::explicit_reason_for(reason).to_owned(),
                        policy_ref: None,
                        omission_marker_ref: format!(
                            "omission-marker:{}",
                            seed.support_pack_item_id
                        ),
                    });
                }
            } else if !included_support_pack_item_ids.contains(&seed.support_pack_item_id) {
                included_support_pack_item_ids.push(seed.support_pack_item_id.clone());
            }
            if matches!(posture.redaction_state, RedactionState::Prohibited) {
                any_prohibited_row = true;
                if !prohibited_items.contains(&seed.support_pack_item_id) {
                    prohibited_items.push(seed.support_pack_item_id.clone());
                }
                high_risk_items.push(HighRiskItemEntry {
                    preview_item_id,
                    data_class: DiagnosticDataClass::HighRisk,
                    high_risk_content_class: seed.high_risk_content_class,
                    handling_summary: format!(
                        "Row was queued and rewritten to '{}' under the local-first default \
                         profile; raw bytes never enter the export.",
                        RedactionState::Prohibited.label()
                    ),
                });
            } else if matches!(seed.data_class, DiagnosticDataClass::HighRisk) {
                high_risk_items.push(HighRiskItemEntry {
                    preview_item_id,
                    data_class: DiagnosticDataClass::HighRisk,
                    high_risk_content_class: seed.high_risk_content_class,
                    handling_summary: format!(
                        "High-risk row materialized as '{}' under the local-first default profile.",
                        materialization_label
                    ),
                });
            }
        }

        if redaction_states_present.is_empty() {
            redaction_states_present.push(RedactionState::NotRequiredMetadata);
        }
        if applied_rule_refs.is_empty() {
            applied_rule_refs.push(super::redaction::RULE_REF_METADATA_CORE.to_owned());
        }

        let secret_scan_summary = SecretScanSummary {
            scan_state: if any_prohibited_row {
                SecretScanState::PassedWithRedactionMarkers
            } else if redaction_states_present
                .iter()
                .all(|s| matches!(s, RedactionState::NotRequiredMetadata))
            {
                SecretScanState::NotApplicableMetadataOnly
            } else {
                SecretScanState::PassedNoMarkers
            },
            detected_marker_count: prohibited_items.len() as u32,
            raw_secret_values_exported: false,
            notes: if any_prohibited_row {
                "At least one queued row was rewritten to 'prohibited'; raw bytes never enter the \
                 export."
                    .into()
            } else {
                "No prohibited rows queued in this preview.".into()
            },
        };

        let reviewer_visible_summary = if any_prohibited_row {
            LocalFirstDefaults::REVIEWER_SUMMARY_PROHIBITED_PRESENT.to_owned()
        } else {
            LocalFirstDefaults::REVIEWER_SUMMARY_DEFAULT_OK.to_owned()
        };

        let redaction_report = RedactionReport {
            report_id: format!("redaction_report:{}", self.bundle_id),
            redaction_profile_ref: LocalFirstDefaults::PROFILE_REF.to_owned(),
            redaction_pass_ref: format!("redaction-pass:{}:0001", self.bundle_id),
            redaction_states_present,
            high_risk_items,
            prohibited_items_confirmed_absent: prohibited_items,
            applied_rule_refs,
            secret_scan_summary,
            reviewer_visible_summary,
        };

        let retained_local_only_count = preview_items
            .iter()
            .filter(|item| {
                matches!(
                    item.redaction.redaction_state,
                    RedactionState::RetainedLocalOnly
                )
            })
            .count() as u32;
        let prohibited_count = preview_items
            .iter()
            .filter(|item| matches!(item.redaction.redaction_state, RedactionState::Prohibited))
            .count() as u32;
        let included_count = included_support_pack_item_ids.len() as u32;
        let excluded_count = excluded_support_pack_item_ids.len() as u32;
        let preview_classification_summary = PreviewClassificationSummary {
            included_count,
            excluded_count,
            retained_local_only_count,
            prohibited_count,
            data_classes_present,
            redaction_states_present: redaction_report.redaction_states_present.clone(),
            included_support_pack_item_ids,
            excluded_support_pack_item_ids,
            summary: format!(
                "{} preview row(s) included by default; {} row(s) excluded, retained locally, \
                 policy locked, or prohibited before export.",
                included_count, excluded_count
            ),
        };

        let mut action_reconstruction_contexts = Vec::with_capacity(self.action_contexts.len());
        for action_seed in &self.action_contexts {
            let Some(preview_item) = preview_items.iter().find(|item| {
                item.parity_binding.support_pack_item_id == action_seed.support_pack_item_id
            }) else {
                return Err(SupportBundlePreviewError::ActionContextMissingPreviewItem {
                    support_pack_item_id: action_seed.support_pack_item_id.clone(),
                });
            };

            action_reconstruction_contexts.push(ActionReconstructionContext {
                reconstruction_context_id: format!(
                    "action-reconstruction:{}:{}",
                    self.bundle_id, action_seed.support_pack_item_id
                ),
                preview_item_id: preview_item.preview_item_id.clone(),
                support_pack_item_id: action_seed.support_pack_item_id.clone(),
                command_id: action_seed.command_id.clone(),
                command_descriptor_ref: action_seed.command_descriptor_ref.clone(),
                invocation_session_id: action_seed.invocation_session_id.clone(),
                target_identity_ref: action_seed.target_identity_ref.clone(),
                action_route_packet_ref: action_seed.action_route_packet_ref.clone(),
                action_origin_class: action_seed.action_origin_class.clone(),
                action_target_class: action_seed.action_target_class.clone(),
                action_route_class: action_seed.action_route_class.clone(),
                action_exposure_class: action_seed.action_exposure_class.clone(),
                policy_source: action_seed.policy_source.clone(),
                route_summary: action_seed.route_summary.clone(),
                reviewed_enforcement_ref: action_seed.reviewed_enforcement_ref.clone(),
                exact_build_refs: self.exact_build.exact_build_refs.clone(),
                redaction_class: action_seed.redaction_class.clone(),
                raw_content_exported: false,
            });
        }

        let preview_snapshot_ref = format!("preview-snapshot:{}", self.bundle_id);

        let reopen_after_export_path = ReopenAfterExportPath {
            manifest_member_path: "support/manifest.json".into(),
            local_reopen_ref: format!("support-preview:{}", self.bundle_id),
            product_route_ref: format!("aureline://support/bundles/{}/preview", self.bundle_id),
            can_reopen_without_network: true,
            preserved_preview_snapshot_ref: preview_snapshot_ref.clone(),
            notes: "Reviewer can reopen the exact local preview snapshot from the on-disk \
                    manifest without network access."
                .into(),
        };

        let manifest_id = format!(
            "support.bundle.manifest.{}",
            self.bundle_id
                .strip_prefix("support-bundle:")
                .unwrap_or(self.bundle_id.as_str())
                .replace([':', '/'], ".")
        );

        let preview_export_parity = PreviewExportParity {
            preview_snapshot_ref: preview_snapshot_ref.clone(),
            preview_item_order_digest: format!("digest:preview-order:{}", self.bundle_id),
            export_manifest_digest: format!("digest:export-manifest:{}", self.bundle_id),
            manifest_reconstructs_shared_payload: true,
            reconstruction_fields: vec![
                "collection_schema_version".into(),
                "build_identity.exact_build_refs".into(),
                "preview_items[].preview_item_id".into(),
                "preview_items[].redaction".into(),
                "review_decisions[]".into(),
                "redaction_report".into(),
                "preview_classification_summary".into(),
                "redaction_controls[]".into(),
                "action_reconstruction_contexts[]".into(),
            ],
            item_decision_refs,
            unknown_field_policy: "preserve_unknown_unless_redacted".into(),
            parity_assertions: vec![
                "preview_items_match_export_manifest_order".into(),
                "exact_build_refs_preserved_after_export".into(),
                "prohibited_rows_never_carry_raw_bytes".into(),
            ],
        };

        let collection_context = CollectionContext {
            generated_at: self.generated_at.clone(),
            actor_class: self.actor_class,
            active_redaction_profile_ref: LocalFirstDefaults::PROFILE_REF.to_owned(),
            collection_intent: self.collection_intent.clone(),
            policy_context: self.policy_context.clone(),
            policy_notes: vec![PolicyNote {
                note_id: "support.preview.policy.metadata_only.local_default".into(),
                severity: PolicyNoteSeverity::Info,
                source_ref: super::redaction::RULE_REF_METADATA_CORE.into(),
                note: "Local default policy permits metadata-only build and policy truth while \
                       leaving high-risk classes absent."
                    .into(),
            }],
        };

        let actionability_warnings: Vec<ActionabilityWarning> = preview_items
            .iter()
            .filter_map(|item| {
                if item.actionability_impact.warning_text.is_some() {
                    Some(ActionabilityWarning {
                        warning_id: format!("warning:{}", item.preview_item_id),
                        preview_item_id: item.preview_item_id.clone(),
                        impact_class: item.actionability_impact.impact_class,
                        warning_text: item
                            .actionability_impact
                            .warning_text
                            .clone()
                            .unwrap_or_default(),
                        required_before_export: matches!(
                            item.actionability_impact.impact_class,
                            ActionabilityImpactClass::BlocksFirstActionableDiagnosis
                        ),
                        acknowledged_by_ref: None,
                    })
                } else {
                    None
                }
            })
            .collect();

        let manifest = SupportBundleManifest {
            collection_schema_version: COLLECTION_SCHEMA_VERSION,
            record_kind: SUPPORT_BUNDLE_MANIFEST_RECORD_KIND.to_owned(),
            manifest_id,
            support_bundle_id: self.bundle_id.clone(),
            title: self.title.clone(),
            build_identity: self.exact_build.to_build_identity(),
            collection_context,
            preview_items,
            review_decisions,
            excluded_classes,
            redaction_report,
            preview_classification_summary,
            redaction_controls,
            action_reconstruction_contexts,
            actionability_warnings,
            reopen_after_export_path,
            preview_export_parity,
            emitted_at: self.generated_at.clone(),
            notes: "Local preview minted by the support-bundle seed; rows mirror what would \
                    leave the machine on export."
                .into(),
        };

        let honesty_marker_present = any_prohibited_row
            || manifest
                .preview_items
                .iter()
                .any(|item| item.size_estimate.estimated_bytes.is_none());

        Ok(SupportBundlePreview {
            record_kind: SUPPORT_BUNDLE_PREVIEW_RECORD_KIND.to_owned(),
            seed_scope_notice: SUPPORT_BUNDLE_PREVIEW_SEED_SCOPE_NOTICE.to_owned(),
            manifest,
            honesty_marker_present,
            preview_snapshot_ref,
        })
    }

    /// Build the preview, then persist its manifest as a single JSON
    /// file. The shell uses this to make a preview reopenable across a
    /// process restart without contacting a support service.
    pub fn write_preview_snapshot(
        self,
        path: impl Into<PathBuf>,
    ) -> Result<SupportBundlePreview, SupportBundlePreviewError> {
        let preview = self.build()?;
        let path = path.into();
        write_preview_snapshot_to_path(&preview, &path)?;
        Ok(preview)
    }
}

fn write_preview_snapshot_to_path(
    preview: &SupportBundlePreview,
    path: &Path,
) -> Result<(), SupportBundlePreviewError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    let bytes = serde_json::to_vec_pretty(preview)?;
    fs::write(path, bytes)?;
    Ok(())
}

fn allowed_narrower_states(redaction_state: RedactionState) -> Vec<RedactionState> {
    match redaction_state {
        RedactionState::NotRequiredMetadata
        | RedactionState::RedactedSummary
        | RedactionState::SanitizedSnapshot => vec![
            RedactionState::RedactedSummary,
            RedactionState::RetainedLocalOnly,
        ],
        RedactionState::OmittedPendingOptIn => vec![RedactionState::RetainedLocalOnly],
        RedactionState::RetainedLocalOnly => vec![RedactionState::Prohibited],
        RedactionState::Prohibited | RedactionState::PolicyLocked => Vec::new(),
    }
}

fn control_note_for_state(redaction_state: RedactionState) -> &'static str {
    match redaction_state {
        RedactionState::NotRequiredMetadata => {
            "Reviewer may narrow metadata rows; broadening beyond metadata requires a reviewed path."
        }
        RedactionState::RedactedSummary | RedactionState::SanitizedSnapshot => {
            "Reviewer may retain or omit this row locally; raw content export is not available here."
        }
        RedactionState::OmittedPendingOptIn => {
            "Row is omitted by default; this alpha path only permits narrower local retention."
        }
        RedactionState::RetainedLocalOnly => {
            "Row is retained locally and can only be narrowed further to an omission marker."
        }
        RedactionState::Prohibited => {
            "Prohibited rows stay visible as omission markers and cannot be exported as raw content."
        }
        RedactionState::PolicyLocked => {
            "Active policy locks this row; the manifest records the lock instead of hiding it."
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bundle::vocabulary::ReleaseChannelClass;

    fn fixture_capture() -> ExactBuildCapture {
        ExactBuildCapture::for_fixture(
            "build-id:aureline:dev:0.0.0:x86_64-unknown-linux-gnu:debug:abcdef123456",
            "0.0.0",
            ReleaseChannelClass::DevLocal,
        )
    }

    fn metadata_seed() -> PreviewItemSeed {
        PreviewItemSeed {
            support_pack_item_id: "support.item.build_identity".into(),
            title: "Exact build and install identity".into(),
            data_class: DiagnosticDataClass::MetadataOnly,
            high_risk_content_class: HighRiskContentClass::NotApplicable,
            bundle_section_class: "build_and_install_truth".into(),
            artifact_kind_class: "exact_build_identity_manifest".into(),
            manifest_path_ref: "preview_items[0]".into(),
            bundle_member_path_ref: Some("manifest/build_identity.json".into()),
            source_refs: vec!["docs/build/exact_build_identity_model.md".into()],
            size_estimate: SizeEstimate {
                estimated_bytes: Some(4096),
                confidence_class: "estimated".into(),
                display_label: "4 KB".into(),
                size_source_class: "collector_estimate".into(),
            },
            impact_class: ActionabilityImpactClass::BlocksFirstActionableDiagnosis,
            impact_summary:
                "Without exact-build identity, support cannot match crashes, symbols, and docs."
                    .into(),
            notes: "Metadata-only; embedded by default.".into(),
        }
    }

    fn secret_seed() -> PreviewItemSeed {
        PreviewItemSeed {
            support_pack_item_id: "support.item.raw_secrets".into(),
            title: "Captured secret material".into(),
            data_class: DiagnosticDataClass::HighRisk,
            high_risk_content_class: HighRiskContentClass::SecretBearing,
            bundle_section_class: "logs_traces_and_manifests".into(),
            artifact_kind_class: "raw_secret_capture".into(),
            manifest_path_ref: "preview_items[1]".into(),
            bundle_member_path_ref: None,
            source_refs: vec!["docs/security/safe_preview_trust_classes.md".into()],
            size_estimate: SizeEstimate {
                estimated_bytes: Some(2048),
                confidence_class: "upper_bound".into(),
                display_label: "<= 2 KB".into(),
                size_source_class: "upper_bound_policy".into(),
            },
            impact_class: ActionabilityImpactClass::None,
            impact_summary:
                "Removing this row has no diagnostic cost because raw secret bytes never travel."
                    .into(),
            notes: "Caller queued raw secret material; the seed rewrites it to 'prohibited'."
                .into(),
        }
    }

    fn route_seed() -> PreviewItemSeed {
        PreviewItemSeed {
            support_pack_item_id: "support.item.execution_context_summary".into(),
            title: "Command and route reconstruction".into(),
            data_class: DiagnosticDataClass::EnvironmentAdjacent,
            high_risk_content_class: HighRiskContentClass::NotApplicable,
            bundle_section_class: "route_and_execution_truth".into(),
            artifact_kind_class: "action_route_truth_packet".into(),
            manifest_path_ref: "preview_items[1]".into(),
            bundle_member_path_ref: Some("manifest/route/command.json".into()),
            source_refs: vec!["docs/support/reconstruction_drill.md".into()],
            size_estimate: SizeEstimate {
                estimated_bytes: Some(4096),
                confidence_class: "estimated".into(),
                display_label: "4 KB".into(),
                size_source_class: "collector_estimate".into(),
            },
            impact_class: ActionabilityImpactClass::High,
            impact_summary:
                "Without this row, support cannot reconstruct command, target, route, or policy."
                    .into(),
            notes: "Metadata-only route truth; raw command arguments are excluded.".into(),
        }
    }

    fn action_context_seed() -> ActionReconstructionSeed {
        ActionReconstructionSeed {
            support_pack_item_id: "support.item.execution_context_summary".into(),
            command_id: "cmd:workspace.import_profile".into(),
            command_descriptor_ref: "cmd-rev:workspace.import_profile:alpha".into(),
            invocation_session_id: "inv:workspace.import_profile:fixture".into(),
            target_identity_ref: "target:local:workspace".into(),
            action_route_packet_ref: Some("route-packet:workspace.import_profile:fixture".into()),
            action_origin_class: "user_keystroke_local".into(),
            action_target_class: "local_host_target".into(),
            action_route_class: "approval_gated_route".into(),
            action_exposure_class: "workspace_visible_mutation".into(),
            policy_source: ActionPolicySourceContext {
                policy_source_ref: "policy-source:local-default:1".into(),
                policy_epoch: "1".into(),
                trust_state: "trusted".into(),
                policy_bundle_ref: Some("policy-bundle:local-default:0001".into()),
                source_class: "invocation_policy_context".into(),
            },
            route_summary:
                "Reviewed command route captures command, invocation, target, route, and policy."
                    .into(),
            reviewed_enforcement_ref: Some(
                "alpha-review-enforcement:cmd:workspace.import_profile".into(),
            ),
            redaction_class: "metadata_safe_default".into(),
        }
    }

    #[test]
    fn protected_walk_metadata_only_preview_carries_exact_build_identity() {
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:metadata-only:0001",
            "Metadata-only support bundle preview",
            "2026-05-10T05:10:00Z",
            fixture_capture(),
        );
        builder.add_item(metadata_seed());

        let preview = builder.build().expect("build preview");
        assert!(preview.manifest.has_exact_build_identity());
        assert!(!preview.honesty_marker_present);
        assert_eq!(preview.manifest.preview_items.len(), 1);
        assert_eq!(preview.manifest.excluded_classes.len(), 0);
        assert_eq!(
            preview
                .manifest
                .collection_context
                .active_redaction_profile_ref,
            LocalFirstDefaults::PROFILE_REF
        );
        let row = &preview.manifest.preview_items[0];
        assert_eq!(
            row.redaction.redaction_state,
            RedactionState::NotRequiredMetadata
        );
        assert!(row.redaction.visible_high_risk_label.is_none());
        assert_eq!(
            preview
                .manifest
                .preview_classification_summary
                .included_count,
            1
        );
        assert_eq!(preview.manifest.redaction_controls.len(), 1);
        assert!(
            !preview.manifest.redaction_controls[0].raw_content_export_allowed,
            "local-first controls must not expose raw-content export"
        );
    }

    #[test]
    fn action_reconstruction_context_records_command_route_policy_and_exact_build() {
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:action-context:0001",
            "Action reconstruction preview",
            "2026-05-10T05:15:00Z",
            fixture_capture(),
        );
        builder.add_item(metadata_seed());
        builder.add_item(route_seed());
        builder.add_action_reconstruction_context(action_context_seed());

        let preview = builder.build().expect("build action context preview");
        assert_eq!(preview.manifest.action_reconstruction_contexts.len(), 1);
        let context = &preview.manifest.action_reconstruction_contexts[0];
        assert_eq!(context.command_id, "cmd:workspace.import_profile");
        assert_eq!(
            context.invocation_session_id,
            "inv:workspace.import_profile:fixture"
        );
        assert_eq!(context.action_route_class, "approval_gated_route");
        assert_eq!(
            context.policy_source.source_class,
            "invocation_policy_context"
        );
        assert_eq!(context.exact_build_refs, fixture_capture().exact_build_refs);
        assert!(!context.raw_content_exported);
        assert!(preview
            .manifest
            .preview_export_parity
            .reconstruction_fields
            .iter()
            .any(|field| field == "action_reconstruction_contexts[]"));
    }

    #[test]
    fn failure_drill_secret_bearing_row_is_rewritten_to_prohibited_and_omitted() {
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:failure-drill:0001",
            "Failure drill: queued secret material is held back",
            "2026-05-10T05:11:00Z",
            fixture_capture(),
        );
        builder.add_item(metadata_seed());
        builder.add_item(secret_seed());

        let preview = builder.build().expect("build failure drill");
        assert!(preview.manifest.has_prohibited_row());
        assert!(preview.honesty_marker_present);

        let secret_row = preview
            .manifest
            .preview_items
            .iter()
            .find(|i| i.parity_binding.support_pack_item_id == "support.item.raw_secrets")
            .expect("secret row preserved in preview");
        assert_eq!(
            secret_row.redaction.redaction_state,
            RedactionState::Prohibited
        );
        assert!(secret_row.redaction.visible_high_risk_label.is_some());

        // The redaction report names the prohibited row.
        assert!(preview
            .manifest
            .redaction_report
            .prohibited_items_confirmed_absent
            .iter()
            .any(|id| id == "support.item.raw_secrets"));
        // And the manifest carries an excluded-class entry that points
        // at the same id with the typed reason.
        assert!(preview.manifest.excluded_classes.iter().any(|c| {
            c.support_pack_item_id.as_deref() == Some("support.item.raw_secrets")
                && matches!(
                    c.exclusion_reason_class,
                    super::super::vocabulary::ExcludedReasonClass::ProhibitedSecretOrToken
                )
        }));
        assert!(
            !preview
                .manifest
                .redaction_report
                .secret_scan_summary
                .raw_secret_values_exported
        );
    }

    #[test]
    fn write_preview_snapshot_round_trips_through_disk() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("preview.json");
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:roundtrip:0001",
            "Round-trip preview",
            "2026-05-10T05:12:00Z",
            fixture_capture(),
        );
        builder.add_item(metadata_seed());
        let preview = builder
            .write_preview_snapshot(&path)
            .expect("write preview");

        let bytes = std::fs::read(&path).expect("read preview");
        let parsed: SupportBundlePreview = serde_json::from_slice(&bytes).expect("parse preview");
        assert_eq!(parsed, preview);
        assert!(parsed.manifest.has_exact_build_identity());
    }

    #[test]
    fn empty_preview_is_rejected() {
        let builder = SupportBundlePreviewBuilder::new(
            "support-bundle:empty:0001",
            "Empty preview",
            "2026-05-10T05:13:00Z",
            fixture_capture(),
        );
        let result = builder.build();
        assert!(matches!(
            result,
            Err(SupportBundlePreviewError::EmptyPreview)
        ));
    }

    #[test]
    fn high_risk_without_subtype_is_rejected() {
        let mut builder = SupportBundlePreviewBuilder::new(
            "support-bundle:bad:0001",
            "Bad preview",
            "2026-05-10T05:14:00Z",
            fixture_capture(),
        );
        let mut bad = secret_seed();
        bad.high_risk_content_class = HighRiskContentClass::NotApplicable;
        builder.add_item(bad);
        let result = builder.build();
        assert!(matches!(
            result,
            Err(SupportBundlePreviewError::MissingHighRiskSubtype { .. })
        ));
    }
}
