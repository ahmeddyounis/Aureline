//! Prototype text stack.
//!
//! Validates the ADR 0002 contract (shaper trait, fallback chain,
//! glyph-cache posture, invalidation model) without pulling in a real
//! native font toolchain. The prototype is intentionally coarse: it
//! models grapheme segmentation, script detection, a four-stage
//! fallback chain, a shape cache, and a raster cache. Every seam the
//! production renderer will honour is named here so later work can
//! replace the stubs one implementation at a time.
//!
//! The prototype does not claim hot-path parity. Known holes are
//! recorded in `prototypes/text_stack/README.md`, not in code
//! comments; this module only documents invariants a reader of the
//! API must know.

use std::collections::hash_map::Entry;
use std::collections::BTreeMap;
use std::collections::HashMap;

/// Stage in the fallback chain; ADR 0002 §Font discovery and fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FallbackStage {
    /// Caller-declared explicit family (stage 1).
    ExplicitFamily,
    /// Script-aware preference group (stage 2): fires the
    /// `fallback_glyph_resolution` hook.
    ScriptPreferenceGroup,
    /// OS system-UI family for the active locale (stage 3).
    SystemUi,
    /// Bundled signed Noto-class subset (stage 4).
    BundledSubset,
    /// Terminal `.notdef` — must be zero on a supported host.
    Missing,
}

impl FallbackStage {
    pub const fn name(self) -> &'static str {
        match self {
            Self::ExplicitFamily => "explicit_family",
            Self::ScriptPreferenceGroup => "script_preference_group",
            Self::SystemUi => "system_ui",
            Self::BundledSubset => "bundled_subset",
            Self::Missing => "missing",
        }
    }

    pub const fn stage_number(self) -> u8 {
        match self {
            Self::ExplicitFamily => 1,
            Self::ScriptPreferenceGroup => 2,
            Self::SystemUi => 3,
            Self::BundledSubset => 4,
            Self::Missing => 5,
        }
    }

    /// ADR 0002: the `fallback_glyph_resolution` hook fires whenever a
    /// cluster resolves through stage 2 or later (Missing inclusive).
    pub const fn fires_fallback_hook(self) -> bool {
        self.stage_number() >= 2
    }

    pub const ALL: &'static [FallbackStage] = &[
        Self::ExplicitFamily,
        Self::ScriptPreferenceGroup,
        Self::SystemUi,
        Self::BundledSubset,
        Self::Missing,
    ];
}

/// Synthetic font handle. The prototype does not open real font files;
/// the handle records which fallback role the font plays so consumers
/// can diff two runs without depending on platform font discovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FontHandle {
    EditorDefault,
    HanFallback,
    HangulFallback,
    ArabicFallback,
    EmojiFallback,
    SystemUi,
    BundledNoto,
}

impl FontHandle {
    pub const fn name(self) -> &'static str {
        match self {
            Self::EditorDefault => "editor_default",
            Self::HanFallback => "han_fallback",
            Self::HangulFallback => "hangul_fallback",
            Self::ArabicFallback => "arabic_fallback",
            Self::EmojiFallback => "emoji_fallback",
            Self::SystemUi => "system_ui",
            Self::BundledNoto => "bundled_noto",
        }
    }

    /// Does the font cover the cluster's first base character?
    pub fn supports(self, cluster: &Cluster) -> bool {
        let script = cluster.script;
        match self {
            Self::EditorDefault => matches!(script, Script::Latin),
            Self::HanFallback => matches!(script, Script::Han | Script::Kana),
            Self::HangulFallback => matches!(script, Script::Hangul),
            Self::ArabicFallback => matches!(script, Script::Arabic | Script::Hebrew),
            Self::EmojiFallback => matches!(script, Script::Emoji),
            // SystemUI covers Latin, Kana, Han, Hangul, Arabic, Hebrew,
            // Emoji, and Unknown — everything the prototype recognises.
            Self::SystemUi => !matches!(script, Script::Unknown),
            // BundledNoto is the terminal stage: always covers so a
            // supported host never falls through to `.notdef`.
            Self::BundledNoto => true,
        }
    }
}

/// Shaper policy (ADR: default `rust_native`; `platform_native` opt-in).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaperPolicy {
    RustNative,
    PlatformNative,
}

impl ShaperPolicy {
    pub const fn name(self) -> &'static str {
        match self {
            Self::RustNative => "rust_native",
            Self::PlatformNative => "platform_native",
        }
    }
}

/// Feature flags on a shaping run. Kept minimal so consumers share one
/// feature vocabulary rather than inventing feature strings per surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct FeatureSet {
    pub ligatures: bool,
    pub stylistic_set: u8,
}

impl FeatureSet {
    pub const fn plain() -> Self {
        Self {
            ligatures: false,
            stylistic_set: 0,
        }
    }

    pub const fn with_ligatures() -> Self {
        Self {
            ligatures: true,
            stylistic_set: 0,
        }
    }
}

/// Coarse script classification. The prototype only resolves enough
/// distinctions to drive the fallback chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Script {
    Latin,
    Han,
    Kana,
    Hangul,
    Arabic,
    Hebrew,
    Emoji,
    Unknown,
}

impl Script {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Latin => "latin",
            Self::Han => "han",
            Self::Kana => "kana",
            Self::Hangul => "hangul",
            Self::Arabic => "arabic",
            Self::Hebrew => "hebrew",
            Self::Emoji => "emoji",
            Self::Unknown => "unknown",
        }
    }

    pub const fn is_rtl(self) -> bool {
        matches!(self, Self::Arabic | Self::Hebrew)
    }

    /// Detect the script of a cluster from its first base codepoint.
    pub fn detect(cluster: &str) -> Self {
        for ch in cluster.chars() {
            if is_continuation(ch) {
                continue;
            }
            return classify_codepoint(ch);
        }
        Self::Unknown
    }
}

fn classify_codepoint(ch: char) -> Script {
    let cp = ch as u32;
    if is_emoji(cp) {
        Script::Emoji
    } else if (0x0590..=0x05FF).contains(&cp) {
        Script::Hebrew
    } else if (0x0600..=0x06FF).contains(&cp)
        || (0x0750..=0x077F).contains(&cp)
        || (0x08A0..=0x08FF).contains(&cp)
        || (0xFB50..=0xFDFF).contains(&cp)
        || (0xFE70..=0xFEFF).contains(&cp)
    {
        Script::Arabic
    } else if (0xAC00..=0xD7AF).contains(&cp)
        || (0x1100..=0x11FF).contains(&cp)
        || (0x3130..=0x318F).contains(&cp)
    {
        Script::Hangul
    } else if (0x3040..=0x309F).contains(&cp)
        || (0x30A0..=0x30FF).contains(&cp)
        || (0x31F0..=0x31FF).contains(&cp)
    {
        Script::Kana
    } else if (0x3400..=0x4DBF).contains(&cp)
        || (0x4E00..=0x9FFF).contains(&cp)
        || (0xF900..=0xFAFF).contains(&cp)
        || (0x20000..=0x2FFFF).contains(&cp)
    {
        Script::Han
    } else if (0x0020..=0x024F).contains(&cp) || (0x2000..=0x206F).contains(&cp) {
        // Basic Latin, Latin-1 Supplement, Latin Extended-A/B, general
        // punctuation dashes and spaces used by code and UI labels.
        Script::Latin
    } else {
        Script::Unknown
    }
}

fn is_emoji(cp: u32) -> bool {
    (0x1F000..=0x1FFFF).contains(&cp)
        || (0x2600..=0x27BF).contains(&cp)
        || (0x2300..=0x23FF).contains(&cp)
        || (0x1F1E6..=0x1F1FF).contains(&cp)
}

fn is_continuation(ch: char) -> bool {
    let cp = ch as u32;
    // Combining diacritics ranges.
    (0x0300..=0x036F).contains(&cp)
        || (0x1AB0..=0x1AFF).contains(&cp)
        || (0x1DC0..=0x1DFF).contains(&cp)
        || (0x20D0..=0x20FF).contains(&cp)
        || (0xFE20..=0xFE2F).contains(&cp)
        // Zero-width joiner.
        || cp == 0x200D
        // Variation selectors.
        || (0xFE00..=0xFE0F).contains(&cp)
        || (0xE0100..=0xE01EF).contains(&cp)
        // Emoji modifiers (skin tone).
        || (0x1F3FB..=0x1F3FF).contains(&cp)
}

fn is_regional_indicator(ch: char) -> bool {
    (0x1F1E6..=0x1F1FF).contains(&(ch as u32))
}

/// One grapheme cluster.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cluster {
    pub text: String,
    pub byte_offset: usize,
    pub script: Script,
    pub is_rtl: bool,
}

/// Segment `text` into grapheme clusters using a simplified rule set
/// sufficient for the smoke corpus: combining marks, variation
/// selectors, and emoji modifiers attach to the previous base; ZWJ
/// (U+200D) additionally extends an emoji cluster across a following
/// emoji; regional indicators pair into flags.
pub fn segment_graphemes(text: &str) -> Vec<Cluster> {
    let mut out: Vec<Cluster> = Vec::new();
    let chars: Vec<(usize, char)> = text.char_indices().collect();
    let mut i = 0;
    while i < chars.len() {
        let (byte_offset, ch) = chars[i];
        if is_continuation(ch) {
            if let Some(prev) = out.last_mut() {
                prev.text.push(ch);
                i += 1;
                continue;
            }
        }
        let mut cluster_text = String::new();
        cluster_text.push(ch);
        i += 1;
        if is_regional_indicator(ch) && i < chars.len() && is_regional_indicator(chars[i].1) {
            cluster_text.push(chars[i].1);
            i += 1;
        }
        while i < chars.len() {
            let next = chars[i].1;
            if is_continuation(next) {
                cluster_text.push(next);
                i += 1;
                continue;
            }
            if cluster_text.ends_with('\u{200D}') && classify_codepoint(next) == Script::Emoji {
                cluster_text.push(next);
                i += 1;
                continue;
            }
            break;
        }
        let script = Script::detect(&cluster_text);
        out.push(Cluster {
            text: cluster_text,
            byte_offset,
            script,
            is_rtl: script.is_rtl(),
        });
    }
    out
}

/// A shaped cluster: the stub resolves one glyph per cluster and records
/// the fallback stage the glyph came from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShapedCluster {
    pub text: String,
    pub byte_offset: usize,
    pub glyph_id: u32,
    pub font: FontHandle,
    pub fallback_stage: FallbackStage,
    pub script: Script,
    pub is_rtl: bool,
}

/// The output of a shaping run.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ShapedRun {
    pub clusters: Vec<ShapedCluster>,
    pub missing_glyph_count: u32,
}

impl ShapedRun {
    pub fn fired_fallback_hook(&self) -> bool {
        self.clusters
            .iter()
            .any(|c| c.fallback_stage.stage_number() >= 2)
    }

    pub fn fallback_stage_counts(&self) -> [u64; 5] {
        let mut counts = [0u64; 5];
        for cluster in &self.clusters {
            let idx = (cluster.fallback_stage.stage_number() - 1) as usize;
            counts[idx] += 1;
        }
        counts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Ltr,
    Rtl,
}

impl Direction {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }
}

/// Deterministic font-discovery and fallback chain. ADR 0002 names
/// exactly four stages; this struct keeps the roles named rather than
/// ordered-by-index so reopening a stage is a field change, not a
/// list-index shift.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FallbackChain {
    pub explicit: FontHandle,
    pub per_script: BTreeMap<Script, FontHandle>,
    pub system_ui: FontHandle,
    pub bundled_subset: FontHandle,
}

impl FallbackChain {
    /// The default chain used by the editor theme in the prototype.
    pub fn default_editor() -> Self {
        let mut per_script = BTreeMap::new();
        per_script.insert(Script::Han, FontHandle::HanFallback);
        per_script.insert(Script::Kana, FontHandle::HanFallback);
        per_script.insert(Script::Hangul, FontHandle::HangulFallback);
        per_script.insert(Script::Arabic, FontHandle::ArabicFallback);
        per_script.insert(Script::Hebrew, FontHandle::ArabicFallback);
        per_script.insert(Script::Emoji, FontHandle::EmojiFallback);
        Self {
            explicit: FontHandle::EditorDefault,
            per_script,
            system_ui: FontHandle::SystemUi,
            bundled_subset: FontHandle::BundledNoto,
        }
    }

    /// Resolve a cluster through the full chain in ADR order.
    pub fn resolve(&self, cluster: &Cluster) -> (FontHandle, FallbackStage) {
        if self.explicit.supports(cluster) {
            return (self.explicit, FallbackStage::ExplicitFamily);
        }
        if let Some(font) = self.per_script.get(&cluster.script) {
            if font.supports(cluster) {
                return (*font, FallbackStage::ScriptPreferenceGroup);
            }
        }
        if self.system_ui.supports(cluster) {
            return (self.system_ui, FallbackStage::SystemUi);
        }
        if self.bundled_subset.supports(cluster) {
            return (self.bundled_subset, FallbackStage::BundledSubset);
        }
        (self.bundled_subset, FallbackStage::Missing)
    }
}

/// Shape-cache key.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ShapeKey {
    pub cluster_text: String,
    pub font: FontHandle,
    pub features: FeatureSet,
    pub direction: Direction,
    pub script: Script,
}

/// Raster-cache key.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct RasterKey {
    pub glyph_id: u32,
    pub font: FontHandle,
    /// Pixel size in units of 1/256 px so the key stays integer.
    pub px_size_q8: u32,
    pub subpixel_variant: u8,
    pub scale_bucket: u8,
}

/// Metrics accumulator. Structural counts only; no wall-clock values.
/// Consumers layer timing on top when they need it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ShapingMetrics {
    pub shape_calls: u64,
    pub shape_cache_hits: u64,
    pub shape_cache_misses: u64,
    pub raster_cache_hits: u64,
    pub raster_cache_misses: u64,
    pub cluster_count: u64,
    pub missing_glyph_count: u64,
    pub fallback_stage_counts: [u64; 5],
}

impl ShapingMetrics {
    pub fn merge_run(&mut self, run: &ShapedRun) {
        self.cluster_count += run.clusters.len() as u64;
        self.missing_glyph_count += u64::from(run.missing_glyph_count);
        for (idx, count) in run.fallback_stage_counts().iter().enumerate() {
            self.fallback_stage_counts[idx] += count;
        }
    }
}

/// Shaper trait — lifted from ADR 0002 into the prototype API. The
/// production renderer implements the same trait against `rustybuzz`
/// or a platform-native engine.
pub trait Shaper {
    fn policy(&self) -> ShaperPolicy;
    fn shape(&mut self, text: &str, chain: &FallbackChain, features: FeatureSet) -> ShapedRun;
    fn shape_metrics(&self) -> &ShapingMetrics;
}

/// Reference stub Shaper. Pure Rust; no native toolchain.
#[derive(Debug, Default)]
pub struct StubShaper {
    policy: ShaperPolicy,
    shape_cache: HashMap<ShapeKey, ShapedCluster>,
    metrics: ShapingMetrics,
}

impl StubShaper {
    pub fn new() -> Self {
        Self {
            policy: ShaperPolicy::RustNative,
            shape_cache: HashMap::new(),
            metrics: ShapingMetrics::default(),
        }
    }

    pub fn with_policy(policy: ShaperPolicy) -> Self {
        Self {
            policy,
            shape_cache: HashMap::new(),
            metrics: ShapingMetrics::default(),
        }
    }

    pub fn shape_cache_len(&self) -> usize {
        self.shape_cache.len()
    }
}

impl Default for ShaperPolicy {
    fn default() -> Self {
        Self::RustNative
    }
}

impl Shaper for StubShaper {
    fn policy(&self) -> ShaperPolicy {
        self.policy
    }

    fn shape(&mut self, text: &str, chain: &FallbackChain, features: FeatureSet) -> ShapedRun {
        self.metrics.shape_calls += 1;
        let clusters = segment_graphemes(text);
        let mut shaped = Vec::with_capacity(clusters.len());
        let mut missing_glyph_count = 0u32;
        for cluster in clusters {
            let (font, stage) = chain.resolve(&cluster);
            let direction = if cluster.is_rtl {
                Direction::Rtl
            } else {
                Direction::Ltr
            };
            let key = ShapeKey {
                cluster_text: cluster.text.clone(),
                font,
                features,
                direction,
                script: cluster.script,
            };
            let shaped_cluster = if let Some(cached) = self.shape_cache.get(&key) {
                self.metrics.shape_cache_hits += 1;
                ShapedCluster {
                    byte_offset: cluster.byte_offset,
                    ..cached.clone()
                }
            } else {
                self.metrics.shape_cache_misses += 1;
                let glyph_id = synthesize_glyph_id(&cluster.text, font);
                let cluster_shape = ShapedCluster {
                    text: cluster.text.clone(),
                    byte_offset: cluster.byte_offset,
                    glyph_id,
                    font,
                    fallback_stage: stage,
                    script: cluster.script,
                    is_rtl: cluster.is_rtl,
                };
                self.shape_cache.insert(key, cluster_shape.clone());
                cluster_shape
            };
            if matches!(stage, FallbackStage::Missing) {
                missing_glyph_count += 1;
            }
            shaped.push(shaped_cluster);
        }
        let run = ShapedRun {
            clusters: shaped,
            missing_glyph_count,
        };
        self.metrics.merge_run(&run);
        run
    }

    fn shape_metrics(&self) -> &ShapingMetrics {
        &self.metrics
    }
}

/// FNV-1a over the cluster text, salted with the font handle so the
/// same text on different fallback fonts yields distinct glyph ids.
fn synthesize_glyph_id(text: &str, font: FontHandle) -> u32 {
    let mut hash: u64 = 0xCBF29CE484222325;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001B3);
    }
    hash ^= u64::from(font as u32).wrapping_add(1);
    hash = hash.wrapping_mul(0x100000001B3);
    ((hash ^ (hash >> 32)) & 0xFFFF_FFFF) as u32
}

/// High-level text layer: pairs the shaper with the raster cache so the
/// caller sees one API and one metrics stream. Invalidation follows
/// ADR 0002: a px_size / scale_bucket change invalidates the raster
/// cache but not the shape cache.
#[derive(Debug)]
pub struct TextLayer {
    shaper: StubShaper,
    raster_cache: HashMap<RasterKey, u32>,
    chain: FallbackChain,
    px_size_q8: u32,
    scale_bucket: u8,
    metrics: ShapingMetrics,
}

impl TextLayer {
    pub const DEFAULT_PX_SIZE_Q8: u32 = 14 * 256;
    pub const DEFAULT_SCALE_BUCKET: u8 = 1;

    pub fn new_default() -> Self {
        Self {
            shaper: StubShaper::new(),
            raster_cache: HashMap::new(),
            chain: FallbackChain::default_editor(),
            px_size_q8: Self::DEFAULT_PX_SIZE_Q8,
            scale_bucket: Self::DEFAULT_SCALE_BUCKET,
            metrics: ShapingMetrics::default(),
        }
    }

    pub fn with_chain(chain: FallbackChain) -> Self {
        let mut layer = Self::new_default();
        layer.chain = chain;
        layer
    }

    pub fn chain(&self) -> &FallbackChain {
        &self.chain
    }

    pub fn policy(&self) -> ShaperPolicy {
        self.shaper.policy()
    }

    /// Invalidate the raster cache as a consequence of a monitor scale
    /// change. Corresponds to ADR's `multi_monitor_scale_change` hook.
    pub fn on_scale_change(&mut self, new_scale_bucket: u8, new_px_size_q8: u32) {
        self.raster_cache.clear();
        self.scale_bucket = new_scale_bucket;
        self.px_size_q8 = new_px_size_q8;
    }

    pub fn raster_cache_len(&self) -> usize {
        self.raster_cache.len()
    }

    pub fn shape_cache_len(&self) -> usize {
        self.shaper.shape_cache_len()
    }

    /// Shape and rasterise a string end to end. Raster cache hits mean
    /// no GPU upload would have happened; misses would upload a new
    /// glyph in the production pipeline.
    pub fn render(&mut self, text: &str, features: FeatureSet) -> ShapedRun {
        let run = self.shaper.shape(text, &self.chain, features);
        for cluster in &run.clusters {
            let key = RasterKey {
                glyph_id: cluster.glyph_id,
                font: cluster.font,
                px_size_q8: self.px_size_q8,
                subpixel_variant: 0,
                scale_bucket: self.scale_bucket,
            };
            match self.raster_cache.entry(key) {
                Entry::Occupied(_) => {
                    self.metrics.raster_cache_hits += 1;
                }
                Entry::Vacant(entry) => {
                    self.metrics.raster_cache_misses += 1;
                    entry.insert(cluster.glyph_id);
                }
            }
        }
        self.merge_shape_side();
        self.metrics.merge_run(&run);
        run
    }

    fn merge_shape_side(&mut self) {
        let shaper = self.shaper.shape_metrics();
        self.metrics.shape_calls = shaper.shape_calls;
        self.metrics.shape_cache_hits = shaper.shape_cache_hits;
        self.metrics.shape_cache_misses = shaper.shape_cache_misses;
    }

    pub fn metrics(&self) -> TextLayerMetrics {
        let m = &self.metrics;
        TextLayerMetrics {
            shape_calls: m.shape_calls,
            shape_cache_hits: m.shape_cache_hits,
            shape_cache_misses: m.shape_cache_misses,
            raster_cache_hits: m.raster_cache_hits,
            raster_cache_misses: m.raster_cache_misses,
            cluster_count: m.cluster_count,
            missing_glyph_count: m.missing_glyph_count,
            fallback_stage_counts: m.fallback_stage_counts,
        }
    }
}

/// Public, cloneable snapshot of the text layer's metrics.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TextLayerMetrics {
    pub shape_calls: u64,
    pub shape_cache_hits: u64,
    pub shape_cache_misses: u64,
    pub raster_cache_hits: u64,
    pub raster_cache_misses: u64,
    pub cluster_count: u64,
    pub missing_glyph_count: u64,
    pub fallback_stage_counts: [u64; 5],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_resolves_through_explicit_family() {
        let mut layer = TextLayer::new_default();
        let run = layer.render("hello", FeatureSet::plain());
        assert_eq!(run.clusters.len(), 5);
        for cluster in &run.clusters {
            assert_eq!(cluster.fallback_stage, FallbackStage::ExplicitFamily);
            assert_eq!(cluster.font, FontHandle::EditorDefault);
            assert_eq!(cluster.script, Script::Latin);
        }
        assert_eq!(run.missing_glyph_count, 0);
        assert!(!run.fired_fallback_hook());
    }

    #[test]
    fn cjk_falls_through_script_preference_group() {
        let mut layer = TextLayer::new_default();
        let run = layer.render("漢字", FeatureSet::plain());
        assert_eq!(run.clusters.len(), 2);
        for cluster in &run.clusters {
            assert_eq!(cluster.fallback_stage, FallbackStage::ScriptPreferenceGroup);
            assert_eq!(cluster.font, FontHandle::HanFallback);
            assert_eq!(cluster.script, Script::Han);
        }
        assert!(run.fired_fallback_hook());
    }

    #[test]
    fn arabic_is_rtl_and_script_preference() {
        let mut layer = TextLayer::new_default();
        let run = layer.render("مرحبا", FeatureSet::plain());
        assert!(run.clusters.iter().all(|c| c.is_rtl));
        assert!(run
            .clusters
            .iter()
            .all(|c| c.font == FontHandle::ArabicFallback));
        assert!(run.fired_fallback_hook());
    }

    #[test]
    fn emoji_zwj_cluster_coalesces() {
        let mut layer = TextLayer::new_default();
        // Family ZWJ sequence: man + ZWJ + woman + ZWJ + girl.
        let run = layer.render("👨\u{200D}👩\u{200D}👧", FeatureSet::plain());
        assert_eq!(
            run.clusters.len(),
            1,
            "ZWJ family should coalesce into one cluster"
        );
        assert_eq!(run.clusters[0].font, FontHandle::EmojiFallback);
    }

    #[test]
    fn flag_regional_indicators_pair() {
        let mut layer = TextLayer::new_default();
        // Japan flag = RI(U+1F1EF) + RI(U+1F1F5).
        let run = layer.render("\u{1F1EF}\u{1F1F5}", FeatureSet::plain());
        assert_eq!(run.clusters.len(), 1);
        assert_eq!(run.clusters[0].font, FontHandle::EmojiFallback);
    }

    #[test]
    fn combining_mark_attaches_to_base() {
        let mut layer = TextLayer::new_default();
        // Decomposed "á" = a + U+0301.
        let run = layer.render("a\u{0301}b", FeatureSet::plain());
        assert_eq!(run.clusters.len(), 2);
        assert_eq!(run.clusters[0].text.chars().count(), 2);
        assert_eq!(run.clusters[1].text, "b");
    }

    #[test]
    fn shape_cache_hits_on_repeat() {
        let mut layer = TextLayer::new_default();
        layer.render("hello", FeatureSet::plain());
        let before = layer.metrics();
        layer.render("hello", FeatureSet::plain());
        let after = layer.metrics();
        assert!(
            after.shape_cache_hits > before.shape_cache_hits,
            "second render should be served from the shape cache"
        );
        assert_eq!(after.shape_cache_misses, before.shape_cache_misses);
    }

    #[test]
    fn raster_cache_hits_on_repeat() {
        let mut layer = TextLayer::new_default();
        layer.render("hi", FeatureSet::plain());
        let before_hits = layer.metrics().raster_cache_hits;
        layer.render("hi", FeatureSet::plain());
        let after_hits = layer.metrics().raster_cache_hits;
        assert!(after_hits > before_hits);
    }

    #[test]
    fn scale_change_invalidates_raster_cache_but_not_shape_cache() {
        let mut layer = TextLayer::new_default();
        layer.render("hi", FeatureSet::plain());
        let shape_before = layer.shape_cache_len();
        let raster_before = layer.raster_cache_len();
        assert!(raster_before > 0);
        layer.on_scale_change(2, 28 * 256);
        assert_eq!(layer.raster_cache_len(), 0);
        assert_eq!(layer.shape_cache_len(), shape_before);
    }

    #[test]
    fn fallback_histogram_counts_every_cluster() {
        let mut layer = TextLayer::new_default();
        layer.render("hello 漢字 مرحبا", FeatureSet::plain());
        let metrics = layer.metrics();
        let total: u64 = metrics.fallback_stage_counts.iter().sum();
        assert_eq!(total, metrics.cluster_count);
    }

    #[test]
    fn deterministic_glyph_ids_across_runs() {
        let mut a = TextLayer::new_default();
        let mut b = TextLayer::new_default();
        let ra = a.render("fn main() { println!(\"hi\"); }", FeatureSet::plain());
        let rb = b.render("fn main() { println!(\"hi\"); }", FeatureSet::plain());
        assert_eq!(
            ra.clusters.iter().map(|c| c.glyph_id).collect::<Vec<_>>(),
            rb.clusters.iter().map(|c| c.glyph_id).collect::<Vec<_>>()
        );
    }
}
