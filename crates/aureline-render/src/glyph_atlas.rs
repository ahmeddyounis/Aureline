//! Glyph atlas and raster-cache management.
//!
//! The renderer keeps a per-surface glyph cache keyed by font face, glyph id,
//! and scale bucket. The backing store is a set of "atlas shards" (one per
//! scale bucket) that own rasterized glyph images and expose eviction metrics.
//!
//! This module intentionally models the atlas as a cache of glyph images rather
//! than as a concrete GPU texture. The current renderer backend blits a CPU
//! raster buffer; later GPU paths can replace the backing store while keeping
//! the same contract and metrics vocabulary.

use std::collections::HashMap;

use aureline_text::shaping::FontSystem;
use swash::scale::{image::Image, Render, ScaleContext, Source, StrikeWith};
use swash::zeno::Format;

/// Unique key for a rasterized glyph entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlyphKey {
    pub glyph_id: u32,
    pub font_id: fontdb::ID,
    /// Pixel size in units of 1/256 px so the key stays integer.
    pub px_size_q8: u32,
    pub subpixel_variant: u8,
    pub scale_bucket: u8,
}

/// LRU eviction reason code for an atlas shard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvictionReason {
    /// Entry was evicted due to LRU pressure.
    Lru,
    /// Entry was evicted because the shard reached its configured capacity.
    AtlasFull,
    /// Entry was evicted because its font was unloaded.
    FontUnloaded,
    /// Entry was evicted because the scale bucket was retired.
    ScaleBucketRetired,
}

impl EvictionReason {
    pub const fn id(self) -> &'static str {
        match self {
            Self::Lru => "lru",
            Self::AtlasFull => "atlas_full",
            Self::FontUnloaded => "font_unloaded",
            Self::ScaleBucketRetired => "scale_bucket_retired",
        }
    }
}

/// A cached glyph image entry.
#[derive(Clone)]
pub struct GlyphEntry {
    pub key: GlyphKey,
    pub image: Image,
    last_used: u64,
}

/// Structural atlas statistics for diagnostics and benchmarks.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GlyphAtlasStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub shard_count: usize,
    pub entry_count: usize,
}

struct AtlasShard {
    entries: HashMap<GlyphKey, GlyphEntry>,
    max_entries: usize,
}

impl AtlasShard {
    fn new(max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_entries: max_entries.max(1),
        }
    }

    fn clear(&mut self) -> usize {
        let count = self.entries.len();
        self.entries.clear();
        count
    }
}

/// Per-surface glyph atlas: one shard per scale bucket.
pub struct GlyphAtlas {
    shards: HashMap<u8, AtlasShard>,
    scale: ScaleContext,
    hits: u64,
    misses: u64,
    evictions: u64,
    use_counter: u64,
    max_entries_per_shard: usize,
    hint: bool,
}

impl GlyphAtlas {
    /// Creates a new glyph atlas with the given per-shard capacity.
    pub fn new(max_entries_per_shard: usize) -> Self {
        Self {
            shards: HashMap::new(),
            scale: ScaleContext::new(),
            hits: 0,
            misses: 0,
            evictions: 0,
            use_counter: 0,
            max_entries_per_shard: max_entries_per_shard.max(1),
            hint: true,
        }
    }

    /// Enables or disables hinting for outline rasterization.
    pub fn set_hinting(&mut self, hint: bool) {
        self.hint = hint;
    }

    /// Clears one scale-bucket shard (used by scale-change invalidation).
    pub fn clear_scale_bucket(&mut self, scale_bucket: u8) -> Option<(usize, EvictionReason)> {
        let removed = self.shards.get_mut(&scale_bucket)?.clear();
        if removed > 0 {
            self.evictions = self.evictions.saturating_add(removed as u64);
        }
        Some((removed, EvictionReason::ScaleBucketRetired))
    }

    /// Returns current atlas statistics.
    pub fn stats(&self) -> GlyphAtlasStats {
        let entry_count = self.shards.values().map(|s| s.entries.len()).sum();
        GlyphAtlasStats {
            hits: self.hits,
            misses: self.misses,
            evictions: self.evictions,
            shard_count: self.shards.len(),
            entry_count,
        }
    }

    /// Returns a cached glyph entry, rasterizing it into the shard if missing.
    ///
    /// The returned entry is stored in the atlas until evicted.
    pub fn get_or_rasterize(
        &mut self,
        font_system: &mut FontSystem,
        key: GlyphKey,
    ) -> Option<&GlyphEntry> {
        let shard = self
            .shards
            .entry(key.scale_bucket)
            .or_insert_with(|| AtlasShard::new(self.max_entries_per_shard));

        self.use_counter = self.use_counter.saturating_add(1);
        let now = self.use_counter;

        if shard.entries.contains_key(&key) {
            self.hits = self.hits.saturating_add(1);
            {
                let entry = shard.entries.get_mut(&key)?;
                entry.last_used = now;
            }
            return shard.entries.get(&key);
        }

        self.misses = self.misses.saturating_add(1);
        let image = rasterize_glyph(&mut self.scale, font_system, key, self.hint)?;

        if shard.entries.len() >= shard.max_entries {
            if let Some((victim_key, _)) = shard
                .entries
                .iter()
                .min_by_key(|(_, entry)| entry.last_used)
                .map(|(k, v)| (*k, v.last_used))
            {
                shard.entries.remove(&victim_key);
                self.evictions = self.evictions.saturating_add(1);
            }
        }

        shard.entries.insert(
            key,
            GlyphEntry {
                key,
                image,
                last_used: now,
            },
        );
        shard.entries.get(&key)
    }
}

impl Default for GlyphAtlas {
    fn default() -> Self {
        Self::new(8192)
    }
}

fn rasterize_glyph(
    context: &mut ScaleContext,
    font_system: &mut FontSystem,
    key: GlyphKey,
    hint: bool,
) -> Option<Image> {
    let font = font_system.swash_font(key.font_id)?;
    let size = (key.px_size_q8 as f32) / 256.0;
    let mut scaler = context.builder(font).size(size).hint(hint).build();

    let glyph_id = u16::try_from(key.glyph_id).ok()?;
    Render::new(&[
        Source::ColorOutline(0),
        Source::ColorBitmap(StrikeWith::BestFit),
        Source::Outline,
    ])
    .format(Format::Alpha)
    .render(&mut scaler, glyph_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atlas_stats_start_empty() {
        let atlas = GlyphAtlas::new(4);
        let stats = atlas.stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
        assert_eq!(stats.entry_count, 0);
    }

    #[test]
    fn rasterizes_and_reuses_cached_entry() {
        let mut fonts = FontSystem::new();
        fonts.load_font_data(font_test_data::AHEM.to_vec());
        let font_id = fonts
            .database()
            .faces()
            .next()
            .map(|face| face.id)
            .expect("test font should load");
        let glyph_id = fonts
            .rustybuzz_face(font_id)
            .and_then(|face| face.glyph_index('A').map(|gid| u32::from(gid.0)))
            .expect("test font should include 'A'");

        let mut atlas = GlyphAtlas::new(16);
        let key = GlyphKey {
            glyph_id,
            font_id,
            px_size_q8: 16 * 256,
            subpixel_variant: 0,
            scale_bucket: 16,
        };

        let (first_key, first_len) = {
            let first = atlas
                .get_or_rasterize(&mut fonts, key)
                .expect("glyph rasterization should succeed");
            (first.key, first.image.data.len())
        };
        assert!(first_len > 0);
        let stats = atlas.stats();
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 1);

        let second_key = atlas
            .get_or_rasterize(&mut fonts, key)
            .expect("glyph should be cached")
            .key;
        assert_eq!(first_key, second_key);
        let stats = atlas.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }
}
