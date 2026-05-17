#!/usr/bin/env python3
"""Compatibility wrapper for the beta distributed-compatibility gate."""

from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from tools.ci.m3.distributed_compatibility.__main__ import main


if __name__ == "__main__":
    raise SystemExit(main())
