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
pub mod certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile;
pub mod implement_framework_generators_or_codemods_with_preview_diff_rollback_and_execution_context_reuse;
pub mod implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners;
pub mod implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows;
pub mod ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance;
pub mod ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth;
