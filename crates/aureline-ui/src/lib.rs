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
//!
//! [`m5_accessibility_and_continuity::M5AccessibilityContinuitySetPacket`] freezes the
//! *accessibility and interruption-safety contract* those same dense surfaces must hold
//! so the shared structured-input model stays fully usable under keyboard-only,
//! assistive-tech, reduced-motion, reconnect, and restart conditions. Each
//! [`m5_accessibility_and_continuity::SurfaceRecord`] binds a surface's keyboard
//! reachability (deterministic focus order, roving focus, batch-action parity, an
//! escapable focus trap), its assistive-tech reachability (permanent screen-reader
//! labels, inline validation links announced, blocked-submit reasons in a live region,
//! the step position announced), its reduced-motion behavior (bound to the shared
//! [`motion::ReducedMotionSubstitutionClass`] so state never depends on animation), and
//! its interruption-safe continuity (a recovery journal that preserves the current step,
//! blocked fields, and draft across reconnect, restart, missing dependency, and crash),
//! then re-derives a [`m5_accessibility_and_continuity::ContinuityClaim`] so a surface
//! that drops a keyboard path, a screen-reader label, an announced validation link or
//! blocked submit, a motion-independent state, a preserved step/blocked-field/draft, a
//! read-only imported review, a recovery path, or a continuity journal floors to an
//! explicit blocked-submit state with a keyboard recovery path. Extension and
//! provider-owned surfaces ingest this packet rather than re-inventing focus order,
//! screen-reader labelling, or recovery semantics, so they cannot quietly regress
//! accessibility or interruption behavior.
//!
//! [`m5_form_family_certification::M5FormFamilyCertificationSetPacket`] is the *promotion
//! model* that certifies the seven preceding component lanes as a whole: it freezes one
//! [`m5_form_family_certification::FamilyRecord`] per claimed M5 mutation-capable form
//! family (provider, admin, request, package, settings, import, and project lanes), binds an
//! [`m5_form_family_certification::EvidenceCell`] per required `(dimension, lane)` proof pair
//! — field/form validation, parameter provenance, draft-versus-applied truth, interruption
//! recovery, and staged-review-before-commit, each pointing at the upstream lane's support
//! export — and re-derives a [`m5_form_family_certification::FamilyDecision`] that floors the
//! family's claimed [`m5_form_family_certification::QualificationTier`] when any of that
//! proof is stale, partial, missing, or failing, or when the certification-freshness window
//! has elapsed or a consumer surface renders a wider tier than the evidence supports. The
//! About, help, service-health, compatibility, release, and support surfaces ingest this
//! packet rather than re-deciding which form families are certified, so a claimed family can
//! never read wider than its current structured-input, provenance, draft-recovery, and
//! staged-review evidence backs.
//!
//! [`m5_annotation_rows::AnnotationRow`] freezes the reusable annotation row for
//! build, test, security, and provider findings. It keeps source provider/scanner
//! provenance, typed file/symbol/manifest anchors, severity, confidence,
//! freshness, stale/superseded/partial handoff state, suppression, remediation,
//! and open-details action as separate export-safe fields so review panes,
//! project-health centers, companion clients, support exports, and release proof
//! render the same anchor truth instead of silently retargeting stale findings.
//!
//! [`m5_dependency_rows::DependencyRow`] freezes the reusable dependency row for
//! package-manager, review, project-health, framework-pack health, companion,
//! support, and release surfaces. It keeps package name and ecosystem, direct
//! versus transitive relation, current-to-target version delta, manifest scope,
//! lockfile impact, advisory counts, and changelog/license actions as separate
//! export-safe fields, and preserves limited, blocked, and policy-constrained
//! update states as visible rows instead of reducing them to disabled buttons.
//!
//! [`m5_pipeline_run_rows::PipelineRunRow`] freezes the reusable pipeline-run
//! row for review, pipeline, project-health, companion, support, and release
//! surfaces. It preserves provider/run identity, trigger, branch/change
//! relation, artifact counts, freshness, provider handoff, and rerun/cancel
//! authority so reduced-capability consumers show limited-action notes instead
//! of hiding controls.
//!
//! [`m5_manifest_diff_cards::ManifestDiffCard`] freezes the reusable manifest
//! diff card for package, review, project-health, companion, support, and
//! release surfaces. It preserves scripts/hooks preview, peer/runtime
//! constraints, checkpoint/rollback state, and apply boundary so package changes
//! cannot flatten into a generic update card.
//!
//! [`m5_security_finding_cards::SecurityFindingCard`] freezes the reusable
//! security-finding card for package, secret, policy, and code-analysis
//! findings. It keeps finding class, affected scope, severity, confidence,
//! freshness, fix availability, controlled suppression labels, remediation
//! path, local validation, docs/help path, and audit actions as separate
//! export-safe fields for review, package, health, companion, support, and
//! release surfaces.
//!
//! [`m5_decision_feedback_component_matrix::M5DecisionFeedbackComponentMatrixPacket`]
//! freezes the reusable badge-chip-pill, popover, dialog-sheet,
//! banner-inline-notice, toast, empty-state, loading-state, and
//! consequence-block decision/feedback primitives into one export-safe matrix.
//! It binds every primitive to one shared state taxonomy (info, success,
//! warning, blocked, pending, degraded, acknowledged, dismissed) and to the
//! family-specific badge, popover, dialog, notice, toast, empty-state,
//! loading, and consequence vocabularies, so badge meaning never depends on
//! color alone, popovers never carry the only critical instruction, high-risk
//! dialogs never use generic Yes/No copy, toasts never become the only durable
//! truth, useful panes are never blanked during loading, and full-screen
//! spinners never replace partial capability across the shell, entry, trust,
//! review, repair, and notification surfaces.
//!
//! [`m5_visual_foundation_matrix::M5VisualFoundationMatrixPacket`] freezes
//! Aureline's concrete visual foundation — color system, semantic theme
//! tokens, syntax / diff / chart token families, typography, and
//! spacing / sizing / radii / elevation geometry with minimum hit-target
//! rules — into one export-safe matrix. It binds every governed family to one
//! shared semantic-role taxonomy (brand, interactive, neutral, status, syntax,
//! diff, chart) and to the family-specific color, theme-token, syntax, diff,
//! chart, typography, geometry, and hit-target vocabularies, and back to the
//! already-landed design-system foundations and publication packets, so status
//! and trust meaning never collapse into color-only cues, syntax and diff
//! palettes never collide with diagnostics, chart meaning never depends on
//! color alone, hit targets never shrink below supported minima, and no
//! surface forks local geometry away from the shared foundation across the
//! shell, editor, review, data, and docs surfaces.
//!
//! [`m5_motion_layer_iconography_matrix::M5MotionLayerIconographyMatrixPacket`]
//! freezes Aureline's motion, overlay, layering, symbol, and illustration
//! grammar — motion-token and reduced-motion families, opacity / scrim classes,
//! z-order layer tiers, portal ownership, iconography, and illustration
//! boundaries — into one export-safe matrix. It binds every governed family to
//! one shared interaction-role taxonomy (motion, overlay, layer, portal, icon,
//! illustration, attention) and to the family-specific motion, reduced-motion,
//! scrim, layer, portal, icon, and illustration vocabularies, and back to the
//! already-landed design-system foundations and publication packets, so motion
//! never delays input on protected paths, scrims never erase workspace
//! orientation or contrast, overlays and portals never bypass the shared z-order
//! model, uncommon or destructive icons never ship unlabeled, and illustrations
//! never impersonate trust, severity, or operational truth across the desktop,
//! dialog, onboarding, notification, and embedded surfaces.
//!
//! [`m5_shell_metric_density_matrix::M5ShellMetricDensityMatrixPacket`]
//! freezes Aureline's concrete shell geometry and density behavior — shell-zone
//! metrics, minimum sizes and hit targets, density modes, responsive window
//! classes, and adaptive-collapse priorities — into one export-safe matrix. It
//! binds every governed family to one shared shell-geometry-role taxonomy
//! (zone, metric, hit_target, density, responsive, collapse,
//! workspace_dominance) and to the family-specific shell-metric, minimum-size,
//! density, responsive, and collapse vocabularies, and back to the
//! already-landed shell-zone and reusable-shell-primitive packets, so the main
//! workspace stays dominant, zones honor declared minimum and recommended sizes,
//! density changes presentation rather than information architecture, responsive
//! collapse preserves task identity and recovery-critical state, hit targets
//! never shrink below supported minimums, and extension or embedded surfaces
//! never invent private widths that fracture the shell across the desktop,
//! editor, review, notebook, and data surfaces.
//!
//! [`m5_install_topology_matrix::M5InstallTopologyMatrixPacket`] freezes
//! Aureline's concrete delivery-topology behavior — per-user managed install,
//! per-machine managed install, side-by-side stable-plus-preview, portable mode,
//! and offline / air-gap bundles — into one export-safe matrix. It binds every
//! governed family to one shared install-topology-role taxonomy (install_mode,
//! updater_owner, binary_root, writable_state_roots, policy_roots,
//! rollback_target, rollout_ring) and to the family-specific per-user,
//! per-machine, side-by-side, portable, and offline / air-gap vocabularies, and
//! back to the already-landed coexistence / fleet-rollout and native-desktop
//! packets, so binary placement and updater ownership stay inspectable, portable
//! mode never spills machine-global durable state, stable and preview channels
//! never corrupt one another, rollback targets the full artifact graph, and
//! rollout rings keep promotion and rollback evidence across the About, update,
//! diagnostics, admin, docs, and support surfaces.
//!
//! [`m5_window_restore_matrix::M5WindowRestoreMatrixPacket`] freezes Aureline's
//! concrete multi-window ownership and restore-orchestration behavior — shared
//! workspace authority backing multiple windows, window-local pane topology,
//! skeleton-first / hydrate-second restore, no-rerun session hydration, and
//! display-topology recovery — into one export-safe matrix. It binds every
//! governed family to one shared window-restore-role taxonomy (workspace_authority,
//! window_topology, pane_role, layout_skeleton, session_hydration, restore_fidelity,
//! display_affinity) and to the family-specific shared-authority, window-local,
//! skeleton-restore, no-rerun, and display-recovery vocabularies, and back to the
//! already-landed multi-window-parity and monitor-geometry-remap packets, so
//! workspace authority and window topology stay separately inspectable, selections
//! and focus stay window-local, restore rebuilds layout skeletons before hydrating
//! heavy dependencies, session-scoped tools never silently rerun or reacquire
//! broader authority, and display-topology changes keep every window and dialog
//! reachable across the shell, recovery, diagnostics, admin, docs, and support
//! surfaces.
//!
//! [`m5_workspace_authority_and_window_topology_registries::M5WorkspaceAuthorityWindowTopologyRegistriesPacket`]
//! is the first implement lane over that frozen matrix: it turns the shared
//! workspace-authority grammar and the window-local-topology grammar into registry
//! resolvers, so every claimed workspace resolves to one stable
//! workspace-authority object — the authority scope, the windows it backs, the
//! stable versioned pane-tree IDs, the shared dirty-buffer / save / checkpoint
//! state, the authoritative workspace state root, and the distinct profile-defaults
//! reference — that the shell, recovery, diagnostics, admin, and support / export
//! surfaces inspect without manual reconstruction, multiple windows share one
//! authority while selection and focus stay window-local without dirty-state drift,
//! window topology never absorbs shared authority into private window state, and a
//! workspace that cannot explain which state is shared and which is window-local
//! degrades honestly.
//!
//! [`m5_skeleton_first_restore_and_session_hydration_registries::M5SkeletonFirstRestoreSessionHydrationRegistriesPacket`]
//! is the skeleton-first / hydrate-second implement lane over that frozen matrix:
//! it turns the skeleton-first-restore grammar and the no-rerun-session-hydration
//! grammar into registry resolvers, so every claimed restore rebuilds one stable
//! restore-skeleton object — the restore-fidelity class, the window shell, the
//! stable versioned pane-tree structure, the preserved pane roles and placeholder
//! set, the layout-skeleton root, and the distinct deferred-hydration plan — before
//! any heavy dependency hydrates, restore is progressively truthful instead of
//! all-or-nothing, a missing dependency produces a pane-role-preserving placeholder
//! instead of a silent layout collapse, session-scoped tools never silently rerun or
//! reacquire broader authority, and support / export can explain which panes came
//! back live, as placeholders, context-only, or evidence-only.
//!
//! [`m5_window_restore_shared_consumers_one_registry_across_surfaces::M5WindowRestoreSharedConsumersPacket`]
//! is the consumer-adoption capstone over that frozen matrix: it binds each of the
//! five window-restore families to the concrete restore-coordinator, shell,
//! workspace, session, diagnostics, docs / help, CLI / export, support-export, and
//! product consumers that render it, and proves — by fixtures, not screenshots —
//! that the same restore profile presents the same window-restore-role, family,
//! registry-reference, restore-context, surface-context, and session-continuity
//! grammar wherever it appears, so a family is adopted by two or more consumers,
//! restore class / no-rerun semantics / placeholder posture / display affinity never
//! drift between surfaces, and a surface that reruns session-scoped work, deletes
//! layout structure silently, strands a window off-screen after a display-topology
//! remap, merges workspace-authority and window-topology into one blob, or overclaims
//! restore fidelity degrades honestly rather than silently.
//!
//! [`m5_window_restore_accessibility_parity_and_narrowing_when_shared_authority_restore_fidelity_display_remap_or_no_rerun_session_truth_is_stale::WindowRestoreAccessibilityPacket`]
//! is the accessibility-localization-support-export parity and honest auto-narrowing
//! capstone over that frozen matrix: it certifies, per workspace-restore family,
//! that workspace authority, window topology, restore-fidelity class,
//! missing-dependency posture, display-remap history, and no-rerun session state
//! stay keyboard-reachable, screen-reader-announced, high-zoom-legible,
//! high-contrast-safe, localization-safe, and CLI/export-safe, and that when a
//! skeleton-first family's layout-skeleton proof is only partially disclosed, a
//! no-rerun session-replay fence cannot be confirmed, or a display-remap recovery
//! evidence has aged out, the family's claim auto-narrows from
//! trusted_restore_surface / reviewable_restore_surface to a
//! layout-skeleton-disclosed / session-replay-unverified / display-recovery-unverified
//! projection that discloses the narrowing with a precise trigger and preserves the
//! canonical window-restore identity, so a fidelity-overclaimed, evidence-aged, or
//! policy-blocked state can never keep a trusted, stable restore claim across the
//! restore-coordinator, shell, workspace, session, diagnostics, docs / help, CLI /
//! export, support-export, and product surfaces.
//!
//! [`m5_window_restore_surface_certification::WindowRestoreProfileCertificationPacket`]
//! is the closing surface-certification capstone that certifies the shared
//! window-restore truth holds on every claimed M5 desktop workspace profile — live
//! trusted restore surface, reviewable restore structure, disclosed layout-skeleton
//! profile, unverified session-replay profile, and unverified display-recovery
//! profile — scoring each across nine truth axes (visual, keyboard, screen-reader,
//! high-zoom-reflow, high-contrast, localization, CLI/export, degraded-state, and
//! window-restore-component-truth behavior) so a degraded axis auto-narrows the
//! restore claim to the weakest supported ceiling, only a live first-party profile
//! certifies a trusted restore surface, CLI/export parity always certifies, and every
//! B141 hard invariant holds, with every row citing the one canonical window-restore
//! proof bundle rather than hand-authored release, docs, or support prose.
//!
//! [`m5_install_topology_and_state_root_registries::M5InstallTopologyStateRootRegistriesPacket`]
//! is the first implement lane over that frozen matrix: it turns the per-user
//! managed, per-machine managed, and side-by-side install-topology grammar and
//! the portable-mode / offline-air-gap state-root-boundary grammar into registry
//! resolvers, so every claimed delivery profile resolves to one stable
//! install-topology object — install mode, channel, updater owner, binary root,
//! writable state roots, policy roots, and rollback target — that About, update,
//! diagnostics, admin, and support / export surfaces inspect without manual
//! reconstruction, managed-versus-user scopes and side-by-side channels resolve to
//! explicitly isolated state namespaces, and a profile that cannot explain its
//! shared-versus-isolated state degrades honestly.
//!
//! [`m5_portable_mode_state_containment_and_diagnostics::M5PortableModeStateContainmentAndDiagnosticsPacket`]
//! is the portable-mode runtime-enforcement lane over that frozen matrix: it
//! makes *portable* a contract by resolving every claimed portable profile to a
//! colocated or explicitly named sibling-state layout with a complete durable-root
//! inventory of settings, secrets, services, and shell hooks, proving hidden
//! machine-global mutation is absent or explicitly blocked, keeping portable state
//! distinguishable from ordinary installed state, and publishing discoverable
//! portable-mode diagnostics — executable root, state roots, log / crash
//! locations, update posture, and unsupported shell-integration paths — with
//! documented retained-versus-replaced update continuity, so a portable profile
//! that spills durable state into a hidden machine-global path, cannot name its
//! roots, or leaves its diagnostics implicit degrades honestly across the About,
//! update, diagnostics, admin, docs, and support surfaces.
//!
//! [`m5_managed_deployment_operations_and_policy_bootstrap_injection::M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket`]
//! is the managed-deployment execution lane over that frozen matrix: it makes
//! *managed deployment* a contract by resolving every claimed managed profile's
//! silent install, silent uninstall, repair-or-verify, channel-pinning, and
//! update-deferral operation to one inspectable object — the operation, the
//! operation-target / receipt / failure-diagnostics roots, the copyable install-ID
//! / timestamp / failure-summary / repair-verify receipt, and the explicit
//! admin-versus-user ownership — and every policy-bundle / bootstrap injection to
//! one published record — policy-bundle source, bootstrap target, applied
//! settings, admin owner, and deferral window — so a managed installer presented
//! as user-controlled, a failure that strands the user in an ambiguous ownership
//! state, or a bootstrap-policy / channel-pinning / repair-verify semantics drift
//! degrades honestly across the installer, update, diagnostics, admin, docs, and
//! support surfaces.
//!
//! [`m5_channel_isolation_precedence_review_and_rollback_targets::M5ChannelIsolationPrecedenceReviewAndRollbackTargetsPacket`]
//! is the side-by-side coexistence-execution lane over that frozen matrix: it makes
//! *side-by-side channel isolation* a contract by resolving every claimed
//! side-by-side profile's stable, preview, beta, and LTS channel to one inspectable
//! object — the channel, the channel / state-namespace / secrets-namespace roots,
//! the isolated channel-root / state-namespace / secrets-namespace / services-namespace
//! inventory, and the explicit isolated-versus-governed-handoff containment — and
//! every file-association / protocol-handler / deep-link / default-open precedence
//! rule to one published record — owner channel, precedence rank, conflict
//! resolution, full rollback artifact graph, and inspectable-before-and-after — so a
//! preview or beta channel that reused the stable state namespace without a governed
//! handoff, a handler ownership that became a last-writer-wins accident, or a
//! rollback target narrowed below the full artifact graph degrades honestly across
//! the installer, update, diagnostics, admin, docs, and support surfaces.
//!
//! [`m5_install_topology_shared_consumers_one_registry_across_surfaces::M5InstallTopologySharedConsumersPacket`]
//! is the consumer-adoption capstone over that frozen matrix: it binds each of the
//! five install-topology families to the concrete installer / package-manager,
//! About / shell, update-center / updater, diagnostics, admin, docs / help, CLI /
//! export, support-export, and product / fleet-rollout consumers that render it, and
//! proves — by fixtures, not screenshots — that the same delivery profile presents
//! the same install-topology-role, family, registry-reference, channel,
//! surface-context, and ownership-identity grammar wherever it appears, so a family
//! is adopted by two or more consumers, install mode / channel / updater owner /
//! state roots / rollback target / rollout ring never drift between surfaces, and a
//! surface that reuses a stable state namespace, spills machine-global durable
//! state, hides updater ownership, narrows rollback below the full artifact graph, or
//! outpaces ring evidence degrades honestly rather than silently.
//!
//! [`m5_install_topology_accessibility_parity_and_narrowing_when_install_topology_state_root_repair_verify_or_rollout_evidence_is_stale::InstallTopologyAccessibilityPacket`]
//! is the accessibility-localization-support-export parity and auto-narrowing
//! capstone over that frozen install-topology matrix. It certifies, per
//! delivery-topology family, that install-mode / updater-owner / state-root /
//! repair-verify / rollout-ring / rollback claims stay keyboard-reachable,
//! screen-reader-announced, high-zoom-legible, high-contrast-safe,
//! localization-safe, and CLI/export-safe, and auto-narrows a side-by-side family
//! whose state-boundary proof is only partially disclosed, a portable / offline
//! family whose repair/verify coverage is unconfirmed, or a family whose
//! rollout-ring evidence has aged out or is policy-blocked to a
//! state-boundary-disclosed / repair-verify-unverified / rollout-evidence-unverified
//! projection, so no claimed delivery profile stays green after its B140 evidence
//! ages out or becomes policy-blocked.
//!
//! [`m5_install_topology_surface_certification::InstallTopologyProfileCertificationPacket`]
//! is the closing B140 surface-certification capstone over that frozen
//! install-topology matrix. Keyed on the claimed delivery profile — a live
//! trusted delivery surface, a reviewable delivery structure, a disclosed
//! state-boundary profile, an unverified repair/verify profile, and an unverified
//! rollout-evidence profile — it certifies each profile across nine truth axes
//! (visual, keyboard, screen-reader, high-zoom-reflow, high-contrast,
//! localization, CLI/export, degraded-state, and install-topology-component-truth)
//! and either delivers its claim (green) or auto-narrows to a state-boundary-disclosed
//! / repair-verify-unverified / rollout-evidence-unverified projection (yellow).
//! Only a live first-party profile may certify a trusted delivery surface,
//! CLI/export parity must always certify, every B140 hard invariant must hold, and
//! every row cites the one canonical install-topology proof bundle so release,
//! docs, and support consume a single delivery-topology certification source.
//!
//! [`m5_pipeline_dependency_finding_components::M5PipelineDependencyFindingComponentProof`]
//! validates the first-consumer proof across the five component families and
//! checks that review panes, package centers, project-health centers, companion
//! clients, support export, and release proof preserve controlled labels and
//! narrow action authority explicitly.
//!
//! [`m5_platform_fit_matrix::M5PlatformFitMatrixPacket`] freezes Aureline's
//! desktop platform-fit conventions — platform conventions, shortcut notation,
//! file / path / reveal / save terminology, live theme / contrast / accent /
//! text-scale response, credential-store wording, and IME / dead-key / AltGr /
//! dictation / emoji / layout-switch behavior — into one export-safe matrix. It
//! binds every governed family to one shared platform-fit-role taxonomy
//! (shortcut, window_menu, path_terminology, appearance, credential_wording,
//! input_fidelity, command_stability) and to the family-specific
//! platform-convention, shortcut-notation, file-path-reveal, theme-contrast,
//! credential-wording, and input-method vocabularies, and back to the
//! already-landed native-desktop matrix, so command IDs stay stable while
//! platform labels and shortcut notation adapt, primary actions are never hidden
//! in OS chrome alone, file / path / reveal / save terminology matches the host,
//! theme / contrast changes apply live or explain their fallback, credential-store
//! wording stays truthful and non-leaky, and input methods never corrupt text or
//! trust fidelity across macOS, Windows, and Linux.
//!
//! [`m5_platform_fit_accessibility_parity_and_narrowing_when_platform_convention_native_affordance_or_input_method_truth_is_stale::PlatformFitAccessibilityPacket`]
//! is the accessibility-localization-support-export parity and auto-narrowing
//! capstone over that frozen platform-fit matrix. It certifies, per platform-fit
//! family, that platform-convention / shortcut / path / appearance /
//! credential-wording / input-method claims stay keyboard-reachable,
//! screen-reader-announced, high-zoom-legible, high-contrast-safe,
//! localization-safe, and CLI/export-safe, exports the active platform profile and
//! its shortcut / path / appearance / credential / input state into diagnostics and
//! support bundles without a raw payload, and auto-narrows the affected claim to a
//! path-terminology-disclosed / appearance-response-unverified /
//! credential-wording-unverified / input-fidelity-unverified projection whenever the
//! platform-fit, input-method, or screenshot/help parity evidence is stale, missing,
//! or failing.
//!
//! [`m5_repository_bootstrap_matrix::M5RepositoryBootstrapMatrixPacket`] freezes
//! Aureline's concrete repository-acquisition and workspace-bootstrap behavior —
//! open a local checkout, clone a remote source, open an archive, import a bundle,
//! and resume a partial-acquisition snapshot — into one export-safe matrix. It binds
//! every governed family to one shared repository-bootstrap-role taxonomy
//! (source_locator, checkout_plan, credential_posture, evidence_packet, staged_trust,
//! resumable_acquisition, post_open_queue) and to the family-specific open-local,
//! clone-remote, open-archive, import-bundle, and resume-snapshot vocabularies, and
//! back to the already-landed repository-acquisition and source-acquisition-review
//! packets, so clone and open stay distinct verbs even when a local checkout already
//! exists, checkout cost / topology / credential posture stay visible before any
//! network or disk mutation, repo hooks / tasks / extensions / package restores /
//! submodule or LFS hydration / generator installs never run implicitly during
//! acquisition, signer and mirror provenance stay continuous across offline and
//! mirrored fetches, and interrupted acquisition stays resumable or discardable with
//! evidence.
//!
//! [`m5_settings_governance_matrix::M5SettingsGovernanceMatrixPacket`] freezes
//! Aureline's concrete settings-resolver, sync-conflict, and capability-lifecycle
//! runtime behavior — resolve an effective setting from the winning scope, land a
//! write intent in the chosen artifact and scope, sync a scope bundle across devices,
//! migrate a settings schema across versions, and roll out a capability lifecycle —
//! into one export-safe matrix. It binds every governed family to one shared
//! settings-governance-role taxonomy (setting_definition, effective_resolution,
//! write_intent, policy_constraint, sync_conflict, schema_migration,
//! capability_lifecycle) and to the family-specific resolve-setting, write-setting,
//! sync-scope, migrate-schema, and rollout-capability vocabularies, and back to the
//! already-landed effective-setting and capability-lifecycle packets, so stable setting
//! IDs are never recycled, winning scope / shadowed values / restart posture / lock
//! source stay inspectable, writes land only in the chosen artifact and scope with
//! preview / checkpoint / rollback evidence, sync never silently overwrites local
//! authoritative state during outages, machine-only state never masquerades as
//! portable, and kill-switch or DisabledByPolicy states preserve user data and explain
//! themselves.
//!
//! [`m5_stable_line_surface_certification::StableLineProtectionProfileCertificationPacket`]
//! is the closing B146 surface-certification capstone over that frozen stable-line-protection
//! matrix: after the 1221–1227 implement lanes resolve the protection-plan, correction-queue,
//! refresh-policy, claim-downgrade, deferral-backlog, correction-conversion, bundle-refresh-audit,
//! shipping-line-drift, defect-ledger, backport-decision-timer, correction-report, train-comparison,
//! lts-readiness-decision, and line-creation-gate registries, it certifies that the shared
//! stable-line operating truth holds on every claimed M5 supported line (a live supported-line
//! operating lane, a reviewable stable-line structure, a disclosed correction-ownership profile, an
//! unverified bundle-currentness profile, and an unverified LTS-readiness profile). Each profile is
//! scored across nine truth axes and either passes (green), auto-narrows its operating claim to the
//! weakest supported ceiling with a bound reason and frozen downgrade trigger (yellow), or blocks
//! (red) when a degraded axis hides behind a fresh certified claim, a B146 hard invariant breaks
//! (widening support language without current refresh and correction evidence, drifting a shipping
//! line on stale evidence or frozen launch bundles, relying on tribal backport memory instead of a
//! documented correction packet, claiming LTS eligibility without current rollback and support
//! evidence, or leaving a supported-line defect unowned or unresolved past its SLA), CLI/export
//! parity drops, or a non-live profile claims a certified operating line, so release, help, support,
//! and public-proof surfaces consume one stable-line certification source rather than hand-authored
//! prose.
//!
//! [`m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries::M5StableLineLtsReadinessDecisionLineCreationGateRegistriesPacket`]
//! makes any future LTS promise evidence-bearing over that frozen stable-line-protection
//! matrix. It records the 91–180 day *LTS-readiness-decision* grammar per candidate line
//! (one typed decision section per operating proof — backport branch posture, correction-line
//! health, rollback evidence, support-window posture, mirror/air-gap continuity proof, and
//! advisory/revocation readiness — bound to the named decision-forum outcome) and gates each
//! candidate through a *line-creation-gate* grammar (whether LTS line creation or LTS-style
//! language is committed on a green packet, blocked because the packet is missing or stale, or
//! narrowed back to a plain stable posture) so no LTS label can widen without a green decision
//! packet backed by current rollback and support evidence — a blocked or missing packet forces
//! the narrower stable-line posture instead — and release/help/support/public-proof surfaces
//! explain why a line is or is not LTS-ready from packet-backed facts rather than generic
//! enterprise-language placeholders.
//!
//! [`m5_stable_line_correction_report_and_train_comparison_registries::M5StableLineCorrectionReportTrainComparisonRegistriesPacket`]
//! makes post-launch learning durable over that frozen stable-line-protection matrix. It
//! publishes a post-launch *correction-report* grammar per release train (one typed report
//! section per operating signal — adoption blockers, crash/support signals, compatibility-report
//! freshness deltas, bundle drift, public-truth deltas, and backport exceptions or deferrals —
//! each linked to its correction packets, supported-line defect-ledger entries, and current claim
//! rows) and drives each train through a *train-comparison* grammar (the comparison scope an
//! issue sits in across trains — a corrected issue, a remaining narrowed claim, or an open
//! exception still needing explicit closure) so operators can compare trains and see which
//! supported-line issues were corrected, which narrowed claims remain, and which exceptions
//! still need closure, and the checked-in correction report becomes the export-safe, docs-safe
//! operating truth release/help/support/public-proof surfaces cite instead of rereading raw
//! incident tickets.
//!
//! [`m5_stable_line_defect_ledger_and_backport_decision_timer_registries::M5StableLineDefectLedgerBackportDecisionTimerRegistriesPacket`]
//! turns supported-line servicing into measurable program truth over that frozen
//! stable-line-protection matrix. It records every supported-line defect in a
//! *defect-ledger* grammar (its affected line, defect class — crash-recovery,
//! rollback/update, support-export, migration/import, compatibility-regression, or
//! security/data-loss — the yes/no/defer backport decision, decision age, rollback
//! target, correction-packet state, and owning release/support roster) and drives each
//! defect through a *backport-decision-timer* grammar (the alert scope a defect's
//! backport decision sits in — a missing backport decision, an overdue backport decision
//! past its SLA, or a narrowed support claim) so a missing or overdue decision raises a
//! visible alert that can block promotion or force narrowing of the relevant stable/LTS
//! support claim, and the first correction-packet exercise ships as a checked-in proof
//! artifact demonstrating the stable line can service itself before any LTS language
//! widens.
//!
//! [`m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries::M5StableLineBundleRefreshAuditShippingLineDriftReportRegistriesPacket`]
//! keeps onboarding and migration promises honest after launch over that frozen
//! stable-line-protection matrix. It turns the *bundle-refresh-audit* grammar (how an active
//! shipping line audits each claimed bundle — a launch bundle, an imported-user handoff
//! bundle, and an org-approved bundle — for freshness, reversibility, missing artifacts, and
//! unsupported drift, publishing exact bundle age, install topology, rollback / reversibility
//! target, and retest posture) and the *shipping-line-drift-report* grammar (the drift report
//! emitted when an audit finds shipping-line drift, recording whether the bundle went stale,
//! became non-reversible, or drifted into an unsupported / missing-artifact state and naming
//! the active drift reason) into registry resolvers, so start-center, migration/help,
//! release/support, and admin/public-proof consumers render bundle drift and retest state
//! from the audit packet, and a stale, non-reversible, or unsupported bundle narrows its
//! onboarding / migration / support claim automatically until refreshed.
//!
//! [`m5_stable_line_deferral_backlog_and_correction_conversion_registries::M5StableLineDeferralBacklogCorrectionConversionRegistriesPacket`]
//! turns leftover launch-time "may slip to v1.0.x" caveats into explicit post-stable truth
//! over that frozen stable-line-protection matrix. It records every bounded launch-era
//! deferral — bounded-feature, performance-posture, migration-path, compatibility-caveat,
//! known-limit, and documentation-gap items — in a supported-line *deferral-backlog* grammar
//! (its affected lines, current correction status, linked claim rows, and required
//! narrow/defer/ship decision) and drives each item through the *correction-conversion*
//! grammar (a release-room report distinguishing shipped corrections, explicit defers, and
//! visible claim narrowings), so an unresolved or overdue "may slip to v1.0.x" item can never
//! stay invisible: it appears as a shipped correction, an explicit defer, or a visible
//! narrowing on the affected supported line, and operators can export one report showing which
//! supported-line claims narrowed because a bounded correction missed its target train.
//!
//! [`m5_stable_line_refresh_policy_and_claim_downgrade_registries::M5StableLineRefreshPolicyClaimDowngradeRegistriesPacket`]
//! makes evidence refresh an ordinary release operation over that frozen
//! stable-line-protection matrix. It turns the *refresh-policy* grammar (how an active
//! stable line schedules a refresh cadence for each evidence surface it publishes — the
//! certified-archetype report, compatibility packet, known-limits doc, release/help/About
//! surface, public-proof surface, and support-export packet, each carrying an exact
//! last-run identity, next-run identity, next-run owner, last-success state, and freshness
//! SLO) and the *claim-downgrade packet* grammar (the machine-readable packet emitted when
//! a surface misses its refresh window, moving the affected claim automatically to
//! Retest-pending, Evidence-stale, or a narrower support-language claim and naming the
//! active downgrade reason) into registry resolvers, so release/help/support/public-proof
//! consumers see refresh age, next-run owner, last-success state, and any active downgrade
//! reason, a stale surface narrows its claim automatically, and support and shiproom
//! exports can prove stable-line truth is current or explicitly downgraded rather than
//! silently stale.
//!
//! [`m5_stable_line_protection_plan_and_correction_queue_registries::M5StableLineProtectionPlanCorrectionQueueRegistriesPacket`]
//! is the first implement lane over that frozen stable-line-protection matrix. It turns the
//! *protection-plan* grammar (how a supported line binds each protected journey — crash
//! recovery, rollback/update, support export, and migration/import, plus other named
//! launch-bearing flows — to its regression queue, publishing the queued-regression issue
//! IDs, release line, correction packet, rollback target, and delayed-breadth ledger) and
//! the *correction-lane queue* grammar (which protected-path regression is queued for
//! correction and which breadth work is intentionally delayed while it stays open) into
//! registry resolvers, so release operators can open one stable-line protection view showing
//! which journeys are guarded, which regressions are queued, and which breadth items are
//! delayed, exact issue / release-line / correction-packet / rollback linkage stays visible,
//! and stable-line breadth work can never silently outrank a crash / rollback /
//! support-export / migration regression without a recorded override or claim-narrowing
//! action.
//!
//! [`m5_supported_line_surface_certification::SupportedLineTransparencyProfileCertificationPacket`]
//! is the closing B147 surface-certification capstone over that frozen supported-line-transparency
//! matrix. After the public-proof-ledger, transparency-report, migration-scoreboard, ORR-history,
//! correction-train-archive, truth-feed, and retention-governance implement lanes, it certifies that
//! the shared durable-proof truth holds on every claimed M5 supported line — current public-proof
//! ledgers, export-safe transparency reports, versioned migration scoreboards, retained ORR history,
//! and archived correction trains — and auto-narrows any profile that cannot sustain it. Keyed on the
//! claimed profile (a live, first-party supported-line operating lane; a reviewable transparency
//! structure; a disclosed correction-archive profile; an unverified migration-scoreboard profile; and
//! an unverified ORR-history profile), each row certifies the profile across nine truth axes — visual,
//! keyboard, screen-reader, high-zoom-reflow, high-contrast, localization, CLI/export, degraded-state,
//! and supported-line-proof-truth behavior — and either passes (green), auto-narrows its operating
//! claim to the weakest supported ceiling (yellow), or blocks (red) when a degraded axis hides behind a
//! fresh certified claim, a B147 hard invariant breaks, CLI/export parity drops, or a non-live profile
//! claims a certified operating line. Only a live, first-party supported-line operating lane with
//! current, export-safe, and internally consistent durable proof may certify a certified operating
//! line, and every row cites the one canonical supported-line-transparency proof bundle, so release,
//! help, docs, support, public-proof, and partner/procurement surfaces consume one supported-line
//! certification source rather than hand-authored prose.
//!
//! [`m5_supported_line_retention_policy_and_stale_escalation_registries::M5SupportedLineRetentionPolicyStaleEscalationRegistriesPacket`]
//! keeps the B147 supported-line proof artifacts alive and reviewable after they first ship over the frozen
//! supported-line-transparency matrix, so transparency reports, migration scoreboards, ORR histories,
//! correction-train archives, and truth feeds stay current, diffable, and line-canonical rather than decaying into
//! one-off launch appendices. It carries one *retention policy* per B147 artifact class — a public-proof-ledger
//! policy, a migration-scoreboard policy, a transparency-report policy, a correction-archive policy, a truth-feed
//! policy, and an ORR-history policy — each naming its accountable owner, review cadence, retention window, archive
//! class, and destruction-or-long-term-retention rule so every class can be inspected in one checked-in policy
//! packet. It raises one typed *stale escalation* per missed cadence (a missing scheduled snapshot, a stale line
//! feed, or a snapshot mismatched with the active supported-line matrix) so automation blocks a supported line from
//! staying green on expired evidence, and the checked-in policy packet exposes each snapshot's age and provenance so
//! support bundles, docs/help/About truth, and public-proof consumers pull the freshest permitted snapshot. Every
//! registry row binds a consumer surface to resolved retention-policy and stale-escalation entries that reuse the
//! frozen matrix vocabulary, so a reviewer can read the retention discipline for any B147 artifact class directly and
//! a missing or stale snapshot surfaces as a blocker rather than a silent decay.
//!
//! [`m5_supported_line_truth_feed_and_audience_packet_registries::M5SupportedLineTruthFeedAudiencePacketRegistriesPacket`]
//! lets external evaluators and support paths consume one current supported-line truth feed instead of
//! hand-assembled fragments over the frozen supported-line-transparency matrix. It bundles one export-safe
//! *truth feed* per active stable or LTS-candidate line — a public-proof summary, a migration-scoreboard summary, a
//! transparency snapshot, a correction-history summary, a claim-history summary, and a release-evidence link, each
//! tracked against exact build / release-line identity with a stable ID and freshness date, public-safe
//! correction-history and claim-history summaries separated from internal-only incident / security payloads — and
//! links out to compatibility reports, known limits, migration guides, and release evidence rather than duplicating
//! them. It projects that one canonical feed into export-safe *audience packet* variants (a support bundle, a
//! procurement bundle, or a partner-review bundle) that exclude internal-only detail by default while still naming
//! the current claim, evidence freshness, migration posture, and correction history. Every registry row binds a
//! consumer surface to resolved truth-feed and audience-packet entries that reuse the frozen matrix vocabulary, so a
//! support, procurement, or partner reviewer opens one current feed directly, a claim never runs ahead of current
//! proof, and no packet variant leaks internal-only detail or lets a stale feed read as green.
//!
//! [`m5_supported_line_correction_train_archive_and_closure_gate_registries::M5SupportedLineCorrectionTrainArchiveClosureGateRegistriesPacket`]
//! makes every correction line auditable end to end over the frozen supported-line-transparency matrix, so
//! release, support, and procurement readers can see what changed, why, and how it was recovered without private
//! shiproom materials. It archives one *correction-train archive* record per shipped correction packet on each
//! active supported line — a hotfix packet, a backport packet, a rollback outcome, an advisory publication, a
//! public-communication bundle, or a revocation record, tracked against exact build / release-line identity, with
//! public-safe advisory and public-communication history separated from internal-only hotfix / backport / rollback /
//! revocation incident payloads — preserving machine-readable joins to bug IDs, supported-line defect ledgers,
//! release artifact graphs, and the public-claim or support-window state the correction affected. It emits one
//! *closure gate* event per archive-coverage gap (missing archive coverage, a broken exact-build join, or an
//! untraceable correction line) so missing coverage or a broken join blocks correction-line closure until fixed.
//! Every registry row binds a consumer surface to resolved archive and closure-gate entries that reuse the frozen
//! matrix vocabulary, so a correction can be traced from an advisory or release note back to its archived evidence
//! bundle, exact-build provenance stays joined to public communication, and no correction line closes while its
//! archive coverage or build joins remain broken.
//!
//! [`m5_supported_line_orr_history_and_follow_up_closure_registries::M5SupportedLineOrrHistoryEventFollowUpClosureRegistriesPacket`]
//! preserves supported-line launch and servicing memory over the frozen supported-line-transparency
//! matrix so later promotion, support, and postmortem work never depends on shiproom folklore. It
//! archives one *ORR-history event* per recorded operational-readiness decision on each active stable
//! or LTS-candidate line — an archived ORR packet, a freeze exception, a rehearsal outcome, a cohort
//! transition, a go/no-go decision, or a post-review action-item closure, tracked against exact
//! build / release-line identity, with public-safe cohort-transition and go/no-go decision history
//! separated from internal-only freeze / rehearsal / action-item minutiae — and emits one *follow-up
//! closure* event per closure-drift scope (an unclosed action item, stale rehearsal evidence, or an
//! unreconstructable line history) so support, partner, procurement, and governance reviews see
//! unclosed follow-up work or stale rehearsal evidence on the active line rather than only in an
//! archived meeting packet. Every registry row binds a consumer surface to resolved history and
//! closure entries that reuse the frozen matrix vocabulary, so a current supported line can be
//! reconstructed from ORR history without shiproom notes, widening and maintenance decisions stay
//! reconstructable, and no line keeps a go/no-go or cohort claim ahead of its recorded decision
//! history.
//!
//! [`m5_supported_line_migration_scoreboard_and_scoreboard_delta_registries::M5SupportedLineMigrationScoreboardScoreboardDeltaRegistriesPacket`]
//! keeps post-launch migration and switching promises tied to real field outcomes rather than
//! frozen launch-time confidence over the frozen supported-line-transparency matrix. It publishes
//! one versioned, scored *migration scoreboard* per active stable or LTS-candidate line — one row
//! per importer / bridge outcome class (cleanly imported, translated, partial, shimmed,
//! unsupported item category, and rollback-cleanliness result, tracked by source tool / version /
//! archetype), each bound to one supported-line identity with rollback cleanliness, docs/help
//! parity, and linked compatibility evidence, and public-safe outcome classes separated from
//! internal-only migration detail — and one *scoreboard delta* per scoreboard change (a field-pain
//! cluster, an unsupported-category growth, or a docs/help-or-rollback gap) so docs/help/migration
//! owners, support, and procurement reviews see where field pain, unsupported-item categories,
//! docs/help gaps, or rollback failures are accumulating against the last published scoreboard
//! rather than reconstructing data from anecdotal support threads by hand. Every registry row binds
//! a consumer surface to resolved scoreboard and delta entries that reuse the frozen matrix
//! vocabulary, so claim-state or support-language changes cite scoreboard data, docs/help owners
//! identify concrete deltas without manual reconstruction, and no line keeps replacement-grade or
//! daily-driver language ahead of current migration truth.
//!
//! [`m5_supported_line_transparency_report_and_snapshot_diff_registries::M5SupportedLineTransparencyReportSnapshotDiffRegistriesPacket`]
//! turns maintainer / upstream durability into support-line-safe product truth over the frozen
//! supported-line-transparency matrix. It publishes one export-safe *transparency report* per
//! active stable or LTS-candidate line — summarizing critical-upstream status, backup-maintainer
//! coverage, signer-quorum health, emergency-authority coverage, sustainment / sponsor posture,
//! and unresolved red-risk dependencies, with public-safe health separated from internal-only
//! incident / security detail — and one *report snapshot diff* per published snapshot (a
//! health-status change, a coverage narrowing, or a red-risk drift) so support, procurement, and
//! OSS-governance reviews see trend and drift against the prior published report rather than
//! rereading repository-maintenance notes by hand. Every registry row binds a consumer surface to
//! resolved report and diff entries that reuse the frozen matrix vocabulary, so a red-risk upstream
//! or signing gap surfaces on the affected line, public-safe and internal-only variants share one
//! canonical record identity, and no line stays green on stale or opaque upstream health.
//!
//! [`m5_supported_line_public_proof_ledger_and_claim_history_diff_registries::M5SupportedLinePublicProofLedgerClaimHistoryDiffRegistriesPacket`]
//! makes that durable external-proof object model operable over the frozen
//! supported-line-transparency matrix. It publishes one line-by-line *public-proof ledger*
//! per active stable or LTS-candidate line — joining its compatibility report, benchmark /
//! evidence packet, support-window statement, known-limits set, and deprecation / successor
//! report to one supported-line identity with its freshness state, last-versus-current diff,
//! and the exact evidence-packet refs currently backing its public claims — and one
//! *claim-history diff* per proof change (a freshness change, a scope narrowing, or a
//! release-line reassociation) so a stale or mismatched compatibility / benchmark /
//! known-limits / deprecation asset becomes a typed diff event rather than an implicit docs
//! mismatch. Every registry row binds a consumer surface to resolved ledger and diff entries
//! that reuse the frozen matrix vocabulary, so docs / help / About and support / procurement
//! packet builders read one canonical, freshness-checked source that shows current-versus-
//! previous claim-state history and can never widen a claim because a report once existed or
//! leave public-proof, migration, or history unjoined from exact build and release-line
//! identity.
//!
//! [`m5_historical_reference_matrix::M5HistoricalReferenceMatrixPacket`] opens B149 by freezing
//! Aureline's non-live-evidence object model — the retirement / last-supported snapshots, captured
//! support / export evidence bundles, archived runbook execution packets, imported / offline route
//! evidence, and review / incident snapshots that no longer point at live mutable state — into one
//! export-safe matrix. It binds every governed class to one shared historical-reference role taxonomy
//! (snapshot_labeling, capture_time_attribution, provenance_attribution, mutation_blocked_posture,
//! live_target_handoff, imported_offline_disclosure, expiry_removal_handling) and required visible state
//! (snapshot label, capture time, provenance, live-target availability, imported / offline status,
//! mutation-blocked posture, and expiry / removed handling), makes captured / archived and imported /
//! offline evidence mechanically distinct from ordinary live objects, read-only cached current state, and
//! restore-capable workspaces, and binds back to the already-landed stable-proof-index and
//! migration-task-row packets, so archived or imported evidence never looks live / writable / current by
//! omission, no live target is reopened from a snapshot without validating identity, trust, route, and
//! authority, no expired / removed artifact is dead-linked when metadata / provenance / cleanup state can
//! be shown, and non-live evidence stays joined to capture time, provenance, retention state, and any
//! current live-target mismatch.
//!
//! [`m5_historical_snapshot_descriptor_and_change_diff_registries::M5HistoricalSnapshotDescriptorChangeDiffRegistriesPacket`]
//! is the first B149 implement lane over that frozen historical-reference matrix. It makes the two
//! descriptor-bearing classes — a retirement snapshot and a captured support / export evidence packet —
//! operable by turning the historical-snapshot-descriptor grammar (one machine-readable descriptor per
//! preserved object: canonical object ID and source class, capture time, producer / build identity,
//! provenance lineage and trust class, retention / removal state, and analysis-only / reopenable /
//! metadata-only disposition bound to capture-context joins) and the descriptor-change-diff grammar (a
//! producer-build change, a target-link change, or a retention-state change) into registry resolvers that
//! emit export-safe, honest projections. Each registry row binds a shell / archive-viewer, help / docs,
//! support, review / incident, runbook-archive, or companion / export surface to resolved descriptor and
//! change-diff entries across the canonical, accessible, and audit resolution forms, so at least one
//! retirement snapshot and one support / export packet emit a descriptor with stable IDs and provenance
//! joins, non-live state / capture time / live-target references surface without hand-authored duplicate
//! prose, and a changed producer build, target link, or retention state produces a visible descriptor diff
//! instead of a silent mutation. Registry-A reuses the matrix historical-snapshot-descriptor domain schema
//! and Registry-B mints the descriptor-change-diff domain schema fresh.
//!
//! [`m5_archived_snapshot_viewer_and_analysis_only_banner_consumers::M5ArchivedSnapshotViewerConsumersPacket`]
//! is the B149 archive-consumer lane over that frozen historical-reference matrix. Where the matrix and the
//! descriptor lane describe what is preserved, this lane proves how it is shown: every archive-bearing
//! surface — a support bundle viewer, a retirement snapshot page, a review / incident evidence reopen flow,
//! and the shell, help / docs, runbook-archive, release-center, companion / export, program-governance, and
//! CLI / export consumers among them — frames a preserved snapshot with one canonical archive/state banner
//! and fact grid (snapshot label, capture time, provenance, analysis-only posture, and the exact action set
//! allowed on archived evidence). It binds each preserved-evidence profile to the surfaces that render it and
//! proves — by fixtures, not screenshots — that the same profile presents the same banner grammar wherever it
//! appears, that inspect / compare / export-evidence are always available while a discoverable
//! open-current-live-object action appears only where the live target still exists (mutation affordances are
//! disabled by construction), that no surface presents a write-capable control as if the current object were
//! open live, reopens a live target without validating identity / trust / route / authority, dead-links an
//! expired or removed artifact, leaves non-live evidence unjoined to its capture context, or lets archived /
//! imported evidence look live by omission, and that keyboard focus and screen-reader announcement can
//! discover the non-live state, provenance, and open-live-target action without pointer-only chrome.
//!
//! [`m5_historical_versus_live_compare_flow::M5HistoricalVersusLiveCompareFlowPacket`]
//! is the B149 historical-vs-live compare-flow lane over that frozen historical-reference matrix. Where the
//! archive-viewer lane proves how a single preserved snapshot is shown as non-live, this lane proves how a
//! preserved snapshot is compared against its current live object without collapsing the two into one
//! ambiguous view: every compare surface pairs a historical snapshot with its live target, labels identity,
//! freshness, and drift, and — when the pairing is only approximate, its target is missing, or the pairing is
//! policy-blocked — narrows the comparison with an explicit mismatch reason (missing target, changed scope,
//! changed branch / worktree, retired capability, or unsupported skew) instead of dead-ending or failing
//! silently. The historical side stays mutation blocked while navigation to a validated current live object
//! and export of the comparison packet remain available, and the compare action set is closed and
//! analysis-only (no apply / sync / restore variant) so a compare flow can never imply that applying or
//! syncing the historical snapshot is safe unless an explicit, reviewed mutation handoff takes over. It binds
//! each preserved-snapshot profile to the surfaces that compare it, proves the same profile carries identical
//! historical grammar wherever it appears, and requires keyboard focus and screen-reader announcement to
//! discover the compare state, provenance, and open-live-target action.
//!
//! [`m5_live_target_handoff_packet_and_route_validation::M5LiveTargetHandoffPacket`]
//! is the B149 live-target-handoff implement lane over that frozen historical-reference matrix. It makes
//! "open current live object" a reviewable, validated handoff rather than a hidden jump from non-live evidence
//! into live mutable state: every binding carries a typed, versioned handoff packet (source snapshot id,
//! target identity, required route class, trust / auth prerequisites, requested authority class, and a
//! fallback behavior) and validates target existence, current scope / workset visibility, remote / managed
//! route availability, trust posture, and required auth / approval before completing the pivot. A cleared
//! handoff completes and offers the open-current-live-object action at the validated authority; a blocked
//! handoff reports the exact blocker (target missing, out of scope, route unavailable, trust insufficient,
//! auth / approval missing, retired capability, or policy / lifecycle block) and falls back to a
//! satisfy-prerequisite or metadata-only exit instead of a dead end. The requested authority may never exceed
//! what a direct open would grant, auth / approval prerequisites are named as controlled tokens rather than
//! embedded secrets, and any actual elevation is delegated to a separate, reviewed authority handoff — this
//! lane defines the typed handoff and its validation checks and never bypasses approval, trust, or auth
//! refresh. It binds each preserved-snapshot profile to the surfaces that hand it off, proves the same profile
//! carries identical historical grammar wherever it appears, and requires keyboard focus and screen-reader
//! announcement to discover the handoff state, provenance, and open-live-target action.
//!
//! [`m5_archived_object_expiry_removal_state_and_metadata_fallback::M5ArchivedEvidenceStatePacket`]
//! is the B149 expired / removed / retention-ended / missing-live-target state lane over that frozen
//! historical-reference matrix. It keeps a preserved object honest after its retention window closes, its
//! content is expired or removed, or its live target disappears: every binding carries an explicit lifecycle
//! state (preserved-available, expired, removed, retention-window-ended, missing-live-target, or metadata-only)
//! with a stable label, and every non-available state carries a removal / expiry note naming the reason, a
//! never-omitted explanation, the preserved-metadata note, and a removal attribution joining the outcome to a
//! retention / deletion receipt, a retirement closure ledger, and a support packet manifest. When the content
//! bytes are gone the binding still renders capture time, provenance, and the removal / expiry reason instead of
//! degrading to a generic dead link; a reviewed remove action appears only where a safe cleanup is appropriate
//! and the open-current-live-object action only when the archive is preserved with a live target, so an expired
//! or removed object is never presented as live or current. The support export and matrix CSV preserve the same
//! expired / removed vocabulary the product UI uses, and keyboard focus and screen-reader announcement are
//! required to discover the archived state, provenance, and removal / expiry reason.
//!
//! [`m5_imported_offline_evidence_lineage_propagation::M5ImportedOfflineLineagePacket`]
//! is the B149 cross-surface lineage-propagation lane over that frozen historical-reference matrix. It carries the
//! imported / offline evidence descriptor and its "Showing imported or offline evidence" label into the first
//! downstream consumers that can ingest archived data — companion cards, browser / export handoffs, support
//! packets, and AI explanation / evidence consumers — proving a companion / export surface and a support / AI
//! consumer render the same non-live vocabulary and lineage fields as the primary archive viewer. Every binding
//! joins its lineage back to a source snapshot descriptor and, when the lineage is joinable, its live-target
//! handoff packet, otherwise a metadata-only exit; the consumer action set is closed and analysis-only, so a
//! historical packet can never be ranked, narrated, or summarized as current route, health, or provider truth,
//! and the descriptor names its joins by controlled id rather than embedding a live route or secret, keeping the
//! export free of leaked secrets or stale authority. Keyboard focus and screen-reader announcement are required
//! to discover the non-live boundary, provenance, and lineage join.
//!
//! [`m5_historical_evidence_drill_corpus::M5HistoricalEvidenceDrillCorpusPacket`]
//! is the B149 fixture-corpus + regression-drill lane over that frozen historical-reference matrix. It seeds the
//! reusable corpus QA, release, and support pull to prove the archived-snapshot, imported / offline evidence, and
//! live-target handoff loops stay honest under failure: a last-supported retirement snapshot, a captured support /
//! export evidence bundle, a runbook / incident archived packet, and an imported / offline route packet — each
//! with known provenance and handoff expectations — exercised by six drills that either clear the live-target
//! handoff or block it with an exact, named blocker (missing target, trust block, route unavailable, expired
//! snapshot, or imported / offline evidence only) and fall back to a satisfy-prerequisite or metadata-only exit
//! rather than a dead end. The corpus covers all six historical-reference states and all four handoff outcomes, so
//! a drill can distinguish every exact blocker; each blocker maps into the live-target-handoff module's own
//! [`m5_live_target_handoff_packet_and_route_validation::HandoffBlockerReason`] vocabulary. Every binding binds
//! back to screenshots, an accessibility check, the CLI / support export, and the health dashboard, keeps the
//! non-live grammar and capture-context join, and never dead-links an expired artifact, reopens a live target
//! implicitly, or presents imported / offline evidence as current live truth.
//!
//! [`m5_historical_evidence_surface_certification::HistoricalEvidenceProfileCertificationPacket`]
//! is the closing B149 surface-certification capstone over that frozen historical-reference matrix. After the
//! snapshot-descriptor, archived-snapshot viewer, historical-versus-live compare, live-target-handoff,
//! expiry / removal state, imported / offline lineage-propagation, and drill-corpus implement lanes, it
//! certifies that the shared non-live-evidence truth holds on every claimed M5 support, retirement, incident,
//! review, and export surface. It is keyed on the claimed profile a support engineer, release operator,
//! program-governance owner, or review / incident owner reads a snapshot descriptor, archived packet,
//! imported / offline evidence, or live-target-handoff surface through — a current, fully-attributed non-live
//! evidence lane; a reviewable snapshot-record structure; a disclosed imported / offline-partial profile; an
//! unverified live-target profile; and an unverified expiry / removal-ledger profile — and scores each across
//! nine truth axes (visual, keyboard, screen-reader, high-zoom-reflow, high-contrast, localization, CLI/export,
//! degraded-state, and non-live-evidence-truth behavior). A degraded axis must produce a visible claim
//! narrowing, CLI/export parity must always certify, only a current, fully-attributed non-live-evidence lane
//! may certify a certified non-live-evidence record, and every B149 hard invariant must hold (no archived or
//! imported / offline evidence looks live / writable / current by omission, no live target is reopened from a
//! snapshot without validating identity / trust / route / authority, no expired / removed artifact is
//! dead-linked when metadata / provenance / cleanup state can be shown, no non-live evidence is left unjoined
//! to capture time / provenance / retention state / live-target mismatch, and no snapshot or imported / offline
//! packet is presented as a current live object or reopened through an ambiguous route). Every row cites one
//! canonical historical-reference matrix proof bundle, so support, docs / help, and release / public-proof
//! surfaces ingest the same certification result rather than restating it by hand.
//!
//! [`m5_retired_state_surface_certification::RetiredStateProfileCertificationPacket`]
//! is the closing B148 surface-certification capstone over that frozen retired-state matrix.
//! After the retirement-manifest, impact-report, countdown, review-packet, tombstone,
//! last-supported-snapshot, and closure-ledger / propagation implement lanes, it certifies that
//! retired-state truth holds on every claimed M5 supported line and stable-facing surface —
//! complete retirement manifests, exact-build last-supported snapshots, tombstones and archival
//! routes, closed support notes, and multi-profile propagation — and auto-narrows any profile that
//! cannot sustain it. Keyed on the claimed profile (a live, fully closed retired-state closure lane;
//! a reviewable retirement-record structure; a disclosed archive-partial profile; an unverified
//! propagation profile; and an unverified closure-ledger profile), each row certifies the profile
//! across nine truth axes — visual, keyboard, screen-reader, high-zoom-reflow, high-contrast,
//! localization, CLI/export, degraded-state, and retired-state-truth behavior — and either passes
//! (green), auto-narrows its closure claim to the weakest supported ceiling (yellow), or blocks (red)
//! when a degraded axis hides behind a fresh certified claim, a B148 hard invariant breaks,
//! CLI/export parity drops, or a non-live profile claims a certified retired closure. Only a live,
//! fully closed retired-state closure lane — one whose manifest, last-supported snapshot, tombstone,
//! closure ledger, and deployment-profile propagation all converge on one export-safe record — may
//! certify a certified retired closure, and every row cites the one canonical retired-state matrix
//! proof bundle, so release, help, docs, support, public-proof, marketplace, and partner/procurement
//! surfaces consume one retired-state certification source rather than hand-authored prose. No retired
//! object stays green or actively selectable, no last-supported docs / schemas / evidence are
//! destroyed before support-note closure, and retirement state stays joined to exact build, line
//! identity, deployment profile, and migration outcome.
//!
//! [`m5_retirement_closure_ledger_and_propagation_blocker_gate_registries::M5RetirementClosureLedgerPropagationBlockerGateRegistriesPacket`]
//! propagates retirement manifests, tombstones, and last-supported archive refs into mirror
//! metadata, offline bundle manifests, self-hosted registry / catalog paths, policy bundles, and
//! managed new-tenant / new-workspace gates over the frozen retired-state matrix, so mirrors,
//! offline bundles, self-hosted registries, and managed tenant gates all converge on the same
//! closed retired-state truth. It maintains one export-safe *retirement closure ledger* per
//! retiring object per deployment profile — recording the migration outcome, disable evidence,
//! support-note closure, archival note, propagation status, and any remaining carve-outs joined to
//! its exact-build identity — and one *propagation-blocker gate* per object that blocks final
//! retirement certification while a claimed profile still lags its propagation, diverges from the
//! profiles that already closed the line, or keeps advertising a retired line or capability after
//! another profile closed it. Every registry row binds a consumer surface to resolved closure-ledger
//! and propagation-blocker-gate entries that reuse the frozen matrix vocabulary, so a managed
//! consumer and a mirror / offline / self-hosted consumer agree on retired-state truth for the same
//! object and the propagation names the archival / successor path each profile needs without leaking
//! internal-only detail.
//!
//! [`m5_last_supported_snapshot_and_archive_export_gate_registries::M5LastSupportedSnapshotArchiveExportGateRegistriesPacket`]
//! ships last-supported snapshot and retirement archive bundles for a retiring M5 surface over
//! the frozen retired-state matrix, so migration, audit, procurement, and support can inspect
//! what was retired without keeping the retired surface live forever. It emits one export-safe
//! *last-supported snapshot* per retired object — capturing the docs / help truth, the schemas /
//! contract set, the compatibility report, the known-limits snapshot, the provenance / SBOM
//! reference, and the support-article links for the final supported build or line state, joined
//! to its exact-build identity — and one *archive-export gate* per object that blocks a bundle
//! from being handed off while it still carries a live vendor dependency, would leak a secret or
//! internal-only detail, or is not bound back to the retirement manifest and review packet. Every
//! registry row binds a consumer surface to resolved snapshot and archive-export-gate entries
//! that reuse the frozen matrix vocabulary, so self-hosted, offline, and procurement / support
//! consumers open one export-safe historical reference that names the final supported version /
//! channel and the successor path without contradiction and never keeps a live secret while
//! retaining enough evidence for support, audit, and procurement / reference use.
//!
//! [`m5_retirement_tombstone_and_claim_block_gate_registries::M5RetirementTombstoneClaimBlockGateRegistriesPacket`]
//! adds retired-state tombstones and claim-block logic to the install / update, marketplace,
//! help / About, CLI / headless inspect, and new-tenant surfaces so a retired M5 line or
//! stable-facing surface stops looking selectable or claimable while staying discoverable
//! historically over the frozen retired-state matrix. It emits one export-safe *retirement
//! tombstone* per retired object — carrying the stable identity anchor, the last-supported
//! version marker, the archival pointer, the replacement / successor path, and the removed
//! active-selection affordance so green / support badges and active enablement are gone but the
//! object keeps its stable ID, last supported version, and archive / replacement pointer — and
//! one *claim-block gate* per object that blocks it from being offered for new install, new
//! tenant, or active enablement. Every registry row binds a consumer surface to resolved
//! tombstone and claim-block-gate entries that reuse the frozen matrix vocabulary, so help /
//! About, marketplace, and CLI / headless inspection agree on one retired-state truth for the
//! same object and a retired surface never disappears without a tombstone, successor pointer, or
//! archival route.
//!
//! [`m5_retirement_review_packet_and_closure_gate_registries::M5RetirementReviewPacketClosureGateRegistriesPacket`]
//! forces one typed retirement review packet before a line or stable-facing surface can move to
//! `Retired`, so retirement stops being an ad hoc decision buried in release notes. It emits one
//! export-safe *retirement review packet* per candidate — joining the migration outcome summary,
//! the final compatibility / public-proof join, the exact-build snapshot refs, the support-note
//! closure status, the archival signoff refs, and any unresolved dependent blockers to one
//! candidate identity — and one *support-note closure gate* per candidate that blocks final
//! retirement while the packet is missing its migration outcome or archival refs, still has an
//! unclosed help / support / partner / procurement / incident surface, or would silently drop a
//! recorded exception. Every registry row binds a consumer surface to resolved review-packet and
//! closure-gate entries that reuse the frozen matrix vocabulary, so support, help, and
//! public-proof consumers read the closure state directly from the packet instead of relying on
//! free-text release notes, and no object reaches `Retired` without a completed packet that
//! records who approved it, what evidence was accepted, which surfaces were closed or redirected,
//! and what exceptions remain.
//!
//! [`m5_retirement_countdown_and_safety_gate_registries::M5RetirementCountdownSafetyGateRegistriesPacket`]
//! turns retirement from a hidden date in release notes into a visible, inspectable countdown
//! with an explicit successor path and safe exit steps over the frozen retired-state matrix. It
//! emits one export-safe *retirement countdown* per affected install / update, settings / help,
//! docs, marketplace, and support surface — carrying the first-deprecated version, cutoff
//! version / date, successor route, any remaining overlap window, and a no-surprises explanation
//! of what changes at retirement — and one *pre-retirement safety gate* per candidate that
//! blocks final closure while the candidate is still missing its declared rollback / export
//! path, archive bundle, or successor / fallback route. Every registry row binds a consumer
//! surface to resolved countdown and safety-gate entries that reuse the frozen matrix
//! vocabulary, so at least one product surface and one operator / support surface open the same
//! cutoff and successor data without contradiction and no surface transitions to Retired through
//! a surprise shutdown.
//!
//! [`m5_retirement_impact_report_and_blocker_gate_registries::M5RetirementImpactReportBlockerGateRegistriesPacket`]
//! makes retirement safe by proving who and what still depends on a retiring surface before
//! Aureline closes the support window. It emits one export-safe *retirement impact report* per
//! retirement candidate — classifying each detected dependency (a workflow bundle, migration
//! pack, command / deep link, CLI alias, SDK contract row, saved artifact, profile, recipe,
//! marketplace entry, mirror, or managed / new-tenant offering) as blocking, migration-required,
//! historical-only, mirror-only, tenant-gated, or informational with a typed reason and owning
//! team — and one *impact blocker gate* per candidate that blocks closure while a bundle, a
//! tenant, or a schema / public artifact still points at the retiring surface. Every registry
//! row binds a consumer surface to resolved impact-report and blocker-gate entries that reuse
//! the frozen matrix vocabulary, so review packets, support exports, and public-proof consumers
//! read one canonical retirement blast-radius report joined to the successor path or manual
//! fallback rather than a hand-authored parallel prose.
//!
//! [`m5_retirement_manifest_and_change_diff_registries::M5RetirementManifestChangeDiffRegistriesPacket`]
//! makes that terminal-lifecycle object model operable over the frozen retired-state matrix.
//! It emits one machine-readable *retirement manifest* per retiring class — joining the object
//! identity, last-supported version or channel, retirement trigger, cutoff date, successor
//! reference, disable path, and export / rollback route to one class identity with its
//! exact-build joins — and one *manifest change diff* per changed manifest (a cutoff-date
//! change, a replacement-path change, or a disable / export-route change) so a changed cutoff
//! date or replacement path becomes a visible, typed diff event rather than a silent mutation.
//! Every registry row binds a consumer surface to resolved manifest and change-diff entries
//! that reuse the frozen matrix vocabulary, so CLI, docs / help, partner packets, and support
//! bundles read one canonical retirement object that exposes successor and rollback / export
//! truth without hand-authored parallel prose staying consistent.
//!
//! [`m5_retired_state_matrix::M5RetiredStateMatrixPacket`] freezes Aureline's
//! terminal-lifecycle object model — the supported lines, stable-facing capabilities,
//! bundles, commands / deep links, schema-bearing surfaces, registry-visible packages, and
//! managed / new-tenant-gated features that must move from `Deprecated` to `Retired`
//! cleanly, the required transition metadata each carries (last-supported version or
//! channel, cutoff date, successor path, disable path, export / rollback route, archival
//! note, migration outcome, and support-note closure state), and the closure artifacts each
//! covered class owns — into one export-safe matrix. It binds every governed class to one
//! shared retirement-role taxonomy (last_supported_pin, successor_routing, disable_path,
//! export_rollback_route, archival_note, migration_outcome, support_note_closure), makes
//! `Retired` mechanically distinct from `Deprecated`, `DisabledByPolicy`, and ordinary
//! stable-line narrowing, and binds back to the already-landed stable-proof-index and
//! migration-task-row packets, so no retired surface disappears without a tombstone,
//! archival route, or successor pointer, no retired class stays selectable in a new-install /
//! new-tenant / marketplace / upgrade flow, last-supported docs / schemas / evidence survive
//! until support-note closure and export-safe archive handoff, and retirement state stays
//! joined to exact build, line identity, deployment profile, and migration outcome.
//!
//! [`m5_supported_line_transparency_matrix::M5SupportedLineTransparencyMatrixPacket`]
//! freezes Aureline's durable post-launch external-proof object model — its supported-line
//! proof taxonomy (the public-proof ledger, the transparency report, the migration
//! scoreboard, the ORR-history event, and the correction-train archive), their public-safe
//! versus internal-only visibility posture, and the freshness window, owner, export class,
//! and supported-line association each object must carry — into one export-safe matrix. It
//! binds every governed proof object to one shared transparency-role taxonomy
//! (freshness_window, transparency_disclosure, migration_scoreboard_currency,
//! orr_history_retention, correction_archive_retention, public_proof_freshness,
//! correction_history_join), to the widening stages (alpha, beta, RC, stable, LTS) each
//! object must gate, and back to the already-landed stable-proof-index and migration-task-row
//! packets, so no supported line stays green on stale external proof or opaque upstream
//! health, migration pain stays scored and versioned, ORR and correction history stays
//! retained and archived, transparency reports stay export-safe with no internal-only
//! leakage, public-proof / migration / history stay joined to exact build and release-line
//! identity, and support language never outruns current public proof rather than reading as
//! green.
//!
//! [`m5_stable_line_protection_matrix::M5StableLineProtectionMatrixPacket`] freezes
//! Aureline's concrete post-stable operating model — its stable-line-protection taxonomy
//! (the fresh stable line, the evidence-refresh line, the correction/backport line, the
//! launch-bundle-currentness line, and the LTS-candidate line), the support windows,
//! correction-line owners, backport-decision SLAs, evidence-refresh cadences,
//! bundle-refresh obligations, and LTS-eligibility state each active line must carry —
//! into one export-safe matrix. It binds every governed line to one shared
//! stable-line-protection-role taxonomy (support_window, correction_ownership,
//! evidence_refresh, backport_decision, lts_eligibility, bundle_currentness, defect_ledger),
//! to the widening stages (alpha, beta, RC, stable, LTS) each line must gate, and back to
//! the already-landed claim-manifest and release-center packets, so no shipping line drifts
//! on stale evidence or frozen launch bundles, supported-line defects stay owned and
//! resolved within SLA, backport decisions stay documented rather than tribal memory,
//! evidence refresh stays ordinary release ops, LTS remains a checked-in decision packet
//! backed by current rollback and support evidence, and support language never outruns
//! current refresh and correction proof rather than reading as green.
//!
//! [`m5_launch_control_matrix::M5LaunchControlMatrixPacket`] freezes Aureline's
//! concrete launch-control model — its dogfood-ring / certification-cohort taxonomy
//! (core-team canary, design-partner preview, extension-author, public preview, and
//! certified-archetype cohorts), its readiness events, its rehearsal cadence, its
//! freeze-exception packets, and its explicit go/no-go decisions — into one export-safe
//! matrix. It binds every governed cohort to one shared launch-control-role taxonomy
//! (cohort_membership, readiness_event, rehearsal_currency, freeze_exception_authority,
//! go_no_go_authority, rollback_stop, regression_asset), to the widening stages (alpha,
//! beta, RC, stable, LTS) each cohort must gate, and back to the already-landed
//! cohort-scoreboard and freeze-exception packets, so no stable claim skips cohorts, ring
//! widening depends on current known-limits and rollback-stop rules, Sev-1/Sev-2 incidents
//! generate a linked regression asset before close-out, ORR / publish-rollback /
//! mixed-version / advisory-revocation / support-handoff drills stay current, freeze
//! exceptions stay documented rather than implicit scope widening, go/no-go decisions
//! preserve the exact evidence snapshot and signoff roster, and partner and public support
//! language never outruns current cohort proof rather than reading as green.
//!
//! [`m5_cohort_descriptor_and_evidence_packet_registries::M5CohortDescriptorEvidencePacketRegistriesPacket`]
//! is the first implement lane over that frozen launch-control matrix: it turns the cohort-descriptor grammar
//! (how a widening cohort declares the exact repo / archetype rows, bundle IDs, install topology, toolchain
//! envelope, known limits, rollback target, and diagnostics posture it is auditable by — the dogfood core-team
//! canary, migration-alpha, extension-author, design-partner-preview, public-preview, and certified-archetype
//! archetype it classifies) and the cohort-evidence-packet grammar (how a launch-bearing lane proves which
//! cohort evidence — dogfood-ring telemetry, current rehearsal cadence, or an explicit go/no-go signoff — backs
//! it, keeping partner / public support language and known-limits packets bound to that proof rather than to
//! hand-edited prose) into registry resolvers, so every claimed M5 launch-bearing cohort resolves to one typed
//! cohort-descriptor object and one cohort-evidence-packet object that the shiproom, release-center,
//! executive-steering, program-governance, and support / export surfaces inspect without manual reconstruction,
//! so a cohort can never widen without preserving its rollback and diagnostics posture, partner / public support
//! language never runs ahead of cohort proof, the exact rows / bundles / toolchains / deployment profiles stay
//! visible before widening, and a cohort that cannot explain the descriptor it declared or the evidence that
//! backs it degrades honestly instead of reading as a clean pass.
//!
//! [`m5_ring_progression_and_rollback_stop_registries::M5RingProgressionRollbackStopRegistriesPacket`]
//! governs ring widening by explicit stop conditions rather than schedule optimism over that same frozen
//! launch-control matrix: it turns the ring-progression grammar (how each widening transition — canary, broad
//! internal dogfood, design-partner preview, public preview, and certified stable — declares its minimum entry
//! evidence, soak-window expectation, why widening is allowed, its known-limits packet, issue-template linkage,
//! claim-narrowing action, and the rollback-stop reference that immediately stops it) and the rollback-stop
//! grammar (how a launch-bearing lane records the rollback-stop condition — a crash / data-loss / trust defect, a
//! repeated protected-metric regression, or a stale readiness packet — that halts ring progression while it is
//! active) into registry resolvers, so every ring transition can state why widening is allowed and what
//! immediately stops it, known-limits and rollback posture stay visible before any ring widens, ring progression
//! can never advance on a claimed lane while a rollback-stop condition is active, and a ring that cannot explain
//! its progression rule or the stop condition that backs it degrades honestly instead of reading as a clean pass.
//!
//! [`m5_regression_asset_and_incident_close_registries::M5RegressionAssetIncidentCloseRegistriesPacket`]
//! governs incident-close regression-asset requirements over that same frozen launch-control matrix: it turns the
//! regression-asset grammar (how each Sev-1 / Sev-2 incident or launch-bearing failure links a regression asset —
//! an automated test, a fixture repository, a recovery drill, a protected-corpus case, a schema/policy guard, or a
//! monitoring regression check — and preserves the exact build, affected row, cohort/ring, and workaround lineage
//! on that asset before closure) and the incident-close grammar (how a severe incident records the linked
//! regression asset, the exact build and affected row, the cohort/ring lineage, and the close-lineage freshness
//! that keeps it queryable) into registry resolvers, so a severe incident can never close without an attributable
//! regression asset or an explicit approved exception, regression assets stay linked to the lane / cohort / build
//! that exposed the defect, incident-close lineage stays queryable without tribal memory, and an incident that
//! cannot show the regression asset it linked or the lineage that backs it degrades honestly instead of reading as
//! a clean pass.
//!
//! [`m5_freeze_exception_and_go_no_go_registries::M5FreezeExceptionGoNoGoRegistriesPacket`]
//! governs phase-level change budgets, freeze-exception packets, and explicit channel-widening go/no-go decisions
//! over that same frozen launch-control matrix: it turns the freeze-exception grammar (how each governed change
//! class — phase-allowed, exception-required, api/contract, scope-widening, migration/data, and
//! dependency/toolchain change — carries its exception scope, rollback/narrowing, docs/support/migration, and
//! owner/risk capture so a freeze exception can never become undocumented scope widening) and the go/no-go grammar
//! (how a launch-bearing lane records the go / no-go / conditional-go decision with the preserved evidence
//! snapshot, ORR signoff, named on-call roster, and authorized widening stage that justified widening) into
//! registry resolvers, so no item enters committed scope or widening readiness without the required B145 fields,
//! freeze exceptions stay exportable attributable packets rather than chat-only approvals, milestone accounting can
//! distinguish integrated work from done work, and a lane that cannot show the change budget it scoped or the
//! go/no-go evidence that backs it degrades honestly instead of reading as a clean pass.
//!
//! [`m5_orr_review_and_rehearsal_drill_registries::M5OrrReviewRehearsalDrillRegistriesPacket`]
//! exercises launch-bearing lanes before widening over that same frozen launch-control matrix: it turns the
//! operational-readiness-review grammar (how each ORR / rehearsal packet kind — monthly ORR, release-candidate
//! ORR, publish/rollback drill, mixed-version drill, advisory/revocation drill, and support/incident handoff
//! drill — names its readiness scope, its release / advisory / support-room / docs-comms / backup-signer role
//! roster, and its rehearsal-freshness expiry so a stable claim can never widen on a stale, skipped, or
//! contradictory rehearsal packet) and the rehearsal-drill grammar (how a launch-bearing lane records the roster
//! coverage — full roster, backup roster, or conditional roster — with the preserved ORR signoff, the named
//! on-call roster, and the rehearsal-freshness state that justified widening) into registry resolvers, so every
//! claimed launch-bearing lane points at current ORR and rehearsal packets, rehearsal freshness and role coverage
//! read as first-class shiproom and release blockers, stable/LTS promotion halts automatically when a lane's
//! rehearsal state is red or stale, and a lane that cannot show the rehearsal packet it ran or the roster that
//! covered it degrades honestly instead of reading as a clean pass.
//!
//! [`m5_widening_decision_and_ring_history_registries::M5WideningDecisionRingHistoryRegistriesPacket`]
//! makes every claimed stable-widening event reconstructible after the fact over that same frozen
//! launch-control matrix: it turns the stable go/no-go decision-record grammar (how each widening event — an
//! alpha, beta, release-candidate, stable, long-term-support, or correction-reissue widening — records its final
//! go/no-go decision, its open risks, its narrowed claims, its named on-call and signoff roster, and the exact
//! evidence snapshot that justified widening) and the ring-history-snapshot grammar (how a launch-bearing lane
//! preserves the ring history, the prior blockers, and the previous packet freshness — a ring-history, a
//! prior-blocker, or a packet-freshness snapshot scope — with the preserved evidence snapshot, signoff, and named
//! on-call roster) into registry resolvers, so every claimed widening event resolves to one durable go/no-go
//! record tied to exact evidence and roster state, later incident or support review can reconstruct why a lane
//! widened without reading ad hoc meeting notes, shiproom and correction-line flows consume the same record
//! rather than duplicating decision state, and a record that has dropped its evidence snapshot, roster, or ring
//! history degrades honestly instead of reading as a clean pass.
//!
//! [`m5_launch_control_surface_certification::LaunchControlProfileCertificationPacket`]
//! is the closing B145 surface-certification capstone over that frozen launch-control
//! matrix: after the 1213–1218 implement lanes resolve the cohort-descriptor, ring-progression,
//! rollback-stop, regression-asset, incident-close, freeze-exception, go/no-go, ORR-review,
//! rehearsal-drill, widening-decision, and ring-history registries, it certifies that the shared
//! launch-control truth holds on every claimed M5 launch-bearing widening profile (a live certified
//! widening lane, a reviewable launch-control structure, a disclosed freeze-exception profile, an
//! unverified rehearsal-currency profile, and an unverified regression-asset profile). Each profile
//! is scored across nine truth axes and either passes (green), auto-narrows its widening claim to the
//! weakest supported ceiling with a bound reason and frozen downgrade trigger (yellow), or blocks
//! (red) when a degraded axis hides behind a fresh certified claim, a B145 hard invariant breaks
//! (widening a stable claim without current cohort and rehearsal evidence, letting a freeze exception
//! become undocumented scope widening, closing a Sev-1/Sev-2 incident without a regression asset,
//! implying green while go/no-go or ORR records are stale, or maintaining partner/public support
//! language that outruns current cohort proof), CLI/export parity drops, or a non-live profile claims
//! a certified widening lane, so shiproom, docs, support, and public-proof surfaces consume one
//! launch-control certification source rather than hand-authored prose.
//!
//! [`m5_build_lane_trust_matrix::M5BuildLaneTrustMatrixPacket`] freezes Aureline's
//! concrete build-farm trust domains, remote-cache discipline, clean-room rebuild proof,
//! and exact-build supportability — the contributor / PR lane, the protected-merge lane,
//! the release lane, and the emergency-hotfix lane — into one export-safe matrix. It
//! binds every governed lane to one shared build-lane-trust-role taxonomy (cache_posture,
//! publication_authority, credential_boundary, hermetic_input, reproducibility_proof,
//! artifact_convergence, support_identity) and to the lane-specific contributor / PR,
//! protected-merge, release, and emergency-hotfix vocabularies, and back to the
//! already-landed artifact-publication and reproducible-RC packets, so contributor lanes
//! read shared caches but never publish release artifacts, protected-merge lanes stay on
//! controlled credentials and verified caches, release and emergency-hotfix lanes converge
//! binaries / packages / SBOMs / symbols / docs on one exact build identity, remote-cache
//! hits are never treated as reproducibility proof, sidecars stay pinned to the binary
//! build identity, clean-room parity is never overclaimed on partial rebuilds, and
//! non-hermetic inputs, cache poisoning, and unreplayable artifacts block promotion rather
//! than hiding behind green publication rows.
//!
//! [`m5_build_lane_descriptor_and_reproducibility_proof_registries::M5BuildLaneDescriptorReproducibilityProofRegistriesPacket`]
//! is the first implement lane over that frozen build-lane-trust matrix: it turns the build-lane-descriptor
//! grammar (how a lane declares its allowed cache reads / writes, its controlled credential class, its
//! publication rights, and the artifact families it is expected to produce) and the reproducibility-proof
//! grammar (how a release or emergency-hotfix lane proves its inputs came from a verified cache or were
//! re-materialized and that binaries, packages, SBOMs, symbols, docs, and rollback metadata converge on one
//! exact build identity) into registry resolvers, so every claimed M5 build lane resolves to one typed
//! build-lane-descriptor object — the cache posture it classifies, the cache read scope, the cache write
//! scope, the controlled credential class, the publication rights it is bounded to, the expected artifact
//! families, the hermetic-input posture, and the clean-room rebuild rule — and to one reproducibility-proof
//! object — the resolved exact build identity, the verified-versus-re-materialized input-source ledger, the
//! clean-room rebuild diff reference, the sidecar-convergence state, the attestation state, the
//! rollback-metadata reference, and the last rebuild revision — that the release-center, shiproom,
//! diagnostics, provenance, and support / export surfaces inspect without manual reconstruction, so an
//! untrusted lane can never publish a release artifact, a remote-cache hit is never treated as reproducibility
//! proof, the cache / credential / publication boundary stays visible before promotion, and a build lane that
//! cannot explain the descriptor it declared or the build identity it converged on degrades honestly instead
//! of reading as a clean pass.
//!
//! [`m5_verified_input_manifest_and_sidecar_completeness_registries::M5VerifiedInputSidecarCompletenessRegistriesPacket`]
//! is the input-materialization and sidecar-completeness implement lane over that frozen build-lane-trust matrix:
//! it turns the verified-input-manifest grammar (how a lane captures the build-config digest, the
//! materialized-input receipt, the input provenance ledger, the verification authority it is bounded to, the
//! artifact families it expects, the hermetic-input posture, and the re-materialization rule) and the
//! sidecar-completeness-manifest grammar (how a release or emergency-hotfix lane proves that binaries, packages,
//! docs packs, schemas, SBOMs, symbols, source maps, and rollback metadata are all present and bound to one exact
//! build identity) into registry resolvers, so every claimed M5 build lane resolves to one typed
//! verified-input-manifest object — the input source it classifies, the build-config digest, the
//! materialized-input receipt, the input provenance ledger, the verification authority it is bounded to, the
//! expected artifact families, the hermetic-input posture, and the re-materialization rule — and to one
//! sidecar-completeness-manifest object — the resolved exact build identity, the claimed artifact families, the
//! sidecar-family ledger, the binding-identity check, the missing-or-mismatched reference, the attestation state,
//! and the last convergence revision — that the release-center, shiproom, diagnostics, provenance, and support /
//! export surfaces inspect without manual reconstruction, so an unverified input can never enter a protected
//! lane, a missing or mismatched sidecar is never treated as a warning-only state, the build-config-digest /
//! receipt / verification boundary stays visible before promotion, and a build lane that cannot explain the
//! manifest it declared or prove its sidecars converge on one build identity degrades honestly instead of reading
//! as a clean pass.
//!
//! [`m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries::M5CleanRoomRebuildArtifactDiffRegistriesPacket`]
//! is the clean-room-rebuild and artifact-diff implement lane over that frozen build-lane-trust matrix: it turns
//! the clean-room-rebuild-lane grammar (how a protected lane replays its inputs without relying on shared
//! remote-cache state as authority — the rebuild source it classifies, the rebuild-config digest, the replay
//! receipt, the protected-input ledger, the rebuild authority it is bounded to, the artifact families it expects,
//! the hermetic-rebuild posture, and the shared-cache isolation rule) and the artifact-diff-packet grammar (how a
//! release or emergency-hotfix lane emits a deterministic diff comparing rebuild outputs across every claimed
//! artifact family — binaries, packages, docs packs, schemas, SBOMs, symbols, source maps, and rollback metadata —
//! so a material divergence or an omitted family is a blocker rather than a warning) into registry resolvers, so
//! every claimed M5 build lane resolves to one typed clean-room-rebuild-lane object and one artifact-diff-packet
//! object that the release-center, shiproom, diagnostics, provenance, and support / export surfaces inspect
//! without manual reconstruction, so a clean-room rebuild can never rely on a shared remote cache as authority, a
//! material artifact divergence is never treated as a warning-only state, clean-room parity is never overclaimed
//! on a partial rebuild, and a lane that cannot replay its inputs or prove its artifact diff is deterministic
//! degrades honestly instead of reading as a clean pass.
//!
//! [`m5_remote_cache_integrity_and_cache_bypass_drill_registries::M5CacheIntegrityBypassRegistriesPacket`]
//! is the remote-cache-poisoning-detection and cache-bypass-drill implement lane over that frozen
//! build-lane-trust matrix: it turns the remote-cache-integrity-finding grammar (how a protected lane labels the
//! origin of each input — a verified trusted cache, a re-materialized-from-source input, a pinned-digest input, an
//! untrusted shared cache, or a non-hermetic ambient input — and captures the cache-scope digest, the
//! dependency-capture receipt, the captured-input ledger, the hermeticity authority it is bounded to, the artifact
//! families it expects, the hermetic-input posture, and the cache-origin trust rule so a poisoned or untrusted
//! cache hit and an uncaptured non-hermetic input never silently enter a protected lane) and the
//! cache-bypass-drill grammar (how a release or emergency-hotfix lane rehearses a full-cache-bypass rebuild, a
//! partial re-materialization replay, or a hermetic-from-scratch drill and proves that binaries, packages, docs
//! packs, schemas, SBOMs, symbols, source maps, and rollback metadata stay replayable on one exact build identity,
//! so an unreplayable artifact family or an omitted family is a blocker rather than a warning) into registry
//! resolvers, so every claimed M5 build lane resolves to one typed remote-cache-integrity-finding object and one
//! cache-bypass-drill object that the release-center, shiproom, diagnostics, provenance, and support / export
//! surfaces inspect without manual reconstruction, so a non-hermetic or uncaptured input can never enter a
//! protected lane, an unreplayable artifact family is never treated as a warning-only state, the cache-scope
//! digest / receipt / hermeticity boundary stays visible before promotion, and a lane that cannot capture its
//! inputs or prove its cache-bypass drill stays replayable degrades honestly instead of reading as a clean pass.
//!
//! [`m5_exact_build_symbolication_and_mirror_offline_parity_registries::M5SymbolicationMirrorParityRegistriesPacket`]
//! is the exact-build-symbolication and mirror/offline-publication-parity implement lane over that frozen
//! build-lane-trust matrix: it binds crash packets, symbol / source-map manifests, support bundles, and
//! symbolication reports to the same exact-build descriptor B144 produces (naming the build-config digest, the
//! materialized-input receipt, the input provenance ledger, the verification authority, the expected artifact
//! families, the hermetic-input posture, and the re-materialization rule so a support bundle can explain exact
//! versus approximate symbolication against one build identity and an unverified input never enters a protected
//! lane) and proves that mirrored / offline publication preserves the same build identity and freshness
//! vocabulary as default publication across binaries, packages, docs packs, schemas, SBOMs, symbols, source
//! maps, and rollback metadata, so a mismatched build ID or an omitted family is a blocker rather than a
//! warning. Every claimed M5 build lane resolves to one typed exact-build-symbolication object and one
//! mirror-offline-parity object that the release-center, shiproom, diagnostics, provenance, and support / export
//! surfaces inspect without manual reconstruction, so support / export or mirror publication fails when
//! symbolication or build identity cannot be proven exactly and a supportability drill catches mismatched build
//! IDs before stable promotion instead of reading as a clean pass.
//!
//! [`m5_build_lane_trust_shared_consumers_one_registry_across_surfaces::M5BuildLaneTrustSharedConsumersPacket`]
//! is the consumer-adoption capstone over that frozen build-lane-trust matrix: it binds each of the four
//! governed build lanes — the contributor / PR lane, the protected-merge lane, the release lane, and the
//! emergency-hotfix lane — to the concrete About / provenance, Help, service-health, release-center, and
//! support-export consumers (projected through the build-farm, cache-service, release-center, shiproom,
//! provenance-service, diagnostics, docs / help, CLI / export, and support-export surfaces) that render it,
//! and proves by fixtures rather than screenshots that the same build profile presents the same
//! build-lane-trust-role, family, registry-reference, build-context, surface-context, and replay-continuity
//! grammar wherever it appears, so build lane, cache posture, clean-room parity, stale-proof state, and
//! mirror / offline build identity read consistently across every consumer. Each shared lane is adopted by at
//! least two distinct consumers, the build-lane-trust-role word stays a frozen `M5BuildLaneTrustRole` token so
//! no surface reinvents `cache_posture`, `publication_authority`, `reproducibility_proof`, or
//! `artifact_convergence` in its own words, support / export consumers point back at the canonical per-domain
//! schema and the frozen matrix by id, and narrowing is disclosed through an explicit note rather than hidden —
//! so a PR cache never publishes release artifacts, a remote-cache hit is never treated as reproducibility
//! proof, a sidecar never drifts from the binary build identity, clean-room parity is never overclaimed on a
//! partial rebuild, and stale or contradictory B144 evidence narrows the claim rather than reading as a clean
//! pass.
//!
//! [`m5_build_lane_trust_surface_certification::BuildLaneTrustProfileCertificationPacket`]
//! is the closing B144 surface-certification capstone over that frozen build-lane-trust matrix: it certifies
//! that the shared build-lane-trust truth holds on every claimed M5 RC / stable / LTS / mirror-offline
//! publication-bearing profile — a live exact-build supportable lane, a reviewable reproducibility structure, a
//! disclosed cache-discipline profile, an unverified clean-room-parity profile, and an unverified
//! exact-build-supportability profile — scoring each across nine truth axes (visual, keyboard, screen-reader,
//! high-zoom-reflow, high-contrast, localization, CLI/export, degraded-state, and build-lane-trust-component
//! truth) and either passing it (green), auto-narrowing its publication claim to the weakest supported ceiling
//! (yellow), or blocking it (red) when a degraded axis is hidden behind a fresh trusted claim inherited from a
//! healthier profile. Only a live, first-party fully reproducible release lane may certify a trusted exact-build
//! supportable lane, the always-on CLI/export axis must always stay certified so support and automation can
//! reconstruct the build lane, cache posture, publication authority, exact build identity, clean-room rebuild
//! diff, reproducibility proof, sidecar convergence, support packet, and registry reference, and every B144 hard
//! invariant is enforced per row — so no publication lane can advertise full exact-build or clean-room
//! supportability while a PR cache could publish release artifacts, a remote-cache hit is treated as
//! reproducibility proof, a sidecar drifts from the binary build identity, clean-room parity is overclaimed on a
//! partial rebuild, or B144 evidence is stale or missing. Every certified profile cites the one canonical
//! build-lane-trust matrix proof bundle rather than cloning per-profile evidence.
//!
//! [`m5_setting_capability_lifecycle_and_kill_switch_registries::M5SettingCapabilityLifecycleKillSwitchRegistriesPacket`]
//! is the capability-lifecycle / kill-switch implement lane over that frozen settings-governance matrix: it turns
//! the capability-record grammar (how a capability record, Labs enrollment, rollout plan, and dependency marker
//! declare the lifecycle state, the owner, the scope, the review / expiry window, the enabled posture, the
//! artifact dependency marker, the fallback, and the rollback note a capability carries) and the
//! kill-switch-record grammar (how a kill-switch or policy-disable record names the disabling source, the
//! disabled timestamp, the preserved user-authored data, the self-explanation, the capability dependency, the
//! fallback, and the last revision for a kill-switch, policy-disable, dependency-unavailable, review-expired, or
//! manual-opt-out disable) into registry resolvers, so every claimed M5 capability resolves to one
//! capability-record object — the lifecycle state it classifies (Labs / Preview / Beta / generally-available /
//! graduated / deprecated), the owner, the scope, the review / expiry window, the enabled posture, the dependency
//! marker, the fallback, and the rollback note — and every claimed disable resolves to one kill-switch-record
//! object — the disabling source, the disabled timestamp, the preserved-data reference, the explanation
//! reference, the capability dependency, the fallback reference, and the last ledger revision — that the
//! settings, docs / help, bundle, import-apply, and support / export flows inspect before a claim publishes
//! without manual reconstruction, so a stable-facing surface never depends on a hidden Labs / Preview capability
//! without an explicit dependency marker and fallback, a lifecycle or experiment dependency never disappears
//! behind unpublished markers, a kill switch or policy disable always preserves user-authored data and explains
//! its cause, and a lifecycle flow that cannot explain a capability state or a disable cause degrades honestly
//! instead of reading as a clean pass.
//!
//! [`m5_setting_definition_and_effective_setting_registries::M5SettingDefinitionEffectiveSettingRegistriesPacket`]
//! is the first implement lane over that frozen settings-governance matrix: it turns the
//! setting-definition grammar (how a stable setting is declared) and the effective-setting
//! grammar (how its live value is resolved from the winning scope) into registry
//! resolvers, so every claimed M5 setting resolves to one stable setting-definition
//! object — the declared type, the stable setting ID it preserves verbatim and never
//! recycles, the allowed scopes, the declared default, the migration aliases, the restart
//! posture, the sensitivity class, and the capability dependencies — and to one
//! effective-setting object — the resolved value or redacted summary, the shadow chain of
//! scopes that lost, the lock or constraint state, the validation status, the restart
//! state, the capability availability, and the last-applied revision — that the settings,
//! shell, diagnostics, admin, and support / export surfaces inspect without manual
//! reconstruction, so a stable setting ID is never recycled into a different meaning, the
//! shadow chain and restart posture stay visible before the resolution is trusted, and a
//! configuration surface that cannot explain the setting it declared or the scope that won
//! degrades honestly instead of reading as a clean pass.
//!
//! [`m5_setting_schema_migration_and_compatibility_window_registries::M5SettingSchemaMigrationCompatibilityWindowRegistriesPacket`]
//! is the schema-migration + downgrade implement lane over that frozen settings-governance matrix: it turns
//! the schema-migration-record grammar (how a version change declares the old key / alias, new key, transform,
//! lossy fidelity, compatibility window, and rollback note) and the compatibility-window grammar (how an
//! upgrade, import, restore, or downgrade flow discloses whether stored meaning is inside its window, deprecated
//! but supported, or outside the window) into registry resolvers, so every claimed M5 configuration migration
//! resolves to one schema-migration-record object — the fidelity label it classifies (exact / compatible /
//! lossy / manual-review), the old key / alias, the new key, the transform, the compatibility window, the
//! rollback note, the compare-before-apply reference, and the migration provenance reference — and to one
//! compatibility-window object — the window source, the supported version range, the deprecation review, the
//! validation status, the review state, the docs pointer, and the last review revision — that the upgrade,
//! import, restore, downgrade, and support / export flows inspect before apply without manual reconstruction, so
//! a migration never implies full fidelity when it is lossy or requires manual review, a schema change never
//! alters stored meaning without a checked-in migration record and compare surface, a compatibility window
//! always names its window source and downgrade guidance, and a configuration flow that cannot explain what a
//! migration changes degrades honestly instead of reading as a clean pass.
//!
//! [`m5_setting_sync_conflict_and_device_action_registries::M5SettingSyncConflictDeviceActionRegistriesPacket`]
//! is the sync / conflict engine implement lane over that frozen settings-governance matrix: it turns the
//! sync-conflict-packet grammar (how a sync scope bundle, session, and conflict packet declare which field
//! diverged, the local and remote revisions, the field-level keep-local / keep-synced options, the compare
//! surface, and the blocked-state reason a conflict class carries) and the device-action-record grammar (how a
//! device action ledger records the actor, timestamp, transport and policy state, capability dependency,
//! attribution, and last revision for a pause, resume, revoke, forget, or token-rotation action) into registry
//! resolvers, so every claimed M5 sync conflict resolves to one sync-conflict-packet object — the conflict class
//! it classifies (same-key divergent / policy-locked / missing-capability / machine-only / delete-versus-modify /
//! stale-remote), the field path, the local and remote revisions, the keep-local option, the keep-synced option,
//! the compare reference, and the blocked-state reason — and every claimed device action resolves to one
//! device-action-record object — the actor, the action timestamp, the transport state, the policy state, the
//! capability dependency, the attribution reference, and the last ledger revision — that the sync-session,
//! import-apply, outage-recovery, device-review, and support / export flows inspect at the field level before
//! apply without manual reconstruction, so sync never silently overwrites locked, machine-only, or stale-local
//! authoritative state, a conflict never collapses into last-writer-wins, a device action ledger always names its
//! attribution and stays reconstructable, and a sync flow that cannot explain a conflict or a device action
//! degrades honestly instead of reading as a clean pass.
//!
//! [`m5_setting_write_intent_and_policy_constraint_registries::M5SettingWriteIntentPolicyConstraintRegistriesPacket`]
//! is the write-pipeline implement lane over that frozen settings-governance matrix: it turns
//! the setting-write-intent grammar (how a configuration mutation declares the scope, artifact,
//! actor, reason, preview class, and recovery evidence it will land) and the policy / constraint
//! grammar (how a locked or denied write explains itself) into registry resolvers, so every
//! claimed M5 configuration mutation resolves to one write-intent object — the preview class it
//! classifies, the target scope and artifact it lands in and never silently widens, the intended
//! value, the actor, the change reason, the preview reference, and the checkpoint / rollback
//! recovery reference — and to one policy / constraint object — the lock source, the allowed
//! override classes, the expiry / review window, the validation status, the review state, the
//! docs pointer, and the last review revision — that the settings, shell, diagnostics, admin, and
//! support / export surfaces inspect without manual reconstruction, so a scoped write is never
//! rewritten into a broader scope or an unintended artifact, a high-risk write always materializes
//! preview / checkpoint / rollback evidence before it applies, a locked or denied write always
//! names its cause and fallback, and a configuration route that cannot explain where a mutation
//! lands or why a write is locked degrades honestly instead of reading as a clean pass.
//!
//! [`m5_settings_governance_shared_consumers_one_registry_across_surfaces::M5SettingsGovernanceSharedConsumersPacket`]
//! is the consumer-adoption capstone over that frozen settings-governance matrix: it binds each of the five
//! settings-governance families to the concrete settings-resolver, shell, sync-service, policy-service,
//! capability-service, diagnostics, docs / help, CLI / export, and support-export consumers that render it,
//! and proves, by fixtures rather than screenshots, that the same configuration profile presents the same
//! settings-governance-role, family, registry-reference, resolution-context, surface-context, and
//! evidence-continuity grammar wherever it appears, so a family is adopted by two or more consumers,
//! effective-setting state / write intent / migration posture / sync conflict class / device lineage /
//! capability lifecycle never drift between the GUI settings, CLI / headless inspect, Project Doctor,
//! support export, import / export, and policy-explainer surfaces, and a surface that recycles a retired
//! setting ID, rewrites a scoped write into a broader scope, silently overwrites locked or machine-only
//! state during sync, hides a lifecycle or experiment dependency behind unpublished markers, or hides a
//! kill-switch or policy-disable cause behind generic unavailable copy is rejected before the packet can be
//! published.
//!
//! [`m5_settings_governance_surface_certification::SettingsGovernanceProfileCertificationPacket`]
//! is the closing surface-certification capstone over that frozen settings-governance matrix: it certifies
//! that the shared settings-governance truth holds on every claimed M5 configuration-bearing profile (a live
//! trusted settings surface, a reviewable settings structure, a disclosed write-intent profile, an unverified
//! sync-conflict profile, and an unverified capability-lifecycle profile) across nine truth axes — visual,
//! keyboard, screen-reader, high-zoom-reflow, high-contrast, localization, CLI/export, degraded-state, and
//! settings-governance-component-truth behavior — and either passes (green), auto-narrows a not-current axis
//! to the weakest supported configuration ceiling (yellow), or blocks (red) when a degraded axis hides behind
//! a fresh trusted claim, a B143 hard invariant breaks, CLI/export parity drops, or a non-live profile claims
//! a trusted settings surface. Every row cites the one canonical settings-governance matrix proof bundle, so
//! release, docs, and support reference a single configuration-runtime certification source and no profile can
//! advertise full resolver, sync-conflict, or lifecycle continuity when that B143 evidence is stale or
//! missing.
//!
//! [`m5_repository_bootstrap_shared_consumers_one_registry_across_surfaces::M5RepositoryBootstrapSharedConsumersPacket`]
//! is the consumer-adoption capstone over that frozen matrix: it binds each of the
//! five repository-bootstrap families to the concrete acquisition-engine, shell,
//! workspace, git-service, trust-service, diagnostics, docs / help, CLI / export, and
//! support-export consumers that render it — the start-center, OS-open, CLI / headless,
//! deep-link, and import entry surfaces — and proves, by fixtures rather than
//! screenshots, that the same acquisition profile presents the same
//! repository-bootstrap-role, family, registry-reference, entry-context,
//! surface-context, and trust-stage-continuity grammar wherever it appears, so a family
//! is adopted by two or more consumers, entry verbs / trust stages / resumable-partial
//! behavior never drift between surfaces, and a surface that rewrites clone into open
//! over an existing checkout, runs repo-owned actions implicitly, loses signer or
//! mirror provenance, strands partial acquisition, or hides the bootstrap credential
//! posture is rejected before the packet can be published.
//!
//! [`m5_repository_bootstrap_accessibility_parity_and_narrowing_when_checkout_plan_trust_stage_mirror_signer_continuity_or_bootstrap_evidence_is_stale_or_partial::RepositoryBootstrapAccessibilityPacket`]
//! is the accessibility-localization-support-export parity and honest auto-narrowing
//! capstone over that frozen matrix: it certifies, per acquisition family, that the
//! source-locator, checkout-plan, credential-posture, staged-trust, post-open-queue,
//! partial-acquisition, and bootstrap-evidence truth stays keyboard-reachable,
//! screen-reader-announced, high-zoom-legible, high-contrast-safe, localization-safe,
//! and CLI/export-safe, and that the claim auto-narrows from trusted_acquisition_surface
//! / reviewable_acquisition_surface to a checkout-plan-disclosed / trust-stage-unverified
//! / bootstrap-evidence-unverified projection whenever the checkout-plan, staged-trust,
//! mirror/signer-continuity, or bootstrap-evidence proof is stale, partial, or
//! policy-blocked — so accessibility and CLI/export paths inspect the same acquisition
//! truth the GUI entry and recovery surfaces show, and no claimed entry profile can stay
//! green after that proof ages out.
//!
//! [`m5_repository_bootstrap_surface_certification::RepositoryBootstrapProfileCertificationPacket`]
//! is the closing surface-certification capstone over that frozen matrix: keyed on the
//! claimed project-entry profile (a live trusted acquisition surface, a reviewable
//! acquisition structure, a disclosed checkout-plan profile, an unverified trust-stage
//! profile, and an unverified bootstrap-evidence profile) rather than the acquisition
//! family, it certifies each profile across nine truth axes — visual, keyboard,
//! screen-reader, high-zoom-reflow, high-contrast, localization, CLI/export,
//! degraded-state, and repository-bootstrap-component-truth behavior — and either passes
//! (green), auto-narrows its acquisition claim to the weakest supported ceiling (yellow),
//! or blocks (red) when a degraded axis hides behind a fresh trusted claim, a B142 hard
//! invariant breaks, CLI/export parity drops, a non-live profile claims a trusted
//! acquisition surface, or the narrowing is inconsistent. Every row cites the one
//! canonical repository-bootstrap proof bundle, so release, docs, and support consume a
//! single repository-bootstrap certification source rather than hand-authored prose, and
//! no claimed entry profile can advertise full bootstrap continuity once its checkout-plan,
//! trust-stage, or partial-acquisition evidence goes stale.
//!
//! [`m5_source_locator_and_checkout_plan_registries::M5SourceLocatorCheckoutPlanRegistriesPacket`]
//! is the first implement lane over that frozen matrix: it turns the source-locator
//! grammar (open-local / open-archive) and the checkout-plan grammar (clone-remote)
//! into registry resolvers, so every claimed entry flow resolves to one stable
//! source-locator object — the source-locator kind, the literal target it preserves
//! verbatim, the resolved checkout root or archive container, the staged-trust
//! metadata, the disclosed bootstrap credential posture, the signer / mirror
//! provenance, and the distinct mirror / air-gap hint — and to one checkout-plan
//! object — the ref selection, full / partial / sparse mode, depth / filter,
//! submodule mode, LFS posture, destination path, and expected disk / network cost
//! band — that the shell, entry, diagnostics, admin, and support / export surfaces
//! inspect without manual reconstruction, so open and clone stay distinct verbs, the
//! credential posture stays visible before any network or mirror fetch, repo-owned
//! actions never run implicitly during acquisition, and an entry flow that cannot
//! explain the literal target and checkout posture it chose degrades honestly instead
//! of reading as a clean pass.
//!
//! [`m5_bootstrap_credential_posture_and_fetch_route_registries::M5CredentialPostureFetchRouteRegistriesPacket`]
//! is the credential-boundary + mirror/trust-route implement lane over that frozen
//! matrix: it turns the credential-posture grammar (how a bootstrap authenticates and
//! which trust roots or mirrors it depends on) and the fetch-route grammar (public
//! upstream fetch, approved mirror fetch, air-gap bundle import, and managed snapshot
//! resume) into registry resolvers, so every claimed acquisition path resolves to one
//! stable credential-posture object — the auth-source kind and canonical auth mode, the
//! auth-source reference, the proxy / mirror route, the host-key or TLS-pin state, the
//! delegated-token policy, the handle-only secret reference kept out of the export
//! boundary, and the mirror / signer provenance — and to one fetch-route object — the
//! route endpoint class, the signer- and digest-continuity references, the
//! mirror-provenance reference, the recovery language, and the trust-proof reference —
//! that the acquisition, git, trust, diagnostics, CLI, and support / export surfaces
//! inspect without manual reconstruction, so a credential posture never embeds a raw
//! secret or token in a portable manifest, host trust state is disclosed rather than
//! hidden behind generic connected-state copy, public / mirrored / air-gapped / resumed
//! fetch routes stay distinct, signer and mirror provenance stay continuous, and an
//! acquisition path that cannot explain how it authenticated or which trust route it
//! took degrades honestly instead of reading as a clean pass.
//!
//! [`m5_staged_trust_and_post_open_queue_registries::M5StagedTrustPostOpenQueueRegistriesPacket`]
//! is the staged-trust + post-open-queue implement lane over that frozen matrix:
//! it turns the staged-trust grammar (how Aureline browses the tree, manifests, and
//! docs and computes safe metadata before any repo-owned hook, task, extension
//! recommendation, package restore, submodule init, LFS hydrate, or generator install
//! can run) and the post-open bootstrap-queue grammar (typed, attributable work objects
//! that run repo-owned code, hydrate network-backed content, mutate the reviewed
//! checkout, or merely recommend) into registry resolvers, so every claimed acquisition
//! path resolves to one stable staged-trust object — the trust-stage kind and canonical
//! trust mode, the browse-scope reference, the computed-metadata reference, the deferred
//! repo-owned action set, the trust-prompt policy, the explicit-approval reference, and
//! the staged-trust provenance — and to one post-open-queue object — the queue-item kind,
//! the execution site, the trust consequence, the network consequence, the approval
//! requirement, and the attribution reference — that the acquisition, git, trust,
//! diagnostics, CLI, and support / export surfaces inspect without manual reconstruction,
//! so repository open stays useful before any repo-owned action runs, a protected
//! post-open queue item never auto-executes implicitly during acquisition, trust is never
//! widened before browse-safe metadata is computed, every queue row identifies exactly
//! what would run, where, and its trust or network consequence, and an acquisition path
//! that cannot explain its staged trust or post-open queue degrades honestly instead of
//! reading as a clean pass.
//!
//! [`m5_acquisition_evidence_and_partial_recovery_registries::M5AcquisitionEvidencePartialRecoveryRegistriesPacket`]
//! is the evidence-packet + resumable-acquisition implement lane over that frozen matrix:
//! it turns the acquisition-evidence grammar (how Aureline records the clone / fetch
//! transcript, the warnings and retries, the resulting root identity, the
//! omitted-or-unfetched state, and the current bootstrap checkpoint of an acquisition
//! path) and the partial-recovery grammar (typed recovery actions that resume an
//! interrupted acquisition, discard partial state, open the partial root read-only, or
//! merely report status) into registry resolvers, so every claimed acquisition path
//! resolves to one stable acquisition-evidence packet — the evidence kind and canonical
//! evidence mode, the transcript reference, the warnings-and-retries reference, the
//! resulting-root-identity reference, the omitted-or-unfetched reference, the
//! bootstrap-checkpoint reference, and the evidence provenance — and to one
//! partial-recovery object — the recovery-action kind, the recovery site, the state
//! consequence, the lineage consequence, the explicit-action requirement, and the
//! attribution reference — that the acquisition, git, trust, diagnostics, CLI, and
//! support / export surfaces inspect without manual reconstruction, so a partial or
//! interrupted acquisition stays visible and recoverable instead of reading as missing or
//! unsupported data, a recovery action never discards partial state or transcript lineage
//! without an explicit discard or cleanup action, partial content is never presented as a
//! healthy full checkout, every recovery row identifies exactly what the action would do,
//! where, and its state or lineage effect, and an acquisition path that cannot explain its
//! evidence or its recovery choices degrades honestly instead of reading as a clean pass.
//!
//! [`m5_constrained_file_state_matrix::M5ConstrainedFileStateMatrixPacket`] opens B150 by freezing
//! Aureline's constrained-current-object model — the read-only, generated, policy-locked, managed,
//! projection, and captured-snapshot objects a write-capable consumer must never treat as an ordinary
//! directly-writable file — into one export-safe matrix. It binds every governed class to one shared
//! constrained-file-state role taxonomy (state_badge_classification, blocked_write_reason,
//! canonical_source_relation, exact_write_target, allowed_blocked_action_set, safe_next_step_guidance,
//! export_retain_disclosure) and required visible state (state badge, reason, canonical source or live
//! target, exact write target, allowed actions, blocked actions, and export / retain notes), makes a
//! write-constrained object mechanically distinct from an ordinary directly-writable one, routes each class
//! to its canonical constrained-file-state / canonical-source-relation / write-target-review domain schema,
//! and binds back to the already-landed stable-proof-index and migration-task-row packets, so a constrained
//! object never looks directly writable by omission, no generated / managed / projection / archived object
//! silently falls back to a lossy direct write, no AI / automation / import / repair flow bypasses the
//! constrained-state rules, and the canonical source, exact write target, preserved-versus-lost sync, and
//! recovery / regenerate path stay explicit across tabs, breadcrumbs, the status bar, the command palette,
//! editor banners, diff / review headers, write-review sheets, AI / automation mutation paths, and support /
//! export packets.
//!
//! [`m5_constrained_state_descriptor_and_change_diff_registries::M5ConstrainedStateDescriptorChangeDiffRegistriesPacket`]
//! is the first B150 implement lane over that frozen constrained-file-state matrix. It makes every
//! constrained-current-object class — a read-only path case, a generated artifact, a policy-locked object, a
//! managed / mirrored object, a projection, and a captured snapshot — emit one machine-readable
//! constrained-state descriptor with a stable ID, turning the constrained-state-descriptor grammar (per object:
//! its state-class, the reason it is constrained, its canonical-source relation, its exact write target or its
//! absence, its allowed safe actions, and its retained-versus-lost sync notes) and the change-diff grammar (a
//! state-class change, a canonical-source change, or a write-target change) into registry resolvers that emit
//! export-safe, honest projections. Each registry row binds a tab-chrome, status-bar, command-palette,
//! diff / review-header, write-review-sheet, AI / automation-path, or support / export consumer to resolved
//! descriptor and change-diff entries across the canonical, accessible, and audit resolution forms, so
//! consumers can distinguish inspect-only from duplicate / detach / overlay / regenerate / request-approval
//! paths without hand-authored special-case prose, and a changed state-class, canonical source, or write target
//! produces a visible descriptor diff instead of a silent in-place mutation. Registry-A reuses the matrix
//! constrained-file-state domain schema and Registry-B mints the constrained-state-change-diff domain schema
//! fresh.
//!
//! [`m5_file_state_badge_group_and_reason_strip_consumers::M5FileStateBadgeGroupConsumersPacket`] is the B150
//! badge-group / reason-strip consumer lane over that frozen constrained-file-state matrix. It ships one
//! reusable file-state badge group and reason strip — a controlled state-class label (`Read-only`, `Generated`,
//! `Policy locked`, `Managed`, `Projection`, `Captured snapshot`), a plain-language cause, the canonical source,
//! the write disposition, and the nearest safe next step — and wires the tab-chrome, breadcrumb-trail,
//! status-bar, command-palette, editor-banner, diff / review-header, write-review-sheet, AI / automation-path,
//! and support / export consumers to the same constrained-object profile, so one object cannot look writable in
//! one surface and blocked in another. A multi-state object (`Generated` plus `Policy locked`, `Managed` plus
//! `Captured snapshot`) keeps every co-applicable state visible instead of letting one badge hide another, the
//! write-capable safe-next-step affordance appears only where the full badge group is rendered, and every
//! binding names the keyboard and screen-reader routes through which the state class, reason, and next safe
//! action are discoverable without pointer-only chrome.
//!
//! [`m5_canonical_source_relation_and_write_target_review_registries::M5CanonicalSourceRelationWriteTargetReviewRegistriesPacket`]
//! is the B150 canonical-source-relation and write-target-review implement lane over that frozen
//! constrained-file-state matrix. Registry-A links each constrained current object to the authoritative source
//! an edit belongs to — an editable source file, a generator input, an owning rule, a managed authoritative
//! source, or a live target when known — as one machine-readable canonical-source relation row with a stable
//! target identity, so an alias-versus-canonical path case, a generated source sibling, a managed / mirrored
//! object, and an archived-snapshot handoff each expose where authoritative edits land. Registry-B carries the
//! exact write-target explainer for the chosen action — the current pane, the canonical source, a duplicate
//! copy, an overlay patch, or a no-write path — so a save review, compare sheet, review header, or export packet
//! can say what bytes will or will not change without leaving the user to infer it, and a changed state-class,
//! canonical source, or write target surfaces as a visible, typed diff. Registry-A reuses the matrix
//! canonical-source-relation domain schema and Registry-B reuses the matrix write-target-review domain schema;
//! path and target identity are preserved across the tab-chrome, status-bar, command-palette, diff / review-header,
//! AI / automation-path, and support / export consumers without leaking raw secrets.
//!
//! [`m5_write_review_sheet_fallback_paths::M5WriteReviewSheetFallbackPathsPacket`] is the B150
//! write-review-sheet lane over that frozen constrained-file-state matrix. It turns a blocked write on a
//! constrained current object into an explicit reviewed transition — duplicate to an editable copy, detach from
//! a managed source, create an overlay patch, request approval, or regenerate with preview — instead of a silent
//! best-effort fallback: a reviewed sheet is shown before commit that names the exact write target, the side
//! effects, the preserved-versus-lost sync or regenerate path, any required approvals, the checkpoint / undo
//! class, and an export-safe explanation, and one sheet model is reused across the direct-save, code-action,
//! AI-apply, importer, repair, and batch-edit flows so an AI apply and a direct save that hit the same
//! constrained object get the same reviewed transition rather than one of them slipping a hidden bypass. Every
//! one of the five fallback paths is reviewable before commit with explicit retained-versus-lost behaviour, a
//! recovery / undo class is visible before commit on every path, and no constrained write silently mutates the
//! current object through a lossy fallback.
//!
//! [`m5_cross_actor_constrained_write_enforcement::M5CrossActorConstrainedWriteEnforcementPacket`] is the B150
//! actor-parity mutation-gate lane over that frozen constrained-file-state matrix. It ships one shared
//! constrained-write gate and safe-next-step resolver that every mutation-capable actor — a direct edit / save, an
//! AI apply, an automation recipe, an importer, a repair, and a code action — is routed through, so each actor
//! inherits the same state-class block and the same safe-next-step guidance instead of an actor-specific
//! best-effort write. The blocked-write reason is a pure function of the constrained-object class, so an AI apply,
//! a repair, an importer, and a direct save that all land on the same object hit the same structured reason and
//! the same safe next step; a mutation-capable actor can never silently write a generated, managed, projection, or
//! captured-snapshot object just because it bypasses direct typing, because there is no direct-write action to
//! represent and the only write-adjacent action opens the reviewed transition; the gate fails closed when the
//! actor context drifts or a flow cannot explain the exact write target truthfully; and every support / export
//! trace preserves the actor, the blocked reason, and the chosen fallback path.
//!
//! [`m5_constrained_state_drill_corpus::M5ConstrainedStateDrillCorpusPacket`] is the B150 fixture-corpus and
//! regression-drill lane over that same frozen constrained-file-state matrix. It seeds one reusable corpus that
//! proves the constrained-object loops stay honest under failure: every binding seeds one constrained-object fixture
//! — a read-only alias path, a generated / derived artifact, a policy-locked managed mirror, a projection / virtual
//! view, a managed source, or a captured workspace snapshot — exercised by one of nine drills covering the
//! problematic transitions (symlink / alias save, generated-artifact drift, policy-locked managed mirrors,
//! projection export, captured snapshots inside the current workspace, unsupported round trips, and five mixed-state
//! combinations) that attempt a direct write, watch it be denied, and route to the exact reviewed fallback path
//! keyed to the object class. The corpus covers every state class as a primary plus at least five mixed-state
//! combinations, so a drill can prove a lossy direct write, a masked second state, a fallback that does not match its
//! reason, or a grammar that drifts across surfaces is mechanically rejected, and the first seeded support / export
//! packet can replay a constrained write denial and its chosen fallback path from fixtures.
//!
//! [`m5_constrained_state_export_and_review_evidence_packets::M5ConstrainedStateEvidencePacket`] is the B150 support /
//! export and review-evidence packet lane over that same frozen constrained-file-state matrix. It keeps the
//! constrained-object loop explainable once it leaves the live UI: a support bundle, a review / export packet, a piece
//! of local-history / restore evidence, or a docs / help example each preserves the constrained-state class, the
//! canonical source-of-truth relation, the exact write-target decision, and the chosen reviewed fallback path —
//! including whether the operator duplicated, detached, overlaid, requested approval, regenerated, or cancelled, and
//! what sync / regenerate path was preserved versus lost. At least one support bundle and one review / export packet
//! preserve those decisions in both human-readable and machine-readable form; exported packets stay intelligible
//! without the live UI and never flatten a generated, managed, projection, policy-locked, or captured-snapshot object
//! into generic read-only language; and redacted packets keep the omission reason while still preserving the state
//! class and fallback decision.
//!
//! [`m5_constrained_object_surface_certification::ConstrainedObjectProfileCertificationPacket`] is the closing
//! B150 surface-certification capstone over that same frozen constrained-file-state matrix. After the
//! M05-1257..1263 implement lanes resolve the constrained-state descriptors, badge-group / reason-strip
//! consumers, canonical-source relations, write-target reviews, write-review-sheet fallback paths, cross-actor
//! mutation gate, drill corpus, and support / export evidence packets, this capstone certifies that the shared
//! constrained-object truth actually holds on every claimed M5 editor, review, save, AI, repair, and export
//! consumer. It publishes one current certification row per claimed consumer profile — a fully-classified
//! constrained-object lane, a reviewable constrained-state record structure, a disclosed
//! generated-divergence-partial profile, an unverified canonical-source profile, an unverified
//! write-target-review profile, and an unverified actor-parity profile — scored across nine truth axes (visual,
//! keyboard, screen-reader, high-zoom-reflow, high-contrast, localization, CLI/export, degraded-state, and
//! constrained-object-truth behavior) and covering all six frozen object classes. A degraded axis must produce a
//! visible claim narrowing, CLI/export parity must always certify, only a fully-classified constrained-object
//! lane may certify a certified constrained-object record, and every B150 hard invariant must hold (no
//! constrained-state class hides another, no generated / managed / projection / archived object silently falls
//! back to a lossy direct write, no AI / automation / import / repair flow gets a hidden bypass, and the
//! canonical source, exact write target, preserved-versus-lost sync, and recovery / regenerate path stay
//! explicit), so a claim narrows automatically the moment a consumer regresses on shared vocabulary,
//! canonical-source disclosure, write-target review, or actor-parity blocking. Every row cites the one canonical
//! constrained-file-state matrix proof bundle so docs / help, support, and release / public-proof artifacts
//! ingest the same certification result instead of restating it by hand.
//!
//! [`m5_ai_review_assist_matrix::M5AiReviewAssistMatrixPacket`] opens the B151 AI-review-assist batch by
//! freezing the reusable AI review finding row, review scope selector, publish-to-review sheet, and resolution
//! memory row as governed product truth. Each [`m5_ai_review_assist_matrix::M5AiReviewAssistRow`] names its
//! finding class / severity, analyzed diff scope, publish mode / provider destination, local-draft-versus-
//! provider-committed state (see [`m5_ai_review_assist_matrix::M5AiReviewAssistPublishState`]), lifecycle state
//! (open, dismissed, published, outdated, suppressed, rerun recommended), and publish / export fallback, and
//! binds each object class to its canonical per-domain finding / scope-selector / publish-sheet / resolution-
//! memory schema. Hard invariants forbid AI review results publishing or merging implicitly, hiding whether
//! output stays local or becomes a provider comment / suggested patch / check annotation, keeping stale
//! findings looking current after diff or instruction drift, and losing local drafts or evidence when provider
//! write scope is missing or a publish fails. Review detail, the AI review panel, pending-review trays, provider
//! publish review, and support / export packets ingest this matrix instead of minting per-surface AI-review
//! chrome.
//!
//! [`m5_ai_review_shared_consumers_one_vocabulary_across_surfaces::M5AiReviewSharedConsumersPacket`] closes the
//! B151 batch's consumer-adoption arc by binding those four governed AI-review-assist objects to the shared
//! review-detail, AI-review-panel, finding-row, review-scope-selector, publish-to-review-sheet,
//! pending-review-tray, provider-publish-review, resolution-memory-ledger, and support / export consumers that
//! render them, proving — by fixtures — that the same seeded finding presents one identical AI-review-role,
//! object, registry-reference, publish-state, surface-context, and finding-lifecycle vocabulary wherever it
//! appears. Narrowing across desktop / compact / remote / exported representations is disclosed rather than
//! reworded, support / export bindings map every copy / export / open-in-provider payload back to one
//! canonical contract, and the same hard invariants (no implicit publish or merge, no hidden output
//! destination, no stale finding shown as current, no lost local drafts, no finding without scope /
//! destination / lifecycle) are re-asserted per binding.
//!
//! [`m5_ai_review_accessibility_parity_and_narrowing_when_provider_freshness_diff_drift_publish_target_or_finding_lifecycle_state_is_stale::AiReviewAccessibilityPacket`]
//! closes the B151 batch's accessibility-parity arc by certifying — per governed AI-review-assist object —
//! that keyboard-only, screen-reader, high-zoom / high-contrast, CLI/headless, and export flows can inspect,
//! rerun, dismiss, publish, export, and reopen the finding row, review scope selector, publish-to-review
//! sheet, and resolution memory row without losing analyzed scope, destination class, or finding lifecycle
//! truth. When provider freshness is stale, diff drift invalidates prior findings, a publish target is
//! unavailable, or a finding's lifecycle state falls outside live publish-safe conditions, each object's
//! claim auto-narrows to a provider-freshness-unverified / diff-scope-unverified / publish-target-unverified
//! / finding-lifecycle-unverified projection that discloses the narrowing with a precise binding dimension
//! and frozen matrix trigger and preserves the canonical object identity, so a stale, drifted, or
//! publish-unsafe object can never keep a trusted, publish-safe claim, AI review never auto-approves or
//! auto-merges, and no local draft is lost or shown as a provider-committed publish. CLI / support / release
//! exports carry the same scope, destination, and lifecycle labels visible in-product without leaking a raw
//! payload.
//!
//! [`m5_ai_review_assist_surface_certification::AiReviewProfileCertificationPacket`] closes the B151 batch by
//! *certifying* — per claimed consumer profile rather than per object class — that the shared AI-review truth
//! holds on every claimed M5 review, AI, provider, pending-review, and support / export surface. Each
//! [`m5_ai_review_assist_surface_certification::AiReviewProfileCertificationRow`] scores one profile (a
//! fully-classified AI-review lane, a reviewable AI-review record structure, or a disclosed
//! provider-freshness-partial / unverified-diff-scope / unverified-publish-target / unverified-finding-lifecycle
//! profile) across nine truth axes — visual, keyboard, screen-reader, high-zoom-reflow, high-contrast,
//! localization, always-on CLI/export, degraded-state, and ai-review-truth behavior — and either passes
//! (green), auto-narrows its AI-review claim to the weakest supported ceiling (yellow), or blocks (red) when a
//! degraded axis hides behind a fresh certified claim, a B151 hard invariant breaks, CLI/export parity drops,
//! or a non-lane profile claims a certified AI-review record. A degraded axis must produce a visible claim
//! narrowing, only a fully-classified AI-review lane may certify a certified AI-review record, and every row
//! cites the one canonical AI-review-assist matrix proof bundle so support and automation reconstruct the same
//! finding class, analyzed scope, publish destination, local-versus-provider state, and lifecycle state the
//! operator saw.

#![doc(html_root_url = "https://docs.rs/aureline-ui/0.0.0")]

pub mod components;
pub mod density;
pub mod m5_accessibility_and_continuity;
pub mod m5_acquisition_evidence_and_partial_recovery_registries;
pub mod m5_ai_review_accessibility_parity_and_narrowing_when_provider_freshness_diff_drift_publish_target_or_finding_lifecycle_state_is_stale;
pub mod m5_ai_review_assist_matrix;
pub mod m5_ai_review_assist_surface_certification;
pub mod m5_ai_review_finding_and_scope_source_registries;
pub mod m5_ai_review_publish_continuity_and_reconcile_registries;
pub mod m5_ai_review_publish_sheet_and_scope_decision_registries;
pub mod m5_ai_review_resolution_memory_and_finding_lifecycle_registries;
pub mod m5_ai_review_scope_selector_and_rerun_state_registries;
pub mod m5_ai_review_shared_consumers_one_vocabulary_across_surfaces;
pub mod m5_annotation_rows;
pub mod m5_archived_object_expiry_removal_state_and_metadata_fallback;
pub mod m5_archived_snapshot_viewer_and_analysis_only_banner_consumers;
pub mod m5_badge_chip_pill_and_popover_expansion_and_anchored_focus_return;
pub mod m5_banner_inline_notice_and_empty_state_scoped_cause_and_next_action;
pub mod m5_bootstrap_credential_posture_and_fetch_route_registries;
pub mod m5_build_lane_descriptor_and_reproducibility_proof_registries;
pub mod m5_build_lane_trust_matrix;
pub mod m5_build_lane_trust_shared_consumers_one_registry_across_surfaces;
pub mod m5_build_lane_trust_surface_certification;
pub mod m5_button_and_icon_button_state_and_command_attribution;
pub mod m5_canonical_source_relation_and_write_target_review_registries;
pub mod m5_channel_isolation_precedence_review_and_rollback_targets;
pub mod m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries;
pub mod m5_cohort_descriptor_and_evidence_packet_registries;
pub mod m5_color_system_and_semantic_theme_token_registries;
pub mod m5_combobox_and_checkbox_radio_switch_value_source_and_toggle_semantics;
pub mod m5_constrained_file_state_matrix;
pub mod m5_constrained_object_surface_certification;
pub mod m5_constrained_state_descriptor_and_change_diff_registries;
pub mod m5_constrained_state_drill_corpus;
pub mod m5_constrained_state_export_and_review_evidence_packets;
pub mod m5_core_action_input_accessibility_parity_and_narrowing_when_control_truth_is_stale;
pub mod m5_core_action_input_component_matrix;
pub mod m5_core_action_input_component_surface_certification;
pub mod m5_core_action_input_shared_consumers_one_vocabulary_across_surfaces;
pub mod m5_cross_actor_constrained_write_enforcement;
pub mod m5_decision_feedback_accessibility_parity_and_narrowing_when_decision_feedback_truth_is_stale;
pub mod m5_decision_feedback_component_matrix;
pub mod m5_decision_feedback_component_surface_certification;
pub mod m5_decision_feedback_shared_consumers_one_vocabulary_across_surfaces;
pub mod m5_density_mode_registries;
pub mod m5_dependency_rows;
pub mod m5_dialog_sheet_and_consequence_block_rationale_scope_and_rollback_continuity;
pub mod m5_display_topology_recovery_and_role_continuity_registries;
pub mod m5_draft_state_and_autosave;
pub mod m5_exact_build_symbolication_and_mirror_offline_parity_registries;
pub mod m5_field_control_rows;
pub mod m5_file_path_reveal_and_native_window_menu_registries;
pub mod m5_file_state_badge_group_and_reason_strip_consumers;
pub mod m5_form_family_certification;
pub mod m5_form_validation_and_blocked_submit;
pub mod m5_freeze_exception_and_go_no_go_registries;
pub mod m5_historical_evidence_drill_corpus;
pub mod m5_historical_evidence_surface_certification;
pub mod m5_historical_reference_matrix;
pub mod m5_historical_snapshot_descriptor_and_change_diff_registries;
pub mod m5_historical_versus_live_compare_flow;
pub mod m5_iconography_and_illustration_registries;
pub mod m5_imported_offline_evidence_lineage_propagation;
pub mod m5_input_method_and_credential_store_wording_registries;
pub mod m5_install_topology_accessibility_parity_and_narrowing_when_install_topology_state_root_repair_verify_or_rollout_evidence_is_stale;
pub mod m5_install_topology_and_state_root_registries;
pub mod m5_install_topology_matrix;
pub mod m5_install_topology_shared_consumers_one_registry_across_surfaces;
pub mod m5_install_topology_surface_certification;
pub mod m5_last_supported_snapshot_and_archive_export_gate_registries;
pub mod m5_launch_control_matrix;
pub mod m5_launch_control_surface_certification;
pub mod m5_layer_order_and_portal_registries;
pub mod m5_live_target_handoff_packet_and_route_validation;
pub mod m5_managed_deployment_operations_and_policy_bootstrap_injection;
pub mod m5_manifest_diff_cards;
pub mod m5_monitor_geometry_remap_and_restore_bounds;
pub mod m5_motion_layer_iconography_accessibility_parity_and_narrowing_when_motion_layer_or_icon_truth_is_stale;
pub mod m5_motion_layer_iconography_matrix;
pub mod m5_motion_layer_iconography_shared_consumers_one_grammar_across_surfaces;
pub mod m5_motion_layer_iconography_surface_certification;
pub mod m5_motion_token_and_reduced_motion_registries;
pub mod m5_no_rerun_session_recovery_and_authority_replay_fence_registries;
pub mod m5_opacity_scrim_and_overlay_depth_registries;
pub mod m5_orr_review_and_rehearsal_drill_registries;
pub mod m5_ownership_signal_and_conflict_registries;
pub mod m5_parameter_source_and_precedence;
pub mod m5_pipeline_dependency_finding_components;
pub mod m5_pipeline_run_rows;
pub mod m5_platform_fit_accessibility_parity_and_narrowing_when_platform_convention_native_affordance_or_input_method_truth_is_stale;
pub mod m5_platform_fit_matrix;
pub mod m5_platform_fit_shared_consumers_one_convention_across_surfaces;
pub mod m5_platform_fit_surface_certification;
pub mod m5_portable_mode_state_containment_and_diagnostics;
pub mod m5_regression_asset_and_incident_close_registries;
pub mod m5_remote_cache_integrity_and_cache_bypass_drill_registries;
pub mod m5_repository_bootstrap_accessibility_parity_and_narrowing_when_checkout_plan_trust_stage_mirror_signer_continuity_or_bootstrap_evidence_is_stale_or_partial;
pub mod m5_repository_bootstrap_matrix;
pub mod m5_repository_bootstrap_shared_consumers_one_registry_across_surfaces;
pub mod m5_repository_bootstrap_surface_certification;
pub mod m5_required_evidence_check_and_local_ci_parity_registries;
pub mod m5_responsive_geometry_and_collapse_priority_registries;
pub mod m5_retired_state_matrix;
pub mod m5_retired_state_surface_certification;
pub mod m5_retirement_closure_ledger_and_propagation_blocker_gate_registries;
pub mod m5_retirement_countdown_and_safety_gate_registries;
pub mod m5_retirement_impact_report_and_blocker_gate_registries;
pub mod m5_retirement_manifest_and_change_diff_registries;
pub mod m5_retirement_review_packet_and_closure_gate_registries;
pub mod m5_retirement_tombstone_and_claim_block_gate_registries;
pub mod m5_review_pack_evaluator_matrix;
pub mod m5_review_pack_record_and_result_registries;
pub mod m5_ring_progression_and_rollback_stop_registries;
pub mod m5_security_finding_cards;
pub mod m5_setting_capability_lifecycle_and_kill_switch_registries;
pub mod m5_setting_definition_and_effective_setting_registries;
pub mod m5_setting_schema_migration_and_compatibility_window_registries;
pub mod m5_setting_sync_conflict_and_device_action_registries;
pub mod m5_setting_write_intent_and_policy_constraint_registries;
pub mod m5_settings_governance_matrix;
pub mod m5_settings_governance_shared_consumers_one_registry_across_surfaces;
pub mod m5_settings_governance_surface_certification;
pub mod m5_shell_metric_and_minimum_size_registries;
pub mod m5_shell_metric_density_accessibility_parity_and_narrowing_when_shell_metric_density_or_adaptive_geometry_truth_is_stale;
pub mod m5_shell_metric_density_matrix;
pub mod m5_shell_metric_density_shared_consumers_one_geometry_across_surfaces;
pub mod m5_shell_metric_density_surface_certification;
pub mod m5_shortcut_notation_and_command_label_registries;
pub mod m5_skeleton_first_restore_and_session_hydration_registries;
pub mod m5_source_locator_and_checkout_plan_registries;
pub mod m5_spacing_sizing_radii_elevation_and_hit_target_registries;
pub mod m5_split_button_and_segmented_control_safe_default_and_selected_mode;
pub mod m5_stable_line_bundle_refresh_audit_and_shipping_line_drift_report_registries;
pub mod m5_stable_line_correction_report_and_train_comparison_registries;
pub mod m5_stable_line_defect_ledger_and_backport_decision_timer_registries;
pub mod m5_stable_line_deferral_backlog_and_correction_conversion_registries;
pub mod m5_stable_line_lts_readiness_decision_and_line_creation_gate_registries;
pub mod m5_stable_line_protection_matrix;
pub mod m5_stable_line_protection_plan_and_correction_queue_registries;
pub mod m5_stable_line_refresh_policy_and_claim_downgrade_registries;
pub mod m5_stable_line_surface_certification;
pub mod m5_staged_review_sheets;
pub mod m5_staged_trust_and_post_open_queue_registries;
pub mod m5_structured_input_and_staged_review;
pub mod m5_supported_line_correction_train_archive_and_closure_gate_registries;
pub mod m5_supported_line_migration_scoreboard_and_scoreboard_delta_registries;
pub mod m5_supported_line_orr_history_and_follow_up_closure_registries;
pub mod m5_supported_line_public_proof_ledger_and_claim_history_diff_registries;
pub mod m5_supported_line_retention_policy_and_stale_escalation_registries;
pub mod m5_supported_line_surface_certification;
pub mod m5_supported_line_transparency_matrix;
pub mod m5_supported_line_transparency_report_and_snapshot_diff_registries;
pub mod m5_supported_line_truth_feed_and_audience_packet_registries;
pub mod m5_syntax_diff_and_chart_token_registries;
pub mod m5_system_appearance_live_apply_and_source_provenance_registries;
pub mod m5_text_field_and_search_field_labels_validation_and_privacy;
pub mod m5_toast_and_loading_state_acknowledgement_and_loading_fidelity;
pub mod m5_typography_scale_font_stack_and_overflow_registries;
pub mod m5_verified_input_manifest_and_sidecar_completeness_registries;
pub mod m5_visual_foundation_matrix;
pub mod m5_visual_foundations_accessibility_parity_and_narrowing_when_visual_foundation_truth_is_stale;
pub mod m5_visual_foundations_shared_consumers_one_vocabulary_across_surfaces;
pub mod m5_visual_foundations_surface_certification;
pub mod m5_widening_decision_and_ring_history_registries;
pub mod m5_window_restore_accessibility_parity_and_narrowing_when_shared_authority_restore_fidelity_display_remap_or_no_rerun_session_truth_is_stale;
pub mod m5_window_restore_matrix;
pub mod m5_window_restore_shared_consumers_one_registry_across_surfaces;
pub mod m5_window_restore_surface_certification;
pub mod m5_workspace_authority_and_window_topology_registries;
pub mod m5_write_review_sheet_fallback_paths;
pub mod motion;
pub mod themes;
pub mod tokens;
