#!/usr/bin/env python3
"""Compatibility wrapper for the canonical corpus freshness gate."""

from __future__ import annotations

import runpy
from pathlib import Path


if __name__ == "__main__":
    runpy.run_path(
        str(Path(__file__).resolve().parents[2] / "ci" / "check_corpus_freshness.py"),
        run_name="__main__",
    )
