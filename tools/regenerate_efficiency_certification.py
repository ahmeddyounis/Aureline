#!/usr/bin/env python3
"""Regenerate the M5 efficiency certification proof packet.

The canonical proof packet is produced by the shell efficiency certification lane
and dumped by the conformance example, so the checked-in evidence can never
disagree with what ships. This script runs that example and writes the proof
packet artifact. Run after editing the seeded certification subjects, the drill
logic, or the upstream energy/thermal and session-pressure evidence:

    python3 tools/regenerate_efficiency_certification.py

then re-run the gate and drift test:

    python3 ci/check_efficiency_certification.py --repo-root .
    cargo test -p aureline-shell efficiency_certification
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
PROOF_PACKET_REL = "artifacts/efficiency/m5-efficiency-proof-packet.json"


def dump_packet() -> dict:
    """Runs the conformance example and returns the parsed proof packet."""
    result = subprocess.run(
        [
            "cargo",
            "run",
            "-p",
            "aureline-shell",
            "--example",
            "dump_efficiency_certification",
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
    packet = dump_packet()
    if not packet.get("rows"):
        print("no certification rows produced", file=sys.stderr)
        return 1
    write_json(PROOF_PACKET_REL, packet)
    print(
        f"{len(packet['rows'])} rows, "
        f"promotion: {packet['promotion_gate']['decision']}, "
        f"as_of: {packet['as_of']}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
