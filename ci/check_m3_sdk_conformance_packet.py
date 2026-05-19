#!/usr/bin/env python3
"""Verify the canonical SDK conformance packet and bridge scorecard.

The check re-runs the generator at
``tools/extensions/m3/sdk_conformance_packet/aureline_sdk_conformance_packet.py``
against the canonical beta-line fixture with ``--check`` and asserts
that the committed artifacts under ``artifacts/extensions/m3/`` match
the generator output byte-for-byte. Drift between the SDK contract,
sample-pack outcome, lifecycle metadata, docs-freshness sweep, or
bridge matrix and the committed packet fails CI.

The script also smoke-tests each refusal fixture so the typed
refusal paths cannot quietly stop firing.
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path


GENERATOR_REL = (
    "tools/extensions/m3/sdk_conformance_packet/aureline_sdk_conformance_packet.py"
)
CANONICAL_FIXTURE_REL = (
    "fixtures/extensions/m3/sdk_conformance_packet/ready_for_authors_beta_line.json"
)
PACKET_JSON_REL = "artifacts/extensions/m3/sdk_conformance_packet.json"
PACKET_MD_REL = "artifacts/extensions/m3/sdk_conformance_packet.md"
SCORECARD_JSON_REL = "artifacts/extensions/m3/bridge_compatibility_scorecard.json"

REFUSAL_FIXTURES = {
    "fixtures/extensions/m3/sdk_conformance_packet/refused_docs_freshness_drift.json": (
        "refused_inconsistent_input",
        "docs_freshness_drift_detected",
    ),
    "fixtures/extensions/m3/sdk_conformance_packet/refused_bridge_matrix_missing_required_state.json": (
        "refused_inconsistent_input",
        "bridge_matrix_missing_required_state",
    ),
}

PARTIAL_FIXTURE_REL = (
    "fixtures/extensions/m3/sdk_conformance_packet/partially_ready_preview_beta_line.json"
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    return parser.parse_args()


def run_generator(
    *,
    repo_root: Path,
    fixture_rel: str,
    extra_args: list[str],
) -> subprocess.CompletedProcess[str]:
    cmd = [
        sys.executable,
        str((repo_root / GENERATOR_REL).resolve()),
        "--repo-root",
        str(repo_root),
        "generate",
        "--fixture",
        str((repo_root / fixture_rel).resolve()),
        *extra_args,
    ]
    return subprocess.run(cmd, capture_output=True, text=True)


def assert_canonical_artifacts_fresh(repo_root: Path) -> int:
    result = run_generator(
        repo_root=repo_root,
        fixture_rel=CANONICAL_FIXTURE_REL,
        extra_args=[
            "--packet-json",
            str((repo_root / PACKET_JSON_REL).resolve()),
            "--packet-md",
            str((repo_root / PACKET_MD_REL).resolve()),
            "--scorecard-json",
            str((repo_root / SCORECARD_JSON_REL).resolve()),
            "--check",
        ],
    )
    if result.returncode != 0:
        sys.stdout.write(result.stdout)
        sys.stderr.write(result.stderr)
        print(
            "[sdk-conformance-packet] canonical artifacts drift detected; "
            "rerun the generator without --check to refresh.",
            file=sys.stderr,
        )
        return 1
    return 0


def assert_fixture_outcome(
    repo_root: Path,
    *,
    fixture_rel: str,
    expected_decision: str,
    expected_reason: str,
) -> int:
    result = run_generator(
        repo_root=repo_root,
        fixture_rel=fixture_rel,
        extra_args=[],
    )
    try:
        payload = json.loads(result.stdout)
    except json.JSONDecodeError:
        sys.stderr.write(result.stderr)
        print(
            f"[sdk-conformance-packet] generator emitted non-JSON output for {fixture_rel}",
            file=sys.stderr,
        )
        return 1
    observed_decision = payload.get("decision_class")
    observed_reason = payload.get("reason_class")
    if observed_decision != expected_decision or observed_reason != expected_reason:
        print(
            f"[sdk-conformance-packet] fixture {fixture_rel} expected "
            f"{expected_decision}/{expected_reason} but observed "
            f"{observed_decision}/{observed_reason}",
            file=sys.stderr,
        )
        return 1
    return 0


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    exit_code = assert_canonical_artifacts_fresh(repo_root)
    exit_code |= assert_fixture_outcome(
        repo_root,
        fixture_rel=PARTIAL_FIXTURE_REL,
        expected_decision="partially_ready_preview_surfaces_only",
        expected_reason="some_claimed_surfaces_preview_in_beta",
    )
    for fixture_rel, (expected_decision, expected_reason) in REFUSAL_FIXTURES.items():
        exit_code |= assert_fixture_outcome(
            repo_root,
            fixture_rel=fixture_rel,
            expected_decision=expected_decision,
            expected_reason=expected_reason,
        )
    if exit_code == 0:
        print("[sdk-conformance-packet] OK")
    return exit_code


if __name__ == "__main__":
    raise SystemExit(main())
