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

pub mod add_generation_diff_review_rollback_or_delete_generated_recovery_and_managed_zone_honesty;
pub mod implement_the_signed_template_registry_provenance_or_mirror_support_and_template_health_rows;
