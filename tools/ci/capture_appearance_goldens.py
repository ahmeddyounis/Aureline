#!/usr/bin/env python3
"""Capture appearance golden screenshots for protected shell surfaces.

This script is intended for developer-local runs. It launches the native shell,
captures a single screenshot after the first frame, and exits.
"""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]


@dataclass(frozen=True)
class CaptureCase:
    case_id: str
    theme_class: str
    density_class: str
    reduced_motion_posture: str

    def filename(self) -> str:
        parts = [
            "shell_chrome",
            self.theme_class,
            self.density_class,
            self.reduced_motion_posture,
        ]
        return ".".join(parts) + ".png"


CASES: tuple[CaptureCase, ...] = (
    CaptureCase("dark_standard", "dark_reference", "standard", "motion_standard"),
    CaptureCase("light_standard", "light_parity", "standard", "motion_standard"),
    CaptureCase("hc_dark_standard", "high_contrast_dark", "standard", "motion_standard"),
    CaptureCase("hc_light_standard", "high_contrast_light", "standard", "motion_standard"),
    CaptureCase("dark_compact", "dark_reference", "compact", "motion_standard"),
)


def exe_name() -> str:
    return "aureline_shell.exe" if os.name == "nt" else "aureline_shell"


def build_shell() -> Path:
    subprocess.run(
        ["cargo", "build", "-p", "aureline-shell", "--bin", "aureline_shell"],
        cwd=REPO_ROOT,
        check=True,
    )
    binary = REPO_ROOT / "target" / "debug" / exe_name()
    if not binary.exists():
        raise SystemExit(f"built binary missing: {binary}")
    return binary


def run_capture(binary: Path, *, out_path: Path, case: CaptureCase, window_size: str, renderer: str) -> None:
    cmd = [
        str(binary),
        "--emit-screenshot",
        str(out_path),
        "--window-size",
        window_size,
        "--renderer",
        renderer,
        "--theme-class",
        case.theme_class,
        "--density-class",
        case.density_class,
        "--reduced-motion-posture",
        case.reduced_motion_posture,
    ]
    subprocess.run(cmd, cwd=REPO_ROOT, check=True)


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description="Capture appearance golden screenshots.")
    parser.add_argument(
        "--out-dir",
        default="tests/golden/appearance/shell_chrome/baselines",
        help="Output directory for generated screenshots (repo-relative).",
    )
    parser.add_argument(
        "--window-size",
        default="1280x720",
        help="Logical window size passed to the shell (e.g. 1280x720).",
    )
    parser.add_argument(
        "--renderer",
        default="software",
        choices=("software", "gpu"),
        help="Render backend used for capture.",
    )
    args = parser.parse_args(argv)

    out_dir = (REPO_ROOT / args.out_dir).resolve()
    out_dir.mkdir(parents=True, exist_ok=True)

    binary = build_shell()

    for case in CASES:
        out_path = out_dir / case.filename()
        print(f"[appearance-goldens] capturing {case.case_id} -> {out_path.relative_to(REPO_ROOT)}")
        run_capture(
            binary,
            out_path=out_path,
            case=case,
            window_size=args.window_size,
            renderer=args.renderer,
        )

    print(f"[appearance-goldens] done: {out_dir.relative_to(REPO_ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))

