#!/usr/bin/env python3
"""Certify the M5 Problems / output / execution-evidence qualification capstone.

This is the release-automation companion to the Rust truth source at
``crates/aureline-runtime/src/certify_m5_problems_output_and_execution_evidence_truth``.
It binds the M5 Problems-row, output-channel, execution-evidence-projection,
chronology, and fallback-drill lanes into one promotion model: each claimed M5
tooling profile (Problems panel, output channel, terminal runner, debug console,
notebook output, pipeline overlay, AI-tool evidence, support export) is graded across
seven dimensions the source set treats as one causal chain — Problems correlation,
output-channel identity, evidence-projection lineage, causal-link integrity,
confidence honesty, stale/superseded handling, and reopen-to-origin parity.

The tool ingests the checked-in support export
(``artifacts/tooling/m5-problems-output-evidence-certification/support_export.json``)
and, per profile, **independently** re-derives the effective grade so the artifact can
never imply a wider claim than the current evidence backs:

* a broken Problems-correlation / output-channel-identity / projection-lineage /
  causal-link / confidence / stale-superseded / reopen invariant auto-narrows the
  profile to ``blocked``;
* a required dimension with no proof, or proof that fails closed, narrows to
  ``blocked``;
* a first-party (non-overlay) profile leaning on imported provider proof for a live
  claim narrows to ``blocked``;
* honestly labeled stale proof narrows to ``retest_pending`` while staying reopenable;
* a read-only overlay profile holds at ``limited``; a Labs profile makes no claim.

A narrowed profile must carry a strictly lower effective grade, a recorded trigger,
and a precise narrowed label. The four release-bearing integrity axes (causal-link
integrity, confidence honesty, stale/superseded handling, reopenable-evidence parity)
must each appear as an explicit release-evidence row consistent with the profiles.

Subcommands::

    validate    Re-derive from the support export and fail on any overclaim or drift
    corpus      Re-derive every checked-in perturbation fixture against its expected
    self-test   End-to-end: validate plus the corpus pass
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]

SUPPORT_EXPORT_REF = (
    "artifacts/tooling/m5-problems-output-evidence-certification/support_export.json"
)
SCHEMA_REF = "schemas/tooling/m5-problems-output-evidence-certification.schema.json"
DOC_REF = "docs/tooling/m5-problems-output-evidence-certification.md"
FIXTURE_DIR = "fixtures/tooling/m5-problems-output-evidence-certification"

RECORD_KIND = "m5_problems_output_evidence_certification_packet"
SCHEMA_VERSION = 1
TAXONOMY_VERSION = 1

REDACTION_TOKENS = {
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
}

UPSTREAM_LANE_REFS = [
    "artifacts/tooling/m5-execution-evidence/support_export.json",
    "artifacts/tooling/m5-problem-records/support_export.json",
    "artifacts/tooling/m5-execution-evidence-projections/support_export.json",
    "artifacts/tooling/m5-chronology-reuse/support_export.json",
    "artifacts/tooling/m5-output-channels/support_export.json",
    "artifacts/tooling/m5-fallback-evidence-drills/support_export.json",
]

PROFILES = [
    "problems_panel",
    "output_channel",
    "terminal_runner",
    "debug_console",
    "notebook_output",
    "pipeline_overlay",
    "ai_tool_evidence",
    "support_export",
]

# Dimensions are declared in priority order: the most severe broken invariant wins.
DIMENSIONS = [
    "problems_correlation",
    "output_channel_identity",
    "evidence_projection_lineage",
    "causal_link_integrity",
    "confidence_honesty",
    "stale_superseded_handling",
    "reopen_to_origin_parity",
]

DIMENSION_TRIGGER = {
    "problems_correlation": "problems_correlation_lost",
    "output_channel_identity": "output_channel_identity_flattened",
    "evidence_projection_lineage": "projection_lineage_flattened",
    "causal_link_integrity": "causal_link_broken",
    "confidence_honesty": "confidence_overclaimed",
    "stale_superseded_handling": "superseded_state_hidden",
    "reopen_to_origin_parity": "reopen_path_lost",
}

AXES = [
    ("causal_link_integrity", "causal_link_integrity"),
    ("confidence_honesty", "confidence_honesty"),
    ("stale_superseded_handling", "stale_superseded_handling"),
    ("reopenable_evidence_parity", "reopen_to_origin_parity"),
]

GRADE_RANK = {
    "labs_not_claimed": 0,
    "blocked": 1,
    "retest_pending": 2,
    "limited": 3,
    "qualified": 4,
}

CLAIMABLE_GRADES = {"qualified", "limited", "labs_not_claimed"}

OVERLAY_ORIGINS = {"remote_linked_run", "pipeline_provider_run", "imported_provider_evidence"}

GENERIC_LABELS = {
    "unavailable",
    "not available",
    "n/a",
    "error",
    "provider error",
    "request failed",
    "failed",
    "narrowed",
    "blocked",
    "limited",
}

FORBIDDEN_SUBSTRINGS = ("api_key", "password", "secret", "bearer ")


# --------------------------------------------------------------------------- #
# Derivation (mirrors the Rust derive_outcome / status_ceiling).
# --------------------------------------------------------------------------- #


def dimension_status(dim: dict, overlay: bool) -> str:
    if not dim["invariant_holds"]:
        return "failing"
    currency = dim["proof_currency"]
    if currency in ("missing_proof", "requires_review"):
        return "failing"
    if currency == "stale_expired":
        return "stale"
    if currency == "imported_current":
        return "current" if overlay else "failing"
    return "current"  # verified_current / cached_within_window


def status_ceiling(overlay: bool, dimensions: list, representative_freshness: str):
    present = {d["dimension"] for d in dimensions}
    for required in DIMENSIONS:
        if required not in present:
            return "blocked", "missing_dimension_proof"

    for dimension in DIMENSIONS:
        match = next((d for d in dimensions if d["dimension"] == dimension), None)
        if match is not None and not match["invariant_holds"]:
            return "blocked", DIMENSION_TRIGGER[dimension]

    if any(d["proof_currency"] in ("missing_proof", "requires_review") for d in dimensions):
        return "blocked", "missing_dimension_proof"

    if not overlay and any(d["proof_currency"] == "imported_current" for d in dimensions):
        return "blocked", "imported_overlay_claims_live"

    stale = any(dimension_status(d, overlay) == "stale" for d in dimensions) or (
        representative_freshness in ("stale_expired", "missing")
    )
    if stale:
        return "retest_pending", "stale_evidence"

    return "qualified", "stale_evidence"


def derive_outcome(profile: dict):
    """Returns (effective_grade, narrow_trigger_or_None)."""
    claimed = profile["claimed_grade"]
    if claimed == "labs_not_claimed":
        return "labs_not_claimed", None
    overlay = profile["origin_class"] in OVERLAY_ORIGINS
    ceiling, trigger = status_ceiling(
        overlay, profile["dimensions"], profile["representative_freshness"]
    )
    effective = ceiling if GRADE_RANK[ceiling] <= GRADE_RANK[claimed] else claimed
    if GRADE_RANK[effective] < GRADE_RANK[claimed]:
        return effective, trigger
    return effective, None


def dimension_backs_claim(dim: dict, overlay: bool) -> bool:
    if not dim["invariant_holds"]:
        return False
    currency = dim["proof_currency"]
    if currency in ("verified_current", "cached_within_window"):
        return True
    if currency == "imported_current" and overlay:
        return True
    return False


def derive_release_rows(profiles: list) -> list:
    claimed = [p for p in profiles if p["claimed_grade"] != "labs_not_claimed"]
    rows = []
    for axis, dimension in AXES:
        holding = 0
        for p in claimed:
            overlay = p["origin_class"] in OVERLAY_ORIGINS
            match = next((d for d in p["dimensions"] if d["dimension"] == dimension), None)
            if match is not None and dimension_backs_claim(match, overlay):
                holding += 1
        worst = "qualified"
        for p in claimed:
            if GRADE_RANK[p["effective_grade"]] < GRADE_RANK[worst]:
                worst = p["effective_grade"]
        rows.append(
            {
                "axis": axis,
                "dimension": dimension,
                "profiles_holding": holding,
                "profiles_claimed": len(claimed),
                "worst_effective_grade": worst,
            }
        )
    return rows


# --------------------------------------------------------------------------- #
# Validation.
# --------------------------------------------------------------------------- #


def _contains_forbidden(value) -> bool:
    if isinstance(value, str):
        lowered = value.lower()
        return any(token in lowered for token in FORBIDDEN_SUBSTRINGS)
    if isinstance(value, list):
        return any(_contains_forbidden(v) for v in value)
    if isinstance(value, dict):
        return any(_contains_forbidden(v) for v in value.values())
    return False


def _proof_reopenable(dim: dict) -> bool:
    ref = (dim.get("proof_ref") or "").strip()
    fp = (dim.get("proof_fingerprint_token") or "").strip()
    return bool(ref) and bool(fp) and ref != fp


def _dim_well_formed(dim: dict) -> bool:
    if not (dim.get("summary") or "").strip():
        return False
    if dim["proof_currency"] == "missing_proof":
        return dim.get("proof_ref") is None and dim.get("proof_fingerprint_token") is None
    return _proof_reopenable(dim)


def validate_packet(packet: dict) -> list:
    violations: list[str] = []

    if packet.get("record_kind") != RECORD_KIND:
        violations.append("wrong_record_kind")
    if packet.get("schema_version") != SCHEMA_VERSION:
        violations.append("wrong_schema_version")
    if packet.get("taxonomy_version") != TAXONOMY_VERSION:
        violations.append("wrong_taxonomy_version")
    for field in ("packet_id", "label", "as_of", "minted_at"):
        if not (packet.get(field) or "").strip():
            violations.append(f"missing_identity:{field}")
    if packet.get("redaction_class_token") not in REDACTION_TOKENS:
        violations.append("missing_identity:redaction_class_token")

    refs = set(packet.get("source_contract_refs", []))
    for required in (SCHEMA_REF, DOC_REF, SUPPORT_EXPORT_REF):
        if required not in refs:
            violations.append(f"missing_source_contract:{required}")

    upstream = set(packet.get("upstream_lane_refs", []))
    for required in UPSTREAM_LANE_REFS:
        if required not in upstream:
            violations.append(f"missing_upstream_lane_ref:{required}")

    profiles = packet.get("profiles", [])
    seen = {}
    for profile in profiles:
        seen[profile["profile"]] = seen.get(profile["profile"], 0) + 1
    for required in PROFILES:
        if required not in seen:
            violations.append(f"required_profile_missing:{required}")
    for token, count in seen.items():
        if count > 1:
            violations.append(f"duplicate_profile:{token}")

    narrowed_count = 0
    fully_qualified = 0
    overlay_count = 0
    for profile in profiles:
        violations.extend(_validate_profile(profile))
        overlay = profile["origin_class"] in OVERLAY_ORIGINS
        if overlay:
            overlay_count += 1
        if GRADE_RANK[profile["effective_grade"]] < GRADE_RANK[profile["claimed_grade"]]:
            narrowed_count += 1
        if profile["claimed_grade"] == "qualified" and profile["effective_grade"] == "qualified":
            fully_qualified += 1

    if narrowed_count == 0:
        violations.append("narrowed_profile_case_missing")
    if fully_qualified == 0:
        violations.append("current_profile_case_missing")
    if overlay_count == 0:
        violations.append("overlay_profile_case_missing")

    # Release-evidence rows must match the derivation.
    expected_rows = derive_release_rows(profiles)
    actual_rows = [
        {k: row[k] for k in ("axis", "dimension", "profiles_holding", "profiles_claimed", "worst_effective_grade")}
        for row in packet.get("release_evidence_rows", [])
    ]
    if actual_rows != expected_rows:
        violations.append("release_evidence_row_drift")

    if not all(packet.get("guardrails", {}).get(k) for k in GUARDRAIL_KEYS):
        violations.append("guardrails_incomplete")
    if not all(packet.get("consumer_surfaces", {}).get(k) for k in CONSUMER_KEYS):
        violations.append("consumer_surfaces_incomplete")
    freshness = packet.get("evidence_freshness", {})
    if (
        not isinstance(freshness.get("evidence_freshness_slo_hours"), int)
        or freshness.get("evidence_freshness_slo_hours", 0) < 1
        or not (freshness.get("last_evidence_refresh") or "").strip()
        or not freshness.get("auto_narrow_on_stale")
    ):
        violations.append("evidence_freshness_incomplete")

    if _contains_forbidden(packet):
        violations.append("raw_boundary_material_in_export")

    return violations


GUARDRAIL_KEYS = [
    "panes_never_flatten_identity",
    "structured_and_heuristic_origins_stay_distinct",
    "stale_and_superseded_state_stays_visible",
    "canonical_evidence_stays_reopenable",
    "imported_overlay_never_claims_live",
    "stale_or_failing_proof_auto_narrows",
    "primary_alert_summary_keeps_evidence_reopenable",
]

CONSUMER_KEYS = [
    "about_surface_ingests",
    "help_surface_ingests",
    "service_health_ingests",
    "compatibility_surface_ingests",
    "release_evidence_ingests",
    "support_export_ingests",
    "ai_evidence_ingests",
    "narrowed_profiles_labeled_below_claim",
]


def _validate_profile(profile: dict) -> list:
    token = profile.get("profile", "<unknown>")
    out = []
    if profile["claimed_grade"] not in CLAIMABLE_GRADES:
        out.append(f"claimed_grade_not_a_claim:{token}")
    dims = profile.get("dimensions", [])
    present = {d["dimension"] for d in dims}
    if present != set(DIMENSIONS) or len(dims) != len(DIMENSIONS):
        out.append(f"dimension_coverage_missing:{token}")
    for dim in dims:
        if not _dim_well_formed(dim):
            out.append(f"dimension_proof_not_reopenable:{token}:{dim['dimension']}")

    effective, trigger = derive_outcome(profile)
    if profile["effective_grade"] != effective:
        out.append(
            f"effective_grade_drift:{token}:{profile['effective_grade']}!={effective}"
        )
    if profile.get("narrow_trigger") != trigger:
        out.append(
            f"narrow_trigger_drift:{token}:{profile.get('narrow_trigger')}!={trigger}"
        )
    narrowed = GRADE_RANK[profile["effective_grade"]] < GRADE_RANK[profile["claimed_grade"]]
    if narrowed:
        label = (profile.get("narrowed_label") or "").strip()
        if not profile.get("narrow_trigger") or not label or label.lower() in GENERIC_LABELS:
            out.append(f"narrowed_profile_missing_label_or_trigger:{token}")
    if not profile.get("canonical_evidence_reopenable"):
        out.append(f"canonical_evidence_not_reopenable:{token}")
    if not profile.get("primary_alert_keeps_evidence_reopenable"):
        out.append(f"primary_alert_drops_evidence:{token}")
    if not profile.get("upstream_lane_refs") or not profile.get("evidence_refs"):
        out.append(f"profile_evidence_missing:{token}")
    return out


# --------------------------------------------------------------------------- #
# Corpus.
# --------------------------------------------------------------------------- #


def run_corpus(repo_root: Path) -> list:
    failures = []
    fixture_dir = repo_root / FIXTURE_DIR
    index = json.loads((fixture_dir / "index.json").read_text())
    for name in index["cases"]:
        case = json.loads((fixture_dir / name).read_text())
        profile = case["input"]
        effective, trigger = derive_outcome(profile)
        narrowed = GRADE_RANK[effective] < GRADE_RANK[profile["claimed_grade"]]
        expected = case["expected"]
        if effective != expected["effective_grade"]:
            failures.append(
                f"{case['case_id']}: effective {effective} != expected {expected['effective_grade']}"
            )
        if narrowed != expected["narrowed"]:
            failures.append(
                f"{case['case_id']}: narrowed {narrowed} != expected {expected['narrowed']}"
            )
        if trigger != expected.get("narrow_trigger"):
            failures.append(
                f"{case['case_id']}: trigger {trigger} != expected {expected.get('narrow_trigger')}"
            )
    return failures


# --------------------------------------------------------------------------- #
# CLI.
# --------------------------------------------------------------------------- #


def load_support_export(repo_root: Path) -> dict:
    return json.loads((repo_root / SUPPORT_EXPORT_REF).read_text())


def cmd_validate(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    violations = validate_packet(packet)
    if violations:
        sys.stderr.write("problems/output/evidence qualification FAILED validation:\n")
        for violation in violations:
            sys.stderr.write(f"  - {violation}\n")
        return 1
    narrowed = sum(
        1
        for p in packet["profiles"]
        if GRADE_RANK[p["effective_grade"]] < GRADE_RANK[p["claimed_grade"]]
    )
    sys.stdout.write(
        f"problems/output/evidence qualification OK: {len(packet['profiles'])} profiles, "
        f"{narrowed} narrowed, {len(packet['release_evidence_rows'])} release-evidence rows\n"
    )
    return 0


def cmd_corpus(repo_root: Path) -> int:
    failures = run_corpus(repo_root)
    if failures:
        sys.stderr.write("problems/output/evidence qualification corpus FAILED:\n")
        for failure in failures:
            sys.stderr.write(f"  - {failure}\n")
        return 1
    index = json.loads((repo_root / FIXTURE_DIR / "index.json").read_text())
    sys.stdout.write(
        f"problems/output/evidence qualification corpus OK: {len(index['cases'])} cases\n"
    )
    return 0


def cmd_self_test(repo_root: Path) -> int:
    return cmd_validate(repo_root) | cmd_corpus(repo_root)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["validate", "corpus", "self-test"])
    parser.add_argument("--repo-root", default=str(REPO_ROOT))
    args = parser.parse_args()
    repo_root = Path(args.repo_root).resolve()
    return {
        "validate": cmd_validate,
        "corpus": cmd_corpus,
        "self-test": cmd_self_test,
    }[args.command](repo_root)


if __name__ == "__main__":
    raise SystemExit(main())
