//! HarfBuzz-class shaping implementation.
//!
//! This module implements the `rust_native` path from ADR 0002 using
//! [`rustybuzz`]. The output is a list of glyph instances annotated with:
//!
//! - the chosen font face id,
//! - the cluster (byte) boundary for caret-safe navigation, and
//! - the fallback stage that resolved the glyph.

use std::collections::HashMap;
use std::hash::{Hash as _, Hasher as _};

use unicode_bidi::BidiInfo;
use unicode_segmentation::UnicodeSegmentation as _;

use crate::shaping::fonts::FontFallbackConfig;
use crate::shaping::types::{FallbackStage, FeatureSet, ShaperPolicy, TextDirection};
use crate::shaping::FontSystem;

/// A single glyph positioned for painting.
#[derive(Debug, Clone, PartialEq)]
pub struct ShapedGlyph {
    /// Font-local glyph id.
    pub glyph_id: u32,
    /// Font face chosen for this glyph.
    pub font_id: fontdb::ID,
    /// Byte offset into the original line that owns this glyph cluster.
    pub cluster: usize,
    /// Horizontal position relative to the line origin, in pixels.
    pub x: f32,
    /// Vertical position relative to the line origin, in pixels.
    pub y: f32,
    /// Horizontal advance in pixels.
    pub advance: f32,
    /// Fallback stage that resolved this glyph.
    pub fallback_stage: FallbackStage,
    /// Logical direction used to shape this glyph run.
    pub direction: TextDirection,
}

/// Output of shaping a single line of text.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ShapedLine {
    /// Glyphs in visual order (left-to-right on the line).
    pub glyphs: Vec<ShapedGlyph>,
    /// Total advance width for the shaped line in pixels.
    pub width_px: f32,
}

/// Structural metrics for shaping runs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ShaperMetrics {
    /// Number of shaping calls observed (cache hits included).
    pub shape_calls: u64,
    /// Number of cache hits.
    pub cache_hits: u64,
    /// Number of cache misses.
    pub cache_misses: u64,
    /// Total glyphs emitted across all runs.
    pub glyph_count: u64,
    /// Number of missing-glyph clusters that fell through to stage 5.
    pub missing_glyph_count: u64,
    /// Counts of fallback stages 1..=5.
    pub fallback_stage_counts: [u64; 5],
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ShapeCacheKey {
    text_hash: u64,
    text_len: usize,
    font_size_bits: u32,
    features: FeatureSet,
    system_ui_family: crate::shaping::fonts::GenericFamily,
    explicit_family_hash: u64,
}

/// Production shaper implementation with an internal memoization cache.
#[derive(Debug)]
pub struct TextShaper {
    policy: ShaperPolicy,
    cache: HashMap<ShapeCacheKey, ShapedLine>,
    metrics: ShaperMetrics,
    max_cache_entries: usize,
}

impl TextShaper {
    /// Creates a new shaper configured for the default policy.
    pub fn new() -> Self {
        Self {
            policy: ShaperPolicy::default(),
            cache: HashMap::new(),
            metrics: ShaperMetrics::default(),
            max_cache_entries: 4096,
        }
    }

    /// Sets the maximum number of cached shaped lines.
    pub fn set_cache_capacity(&mut self, max_entries: usize) {
        self.max_cache_entries = max_entries.max(1);
        if self.cache.len() > self.max_cache_entries {
            self.cache.clear();
        }
    }

    /// Returns the active shaper policy.
    pub fn policy(&self) -> ShaperPolicy {
        self.policy
    }

    /// Returns a snapshot of the current metrics.
    pub fn metrics(&self) -> ShaperMetrics {
        self.metrics
    }

    /// Clears the internal shape cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Shapes a single line of text, applying bidi reordering and the fallback chain.
    pub fn shape_line(
        &mut self,
        font_system: &mut FontSystem,
        text: &str,
        font_size_px: f32,
        fallback: &FontFallbackConfig,
        features: FeatureSet,
    ) -> ShapedLine {
        self.metrics.shape_calls = self.metrics.shape_calls.saturating_add(1);

        let key = cache_key(text, font_size_px, fallback, features);
        if let Some(cached) = self.cache.get(&key) {
            self.metrics.cache_hits = self.metrics.cache_hits.saturating_add(1);
            return cached.clone();
        }
        self.metrics.cache_misses = self.metrics.cache_misses.saturating_add(1);

        let mut line =
            self.shape_line_uncached(font_system, text, font_size_px, fallback, features);

        if self.cache.len() >= self.max_cache_entries {
            // Simple bounded cache: clear rather than partially evict. The atlas
            // is the more important cache layer for the hot path.
            self.cache.clear();
        }
        self.cache.insert(key, line.clone());

        // Keep counts outside the cache so callsites can treat cache hits as
        // real shaping work avoided.
        self.metrics.glyph_count = self
            .metrics
            .glyph_count
            .saturating_add(line.glyphs.len() as u64);
        for glyph in &line.glyphs {
            let idx = (glyph.fallback_stage.stage_number() - 1) as usize;
            if let Some(slot) = self.metrics.fallback_stage_counts.get_mut(idx) {
                *slot = slot.saturating_add(1);
            }
        }
        self.metrics.missing_glyph_count = self.metrics.missing_glyph_count.saturating_add(
            line.glyphs
                .iter()
                .filter(|g| g.fallback_stage == FallbackStage::Missing)
                .count() as u64,
        );

        // Avoid NaN propagation from scaling edge cases; keep widths positive
        // even when the underlying shaper reports RTL advances with a
        // negative pen direction.
        if !line.width_px.is_finite() {
            line.width_px = 0.0;
        } else {
            line.width_px = line.width_px.abs();
        }
        line
    }

    fn shape_line_uncached(
        &mut self,
        font_system: &mut FontSystem,
        text: &str,
        font_size_px: f32,
        fallback: &FontFallbackConfig,
        features: FeatureSet,
    ) -> ShapedLine {
        if text.is_empty() {
            return ShapedLine::default();
        }

        let bidi = BidiInfo::new(text, None);
        let para = match bidi.paragraphs.first() {
            Some(p) => p,
            None => return ShapedLine::default(),
        };
        let line_range = para.range.clone();
        let (_, visual_runs) = bidi.visual_runs(para, line_range);

        let mut out = ShapedLine::default();
        let mut x_cursor = 0.0f32;

        for run in visual_runs {
            let Some(run_text) = text.get(run.clone()) else {
                continue;
            };
            let direction = bidi
                .levels
                .get(run.start)
                .map(|level| {
                    if level.is_rtl() {
                        TextDirection::Rtl
                    } else {
                        TextDirection::Ltr
                    }
                })
                .unwrap_or(TextDirection::Ltr);

            let shaped = shape_visual_run(
                font_system,
                run_text,
                run.start,
                direction,
                font_size_px,
                fallback,
                features,
                x_cursor,
            );
            x_cursor = shaped.width_px;
            out.glyphs.extend(shaped.glyphs);
        }

        out.width_px = x_cursor;
        out
    }
}

impl Default for TextShaper {
    fn default() -> Self {
        Self::new()
    }
}

fn shape_visual_run(
    font_system: &mut FontSystem,
    run_text: &str,
    run_base: usize,
    direction: TextDirection,
    font_size_px: f32,
    fallback: &FontFallbackConfig,
    features: FeatureSet,
    x_origin: f32,
) -> ShapedLine {
    let mut out = ShapedLine::default();
    let mut x_cursor = x_origin;

    let mut segment_clusters: Vec<(usize, &str, fontdb::ID, FallbackStage)> = Vec::new();
    for (rel_byte, cluster) in run_text.grapheme_indices(true) {
        let (font_id, stage) = font_system.resolve_cluster_face(cluster, fallback);
        let Some(font_id) = font_id else {
            continue;
        };
        segment_clusters.push((run_base + rel_byte, cluster, font_id, stage));
    }

    let mut i = 0usize;
    while i < segment_clusters.len() {
        let (_, _, segment_font, segment_stage) = segment_clusters[i];
        let segment_start = i;
        let mut segment_end = i + 1;
        while segment_end < segment_clusters.len() {
            let (_, _, next_font, next_stage) = segment_clusters[segment_end];
            if next_font != segment_font || next_stage != segment_stage {
                break;
            }
            segment_end += 1;
        }

        let mut buffer = rustybuzz::UnicodeBuffer::new();
        for (cluster_byte, cluster_text, _, _) in &segment_clusters[segment_start..segment_end] {
            for ch in cluster_text.chars() {
                buffer.add(ch, (*cluster_byte) as u32);
            }
        }

        buffer.guess_segment_properties();
        buffer.set_direction(match direction {
            TextDirection::Ltr => rustybuzz::Direction::LeftToRight,
            TextDirection::Rtl => rustybuzz::Direction::RightToLeft,
        });

        let hb_features = hb_features(features);
        let Some(face) = font_system.rustybuzz_face(segment_font) else {
            i = segment_end;
            continue;
        };

        let glyph_buffer = rustybuzz::shape(&face, &hb_features, buffer);
        let infos = glyph_buffer.glyph_infos();
        let positions = glyph_buffer.glyph_positions();

        let units_per_em = face.units_per_em().max(1) as f32;
        let scale = font_size_px / units_per_em;

        for (info, pos) in infos.iter().zip(positions.iter()) {
            let x_offset = (pos.x_offset as f32) * scale;
            let y_offset = -(pos.y_offset as f32) * scale;
            let advance = (pos.x_advance as f32) * scale;
            out.glyphs.push(ShapedGlyph {
                glyph_id: info.glyph_id,
                font_id: segment_font,
                cluster: info.cluster as usize,
                x: x_cursor + x_offset,
                y: y_offset,
                advance,
                fallback_stage: segment_stage,
                direction,
            });
            x_cursor += advance;
        }

        i = segment_end;
    }

    let pen_end = x_cursor;
    let advance = (pen_end - x_origin).abs();
    if pen_end < x_origin {
        for glyph in &mut out.glyphs {
            glyph.x += advance;
        }
    }
    out.width_px = x_origin + advance;
    out
}

fn hb_features(features: FeatureSet) -> Vec<rustybuzz::Feature> {
    let mut out = Vec::new();
    if !features.ligatures {
        for tag in ["-liga", "-clig", "-calt"] {
            if let Ok(feature) = tag.parse::<rustybuzz::Feature>() {
                out.push(feature);
            }
        }
    }
    if features.stylistic_set > 0 {
        let ss = features.stylistic_set.min(20);
        let tag = format!("ss{:02}=1", ss);
        if let Ok(feature) = tag.parse::<rustybuzz::Feature>() {
            out.push(feature);
        }
    }
    out
}

fn cache_key(
    text: &str,
    font_size_px: f32,
    fallback: &FontFallbackConfig,
    features: FeatureSet,
) -> ShapeCacheKey {
    ShapeCacheKey {
        text_hash: hash64(text.as_bytes()),
        text_len: text.len(),
        font_size_bits: font_size_px.to_bits(),
        features,
        system_ui_family: fallback.system_ui_family,
        explicit_family_hash: fallback
            .explicit_family
            .as_deref()
            .map(|name| hash64(name.as_bytes()))
            .unwrap_or(0),
    }
}

fn hash64(bytes: &[u8]) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    bytes.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::path::PathBuf;

    #[test]
    fn empty_input_yields_empty_output() {
        let mut shaper = TextShaper::new();
        let mut fonts = FontSystem::new();
        let out = shaper.shape_line(
            &mut fonts,
            "",
            16.0,
            &FontFallbackConfig::ui_sans(),
            FeatureSet::ui_default(),
        );
        assert!(out.glyphs.is_empty());
        assert_eq!(out.width_px, 0.0);
    }

    fn font_system_with_ahem() -> FontSystem {
        let mut fonts = FontSystem::new();
        fonts.load_font_data(font_test_data::AHEM.to_vec());
        fonts
    }

    #[test]
    fn cache_hit_returns_identical_shape() {
        let mut shaper = TextShaper::new();
        let mut fonts = font_system_with_ahem();
        let config = FontFallbackConfig::ui_sans();
        let a = shaper.shape_line(&mut fonts, "hello", 16.0, &config, FeatureSet::plain());
        let b = shaper.shape_line(&mut fonts, "hello", 16.0, &config, FeatureSet::plain());
        assert_eq!(a, b);
        let m = shaper.metrics();
        assert!(m.cache_hits >= 1);
    }

    #[test]
    fn fallback_fixtures_emit_missing_stage() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let fixture_dir = repo_root.join("fixtures/text/font_fallback_cases");
        let mut paths: Vec<_> = std::fs::read_dir(&fixture_dir)
            .expect("fixture directory missing")
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "txt"))
            .collect();
        paths.sort();
        assert!(!paths.is_empty(), "no font fallback fixtures found");

        let config = FontFallbackConfig::ui_sans();
        let mut shaper = TextShaper::new();
        let mut fonts = font_system_with_ahem();

        for path in paths {
            let raw = std::fs::read_to_string(&path).expect("fixture read failed");
            let text = raw.trim_end_matches(['\n', '\r']);
            let shaped =
                shaper.shape_line(&mut fonts, text, 16.0, &config, FeatureSet::ui_default());
            assert!(
                !shaped.glyphs.is_empty(),
                "fixture produced no glyphs: {path:?}"
            );
            assert!(
                shaped
                    .glyphs
                    .iter()
                    .any(|glyph| glyph.fallback_stage == FallbackStage::Missing),
                "fixture should trigger missing fallback: {path:?}"
            );

            let boundaries: HashSet<usize> =
                text.grapheme_indices(true).map(|(idx, _)| idx).collect();
            for glyph in &shaped.glyphs {
                assert!(
                    boundaries.contains(&glyph.cluster),
                    "glyph cluster is not a grapheme boundary: {path:?} offset={}",
                    glyph.cluster
                );
            }
        }
    }
}
