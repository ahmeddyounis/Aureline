//! Font discovery and fallback selection.
//!
//! This module wraps the platform font database (`fontdb`) and exposes the
//! deterministic fallback chain from ADR 0002 as a small, testable API.

use std::collections::HashMap;
use std::sync::Arc;

use fontdb::{Database, Family, Query};
use rustybuzz::Face as HbFace;
use swash::FontRef;

use crate::shaping::types::FallbackStage;

/// Generic family choice used as the stage-3 system UI selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GenericFamily {
    /// Sans-serif UI text.
    SansSerif,
    /// Monospace code/terminal text.
    Monospace,
    /// Serif (rare in the shell, but part of the vocabulary).
    Serif,
}

impl GenericFamily {
    fn as_fontdb_family(self) -> Family<'static> {
        match self {
            Self::SansSerif => Family::SansSerif,
            Self::Monospace => Family::Monospace,
            Self::Serif => Family::Serif,
        }
    }
}

/// Inputs for selecting fonts during shaping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontFallbackConfig {
    /// Optional explicit family (stage 1).
    pub explicit_family: Option<String>,
    /// Generic family used for the OS system-ui stage (stage 3).
    pub system_ui_family: GenericFamily,
}

impl FontFallbackConfig {
    /// Default configuration for UI copy.
    pub fn ui_sans() -> Self {
        Self {
            explicit_family: None,
            system_ui_family: GenericFamily::SansSerif,
        }
    }

    /// Default configuration for code-like roles.
    pub fn monospace() -> Self {
        Self {
            explicit_family: None,
            system_ui_family: GenericFamily::Monospace,
        }
    }
}

#[derive(Debug, Clone)]
struct CachedFace {
    bytes: Arc<Vec<u8>>,
    face_index: u32,
    swash_offset: u32,
    swash_key: swash::CacheKey,
}

impl CachedFace {
    fn as_swash(&self) -> FontRef<'_> {
        FontRef {
            data: self.bytes.as_slice(),
            offset: self.swash_offset,
            key: self.swash_key,
        }
    }

    fn as_rustybuzz(&self) -> Option<HbFace<'_>> {
        HbFace::from_slice(self.bytes.as_slice(), self.face_index)
    }
}

/// Platform font database plus face-data cache.
#[derive(Debug)]
pub struct FontSystem {
    db: Database,
    cached_faces: HashMap<fontdb::ID, CachedFace>,
    last_resort_cache: HashMap<u32, Option<fontdb::ID>>,
}

impl FontSystem {
    /// Creates an empty font system.
    pub fn new() -> Self {
        Self {
            db: Database::new(),
            cached_faces: HashMap::new(),
            last_resort_cache: HashMap::new(),
        }
    }

    /// Creates a font system loaded with platform fonts.
    pub fn with_system_fonts() -> Self {
        let mut system = Self::new();
        system.db.load_system_fonts();
        system
    }

    /// Returns the number of loaded font faces.
    pub fn face_count(&self) -> usize {
        self.db.len()
    }

    /// Loads font data from bytes (TTF/OTF/TTC).
    pub fn load_font_data(&mut self, data: Vec<u8>) {
        self.db.load_font_data(data);
    }

    /// Returns the underlying database for diagnostics.
    pub fn database(&self) -> &Database {
        &self.db
    }

    fn cached_face(&mut self, id: fontdb::ID) -> Option<&CachedFace> {
        if !self.cached_faces.contains_key(&id) {
            let (bytes, face_index) = self
                .db
                .with_face_data(id, |data, face_index| (data.to_vec(), face_index))?;
            let font_ref = FontRef::from_index(&bytes, face_index as usize)?;
            let swash_offset = font_ref.offset;
            let swash_key = font_ref.key;
            self.cached_faces.insert(
                id,
                CachedFace {
                    bytes: Arc::new(bytes),
                    face_index,
                    swash_offset,
                    swash_key,
                },
            );
        }
        self.cached_faces.get(&id)
    }

    /// Returns a [`swash::FontRef`] for the given face id.
    pub fn swash_font(&mut self, id: fontdb::ID) -> Option<FontRef<'_>> {
        Some(self.cached_face(id)?.as_swash())
    }

    /// Returns a `rustybuzz` face handle for the given face id.
    pub fn rustybuzz_face(&mut self, id: fontdb::ID) -> Option<HbFace<'_>> {
        self.cached_face(id)?.as_rustybuzz()
    }

    /// Resolves the stage-1 explicit family if provided.
    pub fn resolve_explicit_face(&self, family_name: &str) -> Option<fontdb::ID> {
        let query = Query {
            families: &[Family::Name(family_name)],
            ..Query::default()
        };
        self.db.query(&query)
    }

    /// Resolves the stage-3 system-ui face based on the configured generic family.
    pub fn resolve_system_ui_face(&self, family: GenericFamily) -> Option<fontdb::ID> {
        let query = Query {
            families: &[family.as_fontdb_family()],
            ..Query::default()
        };
        self.db.query(&query)
    }

    /// Returns true if the font supports every scalar in the cluster.
    pub fn supports_cluster(&mut self, id: fontdb::ID, cluster: &str) -> bool {
        let Some(face) = self.rustybuzz_face(id) else {
            return false;
        };
        for ch in cluster.chars() {
            if is_ignorable_for_coverage(ch) {
                continue;
            }
            if face.glyph_index(ch).is_none() {
                return false;
            }
        }
        true
    }

    /// Attempts to find a script-aware preference font for a cluster (stage 2).
    pub fn resolve_script_preference(&mut self, cluster: &str) -> Option<fontdb::ID> {
        let families = script_preference_families(cluster);
        for family in families {
            if let Some(id) = self.resolve_explicit_face(family) {
                if self.supports_cluster(id, cluster) {
                    return Some(id);
                }
            }
        }
        None
    }

    /// Resolves a cluster through the deterministic fallback chain.
    pub fn resolve_cluster_face(
        &mut self,
        cluster: &str,
        config: &FontFallbackConfig,
    ) -> (Option<fontdb::ID>, FallbackStage) {
        if let Some(family) = config.explicit_family.as_deref() {
            if let Some(id) = self.resolve_explicit_face(family) {
                if self.supports_cluster(id, cluster) {
                    return (Some(id), FallbackStage::ExplicitFamily);
                }
            }
        }

        if let Some(id) = self.resolve_script_preference(cluster) {
            return (Some(id), FallbackStage::ScriptPreferenceGroup);
        }

        if let Some(id) = self.resolve_system_ui_face(config.system_ui_family) {
            if self.supports_cluster(id, cluster) {
                return (Some(id), FallbackStage::SystemUi);
            }
        }

        if let Some(id) = self.resolve_any_face_for_cluster(cluster) {
            return (Some(id), FallbackStage::BundledSubset);
        }

        // Terminal stage: pick a deterministic face even if it lacks coverage
        // so downstream shaping can emit `.notdef` glyphs with stable
        // cluster boundaries for cursoring diagnostics.
        (self.resolve_terminal_face(config), FallbackStage::Missing)
    }

    fn resolve_any_face_for_cluster(&mut self, cluster: &str) -> Option<fontdb::ID> {
        let Some(codepoint) = first_base_codepoint(cluster) else {
            return None;
        };
        if let Some(cached) = self.last_resort_cache.get(&codepoint) {
            return *cached;
        }

        let face_ids: Vec<_> = self.db.faces().map(|face| face.id).collect();
        let found = face_ids
            .into_iter()
            .find(|id| self.supports_cluster(*id, cluster));
        self.last_resort_cache.insert(codepoint, found);
        found
    }

    fn resolve_terminal_face(&self, config: &FontFallbackConfig) -> Option<fontdb::ID> {
        if let Some(family) = config.explicit_family.as_deref() {
            if let Some(id) = self.resolve_explicit_face(family) {
                return Some(id);
            }
        }
        if let Some(id) = self.resolve_system_ui_face(config.system_ui_family) {
            return Some(id);
        }
        self.db.faces().next().map(|face| face.id)
    }
}

impl Default for FontSystem {
    fn default() -> Self {
        Self::new()
    }
}

fn is_ignorable_for_coverage(ch: char) -> bool {
    matches!(ch, '\u{200D}' | '\u{200C}' | '\u{FE0E}' | '\u{FE0F}')
        || ch.is_whitespace()
        || ch.is_ascii_punctuation()
}

fn first_base_codepoint(cluster: &str) -> Option<u32> {
    cluster
        .chars()
        .find(|ch| !is_ignorable_for_coverage(*ch))
        .map(|ch| ch as u32)
}

fn looks_like_emoji(cluster: &str) -> bool {
    cluster.chars().any(|ch| {
        let cp = ch as u32;
        (0x1F000..=0x1FFFF).contains(&cp) || (0x2600..=0x27BF).contains(&cp)
    })
}

fn script_preference_families(cluster: &str) -> &'static [&'static str] {
    if looks_like_emoji(cluster) {
        return &[
            "Apple Color Emoji",
            "Segoe UI Emoji",
            "Noto Color Emoji",
            "Twemoji Mozilla",
        ];
    }

    // The script preference groups are intentionally small. Stage 3
    // (system-ui) remains the real fallback for hosts whose font set does not
    // include these families.
    for ch in cluster.chars() {
        if is_ignorable_for_coverage(ch) {
            continue;
        }
        use unicode_script::UnicodeScript as _;
        match ch.script() {
            unicode_script::Script::Han
            | unicode_script::Script::Hiragana
            | unicode_script::Script::Katakana
            | unicode_script::Script::Hangul => {
                return &[
                    "PingFang SC",
                    "Hiragino Sans",
                    "Yu Gothic",
                    "Meiryo",
                    "Noto Sans CJK JP",
                    "Noto Sans JP",
                    "Noto Sans CJK SC",
                    "Noto Sans CJK KR",
                ];
            }
            unicode_script::Script::Arabic => {
                return &[
                    "Geeza Pro",
                    "Arial",
                    "Noto Sans Arabic",
                    "Noto Naskh Arabic",
                ];
            }
            unicode_script::Script::Hebrew => {
                return &["Arial Hebrew", "Times New Roman", "Noto Sans Hebrew"];
            }
            _ => {}
        }
        break;
    }

    &[]
}
