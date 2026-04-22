#!/usr/bin/env python3
"""Audit one power / thermal capture for context and policy violations."""

from __future__ import annotations

import argparse
import sys

from common import CaptureError, audit_capture, load_capture


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("capture", help="Path to a power_thermal_capture JSON file")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        capture = load_capture(args.capture)
    except CaptureError as exc:
        print(str(exc), file=sys.stderr)
        return 2

    result = audit_capture(capture)
    if result.errors:
        print("AUDIT FAILED")
        for entry in result.errors:
            print(f"ERROR: {entry}")
        for entry in result.warnings:
            print(f"WARN: {entry}")
        return 1

    print("AUDIT PASSED")
    for entry in result.warnings:
        print(f"WARN: {entry}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
