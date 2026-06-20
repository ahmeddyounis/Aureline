#!/usr/bin/env python3
"""Enforce reproducibility packs for public, procurement, and enterprise benchmark comparisons.

This gate makes every public or enterprise-facing M5 benchmark comparison
reproducible enough to survive independent review. It validates the checked-in
public-comparison pack register against its boundary schema, binds every
reproducibility pack to a publication pack in the benchmark-governance matrix,
requires the pack's posture, metric set, and reference-hardware and lab-image
identity to agree with the governance pack, requires a claim-bearing pack to carry
the raw configuration, exact commands, raw-run-metadata refs, environment notes,
and reproduction recipe an independent reviewer reruns or audits against, requires
the pack to disclose every field its posture requires, refuses to let a
non-claim-bearing pack authorize a public, procurement, or enterprise surface,
narrows a claim-bearing pack whose freshness has expired, requires exactly one
in-force reproducibility pack for every governance publication pack so no claim
ships without one, validates the worked standalone sample pack the same way, and
replays the fixtures that prove each fail-closed path.

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


REGISTER_REL = "artifacts/benchmarks/public-comparison-pack-register.json"
SCHEMA_REL = "schemas/benchmarks/public-comparison-pack.schema.json"
MATRIX_REL = "artifacts/benchmarks/m5-benchmark-governance.json"
FIXTURE_REGISTER_REL = "fixtures/benchmarks/public-comparison/manifest.yaml"
SAMPLE_PACK_DIR_REL = "artifacts/benchmarks/sample-public-pack"

# Postures that state a measured result; they carry the stricter raw-config,
# environment, raw-run-metadata, and reproduction requirements.
CLAIM_BEARING_POSTURES = {"aureline_only_claim", "public_head_to_head_comparison"}

# Surfaces that publish a measured comparison; only a claim-bearing pack may
# authorize one of these.
CLAIM_BEARING_SURFACES = {"public_comparison", "procurement_packet", "enterprise_evaluation"}

# Statuses that put a pack in force for its governance publication pack.
IN_FORCE_STATUSES = {"publishable", "quarantined"}


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
            "Override the evaluation date (YYYY-MM-DD) for freshness narrowing. "
            "Defaults to the register generated_at date so the gate is deterministic."
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


def parse_date(value: Any) -> dt.date | None:
    if not isinstance(value, str) or not value:
        return None
    try:
        return dt.date.fromisoformat(value)
    except ValueError:
        return None


def strip_meta(payload: dict[str, Any]) -> dict[str, Any]:
    return {k: v for k, v in payload.items() if k not in {"__fixture__", "$schema"}}


# --------------------------------------------------------------------------- #
# Governance-matrix context.
# --------------------------------------------------------------------------- #


def load_matrix_context(repo_root: Path) -> tuple[dict[str, dict[str, Any]], set[str]]:
    matrix = load_json(repo_root / MATRIX_REL)
    packs: dict[str, dict[str, Any]] = {}
    for row in matrix.get("publication_packs", []):
        if not isinstance(row, dict) or not isinstance(row.get("pack_ref"), str):
            continue
        packs[row["pack_ref"]] = {
            "posture": row.get("posture"),
            "metric_refs": set(row.get("metric_refs", [])),
            "required_disclosure_fields": set(row.get("required_disclosure_fields", [])),
            "hardware_profile_ref": row.get("hardware_profile_ref"),
            "lab_image_ref": row.get("lab_image_ref"),
        }
    metric_ids = {
        row.get("metric_ref")
        for row in matrix.get("protected_metrics", [])
        if isinstance(row, dict) and isinstance(row.get("metric_ref"), str)
    }
    return packs, metric_ids


# --------------------------------------------------------------------------- #
# Per-pack admission rules (shared by the register, the sample pack, and the
# fixture replay).
# --------------------------------------------------------------------------- #


def reject_pack(
    record: dict[str, Any],
    matrix_packs: dict[str, dict[str, Any]],
    metric_ids: set[str],
    today: dt.date,
) -> str | None:
    """Return the rule id that rejects a single reproducibility pack, or None.

    These are the per-pack obligations that hold for any reproducibility pack in
    isolation against the governance matrix. Register-wide coverage (one in-force
    pack per governance publication pack) is checked separately.
    """
    gov_ref = record.get("governance_pack_ref")
    matrix_pack = matrix_packs.get(gov_ref)
    if matrix_pack is None:
        return "governance_pack_unresolved"

    if record.get("posture") != matrix_pack["posture"]:
        return "posture_mismatch"

    if set(record.get("metric_refs", [])) != matrix_pack["metric_refs"]:
        return "metric_refs_mismatch"

    if any(metric not in metric_ids for metric in record.get("metric_refs", [])):
        return "unresolved_metric"

    environment = record.get("environment") or {}
    if (
        environment.get("hardware_profile_ref") != matrix_pack["hardware_profile_ref"]
        or environment.get("lab_image_ref") != matrix_pack["lab_image_ref"]
    ):
        return "hardware_identity_mismatch"

    disclosed = set(record.get("disclosed_fields", []))
    if not matrix_pack["required_disclosure_fields"].issubset(disclosed):
        return "undisclosed_required_field"

    posture = record.get("posture")
    surfaces = set(record.get("surfaces", []))
    if posture not in CLAIM_BEARING_POSTURES and (surfaces & CLAIM_BEARING_SURFACES):
        return "non_claim_surface_scope"

    if posture in CLAIM_BEARING_POSTURES:
        raw = record.get("raw_configuration") or {}
        if not raw.get("command_lines") or not raw.get("build_identity_refs"):
            return "missing_raw_configuration"

        if not environment.get("power_thermal_posture") or not environment.get("environment_notes"):
            return "missing_environment_metadata"

        if not record.get("raw_run_metadata_refs") or not record.get("raw_run_metadata_retained"):
            return "missing_raw_run_metadata"

        reproduction = record.get("reproduction") or {}
        if not reproduction.get("steps") or not reproduction.get("rerun_recipe_ref"):
            return "missing_reproduction_recipe"

        if posture == "public_head_to_head_comparison":
            comparison = record.get("comparison") or {}
            if not comparison.get("competitor_version") or not comparison.get("task_parity_note"):
                return "missing_comparison_disclosure"

        expires = parse_date((record.get("freshness") or {}).get("expires_on"))
        if expires is not None and expires < today:
            return "expired_freshness_blocks_claim"

    return None


def _remediation_for(rejected_by: str) -> str:
    return {
        "schema_required_field": "Bring the pack into conformance with its boundary schema.",
        "governance_pack_unresolved": "Bind every reproducibility pack to a publication pack declared in the governance matrix.",
        "posture_mismatch": "Set the pack posture to the one its governance publication pack declares.",
        "metric_refs_mismatch": "Cover exactly the protected metrics the governance publication pack covers.",
        "unresolved_metric": "Bind every metric_ref to a protected metric declared in the governance matrix.",
        "hardware_identity_mismatch": "Match the reference-hardware profile and lab-image revision bound by the governance publication pack.",
        "undisclosed_required_field": "Disclose every field the governance publication pack requires; an incomplete pack is not claim-bearing.",
        "non_claim_surface_scope": "Only a claim-bearing pack may authorize a public, procurement, or enterprise surface.",
        "missing_raw_configuration": "A public claim must carry its exact command line(s) and build identity; do not ship a claim without raw configuration.",
        "missing_environment_metadata": "A public claim must carry its power/thermal posture and environment notes; do not ship a claim without environment metadata.",
        "missing_raw_run_metadata": "Retain and reference the raw run metadata an independent reviewer reruns or audits against.",
        "missing_reproduction_recipe": "Carry the reproduction steps and a rerun-recipe ref so the claim can be rerun later.",
        "missing_comparison_disclosure": "A head-to-head pack must disclose the competitor version and the task-parity conditions of the comparison.",
        "expired_freshness_blocks_claim": "Refresh or withdraw the claim-bearing pack; its freshness window has expired.",
    }.get(rejected_by, "Correct the rejected reproducibility pack.")


# --------------------------------------------------------------------------- #
# Register validation.
# --------------------------------------------------------------------------- #


def validate_schema(
    repo_root: Path, register: dict[str, Any], findings: list[Finding]
) -> Draft202012Validator:
    validator = Draft202012Validator(load_json(repo_root / SCHEMA_REL))
    for error in sorted(validator.iter_errors(register), key=lambda e: list(e.path)):
        location = "/".join(str(p) for p in error.path) or "<root>"
        add_finding(
            findings,
            "register.schema",
            f"public-comparison pack register fails its schema at {location}: {error.message}",
            "Bring the register back into conformance with its boundary schema.",
            ref=REGISTER_REL,
        )
    return validator


def validate_source_refs(
    repo_root: Path, register: dict[str, Any], findings: list[Finding]
) -> None:
    source_refs = register.get("source_refs", [])
    for required in (SCHEMA_REL, MATRIX_REL):
        if required not in source_refs:
            add_finding(
                findings,
                "register.source_refs.missing_required",
                f"register source_refs must cite {required}",
                f"Add {required} to source_refs so the binding is explicit.",
                ref=REGISTER_REL,
            )
    for ref in source_refs:
        if isinstance(ref, str) and "/" in ref and not (repo_root / ref.split("#", 1)[0]).exists():
            add_finding(
                findings,
                "register.source_refs.unresolved",
                f"register cites a missing source artifact: {ref}",
                "Seed the referenced artifact or correct the source ref.",
                ref=ref,
            )
    matrix_ref = register.get("matrix_ref")
    if isinstance(matrix_ref, str) and not (repo_root / matrix_ref).exists():
        add_finding(
            findings,
            "register.matrix_ref.unresolved",
            f"register cites a missing governance matrix: {matrix_ref}",
            "Point matrix_ref at the canonical benchmark-governance matrix.",
            ref=matrix_ref,
        )


def validate_packs(
    register: dict[str, Any],
    matrix_packs: dict[str, dict[str, Any]],
    metric_ids: set[str],
    today: dt.date,
    findings: list[Finding],
) -> None:
    in_force_by_gov: dict[str, list[str]] = {}
    seen_pack_refs: set[str] = set()

    for record in register.get("packs", []):
        pack_ref = record.get("pack_ref", "<unknown>")
        if pack_ref in seen_pack_refs:
            add_finding(
                findings,
                "register.duplicate_pack_ref",
                f"reproducibility pack id {pack_ref} appears more than once",
                "Give every pack a unique pack_ref.",
                ref=REGISTER_REL,
            )
        seen_pack_refs.add(pack_ref)

        rejected_by = reject_pack(record, matrix_packs, metric_ids, today)
        if rejected_by is not None:
            add_finding(
                findings,
                f"pack.{rejected_by}",
                f"reproducibility pack {pack_ref} is rejected by {rejected_by}",
                _remediation_for(rejected_by),
                ref=REGISTER_REL,
                details={"governance_pack_ref": record.get("governance_pack_ref")},
            )

        if record.get("status") in IN_FORCE_STATUSES:
            in_force_by_gov.setdefault(record.get("governance_pack_ref"), []).append(pack_ref)

    _validate_coverage(matrix_packs, in_force_by_gov, findings)


def _validate_coverage(
    matrix_packs: dict[str, dict[str, Any]],
    in_force_by_gov: dict[str, list[str]],
    findings: list[Finding],
) -> None:
    for gov_ref in matrix_packs:
        in_force = in_force_by_gov.get(gov_ref, [])
        if not in_force:
            add_finding(
                findings,
                "register.publication_pack_missing_reproducibility_pack",
                f"governance publication pack {gov_ref} has no in-force reproducibility pack",
                "Record one in-force reproducibility pack per governance publication pack so no claim ships without one.",
                ref=REGISTER_REL,
            )
        elif len(in_force) > 1:
            add_finding(
                findings,
                "register.multiple_in_force_packs",
                f"governance publication pack {gov_ref} has more than one in-force reproducibility pack",
                "Supersede prior packs so exactly one stays in force per governance publication pack.",
                ref=REGISTER_REL,
                details={"pack_refs": in_force},
            )


# --------------------------------------------------------------------------- #
# Sample-pack validation.
# --------------------------------------------------------------------------- #


def validate_sample_packs(
    repo_root: Path,
    validator: Draft202012Validator,
    matrix_packs: dict[str, dict[str, Any]],
    metric_ids: set[str],
    today: dt.date,
    findings: list[Finding],
) -> int:
    sample_dir = repo_root / SAMPLE_PACK_DIR_REL
    if not sample_dir.exists():
        add_finding(
            findings,
            "sample.missing_directory",
            f"sample pack directory is missing: {SAMPLE_PACK_DIR_REL}",
            "Seed the worked standalone sample pack directory.",
            ref=SAMPLE_PACK_DIR_REL,
        )
        return 0
    count = 0
    for path in sorted(sample_dir.glob("*.json")):
        rel = str(path.relative_to(repo_root))
        record = strip_meta(load_json(path))
        count += 1
        schema_errors = list(validator.iter_errors(record))
        if schema_errors:
            add_finding(
                findings,
                "sample.schema",
                f"sample pack {rel} fails its schema: {schema_errors[0].message}",
                "Bring the sample pack into conformance with its boundary schema.",
                ref=rel,
            )
            continue
        rejected_by = reject_pack(record, matrix_packs, metric_ids, today)
        if rejected_by is not None:
            add_finding(
                findings,
                f"sample.{rejected_by}",
                f"sample pack {rel} is rejected by {rejected_by}",
                _remediation_for(rejected_by),
                ref=rel,
            )
    if count == 0:
        add_finding(
            findings,
            "sample.no_packs",
            f"no sample pack JSON found under {SAMPLE_PACK_DIR_REL}",
            "Seed at least one worked standalone sample pack.",
            ref=SAMPLE_PACK_DIR_REL,
        )
    return count


# --------------------------------------------------------------------------- #
# Consumer projection.
# --------------------------------------------------------------------------- #


def build_consumer_projection(
    register: dict[str, Any], today: dt.date
) -> dict[str, Any]:
    """Project the pack ids and claim posture docs, help, release, and procurement cite."""
    rows: list[dict[str, Any]] = []
    for record in register.get("packs", []):
        if record.get("status") not in IN_FORCE_STATUSES:
            continue
        expires = parse_date((record.get("freshness") or {}).get("expires_on"))
        rows.append(
            {
                "pack_ref": record.get("pack_ref"),
                "governance_pack_ref": record.get("governance_pack_ref"),
                "posture": record.get("posture"),
                "claim_bearing": record.get("posture") in CLAIM_BEARING_POSTURES,
                "surfaces": sorted(record.get("surfaces", [])),
                "expires_on": (record.get("freshness") or {}).get("expires_on"),
                "expired": bool(expires and expires < today),
                "raw_run_metadata_retained": bool(record.get("raw_run_metadata_retained")),
            }
        )
    rows.sort(key=lambda r: (r["governance_pack_ref"] or "", r["pack_ref"] or ""))
    return {
        "evaluated_on": today.isoformat(),
        "in_force_pack_count": len(rows),
        "claim_bearing_pack_count": sum(1 for r in rows if r["claim_bearing"]),
        "expired_claim_pack_count": sum(1 for r in rows if r["claim_bearing"] and r["expired"]),
        "packs": rows,
    }


# --------------------------------------------------------------------------- #
# Fixture replay.
# --------------------------------------------------------------------------- #


def replay_fixtures(
    repo_root: Path,
    validator: Draft202012Validator,
    matrix_packs: dict[str, dict[str, Any]],
    metric_ids: set[str],
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

        record = strip_meta(fixture)
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
            rejected_by = reject_pack(record, matrix_packs, metric_ids, today)
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


def resolve_today(register: dict[str, Any], override: str | None) -> dt.date:
    if override:
        try:
            return dt.date.fromisoformat(override)
        except ValueError as exc:
            raise SystemExit(f"--today must be an ISO date: {override!r}") from exc
    generated = register.get("generated_at", "")
    parsed = parse_date(generated[:10] if isinstance(generated, str) else None)
    if parsed is None:
        raise SystemExit("register generated_at is not a parseable date; pass --today")
    return parsed


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    findings: list[Finding] = []
    register = load_json(repo_root / REGISTER_REL)
    if not isinstance(register, dict):
        raise SystemExit("register must be a JSON object")

    today = resolve_today(register, args.today)
    matrix_packs, metric_ids = load_matrix_context(repo_root)

    validator = validate_schema(repo_root, register, findings)
    validate_source_refs(repo_root, register, findings)
    validate_packs(register, matrix_packs, metric_ids, today, findings)
    sample_count = validate_sample_packs(
        repo_root, validator, matrix_packs, metric_ids, today, findings
    )
    projection = build_consumer_projection(register, today)
    fixture_count = replay_fixtures(
        repo_root, validator, matrix_packs, metric_ids, today, findings
    )

    errors = [f for f in findings if f.severity == "error"]
    warnings = [f for f in findings if f.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[public-comparison-pack] {status} "
        f"({len(errors)} errors, {len(warnings)} warnings) -- "
        f"packs: {len(register.get('packs', []))}, "
        f"matrix publication packs: {len(matrix_packs)}, "
        f"in-force packs: {projection['in_force_pack_count']}, "
        f"claim-bearing: {projection['claim_bearing_pack_count']}, "
        f"expired claims: {projection['expired_claim_pack_count']}, "
        f"sample packs: {sample_count}, fixtures: {fixture_count}, "
        f"evaluated_on: {today.isoformat()}"
    )
    for row in projection["packs"]:
        flag = " EXPIRED" if row["expired"] else ""
        bearing = "claim" if row["claim_bearing"] else "non-claim"
        print(
            f"[public-comparison-pack]   {row['pack_ref']} -> {row['governance_pack_ref']} "
            f"({row['posture']}, {bearing}, expires {row['expires_on']}){flag}"
        )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[public-comparison-pack] {prefix} {finding.check_id}: {finding.message}{suffix}")
        print(f"[public-comparison-pack]   remediation: {finding.remediation}")

    if args.report:
        report = {
            "schema_version": 1,
            "check_id": "public_comparison_pack",
            "evaluated_on": today.isoformat(),
            "status": "pass" if not errors else "fail",
            "register_ref": REGISTER_REL,
            "matrix_ref": MATRIX_REL,
            "pack_count": len(register.get("packs", [])),
            "matrix_publication_pack_count": len(matrix_packs),
            "sample_pack_count": sample_count,
            "fixture_count": fixture_count,
            "consumer_projection": projection,
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
        print("[public-comparison-pack] interrupted", file=sys.stderr)
        sys.exit(130)
