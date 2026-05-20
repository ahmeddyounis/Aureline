//! Voice/dictation conformance corpus harness (beta qualification lane).
//!
//! This module turns the bounded voice preview surface into a regression-gated
//! qualified row. It loads the drill corpus pinned at
//! `fixtures/ux/m3/voice_conformance_corpus/` and replays every drill through
//! the canonical validation [`crate::voice::build_voice_preview_row`] owned by
//! [`crate::voice`] — it never re-implements the ruleset.
//!
//! - [`seeded_voice_conformance_corpus`] is the single mint-from-truth source
//!   for the checked-in fixtures and manifest. The corpus is built by cloning
//!   the real claimed product rows and mutating exactly one field per negative
//!   drill, so the proof lane stays tied to the actual surface.
//! - [`run_corpus`] / [`run_corpus_from_repo_root`] return a [`CorpusReport`]
//!   that other harnesses (UI checks, support-export parity reviews) can quote.
//! - [`compute_voice_qualification`] produces the qualification packet that can
//!   keep a row Preview/Beta or force it back to Labs when corpus/privacy/parity
//!   proof is stale or incomplete.
//! - [`render_privacy_and_parity_report`] and
//!   [`render_command_equivalence_audit`] render the published artifacts.

mod corpus;
mod report;
mod runner;

use std::fs;
use std::io;
use std::path::Path;

pub use corpus::{
    seeded_voice_conformance_corpus, VoiceConformanceCorpus, VoiceCorpusManifest,
    VoiceDrillRecordType, VoiceNegativeDetection, VoiceNegativeDrill, VoicePositiveDrill,
    CORPUS_DESCRIPTION, CORPUS_DIR_REL, CORPUS_ID, CORPUS_SCHEMA_VERSION, MANIFEST_FILE_NAME,
};
pub use report::{
    compute_voice_qualification, fresh_complete_proof, render_command_equivalence_audit,
    render_privacy_and_parity_report, seeded_voice_qualification_packet, VoiceQualificationPacket,
    VoiceQualificationVerdict, VoiceRowProofStatus, VoiceRowQualification,
    VOICE_COMMAND_EQUIVALENCE_AUDIT_REF, VOICE_PREVIEW_BETA_AUDIT_DOC_REF,
    VOICE_PRIVACY_PARITY_REPORT_REF, VOICE_QUALIFICATION_PACKET_ID,
    VOICE_QUALIFICATION_PACKET_RECORD_KIND, VOICE_QUALIFICATION_SCHEMA_VERSION,
};
pub use runner::{
    corpus_dir_from_repo_root, load_corpus, run_corpus, run_corpus_from_repo_root,
    scan_for_raw_voice_leak, CorpusReport, DrillFailureReason, DrillOutcome, DrillReport,
};

/// Serializes a value as pretty JSON with a trailing newline (the on-disk
/// fixture form).
pub fn fixture_json<T: serde::Serialize>(value: &T) -> Result<String, serde_json::Error> {
    let mut json = serde_json::to_string_pretty(value)?;
    json.push('\n');
    Ok(json)
}

/// Writes the seeded corpus (manifest + every fixture) to `corpus_dir`.
///
/// This is the regeneration path used by the headless inspector; the
/// conformance test additionally asserts the on-disk corpus is bit-for-bit
/// equal to the seed so the fixtures can never drift silently.
pub fn write_corpus(corpus_dir: &Path, corpus: &VoiceConformanceCorpus) -> io::Result<()> {
    fs::create_dir_all(corpus_dir.join("positive"))?;
    fs::create_dir_all(corpus_dir.join("negative"))?;

    let manifest =
        fixture_json(&corpus.manifest).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    fs::write(corpus_dir.join(MANIFEST_FILE_NAME), manifest)?;

    for (spec, row) in &corpus.positives {
        let json = fixture_json(row).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        fs::write(corpus_dir.join(&spec.fixture), json)?;
    }
    for (spec, row) in &corpus.negatives {
        let json = fixture_json(row).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        fs::write(corpus_dir.join(&spec.fixture), json)?;
    }
    Ok(())
}
