//! Source-class vocabulary: which lexical lane produced a row.
//!
//! M1 ships only filename and path lanes. Any future semantic / symbol /
//! reference lane lives in a separate crate and produces its own source
//! class; the search shell MUST NOT relabel a lexical row as semantic just
//! because a richer surface is wired up alongside it.

use serde::{Deserialize, Serialize};

/// Stable lexical-source vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceClass {
    /// Match was found in the basename (filename) of a workspace file.
    LexicalFilename,
    /// Match was found in the full workspace-relative path of a file.
    LexicalPath,
}

impl SourceClass {
    /// Stable token used in records, fixtures, and shell snapshots.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LexicalFilename => "lexical_filename",
            Self::LexicalPath => "lexical_path",
        }
    }

    /// Human-readable label suitable for a search-shell group header.
    pub const fn group_label(self) -> &'static str {
        match self {
            Self::LexicalFilename => "Filenames",
            Self::LexicalPath => "Paths",
        }
    }

    /// Short attribution badge surfaces render next to a row to make the
    /// match lane explicit (the search shell never implies semantic depth).
    pub const fn badge(self) -> &'static str {
        match self {
            Self::LexicalFilename => "filename",
            Self::LexicalPath => "path",
        }
    }
}
