#!/usr/bin/env python3
"""Freeze and certify the M5 structured-native versus heuristic-fallback proof corpus:
one parse-evidence case — a native structured diagnostic, a normalized task event, an
imported provider annotation, or a heuristic text parse — exercised through a failure
drill (native baseline, malformed output, stale run, superseded retry, reconnect, lost
channel, partial export, imported evidence, or output-channel virtualization) and
rendered onto the claimed M5 tooling profiles (Problems panel, output channel, terminal
runner, debug console, notebook output, pipeline overlay, AI-tool evidence, support
export).

Where ``tools/release/execution_evidence_causality.py`` certifies the *lane* matrix,
``tools/release/problem_records_causality.py`` certifies the *individual Problems row*,
``tools/release/execution_evidence_projections.py`` certifies the *projected overlay*,
and ``tools/release/chronology_reuse.py`` certifies the *chronology entry*, this tool
certifies the *parse-evidence drill case*. The canonical truth is the checked-in support
export (``artifacts/tooling/m5-fallback-evidence-drills/support_export.json``).

This tool ingests that set and, per case, **independently** re-derives an effective claim
that never reads wider than the evidence supports:

* a heuristic fallback reads visibly distinct from native/structured evidence on every
  claimed profile and keeps a raw-output backlink;
* the problem-source class, run/step/provider/channel lineage, and stable channel id stay
  reopenable on demand on every profile, and the freshness/confidence labels stay visible;
* a reconnect or lost-channel drill never drops evidence, a partial export stays
  reviewable without the originating UI, and an imported/remote/pipeline origin never
  claims live local authority;
* stale, superseded, and missing freshness states and stale/missing verification proofs
  narrow a first-party case rather than reading as fresh; an output-channel virtualization
  drill that loses stream-first paging, search, or copy/export narrows;
* a case that flattens its source class, lets a heuristic read as structured, flattens
  lineage, diverges its canonical ids, drops a heuristic backlink, loses its reopen path,
  drops evidence on reconnect, exports a non-self-contained slice, lets a profile
  overclaim, or lets an imported origin claim live floors to a raw-output / keyboard
  fallback rather than rendering a clean-but-false row.

The Rust truth source is
``crates/aureline-runtime/src/m5_structured_versus_heuristic_fallback_drills``; this tool
re-derives the same effective claim and narrowing reasons so the checked-in artifacts can
never imply a wider claim than the current evidence backs.

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
SUPPORT_EXPORT_REF = "artifacts/tooling/m5-fallback-evidence-drills/support_export.json"
REPORT_REF = "artifacts/tooling/m5-fallback-evidence-drills/report.md"
SCHEMA_REF = "schemas/tooling/m5-fallback-evidence-drills.schema.json"
FIXTURE_DIR = "fixtures/tooling/m5-fallback-evidence-drills"

RECORD_KIND = "m5_fallback_evidence_drill_set_packet"
SCHEMA_VERSION = 1
TAXONOMY_VERSION = 1

OVERLAY_ORIGINS = {
    "remote_linked_run",
    "pipeline_provider_run",
    "imported_provider_evidence",
}
HEURISTIC_TIERS = {"heuristic_high", "heuristic_medium", "heuristic_low"}
HEURISTIC_SOURCES = {"heuristic_output_parse"}
REDACTION_TOKENS = {
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
}

REQUIRED_SOURCES = {
    "structured_language_diagnostic",
    "normalized_task_event",
    "heuristic_output_parse",
    "imported_provider_annotation",
}
REAL_CHANNELS_EXCLUDED = {"not_applicable"}
DRILL_KINDS = {
    "native_structured",
    "normalized_task_event",
    "heuristic_text_parse",
    "imported_evidence",
    "malformed_output",
    "stale_run",
    "superseded_retry",
    "reconnect",
    "lost_channel",
    "partial_export",
    "channel_virtualization",
}
PROFILES = {
    "problems_panel",
    "output_channel",
    "terminal_runner",
    "debug_console",
    "notebook_output",
    "pipeline_overlay",
    "ai_tool_evidence",
    "support_export",
}

LABS_CLAIM = "fallback_labs_not_claimed"
CLAIM_RANK = {
    "fallback_unreconstructable": 0,
    "fallback_read_only_overlay": 1,
    "fallback_narrowed": 2,
    "fallback_certified": 3,
}

# Floor reasons break the contract outright rather than merely aging out.
FLOOR_REASONS = {
    "source_kind_flattened",
    "heuristic_indistinct_from_structured",
    "run_channel_lineage_flattened",
    "channel_identity_flattened",
    "canonical_id_divergence",
    "raw_output_backlink_missing",
    "reopen_target_lost",
    "reconnect_drops_evidence",
    "partial_export_incomplete",
    "surface_overclaims",
    "imported_overlay_claims_live",
    "evidence_missing",
}

# Deterministic reason ordering (mirrors the Rust order_index).
REASON_ORDER = {
    "source_kind_flattened": 0,
    "heuristic_indistinct_from_structured": 1,
    "run_channel_lineage_flattened": 2,
    "channel_identity_flattened": 3,
    "canonical_id_divergence": 4,
    "raw_output_backlink_missing": 5,
    "reopen_target_lost": 6,
    "reconnect_drops_evidence": 7,
    "partial_export_incomplete": 8,
    "surface_overclaims": 9,
    "imported_overlay_claims_live": 10,
    "evidence_missing": 11,
    "confidence_unlabeled": 12,
    "freshness_unlabeled": 13,
    "superseded_state_not_marked": 14,
    "virtualization_not_stream_first": 15,
    "search_unavailable": 16,
    "copy_export_unavailable": 17,
    "evidence_stale": 18,
    "verification_proof_stale": 19,
    "verification_proof_missing": 20,
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


def is_heuristic(case: dict) -> bool:
    return (
        case["problem_source_kind"] in HEURISTIC_SOURCES
        or case["declared_confidence_tier"] in HEURISTIC_TIERS
    )


def is_real_channel(case: dict) -> bool:
    return case["output_channel_class"] not in REAL_CHANNELS_EXCLUDED


def claimed_claim(case: dict) -> str:
    if case["claim_posture"] == "labs_unadvertised":
        return LABS_CLAIM
    if case["origin_class"] in OVERLAY_ORIGINS:
        return "fallback_read_only_overlay"
    return "fallback_certified"


def _refs_diverge(a, b) -> bool:
    sa = a.strip() if isinstance(a, str) else None
    sb = b.strip() if isinstance(b, str) else None
    if sa and sb:
        return sa != sb
    return False


def has_canonical_id_divergence(case: dict) -> bool:
    links = case["links"]
    return any(
        _refs_diverge(p.get("bound_run_ref"), links.get("run_ref"))
        or _refs_diverge(p.get("bound_channel_ref"), links.get("channel_ref"))
        or _refs_diverge(p.get("bound_problem_ref"), links.get("problem_ref"))
        for p in case["profiles"]
    )


def intrinsic_reasons(case: dict, stale_window: bool) -> list[str]:
    integ = case["integrity"]
    virt = case["virtualization"]
    ver = case["verification"]
    heuristic = is_heuristic(case)
    overlay = case["origin_class"] in OVERLAY_ORIGINS
    reasons: list[str] = []

    if not integ["preserves_source_kind"]:
        reasons.append("source_kind_flattened")
    if heuristic and (
        not integ["heuristic_visibly_distinct_from_structured"]
        or any(not p["fallback_visibly_distinct"] for p in case["profiles"])
    ):
        reasons.append("heuristic_indistinct_from_structured")

    if not integ["preserves_run_channel_lineage"] or any(
        not p["lineage_visible"] for p in case["profiles"]
    ):
        reasons.append("run_channel_lineage_flattened")
    if is_real_channel(case) and not integ["channel_identity_stable"]:
        reasons.append("channel_identity_flattened")
    if has_canonical_id_divergence(case):
        reasons.append("canonical_id_divergence")

    if heuristic and not integ["raw_output_backlink_present"]:
        reasons.append("raw_output_backlink_missing")
    if not integ["confidence_label_visible"]:
        reasons.append("confidence_unlabeled")
    if not integ["freshness_state_labeled"]:
        reasons.append("freshness_unlabeled")

    if case["declared_reopen_target"] == "none_keyboard_fallback":
        reasons.append("reopen_target_lost")

    if not integ["reconnect_preserves_evidence"]:
        reasons.append("reconnect_drops_evidence")
    if not integ["partial_export_self_contained"]:
        reasons.append("partial_export_incomplete")

    if not virt["stream_first"] or not virt["bounded_memory"]:
        reasons.append("virtualization_not_stream_first")
    if not virt["searchable"]:
        reasons.append("search_unavailable")
    if not virt["copy_exportable"]:
        reasons.append("copy_export_unavailable")

    fs = case["declared_freshness_state"]
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

    if overlay and not integ["imported_evidence_read_only"]:
        reasons.append("imported_overlay_claims_live")

    return reasons


def derive_effective(claimed: str, reasons: list[str]) -> str:
    if any(r in FLOOR_REASONS for r in reasons):
        return "fallback_unreconstructable"
    if not reasons:
        return claimed
    if claimed == "fallback_read_only_overlay":
        return "fallback_unreconstructable"
    return "fallback_narrowed"


def case_reasons(case: dict, stale_window: bool) -> list[str]:
    claimed = claimed_claim(case)
    reasons = intrinsic_reasons(case, stale_window)
    intrinsic = derive_effective(claimed, reasons)
    if any(overclaims(intrinsic, p["rendered_claim"]) for p in case["profiles"]):
        reasons.append("surface_overclaims")
    return order_reasons(reasons)


def narrow(case: dict, stale_window: bool) -> dict:
    claimed = claimed_claim(case)
    if claimed == LABS_CLAIM:
        return {
            "claimed": LABS_CLAIM,
            "effective": LABS_CLAIM,
            "reasons": [],
            "narrowed": False,
        }
    reasons = case_reasons(case, stale_window)
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


def floored_keeps_fallback(case: dict, effective: str) -> bool:
    if effective != "fallback_unreconstructable":
        return True
    if case["declared_reopen_target"] in ("raw_output_backlink", "none_keyboard_fallback"):
        return True
    if case["integrity"]["raw_output_backlink_present"]:
        return True
    return present(case["links"].get("raw_output_backlink_ref"))


def surface_overclaims(case: dict, effective: str) -> bool:
    return any(overclaims(effective, p["rendered_claim"]) for p in case["profiles"])


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
    cases = packet.get("cases", [])
    if not cases:
        v.append("empty_cases")

    seen: set[str] = set()
    sources: set[str] = set()
    drills: set[str] = set()
    profiles: set[str] = set()
    heuristic_present = False
    demonstrates_narrowing = False
    stale_window = False  # canonical packet is evaluated at its own as_of

    for case in cases:
        cid = case.get("case_id", "")
        if cid in seen:
            v.append("duplicate_case_id")
        seen.add(cid)
        sources.add(case.get("problem_source_kind"))
        drills.add(case.get("drill_kind"))
        for p in case.get("profiles", []):
            profiles.add(p.get("profile"))
        if is_heuristic(case):
            heuristic_present = True

        if (
            not present(case.get("case_id"))
            or not present(case.get("label_summary"))
            or not present(case.get("links", {}).get("execution_context_ref"))
        ):
            v.append("case_missing_identity")
        if case.get("origin_class") in OVERLAY_ORIGINS and not present(
            case.get("links", {}).get("provider_ref")
        ):
            v.append("overlay_missing_provider_ref")
        if is_real_channel(case) and not present(case.get("links", {}).get("channel_ref")):
            v.append("real_channel_missing_channel_ref")
        if is_heuristic(case) and not present(
            case.get("links", {}).get("raw_output_backlink_ref")
        ):
            v.append("heuristic_missing_backlink_ref")
        if not case.get("profiles"):
            v.append("case_missing_profile")
        for p in case.get("profiles", []):
            if not present(p.get("source_case_ref")):
                v.append("profile_missing_source_ref")

        decision = narrow(case, stale_window)
        if decision["narrowed"]:
            demonstrates_narrowing = True
            if not decision["reasons"]:
                v.append("narrowed_case_missing_label_or_trigger")
        if not floored_keeps_fallback(case, decision["effective"]):
            v.append("floored_case_loses_fallback")
        if surface_overclaims(case, decision["effective"]):
            v.append("profile_overclaims")

    if not REQUIRED_SOURCES.issubset(sources):
        v.append("problem_source_missing")
    if drills != DRILL_KINDS:
        v.append("drill_kind_missing")
    if not PROFILES.issubset(profiles):
        v.append("profile_missing")
    if not heuristic_present:
        v.append("heuristic_case_missing")
    if not demonstrates_narrowing:
        v.append("downgraded_case_missing")
    if contains_forbidden(packet):
        v.append("raw_boundary_material_in_export")

    # de-duplicate while keeping order
    out: list[str] = []
    for item in v:
        if item not in out:
            out.append(item)
    return out


def claim_distribution(cases: list[dict]) -> dict:
    dist = {
        "certified": 0,
        "narrowed": 0,
        "overlay": 0,
        "unreconstructable": 0,
        "labs": 0,
    }
    bucket = {
        "fallback_certified": "certified",
        "fallback_narrowed": "narrowed",
        "fallback_read_only_overlay": "overlay",
        "fallback_unreconstructable": "unreconstructable",
        LABS_CLAIM: "labs",
    }
    for case in cases:
        dist[bucket[narrow(case, False)["effective"]]] += 1
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


def apply_overrides(case: dict, overrides: dict) -> dict:
    out = json.loads(json.dumps(case))
    for dotted, value in overrides.items():
        _set_path(out, dotted.split("."), value)
    return out


def base_case(cases: list[dict], cid: str) -> dict:
    for case in cases:
        if case["case_id"] == cid:
            return case
    raise SystemExit(f"base case not found: {cid}")


C_NATIVE = "fallback:native-structured-language-problems:0001"
C_HEUR = "fallback:heuristic-parse-terminal:0001"
C_IMPORTED = "fallback:imported-provider-annotation:0001"
C_RECONNECT = "fallback:pipeline-reconnect:0001"
C_NOTEBOOK_STALE = "fallback:notebook-heuristic-stale:0001"
C_SUPERSEDED = "fallback:superseded-retry-marked:0001"
C_VIRT = "fallback:channel-virtualization-large-log:0001"
C_PARTIAL = "fallback:partial-export-support-bundle:0001"
C_PERF = "fallback:heuristic-stale-proof:0001"
C_LOSTCH = "fallback:lost-channel-remote:0001"
C_LABS = "fallback:labs-heuristic-notebook:0001"

UNREC = "fallback_unreconstructable"
NARROW = "fallback_narrowed"
CERT = "fallback_certified"
OVERLAY = "fallback_read_only_overlay"

# (case_id, base_id, overrides, stale_window, description, effective, narrowed, reasons)
CASES = [
    ("clean-certified", C_NATIVE, {}, False,
     "A clean native structured diagnostic certifies across the Problems panel, output channel, and debug console.",
     CERT, False, []),
    ("clean-heuristic-distinct", C_HEUR, {}, False,
     "A clean heuristic parse that reads visibly distinct from structured evidence certifies.",
     CERT, False, []),
    ("source-kind-flattened", C_NATIVE,
     {"integrity.preserves_source_kind": False, "profiles[*].rendered_claim": UNREC}, False,
     "Flattening the problem-source class floors the case to a raw fallback.",
     UNREC, True, ["source_kind_flattened"]),
    ("heuristic-indistinct", C_HEUR,
     {"integrity.heuristic_visibly_distinct_from_structured": False, "profiles[*].rendered_claim": UNREC}, False,
     "A heuristic parse not visibly distinct from structured evidence floors.",
     UNREC, True, ["heuristic_indistinct_from_structured"]),
    ("profile-hides-heuristic-distinction", C_HEUR,
     {"profiles[0].fallback_visibly_distinct": False, "profiles[*].rendered_claim": UNREC}, False,
     "A single profile that hides the heuristic-versus-structured distinction floors.",
     UNREC, True, ["heuristic_indistinct_from_structured"]),
    ("lineage-flattened", C_NATIVE,
     {"integrity.preserves_run_channel_lineage": False, "profiles[*].rendered_claim": UNREC}, False,
     "Flattening run/step/provider/channel lineage floors the case.",
     UNREC, True, ["run_channel_lineage_flattened"]),
    ("profile-hides-lineage", C_NATIVE,
     {"profiles[0].lineage_visible": False, "profiles[*].rendered_claim": UNREC}, False,
     "A single profile that cannot reveal lineage floors the case.",
     UNREC, True, ["run_channel_lineage_flattened"]),
    ("channel-identity-flattened", C_NATIVE,
     {"integrity.channel_identity_stable": False, "profiles[*].rendered_claim": UNREC}, False,
     "An output channel that loses its stable canonical id floors the case.",
     UNREC, True, ["channel_identity_flattened"]),
    ("canonical-id-divergence", C_NATIVE,
     {"profiles[1].bound_run_ref": "run.some.other.0009", "profiles[*].rendered_claim": UNREC}, False,
     "A profile that points at a different run id breaks the single-canonical-id contract and floors.",
     UNREC, True, ["canonical_id_divergence"]),
    ("heuristic-no-backlink", C_HEUR,
     {"integrity.raw_output_backlink_present": False, "profiles[*].rendered_claim": UNREC}, False,
     "A heuristic case without a raw-output backlink floors to a raw fallback.",
     UNREC, True, ["raw_output_backlink_missing"]),
    ("reopen-target-lost", C_NATIVE,
     {"declared_reopen_target": "none_keyboard_fallback", "profiles[*].rendered_claim": UNREC}, False,
     "Losing reopen-to-origin floors the case but keeps the keyboard fallback.",
     UNREC, True, ["reopen_target_lost"]),
    ("reconnect-drops-evidence", C_RECONNECT,
     {"integrity.reconnect_preserves_evidence": False, "profiles[*].rendered_claim": UNREC}, False,
     "A reconnect that drops the evidence/backlinks floors the case.",
     UNREC, True, ["reconnect_drops_evidence"]),
    ("lost-channel-drops-evidence", C_LOSTCH,
     {"integrity.reconnect_preserves_evidence": False, "profiles[*].rendered_claim": UNREC}, False,
     "A lost channel that drops the evidence/backlinks floors the case.",
     UNREC, True, ["reconnect_drops_evidence"]),
    ("partial-export-incomplete", C_PARTIAL,
     {"integrity.partial_export_self_contained": False, "profiles[*].rendered_claim": UNREC}, False,
     "A partial export that needs the originating UI to be reviewable floors the case.",
     UNREC, True, ["partial_export_incomplete"]),
    ("surface-overclaims", C_NATIVE,
     {"integrity.confidence_label_visible": False, "profiles[0].rendered_claim": CERT,
      "profiles[1].rendered_claim": NARROW, "profiles[2].rendered_claim": NARROW}, False,
     "A narrowed case whose profile still renders certified floors as an overclaim.",
     UNREC, True, ["surface_overclaims", "confidence_unlabeled"]),
    ("imported-claims-live", C_IMPORTED,
     {"integrity.imported_evidence_read_only": False, "profiles[*].rendered_claim": UNREC}, False,
     "An imported overlay claiming live local authority floors below the read-only overlay.",
     UNREC, True, ["imported_overlay_claims_live"]),
    ("evidence-missing", C_NATIVE,
     {"declared_freshness_state": "missing", "profiles[*].rendered_claim": UNREC}, False,
     "Missing evidence floors the case.",
     UNREC, True, ["evidence_missing"]),
    ("confidence-unlabeled", C_NATIVE,
     {"integrity.confidence_label_visible": False, "profiles[*].rendered_claim": NARROW}, False,
     "Hiding the confidence tier narrows the case.",
     NARROW, True, ["confidence_unlabeled"]),
    ("freshness-unlabeled", C_NATIVE,
     {"integrity.freshness_state_labeled": False, "profiles[*].rendered_claim": NARROW}, False,
     "Hiding the freshness state narrows a first-party case but keeps it reopenable.",
     NARROW, True, ["freshness_unlabeled"]),
    ("superseded-not-marked", C_NATIVE,
     {"declared_freshness_state": "superseded_by_newer_run", "integrity.superseded_state_marked": False,
      "profiles[*].rendered_claim": NARROW}, False,
     "An unmarked superseded run narrows the case.",
     NARROW, True, ["superseded_state_not_marked"]),
    ("superseded-marked-visible", C_SUPERSEDED, {}, False,
     "A marked superseded retry stays certified because the state is visible.",
     CERT, False, []),
    ("virtualization-not-stream-first", C_VIRT,
     {"virtualization.stream_first": False, "profiles[*].rendered_claim": NARROW}, False,
     "A large log that loses stream-first paging narrows the case.",
     NARROW, True, ["virtualization_not_stream_first"]),
    ("search-unavailable", C_VIRT,
     {"virtualization.searchable": False, "profiles[*].rendered_claim": NARROW}, False,
     "An output channel that loses search narrows the case.",
     NARROW, True, ["search_unavailable"]),
    ("copy-export-unavailable", C_VIRT,
     {"virtualization.copy_exportable": False, "profiles[*].rendered_claim": NARROW}, False,
     "An output channel that loses copy/export narrows the case.",
     NARROW, True, ["copy_export_unavailable"]),
    ("first-party-stale", C_NATIVE,
     {"declared_freshness_state": "stale_expired", "profiles[*].rendered_claim": NARROW}, False,
     "A first-party stale case narrows rather than reading as fresh.",
     NARROW, True, ["evidence_stale"]),
    ("missing-proof", C_NATIVE,
     {"verification.proof_currency": "missing_proof", "verification.proof_ref": None,
      "profiles[*].rendered_claim": NARROW}, False,
     "A missing verification proof narrows a first-party case.",
     NARROW, True, ["verification_proof_missing"]),
    ("stale-window", C_NATIVE,
     {"profiles[*].rendered_claim": NARROW}, True,
     "An elapsed verification window ages out a current proof to narrowed.",
     NARROW, True, ["verification_proof_stale"]),
    ("imported-overlay-clean", C_IMPORTED, {}, False,
     "An imported provider annotation reused read-only stays a read-only overlay, not narrowed.",
     OVERLAY, False, []),
    ("overlay-any-gap-floors", C_IMPORTED,
     {"integrity.freshness_state_labeled": False, "profiles[*].rendered_claim": UNREC}, False,
     "An overlay with any non-floor gap drops below the read-only overlay rather than holding it.",
     UNREC, True, ["freshness_unlabeled"]),
    ("perf-stale-proof-narrows", C_PERF, {}, False,
     "The canonical heuristic perf verdict narrows via a stale verification proof and stays reopenable.",
     NARROW, True, ["verification_proof_stale"]),
    ("notebook-stale-narrows", C_NOTEBOOK_STALE, {}, False,
     "The canonical notebook heuristic stale run narrows and stays reopenable.",
     NARROW, True, ["evidence_stale"]),
    ("labs-not-claimed", C_LABS, {}, False,
     "A Labs heuristic parse makes no public claim and is never widened or narrowed.",
     LABS_CLAIM, False, []),
]


def run_corpus_from_cases(cases: list[dict]) -> list[str]:
    failures: list[str] = []
    for case in CASES:
        case_id, base_id, overrides, stale_window, _desc, exp_eff, exp_narrowed, exp_reasons = case
        entry = apply_overrides(base_case(cases, base_id), overrides)
        decision = narrow(entry, stale_window)
        if decision["effective"] != exp_eff:
            failures.append(f"{case_id}: effective {decision['effective']} != {exp_eff}")
        if decision["narrowed"] != exp_narrowed:
            failures.append(f"{case_id}: narrowed {decision['narrowed']} != {exp_narrowed}")
        if decision["reasons"] != exp_reasons:
            failures.append(f"{case_id}: reasons {decision['reasons']} != {exp_reasons}")
    return failures


def run_corpus_from_disk(repo_root: Path, cases: list[dict]) -> list[str]:
    out_dir = repo_root / FIXTURE_DIR
    index_path = out_dir / "index.json"
    if not index_path.exists():
        return [f"missing corpus index: {index_path}"]
    index = json.loads(index_path.read_text(encoding="utf-8"))
    failures: list[str] = []
    for filename in index["cases"]:
        payload = json.loads((out_dir / filename).read_text(encoding="utf-8"))
        case_id = payload["case_id"]
        entry = apply_overrides(
            base_case(cases, payload["base_case_id"]),
            payload["overrides"],
        )
        decision = narrow(entry, payload["stale_window"])
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
            "base_case_id": base_id,
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
        "corpus_id": "m5-fallback-evidence-drills-corpus:0001",
        "description": (
            "Perturbation corpus for the structured-versus-heuristic fallback drill engine. "
            "Each case starts from a canonical parse-evidence case, applies dotted-path "
            "overrides, and asserts the re-derived effective claim, narrowed flag, and "
            "ordered narrowing reasons."
        ),
        "source_set_ref": SUPPORT_EXPORT_REF,
        "cases": case_files,
    }
    (out_dir / "index.json").write_text(json.dumps(index, indent=2) + "\n", encoding="utf-8")


def cmd_validate(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    violations = validate_packet(packet)
    if violations:
        sys.stderr.write("fallback-evidence drill set FAILED validation:\n")
        for v in violations:
            sys.stderr.write(f"  - {v}\n")
        return 1
    dist = claim_distribution(packet["cases"])
    sys.stdout.write(
        f"fallback-evidence drill set OK: {len(packet['cases'])} cases, distribution {dist}\n"
    )
    return 0


def cmd_corpus(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    cases = packet["cases"]
    failures = run_corpus_from_cases(cases)
    failures += run_corpus_from_disk(repo_root, cases)
    if failures:
        sys.stderr.write("fallback-evidence drill corpus FAILED:\n")
        for f in failures:
            sys.stderr.write(f"  - {f}\n")
        return 1
    sys.stdout.write(f"fallback-evidence drill corpus OK: {len(CASES)} cases\n")
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
