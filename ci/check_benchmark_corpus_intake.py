#!/usr/bin/env python3
"""Enforce the corpus-intake gate for partner- or customer-data benchmark corpora.

This gate makes corpus licensing, privacy/redaction posture, retention, and
synthetic-fallback substitution enforceable rather than implicit. It validates
the checked-in corpus-intake ledger against its boundary schema, resolves every
admitted real-data corpus to a materialised id in the canonical corpus register,
requires an approved intake decision before any corpus may back a protected or
public-proof lane, requires a data-steward and privacy review on every sensitive
corpus and a legal clearance on every licensed corpus, blocks real-data CI when
redaction is pending or impossible, keeps a sensitive corpus time-boxed with a
named retention owner and an unexpired purge date, requires an identifiable,
available synthetic fallback whenever real data cannot enter CI, cross-checks the
governance matrix so no protected metric rests on an un-admitted or blocked
corpus, projects the redaction-safe sensitivity view that release, support, and
evaluation packets surface, and replays the redaction fixtures that prove each
fail-closed path.

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


LEDGER_REL = "artifacts/benchmarks/corpus-intake-ledger.json"
SCHEMA_REL = "schemas/benchmarks/corpus-intake-record.schema.json"
MATRIX_REL = "artifacts/benchmarks/m5-benchmark-governance.json"
CANONICAL_CORPUS_REL = "fixtures/benchmarks/corpus_manifest.yaml"
FIXTURE_REGISTER_REL = "fixtures/benchmarks/redaction/manifest.yaml"

# Sensitivity classes that require an approved privacy/data-steward intake review
# before any protected use.
SENSITIVE_CLASSES = {
    "partner_confidential",
    "customer_confidential",
    "regulated_personal_data",
}

# Origins that carry external rights and need a cleared license before a
# protected lane will admit them.
LICENSED_ORIGINS = {
    "vendored_third_party",
    "partner_provided",
    "customer_provided",
    "field_collected",
}

# Use classes that require an approved intake decision; a corpus that is blocked
# may not claim any of them.
PROTECTED_USE_CLASSES = {
    "protected_ci_gate",
    "public_head_to_head_proof",
    "aureline_only_proof",
}

ADMITTED = {"admitted_real_data", "admitted_synthetic_only"}
BLOCKED = {"blocked_pending_intake", "blocked_unredactable"}
CLOSED_DECISIONS = {"rejected", "expired", "withdrawn"}

# Authorities every sensitive corpus must carry on its approvals.
SENSITIVE_REQUIRED_AUTHORITIES = {"data_steward", "privacy_review"}
# Authority a licensed corpus must carry to clear its license.
LICENSE_REQUIRED_AUTHORITY = "legal_review"


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
            "Override the evaluation date (YYYY-MM-DD) for retention-expiry "
            "narrowing. Defaults to the ledger generated_at date so the gate is "
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


def load_canonical_corpus_ids(repo_root: Path) -> set[str]:
    """Materialised corpus ids declared in the canonical corpus register."""
    corpus_doc = render_yaml_as_json(repo_root / CANONICAL_CORPUS_REL)
    ids: set[str] = set()
    collect_strings(corpus_doc, "corpus.", ids)
    return ids


# --------------------------------------------------------------------------- #
# Per-record admission rules (shared by the ledger and the fixture replay).
# --------------------------------------------------------------------------- #


def _approval_authorities(record: dict[str, Any]) -> set[str]:
    return {
        row.get("authority")
        for row in record.get("approvals", [])
        if isinstance(row, dict) and isinstance(row.get("authority"), str)
    }


def reject_intake_record(
    record: dict[str, Any], corpus_ids: set[str], today: dt.date
) -> str | None:
    """Return the rule id that rejects a single intake record, or None.

    These are the per-record obligations that hold for any intake record in
    isolation. Ledger-wide coverage (every matrix corpus has an admitted record)
    is checked separately.
    """
    corpus_ref = record.get("corpus_ref")
    origin = record.get("corpus_origin_class")
    sensitivity = record.get("sensitivity_class")
    admissibility = record.get("ci_admissibility")
    use_classes = set(record.get("approved_use_classes", []))
    license_block = record.get("license") or {}
    redaction = record.get("redaction") or {}
    retention = record.get("retention") or {}
    fallback = record.get("synthetic_fallback") or {}
    decision = record.get("intake_decision") or {}
    decision_status = decision.get("status")
    authorities = _approval_authorities(record)

    # 1. Real data must be materialised in the canonical corpus register.
    if admissibility == "admitted_real_data" and corpus_ref not in corpus_ids:
        return "real_corpus_not_materialized"

    # 2. Origin and sensitivity must be coherent.
    synthetic_or_authored = origin in {"original_project_authored", "synthetic_generated"}
    if synthetic_or_authored and sensitivity in SENSITIVE_CLASSES:
        return "origin_sensitivity_inconsistent"
    if origin in {"partner_provided", "customer_provided"} and sensitivity in {
        "non_sensitive",
        "internal_only",
    }:
        return "origin_sensitivity_inconsistent"

    # 3. A licensed corpus must be cleared.
    if origin in LICENSED_ORIGINS and not license_block.get("cleared"):
        return "license_not_cleared"

    # 4. A closed (rejected/expired/withdrawn) decision must leave the corpus blocked.
    if decision_status in CLOSED_DECISIONS and admissibility not in BLOCKED:
        return "rejected_corpus_not_blocked"

    # 5. An admitted corpus needs an approved intake decision.
    if admissibility in ADMITTED and decision_status != "approved":
        return "admitted_without_approval"

    # 6. A blocked corpus may not claim a protected or public-proof use.
    if admissibility in BLOCKED and (use_classes & PROTECTED_USE_CLASSES):
        return "blocked_claims_protected_use"

    # 7. A sensitive corpus needs a data-steward and privacy review; a licensed
    #    corpus needs a legal clearance.
    if sensitivity in SENSITIVE_CLASSES and not SENSITIVE_REQUIRED_AUTHORITIES.issubset(
        authorities
    ):
        return "sensitive_corpus_missing_review"
    if origin in LICENSED_ORIGINS and LICENSE_REQUIRED_AUTHORITY not in authorities:
        return "licensed_corpus_missing_legal_review"

    # 8. Pending redaction cannot ride into real-data CI.
    redaction_class = redaction.get("redaction_class")
    if redaction_class == "redaction_pending" and admissibility == "admitted_real_data":
        return "redaction_pending_blocks_ci"

    # 9. Unredactable content cannot be admitted as real data.
    if (
        redaction_class == "unredactable_use_synthetic"
        and admissibility == "admitted_real_data"
    ):
        return "unredactable_requires_synthetic"

    # 10-12. Synthetic-fallback obligations.
    if admissibility == "admitted_synthetic_only" and not fallback.get("required"):
        return "synthetic_fallback_not_marked"
    if fallback.get("required"):
        if fallback.get("status") != "available" or not fallback.get("fallback_corpus_ref"):
            return "synthetic_fallback_unavailable"
        if fallback.get("fallback_corpus_ref") not in corpus_ids:
            return "synthetic_fallback_unresolved"

    # 13. A sensitive corpus must be time-boxed with a purge-due date.
    if sensitivity in SENSITIVE_CLASSES and (
        retention.get("retention_class") != "sensitive_time_boxed"
        or not retention.get("purge_due_on")
    ):
        return "retention_posture_missing"

    # 14. An in-force admitted corpus past its purge-due date fails closed.
    purge_due = parse_date(retention.get("purge_due_on"))
    if (
        purge_due is not None
        and purge_due < today
        and admissibility in ADMITTED
        and decision_status == "approved"
    ):
        return "retention_overdue"

    return None


def _remediation_for(rejected_by: str) -> str:
    return {
        "real_corpus_not_materialized": "An admitted_real_data corpus must resolve to a materialised id in the corpus register, or be admitted_synthetic_only.",
        "origin_sensitivity_inconsistent": "Make the sensitivity class match the corpus origin; synthetic/original content is not partner/customer-sensitive.",
        "license_not_cleared": "Clear the license for the approved use classes before admitting a vendored, partner, customer, or field corpus.",
        "rejected_corpus_not_blocked": "A rejected, expired, or withdrawn intake decision must leave the corpus blocked.",
        "admitted_without_approval": "A corpus may not enter a protected lane without an approved intake decision.",
        "blocked_claims_protected_use": "A blocked corpus cannot be approved for a protected_ci_gate or public-proof use.",
        "sensitive_corpus_missing_review": "A sensitive corpus needs both a data-steward and a privacy review approval.",
        "licensed_corpus_missing_legal_review": "A vendored, partner, customer, or field corpus needs a legal-review clearance.",
        "redaction_pending_blocks_ci": "Verify the redaction pass before admitting real data, or fall back to synthetic.",
        "unredactable_requires_synthetic": "Unredactable content cannot enter real-data CI; admit it synthetic-only or block it.",
        "synthetic_fallback_not_marked": "An admitted_synthetic_only corpus must mark its synthetic fallback required.",
        "synthetic_fallback_unavailable": "Provide an available synthetic fallback corpus id so the metric stays reproducible.",
        "synthetic_fallback_unresolved": "Point the synthetic fallback at a materialised corpus id in the corpus register.",
        "retention_posture_missing": "A sensitive corpus must be sensitive_time_boxed with a purge-due date and a retention owner.",
        "retention_overdue": "Purge or re-clear the corpus; its retention window has expired.",
    }.get(rejected_by, "Correct the rejected intake record.")


# --------------------------------------------------------------------------- #
# Ledger validation.
# --------------------------------------------------------------------------- #


def validate_schema(
    repo_root: Path, ledger: dict[str, Any], findings: list[Finding]
) -> Draft202012Validator:
    validator = Draft202012Validator(load_json(repo_root / SCHEMA_REL))
    for error in sorted(validator.iter_errors(ledger), key=lambda e: list(e.path)):
        location = "/".join(str(p) for p in error.path) or "<root>"
        add_finding(
            findings,
            "ledger.schema",
            f"corpus-intake ledger fails its schema at {location}: {error.message}",
            "Bring the ledger back into conformance with its boundary schema.",
            ref=LEDGER_REL,
        )
    return validator


def validate_source_refs(
    repo_root: Path, ledger: dict[str, Any], findings: list[Finding]
) -> None:
    source_refs = ledger.get("source_refs", [])
    for required in (SCHEMA_REL, MATRIX_REL):
        if required not in source_refs:
            add_finding(
                findings,
                "ledger.source_refs.missing_required",
                f"ledger source_refs must cite {required}",
                f"Add {required} to source_refs so the binding is explicit.",
                ref=LEDGER_REL,
            )
    for ref in source_refs:
        if isinstance(ref, str) and "/" in ref and not (repo_root / ref).exists():
            add_finding(
                findings,
                "ledger.source_refs.unresolved",
                f"ledger cites a missing source artifact: {ref}",
                "Seed the referenced artifact or correct the source ref.",
                ref=ref,
            )
    matrix_ref = ledger.get("matrix_ref")
    if isinstance(matrix_ref, str) and not (repo_root / matrix_ref).exists():
        add_finding(
            findings,
            "ledger.matrix_ref.unresolved",
            f"ledger cites a missing governance matrix: {matrix_ref}",
            "Point matrix_ref at the canonical benchmark-governance matrix.",
            ref=matrix_ref,
        )


def validate_records(
    ledger: dict[str, Any],
    corpus_ids: set[str],
    today: dt.date,
    findings: list[Finding],
) -> dict[str, dict[str, Any]]:
    """Validate each record and return the admitted-record index keyed by corpus_ref."""
    intake_by_corpus: dict[str, dict[str, Any]] = {}
    seen_refs: set[str] = set()

    for record in ledger.get("records", []):
        ref = record.get("intake_ref", "<unknown>")
        if ref in seen_refs:
            add_finding(
                findings,
                "ledger.duplicate_intake_ref",
                f"intake record id {ref} appears more than once",
                "Give every intake record a unique intake_ref.",
                ref=LEDGER_REL,
            )
        seen_refs.add(ref)

        rejected_by = reject_intake_record(record, corpus_ids, today)
        if rejected_by is not None:
            add_finding(
                findings,
                f"record.{rejected_by}",
                f"corpus-intake record {ref} is rejected by {rejected_by}",
                _remediation_for(rejected_by),
                ref=LEDGER_REL,
                details={"corpus_ref": record.get("corpus_ref")},
            )

        corpus_ref = record.get("corpus_ref")
        if isinstance(corpus_ref, str):
            if corpus_ref in intake_by_corpus:
                add_finding(
                    findings,
                    "ledger.duplicate_corpus_intake",
                    f"corpus {corpus_ref} carries more than one intake record",
                    "Record exactly one intake record per corpus id.",
                    ref=LEDGER_REL,
                )
            intake_by_corpus[corpus_ref] = record

    return intake_by_corpus


# --------------------------------------------------------------------------- #
# Governance-matrix cross-check.
# --------------------------------------------------------------------------- #


def _matrix_corpus_refs(matrix: dict[str, Any]) -> set[str]:
    refs: set[str] = set()
    for row in matrix.get("corpus_manifests", []):
        if isinstance(row, dict) and isinstance(row.get("corpus_ref"), str):
            refs.add(row["corpus_ref"])
    for metric in matrix.get("protected_metrics", []):
        if not isinstance(metric, dict):
            continue
        for corpus_ref in metric.get("corpus_refs", []):
            if isinstance(corpus_ref, str):
                refs.add(corpus_ref)
    return refs


def validate_matrix_coverage(
    matrix: dict[str, Any],
    intake_by_corpus: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> None:
    """Every corpus the matrix binds must carry an admitted intake record."""
    for corpus_ref in sorted(_matrix_corpus_refs(matrix)):
        record = intake_by_corpus.get(corpus_ref)
        if record is None:
            add_finding(
                findings,
                "matrix.corpus_missing_intake",
                f"protected corpus {corpus_ref} has no corpus-intake record",
                "Record an approved intake record for every corpus a protected metric binds.",
                ref=LEDGER_REL,
            )
            continue
        admissibility = record.get("ci_admissibility")
        if admissibility in BLOCKED:
            add_finding(
                findings,
                "matrix.protected_metric_on_blocked_corpus",
                f"protected corpus {corpus_ref} is {admissibility} and cannot back a protected metric",
                "Clear the corpus's intake or move the metric onto an admitted corpus or its synthetic fallback.",
                ref=LEDGER_REL,
            )
            continue
        if "protected_ci_gate" not in set(record.get("approved_use_classes", [])):
            add_finding(
                findings,
                "matrix.corpus_not_approved_for_ci",
                f"protected corpus {corpus_ref} is not approved for protected_ci_gate use",
                "Approve the corpus for protected_ci_gate before a protected metric binds it.",
                ref=LEDGER_REL,
            )


# --------------------------------------------------------------------------- #
# Redaction-safe sensitivity projection (release / support / evaluation).
# --------------------------------------------------------------------------- #


def build_sensitivity_projection(
    ledger: dict[str, Any], today: dt.date
) -> dict[str, Any]:
    """Project the redaction-safe corpus sensitivity view downstream packets show."""
    rows: list[dict[str, Any]] = []
    for record in ledger.get("records", []):
        retention = record.get("retention") or {}
        fallback = record.get("synthetic_fallback") or {}
        purge_due = parse_date(retention.get("purge_due_on"))
        rows.append(
            {
                "corpus_ref": record.get("corpus_ref"),
                "sensitivity_class": record.get("sensitivity_class"),
                "redaction_class": (record.get("redaction") or {}).get("redaction_class"),
                "ci_admissibility": record.get("ci_admissibility"),
                "approved_use_classes": list(record.get("approved_use_classes", [])),
                "intake_status": (record.get("intake_decision") or {}).get("status"),
                "synthetic_fallback_required": bool(fallback.get("required")),
                "synthetic_fallback_ref": fallback.get("fallback_corpus_ref"),
                "synthetic_fallback_status": fallback.get("status"),
                "retention_class": retention.get("retention_class"),
                "purge_due_on": retention.get("purge_due_on"),
                "purge_overdue": bool(purge_due and purge_due < today),
            }
        )
    rows.sort(key=lambda row: (row["sensitivity_class"] or "", row["corpus_ref"] or ""))
    return {
        "evaluated_on": today.isoformat(),
        "corpus_count": len(rows),
        "sensitive_corpus_count": sum(
            1 for row in rows if row["sensitivity_class"] in SENSITIVE_CLASSES
        ),
        "synthetic_only_count": sum(
            1 for row in rows if row["ci_admissibility"] == "admitted_synthetic_only"
        ),
        "blocked_corpus_count": sum(
            1 for row in rows if row["ci_admissibility"] in BLOCKED
        ),
        "corpora": rows,
    }


# --------------------------------------------------------------------------- #
# Fixture replay.
# --------------------------------------------------------------------------- #


def replay_fixtures(
    repo_root: Path,
    validator: Draft202012Validator,
    corpus_ids: set[str],
    today: dt.date,
    findings: list[Finding],
) -> int:
    register = render_yaml_as_json(repo_root / FIXTURE_REGISTER_REL)
    count = 0
    for entry in register.get("fixtures", []):
        rel = entry.get("file") if isinstance(entry, dict) else entry
        fixture = load_json(repo_root / rel)
        expect = fixture.get("__fixture__", {})
        count += 1

        record = {
            key: value
            for key, value in fixture.items()
            if key not in {"__fixture__", "$schema"}
        }
        schema_valid = not list(validator.iter_errors(record))
        if schema_valid != bool(expect.get("expect_schema_valid")):
            add_finding(
                findings,
                "fixture.schema_expectation",
                (
                    f"fixture {expect.get('fixture_id')} schema validity "
                    f"{schema_valid} != expected {expect.get('expect_schema_valid')}"
                ),
                "Align the fixture payload or its expectation.",
                ref=rel,
            )

        if not schema_valid:
            rejected_by = "schema_required_field"
        else:
            rejected_by = reject_intake_record(record, corpus_ids, today)
        admitted = rejected_by is None

        if admitted != bool(expect.get("expect_admitted")):
            add_finding(
                findings,
                "fixture.admission_expectation",
                (
                    f"fixture {expect.get('fixture_id')} admitted={admitted} "
                    f"!= expected {expect.get('expect_admitted')}"
                ),
                "Align the fixture payload or its expectation.",
                ref=rel,
            )
        if not admitted and rejected_by != expect.get("rejected_by"):
            add_finding(
                findings,
                "fixture.reason_expectation",
                (
                    f"fixture {expect.get('fixture_id')} rejected_by "
                    f"{rejected_by} != expected {expect.get('rejected_by')}"
                ),
                "Align the fixture's rejected_by with the rule that fires.",
                ref=rel,
            )
    return count


# --------------------------------------------------------------------------- #
# Entry point.
# --------------------------------------------------------------------------- #


def resolve_today(ledger: dict[str, Any], override: str | None) -> dt.date:
    if override:
        try:
            return dt.date.fromisoformat(override)
        except ValueError as exc:
            raise SystemExit(f"--today must be an ISO date: {override!r}") from exc
    generated = ledger.get("generated_at", "")
    parsed = parse_date(generated[:10] if isinstance(generated, str) else None)
    if parsed is None:
        raise SystemExit("ledger generated_at is not a parseable date; pass --today")
    return parsed


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    findings: list[Finding] = []
    ledger = load_json(repo_root / LEDGER_REL)
    if not isinstance(ledger, dict):
        raise SystemExit("ledger must be a JSON object")

    today = resolve_today(ledger, args.today)
    corpus_ids = load_canonical_corpus_ids(repo_root)
    matrix = load_json(repo_root / MATRIX_REL)

    validator = validate_schema(repo_root, ledger, findings)
    validate_source_refs(repo_root, ledger, findings)
    intake_by_corpus = validate_records(ledger, corpus_ids, today, findings)
    validate_matrix_coverage(matrix, intake_by_corpus, findings)
    projection = build_sensitivity_projection(ledger, today)
    fixture_count = replay_fixtures(repo_root, validator, corpus_ids, today, findings)

    errors = [f for f in findings if f.severity == "error"]
    warnings = [f for f in findings if f.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[corpus-intake] {status} "
        f"({len(errors)} errors, {len(warnings)} warnings) -- "
        f"records: {len(ledger.get('records', []))}, "
        f"sensitive: {projection['sensitive_corpus_count']}, "
        f"synthetic-only: {projection['synthetic_only_count']}, "
        f"blocked: {projection['blocked_corpus_count']}, "
        f"fixtures: {fixture_count}, evaluated_on: {today.isoformat()}"
    )
    for row in projection["corpora"]:
        if row["sensitivity_class"] in SENSITIVE_CLASSES or row["ci_admissibility"] in BLOCKED:
            flag = " PURGE-OVERDUE" if row["purge_overdue"] else ""
            print(
                f"[corpus-intake]   {row['corpus_ref']}: {row['sensitivity_class']} / "
                f"{row['redaction_class']} -> {row['ci_admissibility']}"
                f" (fallback {row['synthetic_fallback_status']}){flag}"
            )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[corpus-intake] {prefix} {finding.check_id}: {finding.message}{suffix}")
        print(f"[corpus-intake]   remediation: {finding.remediation}")

    if args.report:
        report = {
            "schema_version": 1,
            "check_id": "benchmark_corpus_intake",
            "evaluated_on": today.isoformat(),
            "status": "pass" if not errors else "fail",
            "ledger_ref": LEDGER_REL,
            "matrix_ref": MATRIX_REL,
            "record_count": len(ledger.get("records", [])),
            "fixture_count": fixture_count,
            "sensitivity_projection": projection,
            "finding_counts": {"error": len(errors), "warning": len(warnings)},
            "findings": [f.as_report() for f in findings],
        }
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(
            json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[corpus-intake] interrupted", file=sys.stderr)
        sys.exit(130)
