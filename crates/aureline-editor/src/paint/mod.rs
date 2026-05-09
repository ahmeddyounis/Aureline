//! Software viewport compositor and paint pipeline.
//!
//! The current renderer backend blits a CPU-rasterized `0RGB` buffer. This
//! module provides the editor viewport compositor that maintains a retained
//! text layer and paints overlays without re-shaping or re-rasterizing glyphs.

use aureline_render::glyph_atlas::GlyphKey;
use aureline_render::PixelRect;
use aureline_text::shaping::{FeatureSet, FontFallbackConfig, FontSystem, TextShaper};
use aureline_ui::tokens::ColorRgba;
use unicode_segmentation::UnicodeSegmentation as _;

use crate::viewport::{EditorViewport, LineLayout, ViewportLayout};

/// Paint style inputs for the viewport compositor.
#[derive(Debug, Clone)]
pub struct ViewportPaintStyle {
    /// Background fill for the editor content area.
    pub background: ColorRgba,
    /// Text color for source lines.
    pub text: ColorRgba,
    /// Selection highlight color.
    pub selection_fill: ColorRgba,
    /// Caret color.
    pub caret: ColorRgba,
    /// Font size in pixels.
    pub font_size_px: f32,
    /// Horizontal padding inside the viewport.
    pub padding_x_px: u32,
    /// Vertical padding inside the viewport.
    pub padding_y_px: u32,
}

impl ViewportPaintStyle {
    /// Returns the line height in pixels for this style.
    pub fn line_height_px(&self, runtime: &mut EditorTextRuntime) -> u32 {
        let (_, height) = monospace_ascent_and_height(runtime, self.font_size_px);
        height.max(1)
    }
}

/// Runtime state shared across viewport paint calls.
///
/// This type bundles the font database, shaping engine, and glyph atlas used
/// by [`ViewportCompositor`]. Callers should keep one instance per process (or
/// per renderer context) so glyph caching and font discovery remain stable.
pub struct EditorTextRuntime {
    /// System font database and resolver.
    pub font_system: FontSystem,
    /// Line shaper used to convert UTF-8 text into positioned glyphs.
    pub shaper: TextShaper,
    /// Glyph atlas cache used by the current renderer backend.
    pub atlas: aureline_render::GlyphAtlas,
    /// Monospace fallback configuration for editor text shaping.
    pub fallback: FontFallbackConfig,
    /// OpenType feature set applied during shaping.
    pub features: FeatureSet,
}

impl EditorTextRuntime {
    /// Creates a runtime using system font discovery.
    pub fn with_system_fonts() -> Self {
        Self {
            font_system: FontSystem::with_system_fonts(),
            shaper: TextShaper::new(),
            atlas: aureline_render::GlyphAtlas::default(),
            fallback: FontFallbackConfig::monospace(),
            features: FeatureSet::plain(),
        }
    }

    /// Creates a runtime with an empty font database.
    pub fn empty() -> Self {
        Self {
            font_system: FontSystem::new(),
            shaper: TextShaper::new(),
            atlas: aureline_render::GlyphAtlas::default(),
            fallback: FontFallbackConfig::monospace(),
            features: FeatureSet::plain(),
        }
    }
}

impl Default for EditorTextRuntime {
    fn default() -> Self {
        Self::with_system_fonts()
    }
}

/// Software compositor for one editor viewport instance.
#[derive(Debug, Default)]
pub struct ViewportCompositor {
    retained_text_layer: Vec<u32>,
    retained_size: (u32, u32),
    retained_first_visible_line: usize,
    line_height_px: u32,
}

impl ViewportCompositor {
    /// Ensures the compositor owns a retained text buffer matching `viewport_size`.
    pub fn resize(&mut self, viewport_width: u32, viewport_height: u32) {
        self.retained_size = (viewport_width, viewport_height);
        let required = (viewport_width as usize).saturating_mul(viewport_height as usize);
        if self.retained_text_layer.len() != required {
            self.retained_text_layer.resize(required, 0);
        }
    }

    /// Fully repaints the text layer for `viewport`.
    pub fn repaint_text_layer(
        &mut self,
        viewport: &mut EditorViewport,
        document_lines: &[String],
        runtime: &mut EditorTextRuntime,
        style: &ViewportPaintStyle,
        viewport_size: (u32, u32),
    ) {
        self.resize(viewport_size.0, viewport_size.1);
        let (width, height) = viewport_size;
        fill(&mut self.retained_text_layer, style.background);

        let line_height = style.line_height_px(runtime);
        self.line_height_px = line_height;
        self.retained_first_visible_line = viewport.scroll_line();

        let visible = visible_line_count(height, line_height, style.padding_y_px);
        let mut layout = ViewportLayout {
            first_visible_line: self.retained_first_visible_line,
            line_height_px: line_height,
            viewport_width_px: width,
            viewport_height_px: height,
            lines: Vec::new(),
        };

        for i in 0..visible {
            let line_index = self
                .retained_first_visible_line
                .saturating_add(i as usize);
            if line_index >= document_lines.len() {
                break;
            }
            let y_top = style
                .padding_y_px
                .saturating_add(i.saturating_mul(line_height)) as i32;
            if y_top as u32 >= height {
                break;
            }
            let line_text = document_lines
                .get(line_index)
                .map(|s| s.as_str())
                .unwrap_or("");
            let line_layout = paint_line(
                &mut self.retained_text_layer,
                width,
                height,
                style.padding_x_px as i32,
                y_top,
                line_index,
                line_text,
                runtime,
                style,
            );
            layout.lines.push(line_layout);
        }

        viewport.set_layout(layout);
    }

    /// Applies a line-scroll translation to the retained text layer.
    ///
    /// When the scroll delta is too large or the viewport has no retained
    /// buffer, the compositor falls back to a full repaint.
    pub fn scroll_text_layer(
        &mut self,
        viewport: &mut EditorViewport,
        document_lines: &[String],
        runtime: &mut EditorTextRuntime,
        style: &ViewportPaintStyle,
        viewport_size: (u32, u32),
    ) {
        let (width, height) = viewport_size;
        let line_height = style.line_height_px(runtime);
        if self.retained_size != (width, height) || self.line_height_px != line_height {
            self.repaint_text_layer(viewport, document_lines, runtime, style, viewport_size);
            return;
        }
        let new_first = viewport.scroll_line();
        let old_first = self.retained_first_visible_line;
        if new_first == old_first {
            return;
        }

        let delta_lines = if new_first > old_first {
            i32::try_from(new_first - old_first).unwrap_or(i32::MAX)
        } else {
            -i32::try_from(old_first - new_first).unwrap_or(i32::MAX)
        };

        let abs_lines = delta_lines.unsigned_abs() as u32;
        let shift_px = abs_lines.saturating_mul(line_height);
        if shift_px == 0 || shift_px >= height {
            self.repaint_text_layer(viewport, document_lines, runtime, style, viewport_size);
            return;
        }

        let stride = width as usize;
        let shift_rows = shift_px as usize;
        let total_rows = height as usize;
        if stride == 0 || total_rows == 0 {
            return;
        }

        if delta_lines > 0 {
            // Scroll down: content moves up.
            let src_start = shift_rows.saturating_mul(stride);
            let count = total_rows
                .saturating_sub(shift_rows)
                .saturating_mul(stride);
            self.retained_text_layer.copy_within(src_start..src_start + count, 0);
            let len = self.retained_text_layer.len();
            fill_range(
                &mut self.retained_text_layer,
                count..len,
                style.background,
            );
        } else {
            // Scroll up: content moves down.
            let dst_start = shift_rows.saturating_mul(stride);
            let count = total_rows
                .saturating_sub(shift_rows)
                .saturating_mul(stride);
            self.retained_text_layer
                .copy_within(0..count, dst_start);
            let len = self.retained_text_layer.len();
            let end = dst_start.min(len);
            fill_range(
                &mut self.retained_text_layer,
                0..end,
                style.background,
            );
        }

        // Update cached layout: keep lines that remain visible and paint the newly exposed strip.
        let mut layout = viewport.layout().clone();
        layout.first_visible_line = new_first;
        layout.line_height_px = line_height;
        layout.viewport_width_px = width;
        layout.viewport_height_px = height;

        let visible = visible_line_count(height, line_height, style.padding_y_px) as usize;
        let mut next_lines: Vec<LineLayout> = Vec::with_capacity(visible);

        for i in 0..visible {
            let line_index = new_first.saturating_add(i);
            if line_index >= document_lines.len() {
                break;
            }
            let y_top = style
                .padding_y_px
                .saturating_add((i as u32).saturating_mul(line_height)) as i32;
            if y_top as u32 >= height {
                break;
            }

            if let Some(existing) = layout.line(line_index) {
                let mut reused = existing.clone();
                reused.y_top_px = y_top;
                next_lines.push(reused);
                continue;
            }

            let line_text = document_lines
                .get(line_index)
                .map(|s| s.as_str())
                .unwrap_or("");
            let line_layout = paint_line(
                &mut self.retained_text_layer,
                width,
                height,
                style.padding_x_px as i32,
                y_top,
                line_index,
                line_text,
                runtime,
                style,
            );
            next_lines.push(line_layout);
        }

        layout.lines = next_lines;
        self.retained_first_visible_line = new_first;
        viewport.set_layout(layout);
    }

    /// Composes the retained text layer plus overlays into `window_buffer`.
    pub fn compose_into_window(
        &self,
        window_buffer: &mut [u32],
        window_width: u32,
        window_height: u32,
        viewport_rect: PixelRect,
        viewport: &EditorViewport,
        style: &ViewportPaintStyle,
        clip: Option<PixelRect>,
    ) {
        if viewport_rect.is_empty()
            || window_width == 0
            || window_height == 0
            || self.retained_size.0 == 0
            || self.retained_size.1 == 0
        {
            return;
        }

        let clip = clip.unwrap_or(PixelRect::new(0, 0, window_width, window_height));
        let Some(intersection) = viewport_rect.intersection(clip) else {
            return;
        };
        if intersection.is_empty() {
            return;
        }

        copy_text_layer(
            window_buffer,
            window_width,
            window_height,
            &self.retained_text_layer,
            self.retained_size.0,
            self.retained_size.1,
            viewport_rect,
            intersection,
        );

        paint_overlays(
            window_buffer,
            window_width,
            window_height,
            viewport_rect,
            intersection,
            viewport,
            style,
        );
    }
}

fn visible_line_count(viewport_height: u32, line_height_px: u32, padding_y_px: u32) -> u32 {
    if viewport_height <= padding_y_px * 2 || line_height_px == 0 {
        return 0;
    }
    let inner_h = viewport_height.saturating_sub(padding_y_px.saturating_mul(2));
    (inner_h / line_height_px).saturating_add(2)
}

fn fill(buffer: &mut [u32], color: ColorRgba) {
    let value = color.to_u32_rgb();
    for px in buffer.iter_mut() {
        *px = value;
    }
}

fn fill_range(buffer: &mut [u32], range: std::ops::Range<usize>, color: ColorRgba) {
    let value = color.to_u32_rgb();
    let start = range.start.min(buffer.len());
    let end = range.end.min(buffer.len());
    for px in buffer[start..end].iter_mut() {
        *px = value;
    }
}

fn monospace_ascent_and_height(runtime: &mut EditorTextRuntime, font_size_px: f32) -> (f32, u32) {
    if font_size_px <= 0.0 || !font_size_px.is_finite() {
        return (0.0, 0);
    }

    let font_id = runtime
        .font_system
        .resolve_system_ui_face(runtime.fallback.system_ui_family)
        .or_else(|| {
            runtime
                .font_system
                .database()
                .faces()
                .next()
                .map(|face| face.id)
        });
    let Some(font_id) = font_id else {
        return (font_size_px, font_size_px.ceil().max(1.0) as u32);
    };
    let Some(font) = runtime.font_system.swash_font(font_id) else {
        return (font_size_px, font_size_px.ceil().max(1.0) as u32);
    };

    let metrics = font.metrics(&[]).scale(font_size_px);
    let ascent = metrics.ascent.max(0.0);
    let raw_height = (metrics.ascent - metrics.descent + metrics.leading).max(font_size_px);
    let height_px = raw_height.ceil().max(1.0) as u32;
    (ascent, height_px)
}

fn paint_line(
    viewport_buffer: &mut [u32],
    viewport_width: u32,
    viewport_height: u32,
    x_left: i32,
    y_top: i32,
    line_index: usize,
    text: &str,
    runtime: &mut EditorTextRuntime,
    style: &ViewportPaintStyle,
) -> LineLayout {
    let (ascent, line_height) = monospace_ascent_and_height(runtime, style.font_size_px);
    let baseline_x = x_left as f32;
    let baseline_y = (y_top as f32) + ascent;

    let shaped = runtime.shaper.shape_line(
        &mut runtime.font_system,
        text,
        style.font_size_px,
        &runtime.fallback,
        runtime.features,
    );

    let grapheme_offsets = grapheme_boundary_offsets(text);
    let mut grapheme_x_px = Vec::with_capacity(grapheme_offsets.len());
    for offset in &grapheme_offsets {
        grapheme_x_px.push(caret_x_for_byte_offset(&shaped.glyphs, *offset).round().max(0.0) as u32);
    }

    let px_size_q8 = ((style.font_size_px.max(0.01) * 256.0).round() as u32).max(1);
    let scale_bucket = 16u8;

    for glyph in shaped.glyphs {
        let entry = runtime.atlas.get_or_rasterize(
            &mut runtime.font_system,
            GlyphKey {
                glyph_id: glyph.glyph_id,
                font_id: glyph.font_id,
                px_size_q8,
                subpixel_variant: 0,
                scale_bucket,
            },
        );
        let Some(entry) = entry else {
            continue;
        };
        let placement = entry.image.placement;
        if placement.width == 0 || placement.height == 0 {
            continue;
        }

        let glyph_x = baseline_x + glyph.x;
        let glyph_y = baseline_y + glyph.y;
        let dst_x = (glyph_x + placement.left as f32).round() as i32;
        let dst_y = (glyph_y - placement.top as f32).round() as i32;

        let expected_mask = (placement.width as usize).saturating_mul(placement.height as usize);
        if entry.image.data.len() == expected_mask {
            blend_alpha_mask(
                viewport_buffer,
                viewport_width,
                viewport_height,
                dst_x,
                dst_y,
                placement.width,
                placement.height,
                &entry.image.data,
                style.text,
            );
        } else if entry.image.data.len() == expected_mask.saturating_mul(4) {
            blend_rgba_image(
                viewport_buffer,
                viewport_width,
                viewport_height,
                dst_x,
                dst_y,
                placement.width,
                placement.height,
                &entry.image.data,
            );
        }
    }

    let y_top_px = y_top;
    let _ = line_height;
    LineLayout {
        line_index,
        y_top_px,
        grapheme_x_px,
    }
}

fn grapheme_boundary_offsets(text: &str) -> Vec<usize> {
    let mut offsets = Vec::new();
    for (byte, _) in text.grapheme_indices(true) {
        offsets.push(byte);
    }
    offsets.push(text.len());
    offsets
}

fn caret_x_for_byte_offset(glyphs: &[aureline_text::shaping::ShapedGlyph], byte_offset: usize) -> f32 {
    if byte_offset == 0 {
        return 0.0;
    }
    let mut best: f32 = 0.0;
    for glyph in glyphs {
        if glyph.cluster == byte_offset {
            return glyph.x.max(0.0);
        }
        if glyph.cluster < byte_offset {
            best = best.max(glyph.x + glyph.advance);
        }
    }
    best.max(0.0)
}

fn copy_text_layer(
    window_buffer: &mut [u32],
    window_width: u32,
    window_height: u32,
    text_layer: &[u32],
    text_layer_width: u32,
    text_layer_height: u32,
    viewport_rect: PixelRect,
    clip_rect: PixelRect,
) {
    if window_width == 0
        || window_height == 0
        || text_layer_width == 0
        || text_layer_height == 0
        || viewport_rect.is_empty()
        || clip_rect.is_empty()
    {
        return;
    }
    let window_stride = window_width as usize;
    let layer_stride = text_layer_width as usize;

    for row in 0..clip_rect.height {
        let y = clip_rect.y.saturating_add(row);
        if y >= window_height {
            break;
        }
        if y < viewport_rect.y {
            continue;
        }
        let layer_y = y.saturating_sub(viewport_rect.y);
        if layer_y >= text_layer_height {
            continue;
        }

        let dst_y = y as usize;
        let dst_row = dst_y.saturating_mul(window_stride);
        let src_row = (layer_y as usize).saturating_mul(layer_stride);

        let start_x = clip_rect.x.max(viewport_rect.x);
        let end_x = clip_rect
            .right()
            .min(viewport_rect.right())
            .min(window_width);
        if end_x <= start_x {
            continue;
        }
        let dst_start = dst_row.saturating_add(start_x as usize);
        let dst_end = dst_row.saturating_add(end_x as usize);

        let layer_start_x = start_x.saturating_sub(viewport_rect.x);
        let src_start = src_row.saturating_add(layer_start_x as usize);
        let src_end = src_start.saturating_add((end_x - start_x) as usize);

        if dst_end <= window_buffer.len() && src_end <= text_layer.len() {
            window_buffer[dst_start..dst_end].copy_from_slice(&text_layer[src_start..src_end]);
        }
    }
}

fn paint_overlays(
    window_buffer: &mut [u32],
    window_width: u32,
    window_height: u32,
    viewport_rect: PixelRect,
    clip_rect: PixelRect,
    viewport: &EditorViewport,
    style: &ViewportPaintStyle,
) {
    let Some((sel_start, sel_end)) = viewport.selection_range() else {
        paint_caret(
            window_buffer,
            window_width,
            window_height,
            viewport_rect,
            clip_rect,
            viewport,
            style,
        );
        return;
    };

    let layout = viewport.layout();
    for line in sel_start.line..=sel_end.line {
        let Some(line_layout) = layout.line(line) else {
            continue;
        };
        let y_top = viewport_rect.y as i32 + line_layout.y_top_px;
        if y_top >= viewport_rect.bottom() as i32 {
            continue;
        }

        let line_height = layout.line_height_px.max(1);
        let y0 = y_top.max(viewport_rect.y as i32);
        let y1 = (y_top + line_height as i32).min(viewport_rect.bottom() as i32);
        if y1 <= y0 {
            continue;
        }

        let start_col = if line == sel_start.line { sel_start.grapheme } else { 0 };
        let end_col = if line == sel_end.line { sel_end.grapheme } else { usize::MAX };

        let x_positions = &line_layout.grapheme_x_px;
        if x_positions.is_empty() {
            continue;
        }
        let max_col = x_positions.len().saturating_sub(1);
        let start_col = start_col.min(max_col);
        let end_col = end_col.min(max_col);

        let x0 = viewport_rect.x as i32 + x_positions[start_col] as i32;
        let x1 = viewport_rect.x as i32 + x_positions[end_col] as i32;
        let (x0, x1) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };

        fill_rect_alpha_clipped(
            window_buffer,
            window_width,
            window_height,
            PixelRect::new(
                x0.max(0) as u32,
                y0.max(0) as u32,
                (x1 - x0).max(0) as u32,
                (y1 - y0).max(0) as u32,
            ),
            style.selection_fill,
            clip_rect,
        );
    }

    paint_caret(
        window_buffer,
        window_width,
        window_height,
        viewport_rect,
        clip_rect,
        viewport,
        style,
    );
}

fn paint_caret(
    window_buffer: &mut [u32],
    window_width: u32,
    window_height: u32,
    viewport_rect: PixelRect,
    clip_rect: PixelRect,
    viewport: &EditorViewport,
    style: &ViewportPaintStyle,
) {
    let caret = viewport.caret();
    let layout = viewport.layout();
    let Some(line_layout) = layout.line(caret.line) else {
        return;
    };
    let x_positions = &line_layout.grapheme_x_px;
    if x_positions.is_empty() {
        return;
    }
    let col = caret.grapheme.min(x_positions.len().saturating_sub(1));
    let caret_x = viewport_rect.x.saturating_add(x_positions[col]);
    let caret_y_top = viewport_rect.y as i32 + line_layout.y_top_px;
    let line_height = layout.line_height_px.max(1);

    let rect = PixelRect::new(
        caret_x,
        caret_y_top.max(0) as u32,
        2,
        line_height,
    );
    fill_rect_alpha_clipped(
        window_buffer,
        window_width,
        window_height,
        rect,
        style.caret,
        clip_rect,
    );
}

fn fill_rect_alpha_clipped(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    rect: PixelRect,
    color: ColorRgba,
    clip: PixelRect,
) {
    let Some(target) = rect.intersection(clip) else {
        return;
    };
    if target.is_empty() || width == 0 || height == 0 {
        return;
    }
    let stride = width as usize;
    let x0 = target.x.min(width) as usize;
    let x1 = target.right().min(width) as usize;
    let y0 = target.y.min(height) as usize;
    let y1 = target.bottom().min(height) as usize;
    for y in y0..y1 {
        let row = y.saturating_mul(stride);
        for x in x0..x1 {
            if let Some(px) = buffer.get_mut(row.saturating_add(x)) {
                *px = color.blend_over_u32(*px);
            }
        }
    }
}

fn blend_alpha_mask(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    dst_x: i32,
    dst_y: i32,
    mask_width: u32,
    mask_height: u32,
    mask: &[u8],
    color: ColorRgba,
) {
    if width == 0 || height == 0 || mask_width == 0 || mask_height == 0 {
        return;
    }
    let dst_width = width as usize;
    let dst_height = height as usize;
    let src_width = mask_width as usize;
    let src_height = mask_height as usize;

    let src_x0 = if dst_x < 0 { (-dst_x) as usize } else { 0 };
    let src_y0 = if dst_y < 0 { (-dst_y) as usize } else { 0 };
    if src_x0 >= src_width || src_y0 >= src_height {
        return;
    }

    let dst_x0 = if dst_x < 0 { 0 } else { dst_x as usize };
    let dst_y0 = if dst_y < 0 { 0 } else { dst_y as usize };
    if dst_x0 >= dst_width || dst_y0 >= dst_height {
        return;
    }

    let src_end_x = src_width.min(dst_width.saturating_sub(dst_x0).saturating_add(src_x0));
    let src_end_y = src_height.min(dst_height.saturating_sub(dst_y0).saturating_add(src_y0));

    let mut dy = dst_y0;
    for sy in src_y0..src_end_y {
        let dst_row = dy.saturating_mul(dst_width);
        dy = dy.saturating_add(1);
        let mut dx = dst_x0;
        for sx in src_x0..src_end_x {
            let idx = sy.saturating_mul(src_width).saturating_add(sx);
            let alpha = *mask.get(idx).unwrap_or(&0);
            if alpha == 0 {
                dx = dx.saturating_add(1);
                continue;
            }
            let tinted = ColorRgba {
                r: color.r,
                g: color.g,
                b: color.b,
                a: alpha,
            };
            if let Some(px) = buffer.get_mut(dst_row.saturating_add(dx)) {
                *px = tinted.blend_over_u32(*px);
            }
            dx = dx.saturating_add(1);
        }
    }
}

fn blend_rgba_image(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    dst_x: i32,
    dst_y: i32,
    image_width: u32,
    image_height: u32,
    image: &[u8],
) {
    if width == 0 || height == 0 || image_width == 0 || image_height == 0 {
        return;
    }

    let src_width = image_width as usize;
    let src_height = image_height as usize;
    let dst_width = width as usize;
    let dst_height = height as usize;

    let src_x0 = if dst_x < 0 { (-dst_x) as usize } else { 0 };
    let src_y0 = if dst_y < 0 { (-dst_y) as usize } else { 0 };
    if src_x0 >= src_width || src_y0 >= src_height {
        return;
    }

    let dst_x0 = if dst_x < 0 { 0 } else { dst_x as usize };
    let dst_y0 = if dst_y < 0 { 0 } else { dst_y as usize };
    if dst_x0 >= dst_width || dst_y0 >= dst_height {
        return;
    }

    let src_end_x = src_width.min(dst_width.saturating_sub(dst_x0).saturating_add(src_x0));
    let src_end_y = src_height.min(dst_height.saturating_sub(dst_y0).saturating_add(src_y0));

    let mut dy = dst_y0;
    for sy in src_y0..src_end_y {
        let dst_row = dy.saturating_mul(dst_width);
        dy = dy.saturating_add(1);
        let mut dx = dst_x0;
        for sx in src_x0..src_end_x {
            let base = (sy.saturating_mul(src_width).saturating_add(sx)).saturating_mul(4);
            let Some(chunk) = image.get(base..base.saturating_add(4)) else {
                dx = dx.saturating_add(1);
                continue;
            };
            let color = ColorRgba {
                r: chunk[0],
                g: chunk[1],
                b: chunk[2],
                a: chunk[3],
            };
            if color.a == 0 {
                dx = dx.saturating_add(1);
                continue;
            }
            if let Some(px) = buffer.get_mut(dst_row.saturating_add(dx)) {
                *px = color.blend_over_u32(*px);
            }
            dx = dx.saturating_add(1);
        }
    }
}
