//! External-drift detection for compare-before-write save safety.
//!
//! The staged save pipeline MUST refuse to commit bytes when the pinned save
//! target no longer matches authoritative on-disk identity. This module
//! centralizes that decision so save flows do not re-derive the conflict rules
//! ad hoc.

use aureline_vfs::{AtomicWriteMode, SaveOutcome, SaveTargetToken, VfsRoot};

/// Conflict returned when the pinned save target has drifted from the current
/// authoritative identity on the root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalDriftConflict {
    /// Typed outcome describing the drift class.
    pub outcome: SaveOutcome,
    /// Human-readable failure detail that can be recorded into a save manifest.
    pub detail: String,
}

impl std::fmt::Display for ExternalDriftConflict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{outcome}: {detail}",
            outcome = self.outcome.as_str(),
            detail = self.detail
        )
    }
}

impl std::error::Error for ExternalDriftConflict {}

/// Detects external drift for a pinned [`SaveTargetToken`].
///
/// Returns `Ok(())` when the current canonical object identity and
/// compare-before-write generation token match the token pinned at open time.
/// Otherwise returns an [`ExternalDriftConflict`] describing why the save must
/// refuse to commit.
pub fn detect_external_drift(
    root: &dyn VfsRoot,
    token: &SaveTargetToken,
) -> Result<(), ExternalDriftConflict> {
    let canonical_uri = &token.identity.canonical_filesystem_object.canonical_uri;
    let presentation_uri = &token.identity.presentation_path.uri;

    // Re-resolve the presentation path so wrong-target prevention can fail
    // closed when the canonical target changes.
    match root.identity_record(presentation_uri) {
        Ok(identity) => {
            if identity.canonical_filesystem_object.canonical_uri != *canonical_uri {
                let detail = format!(
                    "canonical target drifted: opened {} now resolves to {}",
                    canonical_uri, identity.canonical_filesystem_object.canonical_uri
                );
                return Err(ExternalDriftConflict {
                    outcome: SaveOutcome::WrongTargetPrevented,
                    detail,
                });
            }
        }
        Err(err) => {
            return Err(ExternalDriftConflict {
                outcome: SaveOutcome::WrongTargetPrevented,
                detail: err.to_string(),
            });
        }
    }

    // Compare-before-write (correctness floor).
    let current_generation = match root.read_generation_token(canonical_uri) {
        Ok(token) => token,
        Err(err) => {
            return Err(ExternalDriftConflict {
                outcome: SaveOutcome::WrongTargetPrevented,
                detail: err.to_string(),
            });
        }
    };

    let pinned = &token.compare_before_write_generation_token;
    if pinned.kind != current_generation.kind || pinned.value != current_generation.value {
        let outcome = if token.atomic_write_mode == AtomicWriteMode::ConditionalRemoteWrite {
            SaveOutcome::SaveConflict
        } else {
            SaveOutcome::ExternalChangeDetected
        };
        let detail = format!(
            "generation_token_mismatch: pinned {pinned_value} observed {observed_value}",
            pinned_value = pinned.value,
            observed_value = current_generation.value,
        );
        return Err(ExternalDriftConflict { outcome, detail });
    }

    Ok(())
}
