//! Signed template-registry truth records.
//!
//! This crate owns the typed, export-safe packet that the template gallery,
//! scaffold preflight, run and recovery surfaces, diagnostics, and support
//! exports consume to learn whether a template may be offered — and on what
//! terms. Each row binds a template revision to its provenance and mirror
//! lineage, its signing trust source and signature class, its certification and
//! support class, its declared freshness, and its template-health state, so the
//! signed registry, mirror staleness, and template-health rows stay inspectable
//! from gallery through generation and recovery.
//!
//! It also owns the generation diff-review and recovery packet, which carries
//! managed-zone (authored/generated/runtime-only) truth, generation-diff review
//! state, and rollback or delete-generated recovery actions for a generated
//! project tree, so the diff-review, run, recovery, diagnostics, and support
//! surfaces never overwrite silently or delete authored work.
//!
//! It also owns the framework-pack header, freshness-chip, and capability or
//! downgrade banner packet, which binds each framework pack to its header
//! provenance, its pinned pack version and freshness chip, its capability banner,
//! its support class, and its downgrade banner, so the gallery, pack header, run,
//! diff-review, diagnostics, and support surfaces never present heuristic or
//! bridge behavior as exact first-party truth without current support-class and
//! downgrade cues.
//!
//! It also owns the route-explorer, component-tree, and app-topology view packet,
//! which binds each structural node to its authored/generated/runtime-only origin,
//! the generator version that produced any generated node, the view-scan freshness,
//! how the node truth was derived, its support class, and its downgrade banner, so
//! the route explorer, component tree, app-topology view, diff-review, run,
//! diagnostics, and support surfaces never present heuristic, bridged, or
//! runtime-observed structure as exact authored or generated source truth.
//!
//! It also owns the convention-diagnostic packet, which binds each framework-pack
//! convention diagnostic to its confidence label, analysis freshness, whether and
//! how it may be suppressed, the proving file or manifest that grounds it, its
//! support class, and its downgrade banner, so the editor diagnostics, problems
//! panel, diff-review, run, diagnostics, and support surfaces never present a
//! heuristic, bridged, or ungrounded convention as exact first-party truth, and a
//! suppressed or blocked diagnostic is labeled rather than silently hidden.
//!
//! It also owns the framework generator and codemod run packet, which binds each
//! generator or codemod run to its pinned generator version, whether a preview was
//! produced, whether the change diff was reviewed, whether the run can be rolled
//! back, whether a warm execution context was reused, its run-record freshness, its
//! support class, and its downgrade banner, so the generator gallery, preview pane,
//! diff-review, run, rollback/recovery, diagnostics, and support surfaces never let
//! starter convenience outrun provenance, preview, or rollback, and never present
//! heuristic or bridge behavior as exact first-party truth.
//!
//! It also owns the certified-archetype health-check bundle packet, which binds each
//! health-check bundle run to its archetype certification class, its pinned
//! health-check bundle version, its overall health state, its worst stack-diagnostic
//! severity, whether and how fix-forward guidance is available, its scan freshness,
//! its support class, and its downgrade banner, so the archetype gallery,
//! health-check panel, stack-diagnostics, fix-forward guidance, run, diagnostics, and
//! support surfaces never present an uncertified, heuristic, or bridged health check
//! as exact first-party certified truth without current certification, support-class,
//! and downgrade cues.
//!
//! It also owns the richer framework-pack lane catalog packet, which binds each
//! framework pack — across the Jupyter-adjacency, Terraform, Kubernetes, FastAPI,
//! Nest, Rails, Laravel, and Flutter lanes — to its lane domain, its header
//! provenance, its pinned pack and generator versions, its freshness chip, its
//! capability and support class, its authored/generated/runtime-only origin truth,
//! its archetype health state, and its downgrade banner, so the gallery, pack
//! header, run, diff-review, diagnostics, and support surfaces never let a deeper
//! long-tail of lanes present heuristic, bridged, or runtime-observed structure as
//! exact first-party truth without current support-class, origin-truth, health, and
//! downgrade cues.
//!
//! It also owns the frozen M5 scaffold-component matrix, which locks the reusable
//! scaffold / project-entry components — the scaffold template card, the starter parameter
//! row, the scaffold preflight card, the template health row, the generated-project diff
//! card, and the scaffold handoff banner — into one export-safe packet, binding each family
//! to its starter source class, support class, parameter source layer, immediate-versus-
//! deferred action timing, preflight check side effects, health freshness,
//! generated-versus-user-owned boundary, and delete-generated or continue-without-starter
//! recovery language, so the start-center, gallery, preflight, diff-review, and handoff
//! surfaces never let a generic Create hide a network, dependency-install, remote-provisioning,
//! trust, or managed-workspace side effect, never blur the generated-versus-user-owned
//! boundary, and never invent a parallel scaffold grammar.
//!
//! It also owns the scaffold-template-card and starter-parameter-row controls packet, which
//! implements the first two frozen scaffold-component families as two co-equal control vectors —
//! binding each template card to its starter source class, template support class, target
//! runtime and toolchain, host boundary, and setup-task or extension impact, with a derived
//! source class and support posture so a community, local, mirrored, or unknown starter never
//! reads as governed first-party and bridge behavior never reads as exact first-party support;
//! and binding each parameter row to its source-precedence origin class (`Template default`,
//! `User input`, `Workspace value`, `Policy value`, or `Secret reference`), its frozen source
//! layer, its action timing, its validation state, and a derived portability class so a
//! workspace-scoped or policy-managed value never reads as portable user input and a secret
//! reference never reveals a raw value — so the start-center, template-gallery, parameter-form,
//! and support surfaces can inspect source, support, host, and precedence posture before a user
//! commits, and never route creation through a generic Create.
//!
//! It also owns the scaffold-preflight-card and template-health-row controls packet, which
//! implements the next two frozen scaffold-component families as two co-equal control vectors —
//! binding each preflight card to its target path and name, its generated file and folder counts,
//! its dependency / task / extension impact, the concrete side effect it discloses (package
//! install, dependency restore, remote provisioning, trust prompt, script execution, or extension
//! install), whether that action runs immediately or is deferred, and a named checkpoint or
//! delete-generated recovery path, with a derived severity so a blocked prerequisite never reads
//! as an optional optimization and a not-run or unknown check never reads as passed; and binding
//! each health row to its check name, status, freshness / source, `Blocker` / `Warning` / `Info`
//! severity, auto-fix or manual-fix note, and a derived freshness posture so a stale, expired,
//! never-checked, or unavailable signal never reads as fresh, and it keeps an explicit same-weight
//! path to `Create empty` or `Continue without starter` — so the start-center, preflight,
//! template-health, and support surfaces never let a generic Create hide a side effect and never
//! let a health row monopolize the plain create-without-starter path.
//!
//! It also owns the generated-project-diff-card and scaffold-handoff-banner controls packet, which
//! implements the last two frozen scaffold-component families as two co-equal control vectors —
//! binding each generated-project diff card to its created / modified / renamed / deleted counts (the
//! same create / modify / rename / delete vocabulary Aureline uses for AI patches, importers, and
//! refactors), its template or generator source, its config / dependency / task / extension impact, a
//! named checkpoint or rollback / delete-generated path, and a generated-versus-user-owned boundary
//! cue, with a derived review disposition and boundary posture so a conflict or unavailable diff never
//! reads as a clean applied change and a user-owned zone never reads as free-to-overwrite generated
//! output; and binding each scaffold handoff banner to its created-workspace identity, its trust
//! state, its health summary, its `Run now` / `Run later` / `Review files` / `Open manifest` choices,
//! and a delete-generated or reopen-preflight recovery route, with a derived outcome posture so a
//! partial or failed bootstrap never reads as a clean create — so the diff-review, workspace-handoff,
//! start-center, and support surfaces keep generated output reviewable and recoverable after Aureline
//! writes files, and never assume the safest next step for the user.
//!
//! It also owns the starter-boundary-state controls packet, which implements the cross-cutting
//! starter boundary state any of the six frozen scaffold components can carry when a scaffold's
//! source, availability, trust, or durability is not the plain public-registry default — binding
//! each boundary state to its boundary kind (`public_registry`, `mirror_only`, `offline_cache_only`,
//! `sign_in_required`, `remote_or_managed_workspace`, or `non_durable_temp_staging`), its
//! availability state, its owner and freshness cues, its trust-and-install disclosure, and a
//! delete-generated / reuse-existing / clone-elsewhere / continue-without-starter recovery route,
//! with a derived access and availability posture so a sign-in-gated, managed-remote, or non-durable
//! starter never reads as a plain public-registry create and an unavailable or blocked starter never
//! reads as ready — so the start-center, template-gallery, and scaffold-preflight surfaces can tell
//! a user what a starter depends on before any silent trust or install step, and always keep an
//! explicit recovery path once a starter partially materializes output.
//!
//! It also owns the scaffold-component consumer-adoption packet, which proves the six frozen
//! scaffold-component families are reusable components — not one start-center page plus a few
//! isolated bootstrap objects — by binding every claimed M5 scaffold consumer (the start center,
//! workspace admission, the template registry, framework packs, workflow bundles, help / support,
//! and the safe handoff / export packet) to the same canonical component schemas and the one shared
//! source-and-support / side-effect / health-freshness / recovery-and-ownership descriptor
//! vocabulary, so starter source / support, side-effect, health-freshness, and
//! generated-versus-user-owned / recovery language stops drifting between gallery cards, entry review
//! sheets, workflow-bundle surfaces, and support artifacts; a weakened parity-health mode (an
//! unverified source / support, a pending side-effect disclosure, a stale health signal, or a
//! recovery-required partial generation) auto-narrows the claim with a self-contained banner naming
//! the exact reason and recovery action, and a side-effect-bearing starter never presents a plain
//! ready create.
//!
//! It also owns the scaffold-component accessibility-and-auto-narrowing packet, which certifies —
//! per frozen scaffold-component family — that the scaffold template card, starter parameter row,
//! scaffold preflight card, template health row, generated-project diff card, and scaffold handoff
//! banner stay keyboard-complete, screen-reader-reachable, CLI/headless-reachable, and export-safe
//! without a raw value, and that they auto-narrow honestly: when a template's freshness drifts, a
//! prerequisite health check is blocked, a starter parameter is secret-bound and cannot travel, a
//! generation diff's truth is partial, or a validation state is cached / not checked, the
//! component's readiness claim drops from a qualified starter to a secret-bound-parameter,
//! blocked-prerequisite, drifted-template, partial-generation, or unchecked-validation projection
//! that names its precise trigger and binding dimension and keeps the starter source / support /
//! recovery boundary — so incomplete readiness evidence never presents a starter as fully qualified,
//! and the hierarchy-heavy generated-project diff tree always binds a flat list / textual path.
//!
//! It also owns the scaffold-component surface-certification packet, which *certifies* that the
//! shared scaffold-component truth holds on every claimed M5 stack-entry and project-generation
//! surface — the start center, the template gallery, the scaffold preflight, the generation
//! diff-review, the workspace handoff, the template-health dashboard, the support / export bundle,
//! and the CLI / headless surface — scoring each surface across six truth axes (visual, keyboard,
//! screen-reader, always-on export, degraded-state, and source-side-effect-and-recovery) and either
//! passing it green, auto-narrowing its `QualifiedStarter` claim to a blocked-prerequisite,
//! drifted-template, partial-generation, or secret-bound-parameter projection (yellow), or blocking
//! it (red) when a degraded axis is hidden behind a full claim; every certified surface cites the one
//! canonical scaffold-component proof bundle, keeps its starter-source / support / side-effect /
//! generated-versus-user-owned / recovery continuity, never lets a generic `Create` hide a side
//! effect, and never exposes a secret-bound raw value by default.
//!
//! It also owns the framework-pack-header and framework-status-strip controls packet, which
//! implements the frozen framework-pack-header component family as two co-equal control vectors —
//! binding each pack header to its pack identity and version range, its support class, its
//! provider source, its selected workspace scope, its freshness, its certainty, and its execution
//! boundary, with a derived support posture, framework-experience class (core native, pack-backed,
//! bridged, or heuristic), and scope posture (local, container, remote, managed, or unknown) so
//! bridge or heuristic behavior never reads as exact first-party support and a remote, managed,
//! container, or unknown scope never reads as local; and binding each compact status strip to its
//! detected framework and version, its pack health, its compatibility notes, and its
//! bridge-or-heuristic posture wherever a framework-aware feature is claimed — so the
//! framework-pack, route / topology, convention-diagnostics, generator-review, CLI, and support
//! surfaces can tell which pack and version is active, how it is supported, who provides it, and
//! whether the current scope is local or remote before a user trusts a framework lens, and never
//! invent a parallel pack-header grammar.
//!
//! It also owns the route-endpoint-row and component-service-tree-node controls packet, which
//! implements those two frozen topology-explorer component families as two co-equal control
//! vectors — binding each route / endpoint row to its route / matcher, source file / symbol, HTTP /
//! UI / runtime kind, owning framework / app, params / guards, freshness, and evidence source, with
//! a derived certainty posture (exact from source, runtime confirmed, heuristic, or partial /
//! unresolved) and authorship posture (authored, generated, framework provided, runtime only, or
//! unknown origin) so a heuristic route never reads as exact and the authored-versus-generated
//! boundary stays visible at row level; and binding each component / service tree node to its
//! entity kind, source file / symbol, parent / child or provider / consumer relation, related test
//! / story / doc links, and partial or derived notes — so the route-explorer, topology-explorer,
//! editor-gutter, CLI, and support surfaces can inspect a topology row without hiding its evidence
//! basis, every row and node links back to a canonical proving source rather than acting as a
//! hidden parallel model, and a runtime-only or unresolved component never fakes a source it does
//! not have.
//!
//! It also owns the convention-diagnostic-row and derived-relationship-banner controls packet, which
//! implements those two frozen framework-diagnostic component families as two co-equal control
//! vectors — binding each convention-diagnostic row to its distinct diagnostic class (a hard contract
//! violation, a pack limitation, a version mismatch, a heuristic suspicion, a deprecation notice, or
//! an unknown diagnostic), its affected entity / file, its confidence and severity, its detected
//! source, its suggested fix / open-docs action, and its support-class caveat, with a derived
//! certainty posture (exact from source, runtime confirmed, heuristic, or partial / unresolved) so a
//! framework warning never collapses hard contract violations, pack limitations, version mismatches,
//! and heuristic suspicions into one generic warning state and a heuristic suspicion never reads as an
//! exact contract fact; and binding each derived-relationship banner to its source of inference, its
//! last refresh, its exact / partial / heuristic / runtime-confirmed state, its named place of
//! consumption, and its open-raw-source or open-wider-graph actions so an inferred link never reads as
//! exact and a banner appears exactly where inferred framework truth is consumed instead of hiding the
//! approximation in the background — so the convention-diagnostics, editor-gutter, topology-explorer,
//! CLI, and support surfaces can read a framework warning or an inferred relationship without losing
//! its certainty, distinct-class, support-caveat, or proving-source truth, and never invent a parallel
//! diagnostic or banner grammar.
//!
//! Finally, it owns the M5 certification packet, which certifies every claimed M5
//! template, scaffold, framework-pack, and archetype-health *profile* on the
//! mainline branch — binding each profile to a certification verdict, the upstream
//! evidence packet that backs the claim, the downgrade triggers that can narrow it,
//! a rollback posture, and proof freshness — and aggregates the per-profile verdicts
//! into a single promotion verdict, so CI or release tooling can fail promotion or
//! narrow the claim automatically instead of shipping greener than the evidence.

pub mod add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure;
pub mod add_generation_diff_review_rollback_or_delete_generated_recovery_and_managed_zone_honesty;
pub mod add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter;
pub mod add_shared_start_center_workspace_admission_template_registry_framework_pack_workflow_bundle_and_support_consumers_so_scaffold_components_keep_source_side_effect_and_health_language_aligned_across_claimed_m5_profiles;
pub mod certify_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_truth_on_every_claimed_m5_stack_entry_and_project_generation_surface;
pub mod certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile;
pub mod freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix;
pub mod freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix;
pub mod implement_convention_diagnostic_rows_and_derived_relationship_banners_with_diagnostic_class_affected_entity_or_file_certainty_detected_source_suggested_fix_or_open_docs_actions_support_class_caveats_and_open_raw_source_or_wider_graph_continuity;
pub mod implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse;
pub mod implement_framework_pack_headers_and_framework_status_strips_with_pack_identity_version_support_range_provider_source_freshness_compatibility_and_local_versus_remote_scope_truth;
pub mod implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners;
pub mod implement_generated_project_diff_cards_and_scaffold_handoff_banners_with_create_modify_rename_delete_counts_dependency_task_extension_impact_trust_state_and_run_now_later_review_recovery_truth_across_claimed_m5_generation_flows;
pub mod implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_template_freshness_drifted_prerequisite_health_is_blocked_starter_parameters_are_secret_bound_or_generation_diff_truth_is_partial_across_claimed_m5_scaffold_components;
pub mod implement_route_endpoint_rows_and_component_service_tree_nodes_with_authored_versus_generated_state_proving_source_files_or_symbols_exact_versus_heuristic_labels_and_open_source_or_open_references_continuity;
pub mod implement_scaffold_template_cards_and_starter_parameter_rows_with_source_support_host_boundary_and_portability_truth_across_claimed_m5_project_entry_surfaces;
pub mod implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows;
pub mod ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance;
pub mod ship_mirror_offline_auth_boundary_and_managed_zone_starter_states_with_no_silent_trust_no_silent_install_and_non_durable_temp_staging_honesty_across_claimed_m5_scaffold_surfaces;
pub mod ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth;
pub mod ship_scaffold_preflight_cards_and_template_health_rows_with_generated_file_counts_immediate_versus_deferred_actions_blocked_warning_optional_checks_and_create_empty_parity_across_claimed_m5_bootstrap_lanes;
