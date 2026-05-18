//! Governed schema and record-class registry access.
//!
//! This crate embeds the checked-in schema-family and record-class registries
//! used by trust, support, release, admin, and automation surfaces. It validates
//! that governed payload families name owners, schema versions, consent classes,
//! endpoint classes, record-class bindings, retention posture, lifecycle state,
//! and downgrade rules before downstream code renders or emits those contracts.

#![doc(html_root_url = "https://docs.rs/aureline-governance/0.0.0")]

pub mod schema_registry;

pub use schema_registry::{
    load_default_record_class_registry, load_default_schema_registry, validate_default_registries,
    DowngradeRule, GovernanceSurfaceClass, GovernedRecordClassRegistry, GovernedRecordClassRow,
    GovernedSchemaRegistry, GovernedSchemaRow, PacketVersionSupport, SchemaRegistryError,
    SchemaRegistryValidationReport, SeparationRule, SurfaceProjection, SurfaceSchemaRow,
    GOVERNED_RECORD_CLASS_REGISTRY_JSON, GOVERNED_RECORD_CLASS_REGISTRY_PATH,
    GOVERNED_SCHEMA_REGISTRY_JSON, GOVERNED_SCHEMA_REGISTRY_PATH,
};
