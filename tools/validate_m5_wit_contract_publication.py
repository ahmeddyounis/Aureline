#!/usr/bin/env python3
"""Validate the M5 extension-host WIT contract publication packet.

Validates:
- artifacts/contracts/m5-wit-contract-publication.json against
  schemas/public/m5-contracts/m5_wit_contract_publication.schema.json
- each standalone negotiation fixture under fixtures/contracts/m5-wit-negotiation/
  against the negotiation_fixture $def, and confirms it byte-matches the
  corresponding fixture embedded in the packet (no drift)
- the packet's semantic invariants (negotiated subset of offered subset of
  declared, no widening, no silent drop, fail-closed derivation, deprecated-world
  handling, and the per-outcome shape), mirroring the typed Rust consumer
- the capability-diff invariants (additive-minor adds-only / backward-compatible,
  deprecation carries a successor, breaking requires a guest upgrade)
- that the four required negotiation outcomes are covered
- that every referenced repo-relative path exists and every published WIT package
  file exists
- that the checked-in capability-diff Markdown matches the canonical packet
  (no hand-edit drift)

The validator imports the projection and compute helpers from the regenerator so
the two cannot drift.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

TOOLS_DIR = Path(__file__).resolve().parent
if str(TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(TOOLS_DIR))

import regenerate_m5_wit_contract_publication as gen  # noqa: E402

REPO_ROOT = gen.REPO_ROOT
SCHEMA_PATH = (
    REPO_ROOT
    / "schemas/public/m5-contracts/m5_wit_contract_publication.schema.json"
)
PACKET_PATH = gen.PACKET_PATH
DIFF_MD_PATH = gen.DIFF_MD_PATH
FIXTURES_DIR = gen.FIXTURES_DIR

# Refs that are world identities or matrix-row anchors, not file paths.
NON_PATH_PREFIXES = ("aureline:",)


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def is_path_ref(value: str) -> bool:
    if not value or value.startswith(NON_PATH_PREFIXES):
        return False
    # Strip a trailing "#anchor" fragment before checking existence.
    return "/" in value or value.endswith((".json", ".md", ".yaml", ".wit"))


def repo_path(ref: str) -> Path:
    return REPO_ROOT / ref.split("#", 1)[0]


def main() -> int:
    errors: list[str] = []

    packet = load_json(PACKET_PATH)

    # 1) Schema validation (best effort; skipped with a warning if jsonschema is
    #    not installed, exactly like the sibling validator).
    try:
        import jsonschema  # type: ignore

        schema = load_json(SCHEMA_PATH)
        validator_cls = jsonschema.validators.validator_for(schema)
        validator_cls.check_schema(schema)
        validator = validator_cls(schema)
        for err in sorted(validator.iter_errors(packet), key=lambda e: e.path):
            loc = "/".join(str(p) for p in err.path)
            errors.append(f"[schema] {loc}: {err.message}")
        # Validate each standalone fixture against the negotiation_fixture $def.
        fixture_schema = dict(schema)
        fixture_schema_ref = {
            "$schema": schema["$schema"],
            "$id": schema["$id"] + "#fixture",
            "$ref": "#/$defs/negotiation_fixture",
            "$defs": schema["$defs"],
        }
        fx_validator = validator_cls(fixture_schema_ref)
        for outcome in gen.NEGOTIATION_OUTCOMES:
            fx = load_json(FIXTURES_DIR / f"{outcome}.json")
            for err in fx_validator.iter_errors(fx):
                errors.append(f"[schema:{outcome}] {err.message}")
    except ImportError:
        print("[warn] jsonschema not installed; skipping JSON Schema validation")

    # 2) No-drift: regenerate in memory and compare to the checked-in files.
    rebuilt = gen.build_packet()
    if json.dumps(rebuilt, indent=2) != json.dumps(packet, indent=2):
        errors.append("[drift] packet does not match its regenerator output")

    rebuilt_md = gen.build_diff_markdown(rebuilt)
    if DIFF_MD_PATH.read_text(encoding="utf-8") != rebuilt_md:
        errors.append("[drift] capability-diff Markdown does not match regenerator")

    # 3) Standalone fixtures equal the embedded fixtures.
    embedded = {f["outcome"]: f for f in packet["negotiation_fixtures"]}
    for outcome in gen.NEGOTIATION_OUTCOMES:
        path = FIXTURES_DIR / f"{outcome}.json"
        if not path.exists():
            errors.append(f"[fixture] missing standalone fixture {path.name}")
            continue
        standalone = load_json(path)
        if standalone != embedded.get(outcome):
            errors.append(
                f"[fixture] {outcome}.json differs from the embedded packet fixture"
            )

    # 4) Required outcomes covered exactly once.
    outcomes = [f["outcome"] for f in packet["negotiation_fixtures"]]
    for required in gen.NEGOTIATION_OUTCOMES:
        if outcomes.count(required) != 1:
            errors.append(f"[outcome] {required} must appear exactly once")

    # 5) Semantic invariants per fixture (mirrors the typed Rust consumer).
    for fixture in packet["negotiation_fixtures"]:
        for issue in gen.fixture_issues(fixture):
            errors.append(f"[fixture:{fixture['outcome']}] {issue}")

    # 6) Capability-diff invariants.
    for diff in packet["capability_diffs"]:
        for issue in gen.diff_issues(diff):
            errors.append(f"[diff:{diff['diff_id']}] {issue}")

    # 7) Summary agrees with recomputation.
    if packet["summary"] != gen.compute_summary(packet):
        errors.append("[summary] recorded summary disagrees with recomputation")

    # 8) Every published WIT package file exists; every diff names published packages.
    package_identities = {p["package_identity"] for p in packet["packages"]}
    for pkg in packet["packages"]:
        if pkg["publication_state"] == "published":
            wit_path = REPO_ROOT / pkg["wit_package_ref"]
            if not wit_path.exists():
                errors.append(f"[wit] missing WIT file {pkg['wit_package_ref']}")
        for ref_field in ("predecessor_package_ref", "successor_package_ref"):
            ref = pkg[ref_field]
            if ref is not None and ref not in package_identities:
                errors.append(
                    f"[package:{pkg['package_identity']}] {ref_field} {ref} "
                    "is not a published package"
                )
    for diff in packet["capability_diffs"]:
        for ref_field in ("from_package_ref", "to_package_ref"):
            if diff[ref_field] not in package_identities:
                errors.append(
                    f"[diff:{diff['diff_id']}] {ref_field} {diff[ref_field]} "
                    "is not a published package"
                )

    # 9) Every referenced repo-relative path exists.
    for key in (
        "overview_page",
        "evidence_page",
        "schema_ref",
        "contract_matrix_ref",
        "capability_world_registry_ref",
        "negotiation_schema_ref",
        "adr_ref",
        "root_package_ref",
        "wit_index_ref",
        "capability_diff_report_ref",
    ):
        ref = packet[key]
        if is_path_ref(ref) and not repo_path(ref).exists():
            errors.append(f"[path] {key} -> missing {ref}")
    for pkg in packet["packages"]:
        for ref in (pkg["wit_package_ref"], pkg["registry_row_ref"]):
            if is_path_ref(ref) and not repo_path(ref).exists():
                errors.append(
                    f"[path] package {pkg['package_identity']} -> missing {ref}"
                )

    if errors:
        print("M5 WIT contract publication: FAIL")
        for err in errors:
            print(f"  - {err}")
        return 1

    summary = packet["summary"]
    print("M5 WIT contract publication: OK")
    print(
        f"  packages={summary['package_count']} "
        f"(deprecated={summary['deprecated_package_count']}), "
        f"fixtures={summary['negotiation_fixture_count']} "
        f"outcomes={','.join(summary['outcomes_covered'])}, "
        f"diffs={summary['capability_diff_count']}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
