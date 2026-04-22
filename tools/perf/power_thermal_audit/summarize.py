#!/usr/bin/env python3
"""Print a concise summary for one power / thermal capture."""

from __future__ import annotations

import argparse
import sys

from common import CaptureError, load_capture, summarize_capture


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
    print(summarize_capture(capture))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
