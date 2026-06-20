#!/usr/bin/env python3
"""Enforce the shiproom benchmark-freshness and comparability ledger.

This gate makes benchmark claim freshness and comparability mechanically
enforceable so a stale corpus revision, an incomparable hardware class or lab
image, a drifted threshold version, an incomplete run, or an aged reference
capture cannot hide behind a green shiproom tile. It validates the ledger
against its schema, resolves the canonical current corpus, lab-image, and
protected-metrics revisions, recomputes for each claim publication entry the
fired downgrade reasons, the freshness state, and the narrowed effective claim
directly from the run that backs it, fails closed when the stored values drift
from the recompute, holds promotion when a claim-bearing entry narrows below the
posture it asserts, keeps superseded runs reviewable without letting them stand
as current proof, keeps release packets aligned with the recomputed freshness
state, runs negative drills proving each rejection fires, and replays the
incomparable-run fixtures that prove each detection path.

Exit code is 0 when every check passes and 1 when any finding is at severity
``error``. With ``--require-proceed`` the gate also fails (exit code 2) when the
recomputed promotion verdict is ``hold``.
"""

from __future__ import annotations

import argparse
import copy
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

try:
    from jsonschema import Draft202012Validator
except Exception as exc:  # pragma: no cover - dependency guard
    raise SystemExit(
        "python jsonschema is required: pip install jsonschema"
    ) from exc


LEDGER_REL = "artifacts/benchmarks/shiproom-benchmark-freshness.json"
LEDGER_SCHEMA_REL = "schemas/benchmarks/shiproom-benchmark-freshness.schema.json"
MATRIX_REL = "artifacts/benchmarks/m5-benchmark-governance.json"
FIXTURE_REGISTER_REL = "fixtures/benchmarks/incomparable-runs/manifest.yaml"

CANONICAL_CORPUS_REL = "fixtures/benchmarks/corpus_manifest.yaml"
CANONICAL_LAB_IMAGE_REL = "artifacts/perf/lab_image_manifest.yaml"
CANONICAL_METRICS_REL = "artifacts/bench/protected_metrics.yaml"

CLAIM_BEARING_LEVELS = {"aureline_only_claim", "public_head_to_head_comparison"}

# Downgrade reasons grouped by the freshness state they imply. The state is the
# coarse color; each reason independently narrows the claim by its narrows_to.
INCOMPARABLE_REASONS = {
    "incomparable_hardware_class",
    "incomparable_lab_image",
    "threshold_version_drift",
    "incomparable_run_metadata",
    "run_metadata_incomplete",
}
STALE_REASONS = {"stale_freshness", "stale_corpus_revision"}


@dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if payload["ref"] is None:
            payload.pop("ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument(
        "--report",
        default=None,
        help="Write the machine-readable JSON report to this repo-relative path.",
    )
    parser.add_argument(
        "--require-proceed",
        action="store_true",
        help="Publication gate: also fail (exit 2) when the recomputed verdict is hold.",
    )
    return parser.parse_args()


def add_finding(
    findings: list[Finding],
    check_id: str,
    message: str,
    remediation: str,
    *,
    ref: str | None = None,
    severity: str = "error",
    details: dict[str, Any] | None = None,
) -> None:
    findings.append(
        Finding(
            severity=severity,
            check_id=check_id,
            message=message,
            remediation=remediation,
            ref=ref,
            details=details or {},
        )
    )


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


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
                "permitted_classes: [Date, Time, DateTime], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path}: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


def parse_date(value: Any) -> dt.date | None:
    if not isinstance(value, str) or not value:
        return None
    try:
        return dt.date.fromisoformat(value)
    except ValueError:
        return None


# --------------------------------------------------------------------------- #
# Canonical current revisions.
# --------------------------------------------------------------------------- #


@dataclass
class Canonical:
    corpus_revision: int
    lab_image_revision_by_id: dict[str, int]
    threshold_version: int


def load_canonical(repo_root: Path, findings: list[Finding]) -> Canonical:
    corpus_doc = render_yaml_as_json(repo_root / CANONICAL_CORPUS_REL)
    corpus_revision = corpus_doc.get("manifest_revision")
    if not isinstance(corpus_revision, int):
        add_finding(
            findings,
            "canonical.corpus_revision",
            "corpus register has no integer manifest_revision",
            "Set manifest_revision on the corpus register so stale revisions are detectable.",
            ref=CANONICAL_CORPUS_REL,
        )
        corpus_revision = 0

    lab_doc = render_yaml_as_json(repo_root / CANONICAL_LAB_IMAGE_REL)
    lab_image_revision_by_id: dict[str, int] = {}
    for row in lab_doc.get("lab_images", []):
        if not isinstance(row, dict):
            continue
        lab_id = row.get("lab_image_id")
        revision = row.get("revision")
        if isinstance(lab_id, str) and isinstance(revision, int):
            lab_image_revision_by_id[lab_id] = revision

    metrics_doc = render_yaml_as_json(repo_root / CANONICAL_METRICS_REL)
    threshold_version = metrics_doc.get("metrics_file_revision")
    if not isinstance(threshold_version, int):
        add_finding(
            findings,
            "canonical.threshold_version",
            "protected-metrics register has no integer metrics_file_revision",
            "Set metrics_file_revision so threshold drift is detectable.",
            ref=CANONICAL_METRICS_REL,
        )
        threshold_version = 0

    return Canonical(
        corpus_revision=corpus_revision,
        lab_image_revision_by_id=lab_image_revision_by_id,
        threshold_version=threshold_version,
    )


# --------------------------------------------------------------------------- #
# Recompute engine.
# --------------------------------------------------------------------------- #


class ClaimEngine:
    """Recompute effective claims from the ledger's frozen narrowing table."""

    def __init__(self, ledger: dict[str, Any]) -> None:
        self.rank = {
            row["level"]: row["rank"] for row in ledger.get("claim_levels", [])
        }
        self.narrows_to = {
            row["reason"]: row["narrows_to"]
            for row in ledger.get("downgrade_reasons", [])
        }

    def effective(self, ceiling: str, fired: list[str]) -> str:
        candidates = [ceiling]
        candidates.extend(self.narrows_to[r] for r in fired if r in self.narrows_to)
        return min(candidates, key=lambda level: self.rank.get(level, 0))


@dataclass
class Anchors:
    bound_corpus_revision: int
    bound_hardware_profile_ref: str
    bound_hardware_class: str
    bound_lab_image_ref: str
    bound_lab_image_revision: int
    bound_threshold_version: int
    freshness_slo_days: int
    warn_within_days: int


def anchors_from(record: dict[str, Any]) -> Anchors:
    return Anchors(
        bound_corpus_revision=record.get("bound_corpus_revision", 0),
        bound_hardware_profile_ref=record.get("bound_hardware_profile_ref", ""),
        bound_hardware_class=record.get("bound_hardware_class", ""),
        bound_lab_image_ref=record.get("bound_lab_image_ref", ""),
        bound_lab_image_revision=record.get("bound_lab_image_revision", 0),
        bound_threshold_version=record.get("bound_threshold_version", 0),
        freshness_slo_days=record.get("freshness_slo_days", 1),
        warn_within_days=record.get("warn_within_days", 0),
    )


def recompute_run(
    run: dict[str, Any] | None,
    anchors: Anchors,
    canonical: Canonical,
    as_of: dt.date,
) -> tuple[list[str], str]:
    """Return (sorted fired downgrade reasons, freshness state) for a run."""
    fired: set[str] = set()

    if not isinstance(run, dict):
        return ["no_current_run"], "missing"

    if run.get("run_corpus_revision") != canonical.corpus_revision:
        fired.add("stale_corpus_revision")

    if (
        run.get("run_hardware_class") != anchors.bound_hardware_class
        or run.get("run_hardware_profile_ref") != anchors.bound_hardware_profile_ref
    ):
        fired.add("incomparable_hardware_class")

    current_lab_revision = canonical.lab_image_revision_by_id.get(
        anchors.bound_lab_image_ref
    )
    if (
        run.get("run_lab_image_ref") != anchors.bound_lab_image_ref
        or run.get("run_lab_image_revision") != anchors.bound_lab_image_revision
        or (
            current_lab_revision is not None
            and run.get("run_lab_image_revision") != current_lab_revision
        )
    ):
        fired.add("incomparable_lab_image")

    if run.get("run_threshold_version") != canonical.threshold_version:
        fired.add("threshold_version_drift")

    if run.get("reset_pending_axes"):
        fired.add("incomparable_run_metadata")

    completeness = run.get("metadata_completeness") or {}
    if completeness.get("missing_fields"):
        fired.add("run_metadata_incomplete")

    captured = parse_date(run.get("captured_on"))
    if captured is None:
        # A present run with no capture date cannot be assessed for freshness;
        # the capture date is itself required run metadata.
        fired.add("run_metadata_incomplete")
    else:
        age = (as_of - captured).days
        if age > anchors.freshness_slo_days:
            fired.add("stale_freshness")
        elif age > (anchors.freshness_slo_days - anchors.warn_within_days):
            fired.add("aging_evidence")

    if "no_current_run" in fired:
        state = "missing"
    elif fired & INCOMPARABLE_REASONS:
        state = "incomparable"
    elif fired & STALE_REASONS:
        state = "stale"
    elif "aging_evidence" in fired:
        state = "aging"
    else:
        state = "current"

    return sorted(fired), state


# --------------------------------------------------------------------------- #
# Ledger validation.
# --------------------------------------------------------------------------- #


def validate_ledger_schema(
    repo_root: Path, ledger: dict[str, Any], findings: list[Finding]
) -> None:
    schema = load_json(repo_root / LEDGER_SCHEMA_REL)
    validator = Draft202012Validator(schema)
    for error in sorted(validator.iter_errors(ledger), key=lambda e: list(e.path)):
        location = "/".join(str(p) for p in error.path) or "<root>"
        add_finding(
            findings,
            "ledger.schema",
            f"ledger fails its schema at {location}: {error.message}",
            "Bring the ledger back into conformance with its boundary schema.",
            ref=LEDGER_REL,
        )


def validate_source_refs(
    repo_root: Path, ledger: dict[str, Any], findings: list[Finding]
) -> None:
    if LEDGER_SCHEMA_REL not in ledger.get("source_refs", []):
        add_finding(
            findings,
            "ledger.source_refs.schema",
            "ledger source_refs must cite its own schema",
            f"Add {LEDGER_SCHEMA_REL} to source_refs.",
            ref=LEDGER_REL,
        )
    for ref in ledger.get("source_refs", []):
        if isinstance(ref, str) and "/" in ref and not (repo_root / ref).exists():
            add_finding(
                findings,
                "ledger.source_refs.missing",
                f"ledger cites a missing source artifact: {ref}",
                "Seed the referenced artifact or correct the source ref.",
                ref=ref,
            )
    for key in ("governance_matrix_ref", "evidence_index_ref"):
        ref = ledger.get(key)
        if isinstance(ref, str) and not (repo_root / ref).exists():
            add_finding(
                findings,
                f"ledger.{key}.missing",
                f"ledger cites a missing artifact for {key}: {ref}",
                "Publish the referenced artifact or correct the ref.",
                ref=ref,
            )


def validate_claim_levels_against_matrix(
    repo_root: Path, ledger: dict[str, Any], findings: list[Finding]
) -> None:
    """The ledger carries its own claim_levels; they must match the matrix ranks
    so the two artifacts cannot drift on claim ordering."""
    matrix = load_json(repo_root / MATRIX_REL)
    matrix_rank = {
        row.get("level"): row.get("rank")
        for row in matrix.get("claim_levels", [])
        if isinstance(row, dict)
    }
    for row in ledger.get("claim_levels", []):
        level = row.get("level")
        if level not in matrix_rank:
            add_finding(
                findings,
                "ledger.claim_level.unknown",
                f"ledger claim level {level} is not in the governance matrix",
                "Bind the ledger claim levels to the matrix vocabulary.",
                ref=LEDGER_REL,
            )
        elif matrix_rank[level] != row.get("rank"):
            add_finding(
                findings,
                "ledger.claim_level.rank_mismatch",
                (
                    f"ledger claim level {level} rank {row.get('rank')} "
                    f"disagrees with the matrix rank {matrix_rank[level]}"
                ),
                "Keep the ledger claim-level ranks in lockstep with the matrix.",
                ref=LEDGER_REL,
            )


def validate_downgrade_table(ledger: dict[str, Any], findings: list[Finding]) -> None:
    """Every downgrade reason the recompute can fire must be declared with a
    narrows_to target."""
    declared = {
        row.get("reason"): row.get("narrows_to")
        for row in ledger.get("downgrade_reasons", [])
    }
    required = INCOMPARABLE_REASONS | STALE_REASONS | {"aging_evidence", "no_current_run"}
    for reason in sorted(required):
        if reason not in declared:
            add_finding(
                findings,
                "ledger.downgrade_table.missing_reason",
                f"downgrade table omits a reason the engine can fire: {reason}",
                "Declare every fireable downgrade reason with its narrows_to target.",
                ref=LEDGER_REL,
            )


def validate_runs_currency(
    entry: dict[str, Any], findings: list[Finding]
) -> None:
    entry_id = entry.get("entry_id", "<entry>")
    current = entry.get("current_run")
    if isinstance(current, dict) and current.get("is_current") is not True:
        add_finding(
            findings,
            "entry.current_run_not_marked_current",
            f"entry {entry_id} current run is not marked is_current",
            "A current run must carry is_current: true.",
            ref=entry_id,
        )
    current_id = current.get("run_id") if isinstance(current, dict) else None
    for hist in entry.get("historical_runs", []):
        if not isinstance(hist, dict):
            continue
        if hist.get("is_current") is True:
            add_finding(
                findings,
                "entry.historical_run_marked_current",
                f"entry {entry_id} historical run {hist.get('run_id')} is marked current",
                "Historical runs are reviewable for diagnosis but never current proof.",
                ref=entry_id,
            )
        if current_id is not None and hist.get("run_id") == current_id:
            add_finding(
                findings,
                "entry.historical_run_shadows_current",
                f"entry {entry_id} reuses the current run id {current_id} as a historical run",
                "Give every run a distinct id so current and historical proof never alias.",
                ref=entry_id,
            )


def validate_entry(
    entry: dict[str, Any],
    engine: ClaimEngine,
    canonical: Canonical,
    as_of: dt.date,
    findings: list[Finding],
) -> dict[str, Any]:
    """Recompute an entry and report drift. Returns the recomputed view used by
    the projection cross-check."""
    entry_id = entry.get("entry_id", "<entry>")
    anchors = anchors_from(entry)
    fired, state = recompute_run(entry.get("current_run"), anchors, canonical, as_of)

    stored_fired = sorted(entry.get("fired_downgrade_reasons", []))
    if stored_fired != fired:
        add_finding(
            findings,
            "entry.fired_reasons_mismatch",
            f"entry {entry_id} stored downgrade reasons {stored_fired} != recompute {fired}",
            "Record exactly the downgrade reasons the run metadata fires.",
            ref=entry_id,
            details={"recomputed": fired, "stored": stored_fired},
        )

    if entry.get("freshness_state") != state:
        add_finding(
            findings,
            "entry.freshness_state_mismatch",
            f"entry {entry_id} freshness_state {entry.get('freshness_state')} != recompute {state}",
            "Derive the freshness state by precedence missing>incomparable>stale>aging>current.",
            ref=entry_id,
            details={"recomputed": state},
        )

    expected_claim = engine.effective(entry.get("published_claim_ceiling", ""), fired)
    if entry.get("effective_claim") != expected_claim:
        add_finding(
            findings,
            "entry.effective_claim_mismatch",
            f"entry {entry_id} effective_claim {entry.get('effective_claim')} != recompute {expected_claim}",
            "Recompute the effective claim as the lowest of the ceiling and each fired reason.",
            ref=entry_id,
            details={"fired": fired},
        )

    # Guardrail: a green tile may never mask an active downgrade reason.
    if state == "current" and fired:
        add_finding(
            findings,
            "entry.green_tile_masks_downgrade",
            f"entry {entry_id} renders current while {fired} fired",
            "A current (green) entry must carry no fired downgrade reason.",
            ref=entry_id,
        )

    # Guardrail: a claim-bearing effective claim may not ride a non-current run.
    if expected_claim in CLAIM_BEARING_LEVELS and state in {"stale", "incomparable", "missing"}:
        add_finding(
            findings,
            "entry.claim_bearing_on_non_current_run",
            f"entry {entry_id} keeps claim-bearing {expected_claim} on a {state} run",
            "A claim-bearing entry must ride a current, comparable run.",
            ref=entry_id,
        )

    validate_runs_currency(entry, findings)

    # Shiproom blocker recompute.
    posture = entry.get("posture")
    claim_bearing_posture = posture in CLAIM_BEARING_LEVELS
    blocks = claim_bearing_posture and engine.rank.get(
        expected_claim, 0
    ) < engine.rank.get(posture, 0)
    blocker = entry.get("shiproom_blocker") or {}
    if blocker.get("blocks_promotion") != blocks:
        add_finding(
            findings,
            "entry.blocker_state_mismatch",
            f"entry {entry_id} blocks_promotion {blocker.get('blocks_promotion')} != recompute {blocks}",
            "An entry blocks promotion only when a claim-bearing posture narrows below itself.",
            ref=entry_id,
        )
    if sorted(blocker.get("blocker_reasons", [])) != fired:
        add_finding(
            findings,
            "entry.blocker_reasons_mismatch",
            f"entry {entry_id} blocker_reasons {sorted(blocker.get('blocker_reasons', []))} != fired {fired}",
            "Surface exactly the fired downgrade reasons as the blocker reasons.",
            ref=entry_id,
        )

    # Release-packet alignment: a packet may not publish a fresher claim than the run.
    packet = entry.get("release_packet") or {}
    if packet.get("declared_freshness_state") != state:
        add_finding(
            findings,
            "entry.release_packet_freshness_mismatch",
            (
                f"entry {entry_id} release packet freshness "
                f"{packet.get('declared_freshness_state')} != recompute {state}"
            ),
            "Keep the release packet freshness state aligned with the recomputed state.",
            ref=entry_id,
        )
    if packet.get("declared_effective_claim") != expected_claim:
        add_finding(
            findings,
            "entry.release_packet_claim_mismatch",
            (
                f"entry {entry_id} release packet claim "
                f"{packet.get('declared_effective_claim')} != recompute {expected_claim}"
            ),
            "Keep the release packet effective claim aligned with the recomputed claim.",
            ref=entry_id,
        )

    return {"entry_id": entry_id, "fired": fired, "blocks": blocks}


def validate_projection(
    ledger: dict[str, Any],
    entry_views: list[dict[str, Any]],
    findings: list[Finding],
) -> None:
    blocking = [v for v in entry_views if v["blocks"]]
    decision = "hold" if blocking else "proceed"
    blocking_entry_ids = sorted(v["entry_id"] for v in blocking)
    blocking_reasons = sorted({r for v in blocking for r in v["fired"]})

    projection = ledger.get("shiproom_projection") or {}
    if projection.get("promotion_decision") != decision:
        add_finding(
            findings,
            "projection.decision_mismatch",
            f"shiproom projection decision {projection.get('promotion_decision')} != recompute {decision}",
            "Hold promotion when any claim-bearing entry is narrowed below its posture.",
            ref=LEDGER_REL,
        )
    if sorted(projection.get("blocking_entry_ids", [])) != blocking_entry_ids:
        add_finding(
            findings,
            "projection.blocking_entries_mismatch",
            "shiproom projection blocking_entry_ids disagrees with the narrowed entries",
            "List exactly the entries whose claim-bearing posture narrowed below itself.",
            ref=LEDGER_REL,
        )
    if sorted(projection.get("blocking_reasons", [])) != blocking_reasons:
        add_finding(
            findings,
            "projection.blocking_reasons_mismatch",
            "shiproom projection blocking_reasons disagrees with the narrowed entries",
            "List exactly the downgrade reasons behind the blocking entries.",
            ref=LEDGER_REL,
        )
    return decision


def validate_ledger(
    repo_root: Path,
    ledger: dict[str, Any],
    canonical: Canonical,
    findings: list[Finding],
) -> str:
    validate_ledger_schema(repo_root, ledger, findings)
    validate_source_refs(repo_root, ledger, findings)
    validate_claim_levels_against_matrix(repo_root, ledger, findings)
    validate_downgrade_table(ledger, findings)

    as_of = parse_date(ledger.get("as_of"))
    if as_of is None:
        add_finding(
            findings,
            "ledger.as_of_invalid",
            "ledger as_of is not a parseable ISO date",
            "Set as_of to the evaluation date.",
            ref=LEDGER_REL,
        )
        return "proceed"

    engine = ClaimEngine(ledger)
    entry_views: list[dict[str, Any]] = []
    seen_ids: set[str] = set()
    for entry in ledger.get("entries", []):
        entry_id = entry.get("entry_id", "<entry>")
        if entry_id in seen_ids:
            add_finding(
                findings,
                "entry.duplicate_id",
                f"duplicate entry id {entry_id}",
                "Entry ids must be unique.",
                ref=entry_id,
            )
        seen_ids.add(entry_id)
        entry_views.append(validate_entry(entry, engine, canonical, as_of, findings))

    decision = validate_projection(ledger, entry_views, findings)
    return decision


# --------------------------------------------------------------------------- #
# Fixture replay.
# --------------------------------------------------------------------------- #


def replay_fixtures(
    repo_root: Path,
    ledger: dict[str, Any],
    canonical: Canonical,
    findings: list[Finding],
) -> int:
    register_path = repo_root / FIXTURE_REGISTER_REL
    if not register_path.exists():
        add_finding(
            findings,
            "fixtures.register_missing",
            f"incomparable-run fixture register is missing: {FIXTURE_REGISTER_REL}",
            "Seed the fixture register so the detection paths are proven.",
            ref=FIXTURE_REGISTER_REL,
        )
        return 0

    register = render_yaml_as_json(register_path)
    schema = load_json(repo_root / LEDGER_SCHEMA_REL)
    validator = Draft202012Validator(schema)
    engine = ClaimEngine(ledger)

    count = 0
    for rel in register.get("fixtures", []):
        fixture = load_json(repo_root / rel)
        count += 1

        if list(validator.iter_errors(fixture)):
            add_finding(
                findings,
                "fixture.schema",
                f"fixture {rel} fails the ledger schema",
                "Bring the fixture into conformance with the fixture-record schema.",
                ref=rel,
            )
            continue

        as_of = parse_date(fixture.get("as_of"))
        if as_of is None:
            add_finding(
                findings,
                "fixture.as_of_invalid",
                f"fixture {rel} has no parseable as_of",
                "Set a fixture as_of so freshness is deterministic.",
                ref=rel,
            )
            continue

        anchors = anchors_from(fixture)
        fired, state = recompute_run(
            fixture.get("current_run"), anchors, canonical, as_of
        )
        expected_fired = sorted(fixture.get("expected_fired_downgrade_reasons", []))
        if fired != expected_fired:
            add_finding(
                findings,
                "fixture.fired_reasons_mismatch",
                f"fixture {fixture.get('fixture_id')} expects {expected_fired} but recompute fires {fired}",
                "Align the fixture run metadata or its expectation.",
                ref=rel,
            )
        if state != fixture.get("expected_freshness_state"):
            add_finding(
                findings,
                "fixture.freshness_state_mismatch",
                (
                    f"fixture {fixture.get('fixture_id')} expects state "
                    f"{fixture.get('expected_freshness_state')} but recompute yields {state}"
                ),
                "Align the fixture run metadata or its expected freshness state.",
                ref=rel,
            )
        expected_claim = engine.effective(fixture.get("posture", ""), fired)
        if expected_claim != fixture.get("expected_effective_claim"):
            add_finding(
                findings,
                "fixture.effective_claim_mismatch",
                (
                    f"fixture {fixture.get('fixture_id')} expects claim "
                    f"{fixture.get('expected_effective_claim')} but recompute yields {expected_claim}"
                ),
                "Align the fixture expectation with the narrowing engine.",
                ref=rel,
            )
    return count


# --------------------------------------------------------------------------- #
# Negative drills.
# --------------------------------------------------------------------------- #


def run_negative_drills(
    repo_root: Path,
    ledger: dict[str, Any],
    canonical: Canonical,
    findings: list[Finding],
) -> list[dict[str, str]]:
    results: list[dict[str, str]] = []

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        local: list[Finding] = []
        validate_ledger(repo_root, candidate, canonical, local)
        return {f.check_id for f in local}

    def record(drill_id: str, expected: str, fired: bool) -> None:
        results.append(
            {"drill_id": drill_id, "expected_check_id": expected, "status": "passed" if fired else "failed"}
        )
        if not fired:
            add_finding(
                findings,
                "negative_drill.not_rejected",
                f"negative drill {drill_id} did not fire {expected}",
                "The recompute must reject this mutation.",
                ref=drill_id,
            )

    def green_entry(candidate: dict[str, Any]) -> dict[str, Any] | None:
        return next(
            (
                e
                for e in candidate["entries"]
                if e.get("freshness_state") == "current"
                and isinstance(e.get("current_run"), dict)
            ),
            None,
        )

    # 1. A stale corpus revision on a green entry must be detected.
    mutated = copy.deepcopy(ledger)
    target = green_entry(mutated)
    if target is not None:
        target["current_run"]["run_corpus_revision"] = canonical.corpus_revision + 1
        record("stale_corpus_detected", "entry.fired_reasons_mismatch", "entry.fired_reasons_mismatch" in check_ids(mutated))

    # 2. An unreset comparability axis must be detected.
    mutated = copy.deepcopy(ledger)
    target = green_entry(mutated)
    if target is not None:
        target["current_run"]["reset_pending_axes"] = ["hardware_class"]
        record("incomparable_axis_detected", "entry.fired_reasons_mismatch", "entry.fired_reasons_mismatch" in check_ids(mutated))

    # 3. Incomplete run metadata must be detected.
    mutated = copy.deepcopy(ledger)
    target = green_entry(mutated)
    if target is not None:
        target["current_run"]["metadata_completeness"]["missing_fields"] = ["raw_run_metadata_ref"]
        record("incomplete_metadata_detected", "entry.fired_reasons_mismatch", "entry.fired_reasons_mismatch" in check_ids(mutated))

    # 4. An aged-out capture must be detected as stale.
    mutated = copy.deepcopy(ledger)
    target = green_entry(mutated)
    if target is not None:
        target["current_run"]["captured_on"] = "2000-01-01"
        record("stale_freshness_detected", "entry.fired_reasons_mismatch", "entry.fired_reasons_mismatch" in check_ids(mutated))

    # 5. A green tile that hides a fired reason must be rejected.
    mutated = copy.deepcopy(ledger)
    target = next((e for e in mutated["entries"] if e.get("freshness_state") == "incomparable"), None)
    if target is not None:
        target["freshness_state"] = "current"
        record("green_tile_masks_downgrade_rejected", "entry.freshness_state_mismatch", "entry.freshness_state_mismatch" in check_ids(mutated))

    # 6. Marking the current run non-current must be rejected.
    mutated = copy.deepcopy(ledger)
    target = green_entry(mutated)
    if target is not None:
        target["current_run"]["is_current"] = False
        record("current_run_not_marked_current_rejected", "entry.current_run_not_marked_current", "entry.current_run_not_marked_current" in check_ids(mutated))

    # 7. Treating a historical run as current must be rejected.
    mutated = copy.deepcopy(ledger)
    target = next((e for e in mutated["entries"] if e.get("historical_runs")), None)
    if target is not None:
        target["historical_runs"][0]["is_current"] = True
        record("historical_run_marked_current_rejected", "entry.historical_run_marked_current", "entry.historical_run_marked_current" in check_ids(mutated))

    # 8. A proceed verdict that hides a narrowed claim-bearing entry must be rejected.
    mutated = copy.deepcopy(ledger)
    target = next(
        (e for e in mutated["entries"] if e.get("posture") in CLAIM_BEARING_LEVELS and isinstance(e.get("current_run"), dict)),
        None,
    )
    if target is not None:
        target["current_run"]["run_corpus_revision"] = canonical.corpus_revision + 1
        record("narrowed_claim_holds_promotion", "projection.decision_mismatch", "projection.decision_mismatch" in check_ids(mutated))

    return results


# --------------------------------------------------------------------------- #
# Entry point.
# --------------------------------------------------------------------------- #


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    findings: list[Finding] = []
    ledger = load_json(repo_root / LEDGER_REL)
    if not isinstance(ledger, dict):
        raise SystemExit("ledger must be a JSON object")

    canonical = load_canonical(repo_root, findings)
    decision = validate_ledger(repo_root, ledger, canonical, findings)
    fixture_count = replay_fixtures(repo_root, ledger, canonical, findings)
    drill_results = run_negative_drills(repo_root, ledger, canonical, findings)

    errors = [f for f in findings if f.severity == "error"]
    warnings = [f for f in findings if f.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[benchmark-shiproom-freshness] {status} "
        f"({len(errors)} errors, {len(warnings)} warnings) -- "
        f"entries: {len(ledger.get('entries', []))}, "
        f"fixtures: {fixture_count}, drills: {len(drill_results)}, "
        f"promotion: {decision}, as_of: {ledger.get('as_of')}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        suffix = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[benchmark-shiproom-freshness] {prefix} {finding.check_id}: {finding.message}{suffix}"
        )
        print(f"[benchmark-shiproom-freshness]   remediation: {finding.remediation}")

    if args.report:
        report = {
            "schema_version": 1,
            "check_id": "benchmark_shiproom_freshness",
            "evaluated_on": ledger.get("as_of"),
            "status": "pass" if not errors else "fail",
            "ledger_ref": LEDGER_REL,
            "entry_count": len(ledger.get("entries", [])),
            "fixture_count": fixture_count,
            "drill_count": len(drill_results),
            "promotion_decision": decision,
            "negative_drills": drill_results,
            "finding_counts": {"error": len(errors), "warning": len(warnings)},
            "findings": [f.as_report() for f in findings],
        }
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(
            json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )

    if errors:
        return 1
    if args.require_proceed and decision == "hold":
        print(
            "[benchmark-shiproom-freshness] PUBLICATION HELD: shiproom promotion blocked",
            file=sys.stderr,
        )
        return 2
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[benchmark-shiproom-freshness] interrupted", file=sys.stderr)
        sys.exit(130)
