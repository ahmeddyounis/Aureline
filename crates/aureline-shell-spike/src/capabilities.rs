//! Machine-readable capability manifest.
//!
//! Written once per spike run. Later benchmark and conformance tasks
//! consume it by name to pick up the hook vocabulary, zone layout,
//! and backend posture without having to re-derive them.

use crate::hooks::Hook;
use crate::render_path::SurfaceOwnership;
use crate::zones::{ShellFrame, ZoneId};
use crate::SpikeBuildIdentity;

use std::fmt::Write as _;

/// The backend a spike run was built against. The binary picks one at
/// startup and records it here so traces can be grouped by backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// No native window; the run emitted traces and exited. This is the
    /// default for environments without a display (CI, sandboxed hosts).
    Headless,
    /// A native window backed by a future `winit`+`wgpu` integration.
    /// The spike declares this variant so capability consumers can
    /// round-trip without schema churn when the backend lands.
    NativeWindow,
}

impl Backend {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Headless => "headless",
            Self::NativeWindow => "native_window",
        }
    }
}

/// The full manifest written to `artifacts/render/spike_capabilities.json`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityManifest {
    pub schema_version: u32,
    pub scene_id: &'static str,
    pub build: SpikeBuildIdentity,
    pub backend: Backend,
    pub host_os: &'static str,
    pub frame: ShellFrame,
}

impl CapabilityManifest {
    pub const SCHEMA_VERSION: u32 = 1;

    pub fn new(backend: Backend, frame: ShellFrame, scene_id: &'static str) -> Self {
        Self {
            schema_version: Self::SCHEMA_VERSION,
            scene_id,
            build: SpikeBuildIdentity::current(),
            backend,
            host_os: host_os(),
            frame,
        }
    }

    /// Render as deterministic pretty JSON. Keys are emitted in the
    /// order declared here; fixed order is what lets trace diffs on
    /// disk stay readable.
    pub fn to_json(&self) -> String {
        let mut out = String::new();
        out.push_str("{\n");
        writeln_kv(&mut out, 1, "schema_version", &self.schema_version.to_string(), false);
        writeln_kv(&mut out, 1, "scene_id", &quote(self.scene_id), false);

        // build block
        writeln_key(&mut out, 1, "build");
        out.push_str(" {\n");
        writeln_kv(&mut out, 2, "crate_name", &quote(self.build.crate_name), false);
        writeln_kv(&mut out, 2, "crate_version", &quote(self.build.crate_version), false);
        writeln_kv(&mut out, 2, "rustc_target_triple", &quote(self.build.rustc_target_triple), true);
        indent(&mut out, 1);
        out.push_str("},\n");

        writeln_kv(&mut out, 1, "backend", &quote(self.backend.name()), false);
        writeln_kv(&mut out, 1, "host_os", &quote(self.host_os), false);

        // zones array
        writeln_key(&mut out, 1, "zones");
        out.push_str(" [\n");
        let zones = [
            ZoneId::TitleBar,
            ZoneId::Sidebar,
            ZoneId::EditorViewport,
            ZoneId::StatusBar,
        ];
        for (i, zone) in zones.iter().enumerate() {
            let rect = self.frame.zone(*zone);
            let last = i + 1 == zones.len();
            indent(&mut out, 2);
            out.push_str("{\n");
            writeln_kv(&mut out, 3, "id", &quote(zone.name()), false);
            writeln_kv(&mut out, 3, "z_order", &zone.z_order().to_string(), false);
            writeln_kv(
                &mut out,
                3,
                "surface_ownership",
                &quote(SurfaceOwnership::for_zone(*zone).name()),
                false,
            );
            writeln_key(&mut out, 3, "rect");
            out.push_str(" {\n");
            writeln_kv(&mut out, 4, "x", &rect.x.to_string(), false);
            writeln_kv(&mut out, 4, "y", &rect.y.to_string(), false);
            writeln_kv(&mut out, 4, "width", &rect.width.to_string(), false);
            writeln_kv(&mut out, 4, "height", &rect.height.to_string(), true);
            indent(&mut out, 3);
            out.push_str("}\n");
            indent(&mut out, 2);
            if last {
                out.push_str("}\n");
            } else {
                out.push_str("},\n");
            }
        }
        indent(&mut out, 1);
        out.push_str("],\n");

        // hooks array
        writeln_key(&mut out, 1, "hooks");
        out.push_str(" [\n");
        for (i, hook) in Hook::ALL.iter().enumerate() {
            let last = i + 1 == Hook::ALL.len();
            indent(&mut out, 2);
            out.push_str("{\n");
            writeln_kv(&mut out, 3, "name", &quote(hook.name()), false);
            writeln_kv(
                &mut out,
                3,
                "hot_path",
                if hook.is_hot_path() { "true" } else { "false" },
                true,
            );
            indent(&mut out, 2);
            if last {
                out.push_str("}\n");
            } else {
                out.push_str("},\n");
            }
        }
        indent(&mut out, 1);
        out.push_str("]\n");

        out.push_str("}\n");
        out
    }
}

fn host_os() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        "macos"
    }
    #[cfg(target_os = "linux")]
    {
        "linux"
    }
    #[cfg(target_os = "windows")]
    {
        "windows"
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        "unknown"
    }
}

fn indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

fn writeln_key(out: &mut String, depth: usize, key: &str) {
    indent(out, depth);
    out.push('"');
    out.push_str(key);
    out.push_str("\":");
}

fn writeln_kv(out: &mut String, depth: usize, key: &str, value: &str, last: bool) {
    indent(out, depth);
    let _ = write!(out, "\"{key}\": {value}");
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

/// JSON-quote a string. The spike only emits ASCII and pre-validated
/// labels, so the escape set is narrow.
pub fn quote(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_contains_every_hook_name() {
        let manifest = CapabilityManifest::new(
            Backend::Headless,
            ShellFrame::fixture(),
            "shell_spike.fixture_v1",
        );
        let json = manifest.to_json();
        for hook in Hook::ALL {
            assert!(
                json.contains(&format!("\"{}\"", hook.name())),
                "manifest missing hook {}",
                hook.name()
            );
        }
    }

    #[test]
    fn manifest_names_every_zone() {
        let manifest = CapabilityManifest::new(
            Backend::Headless,
            ShellFrame::fixture(),
            "shell_spike.fixture_v1",
        );
        let json = manifest.to_json();
        for zone in ZoneId::ALL {
            assert!(json.contains(&format!("\"{}\"", zone.name())));
        }
    }

    #[test]
    fn quote_escapes_control_characters() {
        assert_eq!(quote("a\tb"), "\"a\\tb\"");
        assert_eq!(quote("a\"b"), "\"a\\\"b\"");
    }
}
