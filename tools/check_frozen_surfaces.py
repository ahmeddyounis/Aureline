#!/usr/bin/env python3
"""Standalone entry point for frozen-surface manifest validation."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
CI_DIR = SCRIPT_DIR / "ci"
if str(CI_DIR) not in sys.path:
    sys.path.insert(0, str(CI_DIR))

from frozen_surface_validation import render_human_summary, validate_frozen_surface_manifest


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument(
        "--report",
        default=None,
        help="Write the machine-readable JSON report to this repo-relative path.",
    )
    parser.add_argument(
        "--scenario",
        default=None,
        help="Optional JSON scenario that overrides changed_files for a deterministic failing example.",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    scenario_path = None if args.scenario is None else Path(args.scenario)

    findings, analysis = validate_frozen_surface_manifest(repo_root, scenario_path)
    sys.stdout.write(render_human_summary(findings, analysis))

    if args.report:
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(json.dumps(analysis, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    return 1 if any(finding.severity == "error" for finding in findings) else 0


if __name__ == "__main__":
    raise SystemExit(main())
