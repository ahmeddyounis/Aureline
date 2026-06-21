#!/usr/bin/env python3
"""Freeze and certify the M5 Problems / output-channel / execution-evidence causality matrix.

Problems rows, output channels, execution-evidence projections, and evidence
bundles are one causal system, not three loosely related panes. A user
investigating a failure must be able to answer **what ran, what produced this
message, how certain the parser was, what run/provider/channel it came from, and
how to reopen the originating evidence** without stitching raw logs together by
hand.

The canonical truth is the checked-in causality matrix support export
(``artifacts/tooling/m5-execution-evidence/support_export.json``). Each row is a
claimed (or Labs) tooling causality *lane*: a problem record, output channel,
execution-evidence projection, or evidence-bundle export bound to its origin
run/step/provider/channel/build-toolchain/host-target identity, its problem-source
kind, its confidence tier, its evidence freshness / stale / superseded state, and
its reopen-to-origin target.

This tool ingests that matrix and, per lane, **independently** re-derives an
effective causal claim that never reads wider than the evidence supports:

* structured and heuristic origins must stay distinct, and a heuristic parse must
  keep a raw-output backlink (Appendix BI.1);
* run / step / provider / channel / build-toolchain / host-target identity, and
  the original adapter, must survive into every overlay (no flattening);
* large logs must stay stream-first, searchable, and exportable (Appendix BI.2);
* stale and superseded state must remain visible; imported provider evidence
  stays a read-only overlay and never claims live local authority;
* the canonical evidence must stay reopenable, and evidence-bundle exports must
  carry the minimum identity to reopen the originating run/channel/artifact.

A lane that keeps the full first-party chain certifies at
``causal_chain_certified``. A first-party lane with a stale/missing/labelled gap
holds at ``causal_chain_narrowed`` (still reopenable). Remote, pipeline/provider,
and imported origins certify only as ``evidence_read_only_overlay``. Any lane that
flattens lineage/channel identity, loses its reopen path, ships an incomplete
bundle, has missing evidence, or lets an imported overlay claim live truth floors
at ``causal_chain_unreconstructable`` — the projection must surface the raw-output
backlink or keyboard fallback rather than a clean-but-false causal claim.
Labs/unadvertised lanes make no public claim and are never widened.

The tool both *generates* the surface-facing artifacts (the frozen claim matrix
JSON and the certification report) and *validates* them, so Problems, output,
diagnostics, AI evidence, support export, review, CLI/headless, and docs surfaces
ingest one governed projection instead of inventing a parallel causal model — and
so a checked-in artifact can never imply a wider causal claim than the current
evidence backs.

Subcommands::

    emit-matrix    Regenerate artifacts/tooling/m5-execution-evidence/matrix.json
    emit-report    Regenerate artifacts/tooling/m5-execution-evidence/report.md
    validate       Re-derive from the source matrix and fail on any overclaim
    corpus         Run the narrowing engine over fixtures/tooling/m5-execution-evidence
    self-test      End-to-end: schema check, emit round-trip, and the corpus pass

``validate`` is the default when no subcommand is given.
"""

from __future__ import annotations

import argparse
import json
import sys
from datetime import datetime, timezone
from pathlib import Path

# --------------------------------------------------------------------------- #
# Repo-relative paths and source contracts.
# --------------------------------------------------------------------------- #

REPO_ROOT = Path(__file__).resolve().parents[2]

SOURCE_MATRIX_REF = "artifacts/tooling/m5-execution-evidence/support_export.json"
PROFILE_MATRIX_REF = "artifacts/tooling/m5-execution-evidence/matrix.json"
REPORT_REF = "artifacts/tooling/m5-execution-evidence/report.md"
CORPUS_DIR_REF = "fixtures/tooling/m5-execution-evidence"

PACKET_SCHEMA_REF = "schemas/tooling/m5-execution-evidence.schema.json"
CONTRACT_DOC_REF = "docs/tooling/m5-execution-evidence.md"

PROFILE_MATRIX_RECORD_KIND = "m5_execution_evidence_causality_claim_matrix"
PROFILE_MATRIX_SCHEMA_VERSION = 1
PROFILE_MATRIX_ID = "m5-execution-evidence-claim-matrix:stable:0001"

# --------------------------------------------------------------------------- #
# Causal-claim ladder. A higher rank asserts more causal authority, so a narrowed
# or floored lane must move strictly lower. These tokens are what surfaces render.
# --------------------------------------------------------------------------- #

CLAIM_CERTIFIED = "causal_chain_certified"
CLAIM_NARROWED = "causal_chain_narrowed"
CLAIM_OVERLAY = "evidence_read_only_overlay"
CLAIM_UNRECON = "causal_chain_unreconstructable"
CLAIM_LABS = "causal_evidence_labs_not_claimed"

CLAIM_RANK = {
    CLAIM_UNRECON: 0,
    CLAIM_OVERLAY: 1,
    CLAIM_NARROWED: 2,
    CLAIM_CERTIFIED: 3,
}
CLAIM_BY_RANK = {rank: token for token, rank in CLAIM_RANK.items()}

# Origins that are inherently read-only overlays: they can never claim live local
# causal authority, only an attributable read-only overlay.
OVERLAY_ORIGINS = {"remote_linked_run", "pipeline_provider_run", "imported_provider_evidence"}
HEURISTIC_TIERS = {"heuristic_high", "heuristic_medium", "heuristic_low"}

# Public consumer surfaces that render a lane's causal claim. A projection must
# never render a claim wider than the lane's effective claim, and imported/overlay
# lanes must stay marked read-only on every surface.
REQUIRED_SURFACES = (
    "problems_panel",
    "output_channel_header",
    "editor_decoration",
    "timeline_history",
    "review_annotation",
    "support_export",
    "ai_evidence",
    "cli_headless",
    "docs_help",
    "public_proof_packet",
)

# Narrowing reasons. Floor reasons drop a lane all the way to unreconstructable;
# the remainder hold a first-party lane at narrowed (still reopenable).
REASON_ORIGIN_FLATTENED = "origin_kind_flattened"
REASON_RAW_BACKLINK_MISSING = "raw_output_backlink_missing"
REASON_CONFIDENCE_UNLABELED = "confidence_unlabeled"
REASON_LINEAGE_FLATTENED = "run_channel_lineage_flattened"
REASON_CHANNEL_IDENTITY_FLATTENED = "channel_identity_flattened"
REASON_BUILD_HOST_TARGET_MISSING = "build_or_host_target_missing"
REASON_STREAM_NOT_VIRTUALIZED = "stream_not_virtualized"
REASON_REOPEN_TARGET_LOST = "reopen_target_lost"
REASON_EXPORT_PACKET_INCOMPLETE = "export_packet_incomplete"
REASON_EVIDENCE_MISSING = "evidence_missing"
REASON_SUPERSEDED_NOT_MARKED = "superseded_state_not_marked"
REASON_UNANCHORED = "evidence_unanchored"
REASON_STALE_EVIDENCE = "evidence_stale"
REASON_STALE_PROOF = "verification_proof_stale"
REASON_MISSING_PROOF = "verification_proof_missing"
REASON_IMPORTED_OVERLAY_CLAIMS_LIVE = "imported_overlay_claims_live"

# Reasons that floor a lane to causal_chain_unreconstructable. Each is a guardrail
# that breaks the "stay reopenable / never flatten lineage / never masquerade as
# live" contract outright rather than merely aging out a claim.
FLOOR_REASONS = {
    REASON_RAW_BACKLINK_MISSING,
    REASON_LINEAGE_FLATTENED,
    REASON_CHANNEL_IDENTITY_FLATTENED,
    REASON_REOPEN_TARGET_LOST,
    REASON_EXPORT_PACKET_INCOMPLETE,
    REASON_EVIDENCE_MISSING,
    REASON_IMPORTED_OVERLAY_CLAIMS_LIVE,
}

# Deterministic ordering so the recorded downgrade_trigger and reason lists are
# stable across runs. Floor reasons sort first so the headline trigger is the most
# severe one.
REASON_ORDER = [
    REASON_LINEAGE_FLATTENED,
    REASON_CHANNEL_IDENTITY_FLATTENED,
    REASON_REOPEN_TARGET_LOST,
    REASON_RAW_BACKLINK_MISSING,
    REASON_EXPORT_PACKET_INCOMPLETE,
    REASON_EVIDENCE_MISSING,
    REASON_IMPORTED_OVERLAY_CLAIMS_LIVE,
    REASON_ORIGIN_FLATTENED,
    REASON_CONFIDENCE_UNLABELED,
    REASON_BUILD_HOST_TARGET_MISSING,
    REASON_STREAM_NOT_VIRTUALIZED,
    REASON_SUPERSEDED_NOT_MARKED,
    REASON_UNANCHORED,
    REASON_STALE_EVIDENCE,
    REASON_STALE_PROOF,
    REASON_MISSING_PROOF,
]

_GENERIC_LABELS = {
    "unavailable",
    "not available",
    "n/a",
    "error",
    "failed",
    "downgraded",
    "unverified",
    "narrowed",
    "stale",
}


# --------------------------------------------------------------------------- #
# Lane helpers.
# --------------------------------------------------------------------------- #


def is_labs(row: dict) -> bool:
    return row.get("claim_posture") == "labs_unadvertised"


def is_overlay_origin(row: dict) -> bool:
    return row.get("origin_class") in OVERLAY_ORIGINS


def claimed_claim(row: dict) -> str:
    """The headline causal claim a lane is eligible to make from its posture/origin."""
    if is_labs(row):
        return CLAIM_LABS
    if is_overlay_origin(row):
        return CLAIM_OVERLAY
    return CLAIM_CERTIFIED


def _order_reasons(reasons: list[str]) -> list[str]:
    index = {token: pos for pos, token in enumerate(REASON_ORDER)}
    return sorted(set(reasons), key=lambda token: index.get(token, len(REASON_ORDER)))


# --------------------------------------------------------------------------- #
# Axis evaluation — re-derive each guardrail rather than trusting a recorded grade.
# --------------------------------------------------------------------------- #


def lane_reasons(row: dict, stale_window: bool) -> list[str]:
    """Every causal-chain reason a lane fails to hold its headline claim."""
    cc = row["causal_chain"]
    ep = row["export_packet"]
    ident = row["identity"]
    psk = row["problem_source_kind"]
    ocl = row["output_channel_class"]
    family = row["surface_family"]
    fresh = row["declared_freshness_state"]
    reopen = row["declared_reopen_target"]
    tier = row["declared_confidence_tier"]
    proof = row["verification"]["proof_currency"]
    overlay = is_overlay_origin(row)

    reasons: list[str] = []

    # Origin honesty: structured vs heuristic must stay distinct.
    if not cc["structured_vs_heuristic_distinct"]:
        reasons.append(REASON_ORIGIN_FLATTENED)

    # A heuristic parse must keep a raw-output backlink and an explicit tier.
    if psk == "heuristic_output_parse":
        if not cc["raw_output_backlink_present"]:
            reasons.append(REASON_RAW_BACKLINK_MISSING)
        if tier not in HEURISTIC_TIERS or not cc["confidence_label_visible"]:
            reasons.append(REASON_CONFIDENCE_UNLABELED)
    elif not cc["confidence_label_visible"]:
        reasons.append(REASON_CONFIDENCE_UNLABELED)

    # Lineage: origin adapter + run/step/provider/channel + overlay lineage.
    if not (
        cc["preserves_origin_adapter"]
        and cc["preserves_run_step_provider_channel"]
        and cc["overlay_preserves_lineage"]
    ):
        reasons.append(REASON_LINEAGE_FLATTENED)
    if not cc["preserves_build_toolchain_host_target"]:
        reasons.append(REASON_BUILD_HOST_TARGET_MISSING)

    # Channel identity: a real channel must carry a stable channel ref.
    if ocl != "not_applicable" and not ident.get("channel_ref"):
        reasons.append(REASON_CHANNEL_IDENTITY_FLATTENED)
    if family == "output_channel" and not cc["stream_first_searchable_exportable"]:
        reasons.append(REASON_STREAM_NOT_VIRTUALIZED)

    # Reopen-to-origin must survive.
    if reopen == "none_keyboard_fallback":
        reasons.append(REASON_REOPEN_TARGET_LOST)

    # Evidence-bundle export minimums.
    if family == "evidence_bundle_export" and not all(ep.values()):
        reasons.append(REASON_EXPORT_PACKET_INCOMPLETE)

    # Evidence freshness / superseded / missing / unanchored.
    if fresh == "missing":
        reasons.append(REASON_EVIDENCE_MISSING)
    if fresh == "superseded_by_newer_run" and not cc["superseded_state_marked"]:
        reasons.append(REASON_SUPERSEDED_NOT_MARKED)
    if fresh == "unanchored":
        reasons.append(REASON_UNANCHORED)
    if fresh == "stale_expired" and not overlay:
        # An overlay snapshot is expected to be cached/stale; a first-party live
        # surface showing a stale projection has aged out of currency.
        reasons.append(REASON_STALE_EVIDENCE)

    # Certification-proof currency (distinct from the evidence's own freshness).
    if proof == "missing_proof":
        reasons.append(REASON_MISSING_PROOF)
    elif proof in {"stale_expired", "requires_review"}:
        reasons.append(REASON_STALE_PROOF)
    elif stale_window and proof in {"verified_current", "cached_within_window"}:
        reasons.append(REASON_STALE_PROOF)

    # Imported/remote overlays must stay read-only.
    if overlay and not cc["imported_overlay_read_only"]:
        reasons.append(REASON_IMPORTED_OVERLAY_CLAIMS_LIVE)

    return _order_reasons(reasons)


def narrow_row(row: dict, stale_window: bool) -> dict:
    """Compute the effective causal claim, reasons, and narrowed flag for one lane."""
    claimed = claimed_claim(row)

    # Labs/unadvertised lanes make no public claim, so they never accrue
    # governance narrowing; they hold their non-claiming token.
    if claimed == CLAIM_LABS:
        return {
            "claimed_causality_claim": CLAIM_LABS,
            "effective_causality_claim": CLAIM_LABS,
            "active_narrowing_reasons": [],
            "narrowed": False,
        }

    reasons = lane_reasons(row, stale_window)
    floored = any(reason in FLOOR_REASONS for reason in reasons)

    if floored:
        effective = CLAIM_UNRECON
    elif reasons:
        # An overlay is already the minimal honest claim: if anything else is off,
        # we can no longer certify even the read-only overlay, so we floor it. A
        # first-party lane holds at narrowed (still reopenable).
        effective = CLAIM_UNRECON if claimed == CLAIM_OVERLAY else CLAIM_NARROWED
    else:
        effective = claimed

    return {
        "claimed_causality_claim": claimed,
        "effective_causality_claim": effective,
        "active_narrowing_reasons": reasons,
        "narrowed": CLAIM_RANK[effective] < CLAIM_RANK[claimed],
    }


def effective_confidence(row: dict, effective: str) -> str:
    """A floored lane cannot assert a confidence tier beyond unmapped/needs-review."""
    if effective == CLAIM_UNRECON:
        return "unmapped_requires_review"
    return row["declared_confidence_tier"]


def downgrade_trigger_for(decision: dict) -> str | None:
    if not decision["narrowed"]:
        return None
    reasons = decision["active_narrowing_reasons"]
    return reasons[0] if reasons else None


def narrowed_label_for(row: dict, decision: dict) -> str | None:
    """Precise, non-generic label for a narrowed/floored lane."""
    if not decision["narrowed"]:
        return None
    trigger = downgrade_trigger_for(decision) or "narrowed"
    effective = decision["effective_causality_claim"]
    reopen = (
        "raw-output backlink"
        if row["declared_reopen_target"] == "raw_output_backlink"
        else row["declared_reopen_target"].replace("_", " ")
    )
    if effective == CLAIM_UNRECON:
        return (
            f"Floored to {effective} below the {decision['claimed_causality_claim']} "
            f"claim: {trigger.replace('_', ' ')}; the {reopen} stays reopenable rather "
            "than rendering a clean-but-false causal claim"
        )
    return (
        f"Held at {effective} below the {decision['claimed_causality_claim']} claim: "
        f"{trigger.replace('_', ' ')}; lineage stays reopenable via the {reopen} "
        "until re-verified"
    )


def identity_summary(row: dict) -> dict:
    ident = row["identity"]
    return {key: ident.get(key) for key in sorted(ident)}


# --------------------------------------------------------------------------- #
# Source matrix + freshness.
# --------------------------------------------------------------------------- #


def parse_rfc3339(value: str) -> datetime:
    text = value.strip()
    if text.endswith("Z"):
        text = text[:-1] + "+00:00"
    parsed = datetime.fromisoformat(text)
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=timezone.utc)
    return parsed


def freshness_stale(matrix: dict, as_of: str | None) -> bool:
    """Whether the matrix verification window has elapsed by ``as_of``."""
    freshness = matrix["verification_freshness"]
    last_refresh = freshness["last_verification_refresh"]
    if as_of is None:
        as_of = last_refresh
    if not freshness.get("auto_downgrade_on_stale", True):
        return False
    slo_hours = freshness["verification_freshness_slo_hours"]
    elapsed = parse_rfc3339(as_of) - parse_rfc3339(last_refresh)
    return elapsed.total_seconds() > slo_hours * 3600


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def resolve(ref: str, override: str | None = None) -> Path:
    if override:
        candidate = Path(override)
        return candidate if candidate.is_absolute() else REPO_ROOT / candidate
    return REPO_ROOT / ref


def display_path(path: Path) -> str:
    try:
        return str(path.relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


# --------------------------------------------------------------------------- #
# Frozen claim statements.
# --------------------------------------------------------------------------- #

STABLE_CLAIM_STATEMENT = (
    "Problems, output channels, execution-evidence projections, and evidence "
    "bundles preserve one causal chain across the claimed M5 tooling lanes: "
    "structured and heuristic origins stay distinct and a heuristic parse keeps a "
    "raw-output backlink; run, step, provider, channel, build/toolchain, and "
    "host/target identity survive into every overlay; large logs stay stream-first, "
    "searchable, and exportable; stale and superseded state stay visible; imported "
    "provider evidence is a read-only overlay that never claims live local "
    "authority; and the canonical evidence stays reopenable to its originating run, "
    "channel, or artifact. Stable IDs, stale/superseded semantics, and confidence "
    "labels are defined once and reused by UI, CLI/headless, support export, AI "
    "evidence, review, and docs consumers."
)
FUTURE_AMBITION_STATEMENT = (
    "A separate M6-only collaboration log model and a broad hosted "
    "pipeline-control surface are not claimed here; such surfaces stay "
    "Labs/unadvertised and out of public execution-evidence support until "
    "separately qualified."
)


# --------------------------------------------------------------------------- #
# Projection (the generated claim matrix).
# --------------------------------------------------------------------------- #


def build_profile_matrix(matrix: dict, as_of: str | None) -> dict:
    freshness = matrix["verification_freshness"]
    effective_as_of = as_of or freshness["last_verification_refresh"]
    stale_window = freshness_stale(matrix, effective_as_of)

    entries = []
    counts = {"certified": 0, "narrowed": 0, "overlay": 0, "unreconstructable": 0, "labs": 0}
    narrowed = 0
    surface_rows = 0
    overclaims = 0

    for row in matrix["rows"]:
        decision = narrow_row(row, stale_window)
        effective = decision["effective_causality_claim"]
        read_only = is_overlay_origin(row)
        projections = [
            {
                "surface_id": surface,
                "rendered_claim": effective,
                "read_only": read_only,
                "source_matrix_ref": PROFILE_MATRIX_ID,
            }
            for surface in REQUIRED_SURFACES
        ]
        surface_rows += len(projections)
        if decision["narrowed"]:
            narrowed += 1

        if effective == CLAIM_CERTIFIED:
            counts["certified"] += 1
        elif effective == CLAIM_NARROWED:
            counts["narrowed"] += 1
        elif effective == CLAIM_OVERLAY:
            counts["overlay"] += 1
        elif effective == CLAIM_UNRECON:
            counts["unreconstructable"] += 1
        elif effective == CLAIM_LABS:
            counts["labs"] += 1

        entries.append(
            {
                "lane_id": row["lane_id"],
                "surface_family": row["surface_family"],
                "origin_class": row["origin_class"],
                "problem_source_kind": row["problem_source_kind"],
                "output_channel_class": row["output_channel_class"],
                "claim_posture": row["claim_posture"],
                "claimed_causality_claim": decision["claimed_causality_claim"],
                "effective_causality_claim": effective,
                "narrowed": decision["narrowed"],
                "active_narrowing_reasons": decision["active_narrowing_reasons"],
                "downgrade_trigger": downgrade_trigger_for(decision),
                "narrowed_label": narrowed_label_for(row, decision),
                "declared_confidence_tier": row["declared_confidence_tier"],
                "effective_confidence_tier": effective_confidence(row, effective),
                "declared_freshness_state": row["declared_freshness_state"],
                "declared_reopen_target": row["declared_reopen_target"],
                "read_only": read_only,
                "identity": identity_summary(row),
                "evidence": {
                    "proof_currency": row["verification"]["proof_currency"],
                    "proof_ref_present": bool(row["verification"].get("proof_ref")),
                    "freshness_state": "stale" if stale_window else "fresh",
                },
                "surface_projections": projections,
            }
        )

    decision = "proceed"
    if overclaims:
        decision = "hold"
    elif narrowed:
        decision = "proceed_with_narrowing"

    return {
        "record_kind": PROFILE_MATRIX_RECORD_KIND,
        "schema_version": PROFILE_MATRIX_SCHEMA_VERSION,
        "matrix_id": PROFILE_MATRIX_ID,
        "label": "M5 Problems / output-channel / execution-evidence causal-claim matrix",
        "source_matrix_ref": SOURCE_MATRIX_REF,
        "source_matrix_packet_id": matrix["packet_id"],
        "as_of": effective_as_of,
        "verification_freshness": {
            "verification_freshness_slo_hours": freshness["verification_freshness_slo_hours"],
            "last_verification_refresh": freshness["last_verification_refresh"],
            "auto_narrow_on_stale": bool(freshness.get("auto_downgrade_on_stale", True)),
            "stale_as_of": stale_window,
        },
        "claim_separation": {
            "stable_claim_statement": STABLE_CLAIM_STATEMENT,
            "future_ambition_statement": FUTURE_AMBITION_STATEMENT,
            "stable_distinguishes_claimed_from_future": True,
        },
        "entries": entries,
        "publication": {"decision": decision},
        "summary": {
            "total_entries": len(entries),
            "narrowed_entries": narrowed,
            "certified_entries": counts["certified"],
            "narrowed_claim_entries": counts["narrowed"],
            "overlay_entries": counts["overlay"],
            "unreconstructable_entries": counts["unreconstructable"],
            "labs_entries": counts["labs"],
            "surface_projection_rows": surface_rows,
            "overclaiming_surface_rows": overclaims,
        },
        "source_contract_refs": [SOURCE_MATRIX_REF, PACKET_SCHEMA_REF, CONTRACT_DOC_REF],
        "redaction_class_token": matrix.get("redaction_class_token", "metadata_safe_default"),
        "minted_at": effective_as_of,
    }


# --------------------------------------------------------------------------- #
# Validation.
# --------------------------------------------------------------------------- #


def validate_profile_matrix(profile_matrix: dict, matrix: dict) -> list[str]:
    findings: list[str] = []
    if profile_matrix.get("record_kind") != PROFILE_MATRIX_RECORD_KIND:
        findings.append("unsupported record_kind")
    if profile_matrix.get("schema_version") != PROFILE_MATRIX_SCHEMA_VERSION:
        findings.append("unsupported schema_version")

    separation = profile_matrix.get("claim_separation", {})
    if not separation.get("stable_distinguishes_claimed_from_future"):
        findings.append("claim_separation must distinguish claimed scope from future ambitions")
    if not separation.get("stable_claim_statement", "").strip():
        findings.append("claim_separation missing stable_claim_statement")

    as_of = profile_matrix.get("as_of")
    stale_window = freshness_stale(matrix, as_of)

    rows_by_id = {row["lane_id"]: row for row in matrix["rows"]}
    entries = profile_matrix.get("entries", [])
    if {entry["lane_id"] for entry in entries} != set(rows_by_id):
        findings.append("entries do not cover the source matrix lanes one-to-one")

    narrowed = 0
    surface_rows = 0
    overclaims = 0
    counts = {"certified": 0, "narrowed": 0, "overlay": 0, "unreconstructable": 0, "labs": 0}
    seen: set[str] = set()

    for entry in entries:
        lane_id = entry.get("lane_id", "")
        if lane_id in seen:
            findings.append(f"{lane_id}: duplicate entry")
        seen.add(lane_id)
        row = rows_by_id.get(lane_id)
        if row is None:
            findings.append(f"{lane_id}: entry has no source matrix lane")
            continue

        decision = narrow_row(row, stale_window)
        effective = decision["effective_causality_claim"]
        if entry.get("effective_causality_claim") != effective:
            findings.append(
                f"{lane_id}: recorded effective claim {entry.get('effective_causality_claim')} "
                f"!= derived {effective}"
            )
        if entry.get("claimed_causality_claim") != decision["claimed_causality_claim"]:
            findings.append(f"{lane_id}: recorded claimed claim mismatch")
        if sorted(entry.get("active_narrowing_reasons", [])) != sorted(
            decision["active_narrowing_reasons"]
        ):
            findings.append(
                f"{lane_id}: narrowing reasons mismatch: recorded "
                f"{sorted(entry.get('active_narrowing_reasons', []))} != derived "
                f"{sorted(decision['active_narrowing_reasons'])}"
            )
        recorded_narrowed = bool(entry.get("narrowed"))
        if recorded_narrowed != decision["narrowed"]:
            findings.append(f"{lane_id}: narrowed flag mismatch")
        if entry.get("effective_confidence_tier") != effective_confidence(row, effective):
            findings.append(f"{lane_id}: effective confidence tier mismatch")

        if recorded_narrowed:
            narrowed += 1
            if not entry.get("downgrade_trigger"):
                findings.append(f"{lane_id}: narrowed entry missing downgrade_trigger")
            label = (entry.get("narrowed_label") or "").strip()
            if not label or label.lower() in _GENERIC_LABELS:
                findings.append(f"{lane_id}: narrowed entry missing a precise label")
        elif entry.get("active_narrowing_reasons"):
            findings.append(f"{lane_id}: un-narrowed entry carries narrowing reasons")

        if effective == CLAIM_CERTIFIED:
            counts["certified"] += 1
        elif effective == CLAIM_NARROWED:
            counts["narrowed"] += 1
        elif effective == CLAIM_OVERLAY:
            counts["overlay"] += 1
        elif effective == CLAIM_UNRECON:
            counts["unreconstructable"] += 1
        elif effective == CLAIM_LABS:
            counts["labs"] += 1

        # Imported/overlay origins must remain read-only on every projection.
        read_only_expected = is_overlay_origin(row)
        if bool(entry.get("read_only")) != read_only_expected:
            findings.append(f"{lane_id}: read_only flag mismatch")

        effective_rank = CLAIM_RANK.get(effective)
        projection_surfaces = []
        for projection in entry.get("surface_projections", []):
            surface_rows += 1
            projection_surfaces.append(projection.get("surface_id"))
            rendered = projection.get("rendered_claim")
            if rendered == CLAIM_LABS and effective == CLAIM_LABS:
                pass
            elif rendered not in CLAIM_RANK:
                findings.append(
                    f"{lane_id}: {projection.get('surface_id')} renders unknown claim {rendered}"
                )
                continue
            elif effective_rank is None or CLAIM_RANK[rendered] > effective_rank:
                overclaims += 1
                findings.append(
                    f"{lane_id}: surface {projection.get('surface_id')} renders {rendered}, "
                    f"wider than effective {effective}"
                )
            if read_only_expected and not bool(projection.get("read_only")):
                findings.append(
                    f"{lane_id}: surface {projection.get('surface_id')} drops the read-only "
                    "marker on an imported/overlay lane"
                )
        if set(projection_surfaces) != set(REQUIRED_SURFACES):
            findings.append(f"{lane_id}: surface coverage mismatch")

    summary = profile_matrix.get("summary", {})
    expected_summary = {
        "total_entries": len(entries),
        "narrowed_entries": narrowed,
        "certified_entries": counts["certified"],
        "narrowed_claim_entries": counts["narrowed"],
        "overlay_entries": counts["overlay"],
        "unreconstructable_entries": counts["unreconstructable"],
        "labs_entries": counts["labs"],
        "surface_projection_rows": surface_rows,
        "overclaiming_surface_rows": overclaims,
    }
    if summary != expected_summary:
        findings.append(f"summary mismatch: expected {expected_summary}, got {summary}")

    expected_decision = "hold" if overclaims else "proceed_with_narrowing" if narrowed else "proceed"
    if profile_matrix.get("publication", {}).get("decision") != expected_decision:
        findings.append(f"publication decision should be {expected_decision}")

    return findings


def validate_source_schema(matrix: dict) -> list[str]:
    """Validate the source packet against the canonical schema, when tooling is present."""
    try:
        from jsonschema import Draft202012Validator  # type: ignore
    except Exception:  # pragma: no cover - schema check is best-effort locally
        return []
    schema = load_json(resolve(PACKET_SCHEMA_REF))
    validator = Draft202012Validator(schema)
    findings = []
    for err in sorted(validator.iter_errors(matrix), key=lambda e: list(e.path)):
        loc = ".".join(map(str, err.path)) or "<root>"
        findings.append(f"schema:{loc}: {err.message}")
    return findings


# --------------------------------------------------------------------------- #
# Report.
# --------------------------------------------------------------------------- #

REASON_GLOSSARY = [
    (REASON_LINEAGE_FLATTENED, "run/step/provider/channel or origin-adapter lineage flattened → floor"),
    (REASON_CHANNEL_IDENTITY_FLATTENED, "output channel lost its stable canonical channel ref → floor"),
    (REASON_REOPEN_TARGET_LOST, "reopen-to-origin lost; only a keyboard fallback remains → floor"),
    (REASON_RAW_BACKLINK_MISSING, "heuristic parse without a raw-output backlink → floor"),
    (REASON_EXPORT_PACKET_INCOMPLETE, "evidence-bundle export missing the minimum reopen identity → floor"),
    (REASON_EVIDENCE_MISSING, "evidence missing → floor"),
    (REASON_IMPORTED_OVERLAY_CLAIMS_LIVE, "imported/remote overlay claims live local authority → floor"),
    (REASON_ORIGIN_FLATTENED, "structured vs heuristic origin not distinct → narrow"),
    (REASON_CONFIDENCE_UNLABELED, "heuristic confidence tier not surfaced → narrow"),
    (REASON_BUILD_HOST_TARGET_MISSING, "build/toolchain or host/target identity not visible → narrow"),
    (REASON_STREAM_NOT_VIRTUALIZED, "large log not stream-first/searchable/exportable → narrow"),
    (REASON_SUPERSEDED_NOT_MARKED, "superseded-by-newer-run state not marked → narrow"),
    (REASON_UNANCHORED, "evidence unanchored to current revision → narrow"),
    (REASON_STALE_EVIDENCE, "first-party evidence projection stale → narrow"),
    (REASON_STALE_PROOF, "verification proof stale or window elapsed → narrow"),
    (REASON_MISSING_PROOF, "verification proof missing → narrow"),
]

_CLAIM_HEADINGS = {
    CLAIM_CERTIFIED: "certified",
    CLAIM_NARROWED: "narrowed",
    CLAIM_OVERLAY: "read-only overlay",
    CLAIM_UNRECON: "unreconstructable / raw-fallback",
    CLAIM_LABS: "labs / not claimed",
}


def render_report(profile_matrix: dict) -> str:
    out: list[str] = []
    out.append("# M5 Problems / Output-Channel / Execution-Evidence Causality Report\n")
    out.append(
        "Generated by `tools/release/execution_evidence_causality.py` from "
        f"`{profile_matrix['source_matrix_ref']}`. Do not edit by hand.\n"
    )
    out.append(f"- Matrix: `{profile_matrix['matrix_id']}`")
    out.append(f"- Source packet: `{profile_matrix['source_matrix_packet_id']}`")
    out.append(f"- As of: `{profile_matrix['as_of']}`")
    freshness = profile_matrix["verification_freshness"]
    out.append(
        f"- Verification freshness SLO: {freshness['verification_freshness_slo_hours']} hours "
        f"(last refresh `{freshness['last_verification_refresh']}`, "
        f"stale as of evaluation: {str(freshness['stale_as_of']).lower()})"
    )
    summary = profile_matrix["summary"]
    out.append(
        f"- Lanes: {summary['total_entries']} "
        f"({summary['certified_entries']} certified, "
        f"{summary['narrowed_claim_entries']} narrowed, "
        f"{summary['overlay_entries']} read-only overlay, "
        f"{summary['unreconstructable_entries']} unreconstructable, "
        f"{summary['labs_entries']} labs)"
    )
    out.append(f"- Publication decision: `{profile_matrix['publication']['decision']}`\n")

    separation = profile_matrix["claim_separation"]
    out.append("## Claimed scope vs future ambition\n")
    out.append(f"**Claimed (stable):** {separation['stable_claim_statement']}\n")
    out.append(f"**Not claimed (future):** {separation['future_ambition_statement']}\n")

    out.append("## Causal-claim ladder\n")
    out.append(
        "- `causal_chain_certified` — full first-party causal chain preserved, fresh, "
        "confidence honest, reopenable."
    )
    out.append(
        "- `causal_chain_narrowed` — a first-party lane held below certified by a "
        "stale/missing/labelled gap, but lineage stays reopenable."
    )
    out.append(
        "- `evidence_read_only_overlay` — remote/pipeline/imported evidence; attributable "
        "and reopenable but never claims live local authority."
    )
    out.append(
        "- `causal_chain_unreconstructable` — lineage/channel/reopen broken or evidence "
        "missing; the lane surfaces a raw-output backlink or keyboard fallback instead of "
        "a clean-but-false causal claim."
    )
    out.append(
        "- `causal_evidence_labs_not_claimed` — Labs/unadvertised; makes no public causal "
        "claim and is never widened.\n"
    )

    out.append("## Auto-narrowing rules\n")
    out.append(
        "A claimed lane auto-narrows below its headline claim when any causal-chain axis "
        "fails or its verification evidence is stale or missing:\n"
    )
    for token, gloss in REASON_GLOSSARY:
        out.append(f"- `{token}` — {gloss}")
    out.append("")

    out.append("## Lanes\n")
    order = [CLAIM_CERTIFIED, CLAIM_NARROWED, CLAIM_OVERLAY, CLAIM_UNRECON, CLAIM_LABS]
    entries_by_claim: dict[str, list[dict]] = {claim: [] for claim in order}
    for entry in profile_matrix["entries"]:
        entries_by_claim.setdefault(entry["effective_causality_claim"], []).append(entry)
    for claim in order:
        bucket = entries_by_claim.get(claim, [])
        if not bucket:
            continue
        out.append(f"### {_CLAIM_HEADINGS[claim]}\n")
        for entry in bucket:
            ident = entry["identity"]
            out.append(f"#### `{entry['lane_id']}` ({entry['surface_family']})\n")
            out.append(
                f"- Claim: `{entry['claimed_causality_claim']}` → effective "
                f"`{entry['effective_causality_claim']}`"
            )
            out.append(
                f"- Origin `{entry['origin_class']}`, problem source "
                f"`{entry['problem_source_kind']}`, output channel "
                f"`{entry['output_channel_class']}`, posture `{entry['claim_posture']}`"
            )
            out.append(
                f"- Confidence `{entry['declared_confidence_tier']}` → effective "
                f"`{entry['effective_confidence_tier']}`; freshness "
                f"`{entry['declared_freshness_state']}`; reopen "
                f"`{entry['declared_reopen_target']}`; read-only "
                f"{str(entry['read_only']).lower()}"
            )
            out.append(
                "- Identity: run `{run}`, step `{step}`, provider `{prov}`, channel "
                "`{chan}`, build/toolchain `{bt}`, host/target `{ht}`, bundle `{bundle}`".format(
                    run=ident.get("run_ref"),
                    step=ident.get("step_ref"),
                    prov=ident.get("provider_ref"),
                    chan=ident.get("channel_ref"),
                    bt=ident.get("build_toolchain_ref"),
                    ht=ident.get("host_target_ref"),
                    bundle=ident.get("evidence_bundle_id"),
                )
            )
            out.append(
                f"- Evidence: proof `{entry['evidence']['proof_currency']}` "
                f"({entry['evidence']['freshness_state']})"
            )
            if entry["narrowed"]:
                out.append(f"- Narrowed (`{entry['downgrade_trigger']}`): {entry['narrowed_label']}")
                out.append(
                    "- Active reasons: " + ", ".join(entry["active_narrowing_reasons"])
                )
            out.append("")

    return "\n".join(out).rstrip() + "\n"


# --------------------------------------------------------------------------- #
# Corpus.
# --------------------------------------------------------------------------- #


def _set_path(target: dict, dotted: str, value) -> None:
    keys = dotted.split(".")
    cursor = target
    for key in keys[:-1]:
        cursor = cursor[key]
    cursor[keys[-1]] = value


def apply_overrides(row: dict, overrides: dict) -> dict:
    clone = json.loads(json.dumps(row))
    for dotted, value in overrides.items():
        _set_path(clone, dotted, value)
    return clone


def run_corpus(corpus_dir: Path, matrix: dict) -> list[str]:
    findings: list[str] = []
    index_path = corpus_dir / "index.json"
    if not index_path.exists():
        return [f"corpus index not found: {index_path}"]
    index = load_json(index_path)
    rows_by_id = {row["lane_id"]: row for row in matrix["rows"]}

    for case_name in index.get("cases", []):
        case_path = corpus_dir / case_name
        if not case_path.exists():
            findings.append(f"{case_name}: case file missing")
            continue
        case = load_json(case_path)
        case_id = case.get("case_id", case_name)
        kind = case.get("kind", "narrowing")

        if kind == "narrowing":
            base = rows_by_id.get(case.get("base_lane_id"))
            if base is None:
                findings.append(f"{case_id}: unknown base_lane_id {case.get('base_lane_id')}")
                continue
            row = apply_overrides(base, case.get("overrides", {}))
            stale_window = bool(case.get("stale_window", False))
            decision = narrow_row(row, stale_window)
            expected = case.get("expected", {})
            for key in ("claimed_causality_claim", "effective_causality_claim", "narrowed"):
                if key in expected and decision[key] != expected[key]:
                    findings.append(
                        f"{case_id}: {key} expected {expected[key]!r}, got {decision[key]!r}"
                    )
            if "active_narrowing_reasons" in expected and sorted(
                decision["active_narrowing_reasons"]
            ) != sorted(expected["active_narrowing_reasons"]):
                findings.append(
                    f"{case_id}: reasons expected {sorted(expected['active_narrowing_reasons'])}, "
                    f"got {sorted(decision['active_narrowing_reasons'])}"
                )
        elif kind == "projection_guard":
            mini = {
                "record_kind": PROFILE_MATRIX_RECORD_KIND,
                "schema_version": PROFILE_MATRIX_SCHEMA_VERSION,
                "matrix_id": PROFILE_MATRIX_ID,
                "as_of": case.get("as_of", matrix["verification_freshness"]["last_verification_refresh"]),
                "claim_separation": {
                    "stable_claim_statement": STABLE_CLAIM_STATEMENT,
                    "future_ambition_statement": FUTURE_AMBITION_STATEMENT,
                    "stable_distinguishes_claimed_from_future": True,
                },
                "entries": case["entries"],
                "publication": case.get("publication", {"decision": "hold"}),
                "summary": case.get("summary", {}),
            }
            mini_rows = {
                "rows": [rows_by_id[e["lane_id"]] for e in case["entries"] if e["lane_id"] in rows_by_id],
                "verification_freshness": matrix["verification_freshness"],
            }
            sub_findings = validate_profile_matrix(mini, mini_rows)
            expects_findings = bool(case.get("expected", {}).get("findings", True))
            if expects_findings and not sub_findings:
                findings.append(f"{case_id}: expected validation findings, got none")
            if not expects_findings and sub_findings:
                findings.append(f"{case_id}: expected clean validation, got {sub_findings}")
        else:
            findings.append(f"{case_id}: unknown case kind {kind}")
    return findings


# --------------------------------------------------------------------------- #
# CLI.
# --------------------------------------------------------------------------- #


def cmd_emit_matrix(args: argparse.Namespace) -> int:
    matrix = load_json(resolve(SOURCE_MATRIX_REF, args.source))
    profile_matrix = build_profile_matrix(matrix, args.as_of)
    out_path = resolve(PROFILE_MATRIX_REF, args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(profile_matrix, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(f"wrote {display_path(out_path)}")
    return 0


def cmd_emit_report(args: argparse.Namespace) -> int:
    matrix = load_json(resolve(SOURCE_MATRIX_REF, args.source))
    profile_matrix = build_profile_matrix(matrix, args.as_of)
    out_path = resolve(REPORT_REF, args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(render_report(profile_matrix), encoding="utf-8")
    print(f"wrote {display_path(out_path)}")
    return 0


def cmd_validate(args: argparse.Namespace) -> int:
    matrix = load_json(resolve(SOURCE_MATRIX_REF, args.source))
    profile_matrix = load_json(resolve(PROFILE_MATRIX_REF, args.profile_matrix))
    findings = validate_profile_matrix(profile_matrix, matrix)
    if findings:
        for finding in findings:
            print(f"execution-evidence causality failed: {finding}", file=sys.stderr)
        return 1
    print(
        json.dumps(
            {
                "status": "pass",
                "matrix": profile_matrix["matrix_id"],
                "decision": profile_matrix["publication"]["decision"],
                "narrowed_entries": profile_matrix["summary"]["narrowed_entries"],
            },
            indent=2,
            sort_keys=True,
        )
    )
    return 0


def cmd_corpus(args: argparse.Namespace) -> int:
    matrix = load_json(resolve(SOURCE_MATRIX_REF, args.source))
    findings = run_corpus(resolve(CORPUS_DIR_REF, args.corpus_dir), matrix)
    if findings:
        for finding in findings:
            print(f"execution-evidence corpus failed: {finding}", file=sys.stderr)
        return 1
    print(json.dumps({"status": "pass", "corpus": CORPUS_DIR_REF}, indent=2, sort_keys=True))
    return 0


def cmd_self_test(args: argparse.Namespace) -> int:
    matrix = load_json(resolve(SOURCE_MATRIX_REF, args.source))
    findings: list[str] = []

    # 1) The source packet validates against the canonical schema.
    findings += [f"schema: {f}" for f in validate_source_schema(matrix)]

    # 2) The generated matrix validates against the source it was built from.
    generated = build_profile_matrix(matrix, None)
    findings += [f"emit-roundtrip: {f}" for f in validate_profile_matrix(generated, matrix)]

    # 3) The checked-in artifact matches a fresh generation (no manual drift).
    committed_path = resolve(PROFILE_MATRIX_REF)
    if committed_path.exists():
        if load_json(committed_path) != generated:
            findings.append("checked-in matrix.json is stale; rerun emit-matrix")
    else:
        findings.append("matrix.json is not checked in; run emit-matrix")

    # 4) The checked-in report matches a fresh render.
    report_path = resolve(REPORT_REF)
    if report_path.exists():
        if report_path.read_text(encoding="utf-8") != render_report(generated):
            findings.append("checked-in report.md is stale; rerun emit-report")
    else:
        findings.append("report.md is not checked in; run emit-report")

    # 5) The fixture corpus passes.
    findings += [f"corpus: {f}" for f in run_corpus(resolve(CORPUS_DIR_REF), matrix)]

    if findings:
        for finding in findings:
            print(f"execution-evidence self-test failed: {finding}", file=sys.stderr)
        return 1
    print(json.dumps({"status": "pass"}, indent=2, sort_keys=True))
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--source", default=None, help="Override the source matrix path.")
    sub = parser.add_subparsers(dest="command")

    p_matrix = sub.add_parser("emit-matrix", help="Regenerate the causal-claim matrix JSON.")
    p_matrix.add_argument("--as-of", default=None, help="RFC 3339 evaluation timestamp.")
    p_matrix.add_argument("--out", default=None)
    p_matrix.set_defaults(func=cmd_emit_matrix)

    p_report = sub.add_parser("emit-report", help="Regenerate the certification report.")
    p_report.add_argument("--as-of", default=None)
    p_report.add_argument("--out", default=None)
    p_report.set_defaults(func=cmd_emit_report)

    p_validate = sub.add_parser("validate", help="Validate the checked-in claim matrix.")
    p_validate.add_argument("profile_matrix", nargs="?", default=None)
    p_validate.set_defaults(func=cmd_validate)

    p_corpus = sub.add_parser("corpus", help="Run the causality corpus.")
    p_corpus.add_argument("--corpus-dir", default=None)
    p_corpus.set_defaults(func=cmd_corpus)

    p_self = sub.add_parser("self-test", help="Schema + emit round-trip + corpus.")
    p_self.set_defaults(func=cmd_self_test)

    args = parser.parse_args(argv)
    if args.command is None:
        args = parser.parse_args((argv or []) + ["validate"])
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
