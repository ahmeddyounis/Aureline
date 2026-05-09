//! Display-topology and mixed-DPI safety guards for native windows.
//!
//! The shell's windowing layer must not strand windows off-screen when monitor
//! topology drifts (dock/undock, display detach, mixed-DPI moves). This module
//! provides best-effort clamping for window bounds plus lightweight diagnostics
//! emitted when topology drift is detected or an adjustment is applied.
//!
//! The guard is intended to be instantiated per top-level window. Multi-window
//! shells should keep one [`DisplaySafetyGuard`] per `winit` window so topology
//! drift and last-known geometry are tracked independently.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::monitor::MonitorHandle;
use winit::window::Window;

const TOPOLOGY_POLL_INTERVAL: Duration = Duration::from_millis(750);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct PhysicalRect {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl PhysicalRect {
    pub(crate) const fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub(crate) fn from_position_size(
        position: PhysicalPosition<i32>,
        size: PhysicalSize<u32>,
    ) -> Self {
        Self::new(position.x, position.y, size.width, size.height)
    }

    fn right(self) -> i64 {
        self.x as i64 + self.width as i64
    }

    fn bottom(self) -> i64 {
        self.y as i64 + self.height as i64
    }

    fn intersection_area(self, other: Self) -> u64 {
        let x0 = (self.x as i64).max(other.x as i64);
        let y0 = (self.y as i64).max(other.y as i64);
        let x1 = self.right().min(other.right());
        let y1 = self.bottom().min(other.bottom());
        if x0 >= x1 || y0 >= y1 {
            return 0;
        }
        ((x1 - x0) as u64).saturating_mul((y1 - y0) as u64)
    }

    fn contains_point(self, point: PhysicalPosition<i32>) -> bool {
        let x = point.x as i64;
        let y = point.y as i64;
        x >= self.x as i64 && x < self.right() && y >= self.y as i64 && y < self.bottom()
    }

    fn center_anchor(self, child: Self) -> PhysicalPosition<i32> {
        let safe_w = self.width as i64;
        let safe_h = self.height as i64;
        let child_w = child.width as i64;
        let child_h = child.height as i64;

        let dx = (safe_w.saturating_sub(child_w) / 2).max(0);
        let dy = (safe_h.saturating_sub(child_h) / 2).max(0);
        PhysicalPosition::new(
            self.x.saturating_add(dx as i32),
            self.y.saturating_add(dy as i32),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DisplayFingerprint {
    fingerprint: u64,
    bounds: PhysicalRect,
}

impl DisplayFingerprint {
    fn from_monitor(monitor: &MonitorHandle) -> Self {
        let position = monitor.position();
        let size = monitor.size();
        let bounds = PhysicalRect::from_position_size(position, size);
        let name = monitor.name().unwrap_or_else(|| "unknown".to_string());
        let scale_bucket = scale_bucket_token(monitor.scale_factor());

        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        position.x.hash(&mut hasher);
        position.y.hash(&mut hasher);
        size.width.hash(&mut hasher);
        size.height.hash(&mut hasher);
        scale_bucket.hash(&mut hasher);

        Self {
            fingerprint: hasher.finish(),
            bounds,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DisplaySafetyAdjustmentRecord {
    record_kind: String,
    schema_version: u32,
    generated_at: String,
    window_ref: String,
    topology_change_classes: Vec<String>,
    scale_factor: f64,
    scale_bucket: String,
    safe_display_fingerprint: u64,
    window_bounds_before: PhysicalRect,
    window_bounds_after: PhysicalRect,
    safe_bounds: PhysicalRect,
    adjustment_classes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct DisplaySafetyTopologyRecord {
    record_kind: String,
    schema_version: u32,
    generated_at: String,
    window_ref: String,
    topology_change_classes: Vec<String>,
    scale_factor: f64,
    scale_bucket: String,
    window_bounds: Option<PhysicalRect>,
    display_fingerprints: Vec<u64>,
}

#[derive(Debug, Default)]
pub(crate) struct DisplaySafetyGuard {
    last_display_fingerprints: Option<Vec<u64>>,
    last_scale_bucket: Option<&'static str>,
    last_scale_factor_bits: Option<u64>,
    last_window_bounds: Option<PhysicalRect>,
    last_poll_at: Option<Instant>,
}

#[derive(Debug, Clone)]
pub(crate) struct DisplaySafetyOutcome {
    pub(crate) topology_change_classes: Vec<&'static str>,
    pub(crate) adjustment: Option<AppliedDisplaySafetyAdjustment>,
}

#[derive(Debug, Clone)]
pub(crate) struct AppliedDisplaySafetyAdjustment {
    pub(crate) window_bounds_before: PhysicalRect,
    pub(crate) window_bounds_after: PhysicalRect,
    pub(crate) safe_bounds: PhysicalRect,
    pub(crate) safe_display_fingerprint: u64,
    pub(crate) adjustment_classes: Vec<&'static str>,
    pub(crate) topology_change_classes: Vec<&'static str>,
}

impl DisplaySafetyGuard {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn poll_and_apply(&mut self, window: &Window) -> DisplaySafetyOutcome {
        let now = Instant::now();
        if let Some(prev_poll) = self.last_poll_at {
            if now.saturating_duration_since(prev_poll) < TOPOLOGY_POLL_INTERVAL {
                return DisplaySafetyOutcome {
                    topology_change_classes: Vec::new(),
                    adjustment: None,
                };
            }
        }
        self.last_poll_at = Some(now);

        let mut topology_change_classes: Vec<&'static str> = Vec::new();
        let displays = collect_displays(window);

        let display_fingerprints = displays.iter().map(|d| d.fingerprint).collect::<Vec<_>>();

        if let Some(prev) = self.last_display_fingerprints.as_ref() {
            if display_fingerprints.len() > prev.len() {
                topology_change_classes.push("display_added");
            } else if display_fingerprints.len() < prev.len() {
                topology_change_classes.push("display_removed");
            }

            if display_fingerprints != *prev {
                topology_change_classes.push("safe_bounds_changed");
            }
        }

        let scale_factor = window.scale_factor();
        let scale_bucket = scale_bucket_token(scale_factor);
        if let Some(prev) = self.last_scale_bucket {
            if prev != scale_bucket {
                topology_change_classes.push("scale_changed");
            }
        }

        let scale_factor_bits = scale_factor.to_bits();
        if let Some(prev_bits) = self.last_scale_factor_bits {
            if prev_bits != scale_factor_bits && !topology_change_classes.contains(&"scale_changed")
            {
                topology_change_classes.push("scale_changed");
            }
        }

        let Some(bounds) = window_bounds(window) else {
            if self.last_display_fingerprints.is_none() || self.last_window_bounds.is_some() {
                topology_change_classes.push("unsupported_display_topology");
            }
            self.last_display_fingerprints = Some(display_fingerprints);
            self.last_scale_bucket = Some(scale_bucket);
            self.last_scale_factor_bits = Some(scale_factor_bits);
            return DisplaySafetyOutcome {
                topology_change_classes,
                adjustment: None,
            };
        };

        let adjustment = if displays.is_empty() {
            None
        } else {
            self.adjust_offscreen_window(window, bounds, &displays, &topology_change_classes)
        };

        self.last_display_fingerprints = Some(display_fingerprints);
        self.last_scale_bucket = Some(scale_bucket);
        self.last_scale_factor_bits = Some(scale_factor_bits);
        self.last_window_bounds = Some(bounds);

        DisplaySafetyOutcome {
            topology_change_classes,
            adjustment,
        }
    }

    fn adjust_offscreen_window(
        &mut self,
        window: &Window,
        bounds: PhysicalRect,
        displays: &[DisplayFingerprint],
        topology_change_classes: &[&'static str],
    ) -> Option<AppliedDisplaySafetyAdjustment> {
        let allow_recenter =
            self.last_window_bounds.is_none() || !topology_change_classes.is_empty();
        if bounds.width == 0 || bounds.height == 0 {
            return None;
        }

        let max_intersection = displays
            .iter()
            .map(|display| bounds.intersection_area(display.bounds))
            .max()
            .unwrap_or(0);
        let offscreen = max_intersection == 0;

        if !offscreen {
            return None;
        }
        if !allow_recenter {
            return None;
        }

        let target_display = window
            .current_monitor()
            .map(|m| DisplayFingerprint::from_monitor(&m))
            .or_else(|| {
                window
                    .primary_monitor()
                    .map(|m| DisplayFingerprint::from_monitor(&m))
            })
            .or_else(|| displays.first().cloned())?;

        let target_bounds = target_display.bounds;
        let safe_display_fingerprint = target_display.fingerprint;

        let target_anchor = target_bounds.center_anchor(bounds);
        if target_bounds.contains_point(target_anchor) {
            window.set_outer_position(target_anchor);
        } else {
            window.set_outer_position(PhysicalPosition::new(target_bounds.x, target_bounds.y));
        }

        let after = window_bounds(window).unwrap_or(bounds);
        if after == bounds {
            return None;
        }

        let adjustment_classes = vec!["snapped_to_safe_bounds", "native_chrome_reprojected"];

        Some(AppliedDisplaySafetyAdjustment {
            window_bounds_before: bounds,
            window_bounds_after: after,
            safe_bounds: target_bounds,
            safe_display_fingerprint,
            adjustment_classes,
            topology_change_classes: topology_change_classes.to_vec(),
        })
    }
}

pub(crate) fn write_display_safety_log(record: &DisplaySafetyAdjustmentRecord) {
    let root = std::path::PathBuf::from(".logs").join("window_display_safety");
    if std::fs::create_dir_all(&root).is_err() {
        return;
    }

    let filename = format!(
        "{}.{}.window_display_safety.json",
        sanitize_filename(&record.window_ref),
        sanitize_filename(&record.generated_at)
    );
    let Ok(json) = serde_json::to_string_pretty(record) else {
        return;
    };
    let _ = std::fs::write(root.join(filename), json);
}

pub(crate) fn write_display_safety_topology_log(record: &DisplaySafetyTopologyRecord) {
    let root = std::path::PathBuf::from(".logs").join("window_display_safety");
    if std::fs::create_dir_all(&root).is_err() {
        return;
    }

    let filename = format!(
        "{}.{}.window_display_safety.json",
        sanitize_filename(&record.window_ref),
        sanitize_filename(&record.generated_at)
    );
    let Ok(json) = serde_json::to_string_pretty(record) else {
        return;
    };
    let _ = std::fs::write(root.join(filename), json);
}

pub(crate) fn materialize_adjustment_record(
    window: &Window,
    adjustment: &AppliedDisplaySafetyAdjustment,
) -> DisplaySafetyAdjustmentRecord {
    DisplaySafetyAdjustmentRecord {
        record_kind: "window_display_safety_record".to_string(),
        schema_version: 1,
        generated_at: aureline_commands::invocation::now_rfc3339(),
        window_ref: format!("{:?}", window.id()),
        topology_change_classes: adjustment
            .topology_change_classes
            .iter()
            .map(|c| (*c).to_string())
            .collect(),
        scale_factor: window.scale_factor(),
        scale_bucket: scale_bucket_token(window.scale_factor()).to_string(),
        safe_display_fingerprint: adjustment.safe_display_fingerprint,
        window_bounds_before: adjustment.window_bounds_before,
        window_bounds_after: adjustment.window_bounds_after,
        safe_bounds: adjustment.safe_bounds,
        adjustment_classes: adjustment
            .adjustment_classes
            .iter()
            .map(|c| (*c).to_string())
            .collect(),
    }
}

pub(crate) fn materialize_topology_record(
    window: &Window,
    topology_change_classes: &[&'static str],
    window_bounds: Option<PhysicalRect>,
) -> DisplaySafetyTopologyRecord {
    let display_fingerprints = collect_displays(window)
        .into_iter()
        .map(|display| display.fingerprint)
        .collect::<Vec<_>>();

    DisplaySafetyTopologyRecord {
        record_kind: "window_display_safety_topology_record".to_string(),
        schema_version: 1,
        generated_at: aureline_commands::invocation::now_rfc3339(),
        window_ref: format!("{:?}", window.id()),
        topology_change_classes: topology_change_classes
            .iter()
            .map(|c| (*c).to_string())
            .collect(),
        scale_factor: window.scale_factor(),
        scale_bucket: scale_bucket_token(window.scale_factor()).to_string(),
        window_bounds,
        display_fingerprints,
    }
}

fn collect_displays(window: &Window) -> Vec<DisplayFingerprint> {
    let mut displays = window
        .available_monitors()
        .map(|monitor| DisplayFingerprint::from_monitor(&monitor))
        .collect::<Vec<_>>();
    displays.sort_by_key(|d| {
        (
            d.bounds.x,
            d.bounds.y,
            d.bounds.width,
            d.bounds.height,
            d.fingerprint,
        )
    });
    displays
}

fn window_bounds(window: &Window) -> Option<PhysicalRect> {
    let position = window.outer_position().ok()?;
    let size = window.outer_size();
    Some(PhysicalRect::from_position_size(position, size))
}

fn sanitize_filename(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            ':' | '/' | '\\' | ' ' | '\t' | '\n' | '\r' => '_',
            other => other,
        })
        .collect()
}

fn scale_bucket_token(scale_factor: f64) -> &'static str {
    if !scale_factor.is_finite() || scale_factor <= 0.0 {
        return "other";
    }
    let approx = |target: f64| (scale_factor - target).abs() <= 0.08;
    if approx(1.0) {
        "1x"
    } else if approx(1.25) {
        "1_25x"
    } else if approx(1.5) {
        "1_5x"
    } else if approx(2.0) {
        "2x"
    } else {
        "other"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::Path;

    #[derive(Debug, Deserialize)]
    struct TopologyFixture {
        displays: Vec<FixtureDisplay>,
        window: PhysicalRect,
        expected: ExpectedOutcome,
    }

    #[derive(Debug, Deserialize)]
    struct FixtureDisplay {
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        #[serde(default)]
        scale_factor: Option<f64>,
        #[serde(default)]
        primary: bool,
    }

    #[derive(Debug, Deserialize)]
    struct ExpectedOutcome {
        offscreen: bool,
        anchor_within_safe_bounds: bool,
    }

    fn load_fixture(path: &Path) -> String {
        std::fs::read_to_string(path).expect("fixture must read")
    }

    #[test]
    fn recenter_logic_is_stable_across_fixture_cases() {
        let root =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/windowing/topology_cases");
        let entries =
            std::fs::read_dir(&root).unwrap_or_else(|err| panic!("fixture root must exist: {err}"));

        for entry in entries {
            let entry = entry.expect("fixture entry must read");
            if entry.file_type().expect("fixture file type").is_dir() {
                continue;
            }
            if entry.path().extension().and_then(|s| s.to_str()) != Some("json") {
                continue;
            }

            let json = load_fixture(&entry.path());
            let fixture: TopologyFixture = serde_json::from_str(&json)
                .unwrap_or_else(|err| panic!("fixture must parse: {err}"));

            let displays = fixture
                .displays
                .iter()
                .map(|display| DisplayFingerprint {
                    fingerprint: 0,
                    bounds: PhysicalRect::new(display.x, display.y, display.width, display.height),
                })
                .collect::<Vec<_>>();

            let max_intersection = displays
                .iter()
                .map(|display| fixture.window.intersection_area(display.bounds))
                .max()
                .unwrap_or(0);
            let offscreen = max_intersection == 0;
            assert_eq!(
                offscreen,
                fixture.expected.offscreen,
                "fixture offscreen state mismatch for {:?}",
                entry.path()
            );

            if displays.is_empty() {
                continue;
            }
            let safe = displays
                .iter()
                .find(|display| display.bounds.contains_point(PhysicalPosition::new(0, 0)))
                .map(|display| display.bounds)
                .unwrap_or(displays[0].bounds);
            let anchor = safe.center_anchor(fixture.window);
            assert_eq!(
                safe.contains_point(anchor),
                fixture.expected.anchor_within_safe_bounds,
                "fixture anchor mismatch for {:?}",
                entry.path()
            );
        }
    }
}
