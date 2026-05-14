//! Install-topology alpha contracts for product and support surfaces.
//!
//! This crate owns the first runtime-consumable install-topology alpha model.
//! It projects one packet into About, update, diagnostics, install-review,
//! CLI, and support-export rows so those surfaces explain the same install
//! mode, channel, updater owner, binary root, durable state roots, repair /
//! verify support, mirror or offline posture, handler ownership, and rollback
//! owner.
//!
//! The crate consumes the already-seeded release topology artifacts by
//! reference. It does not implement an installer, updater, package manager, or
//! fleet-control service. Its job is to keep claimed topology truth bounded,
//! inspectable, and consistent before those mutating systems exist.

#![doc(html_root_url = "https://docs.rs/aureline-install/0.0.0")]

pub mod topology;

pub use topology::{
    ArchitectureClass, BinaryRootClass, ChannelClass, ChannelPinningPosture, ContractRefs,
    HandlerKind, HandlerOwnership, HandlerOwnershipChangePreview, HiddenGlobalStateGuarantee,
    ImportHandoffPosture, InstallModeClass, InstallTopologyAlphaPacket, InstallTopologyCoverage,
    InstallTopologyRow, InstallTopologySupportExport, InstallTopologySurfaceProjection,
    InstallTopologySurfaceRow, InstallTopologyTruthFingerprint, InstallTopologyValidationFinding,
    InstallTopologyValidationReport, ManagedPackageReportClass, MirrorOfflineVerificationState,
    PlatformClass, PolicyInjectionClass, PublicationPostureClass, RepairVerifySupport,
    RollbackOwnerClass, RollbackPosture, RolloutRingClass, ShellIntegrationLimits,
    SideBySideRelationClass, SilentDeploymentPosture, SilentInstallSupportClass,
    StaleHandlerOwnerDiagnostic, TopologySurfaceClass, UpdaterOwnerClass,
    INSTALL_TOPOLOGY_ALPHA_PACKET_RECORD_KIND, INSTALL_TOPOLOGY_ALPHA_SCHEMA_VERSION,
};
