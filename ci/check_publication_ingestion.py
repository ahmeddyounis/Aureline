#!/usr/bin/env python3
"""Enforce the benchmark publication-ingestion register.

This gate makes the rule "every user-facing benchmark claim derives from the one
canonical publication entry" mechanically enforceable, so docs, in-product help,
the About surface, enterprise evaluation packs, and support exports cannot clone
benchmark prose that drifts from the actual claim. It validates the register
against its schema; resolves every binding's consumed entry against the shiproom
freshness ledger and its reproducibility pack against the public-comparison
register; asserts each binding renders the entry's posture, narrowed effective
claim, freshness state, downgrade label, and metric refs verbatim so a narrowed
or quarantined claim propagates to every surface in the same release train;
confirms the reproducibility pack publishes to the binding's surface; rejects any
disclosed field that is not export-safe or is on the forbidden denylist; checks
that every entry is covered by the surfaces it must reach; cross-checks the
register's narrowed/quarantined projection against the ledger; replays the
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


REGISTER_REL = "artifacts/benchmarks/publication-ingestion-register.json"
SCHEMA_REL = "schemas/benchmarks/publication-ingestion.schema.json"
LEDGER_REL = "artifacts/benchmarks/shiproom-benchmark-freshness.json"
PUBLIC_COMPARISON_REL = "artifacts/benchmarks/public-comparison-pack-register.json"
FIXTURE_REGISTER_REL = "fixtures/benchmarks/docs-ingestion/manifest.yaml"

CLAIM_BEARING_LEVELS = {"aureline_only_claim", "public_head_to_head_comparison"}
QUARANTINE_LEVEL = "quarantined_not_comparable"

# Surfaces every covered entry must reach regardless of posture.
ALWAYS_REQUIRED_SURFACES = ("docs", "help", "support_export")


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
# Canonical source indexes.
# --------------------------------------------------------------------------- #


@dataclass
class Canonical:
    entries: dict[str, dict[str, Any]]
    rank: dict[str, int]
    packs: dict[str, dict[str, Any]]
    pack_by_entry: dict[str, dict[str, Any]]


def load_canonical(repo_root: Path) -> Canonical:
    ledger = load_json(repo_root / LEDGER_REL)
    rank = {row["level"]: row["rank"] for row in ledger.get("claim_levels", [])}
    entries: dict[str, dict[str, Any]] = {}
    for entry in ledger.get("entries", []):
        if not isinstance(entry, dict):
            continue
        entry_id = entry.get("entry_id")
        if not isinstance(entry_id, str):
            continue
        blocker = entry.get("shiproom_blocker") or {}
        entries[entry_id] = {
            "posture": entry.get("posture"),
            "published_claim_ceiling": entry.get("published_claim_ceiling"),
            "effective_claim": entry.get("effective_claim"),
            "freshness_state": entry.get("freshness_state"),
            "downgrade_label": blocker.get("downgrade_label"),
            "metric_refs": list(entry.get("metric_refs") or []),
        }

    register = load_json(repo_root / PUBLIC_COMPARISON_REL)
    packs: dict[str, dict[str, Any]] = {}
    pack_by_entry: dict[str, dict[str, Any]] = {}
    for pack in register.get("packs", []):
        if not isinstance(pack, dict):
            continue
        pack_ref = pack.get("pack_ref")
        if not isinstance(pack_ref, str):
            continue
        packs[pack_ref] = pack
        gov = pack.get("governance_pack_ref")
        if isinstance(gov, str):
            pack_by_entry[gov] = pack

    return Canonical(entries=entries, rank=rank, packs=packs, pack_by_entry=pack_by_entry)


# --------------------------------------------------------------------------- #
# Per-binding admission.
# --------------------------------------------------------------------------- #


def reject_binding(
    record: dict[str, Any],
    canonical: Canonical,
    surface_req: dict[str, str],
    allow: set[str],
    forbid: set[str],
) -> str | None:
    """Return the rule that rejects a binding, or None when it is admitted.

    The order of the checks is the rejection precedence the fixtures assert.
    """
    if record.get("renders_verbatim_from_entry") is not True:
        return "not_rendered_verbatim"

    entry_id = record.get("consumes_entry_id")
    entry = canonical.entries.get(entry_id)
    if entry is None:
        return "entry_unresolved"

    pack = canonical.packs.get(record.get("repro_pack_ref"))
    if pack is None:
        return "repro_pack_unresolved"
    if pack.get("governance_pack_ref") != entry_id:
        return "repro_pack_entry_mismatch"

    surface = record.get("surface")
    required_repro = surface_req.get(surface)
    if required_repro is None or required_repro not in (pack.get("surfaces") or []):
        return "surface_not_authorized"

    disclosed = record.get("disclosed_fields") or []
    if any(name in forbid for name in disclosed):
        return "disclosed_field_forbidden"
    if any(name not in allow for name in disclosed):
        return "disclosed_field_not_export_safe"

    projection = record.get("rendered_projection") or {}
    if projection.get("posture") != entry["posture"]:
        return "projection_posture_mismatch"
    if projection.get("effective_claim") != entry["effective_claim"]:
        return "projection_effective_claim_mismatch"
    if projection.get("freshness_state") != entry["freshness_state"]:
        return "projection_freshness_mismatch"
    if projection.get("downgrade_label") != entry["downgrade_label"]:
        return "projection_downgrade_label_mismatch"
    if list(projection.get("metric_refs") or []) != list(entry["metric_refs"]):
        return "projection_metric_refs_mismatch"

    return None


_REMEDIATION = {
    "not_rendered_verbatim": "Render the entry's canonical values; set renders_verbatim_from_entry to true.",
    "entry_unresolved": "Point consumes_entry_id at a claim publication entry in the freshness ledger.",
    "repro_pack_unresolved": "Point repro_pack_ref at a pack in the public-comparison register.",
    "repro_pack_entry_mismatch": "Bind the reproducibility pack whose governance_pack_ref equals the consumed entry id.",
    "surface_not_authorized": "Only bind a surface the reproducibility pack publishes to.",
    "disclosed_field_forbidden": "Remove the forbidden field; raw run, machine, provider, corpus, competitor, and secret data never cross this boundary.",
    "disclosed_field_not_export_safe": "Disclose only fields in the register's export_safe_fields allowlist.",
    "projection_posture_mismatch": "Render the entry's posture verbatim.",
    "projection_effective_claim_mismatch": "Render the entry's narrowed effective claim, not its ceiling or a different level.",
    "projection_freshness_mismatch": "Render the entry's freshness state verbatim.",
    "projection_downgrade_label_mismatch": "Render the entry's downgrade label verbatim.",
    "projection_metric_refs_mismatch": "Render the entry's metric refs verbatim.",
}


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
    for key in ("freshness_ledger_ref", "governance_matrix_ref", "public_comparison_register_ref"):
        ref = register.get(key)
        if isinstance(ref, str) and not (repo_root / ref).exists():
            add_finding(
                findings,
                f"register.{key}.missing",
                f"register cites a missing artifact for {key}: {ref}",
                "Publish the referenced artifact or correct the ref.",
                ref=ref,
            )


def surface_requirements(register: dict[str, Any]) -> dict[str, str]:
    return {
        row["surface"]: row["requires_repro_surface"]
        for row in register.get("surface_kinds", [])
        if isinstance(row, dict) and "surface" in row and "requires_repro_surface" in row
    }


def validate_bindings(
    register: dict[str, Any],
    canonical: Canonical,
    surface_req: dict[str, str],
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

        rejected_by = reject_binding(binding, canonical, surface_req, allow, forbid)
        if rejected_by is not None:
            add_finding(
                findings,
                f"binding.{rejected_by}",
                f"binding {binding_id} is rejected by {rejected_by}",
                _REMEDIATION.get(rejected_by, "Correct the rejected binding."),
                ref=binding_id,
            )


def validate_surface_kinds(register: dict[str, Any], findings: list[Finding]) -> None:
    surfaces = {row.get("surface") for row in register.get("surface_kinds", [])}
    for required in ("docs", "help", "about", "evaluation_pack", "support_export"):
        if required not in surfaces:
            add_finding(
                findings,
                "register.surface_kind_missing",
                f"surface_kinds omits the {required} surface",
                "Declare every consuming surface in surface_kinds.",
                ref=REGISTER_REL,
            )


def required_surfaces_for_entry(
    entry: dict[str, Any],
    pack: dict[str, Any] | None,
    surface_req: dict[str, str],
) -> list[str]:
    if pack is None:
        return []
    repro_surfaces = set(pack.get("surfaces") or [])
    needed: list[str] = []
    for surface in ALWAYS_REQUIRED_SURFACES:
        if surface_req.get(surface) in repro_surfaces:
            needed.append(surface)
    if entry.get("posture") in CLAIM_BEARING_LEVELS and surface_req.get("about") in repro_surfaces:
        needed.append("about")
    if surface_req.get("evaluation_pack") in repro_surfaces:
        needed.append("evaluation_pack")
    return needed


def validate_coverage(
    register: dict[str, Any],
    canonical: Canonical,
    surface_req: dict[str, str],
    findings: list[Finding],
) -> None:
    bound: set[tuple[str, str]] = set()
    for binding in register.get("bindings", []):
        surface = binding.get("surface")
        entry_id = binding.get("consumes_entry_id")
        if isinstance(surface, str) and isinstance(entry_id, str):
            bound.add((surface, entry_id))

    for entry_id, entry in canonical.entries.items():
        pack = canonical.pack_by_entry.get(entry_id)
        if pack is None:
            add_finding(
                findings,
                "coverage.entry_without_repro_pack",
                f"ledger entry {entry_id} has no reproducibility pack to publish from",
                "Bind the entry to a public-comparison reproducibility pack.",
                ref=entry_id,
                severity="warning",
            )
            continue
        for surface in required_surfaces_for_entry(entry, pack, surface_req):
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
    def rank(level: str) -> int:
        return canonical.rank.get(level, 0)

    narrowed = sorted(
        eid
        for eid, e in canonical.entries.items()
        if rank(e.get("effective_claim")) < rank(e.get("published_claim_ceiling"))
    )
    quarantined = sorted(
        eid for eid, e in canonical.entries.items() if e.get("effective_claim") == QUARANTINE_LEVEL
    )

    projection = register.get("propagation_projection") or {}
    if sorted(projection.get("narrowed_entry_ids", [])) != narrowed:
        add_finding(
            findings,
            "propagation.narrowed_mismatch",
            "propagation narrowed_entry_ids disagrees with the ledger",
            "List exactly the entries whose effective claim is below their ceiling.",
            ref=REGISTER_REL,
            details={"recomputed": narrowed},
        )
    if sorted(projection.get("quarantined_entry_ids", [])) != quarantined:
        add_finding(
            findings,
            "propagation.quarantined_mismatch",
            "propagation quarantined_entry_ids disagrees with the ledger",
            "List exactly the entries whose effective claim is quarantined_not_comparable.",
            ref=REGISTER_REL,
            details={"recomputed": quarantined},
        )

    # A binding may never render a stronger claim than the entry it consumes; this
    # is the cross-surface guarantee that a narrowed claim propagates everywhere.
    for binding in register.get("bindings", []):
        entry = canonical.entries.get(binding.get("consumes_entry_id"))
        if entry is None:
            continue
        rendered = (binding.get("rendered_projection") or {}).get("effective_claim")
        if rendered is not None and rank(rendered) > rank(entry.get("effective_claim")):
            add_finding(
                findings,
                "propagation.binding_over_claims",
                f"binding {binding.get('binding_id')} renders {rendered} above the entry's {entry.get('effective_claim')}",
                "A surface must never publish a stronger claim than the entry's narrowed effective claim.",
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

    surface_req = surface_requirements(register)
    allow = {row["field"] for row in register.get("export_safe_fields", []) if "field" in row}
    forbid = {row["field"] for row in register.get("forbidden_fields", []) if "field" in row}

    validate_bindings(register, canonical, surface_req, allow, forbid, findings)
    validate_coverage(register, canonical, surface_req, findings)
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
    surface_req = surface_requirements(register)
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
            rejected_by = reject_binding(record, canonical, surface_req, allow, forbid)
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

    # 1. A binding that renders a different effective claim than the entry must be rejected.
    #    internal_gate_only is no entry's effective claim, so it always mismatches.
    mutated = copy.deepcopy(register)
    target = find_binding(mutated, "docs")
    if target is not None:
        target["rendered_projection"]["effective_claim"] = "internal_gate_only"
        record(
            "binding_over_or_under_claims",
            "binding.projection_effective_claim_mismatch",
            "binding.projection_effective_claim_mismatch" in check_ids(mutated),
        )

    # 2. A binding that renders a fresher state than a quarantined entry must be rejected.
    mutated = copy.deepcopy(register)
    target = next(
        (
            b
            for b in mutated["bindings"]
            if b.get("rendered_projection", {}).get("freshness_state") == "incomparable"
        ),
        None,
    )
    if target is not None:
        target["rendered_projection"]["freshness_state"] = "current"
        record(
            "stale_state_not_propagated",
            "binding.projection_freshness_mismatch",
            "binding.projection_freshness_mismatch" in check_ids(mutated),
        )

    # 3. A binding pointing at an unknown entry must be rejected.
    mutated = copy.deepcopy(register)
    target = find_binding(mutated, "docs")
    if target is not None:
        target["consumes_entry_id"] = "publication_pack.does_not_exist"
        record(
            "unresolved_entry",
            "binding.entry_unresolved",
            "binding.entry_unresolved" in check_ids(mutated),
        )

    # 4. A binding whose reproducibility pack backs a different entry must be rejected.
    mutated = copy.deepcopy(register)
    target = find_binding(mutated, "help")
    if target is not None:
        target["repro_pack_ref"] = "repro_pack.quarantined.legacy_first_paint"
        record(
            "repro_pack_entry_mismatch",
            "binding.repro_pack_entry_mismatch",
            "binding.repro_pack_entry_mismatch" in check_ids(mutated),
        )

    # 5. A binding disclosing a forbidden raw field must be rejected.
    mutated = copy.deepcopy(register)
    target = find_binding(mutated, "support_export")
    if target is not None:
        target["disclosed_fields"] = target["disclosed_fields"] + ["raw_run_log"]
        record(
            "forbidden_field_disclosed",
            "binding.disclosed_field_forbidden",
            "binding.disclosed_field_forbidden" in check_ids(mutated),
        )

    # 6. An evaluation-pack binding on an entry the pack does not publish to enterprise
    #    evaluation must be rejected.
    mutated = copy.deepcopy(register)
    target = find_binding(mutated, "evaluation_pack")
    if target is not None:
        target["consumes_entry_id"] = "publication_pack.methodology.startup_warm_to_first_paint"
        target["repro_pack_ref"] = "repro_pack.methodology.startup_warm_to_first_paint"
        record(
            "surface_not_authorized",
            "binding.surface_not_authorized",
            "binding.surface_not_authorized" in check_ids(mutated),
        )

    # 7. Dropping a required surface binding must be detected as a coverage gap.
    mutated = copy.deepcopy(register)
    before = len(mutated["bindings"])
    mutated["bindings"] = [b for b in mutated["bindings"] if b.get("surface") != "docs"]
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

    # 9. A propagation projection that hides the quarantined entry must be rejected.
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
        f"[publication-ingestion] {status} "
        f"({len(errors)} errors, {len(warnings)} warnings) -- "
        f"bindings: {len(register.get('bindings', []))}, "
        f"entries: {len(canonical.entries)}, "
        f"fixtures: {fixture_count}, drills: {len(drill_results)}, "
        f"as_of: {register.get('as_of')}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[publication-ingestion] {prefix} {finding.check_id}: {finding.message}{suffix}")
        print(f"[publication-ingestion]   remediation: {finding.remediation}")

    if args.report:
        report = {
            "schema_version": 1,
            "check_id": "publication_ingestion",
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
        print("[publication-ingestion] interrupted", file=sys.stderr)
        sys.exit(130)
