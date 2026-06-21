#!/usr/bin/env python3
"""Freeze and certify the M5 execution-evidence projection set: coverage,
flaky-test history, perf-regression notes, notebook-output verdicts, pipeline
annotations, and review-side markers projected away from their original run
surface with run/step/provider/artifact lineage intact.

Where ``tools/release/execution_evidence_causality.py`` certifies the *lane*
matrix (one row per Problems/output/execution-evidence surface family) and
``tools/release/problem_records_causality.py`` certifies the *individual Problems
row*, this tool certifies the *individual projected overlay*. The canonical truth
is the checked-in support export
(``artifacts/tooling/m5-execution-evidence-projections/support_export.json``). Each
projection binds an overlay to the original run/step/provider/artifact lineage, the
revision-remap quality that maps origin anchors onto the current revision/cursor,
the evidence freshness/stale/superseded state, the confidence tier, and the
reopen-to-origin target.

This tool ingests that set and, per projection, **independently** re-derives an
effective claim that never reads wider than the evidence supports:

* origin run/step and provider/artifact identity stay reopenable on demand on every
  rendering surface;
* the revision-remap quality and freshness state stay labelled, and a stale/unmapped
  anchor reads as not-on-current-revision rather than silently current;
* imported/remote/pipeline origins project read-only and never claim live local
  authority, and a rendering surface never renders wider than the effective claim;
* a projection that flattens lineage, hides it from a surface, drops a heuristic
  backlink, loses its reopen path, or lets a surface overclaim floors to a
  raw-output / keyboard fallback rather than a clean-but-false overlay.

The Rust truth source is
``crates/aureline-runtime/src/m5_execution_evidence_projection_overlays``; this tool
re-derives the same effective claim and narrowing reasons so the checked-in artifacts
can never imply a wider claim than the current evidence backs.

Subcommands::

    validate     Re-derive from the support export and fail on any overclaim
    corpus       Run the narrowing engine over the checked-in fixture corpus
    emit-corpus  Regenerate the fixture corpus from the embedded case list
    self-test    End-to-end: validate plus the corpus pass
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
SUPPORT_EXPORT_REF = "artifacts/tooling/m5-execution-evidence-projections/support_export.json"
REPORT_REF = "artifacts/tooling/m5-execution-evidence-projections/report.md"
SCHEMA_REF = "schemas/tooling/m5-execution-evidence-projections.schema.json"
FIXTURE_DIR = "fixtures/tooling/m5-execution-evidence-projections"

RECORD_KIND = "m5_execution_evidence_projection_set_packet"
SCHEMA_VERSION = 1
TAXONOMY_VERSION = 1

OVERLAY_ORIGINS = {
    "remote_linked_run",
    "pipeline_provider_run",
    "imported_provider_evidence",
}
HEURISTIC_TIERS = {"heuristic_high", "heuristic_medium", "heuristic_low"}
REDACTION_TOKENS = {
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
}

PROJECTION_KINDS = {
    "coverage_overlay",
    "flaky_test_history",
    "perf_regression_note",
    "notebook_output_verdict",
    "pipeline_annotation",
    "review_side_marker",
}
PROJECTION_SURFACES = {
    "editor_overlay",
    "diff_review_overlay",
    "notebook_overlay",
    "pipeline_overlay",
    "incident_overlay",
    "timeline_history",
    "support_export",
    "ai_evidence",
}

LABS_CLAIM = "projection_labs_not_claimed"
CLAIM_RANK = {
    "projection_unreconstructable": 0,
    "projection_read_only_overlay": 1,
    "projection_narrowed": 2,
    "projection_certified": 3,
}

# Floor reasons break the contract outright rather than merely aging out.
FLOOR_REASONS = {
    "origin_run_step_flattened",
    "provider_artifact_flattened",
    "lineage_not_visible",
    "raw_output_backlink_missing",
    "reopen_target_lost",
    "surface_overclaims",
    "imported_overlay_claims_live",
    "evidence_missing",
}

# Deterministic reason ordering (mirrors the Rust order_index).
REASON_ORDER = {
    "origin_run_step_flattened": 0,
    "provider_artifact_flattened": 1,
    "lineage_not_visible": 2,
    "reopen_target_lost": 3,
    "raw_output_backlink_missing": 4,
    "surface_overclaims": 5,
    "imported_overlay_claims_live": 6,
    "evidence_missing": 7,
    "remap_quality_unlabeled": 8,
    "stale_remap_unlabeled": 9,
    "freshness_unlabeled": 10,
    "confidence_unlabeled": 11,
    "superseded_state_not_marked": 12,
    "evidence_stale": 13,
    "verification_proof_stale": 14,
    "verification_proof_missing": 15,
}

FORBIDDEN_SUBSTRINGS = ("api_key", "password", "secret", "bearer ")


def present(value) -> bool:
    return isinstance(value, str) and value.strip() != ""


def order_reasons(reasons: list[str]) -> list[str]:
    seen: list[str] = []
    for reason in sorted(reasons, key=lambda r: REASON_ORDER.get(r, 99)):
        if reason not in seen:
            seen.append(reason)
    return seen


def overclaims(effective: str, rendered: str) -> bool:
    er = CLAIM_RANK.get(effective)
    rr = CLAIM_RANK.get(rendered)
    if er is not None and rr is not None:
        return rr > er
    return effective != rendered


def claimed_claim(proj: dict) -> str:
    if proj["claim_posture"] == "labs_unadvertised":
        return LABS_CLAIM
    if proj["origin_class"] in OVERLAY_ORIGINS:
        return "projection_read_only_overlay"
    return "projection_certified"


def intrinsic_reasons(proj: dict, stale_window: bool) -> list[str]:
    integ = proj["integrity"]
    rr = proj["revision_remap"]
    ver = proj["verification"]
    overlay = proj["origin_class"] in OVERLAY_ORIGINS
    reasons: list[str] = []

    if not integ["preserves_origin_run_step"]:
        reasons.append("origin_run_step_flattened")
    if not integ["preserves_provider_artifact"]:
        reasons.append("provider_artifact_flattened")
    if not integ["lineage_visible_on_demand"] or any(
        not r["lineage_visible"] for r in proj["renderings"]
    ):
        reasons.append("lineage_not_visible")

    if proj["declared_confidence_tier"] in HEURISTIC_TIERS and not integ["raw_output_backlink_present"]:
        reasons.append("raw_output_backlink_missing")
    if not integ["confidence_label_visible"]:
        reasons.append("confidence_unlabeled")

    if not rr["remap_quality_labeled"]:
        reasons.append("remap_quality_unlabeled")
    if rr["quality"] == "stale_unmapped" and rr["anchored_to_current_revision"]:
        reasons.append("stale_remap_unlabeled")

    if not integ["freshness_state_labeled"]:
        reasons.append("freshness_unlabeled")

    if proj["declared_reopen_target"] == "none_keyboard_fallback":
        reasons.append("reopen_target_lost")

    fs = proj["declared_freshness_state"]
    if fs == "missing":
        reasons.append("evidence_missing")
    elif fs == "superseded_by_newer_run" and not integ["superseded_state_marked"]:
        reasons.append("superseded_state_not_marked")
    elif fs == "stale_expired" and not overlay:
        reasons.append("evidence_stale")

    pc = ver["proof_currency"]
    if pc == "missing_proof":
        reasons.append("verification_proof_missing")
    elif pc in ("stale_expired", "requires_review"):
        reasons.append("verification_proof_stale")
    elif pc in ("verified_current", "cached_within_window") and stale_window:
        reasons.append("verification_proof_stale")

    if overlay and not integ["imported_overlay_read_only"]:
        reasons.append("imported_overlay_claims_live")

    return reasons


def derive_effective(claimed: str, reasons: list[str]) -> str:
    if any(r in FLOOR_REASONS for r in reasons):
        return "projection_unreconstructable"
    if not reasons:
        return claimed
    if claimed == "projection_read_only_overlay":
        return "projection_unreconstructable"
    return "projection_narrowed"


def projection_reasons(proj: dict, stale_window: bool) -> list[str]:
    claimed = claimed_claim(proj)
    reasons = intrinsic_reasons(proj, stale_window)
    intrinsic = derive_effective(claimed, reasons)
    if any(overclaims(intrinsic, r["rendered_claim"]) for r in proj["renderings"]):
        reasons.append("surface_overclaims")
    return order_reasons(reasons)


def narrow(proj: dict, stale_window: bool) -> dict:
    claimed = claimed_claim(proj)
    if claimed == LABS_CLAIM:
        return {
            "claimed": LABS_CLAIM,
            "effective": LABS_CLAIM,
            "reasons": [],
            "narrowed": False,
        }
    reasons = projection_reasons(proj, stale_window)
    effective = derive_effective(claimed, reasons)
    er = CLAIM_RANK.get(effective)
    cr = CLAIM_RANK.get(claimed)
    narrowed = er is not None and cr is not None and er < cr
    return {
        "claimed": claimed,
        "effective": effective,
        "reasons": reasons,
        "narrowed": narrowed,
    }


def floored_keeps_fallback(proj: dict, effective: str) -> bool:
    if effective != "projection_unreconstructable":
        return True
    if proj["declared_reopen_target"] in ("raw_output_backlink", "none_keyboard_fallback"):
        return True
    if proj["integrity"]["raw_output_backlink_present"]:
        return True
    return present(proj["lineage"].get("raw_output_backlink_ref"))


def surface_overclaims(proj: dict, effective: str) -> bool:
    return any(overclaims(effective, r["rendered_claim"]) for r in proj["renderings"])


def contains_forbidden(value) -> bool:
    if isinstance(value, str):
        low = value.lower()
        return any(sub in low for sub in FORBIDDEN_SUBSTRINGS)
    if isinstance(value, list):
        return any(contains_forbidden(v) for v in value)
    if isinstance(value, dict):
        return any(contains_forbidden(v) for v in value.values())
    return False


def load_support_export(repo_root: Path) -> dict:
    return json.loads((repo_root / SUPPORT_EXPORT_REF).read_text(encoding="utf-8"))


def validate_packet(packet: dict) -> list[str]:
    v: list[str] = []
    if packet.get("record_kind") != RECORD_KIND:
        v.append("wrong_record_kind")
    if packet.get("schema_version") != SCHEMA_VERSION:
        v.append("wrong_schema_version")
    if packet.get("taxonomy_version") != TAXONOMY_VERSION:
        v.append("wrong_taxonomy_version")
    for key in ("packet_id", "label", "as_of", "redaction_class_token"):
        if not present(packet.get(key)):
            v.append("missing_identity")
            break
    if packet.get("redaction_class_token") not in REDACTION_TOKENS:
        v.append("invalid_redaction_class")
    vf = packet.get("verification_freshness", {})
    if vf.get("verification_freshness_slo_hours", 0) < 1 or not present(
        vf.get("last_verification_refresh")
    ):
        v.append("evidence_freshness_incomplete")
    projections = packet.get("projections", [])
    if not projections:
        v.append("empty_projections")

    seen: set[str] = set()
    kinds: set[str] = set()
    surfaces: set[str] = set()
    demonstrates_narrowing = False
    stale_window = False  # canonical packet is evaluated at its own as_of

    for proj in projections:
        pid = proj.get("projection_id", "")
        if pid in seen:
            v.append("duplicate_projection_id")
        seen.add(pid)
        kinds.add(proj.get("projection_kind"))
        for r in proj.get("renderings", []):
            surfaces.add(r.get("surface"))

        if (
            not present(proj.get("projection_id"))
            or not present(proj.get("label_summary"))
            or not present(proj.get("lineage", {}).get("execution_context_ref"))
        ):
            v.append("projection_missing_identity")
        if proj.get("origin_class") in OVERLAY_ORIGINS and not present(
            proj.get("lineage", {}).get("provider_ref")
        ):
            v.append("overlay_missing_provider_ref")
        if not proj.get("renderings"):
            v.append("projection_missing_rendering")
        for r in proj.get("renderings", []):
            if not present(r.get("source_projection_ref")):
                v.append("rendering_missing_source_ref")

        decision = narrow(proj, stale_window)
        if decision["narrowed"]:
            demonstrates_narrowing = True
            if not decision["reasons"]:
                v.append("narrowed_projection_missing_label_or_trigger")
        if not floored_keeps_fallback(proj, decision["effective"]):
            v.append("floored_projection_loses_fallback")
        if surface_overclaims(proj, decision["effective"]):
            v.append("rendering_surface_overclaims")

    if kinds != PROJECTION_KINDS:
        v.append("projection_kind_missing")
    if not PROJECTION_SURFACES.issubset(surfaces):
        v.append("projection_surface_missing")
    if not demonstrates_narrowing:
        v.append("downgraded_projection_case_missing")
    if contains_forbidden(packet):
        v.append("raw_boundary_material_in_export")

    # de-duplicate while keeping order
    out: list[str] = []
    for item in v:
        if item not in out:
            out.append(item)
    return out


def claim_distribution(projections: list[dict]) -> dict:
    dist = {
        "certified": 0,
        "narrowed": 0,
        "overlay": 0,
        "unreconstructable": 0,
        "labs": 0,
    }
    bucket = {
        "projection_certified": "certified",
        "projection_narrowed": "narrowed",
        "projection_read_only_overlay": "overlay",
        "projection_unreconstructable": "unreconstructable",
        LABS_CLAIM: "labs",
    }
    for proj in projections:
        dist[bucket[narrow(proj, False)["effective"]]] += 1
    return dist


# --------------------------------------------------------------------------- #
# Override engine + perturbation corpus.
# --------------------------------------------------------------------------- #

_TOKEN = re.compile(r"^([A-Za-z_][A-Za-z0-9_]*)(?:\[(\*|\d+)\])?$")


def _set_path(node, parts: list[str], value) -> None:
    head, *rest = parts
    m = _TOKEN.match(head)
    if not m:
        raise SystemExit(f"bad override token: {head}")
    key, idx = m.group(1), m.group(2)
    if idx is None:
        if rest:
            _set_path(node[key], rest, value)
        else:
            node[key] = value
    elif idx == "*":
        if not rest:
            raise SystemExit(f"cannot assign scalar to a list via [*]: {head}")
        for elem in node[key]:
            _set_path(elem, rest, value)
    else:
        i = int(idx)
        if rest:
            _set_path(node[key][i], rest, value)
        else:
            node[key][i] = value


def apply_overrides(proj: dict, overrides: dict) -> dict:
    out = json.loads(json.dumps(proj))
    for dotted, value in overrides.items():
        _set_path(out, dotted.split("."), value)
    return out


def base_projection(projections: list[dict], pid: str) -> dict:
    for proj in projections:
        if proj["projection_id"] == pid:
            return proj
    raise SystemExit(f"base projection not found: {pid}")


P_COVERAGE = "projection:coverage-local-test:0001"
P_PERF = "projection:perf-regression-local-task:0001"
P_PIPELINE = "projection:pipeline-annotation-provider:0001"
P_IMPORTED = "projection:coverage-imported-provider:0001"
P_LABS = "projection:notebook-verdict-labs:0001"

UNREC = "projection_unreconstructable"
NARROW = "projection_narrowed"

# (case_id, base_id, overrides, stale_window, description, effective, narrowed, reasons)
CASES = [
    ("clean-certified", P_COVERAGE, {}, False,
     "A clean first-party coverage overlay anchored exactly to the current revision certifies.",
     "projection_certified", False, []),
    ("origin-run-step-flattened", P_COVERAGE,
     {"integrity.preserves_origin_run_step": False, "renderings[*].rendered_claim": UNREC}, False,
     "Flattening origin run/step identity floors the projection to a raw fallback.",
     UNREC, True, ["origin_run_step_flattened"]),
    ("provider-artifact-flattened", P_COVERAGE,
     {"integrity.preserves_provider_artifact": False, "renderings[*].rendered_claim": UNREC}, False,
     "Flattening provider/artifact identity floors the projection.",
     UNREC, True, ["provider_artifact_flattened"]),
    ("lineage-not-visible", P_COVERAGE,
     {"integrity.lineage_visible_on_demand": False, "renderings[*].rendered_claim": UNREC}, False,
     "Lineage that cannot be revealed on demand floors the projection.",
     UNREC, True, ["lineage_not_visible"]),
    ("surface-hides-lineage", P_COVERAGE,
     {"renderings[0].lineage_visible": False, "renderings[*].rendered_claim": UNREC}, False,
     "A single rendering surface that cannot reveal lineage floors the projection.",
     UNREC, True, ["lineage_not_visible"]),
    ("heuristic-no-backlink", P_COVERAGE,
     {"declared_confidence_tier": "heuristic_high", "integrity.raw_output_backlink_present": False,
      "renderings[*].rendered_claim": UNREC}, False,
     "A heuristic projection without a raw-output backlink floors to a raw fallback.",
     UNREC, True, ["raw_output_backlink_missing"]),
    ("reopen-target-lost", P_COVERAGE,
     {"declared_reopen_target": "none_keyboard_fallback", "renderings[*].rendered_claim": UNREC}, False,
     "Losing reopen-to-origin floors the projection but keeps the keyboard fallback.",
     UNREC, True, ["reopen_target_lost"]),
    ("surface-overclaims", P_COVERAGE,
     {"integrity.confidence_label_visible": False, "renderings[0].rendered_claim": "projection_certified",
      "renderings[1].rendered_claim": NARROW, "renderings[2].rendered_claim": NARROW}, False,
     "A narrowed projection whose surface still renders certified floors as an overclaim.",
     UNREC, True, ["surface_overclaims", "confidence_unlabeled"]),
    ("imported-overlay-claims-live", P_PIPELINE,
     {"integrity.imported_overlay_read_only": False, "renderings[*].rendered_claim": UNREC}, False,
     "A pipeline overlay claiming live local authority floors below the read-only overlay.",
     UNREC, True, ["imported_overlay_claims_live"]),
    ("evidence-missing", P_COVERAGE,
     {"declared_freshness_state": "missing", "renderings[*].rendered_claim": UNREC}, False,
     "Missing evidence floors the projection.",
     UNREC, True, ["evidence_missing"]),
    ("remap-quality-unlabeled", P_COVERAGE,
     {"revision_remap.remap_quality_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the revision-remap quality narrows a first-party projection but keeps it reopenable.",
     NARROW, True, ["remap_quality_unlabeled"]),
    ("stale-remap-claims-current", P_COVERAGE,
     {"revision_remap.quality": "stale_unmapped", "revision_remap.anchored_to_current_revision": True,
      "renderings[*].rendered_claim": NARROW}, False,
     "A stale/unmapped anchor that still claims the current revision narrows.",
     NARROW, True, ["stale_remap_unlabeled"]),
    ("freshness-unlabeled", P_COVERAGE,
     {"integrity.freshness_state_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the freshness state narrows the projection.",
     NARROW, True, ["freshness_unlabeled"]),
    ("confidence-unlabeled", P_COVERAGE,
     {"integrity.confidence_label_visible": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the confidence tier narrows the projection.",
     NARROW, True, ["confidence_unlabeled"]),
    ("superseded-not-marked", P_COVERAGE,
     {"declared_freshness_state": "superseded_by_newer_run", "integrity.superseded_state_marked": False,
      "renderings[*].rendered_claim": NARROW}, False,
     "An unmarked superseded run narrows the projection.",
     NARROW, True, ["superseded_state_not_marked"]),
    ("superseded-marked-visible", P_COVERAGE,
     {"declared_freshness_state": "superseded_by_newer_run"}, False,
     "A marked superseded run stays certified because the state is visible.",
     "projection_certified", False, []),
    ("first-party-stale", P_COVERAGE,
     {"declared_freshness_state": "stale_expired", "renderings[*].rendered_claim": NARROW}, False,
     "A first-party stale projection narrows rather than reading as fresh.",
     NARROW, True, ["evidence_stale"]),
    ("missing-proof", P_COVERAGE,
     {"verification.proof_currency": "missing_proof", "verification.proof_ref": None,
      "renderings[*].rendered_claim": NARROW}, False,
     "A missing verification proof narrows a first-party projection.",
     NARROW, True, ["verification_proof_missing"]),
    ("stale-window", P_COVERAGE,
     {"renderings[*].rendered_claim": NARROW}, True,
     "An elapsed verification window ages out a current proof to narrowed.",
     NARROW, True, ["verification_proof_stale"]),
    ("imported-overlay-cached-clean", P_IMPORTED, {}, False,
     "An imported coverage overlay showing a cached snapshot stays a read-only overlay, not narrowed.",
     "projection_read_only_overlay", False, []),
    ("overlay-any-gap-floors", P_PIPELINE,
     {"revision_remap.remap_quality_labeled": False, "renderings[*].rendered_claim": UNREC}, False,
     "An overlay with any non-floor gap drops below the read-only overlay rather than holding it.",
     UNREC, True, ["remap_quality_unlabeled"]),
    ("perf-stale-proof-narrows", P_PERF, {}, False,
     "The canonical perf-regression note narrows via a stale verification proof and stays reopenable.",
     NARROW, True, ["verification_proof_stale"]),
    ("labs-not-claimed", P_LABS, {}, False,
     "A Labs notebook-output verdict makes no public claim and is never widened or narrowed.",
     LABS_CLAIM, False, []),
]


def run_corpus_from_cases(projections: list[dict]) -> list[str]:
    failures: list[str] = []
    for case in CASES:
        case_id, base_id, overrides, stale_window, _desc, exp_eff, exp_narrowed, exp_reasons = case
        proj = apply_overrides(base_projection(projections, base_id), overrides)
        decision = narrow(proj, stale_window)
        if decision["effective"] != exp_eff:
            failures.append(f"{case_id}: effective {decision['effective']} != {exp_eff}")
        if decision["narrowed"] != exp_narrowed:
            failures.append(f"{case_id}: narrowed {decision['narrowed']} != {exp_narrowed}")
        if decision["reasons"] != exp_reasons:
            failures.append(f"{case_id}: reasons {decision['reasons']} != {exp_reasons}")
    return failures


def run_corpus_from_disk(repo_root: Path, projections: list[dict]) -> list[str]:
    out_dir = repo_root / FIXTURE_DIR
    index_path = out_dir / "index.json"
    if not index_path.exists():
        return [f"missing corpus index: {index_path}"]
    index = json.loads(index_path.read_text(encoding="utf-8"))
    failures: list[str] = []
    for filename in index["cases"]:
        payload = json.loads((out_dir / filename).read_text(encoding="utf-8"))
        case_id = payload["case_id"]
        proj = apply_overrides(
            base_projection(projections, payload["base_projection_id"]),
            payload["overrides"],
        )
        decision = narrow(proj, payload["stale_window"])
        exp = payload["expected"]
        if decision["effective"] != exp["effective_claim"]:
            failures.append(f"{case_id}: effective {decision['effective']} != {exp['effective_claim']}")
        if decision["narrowed"] != exp["narrowed"]:
            failures.append(f"{case_id}: narrowed {decision['narrowed']} != {exp['narrowed']}")
        if decision["reasons"] != exp["active_narrowing_reasons"]:
            failures.append(
                f"{case_id}: reasons {decision['reasons']} != {exp['active_narrowing_reasons']}"
            )
    return failures


def write_corpus(repo_root: Path) -> None:
    out_dir = repo_root / FIXTURE_DIR
    out_dir.mkdir(parents=True, exist_ok=True)
    case_files = []
    for case in CASES:
        case_id, base_id, overrides, stale_window, desc, exp_eff, exp_narrowed, exp_reasons = case
        payload = {
            "case_id": case_id,
            "kind": "narrowing",
            "description": desc,
            "base_projection_id": base_id,
            "stale_window": stale_window,
            "overrides": overrides,
            "expected": {
                "effective_claim": exp_eff,
                "narrowed": exp_narrowed,
                "active_narrowing_reasons": exp_reasons,
            },
        }
        filename = f"{case_id}.json"
        (out_dir / filename).write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
        case_files.append(filename)
    index = {
        "corpus_id": "m5-execution-evidence-projections-corpus:0001",
        "description": (
            "Perturbation corpus for the execution-evidence projection engine. Each case starts "
            "from a canonical projection, applies dotted-path overrides, and asserts the re-derived "
            "effective claim, narrowed flag, and ordered narrowing reasons."
        ),
        "source_set_ref": SUPPORT_EXPORT_REF,
        "cases": case_files,
    }
    (out_dir / "index.json").write_text(json.dumps(index, indent=2) + "\n", encoding="utf-8")


def cmd_validate(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    violations = validate_packet(packet)
    if violations:
        sys.stderr.write("projection set FAILED validation:\n")
        for v in violations:
            sys.stderr.write(f"  - {v}\n")
        return 1
    dist = claim_distribution(packet["projections"])
    sys.stdout.write(
        f"projection set OK: {len(packet['projections'])} projections, distribution {dist}\n"
    )
    return 0


def cmd_corpus(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    projections = packet["projections"]
    failures = run_corpus_from_cases(projections)
    failures += run_corpus_from_disk(repo_root, projections)
    if failures:
        sys.stderr.write("projection corpus FAILED:\n")
        for f in failures:
            sys.stderr.write(f"  - {f}\n")
        return 1
    sys.stdout.write(f"projection corpus OK: {len(CASES)} cases\n")
    return 0


def cmd_emit_corpus(repo_root: Path) -> int:
    write_corpus(repo_root)
    sys.stdout.write(f"wrote {len(CASES)} cases + index to {FIXTURE_DIR}\n")
    return 0


def cmd_self_test(repo_root: Path) -> int:
    rc = cmd_validate(repo_root)
    rc |= cmd_corpus(repo_root)
    return rc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "command",
        choices=["validate", "corpus", "emit-corpus", "self-test"],
    )
    parser.add_argument("--repo-root", default=str(REPO_ROOT))
    args = parser.parse_args()
    repo_root = Path(args.repo_root).resolve()
    return {
        "validate": cmd_validate,
        "corpus": cmd_corpus,
        "emit-corpus": cmd_emit_corpus,
        "self-test": cmd_self_test,
    }[args.command](repo_root)


if __name__ == "__main__":
    raise SystemExit(main())
