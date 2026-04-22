#!/usr/bin/env python3
"""Compare two power / thermal captures for posture comparability."""

from __future__ import annotations

import argparse
import sys

from common import CaptureError, compare_captures, load_capture


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("baseline", help="Baseline power_thermal_capture JSON file")
    parser.add_argument("candidate", help="Candidate power_thermal_capture JSON file")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        baseline = load_capture(args.baseline)
        candidate = load_capture(args.candidate)
    except CaptureError as exc:
        print(str(exc), file=sys.stderr)
        return 2

    mismatches, summary = compare_captures(baseline, candidate)
    if mismatches:
        print("NOT COMPARABLE")
        for entry in mismatches:
            print(f"MISMATCH: {entry}")
        return 1

    print("COMPARABLE")
    print(summary)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
