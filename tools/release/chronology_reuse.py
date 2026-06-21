#!/usr/bin/env python3
"""Freeze and certify the M5 task/problem/output chronology-reuse set: one durable
run-lifecycle event — start, progress, retry, cancel, failure, or completion —
written once and reused across the activity center, the history/timeline, an
exported issue packet, a support bundle, and an AI-evidence packet rather than each
surface re-summarising what ran.

Where ``tools/release/execution_evidence_causality.py`` certifies the *lane* matrix,
``tools/release/problem_records_causality.py`` certifies the *individual Problems
row*, and ``tools/release/execution_evidence_projections.py`` certifies the
*projected overlay*, this tool certifies the *individual chronology entry*. The
canonical truth is the checked-in support export
(``artifacts/tooling/m5-chronology-reuse/support_export.json``). Each entry binds its
actor/action/object/outcome grammar to the canonical task/run/channel/problem
objects, the provider/adapter and target scope it ran against, its retry lineage, the
evidence freshness/stale/superseded state, the confidence tier, and the
reopen-to-origin target.

This tool ingests that set and, per entry, **independently** re-derives an effective
claim that never reads wider than the evidence supports:

* a failure shown in the activity center, a support bundle, and an AI-evidence packet
  resolves to one canonical run/channel/problem id rather than three restatements;
* grammar, provider/adapter, target scope, retry lineage, and canonical ids stay
  reopenable on demand on every reuse surface, and the freshness/confidence labels
  stay visible;
* imported/remote/pipeline origins reuse read-only and never claim live local
  authority, an exported packet stays self-contained, and a reuse surface never
  renders wider than the effective claim;
* an entry that flattens its grammar or ids, lets two surfaces disagree about a
  canonical id, hides lineage from a surface, drops a heuristic backlink, loses its
  reopen path, exports a non-self-contained packet, or lets a surface overclaim floors
  to a raw-output / keyboard fallback rather than reusing a clean-but-false row.

The Rust truth source is
``crates/aureline-runtime/src/m5_task_problem_output_chronology_reuse``; this tool
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
SUPPORT_EXPORT_REF = "artifacts/tooling/m5-chronology-reuse/support_export.json"
REPORT_REF = "artifacts/tooling/m5-chronology-reuse/report.md"
SCHEMA_REF = "schemas/tooling/m5-chronology-reuse.schema.json"
FIXTURE_DIR = "fixtures/tooling/m5-chronology-reuse"

RECORD_KIND = "m5_chronology_reuse_set_packet"
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

CHRONOLOGY_PHASES = {
    "run_started",
    "run_progress",
    "run_retried",
    "run_cancelled",
    "run_failed",
    "run_completed",
}
CHRONOLOGY_SURFACES = {
    "activity_center",
    "history_timeline",
    "issue_packet",
    "support_bundle",
    "ai_evidence_packet",
}
EXPORT_SURFACES = {"issue_packet", "support_bundle", "ai_evidence_packet"}

# A recorded action and a recorded outcome can never silently disagree.
PHASE_OUTCOME = {
    "run_started": "in_progress",
    "run_progress": "in_progress",
    "run_retried": "retried",
    "run_cancelled": "cancelled",
    "run_failed": "failed",
    "run_completed": "succeeded",
}

LABS_CLAIM = "chronology_labs_not_claimed"
CLAIM_RANK = {
    "chronology_unreconstructable": 0,
    "chronology_read_only_overlay": 1,
    "chronology_narrowed": 2,
    "chronology_reused": 3,
}

# Floor reasons break the contract outright rather than merely aging out.
FLOOR_REASONS = {
    "grammar_flattened",
    "provider_adapter_flattened",
    "target_scope_flattened",
    "retry_lineage_flattened",
    "canonical_id_flattened",
    "canonical_id_divergence",
    "lineage_not_visible",
    "raw_output_backlink_missing",
    "reopen_target_lost",
    "export_not_self_contained",
    "surface_overclaims",
    "imported_chronology_claims_live",
    "evidence_missing",
}

# Deterministic reason ordering (mirrors the Rust order_index).
REASON_ORDER = {
    "grammar_flattened": 0,
    "provider_adapter_flattened": 1,
    "target_scope_flattened": 2,
    "retry_lineage_flattened": 3,
    "canonical_id_flattened": 4,
    "canonical_id_divergence": 5,
    "lineage_not_visible": 6,
    "reopen_target_lost": 7,
    "raw_output_backlink_missing": 8,
    "export_not_self_contained": 9,
    "surface_overclaims": 10,
    "imported_chronology_claims_live": 11,
    "evidence_missing": 12,
    "freshness_unlabeled": 13,
    "confidence_unlabeled": 14,
    "superseded_state_not_marked": 15,
    "evidence_stale": 16,
    "verification_proof_stale": 17,
    "verification_proof_missing": 18,
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


def claimed_claim(entry: dict) -> str:
    if entry["claim_posture"] == "labs_unadvertised":
        return LABS_CLAIM
    if entry["origin_class"] in OVERLAY_ORIGINS:
        return "chronology_read_only_overlay"
    return "chronology_reused"


def _refs_diverge(a, b) -> bool:
    sa = a.strip() if isinstance(a, str) else None
    sb = b.strip() if isinstance(b, str) else None
    if sa and sb:
        return sa != sb
    return False


def has_canonical_id_divergence(entry: dict) -> bool:
    links = entry["links"]
    return any(
        _refs_diverge(b.get("bound_run_ref"), links.get("run_ref"))
        or _refs_diverge(b.get("bound_channel_ref"), links.get("channel_ref"))
        or _refs_diverge(b.get("bound_problem_ref"), links.get("problem_ref"))
        for b in entry["bindings"]
    )


def intrinsic_reasons(entry: dict, stale_window: bool) -> list[str]:
    integ = entry["integrity"]
    ver = entry["verification"]
    overlay = entry["origin_class"] in OVERLAY_ORIGINS
    reasons: list[str] = []

    if not integ["preserves_actor_action_object_outcome"]:
        reasons.append("grammar_flattened")
    if not integ["preserves_provider_adapter"]:
        reasons.append("provider_adapter_flattened")
    if not integ["preserves_target_scope"]:
        reasons.append("target_scope_flattened")
    if not integ["preserves_retry_lineage"]:
        reasons.append("retry_lineage_flattened")
    if not integ["preserves_canonical_ids"]:
        reasons.append("canonical_id_flattened")
    if has_canonical_id_divergence(entry):
        reasons.append("canonical_id_divergence")

    if not integ["lineage_visible_on_demand"] or any(
        not b["lineage_visible"] for b in entry["bindings"]
    ):
        reasons.append("lineage_not_visible")

    if entry["declared_confidence_tier"] in HEURISTIC_TIERS and not integ["raw_output_backlink_present"]:
        reasons.append("raw_output_backlink_missing")
    if not integ["confidence_label_visible"]:
        reasons.append("confidence_unlabeled")

    if not integ["freshness_state_labeled"]:
        reasons.append("freshness_unlabeled")

    if entry["declared_reopen_target"] == "none_keyboard_fallback":
        reasons.append("reopen_target_lost")

    if not integ["export_self_contained"] and any(
        b["surface"] in EXPORT_SURFACES for b in entry["bindings"]
    ):
        reasons.append("export_not_self_contained")

    fs = entry["declared_freshness_state"]
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

    if overlay and not integ["imported_chronology_read_only"]:
        reasons.append("imported_chronology_claims_live")

    return reasons


def derive_effective(claimed: str, reasons: list[str]) -> str:
    if any(r in FLOOR_REASONS for r in reasons):
        return "chronology_unreconstructable"
    if not reasons:
        return claimed
    if claimed == "chronology_read_only_overlay":
        return "chronology_unreconstructable"
    return "chronology_narrowed"


def entry_reasons(entry: dict, stale_window: bool) -> list[str]:
    claimed = claimed_claim(entry)
    reasons = intrinsic_reasons(entry, stale_window)
    intrinsic = derive_effective(claimed, reasons)
    if any(overclaims(intrinsic, b["rendered_claim"]) for b in entry["bindings"]):
        reasons.append("surface_overclaims")
    return order_reasons(reasons)


def narrow(entry: dict, stale_window: bool) -> dict:
    claimed = claimed_claim(entry)
    if claimed == LABS_CLAIM:
        return {
            "claimed": LABS_CLAIM,
            "effective": LABS_CLAIM,
            "reasons": [],
            "narrowed": False,
        }
    reasons = entry_reasons(entry, stale_window)
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


def floored_keeps_fallback(entry: dict, effective: str) -> bool:
    if effective != "chronology_unreconstructable":
        return True
    if entry["declared_reopen_target"] in ("raw_output_backlink", "none_keyboard_fallback"):
        return True
    if entry["integrity"]["raw_output_backlink_present"]:
        return True
    return present(entry["links"].get("raw_output_backlink_ref"))


def surface_overclaims(entry: dict, effective: str) -> bool:
    return any(overclaims(effective, b["rendered_claim"]) for b in entry["bindings"])


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
    entries = packet.get("entries", [])
    if not entries:
        v.append("empty_entries")

    seen: set[str] = set()
    phases: set[str] = set()
    surfaces: set[str] = set()
    demonstrates_narrowing = False
    stale_window = False  # canonical packet is evaluated at its own as_of

    for entry in entries:
        eid = entry.get("entry_id", "")
        if eid in seen:
            v.append("duplicate_entry_id")
        seen.add(eid)
        action = entry.get("grammar", {}).get("action")
        phases.add(action)
        for b in entry.get("bindings", []):
            surfaces.add(b.get("surface"))

        if (
            not present(entry.get("entry_id"))
            or not present(entry.get("label_summary"))
            or not present(entry.get("links", {}).get("execution_context_ref"))
        ):
            v.append("entry_missing_identity")
        if entry.get("origin_class") in OVERLAY_ORIGINS and not present(
            entry.get("links", {}).get("provider_ref")
        ):
            v.append("overlay_missing_provider_ref")
        if not entry.get("bindings"):
            v.append("entry_missing_binding")
        for b in entry.get("bindings", []):
            if not present(b.get("source_entry_ref")):
                v.append("binding_missing_source_ref")

        grammar = entry.get("grammar", {})
        if grammar.get("outcome") != PHASE_OUTCOME.get(action):
            v.append("phase_outcome_mismatch")
        if action == "run_retried":
            retry = entry.get("retry_lineage", {})
            if retry.get("attempt_index", 0) < 2 or not present(retry.get("retry_of_run_ref")):
                v.append("retry_entry_missing_lineage")

        decision = narrow(entry, stale_window)
        if decision["narrowed"]:
            demonstrates_narrowing = True
            if not decision["reasons"]:
                v.append("narrowed_entry_missing_label_or_trigger")
        if not floored_keeps_fallback(entry, decision["effective"]):
            v.append("floored_entry_loses_fallback")
        if surface_overclaims(entry, decision["effective"]):
            v.append("binding_surface_overclaims")

    if phases != CHRONOLOGY_PHASES:
        v.append("chronology_phase_missing")
    if not CHRONOLOGY_SURFACES.issubset(surfaces):
        v.append("chronology_surface_missing")
    if not demonstrates_narrowing:
        v.append("downgraded_entry_case_missing")
    if contains_forbidden(packet):
        v.append("raw_boundary_material_in_export")

    # de-duplicate while keeping order
    out: list[str] = []
    for item in v:
        if item not in out:
            out.append(item)
    return out


def claim_distribution(entries: list[dict]) -> dict:
    dist = {
        "reused": 0,
        "narrowed": 0,
        "overlay": 0,
        "unreconstructable": 0,
        "labs": 0,
    }
    bucket = {
        "chronology_reused": "reused",
        "chronology_narrowed": "narrowed",
        "chronology_read_only_overlay": "overlay",
        "chronology_unreconstructable": "unreconstructable",
        LABS_CLAIM: "labs",
    }
    for entry in entries:
        dist[bucket[narrow(entry, False)["effective"]]] += 1
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


def apply_overrides(entry: dict, overrides: dict) -> dict:
    out = json.loads(json.dumps(entry))
    for dotted, value in overrides.items():
        _set_path(out, dotted.split("."), value)
    return out


def base_entry(entries: list[dict], eid: str) -> dict:
    for entry in entries:
        if entry["entry_id"] == eid:
            return entry
    raise SystemExit(f"base entry not found: {eid}")


E_STARTED = "chronology:run-started-local-task:0001"
E_RETRIED = "chronology:run-retried-local-task:0001"
E_FAILED = "chronology:run-failed-local-test:0001"
E_PIPELINE = "chronology:run-failed-pipeline-provider:0001"
E_PERF = "chronology:run-completed-perf-local:0001"
E_LABS = "chronology:run-progress-labs:0001"

UNREC = "chronology_unreconstructable"
NARROW = "chronology_narrowed"
REUSED = "chronology_reused"
OVERLAY = "chronology_read_only_overlay"

# (case_id, base_id, overrides, stale_window, description, effective, narrowed, reasons)
CASES = [
    ("clean-reused", E_STARTED, {}, False,
     "A clean first-party run-start event reuses across the activity center and history timeline.",
     REUSED, False, []),
    ("grammar-flattened", E_STARTED,
     {"integrity.preserves_actor_action_object_outcome": False, "bindings[*].rendered_claim": UNREC}, False,
     "Flattening the actor/action/object/outcome grammar floors the entry to a raw fallback.",
     UNREC, True, ["grammar_flattened"]),
    ("provider-adapter-flattened", E_STARTED,
     {"integrity.preserves_provider_adapter": False, "bindings[*].rendered_claim": UNREC}, False,
     "Flattening provider/adapter identity floors the entry.",
     UNREC, True, ["provider_adapter_flattened"]),
    ("target-scope-flattened", E_STARTED,
     {"integrity.preserves_target_scope": False, "bindings[*].rendered_claim": UNREC}, False,
     "Flattening the target scope floors the entry.",
     UNREC, True, ["target_scope_flattened"]),
    ("retry-lineage-flattened", E_RETRIED,
     {"integrity.preserves_retry_lineage": False, "bindings[*].rendered_claim": UNREC}, False,
     "Flattening retry lineage floors a retry entry.",
     UNREC, True, ["retry_lineage_flattened"]),
    ("canonical-id-flattened", E_FAILED,
     {"integrity.preserves_canonical_ids": False, "bindings[*].rendered_claim": UNREC}, False,
     "Flattening the canonical run/channel/problem ids floors the entry.",
     UNREC, True, ["canonical_id_flattened"]),
    ("canonical-id-divergence", E_FAILED,
     {"bindings[3].bound_run_ref": "run.some.other.0009", "bindings[*].rendered_claim": UNREC}, False,
     "A support bundle that points at a different run id breaks the single-canonical-id contract and floors.",
     UNREC, True, ["canonical_id_divergence"]),
    ("lineage-not-visible", E_STARTED,
     {"integrity.lineage_visible_on_demand": False, "bindings[*].rendered_claim": UNREC}, False,
     "Lineage that cannot be revealed on demand floors the entry.",
     UNREC, True, ["lineage_not_visible"]),
    ("surface-hides-lineage", E_STARTED,
     {"bindings[0].lineage_visible": False, "bindings[*].rendered_claim": UNREC}, False,
     "A single reuse surface that cannot reveal lineage floors the entry.",
     UNREC, True, ["lineage_not_visible"]),
    ("heuristic-no-backlink", E_STARTED,
     {"declared_confidence_tier": "heuristic_high", "integrity.raw_output_backlink_present": False,
      "bindings[*].rendered_claim": UNREC}, False,
     "A heuristic entry without a raw-output backlink floors to a raw fallback.",
     UNREC, True, ["raw_output_backlink_missing"]),
    ("reopen-target-lost", E_STARTED,
     {"declared_reopen_target": "none_keyboard_fallback", "bindings[*].rendered_claim": UNREC}, False,
     "Losing reopen-to-origin floors the entry but keeps the keyboard fallback.",
     UNREC, True, ["reopen_target_lost"]),
    ("export-not-self-contained", E_FAILED,
     {"integrity.export_self_contained": False, "bindings[*].rendered_claim": UNREC}, False,
     "An exported failure that needs the originating UI to be reviewable floors the entry.",
     UNREC, True, ["export_not_self_contained"]),
    ("surface-overclaims", E_STARTED,
     {"integrity.confidence_label_visible": False, "bindings[0].rendered_claim": REUSED,
      "bindings[1].rendered_claim": NARROW}, False,
     "A narrowed entry whose surface still renders reused floors as an overclaim.",
     UNREC, True, ["surface_overclaims", "confidence_unlabeled"]),
    ("imported-chronology-claims-live", E_PIPELINE,
     {"integrity.imported_chronology_read_only": False, "bindings[*].rendered_claim": UNREC}, False,
     "A pipeline chronology claiming live local authority floors below the read-only overlay.",
     UNREC, True, ["imported_chronology_claims_live"]),
    ("evidence-missing", E_STARTED,
     {"declared_freshness_state": "missing", "bindings[*].rendered_claim": UNREC}, False,
     "Missing evidence floors the entry.",
     UNREC, True, ["evidence_missing"]),
    ("freshness-unlabeled", E_STARTED,
     {"integrity.freshness_state_labeled": False, "bindings[*].rendered_claim": NARROW}, False,
     "Hiding the freshness state narrows a first-party entry but keeps it reopenable.",
     NARROW, True, ["freshness_unlabeled"]),
    ("confidence-unlabeled", E_STARTED,
     {"integrity.confidence_label_visible": False, "bindings[*].rendered_claim": NARROW}, False,
     "Hiding the confidence tier narrows the entry.",
     NARROW, True, ["confidence_unlabeled"]),
    ("superseded-not-marked", E_STARTED,
     {"declared_freshness_state": "superseded_by_newer_run", "integrity.superseded_state_marked": False,
      "bindings[*].rendered_claim": NARROW}, False,
     "An unmarked superseded run narrows the entry.",
     NARROW, True, ["superseded_state_not_marked"]),
    ("superseded-marked-visible", E_STARTED,
     {"declared_freshness_state": "superseded_by_newer_run"}, False,
     "A marked superseded run stays reused because the state is visible.",
     REUSED, False, []),
    ("first-party-stale", E_STARTED,
     {"declared_freshness_state": "stale_expired", "bindings[*].rendered_claim": NARROW}, False,
     "A first-party stale entry narrows rather than reading as fresh.",
     NARROW, True, ["evidence_stale"]),
    ("missing-proof", E_STARTED,
     {"verification.proof_currency": "missing_proof", "verification.proof_ref": None,
      "bindings[*].rendered_claim": NARROW}, False,
     "A missing verification proof narrows a first-party entry.",
     NARROW, True, ["verification_proof_missing"]),
    ("stale-window", E_STARTED,
     {"bindings[*].rendered_claim": NARROW}, True,
     "An elapsed verification window ages out a current proof to narrowed.",
     NARROW, True, ["verification_proof_stale"]),
    ("pipeline-overlay-clean", E_PIPELINE, {}, False,
     "A pipeline failure reused read-only stays a read-only overlay, not narrowed.",
     OVERLAY, False, []),
    ("overlay-any-gap-floors", E_PIPELINE,
     {"integrity.freshness_state_labeled": False, "bindings[*].rendered_claim": UNREC}, False,
     "An overlay with any non-floor gap drops below the read-only overlay rather than holding it.",
     UNREC, True, ["freshness_unlabeled"]),
    ("perf-stale-proof-narrows", E_PERF, {}, False,
     "The canonical perf verdict narrows via a stale verification proof and stays reopenable.",
     NARROW, True, ["verification_proof_stale"]),
    ("labs-not-claimed", E_LABS, {}, False,
     "A Labs run-progress entry makes no public claim and is never widened or narrowed.",
     LABS_CLAIM, False, []),
]


def run_corpus_from_cases(entries: list[dict]) -> list[str]:
    failures: list[str] = []
    for case in CASES:
        case_id, base_id, overrides, stale_window, _desc, exp_eff, exp_narrowed, exp_reasons = case
        entry = apply_overrides(base_entry(entries, base_id), overrides)
        decision = narrow(entry, stale_window)
        if decision["effective"] != exp_eff:
            failures.append(f"{case_id}: effective {decision['effective']} != {exp_eff}")
        if decision["narrowed"] != exp_narrowed:
            failures.append(f"{case_id}: narrowed {decision['narrowed']} != {exp_narrowed}")
        if decision["reasons"] != exp_reasons:
            failures.append(f"{case_id}: reasons {decision['reasons']} != {exp_reasons}")
    return failures


def run_corpus_from_disk(repo_root: Path, entries: list[dict]) -> list[str]:
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
            base_entry(entries, payload["base_entry_id"]),
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
            "base_entry_id": base_id,
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
        "corpus_id": "m5-chronology-reuse-corpus:0001",
        "description": (
            "Perturbation corpus for the chronology-reuse engine. Each case starts from a "
            "canonical chronology entry, applies dotted-path overrides, and asserts the "
            "re-derived effective claim, narrowed flag, and ordered narrowing reasons."
        ),
        "source_set_ref": SUPPORT_EXPORT_REF,
        "cases": case_files,
    }
    (out_dir / "index.json").write_text(json.dumps(index, indent=2) + "\n", encoding="utf-8")


def cmd_validate(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    violations = validate_packet(packet)
    if violations:
        sys.stderr.write("chronology-reuse set FAILED validation:\n")
        for v in violations:
            sys.stderr.write(f"  - {v}\n")
        return 1
    dist = claim_distribution(packet["entries"])
    sys.stdout.write(
        f"chronology-reuse set OK: {len(packet['entries'])} entries, distribution {dist}\n"
    )
    return 0


def cmd_corpus(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    entries = packet["entries"]
    failures = run_corpus_from_cases(entries)
    failures += run_corpus_from_disk(repo_root, entries)
    if failures:
        sys.stderr.write("chronology-reuse corpus FAILED:\n")
        for f in failures:
            sys.stderr.write(f"  - {f}\n")
        return 1
    sys.stdout.write(f"chronology-reuse corpus OK: {len(CASES)} cases\n")
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
