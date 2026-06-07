//! Canonical slot registry and responsive fallback truth for stable shell surfaces.
//!
//! This module turns shell-zone policy into executable records. Core shell,
//! diagnostics, docs/help, support export, and extension admission checks consume
//! these records to answer three questions without inventing local chrome rules:
//! which slot owns a surface, where it may move under responsive pressure, and
//! what placeholder remains when a dependency disappears.

mod corpus;
mod model;

pub use corpus::{
    canonical_shell_zoning_packet, canonical_slot_registry, placeholder_hydration_cases,
    responsive_fallback_ladders, stable_surface_claims, support_export_lines,
};
pub use model::{
    AdaptiveClass, DeclaredSlotRecord, DependencyLossClass, FallbackPlacement, PlaceholderClass,
    ResponsiveFallbackLadder, ShellSlotId, ShellZoningAuditRecord, ShellZoningPacket,
    SlotHydrationCase, StableSurfaceClaim, SurfaceKind, ZoneId, SHELL_ZONING_PACKET_RECORD_KIND,
    SHELL_ZONING_SCHEMA_VERSION,
};
