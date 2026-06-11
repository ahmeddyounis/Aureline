//! Frozen M5 companion, incident, sync, residency, and offboarding truth packet.
//!
//! This crate owns the single export-safe packet that locks the M5 depth
//! qualification for the companion, incident, managed-sync, residency, and
//! offboarding lanes into one frozen matrix with staged rollout lanes. Each lane
//! row binds a lane to its domain, its qualification class, its staged rollout
//! stage, an explicit locality disclosure of what stays local, what is staged,
//! and what requires provider or admin continuity, its required evidence packet
//! refs, the downgrade triggers that can narrow it, a rollback posture, its
//! source contracts, and the consumer surfaces that must project it.
//!
//! The matrix is the canonical M5 control source for this lane: later companion,
//! incident, support, diagnostics, and Help/About surfaces ingest it instead of
//! cloning status text. It keeps browser and mobile companions narrow, keeps
//! incident packets attributable, keeps managed sync inspectable, keeps
//! customer-managed and end-to-end-encryption residency claims provable, and
//! guarantees offboarding never strands user-owned local work. Credential bodies,
//! raw provider payloads, and raw sync record contents never cross this boundary.

#![doc(html_root_url = "https://docs.rs/aureline-companion/0.0.0")]

pub mod freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes;
