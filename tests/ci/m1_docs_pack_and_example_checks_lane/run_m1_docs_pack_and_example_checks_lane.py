#!/usr/bin/env python3
"""Unattended validation lane for the M1 stale-example detection gate.

This wrapper exercises ``tools/ci/check_stale_examples.py`` end-to-end:

1. A full pass over every protected docs-pack row in
   ``artifacts/ci/m1_stale_example_source_map.yaml`` (the capture
   under ``artifacts/milestones/m1/captures/`` is the durable record
   of pass / fail per pack).
2. A replay of every protected pack's named failure drill via
   ``--force-drill <pack_id>:<drill_id>``. The lane fails loudly if
   any drill silently passes — that would mean the gate has gone
   deaf to a real stale-example pattern.

Exit codes:

- ``0`` — the full pass is clean *and* every named failure drill
  reproduces its declared ``expected_check_id``.
- ``1`` — the full pass observed at least one drift, *or* a forced
  drill failed to reproduce its declared check.

The wrapper writes a durable lane capture in addition to the per-run
capture the validator already writes. Both captures land under
``artifacts/milestones/m1/captures/``.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from pathlib import Path
from typing import Any


DEFAULT_SOURCE_MAP_REL = "artifacts/ci/m1_stale_example_source_map.yaml"
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_VALIDATOR_REL = "tools/ci/check_stale_examples.py"
DEFAULT_FULL_PASS_CAPTURE_REL = (
    "artifacts/milestones/m1/captures/"
    "docs_pack_and_example_checks_validation_capture.json"
)
DEFAULT_LANE_CAPTURE_REL = (
    "artifacts/milestones/m1/captures/"
    "docs_pack_and_example_checks_lane_capture.json"
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--source-map", default=DEFAULT_SOURCE_MAP_REL)
    parser.add_argument("--validator", default=DEFAULT_VALIDATOR_REL)
    parser.add_argument(
        "--full-pass-capture",
        default=DEFAULT_FULL_PASS_CAPTURE_REL,
        help="Path the validator writes for the full pass run.",
    )
    parser.add_argument(
        "--lane-capture",
        default=DEFAULT_LANE_CAPTURE_REL,
        help="Path this wrapper writes for the full lane run.",
    )
    parser.add_argument("--build-identity", default=DEFAULT_BUILD_IDENTITY_REL)
    return parser.parse_args()


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def render_yaml_as_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-rdate",
            "-rtime",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), "
                "permitted_classes: [Time, Date, DateTime], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(
            f"failed to parse YAML at {path} via Ruby/Psych: {stderr}"
        )
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(
            f"Ruby/Psych emitted invalid JSON for {path}: {exc}"
        ) from exc


def run_validator(
    *,
    repo_root: Path,
    validator: str,
    source_map: str,
    report_rel: str,
    build_identity: str,
    force_drill: str | None,
) -> tuple[int, str, str]:
    cmd = [
        sys.executable,
        str(repo_root / validator),
        "--repo-root",
        str(repo_root),
        "--source-map",
        source_map,
        "--report",
        report_rel,
        "--build-identity",
        build_identity,
    ]
    if force_drill is not None:
        cmd.extend(["--force-drill", force_drill])
    proc = subprocess.run(cmd, capture_output=True, text=True)
    return proc.returncode, proc.stdout, proc.stderr


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    source_map_path = repo_root / args.source_map
    source_map = render_yaml_as_json(source_map_path)
    if not isinstance(source_map, dict):
        raise SystemExit(
            f"{args.source_map} must parse as a YAML mapping/object"
        )

    packs = source_map.get("protected_docs_packs") or []
    if not isinstance(packs, list) or not packs:
        raise SystemExit(
            f"{args.source_map} has no protected_docs_packs entries; the "
            "lane cannot run."
        )

    # 1. Full pass.
    full_pass_rc, full_pass_stdout, full_pass_stderr = run_validator(
        repo_root=repo_root,
        validator=args.validator,
        source_map=args.source_map,
        report_rel=args.full_pass_capture,
        build_identity=args.build_identity,
        force_drill=None,
    )
    full_pass_capture_path = repo_root / args.full_pass_capture
    full_pass_capture: dict[str, Any] = {}
    if full_pass_capture_path.exists():
        try:
            full_pass_capture = json.loads(
                full_pass_capture_path.read_text(encoding="utf-8")
            )
        except Exception:  # noqa: BLE001
            full_pass_capture = {}
    full_pass_status = full_pass_capture.get("status") or (
        "PASS" if full_pass_rc == 0 else "FAIL"
    )

    # 2. Per-pack failure-drill replay.
    drill_results: list[dict[str, Any]] = []
    drill_failures: list[dict[str, Any]] = []
    for pack in packs:
        if not isinstance(pack, dict):
            continue
        pack_id = pack.get("pack_id")
        drill = pack.get("failure_drill")
        if not isinstance(pack_id, str) or not pack_id:
            continue
        if not isinstance(drill, dict):
            drill_failures.append(
                {
                    "pack_id": pack_id,
                    "reason": "pack has no failure_drill block",
                }
            )
            continue
        drill_id = drill.get("drill_id")
        expected_check = drill.get("expected_check_id")
        if (
            not isinstance(drill_id, str)
            or not drill_id
            or not isinstance(expected_check, str)
            or not expected_check
        ):
            drill_failures.append(
                {
                    "pack_id": pack_id,
                    "reason": "failure_drill missing drill_id / expected_check_id",
                }
            )
            continue
        drill_capture_rel = (
            "artifacts/milestones/m1/captures/"
            f"docs_pack_and_example_checks_drill_capture_{pack_id}.json"
        )
        drill_rc, drill_stdout, drill_stderr = run_validator(
            repo_root=repo_root,
            validator=args.validator,
            source_map=args.source_map,
            report_rel=drill_capture_rel,
            build_identity=args.build_identity,
            force_drill=f"{pack_id}:{drill_id}",
        )
        drill_capture_path = repo_root / drill_capture_rel
        drill_capture: dict[str, Any] = {}
        if drill_capture_path.exists():
            try:
                drill_capture = json.loads(
                    drill_capture_path.read_text(encoding="utf-8")
                )
            except Exception:  # noqa: BLE001
                drill_capture = {}
        replay = drill_capture.get("forced_drill_replay") or {}
        reproduced = bool(replay.get("reproduced"))
        drill_results.append(
            {
                "pack_id": pack_id,
                "drill_id": drill_id,
                "expected_check_id": expected_check,
                "observed_failed_check_ids": replay.get(
                    "observed_failed_check_ids", []
                ),
                "reproduced": reproduced,
                "exit_code": drill_rc,
                "capture_ref": drill_capture_rel,
            }
        )
        if not reproduced or drill_rc != 0:
            drill_failures.append(
                {
                    "pack_id": pack_id,
                    "drill_id": drill_id,
                    "expected_check_id": expected_check,
                    "observed_failed_check_ids": replay.get(
                        "observed_failed_check_ids", []
                    ),
                    "exit_code": drill_rc,
                }
            )

    lane_status = "PASS"
    if full_pass_rc != 0:
        lane_status = "FAIL"
    if drill_failures:
        lane_status = "FAIL"

    lane_capture = {
        "schema_version": 1,
        "capture_kind": "docs_pack_and_example_checks_lane_capture",
        "captured_at": now_iso_z(),
        "owner_dri": source_map.get("owner_dri"),
        "source_map_ref": args.source_map,
        "validator_entrypoint": args.validator,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/ci/m1_docs_pack_and_example_checks_lane/"
            "run_m1_docs_pack_and_example_checks_lane.py --repo-root ."
        ),
        "status": lane_status,
        "full_pass": {
            "exit_code": full_pass_rc,
            "status": full_pass_status,
            "capture_ref": args.full_pass_capture,
        },
        "drills": drill_results,
        "drill_failures": drill_failures,
    }

    lane_capture_path = repo_root / args.lane_capture
    lane_capture_path.parent.mkdir(parents=True, exist_ok=True)
    lane_capture_path.write_text(
        json.dumps(lane_capture, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )

    label = "docs-pack-and-example-checks-lane"
    print(
        f"[{label}] {lane_status} — full_pass_status={full_pass_status} "
        f"drills_reproduced="
        f"{sum(1 for d in drill_results if d['reproduced'])}/"
        f"{len(drill_results)} — capture: {args.lane_capture}"
    )
    if full_pass_rc != 0:
        print(f"[{label}] full pass exited {full_pass_rc}")
        if full_pass_stdout.strip():
            for line in full_pass_stdout.strip().splitlines():
                print(f"[{label}] full-pass> {line}")
        if full_pass_stderr.strip():
            for line in full_pass_stderr.strip().splitlines():
                print(f"[{label}] full-pass! {line}")
    for drill in drill_results:
        marker = "OK" if drill["reproduced"] else "FAIL"
        print(
            f"[{label}] drill {marker} {drill['pack_id']}:{drill['drill_id']} "
            f"expected={drill['expected_check_id']} reproduced={drill['reproduced']}"
        )
    for failure in drill_failures:
        print(
            f"[{label}] drill failure {failure}"
        )

    return 0 if lane_status == "PASS" else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print(
            "[docs-pack-and-example-checks-lane] interrupted",
            file=sys.stderr,
        )
        sys.exit(130)
