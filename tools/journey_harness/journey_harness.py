#!/usr/bin/env python3
"""Emit protected user-journey trace records.

Reads the committed protected-corpus manifest, assembles one
``journey_trace_record`` per named journey conforming to
``schemas/traces/journey_trace.schema.json``, and writes the record
under ``--out-dir``.

The harness is intentionally conservative: it uses only the Python
standard library, writes deterministic bytes (explicit 2-space JSON
indentation, ``SOURCE_DATE_EPOCH``-pinned build identity, synthetic
monotonic ticks, no wall-clock reads), and never invents a fixture
id or a protected-journey name outside the registers under
``fixtures/benchmarks/`` and ``schemas/traces/``.

Seed verification (``--verify-seed``) re-emits every seeded journey
under a temporary directory and diffs the fresh output against the
committed fixtures under ``fixtures/journeys/`` so a change to this
script, to the corpus manifest, or to the journey templates either
lands with a matching seed refresh or fails the lane.
"""

from __future__ import annotations

import argparse
import difflib
import json
import os
import sys
import tempfile
from pathlib import Path
from typing import Any, Callable

SCHEMA_TOKEN = "aureline.journey_trace.v1"
RECORD_KIND = "journey_trace_record"
SCHEMA_VERSION = 1

CORPUS_MANIFEST_REL = "fixtures/benchmarks/corpus_manifest.yaml"
SEED_OUT_DIR_REL = "fixtures/journeys"

VALID_BACKENDS = {"headless", "native_window", "synthesised"}
VALID_HOST_OS = {"macos", "linux", "windows", "unknown"}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", required=True)
    parser.add_argument("--out-dir", default=None)
    parser.add_argument(
        "--journey",
        default=None,
        help=(
            "Stable id of a single journey to emit. If omitted and "
            "--emit-all is set, every seeded journey is emitted."
        ),
    )
    parser.add_argument("--emit-all", action="store_true")
    parser.add_argument(
        "--host-os",
        default="linux",
        choices=sorted(VALID_HOST_OS),
        help=(
            "host_os_class value to stamp on the emitted record. Defaults "
            "to linux so the committed seed stays byte-stable across "
            "developer hosts; regeneration on a different host rewrites "
            "this field."
        ),
    )
    parser.add_argument(
        "--rustc-target-triple",
        default="unknown-linux-gnu",
        help=(
            "rustc_target_triple value to stamp on the emitted record. "
            "Defaults to unknown-linux-gnu so the committed seed stays "
            "byte-stable."
        ),
    )
    parser.add_argument("--verify-seed", action="store_true")
    return parser.parse_args()


def read_text(path: Path) -> str:
    with path.open("r", encoding="utf-8") as fh:
        return fh.read()


def parse_corpus_manifest_header(text: str) -> dict[str, Any]:
    """Read manifest_id and manifest_revision from the manifest header.

    A full YAML parser is not available in the stdlib; reading the two
    fields we need by line is enough and keeps the tool stdlib-only.
    Mirrors tools/benchmark_lab_emit.py::parse_corpus_manifest.
    """

    manifest_id = None
    manifest_revision = None
    for raw in text.splitlines():
        stripped = raw.strip()
        if stripped.startswith("manifest_id:") and manifest_id is None:
            manifest_id = stripped.split(":", 1)[1].strip()
        elif stripped.startswith("manifest_revision:") and manifest_revision is None:
            value = stripped.split(":", 1)[1].strip()
            manifest_revision = int(value)
        if manifest_id is not None and manifest_revision is not None:
            break
    if manifest_id is None or manifest_revision is None:
        raise SystemExit(
            f"corpus manifest at {CORPUS_MANIFEST_REL} missing manifest_id or manifest_revision"
        )
    return {"manifest_id": manifest_id, "manifest_revision": manifest_revision}


def collect_manifest_fixture_ids(text: str) -> set[str]:
    """Collect the set of `id:` values from the fixture block.

    The manifest is indented YAML; the fixture-entry ids live on lines
    that start with two spaces then "- id:" at the same indent depth.
    A lenient scan is enough: we only need to validate that a journey
    template's fixture_ref resolves against the manifest.
    """

    ids: set[str] = set()
    in_fixtures_block = False
    for raw in text.splitlines():
        stripped = raw.rstrip()
        if stripped.startswith("fixtures:"):
            in_fixtures_block = True
            continue
        if in_fixtures_block and stripped.startswith("#") is False and stripped.startswith("coverage:"):
            break
        if not in_fixtures_block:
            continue
        candidate = stripped.strip()
        if candidate.startswith("- id:"):
            ids.add(candidate.split(":", 1)[1].strip())
    return ids


def pinned_crate_version() -> str:
    """Return a stable crate_version string for the harness.

    The harness is a Python tool, not a Cargo crate, but it carries a
    pinned version string on its build-identity record so the schema's
    minimum build-identity contract stays consistent.
    """

    return "0.0.0"


def build_identity(rustc_target_triple: str) -> dict[str, Any]:
    return {
        "crate_name": "aureline-journey-harness",
        "crate_version": pinned_crate_version(),
        "rustc_target_triple": rustc_target_triple,
    }


def counters_for(checkpoints: list[dict[str, Any]], segments: list[dict[str, Any]], linked_spike_trace_refs: list[str]) -> dict[str, Any]:
    degraded = sum(1 for s in segments if s.get("degraded_reason") is not None)
    fallback = sum(1 for s in segments if s.get("fallback_reason") is not None)
    return {
        "checkpoint_count": len(checkpoints),
        "segment_count": len(segments),
        "degraded_segment_count": degraded,
        "fallback_segment_count": fallback,
        "linked_spike_trace_count": len(linked_spike_trace_refs),
    }


# -----------------------------------------------------------------------------
# Seeded journeys.
#
# Each entry is a factory that returns a fully-materialised journey-trace
# record given the corpus-manifest ref and the build-identity record. The
# ticks are synthetic monotonic counters (0..N) so committed fixtures stay
# byte-stable across hosts.
# -----------------------------------------------------------------------------


def journey_startup_to_first_useful_chrome(
    corpus_manifest: dict[str, Any],
    build: dict[str, Any],
    host_os: str,
) -> dict[str, Any]:
    checkpoints = [
        {
            "checkpoint_id": "ck.warm_start",
            "checkpoint_class": "journey_start",
            "protected_journey": "startup",
            "tick": 0,
            "spike_hook_ref": "warm_start_to_first_paint",
            "note": "Warm start entry — process is live, no frame yet.",
        },
        {
            "checkpoint_id": "ck.first_paint",
            "checkpoint_class": "protected_path_event",
            "protected_journey": "first_paint",
            "tick": 1,
            "spike_hook_ref": "first_paint",
            "note": "First non-blank frame emitted by the shell surface.",
        },
        {
            "checkpoint_id": "ck.first_useful_chrome",
            "checkpoint_class": "protected_path_event",
            "protected_journey": "first_useful_chrome",
            "tick": 2,
            "spike_hook_ref": "reflow_line_range",
            "note": "Chrome (title / status / sidebar) is laid out and readable.",
        },
        {
            "checkpoint_id": "ck.render_submit",
            "checkpoint_class": "journey_end",
            "protected_journey": "render_submission",
            "tick": 3,
            "spike_hook_ref": "frame_submit",
            "note": "Compositor submitted the first frame to the surface.",
        },
    ]
    segments = [
        {
            "segment_id": "seg.startup_to_first_useful_chrome.startup",
            "segment_class": "startup",
            "protected_journey": "startup",
            "from_checkpoint_id": "ck.warm_start",
            "to_checkpoint_id": "ck.first_paint",
            "duration_ticks": 1,
            "degraded_reason": None,
            "fallback_reason": None,
            "note": "Warm start to first paint.",
        },
        {
            "segment_id": "seg.startup_to_first_useful_chrome.first_paint",
            "segment_class": "first_paint",
            "protected_journey": "first_paint",
            "from_checkpoint_id": "ck.first_paint",
            "to_checkpoint_id": "ck.first_useful_chrome",
            "duration_ticks": 1,
            "degraded_reason": None,
            "fallback_reason": None,
            "note": "First paint to first useful chrome.",
        },
        {
            "segment_id": "seg.startup_to_first_useful_chrome.render_submission",
            "segment_class": "render_submission",
            "protected_journey": "render_submission",
            "from_checkpoint_id": "ck.first_useful_chrome",
            "to_checkpoint_id": "ck.render_submit",
            "duration_ticks": 1,
            "degraded_reason": None,
            "fallback_reason": None,
            "note": "First useful chrome to compositor submit.",
        },
    ]
    linked_spike_trace_refs = ["shell_spike.fixture_v1.full_scene"]
    return {
        "schema": SCHEMA_TOKEN,
        "schema_version": SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "trace_id": "journey_trace.seed.startup_to_first_useful_chrome.micro_local_folder",
        "journey_id": "journey.startup_to_first_useful_chrome.micro_local_folder",
        "journey_class": "startup_to_first_useful_chrome",
        "protected_journeys": [
            "startup",
            "first_paint",
            "first_useful_chrome",
            "render_submission",
        ],
        "fixture_ref": "corpus.reference.micro_local_folder",
        "corpus_manifest": corpus_manifest,
        "build": build,
        "backend": "headless",
        "host_os": host_os,
        "hardware_definition_ref": None,
        "environment_ref": None,
        "exact_build_identity_ref": None,
        "degraded_posture": "healthy",
        "fallback_posture": "none",
        "checkpoints": checkpoints,
        "segments": segments,
        "counters": counters_for(checkpoints, segments, linked_spike_trace_refs),
        "linked_spike_trace_refs": linked_spike_trace_refs,
        "evidence_refs": [
            "benchmark_lab.bootstrap_entry_parity",
            "support_bundle.performance_summary",
        ],
        "requirement_refs": [],
    }


def journey_open_edit_save(
    corpus_manifest: dict[str, Any],
    build: dict[str, Any],
    host_os: str,
) -> dict[str, Any]:
    checkpoints = [
        {
            "checkpoint_id": "ck.open_begin",
            "checkpoint_class": "journey_start",
            "protected_journey": "placeholder_open",
            "tick": 0,
            "spike_hook_ref": None,
            "note": "Placeholder open entered — buffer not yet visible.",
        },
        {
            "checkpoint_id": "ck.open_ready",
            "checkpoint_class": "protected_path_event",
            "protected_journey": "placeholder_open",
            "tick": 1,
            "spike_hook_ref": None,
            "note": "Placeholder open complete — buffer visible, ready for edit.",
        },
        {
            "checkpoint_id": "ck.edit_keystroke",
            "checkpoint_class": "protected_path_event",
            "protected_journey": "placeholder_edit",
            "tick": 2,
            "spike_hook_ref": "reflow_line_range",
            "note": "Single keystroke applied to the buffer.",
        },
        {
            "checkpoint_id": "ck.save_begin",
            "checkpoint_class": "protected_path_event",
            "protected_journey": "placeholder_save",
            "tick": 3,
            "spike_hook_ref": None,
            "note": "Save requested — save pipeline begins.",
        },
        {
            "checkpoint_id": "ck.recovery_journal_write",
            "checkpoint_class": "protected_path_event",
            "protected_journey": "recovery_journal_write",
            "tick": 4,
            "spike_hook_ref": None,
            "note": "Recovery-journal entry written before the target file.",
        },
        {
            "checkpoint_id": "ck.save_complete",
            "checkpoint_class": "journey_end",
            "protected_journey": "save_pipeline",
            "tick": 5,
            "spike_hook_ref": None,
            "note": "Target file replaced atomically — save pipeline complete.",
        },
    ]
    segments = [
        {
            "segment_id": "seg.open_edit_save.placeholder_open",
            "segment_class": "placeholder_open",
            "protected_journey": "placeholder_open",
            "from_checkpoint_id": "ck.open_begin",
            "to_checkpoint_id": "ck.open_ready",
            "duration_ticks": 1,
            "degraded_reason": None,
            "fallback_reason": None,
            "note": "Placeholder open lifecycle.",
        },
        {
            "segment_id": "seg.open_edit_save.placeholder_edit",
            "segment_class": "placeholder_edit",
            "protected_journey": "placeholder_edit",
            "from_checkpoint_id": "ck.open_ready",
            "to_checkpoint_id": "ck.edit_keystroke",
            "duration_ticks": 1,
            "degraded_reason": None,
            "fallback_reason": None,
            "note": "Ready to first keystroke.",
        },
        {
            "segment_id": "seg.open_edit_save.placeholder_save",
            "segment_class": "placeholder_save",
            "protected_journey": "placeholder_save",
            "from_checkpoint_id": "ck.edit_keystroke",
            "to_checkpoint_id": "ck.save_begin",
            "duration_ticks": 1,
            "degraded_reason": None,
            "fallback_reason": None,
            "note": "Keystroke to save-requested.",
        },
        {
            "segment_id": "seg.open_edit_save.save_pipeline",
            "segment_class": "save_pipeline",
            "protected_journey": "save_pipeline",
            "from_checkpoint_id": "ck.save_begin",
            "to_checkpoint_id": "ck.save_complete",
            "duration_ticks": 2,
            "degraded_reason": None,
            "fallback_reason": None,
            "note": "Save pipeline including recovery-journal write.",
        },
    ]
    linked_spike_trace_refs: list[str] = []
    return {
        "schema": SCHEMA_TOKEN,
        "schema_version": SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "trace_id": "journey_trace.seed.open_edit_save.first_useful_edit_rust_self_host",
        "journey_id": "journey.open_edit_save.first_useful_edit_rust_self_host",
        "journey_class": "open_edit_save",
        "protected_journeys": [
            "placeholder_open",
            "placeholder_edit",
            "placeholder_save",
            "save_pipeline",
            "recovery_journal_write",
        ],
        "fixture_ref": "corpus.workflow.first_useful_edit_rust_self_host",
        "corpus_manifest": corpus_manifest,
        "build": build,
        "backend": "synthesised",
        "host_os": host_os,
        "hardware_definition_ref": None,
        "environment_ref": None,
        "exact_build_identity_ref": None,
        "degraded_posture": "healthy",
        "fallback_posture": "none",
        "checkpoints": checkpoints,
        "segments": segments,
        "counters": counters_for(checkpoints, segments, linked_spike_trace_refs),
        "linked_spike_trace_refs": linked_spike_trace_refs,
        "evidence_refs": [
            "benchmark_lab.vfs_save_pipeline",
            "benchmark_lab.certified_archetype_workflows",
            "support_bundle.performance_summary",
        ],
        "requirement_refs": [],
    }


def journey_restore_adjacent(
    corpus_manifest: dict[str, Any],
    build: dict[str, Any],
    host_os: str,
) -> dict[str, Any]:
    checkpoints = [
        {
            "checkpoint_id": "ck.restore_prompt_seen",
            "checkpoint_class": "journey_start",
            "protected_journey": "recovery_journal_restore",
            "tick": 0,
            "spike_hook_ref": None,
            "note": "Restore prompt surfaced after crash + extension update.",
        },
        {
            "checkpoint_id": "ck.degraded_missing_target",
            "checkpoint_class": "degraded_transition",
            "protected_journey": "recovery_journal_restore",
            "tick": 1,
            "spike_hook_ref": None,
            "note": "Missing target observed — binary or extension version changed.",
        },
        {
            "checkpoint_id": "ck.recovery_journal_replay",
            "checkpoint_class": "fallback_transition",
            "protected_journey": "recovery_journal_restore",
            "tick": 2,
            "spike_hook_ref": None,
            "note": "Recovery-journal replay active for dirty buffers.",
        },
        {
            "checkpoint_id": "ck.boundary_truth_contract_verified",
            "checkpoint_class": "protected_path_event",
            "protected_journey": "boundary_truth_contract",
            "tick": 3,
            "spike_hook_ref": None,
            "note": "Entry-and-restore boundary contract verified against fixture.",
        },
        {
            "checkpoint_id": "ck.restore_complete",
            "checkpoint_class": "journey_end",
            "protected_journey": "recovery_journal_restore",
            "tick": 4,
            "spike_hook_ref": None,
            "note": "Compatible restore complete — session continues in reduced posture.",
        },
    ]
    segments = [
        {
            "segment_id": "seg.restore_adjacent.recovery_journal_restore",
            "segment_class": "recovery_journal_restore",
            "protected_journey": "recovery_journal_restore",
            "from_checkpoint_id": "ck.restore_prompt_seen",
            "to_checkpoint_id": "ck.recovery_journal_replay",
            "duration_ticks": 2,
            "degraded_reason": "missing_target_state: binary_or_extension_version_changed; missing_extension_host",
            "fallback_reason": "recovery_journal_replay_active",
            "note": "Prompt → degraded-missing-target → journal replay.",
        },
        {
            "segment_id": "seg.restore_adjacent.boundary_truth_contract",
            "segment_class": "boundary_truth_contract",
            "protected_journey": "boundary_truth_contract",
            "from_checkpoint_id": "ck.recovery_journal_replay",
            "to_checkpoint_id": "ck.boundary_truth_contract_verified",
            "duration_ticks": 1,
            "degraded_reason": None,
            "fallback_reason": None,
            "note": "Boundary-truth entry-and-restore contract verified.",
        },
        {
            "segment_id": "seg.restore_adjacent.finalise",
            "segment_class": "recovery_journal_restore",
            "protected_journey": "recovery_journal_restore",
            "from_checkpoint_id": "ck.boundary_truth_contract_verified",
            "to_checkpoint_id": "ck.restore_complete",
            "duration_ticks": 1,
            "degraded_reason": "missing_target_recovered_to_compatible",
            "fallback_reason": None,
            "note": "Contract verification to compatible-restore complete.",
        },
    ]
    linked_spike_trace_refs: list[str] = []
    return {
        "schema": SCHEMA_TOKEN,
        "schema_version": SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "trace_id": "journey_trace.seed.restore_adjacent.restore_last_session_compatible",
        "journey_id": "journey.restore_adjacent.restore_last_session_compatible",
        "journey_class": "restore_adjacent",
        "protected_journeys": [
            "recovery_journal_restore",
            "boundary_truth_contract",
        ],
        "fixture_ref": "corpus.recovery.restore_last_session_compatible",
        "corpus_manifest": corpus_manifest,
        "build": build,
        "backend": "synthesised",
        "host_os": host_os,
        "hardware_definition_ref": None,
        "environment_ref": None,
        "exact_build_identity_ref": None,
        "degraded_posture": "missing_target_recovered_to_compatible",
        "fallback_posture": "recovery_journal_replay_active",
        "checkpoints": checkpoints,
        "segments": segments,
        "counters": counters_for(checkpoints, segments, linked_spike_trace_refs),
        "linked_spike_trace_refs": linked_spike_trace_refs,
        "evidence_refs": [
            "benchmark_lab.recovery_ladder",
            "support_bundle.crash_recovery_summary",
            "boundary_truth.entry_and_restore_result",
        ],
        "requirement_refs": [],
    }


JourneyFactory = Callable[[dict[str, Any], dict[str, Any], str], dict[str, Any]]

# Journey id -> (seed filename, factory). Seed filename order is the order in
# which --emit-all writes files; keep it alphabetical-by-filename so the
# directory listing stays stable.
SEEDED_JOURNEYS: dict[str, tuple[str, JourneyFactory]] = {
    "journey.open_edit_save.first_useful_edit_rust_self_host": (
        "open_edit_save__first_useful_edit_rust_self_host.json",
        journey_open_edit_save,
    ),
    "journey.restore_adjacent.restore_last_session_compatible": (
        "restore_adjacent__restore_last_session_compatible.json",
        journey_restore_adjacent,
    ),
    "journey.startup_to_first_useful_chrome.micro_local_folder": (
        "startup_to_first_useful_chrome__micro_local_folder.json",
        journey_startup_to_first_useful_chrome,
    ),
}


def validate_fixture_refs(
    journey_records: list[dict[str, Any]],
    manifest_ids: set[str],
) -> None:
    missing = []
    for rec in journey_records:
        fr = rec["fixture_ref"]
        if fr not in manifest_ids:
            missing.append((rec["journey_id"], fr))
    if missing:
        lines = [
            f"  - journey {jid} cites fixture_ref {fr} not found in {CORPUS_MANIFEST_REL}"
            for jid, fr in missing
        ]
        raise SystemExit(
            "journey_harness: unresolved fixture_ref(s):\n" + "\n".join(lines)
        )


def validate_checkpoint_and_segment_refs(records: list[dict[str, Any]]) -> None:
    problems: list[str] = []
    for rec in records:
        ids = {cp["checkpoint_id"] for cp in rec["checkpoints"]}
        seen_ids: set[str] = set()
        for cp in rec["checkpoints"]:
            cid = cp["checkpoint_id"]
            if cid in seen_ids:
                problems.append(
                    f"  - journey {rec['journey_id']} duplicates checkpoint_id {cid}"
                )
            seen_ids.add(cid)
        if rec["checkpoints"]:
            if rec["checkpoints"][0]["checkpoint_class"] != "journey_start":
                problems.append(
                    f"  - journey {rec['journey_id']} first checkpoint is not journey_start"
                )
            if rec["checkpoints"][-1]["checkpoint_class"] != "journey_end":
                problems.append(
                    f"  - journey {rec['journey_id']} last checkpoint is not journey_end"
                )
        seg_ids: set[str] = set()
        for seg in rec["segments"]:
            sid = seg["segment_id"]
            if sid in seg_ids:
                problems.append(
                    f"  - journey {rec['journey_id']} duplicates segment_id {sid}"
                )
            seg_ids.add(sid)
            if seg["from_checkpoint_id"] not in ids:
                problems.append(
                    f"  - journey {rec['journey_id']} segment {sid} from_checkpoint_id "
                    f"{seg['from_checkpoint_id']} does not resolve"
                )
            if seg["to_checkpoint_id"] not in ids:
                problems.append(
                    f"  - journey {rec['journey_id']} segment {sid} to_checkpoint_id "
                    f"{seg['to_checkpoint_id']} does not resolve"
                )
    if problems:
        raise SystemExit(
            "journey_harness: checkpoint / segment reference errors:\n"
            + "\n".join(problems)
        )


def dump_json(path: Path, body: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    text = json.dumps(body, indent=2, ensure_ascii=False, sort_keys=False)
    if not text.endswith("\n"):
        text += "\n"
    path.write_text(text, encoding="utf-8")


def build_record(
    journey_id: str,
    repo_root: Path,
    host_os: str,
    rustc_target_triple: str,
) -> tuple[str, dict[str, Any]]:
    if journey_id not in SEEDED_JOURNEYS:
        raise SystemExit(
            f"journey_harness: unknown journey id {journey_id!r}; "
            f"known ids: {', '.join(sorted(SEEDED_JOURNEYS))}"
        )
    manifest_text = read_text(repo_root / CORPUS_MANIFEST_REL)
    manifest_ref = parse_corpus_manifest_header(manifest_text)
    manifest_ids = collect_manifest_fixture_ids(manifest_text)
    build = build_identity(rustc_target_triple)
    filename, factory = SEEDED_JOURNEYS[journey_id]
    record = factory(manifest_ref, build, host_os)
    validate_fixture_refs([record], manifest_ids)
    validate_checkpoint_and_segment_refs([record])
    return filename, record


def emit_seed_set(
    repo_root: Path,
    out_dir: Path,
    host_os: str,
    rustc_target_triple: str,
) -> list[tuple[str, dict[str, Any]]]:
    manifest_text = read_text(repo_root / CORPUS_MANIFEST_REL)
    manifest_ref = parse_corpus_manifest_header(manifest_text)
    manifest_ids = collect_manifest_fixture_ids(manifest_text)
    build = build_identity(rustc_target_triple)
    pairs: list[tuple[str, dict[str, Any]]] = []
    for jid in sorted(SEEDED_JOURNEYS):
        filename, factory = SEEDED_JOURNEYS[jid]
        pairs.append((filename, factory(manifest_ref, build, host_os)))
    records_only = [r for _, r in pairs]
    validate_fixture_refs(records_only, manifest_ids)
    validate_checkpoint_and_segment_refs(records_only)
    for filename, rec in pairs:
        dump_json(out_dir / filename, rec)
    return pairs


def verify_seed(repo_root: Path) -> int:
    committed_dir = repo_root / SEED_OUT_DIR_REL
    with tempfile.TemporaryDirectory() as tmp:
        emit_dir = Path(tmp) / "emit"
        # The committed seed is always stamped with the pinned
        # (linux, unknown-linux-gnu) build identity; regenerations on
        # another host intentionally rewrite those two fields so
        # verify-seed on a developer macOS laptop still compares like
        # for like.
        emit_seed_set(
            repo_root,
            emit_dir,
            host_os="linux",
            rustc_target_triple="unknown-linux-gnu",
        )
        mismatches: list[str] = []
        for jid in sorted(SEEDED_JOURNEYS):
            rel = SEEDED_JOURNEYS[jid][0]
            committed_path = committed_dir / rel
            fresh_path = emit_dir / rel
            committed = committed_path.read_text(encoding="utf-8")
            fresh = fresh_path.read_text(encoding="utf-8")
            if committed != fresh:
                diff = "\n".join(
                    difflib.unified_diff(
                        committed.splitlines(),
                        fresh.splitlines(),
                        fromfile=f"committed:{rel}",
                        tofile=f"fresh:{rel}",
                        lineterm="",
                    )
                )
                mismatches.append(f"{rel}:\n{diff}")
    if mismatches:
        for m in mismatches:
            print(m, file=sys.stderr)
        print(
            "\njourney_harness: committed seed under "
            f"{SEED_OUT_DIR_REL}/ does not match fresh emission; refresh "
            "the seed or fix the harness so the lane stays honest.",
            file=sys.stderr,
        )
        return 1
    print("journey_harness: committed seed matches fresh emission")
    return 0


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if args.verify_seed:
        return verify_seed(repo_root)
    if args.out_dir is None:
        raise SystemExit("--out-dir is required unless --verify-seed is set")
    out_dir = Path(args.out_dir)
    if not out_dir.is_absolute():
        out_dir = (repo_root / out_dir).resolve()
    else:
        out_dir = out_dir.resolve()
    if args.emit_all:
        pairs = emit_seed_set(
            repo_root,
            out_dir,
            host_os=args.host_os,
            rustc_target_triple=args.rustc_target_triple,
        )
        for filename, _rec in pairs:
            print(f"journey_harness: wrote {out_dir / filename}")
        return 0
    if args.journey is None:
        raise SystemExit(
            "journey_harness: --journey is required unless --emit-all or "
            "--verify-seed is set"
        )
    filename, record = build_record(
        args.journey,
        repo_root,
        host_os=args.host_os,
        rustc_target_triple=args.rustc_target_triple,
    )
    dump_json(out_dir / filename, record)
    print(f"journey_harness: wrote {out_dir / filename}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
