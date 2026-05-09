#!/usr/bin/env python3
"""Compare appearance golden screenshots against repo baselines."""

from __future__ import annotations

import argparse
import hashlib
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]


def sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def capture_into(out_dir: Path) -> None:
    subprocess.run(
        [
            "python3",
            str(REPO_ROOT / "tools/ci/capture_appearance_goldens.py"),
            "--out-dir",
            str(out_dir.relative_to(REPO_ROOT)),
        ],
        cwd=REPO_ROOT,
        check=True,
    )


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description="Compare appearance golden screenshots.")
    parser.add_argument(
        "--baseline-dir",
        default="tests/golden/appearance/shell_chrome/baselines",
        help="Baseline directory containing checked-in screenshots (repo-relative).",
    )
    parser.add_argument(
        "--candidate-dir",
        default=None,
        help="Candidate directory to compare against (repo-relative). If omitted, captures into target/appearance-goldens/candidate first.",
    )
    args = parser.parse_args(argv)

    baseline_dir = (REPO_ROOT / args.baseline_dir).resolve()
    if not baseline_dir.exists():
        print(f"[appearance-goldens] missing baseline dir: {baseline_dir}", file=sys.stderr)
        return 2

    if args.candidate_dir:
        candidate_dir = (REPO_ROOT / args.candidate_dir).resolve()
    else:
        candidate_dir = (REPO_ROOT / "target/appearance-goldens/candidate").resolve()
        candidate_dir.mkdir(parents=True, exist_ok=True)
        capture_into(candidate_dir)

    baseline_files = sorted(path for path in baseline_dir.glob("*.png"))
    if not baseline_files:
        print(f"[appearance-goldens] no baseline screenshots found in {baseline_dir}", file=sys.stderr)
        return 2

    failures: list[str] = []
    for baseline in baseline_files:
        candidate = candidate_dir / baseline.name
        if not candidate.exists():
            failures.append(f"missing candidate screenshot: {candidate}")
            continue
        if sha256(baseline) != sha256(candidate):
            failures.append(f"mismatch: {baseline.name}")

    extra = sorted(path.name for path in candidate_dir.glob("*.png") if (baseline_dir / path.name).exists() is False)
    if extra:
        failures.append(f"unexpected candidate screenshots: {extra}")

    if failures:
        print("[appearance-goldens] FAIL", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        print(
            "\n[appearance-goldens] To refresh baselines (intentional change):\n"
            f"  python3 tools/ci/capture_appearance_goldens.py --out-dir {args.baseline_dir}\n",
            file=sys.stderr,
        )
        return 1

    print(f"[appearance-goldens] ok: {len(baseline_files)} screenshots matched")
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))

