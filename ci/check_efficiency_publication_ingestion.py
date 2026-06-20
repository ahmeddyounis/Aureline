#!/usr/bin/env python3
"""Enforce the efficiency publication-ingestion register.

This gate makes the rule "every stable-facing description of low-power behavior
derives from the one canonical efficiency-state claim entry" mechanically
enforceable, so documentation, in-product help, the About surface,
service-health, the policy-or-admin surface, and support exports cannot clone
low-power prose that drifts from the actual state model. The canonical entries
are the rows of the M5 efficiency-state governance matrix; this register binds
each consuming surface to the entry it renders.

It validates the register against its schema; resolves every binding's consumed
entry against the governance matrix; asserts each binding renders the entry's
efficiency state, source-of-change, posture, published ceiling, narrowed
effective posture, certification state, claim-support level, override posture,
recovery state, and fired narrowing reasons verbatim so a narrowed or
unsupported claim propagates to every surface in the same train; rejects any
disclosed field that is not export-safe or is on the forbidden raw-telemetry
denylist; confirms the About surface is bound only for claim-bearing entries;
checks every entry is covered on the surfaces it must reach; cross-checks the
register's narrowed/quarantined projection against the matrix; replays the
docs-ingestion fixtures; and runs negative drills proving each rejection fires.

Exit code is 0 when every check passes and 1 when any finding is at severity
``error``.
"""

from __future__ import annotations

import argparse
import copy
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


REGISTER_REL = "artifacts/efficiency/publication-ingestion-register.json"
SCHEMA_REL = "schemas/efficiency/publication-ingestion.schema.json"
MATRIX_REL = "artifacts/efficiency/m5-efficiency-governance.json"
ADMIN_FIELDS_REL = "artifacts/efficiency/admin-surface-fields.md"
FIXTURE_REGISTER_REL = "fixtures/efficiency/docs-ingestion/manifest.yaml"

# Every consuming surface the register declares, and whether it is always
# required for an entry and whether it is authorized only for claim-bearing
# entries. The About surface advertises only claim-bearing low-power postures.
SURFACE_KINDS: dict[str, dict[str, bool]] = {
    "docs": {"always_required": True, "requires_claim_bearing": False},
    "help": {"always_required": True, "requires_claim_bearing": False},
    "about": {"always_required": False, "requires_claim_bearing": True},
    "service_health": {"always_required": True, "requires_claim_bearing": False},
    "admin": {"always_required": True, "requires_claim_bearing": False},
    "support_export": {"always_required": True, "requires_claim_bearing": False},
}

# Stable governance tokens and labels a consuming surface may disclose. Every
# disclosed field on a binding must be in this allowlist.
EXPORT_SAFE_FIELDS: tuple[str, ...] = (
    "entry_id",
    "title",
    "m5_surface",
    "efficiency_state",
    "source_of_change",
    "posture",
    "published_claim_ceiling",
    "effective_posture",
    "certification_state",
    "claim_support",
    "override_posture",
    "recovery_state",
    "fired_narrowing_reasons",
)

# Raw telemetry, payload, and content fields that must never cross the
# publication boundary into a stable-facing surface or an export.
FORBIDDEN_FIELDS: tuple[str, ...] = (
    "raw_energy_trace",
    "raw_power_samples",
    "raw_thermal_samples",
    "raw_battery_telemetry",
    "raw_log",
    "provider_payload",
    "secret_material",
    "user_content",
    "file_path",
    "machine_label",
)

# certification_state -> claim_support level. A narrowed or unsupported claim is
# the same vocabulary on every surface.
CLAIM_SUPPORT_BY_CERTIFICATION: dict[str, str] = {
    "certified": "supported",
    "narrowed": "narrowed",
    "quarantined": "unsupported",
}

QUARANTINE_CERTIFICATION = "quarantined"

# Fields of the entry the rendered projection must echo verbatim, paired with the
# check id raised when they disagree.
PROJECTION_FIELDS: tuple[tuple[str, str], ...] = (
    ("efficiency_state", "projection_efficiency_state_mismatch"),
    ("source_of_change", "projection_source_of_change_mismatch"),
    ("posture", "projection_posture_mismatch"),
    ("published_claim_ceiling", "projection_published_ceiling_mismatch"),
    ("effective_posture", "projection_effective_posture_mismatch"),
    ("certification_state", "projection_certification_state_mismatch"),
    ("claim_support", "projection_claim_support_mismatch"),
    ("override_posture", "projection_override_posture_mismatch"),
    ("recovery_state", "projection_recovery_state_mismatch"),
    ("fired_narrowing_reasons", "projection_narrowing_reasons_mismatch"),
)

_REMEDIATION = {
    "not_rendered_verbatim": "Render the entry's canonical values; set renders_verbatim_from_entry to true.",
    "entry_unresolved": "Point consumes_entry_id at a row in the efficiency governance matrix.",
    "surface_unknown": "Bind only a surface declared in the register's surface_kinds.",
    "surface_not_authorized": "Bind the About surface only for an entry whose effective posture is claim-bearing.",
    "disclosed_field_forbidden": "Remove the forbidden field; raw energy, power, thermal, battery, log, provider, content, and secret data never cross this boundary.",
    "disclosed_field_not_export_safe": "Disclose only fields in the register's export_safe_fields allowlist.",
    "projection_efficiency_state_mismatch": "Render the entry's efficiency state verbatim.",
    "projection_source_of_change_mismatch": "Render the entry's source-of-change tokens verbatim.",
    "projection_posture_mismatch": "Render the entry's posture verbatim.",
    "projection_published_ceiling_mismatch": "Render the entry's published claim ceiling verbatim.",
    "projection_effective_posture_mismatch": "Render the entry's narrowed effective posture, not its ceiling or a stronger level.",
    "projection_certification_state_mismatch": "Render the entry's certification state verbatim.",
    "projection_claim_support_mismatch": "Render the claim-support level derived from the entry's certification state.",
    "projection_override_posture_mismatch": "Render the entry's override posture verbatim.",
    "projection_recovery_state_mismatch": "Render the entry's recovery state verbatim.",
    "projection_narrowing_reasons_mismatch": "Render the entry's fired narrowing reasons verbatim.",
}


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


def strip_meta(payload: dict[str, Any]) -> dict[str, Any]:
    return {k: v for k, v in payload.items() if k not in {"__fixture__", "$schema"}}


# --------------------------------------------------------------------------- #
# Canonical source index (the governance matrix rows).
# --------------------------------------------------------------------------- #


@dataclass
class Canonical:
    entries: dict[str, dict[str, Any]]
    rank: dict[str, int]
    claim_bearing: dict[str, bool]


def claim_support_for(certification_state: str) -> str:
    return CLAIM_SUPPORT_BY_CERTIFICATION.get(certification_state, "unsupported")


def load_canonical(repo_root: Path) -> Canonical:
    matrix = load_json(repo_root / MATRIX_REL)
    rank: dict[str, int] = {}
    claim_bearing: dict[str, bool] = {}
    for level in matrix.get("claim_levels", []):
        if isinstance(level, dict) and isinstance(level.get("level"), str):
            rank[level["level"]] = int(level.get("rank", 0))
            claim_bearing[level["level"]] = bool(level.get("claim_bearing", False))

    entries: dict[str, dict[str, Any]] = {}
    for row in matrix.get("rows", []):
        if not isinstance(row, dict):
            continue
        entry_id = row.get("row_id")
        if not isinstance(entry_id, str):
            continue
        certification_state = row.get("certification_state")
        effective_posture = row.get("effective_posture")
        entries[entry_id] = {
            "entry_id": entry_id,
            "title": row.get("title"),
            "m5_surface": row.get("m5_surface"),
            "efficiency_state": row.get("efficiency_state"),
            "source_of_change": list(row.get("source_of_change") or []),
            "posture": row.get("posture"),
            "published_claim_ceiling": row.get("published_claim_ceiling"),
            "effective_posture": effective_posture,
            "certification_state": certification_state,
            "claim_support": claim_support_for(certification_state),
            "override_posture": row.get("override_posture"),
            "recovery_state": row.get("recovery_state"),
            "fired_narrowing_reasons": list(row.get("fired_narrowing_reasons") or []),
            "is_claim_bearing": claim_bearing.get(effective_posture, False),
        }

    return Canonical(entries=entries, rank=rank, claim_bearing=claim_bearing)


# --------------------------------------------------------------------------- #
# Per-binding admission.
# --------------------------------------------------------------------------- #


def reject_binding(
    record: dict[str, Any],
    canonical: Canonical,
    allow: set[str],
    forbid: set[str],
) -> str | None:
    """Return the rule that rejects a binding, or None when it is admitted.

    The order of the checks is the rejection precedence the fixtures assert.
    """
    if record.get("renders_verbatim_from_entry") is not True:
        return "not_rendered_verbatim"

    entry = canonical.entries.get(record.get("consumes_entry_id"))
    if entry is None:
        return "entry_unresolved"

    surface = record.get("surface")
    kind = SURFACE_KINDS.get(surface)
    if kind is None:
        return "surface_unknown"
    if kind["requires_claim_bearing"] and not entry["is_claim_bearing"]:
        return "surface_not_authorized"

    disclosed = record.get("disclosed_fields") or []
    if any(name in forbid for name in disclosed):
        return "disclosed_field_forbidden"
    if any(name not in allow for name in disclosed):
        return "disclosed_field_not_export_safe"

    projection = record.get("rendered_projection") or {}
    for field_name, check_id in PROJECTION_FIELDS:
        rendered = projection.get(field_name)
        expected = entry.get(field_name)
        if isinstance(expected, list):
            if list(rendered or []) != list(expected):
                return check_id
        elif rendered != expected:
            return check_id

    return None


# --------------------------------------------------------------------------- #
# Register validation.
# --------------------------------------------------------------------------- #


def validate_schema(repo_root: Path, register: dict[str, Any], findings: list[Finding]) -> None:
    schema = load_json(repo_root / SCHEMA_REL)
    validator = Draft202012Validator(schema)
    for error in sorted(validator.iter_errors(register), key=lambda e: list(e.path)):
        location = "/".join(str(p) for p in error.path) or "<root>"
        add_finding(
            findings,
            "register.schema",
            f"register fails its schema at {location}: {error.message}",
            "Bring the register back into conformance with its boundary schema.",
            ref=REGISTER_REL,
        )


def validate_refs(repo_root: Path, register: dict[str, Any], findings: list[Finding]) -> None:
    if SCHEMA_REL not in register.get("source_refs", []):
        add_finding(
            findings,
            "register.source_refs.schema",
            "register source_refs must cite its own schema",
            f"Add {SCHEMA_REL} to source_refs.",
            ref=REGISTER_REL,
        )
    for ref in register.get("source_refs", []):
        if isinstance(ref, str) and "/" in ref and not (repo_root / ref).exists():
            add_finding(
                findings,
                "register.source_refs.missing",
                f"register cites a missing source artifact: {ref}",
                "Seed the referenced artifact or correct the source ref.",
                ref=ref,
            )
    for key in ("governance_matrix_ref", "admin_surface_fields_ref"):
        ref = register.get(key)
        if isinstance(ref, str) and not (repo_root / ref).exists():
            add_finding(
                findings,
                f"register.{key}.missing",
                f"register cites a missing artifact for {key}: {ref}",
                "Publish the referenced artifact or correct the ref.",
                ref=ref,
            )


def validate_surface_kinds(register: dict[str, Any], findings: list[Finding]) -> None:
    declared = {
        row.get("surface"): row
        for row in register.get("surface_kinds", [])
        if isinstance(row, dict)
    }
    for surface, expected in SURFACE_KINDS.items():
        row = declared.get(surface)
        if row is None:
            add_finding(
                findings,
                "register.surface_kind_missing",
                f"surface_kinds omits the {surface} surface",
                "Declare every consuming surface in surface_kinds.",
                ref=REGISTER_REL,
            )
            continue
        for attr in ("always_required", "requires_claim_bearing"):
            if bool(row.get(attr)) != expected[attr]:
                add_finding(
                    findings,
                    "register.surface_kind_attr_mismatch",
                    f"surface_kinds[{surface}].{attr} disagrees with the canonical rule",
                    "Align the surface_kinds row with the canonical authorization rule.",
                    ref=REGISTER_REL,
                    details={"expected": expected[attr]},
                )


def validate_claim_support_levels(register: dict[str, Any], findings: list[Finding]) -> None:
    declared = {
        row.get("level"): row.get("from_certification_state")
        for row in register.get("claim_support_levels", [])
        if isinstance(row, dict)
    }
    for cert, level in CLAIM_SUPPORT_BY_CERTIFICATION.items():
        if declared.get(level) != cert:
            add_finding(
                findings,
                "register.claim_support_level_mismatch",
                f"claim_support_levels does not map {cert} to {level}",
                "Declare each claim-support level and the certification state it derives from.",
                ref=REGISTER_REL,
            )


def validate_bindings(
    register: dict[str, Any],
    canonical: Canonical,
    allow: set[str],
    forbid: set[str],
    findings: list[Finding],
) -> None:
    seen: set[str] = set()
    for binding in register.get("bindings", []):
        binding_id = binding.get("binding_id", "<binding>")
        if binding_id in seen:
            add_finding(
                findings,
                "binding.duplicate_id",
                f"duplicate binding id {binding_id}",
                "Binding ids must be unique.",
                ref=binding_id,
            )
        seen.add(binding_id)

        rejected_by = reject_binding(binding, canonical, allow, forbid)
        if rejected_by is not None:
            add_finding(
                findings,
                f"binding.{rejected_by}",
                f"binding {binding_id} is rejected by {rejected_by}",
                _REMEDIATION.get(rejected_by, "Correct the rejected binding."),
                ref=binding_id,
            )


def required_surfaces_for_entry(entry: dict[str, Any]) -> list[str]:
    needed: list[str] = []
    for surface, kind in SURFACE_KINDS.items():
        if kind["always_required"]:
            needed.append(surface)
        elif kind["requires_claim_bearing"] and entry["is_claim_bearing"]:
            needed.append(surface)
    return needed


def validate_coverage(
    register: dict[str, Any],
    canonical: Canonical,
    findings: list[Finding],
) -> None:
    bound: set[tuple[str, str]] = set()
    for binding in register.get("bindings", []):
        surface = binding.get("surface")
        entry_id = binding.get("consumes_entry_id")
        if isinstance(surface, str) and isinstance(entry_id, str):
            bound.add((surface, entry_id))

    for entry_id, entry in canonical.entries.items():
        for surface in required_surfaces_for_entry(entry):
            if (surface, entry_id) not in bound:
                add_finding(
                    findings,
                    "coverage.missing_surface_binding",
                    f"entry {entry_id} is not rendered on the required {surface} surface",
                    "Add an ingestion binding so the surface points at this entry instead of cloning prose.",
                    ref=entry_id,
                    details={"surface": surface},
                )


def validate_propagation(
    register: dict[str, Any],
    canonical: Canonical,
    findings: list[Finding],
) -> None:
    def rank(level: Any) -> int:
        return canonical.rank.get(level, 0)

    narrowed = sorted(
        eid
        for eid, e in canonical.entries.items()
        if rank(e.get("effective_posture")) < rank(e.get("published_claim_ceiling"))
    )
    quarantined = sorted(
        eid
        for eid, e in canonical.entries.items()
        if e.get("certification_state") == QUARANTINE_CERTIFICATION
    )

    projection = register.get("propagation_projection") or {}
    if sorted(projection.get("narrowed_entry_ids", [])) != narrowed:
        add_finding(
            findings,
            "propagation.narrowed_mismatch",
            "propagation narrowed_entry_ids disagrees with the matrix",
            "List exactly the entries whose effective posture is below their ceiling.",
            ref=REGISTER_REL,
            details={"recomputed": narrowed},
        )
    if sorted(projection.get("quarantined_entry_ids", [])) != quarantined:
        add_finding(
            findings,
            "propagation.quarantined_mismatch",
            "propagation quarantined_entry_ids disagrees with the matrix",
            "List exactly the entries whose certification state is quarantined.",
            ref=REGISTER_REL,
            details={"recomputed": quarantined},
        )

    # A binding may never render a stronger claim than the entry it consumes; this
    # is the cross-surface guarantee that a narrowed claim propagates everywhere.
    for binding in register.get("bindings", []):
        entry = canonical.entries.get(binding.get("consumes_entry_id"))
        if entry is None:
            continue
        rendered = (binding.get("rendered_projection") or {}).get("effective_posture")
        if rendered is not None and rank(rendered) > rank(entry.get("effective_posture")):
            add_finding(
                findings,
                "propagation.binding_over_claims",
                f"binding {binding.get('binding_id')} renders {rendered} above the entry's {entry.get('effective_posture')}",
                "A surface must never publish a stronger claim than the entry's narrowed effective posture.",
                ref=binding.get("binding_id"),
            )


def validate_register(
    repo_root: Path,
    register: dict[str, Any],
    canonical: Canonical,
    findings: list[Finding],
) -> None:
    validate_schema(repo_root, register, findings)
    validate_refs(repo_root, register, findings)
    validate_surface_kinds(register, findings)
    validate_claim_support_levels(register, findings)

    allow = {row["field"] for row in register.get("export_safe_fields", []) if "field" in row}
    forbid = {row["field"] for row in register.get("forbidden_fields", []) if "field" in row}

    validate_bindings(register, canonical, allow, forbid, findings)
    validate_coverage(register, canonical, findings)
    validate_propagation(register, canonical, findings)


# --------------------------------------------------------------------------- #
# Fixture replay.
# --------------------------------------------------------------------------- #


def replay_fixtures(
    repo_root: Path,
    register: dict[str, Any],
    canonical: Canonical,
    findings: list[Finding],
) -> int:
    register_path = repo_root / FIXTURE_REGISTER_REL
    if not register_path.exists():
        add_finding(
            findings,
            "fixtures.register_missing",
            f"docs-ingestion fixture register is missing: {FIXTURE_REGISTER_REL}",
            "Seed the fixture register so the rejection paths are proven.",
            ref=FIXTURE_REGISTER_REL,
        )
        return 0

    fixture_register = render_yaml_as_json(register_path)
    schema = load_json(repo_root / SCHEMA_REL)
    validator = Draft202012Validator(schema)
    allow = {row["field"] for row in register.get("export_safe_fields", []) if "field" in row}
    forbid = {row["field"] for row in register.get("forbidden_fields", []) if "field" in row}

    count = 0
    for row in fixture_register.get("fixtures", []):
        rel = row.get("file") if isinstance(row, dict) else row
        if not isinstance(rel, str):
            continue
        fixture = load_json(repo_root / rel)
        expect = fixture.get("__fixture__", {})
        record = strip_meta(fixture)
        count += 1

        schema_valid = not list(validator.iter_errors(record))
        if schema_valid != bool(expect.get("expect_schema_valid")):
            add_finding(
                findings,
                "fixture.schema_expectation",
                (
                    f"fixture {expect.get('fixture_id')} schema_valid {schema_valid} "
                    f"!= expected {expect.get('expect_schema_valid')}"
                ),
                "Align the fixture record with its expect_schema_valid flag.",
                ref=rel,
            )
            continue

        if not schema_valid:
            rejected_by = "schema_required_field"
        else:
            rejected_by = reject_binding(record, canonical, allow, forbid)
        admitted = rejected_by is None

        if admitted != bool(expect.get("expect_admitted")):
            add_finding(
                findings,
                "fixture.admission_expectation",
                (
                    f"fixture {expect.get('fixture_id')} admitted {admitted} "
                    f"!= expected {expect.get('expect_admitted')}"
                ),
                "Align the fixture with the admission rules or its expectation.",
                ref=rel,
            )
            continue

        if not admitted and rejected_by != expect.get("rejected_by"):
            add_finding(
                findings,
                "fixture.rejected_by_mismatch",
                (
                    f"fixture {expect.get('fixture_id')} rejected_by "
                    f"{rejected_by} != expected {expect.get('rejected_by')}"
                ),
                "Align the fixture's rejected_by with the rule that fires.",
                ref=rel,
            )
    return count


# --------------------------------------------------------------------------- #
# Negative drills.
# --------------------------------------------------------------------------- #


def run_negative_drills(
    repo_root: Path,
    register: dict[str, Any],
    canonical: Canonical,
    findings: list[Finding],
) -> list[dict[str, str]]:
    results: list[dict[str, str]] = []

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        local: list[Finding] = []
        validate_register(repo_root, candidate, canonical, local)
        return {f.check_id for f in local}

    def record(drill_id: str, expected: str, fired: bool) -> None:
        results.append(
            {
                "drill_id": drill_id,
                "expected_check_id": expected,
                "status": "passed" if fired else "failed",
            }
        )
        if not fired:
            add_finding(
                findings,
                "negative_drill.not_rejected",
                f"negative drill {drill_id} did not fire {expected}",
                "The recompute must reject this mutation.",
                ref=drill_id,
            )

    def find_binding(candidate: dict[str, Any], surface: str) -> dict[str, Any] | None:
        return next(
            (b for b in candidate["bindings"] if b.get("surface") == surface),
            None,
        )

    # 1. A binding that renders a different posture than the entry must be rejected.
    mutated = copy.deepcopy(register)
    target = find_binding(mutated, "docs")
    if target is not None:
        target["rendered_projection"]["posture"] = "state_declared"
        record(
            "binding_renders_wrong_posture",
            "binding.projection_posture_mismatch",
            "binding.projection_posture_mismatch" in check_ids(mutated),
        )

    # 2. A binding that renders a stronger effective posture than a quarantined
    #    entry must be rejected so an unsupported claim cannot be re-inflated.
    mutated = copy.deepcopy(register)
    target = next(
        (
            b
            for b in mutated["bindings"]
            if canonical.entries.get(b.get("consumes_entry_id"), {}).get("certification_state")
            == QUARANTINE_CERTIFICATION
        ),
        None,
    )
    if target is not None:
        target["rendered_projection"]["effective_posture"] = "certified_low_power"
        record(
            "unsupported_claim_reinflated",
            "binding.projection_effective_posture_mismatch",
            "binding.projection_effective_posture_mismatch" in check_ids(mutated),
        )

    # 3. A binding pointing at an unknown entry must be rejected.
    mutated = copy.deepcopy(register)
    target = find_binding(mutated, "docs")
    if target is not None:
        target["consumes_entry_id"] = "eff.does_not_exist"
        record(
            "unresolved_entry",
            "binding.entry_unresolved",
            "binding.entry_unresolved" in check_ids(mutated),
        )

    # 4. An About binding on a non-claim-bearing (quarantined) entry must be rejected.
    mutated = copy.deepcopy(register)
    quarantined_entry = next(
        (
            eid
            for eid, e in canonical.entries.items()
            if e.get("certification_state") == QUARANTINE_CERTIFICATION
        ),
        None,
    )
    target = find_binding(mutated, "about")
    if target is not None and quarantined_entry is not None:
        entry = canonical.entries[quarantined_entry]
        target["consumes_entry_id"] = quarantined_entry
        target["rendered_projection"] = {
            "efficiency_state": entry["efficiency_state"],
            "source_of_change": entry["source_of_change"],
            "posture": entry["posture"],
            "published_claim_ceiling": entry["published_claim_ceiling"],
            "effective_posture": entry["effective_posture"],
            "certification_state": entry["certification_state"],
            "claim_support": entry["claim_support"],
            "override_posture": entry["override_posture"],
            "recovery_state": entry["recovery_state"],
            "fired_narrowing_reasons": entry["fired_narrowing_reasons"],
        }
        record(
            "about_advertises_unsupported",
            "binding.surface_not_authorized",
            "binding.surface_not_authorized" in check_ids(mutated),
        )

    # 5. A binding disclosing a forbidden raw-telemetry field must be rejected.
    mutated = copy.deepcopy(register)
    target = find_binding(mutated, "support_export")
    if target is not None:
        target["disclosed_fields"] = target["disclosed_fields"] + ["raw_energy_trace"]
        record(
            "forbidden_field_disclosed",
            "binding.disclosed_field_forbidden",
            "binding.disclosed_field_forbidden" in check_ids(mutated),
        )

    # 6. A binding disclosing a field outside the export-safe allowlist must be rejected.
    mutated = copy.deepcopy(register)
    target = find_binding(mutated, "admin")
    if target is not None:
        target["disclosed_fields"] = target["disclosed_fields"] + ["internal_only_field"]
        record(
            "not_export_safe_field",
            "binding.disclosed_field_not_export_safe",
            "binding.disclosed_field_not_export_safe" in check_ids(mutated),
        )

    # 7. Dropping a required surface binding must be detected as a coverage gap.
    mutated = copy.deepcopy(register)
    before = len(mutated["bindings"])
    mutated["bindings"] = [b for b in mutated["bindings"] if b.get("surface") != "admin"]
    if len(mutated["bindings"]) != before:
        record(
            "coverage_gap",
            "coverage.missing_surface_binding",
            "coverage.missing_surface_binding" in check_ids(mutated),
        )

    # 8. A binding that admits it does not render verbatim from the entry must be rejected.
    mutated = copy.deepcopy(register)
    target = find_binding(mutated, "docs")
    if target is not None:
        target["renders_verbatim_from_entry"] = False
        record(
            "hand_written_prose",
            "binding.not_rendered_verbatim",
            "binding.not_rendered_verbatim" in check_ids(mutated),
        )

    # 9. A binding that renders the wrong override posture must be rejected.
    mutated = copy.deepcopy(register)
    target = next(
        (
            b
            for b in mutated["bindings"]
            if b.get("rendered_projection", {}).get("override_posture") != "policy_blocked"
        ),
        None,
    )
    if target is not None:
        target["rendered_projection"]["override_posture"] = "policy_blocked"
        record(
            "override_rule_not_propagated",
            "binding.projection_override_posture_mismatch",
            "binding.projection_override_posture_mismatch" in check_ids(mutated),
        )

    # 10. A binding that renders the wrong claim-support level must be rejected.
    mutated = copy.deepcopy(register)
    target = next(
        (
            b
            for b in mutated["bindings"]
            if b.get("rendered_projection", {}).get("claim_support") != "narrowed"
        ),
        None,
    )
    if target is not None:
        target["rendered_projection"]["claim_support"] = "narrowed"
        record(
            "claim_support_not_propagated",
            "binding.projection_claim_support_mismatch",
            "binding.projection_claim_support_mismatch" in check_ids(mutated),
        )

    # 11. A propagation projection that hides the quarantined entry must be rejected.
    mutated = copy.deepcopy(register)
    mutated["propagation_projection"]["quarantined_entry_ids"] = []
    record(
        "quarantine_hidden_from_projection",
        "propagation.quarantined_mismatch",
        "propagation.quarantined_mismatch" in check_ids(mutated),
    )

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
    register = load_json(repo_root / REGISTER_REL)
    if not isinstance(register, dict):
        raise SystemExit("register must be a JSON object")

    canonical = load_canonical(repo_root)
    validate_register(repo_root, register, canonical, findings)
    fixture_count = replay_fixtures(repo_root, register, canonical, findings)
    drill_results = run_negative_drills(repo_root, register, canonical, findings)

    errors = [f for f in findings if f.severity == "error"]
    warnings = [f for f in findings if f.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[efficiency-publication-ingestion] {status} "
        f"({len(errors)} errors, {len(warnings)} warnings) -- "
        f"bindings: {len(register.get('bindings', []))}, "
        f"entries: {len(canonical.entries)}, "
        f"fixtures: {fixture_count}, drills: {len(drill_results)}, "
        f"as_of: {register.get('as_of')}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[efficiency-publication-ingestion] {prefix} {finding.check_id}: {finding.message}{suffix}")
        print(f"[efficiency-publication-ingestion]   remediation: {finding.remediation}")

    if args.report:
        report = {
            "schema_version": 1,
            "check_id": "efficiency_publication_ingestion",
            "evaluated_on": register.get("as_of"),
            "status": "pass" if not errors else "fail",
            "register_ref": REGISTER_REL,
            "binding_count": len(register.get("bindings", [])),
            "entry_count": len(canonical.entries),
            "fixture_count": fixture_count,
            "drill_count": len(drill_results),
            "negative_drills": drill_results,
            "finding_counts": {"error": len(errors), "warning": len(warnings)},
            "findings": [f.as_report() for f in findings],
        }
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(
            json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )

    return 1 if errors else 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[efficiency-publication-ingestion] interrupted", file=sys.stderr)
        sys.exit(130)
