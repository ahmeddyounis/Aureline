#!/usr/bin/env python3
"""Enforce the frozen benchmark-governance matrix and its object fixtures.

This gate makes the checked-in benchmark-corpus, reference-hardware, lab-image,
and protected-metric identities enforceable rather than advisory. It validates
the governance matrix against its schema, resolves every protected metric and
publication pack to canonical corpus, hardware, and lab-image ids, fails closed
when a required identity is missing, recomputes the claim-narrowing engine and
checks it against the stored effective claims, holds publication when a
claim-bearing pack rests on a narrowed metric, keeps self-capture rows from
masquerading as protected reference rows, and replays the scenario and object
fixtures that prove each fail-closed path.

Exit code is 0 when every check passes and 1 when any finding is at severity
``error``.
"""

from __future__ import annotations

import argparse
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


MATRIX_REL = "artifacts/benchmarks/m5-benchmark-governance.json"
MATRIX_SCHEMA_REL = "schemas/benchmarks/m5-benchmark-governance.schema.json"
SCENARIO_REGISTER_REL = "fixtures/benchmarks/m5-benchmark-governance/manifest.yaml"
OBJECT_REGISTER_REL = "fixtures/benchmarks/reference-hardware/manifest.yaml"

OBJECT_SCHEMA_RELS = {
    "corpus_manifest": "schemas/benchmarks/corpus-manifest.schema.json",
    "reference_hardware_profile": "schemas/benchmarks/reference-hardware-profile.schema.json",
    "protected_metric": "schemas/benchmarks/protected-metric.schema.json",
}

CANONICAL_CORPUS_REL = "fixtures/benchmarks/corpus_manifest.yaml"
CANONICAL_HARDWARE_REL = "artifacts/perf/reference_hardware_manifest.yaml"
CANONICAL_LAB_IMAGE_REL = "artifacts/perf/lab_image_manifest.yaml"
CANONICAL_METRICS_REL = "artifacts/bench/protected_metrics.yaml"

# Claim levels that carry an external performance conclusion.
CLAIM_BEARING_LEVELS = {"aureline_only_claim", "public_head_to_head_comparison"}

# Threshold states that fire threshold_drift.
DRIFTED_THRESHOLD_STATES = {"drifted_unreviewed", "stale_recalibration_pending"}

# Canonical hardware row_class -> matrix hardware_class.
ROW_CLASS_TO_HARDWARE_CLASS = {
    "council_reference": "reference_lab",
    "self_capture_placeholder": "self_capture",
}

SELF_CAPTURE_MARKER = ".self_capture"


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
        "--today",
        default=None,
        help=(
            "Override the evaluation date (YYYY-MM-DD) for freshness and waiver "
            "narrowing. Defaults to the matrix freeze date so the gate is "
            "deterministic."
        ),
    )
    parser.add_argument(
        "--report",
        default=None,
        help="Write the machine-readable JSON report to this repo-relative path.",
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


def collect_strings(node: Any, prefix: str, out: set[str]) -> None:
    """Recursively gather every string value beginning with ``prefix``."""
    if isinstance(node, dict):
        for value in node.values():
            collect_strings(value, prefix, out)
    elif isinstance(node, list):
        for item in node:
            collect_strings(item, prefix, out)
    elif isinstance(node, str) and node.startswith(prefix):
        out.add(node)


def parse_date(value: Any) -> dt.date | None:
    if not isinstance(value, str) or not value:
        return None
    try:
        return dt.date.fromisoformat(value)
    except ValueError:
        return None


def is_self_capture(ref: Any) -> bool:
    return isinstance(ref, str) and SELF_CAPTURE_MARKER in ref


# --------------------------------------------------------------------------- #
# Canonical identity registers.
# --------------------------------------------------------------------------- #


@dataclass
class Canonical:
    corpus_ids: set[str]
    corpus_revision: int
    hardware_class_by_id: dict[str, str]
    display_ids: set[str]
    power_posture_ids: set[str]
    lab_image_revision_by_id: dict[str, int]
    metric_ids: set[str]


def load_canonical(repo_root: Path, findings: list[Finding]) -> Canonical:
    corpus_doc = render_yaml_as_json(repo_root / CANONICAL_CORPUS_REL)
    corpus_ids: set[str] = set()
    collect_strings(corpus_doc, "corpus.", corpus_ids)
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

    hardware_doc = render_yaml_as_json(repo_root / CANONICAL_HARDWARE_REL)
    hardware_class_by_id: dict[str, str] = {}
    for row in hardware_doc.get("hardware_rows", []):
        if not isinstance(row, dict):
            continue
        row_id = row.get("id")
        row_class = row.get("row_class")
        if isinstance(row_id, str):
            hardware_class_by_id[row_id] = ROW_CLASS_TO_HARDWARE_CLASS.get(
                row_class, "unknown"
            )
    display_ids = {
        row.get("id")
        for row in hardware_doc.get("display_classes", [])
        if isinstance(row, dict) and isinstance(row.get("id"), str)
    }

    lab_doc = render_yaml_as_json(repo_root / CANONICAL_LAB_IMAGE_REL)
    power_posture_ids = {
        row.get("id")
        for row in lab_doc.get("power_postures", [])
        if isinstance(row, dict) and isinstance(row.get("id"), str)
    }
    lab_image_revision_by_id: dict[str, int] = {}
    for row in lab_doc.get("lab_images", []):
        if not isinstance(row, dict):
            continue
        lab_id = row.get("lab_image_id")
        revision = row.get("revision")
        if isinstance(lab_id, str) and isinstance(revision, int):
            lab_image_revision_by_id[lab_id] = revision

    metrics_doc = render_yaml_as_json(repo_root / CANONICAL_METRICS_REL)
    metric_ids = {
        row.get("fitness_row_id")
        for row in metrics_doc.get("rows", [])
        if isinstance(row, dict) and isinstance(row.get("fitness_row_id"), str)
    }

    return Canonical(
        corpus_ids=corpus_ids,
        corpus_revision=corpus_revision,
        hardware_class_by_id=hardware_class_by_id,
        display_ids={d for d in display_ids if isinstance(d, str)},
        power_posture_ids={p for p in power_posture_ids if isinstance(p, str)},
        lab_image_revision_by_id=lab_image_revision_by_id,
        metric_ids={m for m in metric_ids if isinstance(m, str)},
    )


# --------------------------------------------------------------------------- #
# Claim-narrowing engine.
# --------------------------------------------------------------------------- #


class ClaimEngine:
    """Recompute effective claims from the matrix's frozen narrowing rules."""

    def __init__(self, matrix: dict[str, Any]) -> None:
        self.rank = {
            row["level"]: row["rank"] for row in matrix.get("claim_levels", [])
        }
        self.narrows_to = {
            row["reason"]: row["narrows_to"]
            for row in matrix.get("narrowing_rules", [])
        }

    def effective(self, ceiling: str, fired: list[str]) -> str:
        candidates = [ceiling]
        candidates.extend(self.narrows_to[r] for r in fired if r in self.narrows_to)
        return min(candidates, key=lambda level: self.rank.get(level, 0))


def static_metric_reasons(
    metric: dict[str, Any],
    matrix_corpus_revision: dict[str, int],
    canonical: Canonical,
    today: dt.date,
) -> list[str]:
    """Reasons mechanically detectable from the static matrix at ``today``."""
    fired: set[str] = set()

    if not str(metric.get("hardware_profile_ref", "")).strip():
        fired.add("missing_hardware_identity")
    if not str(metric.get("lab_image_ref", "")).strip():
        fired.add("missing_lab_image_identity")

    for corpus_ref in metric.get("corpus_refs", []):
        bound = matrix_corpus_revision.get(corpus_ref)
        if bound is not None and bound != canonical.corpus_revision:
            fired.add("stale_corpus_revision")

    if metric.get("threshold_state") in DRIFTED_THRESHOLD_STATES:
        fired.add("threshold_drift")

    waiver = metric.get("waiver") or {}
    if waiver.get("class") not in (None, "none"):
        expires = parse_date(waiver.get("expires_on"))
        if expires is not None and expires < today:
            fired.add("expired_waiver")

    freshness = metric.get("freshness") or {}
    expires = parse_date(freshness.get("expires_on"))
    if expires is not None and expires < today:
        fired.add("stale_freshness")

    return sorted(fired)


# --------------------------------------------------------------------------- #
# Matrix validation.
# --------------------------------------------------------------------------- #


def validate_matrix_schema(
    repo_root: Path, matrix: dict[str, Any], findings: list[Finding]
) -> None:
    schema = load_json(repo_root / MATRIX_SCHEMA_REL)
    validator = Draft202012Validator(schema)
    for error in sorted(validator.iter_errors(matrix), key=lambda e: list(e.path)):
        location = "/".join(str(p) for p in error.path) or "<root>"
        add_finding(
            findings,
            "matrix.schema",
            f"matrix fails its schema at {location}: {error.message}",
            "Bring the matrix back into conformance with its boundary schema.",
            ref=MATRIX_REL,
        )


def validate_source_refs(
    repo_root: Path, matrix: dict[str, Any], findings: list[Finding]
) -> None:
    if MATRIX_SCHEMA_REL not in matrix.get("source_refs", []):
        add_finding(
            findings,
            "matrix.source_refs.schema",
            "matrix source_refs must cite its own schema",
            f"Add {MATRIX_SCHEMA_REL} to source_refs.",
            ref=MATRIX_REL,
        )
    for ref in matrix.get("source_refs", []):
        if isinstance(ref, str) and "/" in ref and not (repo_root / ref).exists():
            add_finding(
                findings,
                "matrix.source_refs.missing",
                f"matrix cites a missing source artifact: {ref}",
                "Seed the referenced artifact or correct the source ref.",
                ref=ref,
            )
    evidence = matrix.get("evidence_index_ref")
    if isinstance(evidence, str) and not (repo_root / evidence).exists():
        add_finding(
            findings,
            "matrix.evidence_index.missing",
            f"matrix cites a missing evidence index: {evidence}",
            "Publish the evidence index or correct evidence_index_ref.",
            ref=evidence,
        )


def validate_consumer_bindings(matrix: dict[str, Any], findings: list[Finding]) -> None:
    consumers = {
        row.get("consumer", "")
        for row in matrix.get("consumer_bindings", [])
        if isinstance(row, dict)
    }
    needs = {
        "a release surface": lambda c: c.startswith("release"),
        "a support/export surface": lambda c: "support" in c or "export" in c,
        "a docs or help surface": lambda c: c in {"docs", "help"},
    }
    for label, predicate in needs.items():
        if not any(predicate(c) for c in consumers):
            add_finding(
                findings,
                "matrix.consumer_bindings.missing",
                f"matrix has no consumer binding for {label}",
                "Bind release, support/export, and docs/help to the same matrix ids.",
                ref=MATRIX_REL,
            )


def validate_identities(
    matrix: dict[str, Any],
    canonical: Canonical,
    engine: ClaimEngine,
    today: dt.date,
    findings: list[Finding],
) -> None:
    corpus_rows = {row["corpus_ref"]: row for row in matrix.get("corpus_manifests", [])}
    matrix_corpus_revision = {
        ref: row.get("bound_revision") for ref, row in corpus_rows.items()
    }
    hardware_rows = {
        row["hardware_profile_ref"]: row for row in matrix.get("hardware_profiles", [])
    }
    lab_rows = {row["lab_image_ref"]: row for row in matrix.get("lab_images", [])}
    metric_rows = {row["metric_ref"]: row for row in matrix.get("protected_metrics", [])}

    # Corpus manifests must resolve to canonical corpus ids.
    for ref, row in corpus_rows.items():
        if ref not in canonical.corpus_ids:
            add_finding(
                findings,
                "corpus.unresolved",
                f"corpus manifest {ref} is not a canonical corpus id",
                "Bind corpus rows to ids declared in the corpus register, not ad hoc fixtures.",
                ref=ref,
            )
        if row.get("bound_revision") not in (None, canonical.corpus_revision):
            add_finding(
                findings,
                "corpus.stale_revision",
                f"corpus manifest {ref} is bound to a stale revision",
                "Rebaseline the corpus row on the current corpus-manifest revision.",
                ref=ref,
                severity="warning",
            )

    # Hardware profiles must resolve and keep their declared class honest.
    for ref, row in hardware_rows.items():
        canonical_class = canonical.hardware_class_by_id.get(ref)
        if canonical_class is None:
            add_finding(
                findings,
                "hardware.unresolved",
                f"hardware profile {ref} is not a canonical reference-hardware id",
                "Bind hardware rows to the reference-hardware register, not local machines.",
                ref=ref,
            )
        elif canonical_class != row.get("hardware_class"):
            add_finding(
                findings,
                "hardware.class_mismatch",
                (
                    f"hardware profile {ref} declares {row.get('hardware_class')} "
                    f"but the register classes it {canonical_class}"
                ),
                "A self-capture row cannot be relabelled as a reference row.",
                ref=ref,
            )
        if (
            row.get("display_class_ref")
            and row["display_class_ref"] not in canonical.display_ids
        ):
            add_finding(
                findings,
                "hardware.display_unresolved",
                f"hardware profile {ref} cites an unknown display class",
                "Use a display class declared in the reference-hardware register.",
                ref=row.get("display_class_ref"),
            )
        if (
            row.get("default_power_posture")
            and row["default_power_posture"] not in canonical.power_posture_ids
        ):
            add_finding(
                findings,
                "hardware.power_unresolved",
                f"hardware profile {ref} cites an unknown power posture",
                "Use a power posture declared in the lab-image register.",
                ref=row.get("default_power_posture"),
            )

    # Lab images must resolve and pin the canonical revision.
    for ref, row in lab_rows.items():
        canonical_revision = canonical.lab_image_revision_by_id.get(ref)
        if canonical_revision is None:
            add_finding(
                findings,
                "lab_image.unresolved",
                f"lab image {ref} is not a canonical lab-image id",
                "Bind lab images to the lab-image register, not ad hoc environments.",
                ref=ref,
            )
        elif row.get("bound_revision") != canonical_revision:
            add_finding(
                findings,
                "lab_image.revision_mismatch",
                f"lab image {ref} bound revision does not match the register",
                "Pin the bound revision to the canonical lab-image revision.",
                ref=ref,
            )

    # Protected metrics: identity, fail-closed, guardrail, narrowing recompute.
    for ref, metric in metric_rows.items():
        if ref not in canonical.metric_ids:
            add_finding(
                findings,
                "metric.unresolved",
                f"protected metric {ref} is not a canonical protected-metric id",
                "Bind metrics to the protected-metrics register.",
                ref=ref,
            )

        if not metric.get("corpus_refs"):
            add_finding(
                findings,
                "metric.missing_corpus",
                f"protected metric {ref} binds no corpus manifest",
                "Bind every protected metric to at least one corpus manifest.",
                ref=ref,
            )
        for corpus_ref in metric.get("corpus_refs", []):
            if corpus_ref not in corpus_rows:
                add_finding(
                    findings,
                    "metric.corpus_unbound",
                    f"protected metric {ref} cites corpus {corpus_ref} absent from the matrix",
                    "Declare every cited corpus as a corpus manifest row.",
                    ref=ref,
                )

        hw_ref = metric.get("hardware_profile_ref")
        if not str(hw_ref or "").strip():
            add_finding(
                findings,
                "metric.missing_hardware",
                f"protected metric {ref} has no reference-hardware identity",
                "Bind every protected metric to a reference-hardware profile.",
                ref=ref,
            )
        elif hw_ref not in hardware_rows:
            add_finding(
                findings,
                "metric.hardware_unbound",
                f"protected metric {ref} cites hardware {hw_ref} absent from the matrix",
                "Declare every cited hardware profile as a hardware row.",
                ref=ref,
            )

        lab_ref = metric.get("lab_image_ref")
        if not str(lab_ref or "").strip():
            add_finding(
                findings,
                "metric.missing_lab_image",
                f"protected metric {ref} has no lab-image identity",
                "Bind every protected metric to a lab-image revision.",
                ref=ref,
            )
        elif lab_ref not in lab_rows:
            add_finding(
                findings,
                "metric.lab_image_unbound",
                f"protected metric {ref} cites lab image {lab_ref} absent from the matrix",
                "Declare every cited lab image as a lab-image row.",
                ref=ref,
            )

        # Guardrail: a claim-bearing metric may not ride on self-capture identity.
        claim_bearing = (
            metric.get("published_claim_ceiling") in CLAIM_BEARING_LEVELS
            or metric.get("effective_claim") in CLAIM_BEARING_LEVELS
        )
        if claim_bearing and (is_self_capture(hw_ref) or is_self_capture(lab_ref)):
            add_finding(
                findings,
                "metric.claim_bearing_self_capture",
                f"protected metric {ref} makes a claim on self-capture identity",
                "Self-capture evidence is directional only and cannot carry a public claim.",
                ref=ref,
            )

        # Narrowing recompute against the stored claim.
        fired = static_metric_reasons(metric, matrix_corpus_revision, canonical, today)
        stored = list(metric.get("active_narrowing_reasons", []))
        missing = [r for r in fired if r not in stored]
        if missing:
            add_finding(
                findings,
                "metric.unreported_narrowing",
                f"protected metric {ref} does not report detected narrowing: {missing}",
                "Record every detected narrowing reason on the metric.",
                ref=ref,
                details={"detected": fired, "stored": stored},
            )
        combined = sorted(set(fired) | set(stored))
        expected = engine.effective(metric.get("published_claim_ceiling", ""), combined)
        if expected != metric.get("effective_claim"):
            add_finding(
                findings,
                "metric.effective_claim_mismatch",
                (
                    f"protected metric {ref} effective claim is "
                    f"{metric.get('effective_claim')} but recompute yields {expected}"
                ),
                "Recompute the effective claim as the lowest of the ceiling and fired rules.",
                ref=ref,
                details={"fired": combined},
            )

    return validate_publication_packs(matrix, metric_rows, engine, today, findings)


def validate_publication_packs(
    matrix: dict[str, Any],
    metric_rows: dict[str, dict[str, Any]],
    engine: ClaimEngine,
    today: dt.date,
    findings: list[Finding],
) -> None:
    for pack in matrix.get("publication_packs", []):
        ref = pack.get("pack_ref", "<unknown>")

        for metric_ref in pack.get("metric_refs", []):
            if metric_ref not in metric_rows:
                add_finding(
                    findings,
                    "pack.metric_unbound",
                    f"publication pack {ref} cites unknown metric {metric_ref}",
                    "Bind publication packs to declared protected metrics.",
                    ref=ref,
                )

        # Fail-closed disclosure recompute.
        required = set(pack.get("required_disclosure_fields", []))
        disclosed = set(pack.get("disclosed_fields", []))
        fired: list[str] = []
        if not required.issubset(disclosed):
            fired.append("undisclosed_publication_field")
        expires = parse_date((pack.get("freshness") or {}).get("expires_on"))
        if expires is not None and expires < today:
            fired.append("stale_freshness")
        stored = list(pack.get("active_narrowing_reasons", []))
        missing = [r for r in fired if r not in stored]
        if missing:
            add_finding(
                findings,
                "pack.unreported_narrowing",
                f"publication pack {ref} does not report detected narrowing: {missing}",
                "Record every detected narrowing reason on the pack.",
                ref=ref,
                details={"detected": fired, "stored": stored},
            )
        combined = sorted(set(fired) | set(stored))
        expected = engine.effective(pack.get("published_claim_ceiling", ""), combined)
        if expected != pack.get("effective_claim"):
            add_finding(
                findings,
                "pack.effective_claim_mismatch",
                (
                    f"publication pack {ref} effective claim is "
                    f"{pack.get('effective_claim')} but recompute yields {expected}"
                ),
                "Recompute the pack claim as the lowest of the posture and fired rules.",
                ref=ref,
                details={"fired": combined},
            )

        # Guardrail: a claim-bearing pack may not ride on self-capture identity.
        if pack.get("posture") in CLAIM_BEARING_LEVELS and (
            is_self_capture(pack.get("hardware_profile_ref"))
            or is_self_capture(pack.get("lab_image_ref"))
        ):
            add_finding(
                findings,
                "pack.claim_bearing_self_capture",
                f"publication pack {ref} asserts a claim on self-capture identity",
                "Publish claim-bearing packs only on council reference identity.",
                ref=ref,
            )

        # Publication holds when a claim-bearing pack rests on a narrowed metric.
        if pack.get("posture") in CLAIM_BEARING_LEVELS:
            for metric_ref in pack.get("metric_refs", []):
                metric = metric_rows.get(metric_ref)
                if metric is None:
                    continue
                if metric.get("effective_claim") not in CLAIM_BEARING_LEVELS:
                    add_finding(
                        findings,
                        "pack.publication_blocked",
                        (
                            f"publication pack {ref} asserts {pack.get('posture')} but "
                            f"metric {metric_ref} narrowed to {metric.get('effective_claim')}"
                        ),
                        "Hold publication until the underlying metric regains a claim-bearing level.",
                        ref=ref,
                    )


# --------------------------------------------------------------------------- #
# Fixture replay.
# --------------------------------------------------------------------------- #


def replay_scenario_fixtures(
    repo_root: Path, engine: ClaimEngine, findings: list[Finding]
) -> int:
    register = render_yaml_as_json(repo_root / SCENARIO_REGISTER_REL)
    count = 0
    for rel in register.get("fixtures", []):
        fixture = load_json(repo_root / rel)
        count += 1
        ceiling = fixture.get("published_claim_ceiling", "")
        fired = list(fixture.get("fired_narrowing_reasons", []))
        expected = fixture.get("expected_effective_claim")
        computed = engine.effective(ceiling, fired)
        if computed != expected:
            add_finding(
                findings,
                "scenario_fixture.mismatch",
                (
                    f"scenario {fixture.get('fixture_id')} expects {expected} "
                    f"but the engine computes {computed}"
                ),
                "Align the fixture or the narrowing rules so the engine agrees.",
                ref=rel,
            )
    return count


def admit_object(
    record: dict[str, Any], target: str, canonical: Canonical
) -> str | None:
    """Return the rule id that rejects an object, or None when admitted."""
    if target == "corpus_manifest":
        if record.get("corpus_ref") not in canonical.corpus_ids:
            return "identity_unresolved"
        if record.get("bound_revision") != canonical.corpus_revision:
            return "stale_corpus_revision"
        return None

    if target == "reference_hardware_profile":
        ref = record.get("hardware_profile_ref")
        canonical_class = canonical.hardware_class_by_id.get(ref)
        if canonical_class is None:
            return "identity_unresolved"
        if canonical_class != record.get("hardware_class"):
            return "identity_class_mismatch"
        if (
            record.get("hardware_class") == "self_capture"
            or is_self_capture(ref)
        ) and record.get("council_status") != "not_reference_eligible_without_promotion":
            return "self_capture_masquerade"
        if record.get("lab_image_ref") not in canonical.lab_image_revision_by_id:
            return "lab_image_unresolved"
        return None

    if target == "protected_metric":
        for corpus_ref in record.get("corpus_refs", []):
            if corpus_ref not in canonical.corpus_ids:
                return "identity_unresolved"
        if record.get("hardware_profile_ref") not in canonical.hardware_class_by_id:
            return "identity_unresolved"
        if record.get("lab_image_ref") not in canonical.lab_image_revision_by_id:
            return "identity_unresolved"
        claim_bearing = (
            record.get("published_claim_ceiling") in CLAIM_BEARING_LEVELS
            or record.get("effective_claim") in CLAIM_BEARING_LEVELS
        )
        if claim_bearing and (
            is_self_capture(record.get("hardware_profile_ref"))
            or is_self_capture(record.get("lab_image_ref"))
        ):
            return "claim_bearing_self_capture"
        return None

    return "unknown_target_schema"


def replay_object_fixtures(
    repo_root: Path,
    object_validators: dict[str, Draft202012Validator],
    canonical: Canonical,
    findings: list[Finding],
) -> int:
    register = render_yaml_as_json(repo_root / OBJECT_REGISTER_REL)
    count = 0
    for entry in register.get("fixtures", []):
        rel = entry.get("file") if isinstance(entry, dict) else entry
        fixture = load_json(repo_root / rel)
        expect = fixture.get("__fixture__", {})
        target = expect.get("target_schema")
        count += 1

        if target not in object_validators:
            add_finding(
                findings,
                "object_fixture.unknown_schema",
                f"object fixture {rel} names unknown target schema {target}",
                "Use one of the declared object schemas.",
                ref=rel,
            )
            continue

        record = {
            key: value
            for key, value in fixture.items()
            if key not in {"__fixture__", "$schema"}
        }
        schema_valid = not list(object_validators[target].iter_errors(record))
        if schema_valid != bool(expect.get("expect_schema_valid")):
            add_finding(
                findings,
                "object_fixture.schema_expectation",
                (
                    f"object fixture {expect.get('fixture_id')} schema validity "
                    f"{schema_valid} != expected {expect.get('expect_schema_valid')}"
                ),
                "Align the fixture payload or its expectation.",
                ref=rel,
            )

        if not schema_valid:
            rejected_by = "schema_required_field"
        else:
            rejected_by = admit_object(record, target, canonical)
        admitted = rejected_by is None

        if admitted != bool(expect.get("expect_admitted")):
            add_finding(
                findings,
                "object_fixture.admission_expectation",
                (
                    f"object fixture {expect.get('fixture_id')} admitted={admitted} "
                    f"!= expected {expect.get('expect_admitted')}"
                ),
                "Align the fixture payload or its expectation.",
                ref=rel,
            )
        if not admitted and rejected_by != expect.get("rejected_by"):
            add_finding(
                findings,
                "object_fixture.reason_expectation",
                (
                    f"object fixture {expect.get('fixture_id')} rejected_by "
                    f"{rejected_by} != expected {expect.get('rejected_by')}"
                ),
                "Align the fixture's rejected_by with the rule that fires.",
                ref=rel,
            )
    return count


# --------------------------------------------------------------------------- #
# Entry point.
# --------------------------------------------------------------------------- #


def resolve_today(matrix: dict[str, Any], override: str | None) -> dt.date:
    if override:
        try:
            return dt.date.fromisoformat(override)
        except ValueError as exc:
            raise SystemExit(f"--today must be an ISO date: {override!r}") from exc
    generated = matrix.get("generated_at", "")
    parsed = parse_date(generated[:10] if isinstance(generated, str) else None)
    if parsed is None:
        raise SystemExit("matrix generated_at is not a parseable date; pass --today")
    return parsed


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    findings: list[Finding] = []
    matrix = load_json(repo_root / MATRIX_REL)
    if not isinstance(matrix, dict):
        raise SystemExit("matrix must be a JSON object")

    today = resolve_today(matrix, args.today)
    canonical = load_canonical(repo_root, findings)
    engine = ClaimEngine(matrix)
    object_validators = {
        name: Draft202012Validator(load_json(repo_root / rel))
        for name, rel in OBJECT_SCHEMA_RELS.items()
    }

    validate_matrix_schema(repo_root, matrix, findings)
    validate_source_refs(repo_root, matrix, findings)
    validate_consumer_bindings(matrix, findings)
    validate_identities(matrix, canonical, engine, today, findings)
    scenario_count = replay_scenario_fixtures(repo_root, engine, findings)
    object_count = replay_object_fixtures(
        repo_root, object_validators, canonical, findings
    )

    errors = [f for f in findings if f.severity == "error"]
    warnings = [f for f in findings if f.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[benchmark-governance] {status} "
        f"({len(errors)} errors, {len(warnings)} warnings) -- "
        f"metrics: {len(matrix.get('protected_metrics', []))}, "
        f"packs: {len(matrix.get('publication_packs', []))}, "
        f"scenario fixtures: {scenario_count}, object fixtures: {object_count}, "
        f"evaluated_on: {today.isoformat()}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[benchmark-governance] {prefix} {finding.check_id}: {finding.message}{suffix}")
        print(f"[benchmark-governance]   remediation: {finding.remediation}")

    if args.report:
        report = {
            "schema_version": 1,
            "check_id": "m5_benchmark_governance",
            "evaluated_on": today.isoformat(),
            "status": "pass" if not errors else "fail",
            "matrix_ref": MATRIX_REL,
            "metric_count": len(matrix.get("protected_metrics", [])),
            "pack_count": len(matrix.get("publication_packs", [])),
            "scenario_fixture_count": scenario_count,
            "object_fixture_count": object_count,
            "finding_counts": {"error": len(errors), "warning": len(warnings)},
            "findings": [f.as_report() for f in findings],
        }
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[benchmark-governance] interrupted", file=sys.stderr)
        sys.exit(130)
