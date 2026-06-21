#!/usr/bin/env python3
"""Freeze and certify the M5 output-channel set: stream-first searchable
virtualization, content trust classes, pin/export controls, and live-vs-cached
-vs-stale channel freshness for raw log streams, trusted structured reports, HTML
report bundles, generated artifacts, and trace/profile outputs.

Where ``tools/release/execution_evidence_causality.py`` certifies the *lane* matrix
(one row per Problems/output/execution-evidence surface family),
``tools/release/problem_records_causality.py`` certifies the *individual Problems
row*, and ``tools/release/execution_evidence_projections.py`` certifies the
*projected overlay*, this tool certifies the *individual output channel*. The
canonical truth is the checked-in support export
(``artifacts/tooling/m5-output-channels/support_export.json``). Each channel binds an
output to the original run/step/provider/artifact lineage, the stream-first
virtualization profile that keeps a large log searchable and exportable without full
materialization, the content trust class and pin/export controls that keep
safe-preview distinct from active/open-in-external content, and the
live/cached/stale freshness with fetched-at and provider-unreachable cues.

This tool ingests that set and, per channel, **independently** re-derives an effective
claim that never reads wider than the evidence supports:

* the canonical channel id and origin run/step/provider identity stay reopenable on
  demand on every rendering surface;
* a large log stays stream-first, searchable, exportable, bounded, and chunk-stable;
* the content trust class stays labelled, safe-preview stays distinct from
  active/open-in-external, untrusted active content never auto-opens, and an export
  never leaks active content;
* provider-backed/imported channels disclose fetched-at and provider-unreachable cues
  and never claim live local authority, and a rendering surface never renders wider
  than the effective claim;
* a channel that flattens lineage, hides it from a surface, drops a heuristic backlink,
  loses its reopen path, forces full materialization on export, blurs the trust
  boundary, or masquerades as live floors to a raw-output / keyboard fallback rather
  than a clean-but-false channel.

The Rust truth source is
``crates/aureline-runtime/src/m5_output_channel_virtualization_trust_and_freshness``;
this tool re-derives the same effective claim and narrowing reasons so the checked-in
artifacts can never imply a wider claim than the current evidence backs.

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
SUPPORT_EXPORT_REF = "artifacts/tooling/m5-output-channels/support_export.json"
REPORT_REF = "artifacts/tooling/m5-output-channels/report.md"
SCHEMA_REF = "schemas/tooling/m5-output-channels.schema.json"
FIXTURE_DIR = "fixtures/tooling/m5-output-channels"

RECORD_KIND = "m5_output_channel_set_packet"
SCHEMA_VERSION = 1
TAXONOMY_VERSION = 1

# Chunk-count threshold above which stream-first virtualization is mandatory.
LARGE_CHANNEL_CHUNK_THRESHOLD = 256

OVERLAY_ORIGINS = {
    "remote_linked_run",
    "pipeline_provider_run",
    "imported_provider_evidence",
}
HEURISTIC_TIERS = {"heuristic_high", "heuristic_medium", "heuristic_low"}
REAL_CHANNEL_CLASSES = {
    "task_test_debug_output",
    "extension_ai_tool_output",
    "remote_provider_imported_output",
    "evidence_bundle",
}
REDACTION_TOKENS = {
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
}

PAYLOAD_KINDS = {
    "raw_log_stream",
    "structured_report",
    "html_report_bundle",
    "generated_artifact",
    "trace_profile_output",
}
TRUST_CLASSES = {"raw", "safe_preview", "trusted_structured", "untrusted_active"}
CHANNEL_SURFACES = {
    "output_pane",
    "terminal_pane",
    "problems_panel",
    "diff_review_overlay",
    "timeline_history",
    "support_export",
    "ai_evidence",
}

LABS_CLAIM = "channel_labs_not_claimed"
CLAIM_RANK = {
    "channel_unreconstructable": 0,
    "channel_read_only_overlay": 1,
    "channel_narrowed": 2,
    "channel_certified": 3,
}

# Floor reasons break the contract outright rather than merely aging out.
FLOOR_REASONS = {
    "channel_identity_flattened",
    "run_step_lineage_flattened",
    "provider_identity_flattened",
    "lineage_not_visible",
    "reopen_target_lost",
    "raw_output_backlink_missing",
    "stream_not_virtualized",
    "unbounded_memory",
    "export_forces_full_materialization",
    "trust_boundary_blurred",
    "active_content_auto_opens",
    "export_unsafe",
    "surface_overclaims",
    "imported_channel_claims_live",
    "stale_channel_claims_live",
    "channel_content_missing",
}

# Deterministic reason ordering (mirrors the Rust order_index).
REASON_ORDER = {
    "channel_identity_flattened": 0,
    "run_step_lineage_flattened": 1,
    "provider_identity_flattened": 2,
    "lineage_not_visible": 3,
    "reopen_target_lost": 4,
    "raw_output_backlink_missing": 5,
    "stream_not_virtualized": 6,
    "unbounded_memory": 7,
    "export_forces_full_materialization": 8,
    "trust_boundary_blurred": 9,
    "active_content_auto_opens": 10,
    "export_unsafe": 11,
    "surface_overclaims": 12,
    "imported_channel_claims_live": 13,
    "stale_channel_claims_live": 14,
    "channel_content_missing": 15,
    "trust_class_unlabeled": 16,
    "chunk_ids_unstable": 17,
    "follow_mode_unavailable": 18,
    "safe_preview_unavailable": 19,
    "pin_control_unavailable": 20,
    "export_control_unavailable": 21,
    "fetched_at_missing": 22,
    "provider_unreachable_unmarked": 23,
    "freshness_unlabeled": 24,
    "confidence_unlabeled": 25,
    "superseded_state_not_marked": 26,
    "channel_stale": 27,
    "verification_proof_stale": 28,
    "verification_proof_missing": 29,
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


def claimed_claim(chan: dict) -> str:
    if chan["claim_posture"] == "labs_unadvertised":
        return LABS_CLAIM
    if chan["origin_class"] in OVERLAY_ORIGINS:
        return "channel_read_only_overlay"
    return "channel_certified"


def requires_virtualization(chan: dict) -> bool:
    virt = chan["virtualization"]
    return bool(virt["large_log"]) or virt["total_chunk_count"] > LARGE_CHANNEL_CHUNK_THRESHOLD


def intrinsic_reasons(chan: dict, stale_window: bool) -> list[str]:
    integ = chan["integrity"]
    virt = chan["virtualization"]
    access = chan["access"]
    fresh = chan["freshness"]
    ver = chan["verification"]
    overlay = chan["origin_class"] in OVERLAY_ORIGINS
    requires_virt = requires_virtualization(chan)
    reasons: list[str] = []

    # Channel identity + origin lineage.
    if chan["channel_class"] in REAL_CHANNEL_CLASSES and not present(
        chan["lineage"].get("canonical_channel_ref")
    ):
        reasons.append("channel_identity_flattened")
    if not integ["preserves_run_step_lineage"]:
        reasons.append("run_step_lineage_flattened")
    if not integ["preserves_provider_identity"]:
        reasons.append("provider_identity_flattened")
    if not integ["lineage_visible_on_demand"] or any(
        not r["lineage_visible"] for r in chan["renderings"]
    ):
        reasons.append("lineage_not_visible")

    if chan["declared_confidence_tier"] in HEURISTIC_TIERS and not integ["raw_output_backlink_present"]:
        reasons.append("raw_output_backlink_missing")
    if not integ["confidence_label_visible"]:
        reasons.append("confidence_unlabeled")

    # Stream-first virtualization is mandatory for large logs.
    if requires_virt:
        if not virt["stream_first"] or not virt["searchable"]:
            reasons.append("stream_not_virtualized")
        if not virt["bounded_memory"]:
            reasons.append("unbounded_memory")
        if access["export_supported"] and not virt["exportable_without_full_materialization"]:
            reasons.append("export_forces_full_materialization")
        if not virt["stable_chunk_ids"]:
            reasons.append("chunk_ids_unstable")
        if not virt["follow_mode_supported"]:
            reasons.append("follow_mode_unavailable")

    # Content trust classes and pin/export controls.
    if not access["trust_class_labeled"]:
        reasons.append("trust_class_unlabeled")
    if not access["trust_boundary_preserved"]:
        reasons.append("trust_boundary_blurred")
    if chan["trust_class"] == "untrusted_active" and not access["open_in_external_requires_confirmation"]:
        reasons.append("active_content_auto_opens")
    if access["export_supported"] and not access["export_is_safe"]:
        reasons.append("export_unsafe")
    if not access["safe_preview_available"]:
        reasons.append("safe_preview_unavailable")
    if not access["pin_supported"]:
        reasons.append("pin_control_unavailable")
    if not access["export_supported"]:
        reasons.append("export_control_unavailable")

    if chan["declared_reopen_target"] == "none_keyboard_fallback":
        reasons.append("reopen_target_lost")

    # Freshness + provider/live cues.
    if not integ["freshness_state_labeled"]:
        reasons.append("freshness_unlabeled")
    if fresh["provider_backed"]:
        if not fresh["live_state_honest"]:
            reasons.append("stale_channel_claims_live")
        if not fresh["fetched_at_present"]:
            reasons.append("fetched_at_missing")
        if not fresh["provider_reachable"] and not fresh["provider_unreachable_marked"]:
            reasons.append("provider_unreachable_unmarked")

    fs = chan["declared_freshness_state"]
    if fs == "missing":
        reasons.append("channel_content_missing")
    elif fs == "superseded_by_newer_run" and not integ["superseded_state_marked"]:
        reasons.append("superseded_state_not_marked")
    elif fs == "stale_expired" and not overlay:
        reasons.append("channel_stale")

    pc = ver["proof_currency"]
    if pc == "missing_proof":
        reasons.append("verification_proof_missing")
    elif pc in ("stale_expired", "requires_review"):
        reasons.append("verification_proof_stale")
    elif pc in ("verified_current", "cached_within_window") and stale_window:
        reasons.append("verification_proof_stale")

    if overlay and not integ["imported_channel_read_only"]:
        reasons.append("imported_channel_claims_live")

    return reasons


def derive_effective(claimed: str, reasons: list[str]) -> str:
    if any(r in FLOOR_REASONS for r in reasons):
        return "channel_unreconstructable"
    if not reasons:
        return claimed
    if claimed == "channel_read_only_overlay":
        return "channel_unreconstructable"
    return "channel_narrowed"


def channel_reasons(chan: dict, stale_window: bool) -> list[str]:
    claimed = claimed_claim(chan)
    reasons = intrinsic_reasons(chan, stale_window)
    intrinsic = derive_effective(claimed, reasons)
    if any(overclaims(intrinsic, r["rendered_claim"]) for r in chan["renderings"]):
        reasons.append("surface_overclaims")
    return order_reasons(reasons)


def narrow(chan: dict, stale_window: bool) -> dict:
    claimed = claimed_claim(chan)
    if claimed == LABS_CLAIM:
        return {
            "claimed": LABS_CLAIM,
            "effective": LABS_CLAIM,
            "reasons": [],
            "narrowed": False,
        }
    reasons = channel_reasons(chan, stale_window)
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


def floored_keeps_fallback(chan: dict, effective: str) -> bool:
    if effective != "channel_unreconstructable":
        return True
    if chan["declared_reopen_target"] in ("raw_output_backlink", "none_keyboard_fallback"):
        return True
    if chan["integrity"]["raw_output_backlink_present"]:
        return True
    return present(chan["lineage"].get("raw_output_backlink_ref"))


def surface_overclaims(chan: dict, effective: str) -> bool:
    return any(overclaims(effective, r["rendered_claim"]) for r in chan["renderings"])


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
    channels = packet.get("channels", [])
    if not channels:
        v.append("empty_channels")

    seen: set[str] = set()
    kinds: set[str] = set()
    trust: set[str] = set()
    surfaces: set[str] = set()
    demonstrates_narrowing = False
    stale_window = False  # canonical packet is evaluated at its own as_of

    for chan in channels:
        cid = chan.get("channel_id", "")
        if cid in seen:
            v.append("duplicate_channel_id")
        seen.add(cid)
        kinds.add(chan.get("payload_kind"))
        trust.add(chan.get("trust_class"))
        for r in chan.get("renderings", []):
            surfaces.add(r.get("surface"))

        if (
            not present(chan.get("channel_id"))
            or not present(chan.get("label_summary"))
            or not present(chan.get("lineage", {}).get("execution_context_ref"))
        ):
            v.append("channel_missing_identity")
        if chan.get("origin_class") in OVERLAY_ORIGINS and not present(
            chan.get("lineage", {}).get("provider_ref")
        ):
            v.append("overlay_missing_provider_ref")
        if not chan.get("renderings"):
            v.append("channel_missing_rendering")
        for r in chan.get("renderings", []):
            if not present(r.get("source_channel_ref")):
                v.append("rendering_missing_source_ref")

        decision = narrow(chan, stale_window)
        if decision["narrowed"]:
            demonstrates_narrowing = True
            if not decision["reasons"]:
                v.append("narrowed_channel_missing_label_or_trigger")
        if not floored_keeps_fallback(chan, decision["effective"]):
            v.append("floored_channel_loses_fallback")
        if surface_overclaims(chan, decision["effective"]):
            v.append("rendering_surface_overclaims")

    if kinds != PAYLOAD_KINDS:
        v.append("channel_payload_kind_missing")
    if trust != TRUST_CLASSES:
        v.append("channel_trust_class_missing")
    if not CHANNEL_SURFACES.issubset(surfaces):
        v.append("channel_surface_missing")
    if not demonstrates_narrowing:
        v.append("downgraded_channel_case_missing")
    if contains_forbidden(packet):
        v.append("raw_boundary_material_in_export")

    # de-duplicate while keeping order
    out: list[str] = []
    for item in v:
        if item not in out:
            out.append(item)
    return out


def claim_distribution(channels: list[dict]) -> dict:
    dist = {
        "certified": 0,
        "narrowed": 0,
        "overlay": 0,
        "unreconstructable": 0,
        "labs": 0,
    }
    bucket = {
        "channel_certified": "certified",
        "channel_narrowed": "narrowed",
        "channel_read_only_overlay": "overlay",
        "channel_unreconstructable": "unreconstructable",
        LABS_CLAIM: "labs",
    }
    for chan in channels:
        dist[bucket[narrow(chan, False)["effective"]]] += 1
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


def apply_overrides(chan: dict, overrides: dict) -> dict:
    out = json.loads(json.dumps(chan))
    for dotted, value in overrides.items():
        _set_path(out, dotted.split("."), value)
    return out


def base_channel(channels: list[dict], cid: str) -> dict:
    for chan in channels:
        if chan["channel_id"] == cid:
            return chan
    raise SystemExit(f"base channel not found: {cid}")


C_RAW = "channel:raw-log-local-test:0001"
C_STRUCTURED = "channel:structured-report-local-test:0001"
C_HTML = "channel:html-bundle-local-task:0001"
C_NARROWED = "channel:raw-log-local-task:0001"
C_PIPELINE = "channel:raw-log-pipeline-provider:0001"
C_IMPORTED = "channel:structured-report-imported-provider:0001"
C_LABS = "channel:html-bundle-labs:0001"

UNREC = "channel_unreconstructable"
NARROW = "channel_narrowed"
CERT = "channel_certified"
OVERLAY = "channel_read_only_overlay"

# (case_id, base_id, overrides, stale_window, description, effective, narrowed, reasons)
CASES = [
    ("clean-certified", C_RAW, {}, False,
     "A clean, virtualized first-party raw log stream certifies.",
     CERT, False, []),
    ("channel-identity-flattened", C_RAW,
     {"lineage.canonical_channel_ref": None, "renderings[*].rendered_claim": UNREC}, False,
     "A real channel without a stable canonical channel ref floors to a raw fallback.",
     UNREC, True, ["channel_identity_flattened"]),
    ("run-step-lineage-flattened", C_RAW,
     {"integrity.preserves_run_step_lineage": False, "renderings[*].rendered_claim": UNREC}, False,
     "Flattening origin run/step lineage floors the channel.",
     UNREC, True, ["run_step_lineage_flattened"]),
    ("provider-identity-flattened", C_RAW,
     {"integrity.preserves_provider_identity": False, "renderings[*].rendered_claim": UNREC}, False,
     "Flattening provider identity floors the channel.",
     UNREC, True, ["provider_identity_flattened"]),
    ("lineage-not-visible", C_RAW,
     {"integrity.lineage_visible_on_demand": False, "renderings[*].rendered_claim": UNREC}, False,
     "Lineage that cannot be revealed on demand floors the channel.",
     UNREC, True, ["lineage_not_visible"]),
    ("surface-hides-lineage", C_RAW,
     {"renderings[0].lineage_visible": False, "renderings[*].rendered_claim": UNREC}, False,
     "A single rendering surface that cannot reveal lineage floors the channel.",
     UNREC, True, ["lineage_not_visible"]),
    ("reopen-target-lost", C_RAW,
     {"declared_reopen_target": "none_keyboard_fallback", "renderings[*].rendered_claim": UNREC}, False,
     "Losing reopen-to-origin floors the channel but keeps the keyboard fallback.",
     UNREC, True, ["reopen_target_lost"]),
    ("heuristic-no-backlink", C_RAW,
     {"declared_confidence_tier": "heuristic_high", "integrity.raw_output_backlink_present": False,
      "renderings[*].rendered_claim": UNREC}, False,
     "A heuristic channel without a raw-output backlink floors to a raw fallback.",
     UNREC, True, ["raw_output_backlink_missing"]),
    ("large-log-not-stream-first", C_RAW,
     {"virtualization.stream_first": False, "renderings[*].rendered_claim": UNREC}, False,
     "A large log that is not stream-first forces full materialization and floors.",
     UNREC, True, ["stream_not_virtualized"]),
    ("large-log-not-searchable", C_RAW,
     {"virtualization.searchable": False, "renderings[*].rendered_claim": UNREC}, False,
     "A large log that is not searchable floors.",
     UNREC, True, ["stream_not_virtualized"]),
    ("large-log-unbounded-memory", C_RAW,
     {"virtualization.bounded_memory": False, "renderings[*].rendered_claim": UNREC}, False,
     "A large log that does not bound retained memory floors.",
     UNREC, True, ["unbounded_memory"]),
    ("large-log-export-full-materialization", C_RAW,
     {"virtualization.exportable_without_full_materialization": False,
      "renderings[*].rendered_claim": UNREC}, False,
     "A large log whose export forces full materialization floors.",
     UNREC, True, ["export_forces_full_materialization"]),
    ("trust-boundary-blurred", C_HTML,
     {"access.trust_boundary_preserved": False, "renderings[*].rendered_claim": UNREC}, False,
     "Blurring the safe-preview versus active boundary floors the channel.",
     UNREC, True, ["trust_boundary_blurred"]),
    ("active-content-auto-opens", C_HTML,
     {"access.open_in_external_requires_confirmation": False, "renderings[*].rendered_claim": UNREC}, False,
     "Untrusted active content that opens externally without confirmation floors.",
     UNREC, True, ["active_content_auto_opens"]),
    ("export-unsafe", C_HTML,
     {"access.export_is_safe": False, "renderings[*].rendered_claim": UNREC}, False,
     "An export that would leak active content floors the channel.",
     UNREC, True, ["export_unsafe"]),
    ("surface-overclaims", C_RAW,
     {"access.trust_class_labeled": False, "renderings[0].rendered_claim": CERT,
      "renderings[1].rendered_claim": NARROW, "renderings[2].rendered_claim": NARROW}, False,
     "A narrowed channel whose surface still renders certified floors as an overclaim.",
     UNREC, True, ["surface_overclaims", "trust_class_unlabeled"]),
    ("imported-channel-claims-live", C_PIPELINE,
     {"integrity.imported_channel_read_only": False, "renderings[*].rendered_claim": UNREC}, False,
     "A pipeline channel claiming live local authority floors below the read-only overlay.",
     UNREC, True, ["imported_channel_claims_live"]),
    ("stale-channel-claims-live", C_PIPELINE,
     {"freshness.live_state_honest": False, "renderings[*].rendered_claim": UNREC}, False,
     "A provider-backed channel masquerading as live after a freshness threshold floors.",
     UNREC, True, ["stale_channel_claims_live"]),
    ("channel-content-missing", C_RAW,
     {"declared_freshness_state": "missing", "renderings[*].rendered_claim": UNREC}, False,
     "Missing channel content floors the channel.",
     UNREC, True, ["channel_content_missing"]),
    ("trust-class-unlabeled", C_RAW,
     {"access.trust_class_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the content trust class narrows a first-party channel but keeps it reopenable.",
     NARROW, True, ["trust_class_unlabeled"]),
    ("chunk-ids-unstable", C_RAW,
     {"virtualization.stable_chunk_ids": False, "renderings[*].rendered_claim": NARROW}, False,
     "Unstable chunk ids on a large log narrow the channel.",
     NARROW, True, ["chunk_ids_unstable"]),
    ("follow-mode-unavailable", C_RAW,
     {"virtualization.follow_mode_supported": False, "renderings[*].rendered_claim": NARROW}, False,
     "Missing follow/scroll on a large log narrows the channel.",
     NARROW, True, ["follow_mode_unavailable"]),
    ("safe-preview-unavailable", C_RAW,
     {"access.safe_preview_available": False, "renderings[*].rendered_claim": NARROW}, False,
     "No safe-preview path narrows the channel.",
     NARROW, True, ["safe_preview_unavailable"]),
    ("pin-control-unavailable", C_RAW,
     {"access.pin_supported": False, "renderings[*].rendered_claim": NARROW}, False,
     "A missing pin control narrows the channel.",
     NARROW, True, ["pin_control_unavailable"]),
    ("export-control-unavailable", C_RAW,
     {"access.export_supported": False, "renderings[*].rendered_claim": NARROW}, False,
     "A missing export control narrows the channel.",
     NARROW, True, ["export_control_unavailable"]),
    ("overlay-fetched-at-missing", C_PIPELINE,
     {"freshness.fetched_at_present": False, "renderings[*].rendered_claim": UNREC}, False,
     "A provider-backed overlay without a fetched-at cue drops below the read-only overlay.",
     UNREC, True, ["fetched_at_missing"]),
    ("overlay-provider-unreachable-unmarked", C_PIPELINE,
     {"freshness.provider_reachable": False, "freshness.provider_unreachable_marked": False,
      "renderings[*].rendered_claim": UNREC}, False,
     "A provider unreachable but not cued drops below the read-only overlay.",
     UNREC, True, ["provider_unreachable_unmarked"]),
    ("overlay-provider-unreachable-marked", C_PIPELINE,
     {"freshness.provider_reachable": False, "freshness.provider_unreachable_marked": True}, False,
     "A provider unreachable but honestly cued stays a read-only overlay.",
     OVERLAY, False, []),
    ("freshness-unlabeled", C_RAW,
     {"integrity.freshness_state_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the freshness state narrows the channel.",
     NARROW, True, ["freshness_unlabeled"]),
    ("confidence-unlabeled", C_RAW,
     {"integrity.confidence_label_visible": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the confidence tier narrows the channel.",
     NARROW, True, ["confidence_unlabeled"]),
    ("superseded-not-marked", C_RAW,
     {"declared_freshness_state": "superseded_by_newer_run", "integrity.superseded_state_marked": False,
      "renderings[*].rendered_claim": NARROW}, False,
     "An unmarked superseded run narrows the channel.",
     NARROW, True, ["superseded_state_not_marked"]),
    ("superseded-marked-visible", C_RAW,
     {"declared_freshness_state": "superseded_by_newer_run"}, False,
     "A marked superseded run stays certified because the state is visible.",
     CERT, False, []),
    ("first-party-stale", C_RAW,
     {"declared_freshness_state": "stale_expired", "renderings[*].rendered_claim": NARROW}, False,
     "A first-party stale channel narrows rather than reading as fresh.",
     NARROW, True, ["channel_stale"]),
    ("missing-proof", C_RAW,
     {"verification.proof_currency": "missing_proof", "verification.proof_ref": None,
      "renderings[*].rendered_claim": NARROW}, False,
     "A missing verification proof narrows a first-party channel.",
     NARROW, True, ["verification_proof_missing"]),
    ("stale-window", C_RAW,
     {"renderings[*].rendered_claim": NARROW}, True,
     "An elapsed verification window ages out a current proof to narrowed.",
     NARROW, True, ["verification_proof_stale"]),
    ("imported-overlay-cached-clean", C_IMPORTED, {}, False,
     "An imported structured report showing a cached snapshot stays a read-only overlay.",
     OVERLAY, False, []),
    ("overlay-any-gap-floors", C_PIPELINE,
     {"access.trust_class_labeled": False, "renderings[*].rendered_claim": UNREC}, False,
     "An overlay with any non-floor gap drops below the read-only overlay rather than holding it.",
     UNREC, True, ["trust_class_unlabeled"]),
    ("narrowed-base-stale-proof", C_NARROWED, {}, False,
     "The canonical narrowed raw log narrows via a stale verification proof and stays reopenable.",
     NARROW, True, ["verification_proof_stale"]),
    ("small-log-not-stream-first-ok", C_STRUCTURED, {}, False,
     "A small structured report is not required to be stream-first and certifies.",
     CERT, False, []),
    ("labs-not-claimed", C_LABS, {}, False,
     "A Labs HTML bundle makes no public claim and is never widened or narrowed.",
     LABS_CLAIM, False, []),
]


def run_corpus_from_cases(channels: list[dict]) -> list[str]:
    failures: list[str] = []
    for case in CASES:
        case_id, base_id, overrides, stale_window, _desc, exp_eff, exp_narrowed, exp_reasons = case
        chan = apply_overrides(base_channel(channels, base_id), overrides)
        decision = narrow(chan, stale_window)
        if decision["effective"] != exp_eff:
            failures.append(f"{case_id}: effective {decision['effective']} != {exp_eff}")
        if decision["narrowed"] != exp_narrowed:
            failures.append(f"{case_id}: narrowed {decision['narrowed']} != {exp_narrowed}")
        if decision["reasons"] != exp_reasons:
            failures.append(f"{case_id}: reasons {decision['reasons']} != {exp_reasons}")
    return failures


def run_corpus_from_disk(repo_root: Path, channels: list[dict]) -> list[str]:
    out_dir = repo_root / FIXTURE_DIR
    index_path = out_dir / "index.json"
    if not index_path.exists():
        return [f"missing corpus index: {index_path}"]
    index = json.loads(index_path.read_text(encoding="utf-8"))
    failures: list[str] = []
    for filename in index["cases"]:
        payload = json.loads((out_dir / filename).read_text(encoding="utf-8"))
        case_id = payload["case_id"]
        chan = apply_overrides(
            base_channel(channels, payload["base_channel_id"]),
            payload["overrides"],
        )
        decision = narrow(chan, payload["stale_window"])
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
            "base_channel_id": base_id,
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
        "corpus_id": "m5-output-channels-corpus:0001",
        "description": (
            "Perturbation corpus for the output-channel virtualization/trust/freshness engine. "
            "Each case starts from a canonical channel, applies dotted-path overrides, and asserts "
            "the re-derived effective claim, narrowed flag, and ordered narrowing reasons."
        ),
        "source_set_ref": SUPPORT_EXPORT_REF,
        "cases": case_files,
    }
    (out_dir / "index.json").write_text(json.dumps(index, indent=2) + "\n", encoding="utf-8")


def cmd_validate(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    violations = validate_packet(packet)
    if violations:
        sys.stderr.write("channel set FAILED validation:\n")
        for v in violations:
            sys.stderr.write(f"  - {v}\n")
        return 1
    dist = claim_distribution(packet["channels"])
    sys.stdout.write(
        f"channel set OK: {len(packet['channels'])} channels, distribution {dist}\n"
    )
    return 0


def cmd_corpus(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    channels = packet["channels"]
    failures = run_corpus_from_cases(channels)
    failures += run_corpus_from_disk(repo_root, channels)
    if failures:
        sys.stderr.write("channel corpus FAILED:\n")
        for f in failures:
            sys.stderr.write(f"  - {f}\n")
        return 1
    sys.stdout.write(f"channel corpus OK: {len(CASES)} cases\n")
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
