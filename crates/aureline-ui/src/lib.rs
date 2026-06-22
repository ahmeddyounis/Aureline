//! Shared UI primitives and contracts.
//!
//! This crate is the cross-surface home for appearance primitives and the
//! semantic token registry used by first-party UI surfaces.
//!
//! [`m5_structured_input_and_staged_review::M5StructuredInputSetPacket`] freezes
//! the canonical structured-input, parameter-provenance, draft-state, and
//! staged-review truth shared by mutation-capable forms, wizards, and review
//! sheets across the provider, admin, request, package, import, settings, and
//! project lanes. Each
//! [`m5_structured_input_and_staged_review::FormSurfaceRecord`] binds field
//! provenance, validation state, draft/applied recovery, submit blockers, and a
//! staged-review (commit) sheet, then re-derives a
//! [`m5_structured_input_and_staged_review::SurfaceClaim`] so a form can never
//! submit from a source-hidden state, hide target scope, omitted defaults,
//! blocked prerequisites, or rollback consequences behind a generic Continue, or
//! discard a recoverable draft. Downstream settings, marketplace, request,
//! support, admin, import, and project surfaces ingest this packet rather than
//! minting per-feature form semantics.
//!
//! [`m5_field_control_rows::M5FieldControlRowSetPacket`] freezes the *per-row*
//! primitive those forms are built from: each
//! [`m5_field_control_rows::FieldControlRow`] carries a permanent label, a
//! required/optional marker, a source-of-value tag, an exact field-anchored
//! validation rule, and a restart/reconnect/trust/policy lifecycle implication
//! surfaced on the control itself, then re-derives a
//! [`m5_field_control_rows::RowClaim`] so a row that hides its label or source,
//! silently overrides a policy lock, defers a blocking validation to a banner, or
//! buries a lifecycle implication floors to an explicit blocked state. The shared
//! primitive is proven first across the provider/account-mapping, source-
//! registration, request-environment, package/install, and migration/import lanes.
//!
//! [`m5_form_validation_and_blocked_submit::M5FormValidationSetPacket`] freezes the
//! layer between them: how a form rolls field validity up into a *form-level
//! validation summary* without replacing the field anchors, how it explains
//! *cross-field dependencies* (provider/account mapping, environment selection,
//! package source/registry auth, import/export mode, derived field constraints),
//! and how it emits *machine-readable blocked-submit reasons* that desktop,
//! CLI/headless, support-export, and docs/help surfaces can all reuse. Each
//! [`m5_form_validation_and_blocked_submit::FormValidationRecord`] re-derives a
//! [`m5_form_validation_and_blocked_submit::FormClaim`] so a form can never submit
//! while a blocked prerequisite or cross-field invalidation is hidden, let its
//! form-level summary contradict or replace the field anchors, or ship a
//! blocked-submit reason that is not machine-readable or reusable.
//!
//! [`m5_draft_state_and_autosave::M5DraftStateSetPacket`] freezes what happens
//! *across an interruption* to those same forms: how edits autosave to a
//! [`m5_draft_state_and_autosave::AutosaveJournal`], how a surface keeps
//! draft-versus-applied state explicit, and how a recover-draft action restores
//! work after a crash, restart, reconnect, or missing-dependency condition. Each
//! [`m5_draft_state_and_autosave::DraftJournalRecord`] re-derives a
//! [`m5_draft_state_and_autosave::DraftClaim`] so an autosave indicator can never
//! claim a draft reached a remote/provider target when only local state was saved,
//! a local draft can never read as applied, an applied state must name its target,
//! and a recover-draft action can never imply a remote write or delete unrelated
//! workspace/profile state. Downstream settings, marketplace, request, support,
//! admin, import, and project surfaces ingest this packet rather than minting
//! per-feature draft/autosave/recovery semantics.
//!
//! [`m5_staged_review_sheets::M5StagedReviewSheetSetPacket`] freezes the *commit
//! sheet itself* as the first-class object every consequential M5 mutation flow
//! stops at before it changes remote/provider/admin/package/request/import state.
//! Each [`m5_staged_review_sheets::ReviewSheetRecord`] declares a target scope (a
//! [`m5_staged_review_sheets::ScopeKind`] from single-object to query-backed or
//! workspace-wide), disclosed omitted defaults, a reconciled
//! [`m5_staged_review_sheets::MemberCounts`] of included/excluded/blocked/hidden
//! objects, a disclosed side-effect summary with a rollback/export path, and a
//! scope-and-effect-specific commit action, then re-derives a
//! [`m5_staged_review_sheets::SheetClaim`] so a sheet that hides its scope, lets its
//! counts disagree, leaves collapsed members uncounted, hides the
//! included/excluded/blocked counts, hides omitted defaults or a side effect, buries
//! a blocked prerequisite or rollback consequence behind a generic Continue, or lets
//! an imported review read as a local apply floors to an explicit blocked state. The
//! one review model is reused across provider publish-later, admin/source-management,
//! request replay/mutation, package install/update/remove, and import/export/publish
//! flows.
//!
//! [`m5_parameter_source_and_precedence::M5ParameterSourceSetPacket`] freezes the
//! *parameter-source inspector* those same forms open to answer **why a current
//! value is present and which source actually wins** before a change is committed.
//! Each [`m5_parameter_source_and_precedence::ParameterFieldRecord`] binds a field's
//! per-layer [`m5_parameter_source_and_precedence::SourceCandidate`] values
//! (default, detected, imported, environment-resolved, user-override,
//! policy-provided), each carrying a personal/local vs workspace/shared vs
//! policy-owned [`m5_parameter_source_and_precedence::ValueScope`], to an
//! [`m5_parameter_source_and_precedence::EffectiveResolution`] that must be the
//! highest-precedence present candidate, a
//! [`m5_parameter_source_and_precedence::PolicyLock`] that pins and forbids a silent
//! override, and a [`m5_parameter_source_and_precedence::FallbackDisclosure`] that
//! explains a fall back to a default. It then re-derives a
//! [`m5_parameter_source_and_precedence::ParameterClaim`] so an inspector that hides
//! its effective source, collapses its distinct layers, mis-orders precedence, hides
//! or fails to enforce a lock, lets an imported review read as a user-set value,
//! hides a fallback reason or scope, or allows a submit from an ambiguous
//! source-hidden state floors to an explicit blocked-submit state — keeping imported
//! values, policy locks, detected values, and user overrides visually distinct
//! across the inspector panel, field popover, diagnostics, support export, CLI
//! inspect, and docs/help surfaces.

#![doc(html_root_url = "https://docs.rs/aureline-ui/0.0.0")]

pub mod components;
pub mod density;
pub mod m5_draft_state_and_autosave;
pub mod m5_field_control_rows;
pub mod m5_form_validation_and_blocked_submit;
pub mod m5_parameter_source_and_precedence;
pub mod m5_staged_review_sheets;
pub mod m5_structured_input_and_staged_review;
pub mod motion;
pub mod themes;
pub mod tokens;
