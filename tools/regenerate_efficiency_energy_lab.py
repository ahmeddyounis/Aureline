#!/usr/bin/env python3
"""Regenerate the energy/thermal efficiency-lab fixtures and trace artifacts.

The canonical lab cases are produced by the shell efficiency runtime and dumped
by the conformance example, so the checked-in evidence can never disagree with
what ships. This script runs that example, then writes one fixture per profile
(the full lab case) and one exported trace artifact per profile. Run after
editing the seeded profiles or the lab runtime:

    python3 tools/regenerate_efficiency_energy_lab.py

then re-run the drift test:

    cargo test -p aureline-shell efficiency_energy_lab
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
FIXTURE_DIR_REL = "fixtures/efficiency/lab"
TRACE_DIR_REL = "artifacts/efficiency/m5-efficiency-traces"


def dump_cases() -> list[dict]:
    """Runs the conformance example and returns the parsed lab cases."""
    result = subprocess.run(
        [
            "cargo",
            "run",
            "-p",
            "aureline-shell",
            "--example",
            "dump_efficiency_energy_lab",
            "--locked",
            "--quiet",
        ],
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
        check=True,
    )
    return json.loads(result.stdout)


def write_json(rel: str, payload) -> None:
    path = REPO_ROOT / rel
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(f"wrote {rel}")


def main() -> int:
    cases = dump_cases()
    if not cases:
        print("no lab cases produced", file=sys.stderr)
        return 1

    for case in cases:
        profile_id = case["profile"]["profile_id"]
        write_json(f"{FIXTURE_DIR_REL}/{profile_id}.json", case)
        write_json(f"{TRACE_DIR_REL}/{profile_id}.json", case["trace"])

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
