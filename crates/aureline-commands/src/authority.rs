//! Command-truth and palette-authority conformance model.
//!
//! A [`CommandAuthorityScenarioRecord`] bundles one canonical command descriptor
//! with the way every claimed invocation surface (menu/button, keybinding,
//! palette, CLI/headless, AI tool, recipe, voice, browser companion) reaches that
//! command, plus the invocation-lineage join that a support export must be able to
//! reconstruct. It is the runtime counterpart to the one-command-graph promise:
//! instead of trusting generated help, a scenario proves — against the *same*
//! canonical descriptor — that no surface widens authority, suppresses a preview
//! or approval requirement, lies about its automation labels, or breaks the
//! command-id → invocation → result → evidence → notification → rollback chain.
//!
//! [`CommandAuthorityScenarioRecord::validate`] rejects any scenario that breaks
//! one of those invariants with a descriptive error; conformance corpora pin the
//! expected substring so a regression cannot pass silently.
//! [`CommandAuthorityScenarioRecord::project`] produces the support-safe
//! [`CommandAuthorityProjection`] consumed by the parity report, the release
//! evidence packet, and other harnesses.

use serde::{Deserialize, Serialize};

use crate::descriptor::{CommandDescriptorRecord, CommandId};

/// Stable surface-class vocabulary for command invocation surfaces.
///
/// These are the claimed invocation surfaces a single command graph must keep in
/// parity. UI surfaces drive a person; automation surfaces drive a script, agent,
/// or companion. The split matters for automation-label honesty.
pub mod surface_class {
    /// Application menu entry or toolbar button.
    pub const MENU_OR_BUTTON: &str = "menu_or_button";
    /// Keybinding / chord.
    pub const KEYBINDING: &str = "keybinding";
    /// Command palette row.
    pub const COMMAND_PALETTE: &str = "command_palette";
    /// CLI or headless invocation.
    pub const CLI_HEADLESS: &str = "cli_headless";
    /// AI tool-call surface.
    pub const AI_TOOL: &str = "ai_tool";
    /// Recipe / macro automation step.
    pub const RECIPE: &str = "recipe";
    /// Voice / dictation command surface.
    pub const VOICE: &str = "voice";
    /// Browser-companion surface.
    pub const BROWSER_COMPANION: &str = "browser_companion";

    /// Surfaces that drive automation rather than a person at a UI.
    pub const NON_UI_SURFACES: &[&str] = &[CLI_HEADLESS, AI_TOOL, RECIPE];

    /// Returns true when the surface class is a recognized invocation surface.
    pub fn is_known(value: &str) -> bool {
        matches!(
            value,
            MENU_OR_BUTTON
                | KEYBINDING
                | COMMAND_PALETTE
                | CLI_HEADLESS
                | AI_TOOL
                | RECIPE
                | VOICE
                | BROWSER_COMPANION
        )
    }

    /// Returns true when the surface drives automation rather than a UI user.
    pub fn is_non_ui(value: &str) -> bool {
        NON_UI_SURFACES.contains(&value)
    }
}

/// Frozen automation-label vocabulary the descriptor may advertise.
pub mod automation_label {
    /// Safe to record and replay inside a UI macro.
    pub const MACRO_SAFE: &str = "macro_safe";
    /// Safe to invoke from a recipe automation step.
    pub const RECIPE_SAFE: &str = "recipe_safe";
    /// Safe to invoke from CLI / headless contexts.
    pub const HEADLESS_SAFE: &str = "headless_safe";
    /// Only invokable from an interactive UI surface.
    pub const UI_ONLY: &str = "ui_only";
    /// Always requires an explicit approval, regardless of surface.
    pub const APPROVAL_REQUIRED: &str = "approval_required";
}

/// One claimed invocation surface for a command, with the decision and posture it
/// reports for the scenario's runtime context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceInvocationRecord {
    /// Surface class (see [`surface_class`]).
    pub surface_class: String,
    /// Enablement decision class the surface reports
    /// (`enabled` / `disabled_with_reason` / `hidden_with_reason`).
    pub enablement_decision_class: String,
    /// Disabled-reason code when the surface is not enabled.
    #[serde(default)]
    pub disabled_reason_code: Option<String>,
    /// Preview class the surface declares before dispatch. Must equal the
    /// descriptor's `preview_class`; a weaker value suppresses the requirement.
    pub preview_class_declared: String,
    /// Approval posture the surface declares before dispatch. Must equal the
    /// descriptor's `approval_posture_class`.
    pub approval_posture_class_declared: String,
    /// Authority class the surface claims for the invocation.
    pub authority_class: String,
    /// Whether the surface actually dispatched the command.
    pub dispatched: bool,
    /// Result/outcome code the surface reports (dispatched surfaces only).
    #[serde(default)]
    pub outcome_code: Option<String>,
    /// Invocation-session id minted for this surface; joins to the lineage.
    pub invocation_session_id: String,
    /// Canonical command id an alias resolved to, when an alias was used.
    #[serde(default)]
    pub resolves_to_canonical_command_id: Option<CommandId>,
}

impl SurfaceInvocationRecord {
    /// Returns true when the surface reports the command as enabled.
    pub fn is_enabled(&self) -> bool {
        self.enablement_decision_class == "enabled"
    }
}

/// The invocation-lineage join a support export must be able to reconstruct from
/// a command id alone: command → result packet → evidence ref → notification /
/// activity → rollback handle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvocationLineageRecord {
    /// Canonical command id the lineage belongs to.
    pub command_id: CommandId,
    /// Invocation-session id that produced the result packet.
    pub invocation_session_id: String,
    /// Result-packet id.
    pub result_packet_id: String,
    /// Outcome code recorded in the result packet.
    pub result_outcome_code: String,
    /// Evidence refs emitted by the invocation.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Notification refs joined to the invocation.
    #[serde(default)]
    pub notification_refs: Vec<String>,
    /// Activity-center refs joined to the invocation.
    #[serde(default)]
    pub activity_refs: Vec<String>,
    /// Rollback-handle posture (`reversible_handle` / `not_reversible_by_contract`).
    pub rollback_handle_posture: String,
    /// Rollback-handle id when the invocation is reversible.
    #[serde(default)]
    pub rollback_handle_id: Option<String>,
    /// Support-bundle ref the lineage is exportable into.
    #[serde(default)]
    pub support_bundle_ref: Option<String>,
}

/// A command-truth and palette-authority conformance scenario.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandAuthorityScenarioRecord {
    /// Boundary record kind (`command_authority_scenario_record`).
    pub record_kind: String,
    /// Schema version for this scenario boundary.
    pub schema_version: u32,
    /// Stable scenario id.
    pub scenario_id: String,
    /// The canonical command descriptor every surface must project from.
    pub canonical_descriptor: CommandDescriptorRecord,
    /// Claimed invocation surfaces for this command.
    pub surfaces: Vec<SurfaceInvocationRecord>,
    /// The invocation-lineage join produced by the dispatched surface.
    pub lineage: InvocationLineageRecord,
}

/// Support-safe projection of a validated scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandAuthorityProjection {
    /// Canonical command id.
    pub command_id: CommandId,
    /// Dotted machine verb.
    pub canonical_verb: String,
    /// Descriptor lifecycle state.
    pub lifecycle_state: String,
    /// Declared preview class.
    pub preview_class: String,
    /// Declared approval posture class.
    pub approval_posture_class: String,
    /// Automation labels advertised by the descriptor.
    pub automation_labels: Vec<String>,
    /// Distinct surface classes covered by the scenario, sorted.
    pub surface_classes_covered: Vec<String>,
    /// Canonical enablement decision class agreed by every surface.
    pub agreed_enablement_decision_class: String,
    /// True when no surface widened authority or suppressed preview/approval.
    pub parity_clean: bool,
    /// True when the lineage chain reconstructs end-to-end.
    pub lineage_complete: bool,
    /// True when the command's effect class requires a reversible rollback handle.
    pub rollback_required: bool,
    /// Ordered lineage chain refs a support export can replay.
    pub lineage_chain: Vec<String>,
}

const LIFECYCLE_STATES_REQUIRING_CONTRACT: &[&str] =
    &["beta", "stable", "lts_facing", "deprecated"];

const CAPABILITY_CLASSES_REQUIRING_ROLLBACK: &[&str] = &[
    "recoverable_durable_mutation",
    "externally_visible_mutation",
    "credential_or_secret_bearing",
    "managed_workspace_control",
    "policy_authoring_or_waiver",
    "destructive_bulk_mutation",
    "irreversible_high_blast_mutation",
    "irreversible_publish",
];

impl CommandAuthorityScenarioRecord {
    /// Validates the scenario against the canonical command descriptor.
    ///
    /// Returns `Err(message)` describing the first broken invariant. The message
    /// is the stable substring conformance corpora pin for negative drills.
    pub fn validate(&self) -> Result<(), String> {
        if self.record_kind != "command_authority_scenario_record" {
            return Err(format!(
                "record_kind must be command_authority_scenario_record, got `{}`",
                self.record_kind
            ));
        }
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported command authority scenario schema_version {}",
                self.schema_version
            ));
        }
        if self.scenario_id.trim().is_empty() {
            return Err("scenario_id must be non-empty".to_string());
        }

        let descriptor = &self.canonical_descriptor;
        descriptor
            .validate_minimal()
            .map_err(|detail| format!("canonical descriptor invalid: {detail}"))?;

        self.validate_machine_readable_contract()?;
        self.validate_alias_canonical_resolution()?;
        self.validate_surfaces()?;
        self.validate_lineage()?;
        Ok(())
    }

    /// Stable / claimed commands MUST carry machine-readable lifecycle and
    /// automation metadata, so docs/help and migration aliases stay generated
    /// from the canonical descriptor instead of hand-maintained shadow data.
    fn validate_machine_readable_contract(&self) -> Result<(), String> {
        let descriptor = &self.canonical_descriptor;
        if !LIFECYCLE_STATES_REQUIRING_CONTRACT.contains(&descriptor.lifecycle_state.as_str()) {
            return Ok(());
        }

        if descriptor.automation_labels.is_empty() {
            return Err(format!(
                "stable command {} is missing machine-readable automation metadata",
                descriptor.command_id
            ));
        }
        if descriptor.category_refs.is_empty() {
            return Err(format!(
                "stable command {} is missing machine-readable category metadata",
                descriptor.command_id
            ));
        }
        if descriptor.discoverability_record_refs.is_empty() {
            return Err(format!(
                "stable command {} is missing machine-readable discoverability metadata",
                descriptor.command_id
            ));
        }
        if descriptor
            .invocation_schema_ref
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
        {
            return Err(format!(
                "stable command {} is missing machine-readable invocation schema ref",
                descriptor.command_id
            ));
        }
        if descriptor
            .result_schema_ref
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
        {
            return Err(format!(
                "stable command {} is missing machine-readable result schema ref",
                descriptor.command_id
            ));
        }
        Ok(())
    }

    /// Every alias the descriptor advertises MUST resolve to the canonical
    /// command id, so migration aliases never become shadow commands.
    fn validate_alias_canonical_resolution(&self) -> Result<(), String> {
        let descriptor = &self.canonical_descriptor;
        for alias in &descriptor.aliases {
            match alias.canonical_command_id.as_deref() {
                Some(target) if target == descriptor.command_id => {}
                _ => {
                    return Err(format!(
                        "alias {} does not resolve to canonical command id {}",
                        alias.alias_id, descriptor.command_id
                    ));
                }
            }
        }
        Ok(())
    }

    /// Cross-surface parity: every surface gets the same enablement decision and
    /// may neither suppress the preview/approval requirement nor widen authority.
    fn validate_surfaces(&self) -> Result<(), String> {
        let descriptor = &self.canonical_descriptor;
        if self.surfaces.is_empty() {
            return Err(format!(
                "command {} declares no invocation surface to prove",
                descriptor.command_id
            ));
        }

        let canonical_decision = &self.surfaces[0].enablement_decision_class;
        let canonical_reason = &self.surfaces[0].disabled_reason_code;

        for surface in &self.surfaces {
            if !surface_class::is_known(&surface.surface_class) {
                return Err(format!(
                    "unknown invocation surface class `{}`",
                    surface.surface_class
                ));
            }

            // Same enablement decision across every surface.
            if &surface.enablement_decision_class != canonical_decision
                || &surface.disabled_reason_code != canonical_reason
            {
                return Err(format!(
                    "surface {} diverges from the canonical enablement decision for {}",
                    surface.surface_class, descriptor.command_id
                ));
            }

            // Preview / approval requirements may not be suppressed.
            if surface.preview_class_declared != descriptor.preview_class {
                return Err(format!(
                    "surface {} suppresses the preview requirement for {} \
                     (declared `{}`, descriptor `{}`)",
                    surface.surface_class,
                    descriptor.command_id,
                    surface.preview_class_declared,
                    descriptor.preview_class
                ));
            }
            if surface.approval_posture_class_declared != descriptor.approval_posture_class {
                return Err(format!(
                    "surface {} suppresses the approval requirement for {} \
                     (declared `{}`, descriptor `{}`)",
                    surface.surface_class,
                    descriptor.command_id,
                    surface.approval_posture_class_declared,
                    descriptor.approval_posture_class
                ));
            }

            // Alias resolution stays canonical on every surface.
            if let Some(target) = surface.resolves_to_canonical_command_id.as_deref() {
                if target != descriptor.command_id {
                    return Err(format!(
                        "surface {} resolves to a non-canonical command id `{}` for {}",
                        surface.surface_class, target, descriptor.command_id
                    ));
                }
            }

            // Authority widening / automation-label honesty for enabled surfaces.
            if surface.is_enabled() {
                self.validate_surface_authority(surface)?;
            }
        }

        self.validate_automation_label_honesty()?;
        Ok(())
    }

    /// An enabled surface may not reach a command its descriptor does not admit
    /// onto that surface class.
    fn validate_surface_authority(&self, surface: &SurfaceInvocationRecord) -> Result<(), String> {
        let descriptor = &self.canonical_descriptor;
        let labels = &descriptor.automation_labels;

        // A UI-only command may not be reached from an automation surface at all.
        if labels.iter().any(|l| l == automation_label::UI_ONLY)
            && surface_class::is_non_ui(&surface.surface_class)
        {
            return Err(format!(
                "ui_only command {} exposes a non-UI automation surface ({})",
                descriptor.command_id, surface.surface_class
            ));
        }

        match surface.surface_class.as_str() {
            surface_class::CLI_HEADLESS => {
                if !labels.iter().any(|l| l == automation_label::HEADLESS_SAFE) {
                    return Err(format!(
                        "surface cli_headless widens authority for {}: command is not headless_safe",
                        descriptor.command_id
                    ));
                }
            }
            surface_class::RECIPE => {
                if !labels.iter().any(|l| l == automation_label::RECIPE_SAFE) {
                    return Err(format!(
                        "surface recipe widens authority for {}: command is not recipe_safe",
                        descriptor.command_id
                    ));
                }
            }
            surface_class::AI_TOOL => {
                if descriptor.ai_tool_surfacing_class == "not_ai_callable" {
                    return Err(format!(
                        "surface ai_tool widens authority for {}: command is not ai_callable",
                        descriptor.command_id
                    ));
                }
            }
            surface_class::VOICE | surface_class::BROWSER_COMPANION => {
                if !descriptor
                    .client_scopes
                    .iter()
                    .any(|scope| scope == "companion_surface")
                {
                    return Err(format!(
                        "surface {} widens authority for {}: command is not exposed to the companion surface",
                        surface.surface_class, descriptor.command_id
                    ));
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Automation labels must stay truthful relative to the descriptor and the
    /// surfaces the scenario actually exercises.
    fn validate_automation_label_honesty(&self) -> Result<(), String> {
        let descriptor = &self.canonical_descriptor;
        let labels = &descriptor.automation_labels;

        let declares = |name: &str| labels.iter().any(|l| l == name);

        // approval_required label and descriptor approval posture must agree.
        let approval_label = declares(automation_label::APPROVAL_REQUIRED);
        let approval_posture = descriptor.approval_posture_class != "no_approval_required";
        if approval_label != approval_posture {
            return Err(format!(
                "automation label approval_required disagrees with the descriptor approval posture for {}",
                descriptor.command_id
            ));
        }

        // ui_only forbids advertising any automation label. (Reaching the command
        // from a non-UI surface is rejected per-surface in
        // [`validate_surface_authority`].)
        if declares(automation_label::UI_ONLY)
            && (declares(automation_label::HEADLESS_SAFE)
                || declares(automation_label::RECIPE_SAFE)
                || declares(automation_label::MACRO_SAFE))
        {
            return Err(format!(
                "ui_only command {} also advertises an automation label",
                descriptor.command_id
            ));
        }
        Ok(())
    }

    /// The lineage must reconstruct command → invocation → result → evidence →
    /// notification/activity → rollback so a support export can replay it.
    fn validate_lineage(&self) -> Result<(), String> {
        let descriptor = &self.canonical_descriptor;
        let lineage = &self.lineage;

        if lineage.command_id != descriptor.command_id {
            return Err(format!(
                "lineage command_id `{}` drifts from canonical command id `{}`",
                lineage.command_id, descriptor.command_id
            ));
        }
        if lineage.invocation_session_id.trim().is_empty() {
            return Err("lineage is missing an invocation_session_id".to_string());
        }
        if lineage.result_packet_id.trim().is_empty() {
            return Err("lineage is missing a result_packet_id".to_string());
        }

        // The lineage session must join a dispatched surface.
        let joined = self
            .surfaces
            .iter()
            .any(|s| s.dispatched && s.invocation_session_id == lineage.invocation_session_id);
        if !joined {
            return Err(format!(
                "lineage invocation_session_id `{}` is not joined to any dispatched surface",
                lineage.invocation_session_id
            ));
        }

        if lineage.evidence_refs.is_empty() {
            return Err(format!(
                "support export cannot reconstruct lineage for {} without an evidence ref",
                descriptor.command_id
            ));
        }
        if lineage.notification_refs.is_empty() && lineage.activity_refs.is_empty() {
            return Err(format!(
                "support export cannot reconstruct lineage for {} without a notification or activity row",
                descriptor.command_id
            ));
        }

        if self.rollback_required() {
            if lineage.rollback_handle_posture != "reversible_handle" {
                return Err(format!(
                    "durable command {} must expose a reversible rollback handle for lineage",
                    descriptor.command_id
                ));
            }
            if lineage
                .rollback_handle_id
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
            {
                return Err(format!(
                    "durable command {} must carry a rollback_handle_id for lineage",
                    descriptor.command_id
                ));
            }
        }
        Ok(())
    }

    /// True when the descriptor's effect class requires a reversible rollback
    /// handle to reconstruct lineage.
    ///
    /// A rollback handle is required only when a durable-effect command actually
    /// applied a mutation; a denied, blocked, or cancelled invocation changed
    /// nothing and therefore needs no rollback handle.
    pub fn rollback_required(&self) -> bool {
        let durable = CAPABILITY_CLASSES_REQUIRING_ROLLBACK
            .contains(&self.canonical_descriptor.capability_scope_class.as_str());
        let applied = matches!(
            self.lineage.result_outcome_code.as_str(),
            "succeeded" | "succeeded_with_warnings" | "partially_applied"
        );
        durable && applied
    }

    /// Projects the support-safe conformance result. Callers should
    /// [`validate`](Self::validate) first; `project` reflects the record as-is.
    pub fn project(&self) -> CommandAuthorityProjection {
        let descriptor = &self.canonical_descriptor;
        let lineage = &self.lineage;

        let mut surface_classes_covered: Vec<String> = self
            .surfaces
            .iter()
            .map(|s| s.surface_class.clone())
            .collect();
        surface_classes_covered.sort();
        surface_classes_covered.dedup();

        let agreed_enablement_decision_class = self
            .surfaces
            .first()
            .map(|s| s.enablement_decision_class.clone())
            .unwrap_or_default();

        let parity_clean = self.validate_surfaces().is_ok();
        let lineage_complete = self.validate_lineage().is_ok();

        let mut lineage_chain = vec![
            lineage.command_id.clone(),
            lineage.invocation_session_id.clone(),
            lineage.result_packet_id.clone(),
        ];
        lineage_chain.extend(lineage.evidence_refs.iter().cloned());
        lineage_chain.extend(lineage.notification_refs.iter().cloned());
        lineage_chain.extend(lineage.activity_refs.iter().cloned());
        if let Some(handle) = lineage.rollback_handle_id.as_ref() {
            lineage_chain.push(handle.clone());
        }
        if let Some(bundle) = lineage.support_bundle_ref.as_ref() {
            lineage_chain.push(bundle.clone());
        }

        CommandAuthorityProjection {
            command_id: descriptor.command_id.clone(),
            canonical_verb: descriptor.canonical_verb.clone(),
            lifecycle_state: descriptor.lifecycle_state.clone(),
            preview_class: descriptor.preview_class.clone(),
            approval_posture_class: descriptor.approval_posture_class.clone(),
            automation_labels: descriptor.automation_labels.clone(),
            surface_classes_covered,
            agreed_enablement_decision_class,
            parity_clean,
            lineage_complete,
            rollback_required: self.rollback_required(),
            lineage_chain,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::descriptor::{
        AccessibilityLabelPath, CommandAlias, CommandOriginMetadata, DocsHelpAnchorRef,
        PolicyContext, ResultContract, ShortcutNarrationHint,
    };

    #[allow(clippy::too_many_arguments)]
    fn descriptor(
        command_id: &str,
        verb: &str,
        preview: &str,
        approval: &str,
        capability: &str,
        ai_class: &str,
        labels: &[&str],
        scopes: &[&str],
    ) -> CommandDescriptorRecord {
        CommandDescriptorRecord {
            record_kind: "command_descriptor_record".to_string(),
            command_descriptor_schema_version: 1,
            command_id: command_id.to_string(),
            command_revision_ref: format!("cmd-rev:{verb}:01"),
            canonical_verb: verb.to_string(),
            primary_label_ref: format!("label:{verb}:primary"),
            accessibility_label_path: AccessibilityLabelPath {
                primary_label_ref: format!("label:{verb}:a11y_primary"),
                short_label_ref: format!("label:{verb}:a11y_short"),
                long_description_ref: format!("label:{verb}:a11y_long"),
                role_class: "command".to_string(),
                keyboard_shortcut_narration_ref: format!("label:{verb}:shortcut"),
            },
            docs_help_anchor_ref: DocsHelpAnchorRef {
                pack_id: "pack:test:01".to_string(),
                anchor_id: format!("docs:{verb}"),
                anchor_kind: "docs_page_anchor".to_string(),
            },
            shortcut_narration_hint: ShortcutNarrationHint {
                when_bound_narration_ref: format!("label:{verb}:bound"),
                when_unbound_narration_ref: format!("label:{verb}:unbound"),
                chord_class_hint: "modifier_plus_key".to_string(),
            },
            aliases: vec![CommandAlias {
                alias_id: format!("alias:{verb}:legacy"),
                alias_kind: "legacy_command_id".to_string(),
                canonical_command_id: Some(command_id.to_string()),
                replacement_note_ref: None,
                introduced_version: None,
                deprecation_state: Some("deprecated".to_string()),
                retirement_version: None,
            }],
            category_refs: vec!["category:test".to_string()],
            origin: Some(CommandOriginMetadata {
                origin_class: "core".to_string(),
                source_ref: None,
                publisher_ref: None,
            }),
            invocation_schema_ref: Some(
                "schemas/commands/command_invocation_session.schema.json".to_string(),
            ),
            result_schema_ref: Some(
                "schemas/commands/command_result_packet.schema.json".to_string(),
            ),
            enablement_rule_refs: vec![format!("reason:{verb}:rule")],
            discoverability_record_refs: vec![format!("discover:{verb}:01")],
            automation_labels: labels.iter().map(|l| l.to_string()).collect(),
            typed_arguments: vec![],
            capability_scope_class: capability.to_string(),
            preview_class: preview.to_string(),
            approval_posture_class: approval.to_string(),
            ai_tool_surfacing_class: ai_class.to_string(),
            palette_visibility: "always_visible".to_string(),
            ui_slot_hints: vec![],
            lifecycle_state: "stable".to_string(),
            support_class: "standard_support".to_string(),
            release_channel: "stable_channel".to_string(),
            declared_freshness_class: "authoritative_live".to_string(),
            client_scopes: scopes.iter().map(|s| s.to_string()).collect(),
            result_contract: ResultContract {
                result_contract_class: "no_result_emitted".to_string(),
                artifact_kind_ref: None,
                typed_value_shape_ref: None,
                evidence_ref_class_required: vec![],
            },
            default_enablement_repair_hook_ref: None,
            policy_context: PolicyContext {
                policy_epoch: "pe:01".to_string(),
                trust_state: "trusted".to_string(),
                execution_context_id: Some("exec:01".to_string()),
            },
            redaction_class: "metadata_safe_default".to_string(),
            minted_at: "2026-05-20T00:00:00Z".to_string(),
        }
    }

    fn enabled_surface(class: &str, preview: &str, approval: &str) -> SurfaceInvocationRecord {
        SurfaceInvocationRecord {
            surface_class: class.to_string(),
            enablement_decision_class: "enabled".to_string(),
            disabled_reason_code: None,
            preview_class_declared: preview.to_string(),
            approval_posture_class_declared: approval.to_string(),
            authority_class: "user_initiated_local".to_string(),
            dispatched: class == surface_class::COMMAND_PALETTE,
            outcome_code: Some("succeeded".to_string()),
            invocation_session_id: format!("inv:{class}:01"),
            resolves_to_canonical_command_id: None,
        }
    }

    fn lineage(command_id: &str, rollback_id: Option<&str>) -> InvocationLineageRecord {
        InvocationLineageRecord {
            command_id: command_id.to_string(),
            invocation_session_id: "inv:command_palette:01".to_string(),
            result_packet_id: "result:01".to_string(),
            result_outcome_code: "succeeded".to_string(),
            evidence_refs: vec!["evidence:01".to_string()],
            notification_refs: vec![],
            activity_refs: vec!["activity:01".to_string()],
            rollback_handle_posture: if rollback_id.is_some() {
                "reversible_handle".to_string()
            } else {
                "not_reversible_by_contract".to_string()
            },
            rollback_handle_id: rollback_id.map(str::to_string),
            support_bundle_ref: Some("support-bundle:01".to_string()),
        }
    }

    fn reversible_scenario() -> CommandAuthorityScenarioRecord {
        CommandAuthorityScenarioRecord {
            record_kind: "command_authority_scenario_record".to_string(),
            schema_version: 1,
            scenario_id: "scenario:test:01".to_string(),
            canonical_descriptor: descriptor(
                "cmd:test.toggle",
                "test.toggle",
                "no_preview_required",
                "no_approval_required",
                "inert_metadata_only",
                "not_ai_callable",
                &["macro_safe", "recipe_safe", "headless_safe"],
                &["desktop_product", "cli", "companion_surface"],
            ),
            surfaces: vec![
                enabled_surface(
                    surface_class::COMMAND_PALETTE,
                    "no_preview_required",
                    "no_approval_required",
                ),
                enabled_surface(
                    surface_class::CLI_HEADLESS,
                    "no_preview_required",
                    "no_approval_required",
                ),
            ],
            lineage: lineage("cmd:test.toggle", None),
        }
    }

    #[test]
    fn validates_clean_reversible_scenario() {
        let scenario = reversible_scenario();
        assert!(scenario.validate().is_ok(), "{:?}", scenario.validate());
        let projection = scenario.project();
        assert!(projection.parity_clean);
        assert!(projection.lineage_complete);
        assert!(!projection.rollback_required);
        assert_eq!(
            projection.surface_classes_covered,
            vec!["cli_headless".to_string(), "command_palette".to_string()]
        );
        assert!(projection
            .lineage_chain
            .contains(&"evidence:01".to_string()));
    }

    #[test]
    fn rejects_suppressed_preview() {
        let mut scenario = reversible_scenario();
        scenario.canonical_descriptor.preview_class = "structured_diff_preview".to_string();
        scenario.canonical_descriptor.capability_scope_class =
            "recoverable_durable_mutation".to_string();
        scenario.lineage = lineage("cmd:test.toggle", Some("rollback:01"));
        // The CLI surface still declares no_preview_required -> suppression.
        let err = scenario.validate().unwrap_err();
        assert!(
            err.contains("suppresses the preview requirement"),
            "got: {err}"
        );
    }

    #[test]
    fn rejects_authority_widening_for_non_headless_command() {
        let mut scenario = reversible_scenario();
        scenario.canonical_descriptor.automation_labels =
            vec!["macro_safe".to_string(), "recipe_safe".to_string()];
        let err = scenario.validate().unwrap_err();
        assert!(err.contains("widens authority"), "got: {err}");
        assert!(err.contains("headless_safe"), "got: {err}");
    }

    #[test]
    fn rejects_enablement_divergence() {
        let mut scenario = reversible_scenario();
        scenario.surfaces[1].enablement_decision_class = "disabled_with_reason".to_string();
        scenario.surfaces[1].disabled_reason_code = Some("workspace_trust_restricted".to_string());
        let err = scenario.validate().unwrap_err();
        assert!(
            err.contains("diverges from the canonical enablement decision"),
            "got: {err}"
        );
    }

    #[test]
    fn rejects_lineage_without_evidence() {
        let mut scenario = reversible_scenario();
        scenario.lineage.evidence_refs.clear();
        let err = scenario.validate().unwrap_err();
        assert!(err.contains("without an evidence ref"), "got: {err}");
    }

    #[test]
    fn requires_rollback_handle_for_durable_command() {
        let mut scenario = reversible_scenario();
        scenario.canonical_descriptor.capability_scope_class =
            "recoverable_durable_mutation".to_string();
        // no rollback handle id present
        let err = scenario.validate().unwrap_err();
        assert!(err.contains("rollback handle"), "got: {err}");
    }

    #[test]
    fn rejects_approval_label_mismatch() {
        let mut scenario = reversible_scenario();
        scenario
            .canonical_descriptor
            .automation_labels
            .push("approval_required".to_string());
        let err = scenario.validate().unwrap_err();
        assert!(err.contains("approval_required disagrees"), "got: {err}");
    }

    #[test]
    fn rejects_non_canonical_alias() {
        let mut scenario = reversible_scenario();
        scenario.canonical_descriptor.aliases[0].canonical_command_id =
            Some("cmd:other".to_string());
        let err = scenario.validate().unwrap_err();
        assert!(
            err.contains("does not resolve to canonical command id"),
            "got: {err}"
        );
    }
}
