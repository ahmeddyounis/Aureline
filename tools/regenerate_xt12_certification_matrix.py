#!/usr/bin/env python3
"""Regenerate the M5 switching/visual-system/attention/boundary certification matrix.

This is the closeout certification lane for the M5 depth surfaces. It answers one
question for every *marketed* M5 surface: does that surface end the milestone with
current proof across the learnability, visual-system, durable-attention, and
embedded-boundary dimensions -- or must it narrow or hold?

To keep the answer honest the matrix never re-types any surface's lifecycle truth.
It ingests the canonical M5 feature-family register
(``artifacts/release/m5/publish_the_m5_feature_family_register_owner_map_and_proof_corpus_plan.json``),
which already carries each marketed surface's claim ceiling and effective
(published) label after the family-level narrowing rules ran. The matrix binds, for
every surface, one cell per certification dimension to the canonical depth-lane
evidence that landed earlier in the milestone, grades each cell, and then:

- carries the claim ceiling and the effective label straight from the source
  register (so a surface can never advertise a wider claim here than the canonical
  packet permits);
- derives each surface's certification state -- ``qualified``, ``narrowed``, or
  ``held_back`` -- from that effective label band; and
- automates narrowing: any below-claim surface lists the dimension(s) and the
  reasons (stale, missing, owner sign-off absent, policy-blocked, hidden onboarding
  gap, ...) that hold it back, instead of leaving the call to marketing copy.

The regenerator emits four checked-in artifacts: the structured matrix JSON, a
frozen validation capture the gate cross-checks, the human-readable evidence index,
and the narrowing report. Release center, Help/About, support exports, and
docs/public-truth publication ingest these instead of cloning status text.

Run from the repository root::

    python3 tools/regenerate_xt12_certification_matrix.py

Pass ``--check`` to verify the checked-in artifacts are current without rewriting
them (used by the CI gate).
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

# Allow ``import xt12_certification_lib`` (the shared build logic the CI gate also
# imports) regardless of the caller's working directory.
sys.path.insert(0, str(Path(__file__).resolve().parent))

from xt12_certification_lib import (  # noqa: E402
    ARTIFACT_REL,
    CAPTURE_REL,
    EVIDENCE_INDEX_REL,
    NARROWING_REPORT_REL,
    build_matrix,
    compute_capture,
    dumps,
    render_evidence_index,
    render_narrowing_report,
)


def targets(repo_root: Path) -> list[tuple[Path, str]]:
    matrix = build_matrix(repo_root)
    capture = compute_capture(matrix)
    return [
        (repo_root / ARTIFACT_REL, dumps(matrix)),
        (repo_root / CAPTURE_REL, dumps(capture)),
        (repo_root / EVIDENCE_INDEX_REL, render_evidence_index(matrix)),
        (repo_root / NARROWING_REPORT_REL, render_narrowing_report(matrix)),
    ]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".", help="Repository root (default: cwd).")
    parser.add_argument(
        "--check",
        action="store_true",
        help="Verify the checked-in files are current without rewriting them.",
    )
    args = parser.parse_args()
    repo_root = Path(args.repo_root).resolve()

    written = targets(repo_root)

    if args.check:
        stale: list[Path] = []
        for path, content in written:
            current = path.read_text(encoding="utf-8") if path.exists() else None
            if current != content:
                stale.append(path)
        if stale:
            for path in stale:
                print(f"STALE: {path.relative_to(repo_root)} is out of date; rerun the regenerator")
            return 1
        print("M5 XT certification matrix, capture, index, and narrowing report are current")
        return 0

    for path, content in written:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content, encoding="utf-8")
        print(f"wrote {path.relative_to(repo_root)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
