#!/usr/bin/env python3
"""Certify claimed voice profiles and auto-narrow voice-bearing public claims.

This is the release-automation and claim-governance face of the M5 voice
qualification lane. The canonical truth is the checked-in voice qualification
matrix support export (``artifacts/voice/m5-voice-qualification-matrix/``); this
tool ingests it and, per claimed voice profile, **independently** re-evaluates
the axes that back a public voice claim — command-versus-dictation mode
separation, push-to-talk default, provider/locality disclosure, transcript/audio
boundaries, command parity, keyboard fallback, provider availability, and
verification freshness — and computes an effective public claim that never reads
wider than the evidence supports.

A claimed profile auto-narrows below its headline claim whenever any of those
axes fail or its verification proof is stale, missing, or imported where a local
proof was required. A fully disabled provider floors the claim at
"voice unsupported, keyboard fallback only". Labs/unadvertised profiles make no
public claim and are never widened.

The tool both *generates* the surface-facing artifacts (the profile-claim matrix
JSON and the qualification report) and *validates* them, so release notes,
Help/About, service-health, support export, and public-proof surfaces ingest one
governed projection instead of cloning voice-state text by hand — and so a
checked-in artifact can never imply broader stable voice support than the current
qualification evidence backs.

Subcommands::

    emit-matrix    Regenerate artifacts/voice/m5-voice-profile-matrix.json
    emit-report    Regenerate artifacts/voice/m5-voice-qualification-report.md
    validate       Re-derive from the source matrix and fail on any overclaim
    corpus         Run the narrowing engine over fixtures/voice/qualification-corpus
    self-test      End-to-end check: emit round-trips clean and the corpus passes

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

SOURCE_MATRIX_REF = "artifacts/voice/m5-voice-qualification-matrix/support_export.json"
PROFILE_MATRIX_REF = "artifacts/voice/m5-voice-profile-matrix.json"
REPORT_REF = "artifacts/voice/m5-voice-qualification-report.md"
CORPUS_DIR_REF = "fixtures/voice/qualification-corpus"

MATRIX_SCHEMA_REF = "schemas/voice/m5-voice-qualification-matrix.schema.json"
TRUTH_DOC_REF = "docs/ux/voice-mode-and-privacy-truth.md"

PROFILE_MATRIX_RECORD_KIND = "m5_voice_profile_claim_matrix"
PROFILE_MATRIX_SCHEMA_VERSION = 1
PROFILE_MATRIX_ID = "m5-voice-profile-matrix:stable:0001"

# --------------------------------------------------------------------------- #
# Public claim ladder. A higher rank is a stronger public claim, so a narrowed
# row must move strictly lower. These tokens are what surfaces render.
# --------------------------------------------------------------------------- #

CLAIM_ON_PROFILE = "voice_capable_on_claimed_profile"
CLAIM_NARROWED = "voice_capable_narrowed_profile"
CLAIM_LABS = "labs_unadvertised_not_claimed"
CLAIM_UNSUPPORTED = "voice_unsupported_keyboard_fallback"

PUBLIC_CLAIM_RANK = {
    CLAIM_UNSUPPORTED: 0,
    CLAIM_LABS: 1,
    CLAIM_NARROWED: 2,
    CLAIM_ON_PROFILE: 3,
}
PUBLIC_CLAIM_BY_RANK = {rank: token for token, rank in PUBLIC_CLAIM_RANK.items()}

# Qualification grade (from the source matrix) -> headline public claim.
GRADE_TO_PUBLIC_CLAIM = {
    "qualified_claimed_profile": CLAIM_ON_PROFILE,
    "qualified_narrowed_profile": CLAIM_NARROWED,
    "labs_unadvertised_profile": CLAIM_LABS,
    "qualification_withdrawn": CLAIM_UNSUPPORTED,
    "not_applicable": CLAIM_UNSUPPORTED,
}

# Public surfaces that render a voice claim. A projection must never render a
# claim wider than the row's effective public claim.
REQUIRED_SURFACES = (
    "release_notes",
    "website_docs",
    "help_about",
    "product_badges",
    "service_health",
    "cli_inspection",
    "support_export",
    "public_proof_packet",
)

CLAIMED_POSTURES = {"claimed_beta", "claimed_preview"}
EXPLICIT_ACTIVATIONS = {
    "push_to_talk_held",
    "push_to_talk_toggle",
    "manual_command_activation",
}
EXPLICIT_MODES = {
    "idle_microphone_off",
    "dictation_mode_active",
    "command_mode_active",
    "continuous_listening_active_user_opted_in",
}
HOSTED_PROVIDER_CLASSES = {"approved_remote_disclosed", "enterprise_relay_managed"}
DISCLOSED_TRANSPORTS = {
    "policy_bounded_disclosed_endpoint",
    "explicit_opt_in_disclosed_endpoint",
}
BOUNDED_TRANSCRIPT_EXPORTS = {
    "no_transcript_export",
    "explicit_user_export_redacted",
    "metadata_only_support_export",
    "export_blocked_by_policy",
}
PROVIDER_OR_IMPORTED_ORIGINS = {"provider_linked_profile", "imported_read_only_profile"}
LOCAL_PROOF_CURRENCIES = {"verified_current", "cached_within_window"}
PARITY_KEYS = (
    "stable_command_ids",
    "disabled_with_reason",
    "preview_apply_revert",
    "approval_requirements",
    "undo_grouping",
    "audit_support_lineage",
    "high_impact_review",
    "keyboard_fallback_parity",
)

# Narrowing reasons recorded on a narrowed row. The first six mirror the matrix
# downgrade triggers; the last three cover stale/missing/mismatched proof.
NARROW_MODE_SEPARATION = "mode_separation_unverified"
NARROW_PUSH_TO_TALK = "push_to_talk_default_missing"
NARROW_LOCALITY = "provider_locality_undisclosed"
NARROW_TRANSCRIPT = "transcript_retention_unbounded"
NARROW_PARITY = "command_parity_incomplete"
NARROW_FALLBACK = "keyboard_fallback_missing"
NARROW_PROVIDER_UNAVAILABLE = "provider_unavailable_downgraded"
NARROW_STALE_PROOF = "stale_verification_proof"
NARROW_MISSING_PROOF = "missing_verification_proof"
NARROW_IMPORTED_PROOF = "imported_proof_on_local_profile"


# --------------------------------------------------------------------------- #
# Axis evaluators — mirror the Rust matrix invariants so this tool re-derives
# the truth rather than trusting the recorded effective grade.
# --------------------------------------------------------------------------- #


def _is_claimed(row: dict) -> bool:
    return row.get("claim_posture") in CLAIMED_POSTURES


def mode_separation_ok(row: dict) -> bool:
    """Command vs dictation stays an explicit live mode (not a policy block)."""
    return row["session"]["mode_class"] in EXPLICIT_MODES


def push_to_talk_default_ok(row: dict) -> bool:
    """Activation is explicit by default, or an opted-in continuous/wake mode."""
    session = row["session"]
    if session["activation_class"] in EXPLICIT_ACTIVATIONS:
        return True
    return (
        session["activation_class"] == "wake_phrase_continuous_user_opted_in"
        and session["background_listening_state"] == "on_user_opted_in"
    )


def locality_disclosed_ok(row: dict) -> bool:
    """A hosted provider discloses a remote handoff, hosted locality, and audit."""
    provider = row["provider"]
    if provider["provider_class"] not in HOSTED_PROVIDER_CLASSES:
        return True
    return (
        provider["transport_class"] in DISCLOSED_TRANSPORTS
        and provider["processing_locality"] == "hosted_remote_disclosed"
        and bool(provider.get("audit_capable", False))
    )


def retention_bounded_for_claim(row: dict) -> bool:
    """Transcript retention stays bounded for a fully-claimed profile.

    A deliberately narrowed profile may carry provider-per-contract retention;
    a fully-claimed profile may not.
    """
    if row.get("claimed_grade") != "qualified_claimed_profile":
        return True
    posture = row["provider"]["retention_posture"]
    if posture["audio_retention"] == "audio_retained_provider_per_contract":
        return False
    if posture["retention_mode"] == "transcript_retained_provider_per_contract":
        return False
    return posture["transcript_export"] in BOUNDED_TRANSCRIPT_EXPORTS


def parity_complete(row: dict) -> bool:
    """Every command-parity guarantee holds."""
    parity = row["command_parity"]
    return all(bool(parity.get(key, False)) for key in PARITY_KEYS)


def keyboard_fallback_ok(row: dict) -> bool:
    """Keyboard fallback is complete on both the provider and the session."""
    return bool(row["provider"].get("keyboard_fallback_available", False)) and bool(
        row["session"].get("keyboard_fallback_available", False)
    )


def provider_unavailable(row: dict) -> bool:
    """The provider is disabled or capture/processing is blocked."""
    provider = row["provider"]
    session = row["session"]
    return (
        provider["provider_class"] == "provider_disabled"
        or session["processing_locality"] == "processing_unavailable"
        or session["mode_class"]
        in {"voice_mode_blocked_by_policy", "voice_mode_blocked_by_envelope"}
    )


def provider_fully_disabled(row: dict) -> bool:
    """Capture cannot run at all — voice floors at keyboard-only."""
    provider = row["provider"]
    return (
        provider["provider_class"] == "provider_disabled"
        or provider["retention_posture"]["audio_retention"] == "audio_capture_blocked"
    )


def imported_proof_mismatch(row: dict) -> bool:
    """Imported proof stands in for a local claim, or vice versa."""
    provider_or_imported = row.get("origin_class") in PROVIDER_OR_IMPORTED_ORIGINS
    currency = row["verification"]["proof_currency"]
    if provider_or_imported:
        return currency in LOCAL_PROOF_CURRENCIES
    return currency == "imported_current"


def proof_reason(row: dict, stale_window: bool) -> str | None:
    """Reason a row's verification proof fails to back a current claim."""
    currency = row["verification"]["proof_currency"]
    if currency == "missing_proof":
        return NARROW_MISSING_PROOF
    if currency in {"stale_expired", "requires_review"}:
        return NARROW_STALE_PROOF
    # Freshness overlay: once the matrix freshness window has elapsed, a claim
    # resting on a local/cached proof is no longer current and must narrow.
    # Imported/provider-attested currency carries its own freshness contract.
    if stale_window and currency in LOCAL_PROOF_CURRENCIES:
        return NARROW_STALE_PROOF
    return None


def _dedup(reasons: list[str]) -> list[str]:
    seen: set[str] = set()
    ordered: list[str] = []
    for reason in reasons:
        if reason not in seen:
            seen.add(reason)
            ordered.append(reason)
    return ordered


# --------------------------------------------------------------------------- #
# Narrowing engine.
# --------------------------------------------------------------------------- #


def narrow_row(row: dict, stale_window: bool) -> dict:
    """Compute the effective public claim and narrowing reasons for one row.

    ``stale_window`` is true when the matrix verification-freshness window has
    elapsed as of the evaluation timestamp, which narrows any claim resting on a
    local/cached proof.
    """
    claimed_public = GRADE_TO_PUBLIC_CLAIM[row["claimed_grade"]]
    matrix_effective_public = GRADE_TO_PUBLIC_CLAIM[row["effective_grade"]]
    is_claimed = _is_claimed(row)

    reasons: list[str] = []
    for failed, token in (
        (not mode_separation_ok(row), NARROW_MODE_SEPARATION),
        (not push_to_talk_default_ok(row), NARROW_PUSH_TO_TALK),
        (not locality_disclosed_ok(row), NARROW_LOCALITY),
        (not retention_bounded_for_claim(row), NARROW_TRANSCRIPT),
        (not parity_complete(row), NARROW_PARITY),
        (not keyboard_fallback_ok(row), NARROW_FALLBACK),
        (provider_unavailable(row), NARROW_PROVIDER_UNAVAILABLE),
        (imported_proof_mismatch(row), NARROW_IMPORTED_PROOF),
    ):
        if failed:
            reasons.append(token)
    proof = proof_reason(row, stale_window)
    if proof:
        reasons.append(proof)
    reasons = _dedup(reasons)

    # Labs/unadvertised rows make no public claim, so they never accrue
    # governance narrowing; they hold their (already non-claiming) grade.
    if not is_claimed:
        effective_rank = min(
            PUBLIC_CLAIM_RANK[claimed_public],
            PUBLIC_CLAIM_RANK[matrix_effective_public],
        )
        return {
            "claimed_public_claim": claimed_public,
            "effective_public_claim": PUBLIC_CLAIM_BY_RANK[effective_rank],
            "active_narrowing_reasons": [],
            "narrowed": PUBLIC_CLAIM_RANK[PUBLIC_CLAIM_BY_RANK[effective_rank]]
            < PUBLIC_CLAIM_RANK[claimed_public],
        }

    effective_rank = min(
        PUBLIC_CLAIM_RANK[claimed_public],
        PUBLIC_CLAIM_RANK[matrix_effective_public],
    )
    if reasons:
        # Any failing/stale axis narrows strictly below the headline claim.
        effective_rank = min(effective_rank, PUBLIC_CLAIM_RANK[claimed_public] - 1)
    if provider_fully_disabled(row):
        effective_rank = PUBLIC_CLAIM_RANK[CLAIM_UNSUPPORTED]
    effective_rank = max(PUBLIC_CLAIM_RANK[CLAIM_UNSUPPORTED], effective_rank)

    effective_public = PUBLIC_CLAIM_BY_RANK[effective_rank]
    return {
        "claimed_public_claim": claimed_public,
        "effective_public_claim": effective_public,
        "active_narrowing_reasons": reasons,
        "narrowed": PUBLIC_CLAIM_RANK[effective_public]
        < PUBLIC_CLAIM_RANK[claimed_public],
    }


def qualified_matrix(row: dict) -> dict:
    """Export-safe platform/provider matrix a profile is qualified on."""
    origin = row.get("origin_class")
    provider = row["provider"]
    if provider_fully_disabled(row):
        platforms = ["keyboard_fallback_only"]
    elif origin == "enterprise_managed_profile":
        platforms = ["enterprise_managed"]
    elif origin == "provider_linked_profile":
        platforms = ["provider_linked"]
    elif origin == "imported_read_only_profile":
        platforms = ["imported_read_only"]
    elif provider["processing_locality"] == "hosted_remote_disclosed":
        platforms = ["hosted_remote_disclosed"]
    else:
        platforms = ["desktop_local"]
    return {
        "platforms": platforms,
        "provider_class": provider["provider_class"],
        "processing_locality": provider["processing_locality"],
        "transport_class": provider["transport_class"],
    }


def downgrade_trigger_for(row: dict, decision: dict) -> str | None:
    """Single headline trigger recorded on a narrowed entry."""
    if not decision["narrowed"]:
        return None
    reasons = decision["active_narrowing_reasons"]
    if reasons:
        return reasons[0]
    # Narrowed by the source matrix without an independently-detected axis: quote
    # the matrix's own recorded trigger.
    return row.get("downgrade_trigger")


def narrowed_label_for(row: dict, decision: dict) -> str | None:
    """Precise, non-generic label for a narrowed entry."""
    if not decision["narrowed"]:
        return None
    if row.get("downgraded_label"):
        return row["downgraded_label"]
    trigger = downgrade_trigger_for(row, decision) or "narrowed"
    return (
        f"Held at {decision['effective_public_claim']} below the "
        f"{decision['claimed_public_claim']} claim: {trigger.replace('_', ' ')}; "
        "keyboard / command-palette path stays complete"
    )


# --------------------------------------------------------------------------- #
# Source matrix + freshness.
# --------------------------------------------------------------------------- #


def parse_rfc3339(value: str) -> datetime:
    """Parse an RFC 3339 timestamp (trailing ``Z`` accepted)."""
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
    """Repo-relative display where possible, else the absolute path."""
    try:
        return str(path.relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


# --------------------------------------------------------------------------- #
# Projection (the generated profile-claim matrix).
# --------------------------------------------------------------------------- #

STABLE_CLAIM_STATEMENT = (
    "Voice is supported only on the claimed, currently-qualified profiles in this "
    "matrix, each push-to-talk by default with command and dictation modes kept "
    "separate, provider locality and transcript retention disclosed, and full "
    "command/undo/policy parity with the keyboard."
)
FUTURE_AMBITION_STATEMENT = (
    "Broader always-listening, hosted-by-default, or general voice-assistant "
    "scope is not claimed for stable; such surfaces stay Labs/unadvertised and "
    "out of public voice support until separately qualified."
)


def build_profile_matrix(matrix: dict, as_of: str | None) -> dict:
    """Build the surface-facing profile-claim matrix from the source matrix."""
    freshness = matrix["verification_freshness"]
    effective_as_of = as_of or freshness["last_verification_refresh"]
    stale_window = freshness_stale(matrix, effective_as_of)

    entries = []
    surface_rows = 0
    narrowed = 0
    unsupported = 0
    overclaims = 0
    for row in matrix["rows"]:
        decision = narrow_row(row, stale_window)
        effective = decision["effective_public_claim"]
        projections = [
            {
                "surface_id": surface,
                "rendered_claim": effective,
                "source_matrix_ref": PROFILE_MATRIX_ID,
            }
            for surface in REQUIRED_SURFACES
        ]
        surface_rows += len(projections)
        if decision["narrowed"]:
            narrowed += 1
        if effective == CLAIM_UNSUPPORTED:
            unsupported += 1
        entries.append(
            {
                "profile_id": row["profile_id"],
                "surface_kind": row["surface_kind"],
                "origin_class": row["origin_class"],
                "claim_posture": row["claim_posture"],
                "claimed_grade": row["claimed_grade"],
                "effective_grade": row["effective_grade"],
                "claimed_public_claim": decision["claimed_public_claim"],
                "effective_public_claim": effective,
                "narrowed": decision["narrowed"],
                "active_narrowing_reasons": decision["active_narrowing_reasons"],
                "downgrade_trigger": downgrade_trigger_for(row, decision),
                "narrowed_label": narrowed_label_for(row, decision),
                "qualified_matrix": qualified_matrix(row),
                "evidence": {
                    "proof_currency": row["verification"]["proof_currency"],
                    "freshness_state": "stale" if stale_window else "fresh",
                    "proof_ref_present": bool(row["verification"].get("proof_ref")),
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
        "label": "M5 Voice Profile Claim / Auto-Narrowing Matrix",
        "source_matrix_ref": SOURCE_MATRIX_REF,
        "source_matrix_packet_id": matrix["packet_id"],
        "as_of": effective_as_of,
        "verification_freshness": {
            "verification_freshness_slo_hours": freshness[
                "verification_freshness_slo_hours"
            ],
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
            "unsupported_entries": unsupported,
            "surface_projection_rows": surface_rows,
            "overclaiming_surface_rows": overclaims,
        },
        "source_contract_refs": [
            SOURCE_MATRIX_REF,
            MATRIX_SCHEMA_REF,
            TRUTH_DOC_REF,
        ],
        "redaction_class_token": matrix.get("redaction_class_token", "metadata_safe_default"),
        "minted_at": effective_as_of,
    }


# --------------------------------------------------------------------------- #
# Validation.
# --------------------------------------------------------------------------- #


def validate_profile_matrix(profile_matrix: dict, matrix: dict) -> list[str]:
    """Re-derive from the source matrix and report every overclaim/mismatch."""
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

    rows_by_id = {row["profile_id"]: row for row in matrix["rows"]}
    entries = profile_matrix.get("entries", [])
    if {entry["profile_id"] for entry in entries} != set(rows_by_id):
        findings.append("entries do not cover the source matrix rows one-to-one")

    narrowed = 0
    unsupported = 0
    surface_rows = 0
    overclaims = 0
    seen: set[str] = set()
    for entry in entries:
        profile_id = entry.get("profile_id", "")
        if profile_id in seen:
            findings.append(f"{profile_id}: duplicate entry")
        seen.add(profile_id)
        row = rows_by_id.get(profile_id)
        if row is None:
            findings.append(f"{profile_id}: entry has no source matrix row")
            continue

        decision = narrow_row(row, stale_window)
        effective = decision["effective_public_claim"]
        if entry.get("effective_public_claim") != effective:
            findings.append(
                f"{profile_id}: recorded effective claim "
                f"{entry.get('effective_public_claim')} != derived {effective}"
            )
        if entry.get("claimed_public_claim") != decision["claimed_public_claim"]:
            findings.append(f"{profile_id}: recorded claimed claim mismatch")
        if sorted(entry.get("active_narrowing_reasons", [])) != sorted(
            decision["active_narrowing_reasons"]
        ):
            findings.append(
                f"{profile_id}: narrowing reasons mismatch: recorded "
                f"{sorted(entry.get('active_narrowing_reasons', []))} != derived "
                f"{sorted(decision['active_narrowing_reasons'])}"
            )
        recorded_narrowed = bool(entry.get("narrowed"))
        if recorded_narrowed != decision["narrowed"]:
            findings.append(f"{profile_id}: narrowed flag mismatch")
        if recorded_narrowed:
            narrowed += 1
            if not entry.get("downgrade_trigger"):
                findings.append(f"{profile_id}: narrowed entry missing downgrade_trigger")
            label = (entry.get("narrowed_label") or "").strip()
            if not label or label.lower() in _GENERIC_LABELS:
                findings.append(f"{profile_id}: narrowed entry missing a precise label")
        else:
            if entry.get("active_narrowing_reasons"):
                findings.append(f"{profile_id}: un-narrowed entry carries narrowing reasons")
        if effective == CLAIM_UNSUPPORTED:
            unsupported += 1

        effective_rank = PUBLIC_CLAIM_RANK[effective]
        projection_surfaces = []
        for projection in entry.get("surface_projections", []):
            surface_rows += 1
            projection_surfaces.append(projection.get("surface_id"))
            rendered = projection.get("rendered_claim")
            if rendered not in PUBLIC_CLAIM_RANK:
                findings.append(
                    f"{profile_id}: {projection.get('surface_id')} renders unknown claim {rendered}"
                )
                continue
            if PUBLIC_CLAIM_RANK[rendered] > effective_rank:
                overclaims += 1
                findings.append(
                    f"{profile_id}: surface {projection.get('surface_id')} renders "
                    f"{rendered}, wider than effective {effective}"
                )
        if set(projection_surfaces) != set(REQUIRED_SURFACES):
            findings.append(f"{profile_id}: surface coverage mismatch")

    summary = profile_matrix.get("summary", {})
    expected_summary = {
        "total_entries": len(entries),
        "narrowed_entries": narrowed,
        "unsupported_entries": unsupported,
        "surface_projection_rows": surface_rows,
        "overclaiming_surface_rows": overclaims,
    }
    if summary != expected_summary:
        findings.append(f"summary mismatch: expected {expected_summary}, got {summary}")

    expected_decision = (
        "hold" if overclaims else "proceed_with_narrowing" if narrowed else "proceed"
    )
    if profile_matrix.get("publication", {}).get("decision") != expected_decision:
        findings.append(f"publication decision should be {expected_decision}")

    return findings


_GENERIC_LABELS = {
    "unavailable",
    "not available",
    "n/a",
    "error",
    "provider error",
    "request failed",
    "failed",
    "downgraded",
    "unverified",
    "narrowed",
}


# --------------------------------------------------------------------------- #
# Report.
# --------------------------------------------------------------------------- #


def render_report(profile_matrix: dict) -> str:
    """Deterministic Markdown certification report from the profile matrix."""
    out: list[str] = []
    out.append("# M5 Voice Profile Qualification & Claim-Narrowing Report\n")
    out.append(
        "Generated by `tools/release/voice_claim_narrowing.py` from "
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
        f"- Profiles: {summary['total_entries']} "
        f"({summary['narrowed_entries']} auto-narrowed, "
        f"{summary['unsupported_entries']} held at keyboard-only)"
    )
    out.append(f"- Publication decision: `{profile_matrix['publication']['decision']}`\n")

    separation = profile_matrix["claim_separation"]
    out.append("## Claimed scope vs future ambition\n")
    out.append(f"**Claimed (stable):** {separation['stable_claim_statement']}\n")
    out.append(f"**Not claimed (future):** {separation['future_ambition_statement']}\n")

    out.append("## Auto-narrowing rules\n")
    out.append(
        "A claimed profile auto-narrows below its headline claim when any axis "
        "fails or its verification evidence is stale, missing, or imported where a "
        "local proof was required:\n"
    )
    out.append("- command-versus-dictation mode separation lost → `mode_separation_unverified`")
    out.append("- push-to-talk / explicit-activation default missing → `push_to_talk_default_missing`")
    out.append("- hosted/enterprise provider locality undisclosed → `provider_locality_undisclosed`")
    out.append("- transcript/audio retention unbounded for a full claim → `transcript_retention_unbounded`")
    out.append("- command/undo/policy parity incomplete → `command_parity_incomplete`")
    out.append("- keyboard fallback missing → `keyboard_fallback_missing`")
    out.append("- provider unavailable / capture blocked → `provider_unavailable_downgraded`")
    out.append("- verification proof stale or window elapsed → `stale_verification_proof`")
    out.append("- verification proof missing → `missing_verification_proof`")
    out.append("- imported/local proof mismatch → `imported_proof_on_local_profile`\n")
    out.append(
        "A fully disabled provider floors the claim at "
        "`voice_unsupported_keyboard_fallback`. Labs/unadvertised profiles make no "
        "public claim and are never widened.\n"
    )

    out.append("## Profiles\n")
    for entry in profile_matrix["entries"]:
        verdict = "certified" if not entry["narrowed"] else "auto-narrowed"
        if entry["claim_posture"] == "labs_unadvertised":
            verdict = "labs / not claimed"
        out.append(
            f"### `{entry['profile_id']}` ({entry['surface_kind']}) — {verdict}\n"
        )
        out.append(
            f"- Claim: `{entry['claimed_public_claim']}` → effective "
            f"`{entry['effective_public_claim']}`"
        )
        out.append(
            f"- Posture `{entry['claim_posture']}`, origin `{entry['origin_class']}`, "
            f"claimed grade `{entry['claimed_grade']}` → effective `{entry['effective_grade']}`"
        )
        matrix = entry["qualified_matrix"]
        out.append(
            f"- Qualified matrix: platforms {matrix['platforms']}, provider "
            f"`{matrix['provider_class']}`, locality `{matrix['processing_locality']}`, "
            f"transport `{matrix['transport_class']}`"
        )
        evidence = entry["evidence"]
        out.append(
            f"- Evidence: proof `{evidence['proof_currency']}` "
            f"({evidence['freshness_state']})"
        )
        if entry["narrowed"]:
            out.append(
                f"- Narrowed (`{entry['downgrade_trigger']}`): {entry['narrowed_label']}"
            )
            out.append(
                f"- Active reasons: {', '.join(entry['active_narrowing_reasons'])}"
            )
        out.append("")
    return "\n".join(out).rstrip("\n") + "\n"


# --------------------------------------------------------------------------- #
# Corpus.
# --------------------------------------------------------------------------- #


def _set_path(target: dict, dotted: str, value) -> None:
    """Set ``dotted`` (e.g. ``provider.retention_posture.audio_retention``)."""
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
    """Run the narrowing engine over the fixture corpus; report mismatches."""
    findings: list[str] = []
    index_path = corpus_dir / "index.json"
    if not index_path.exists():
        return [f"corpus index not found: {index_path}"]
    index = load_json(index_path)
    rows_by_id = {row["profile_id"]: row for row in matrix["rows"]}

    for case_name in index.get("cases", []):
        case_path = corpus_dir / case_name
        if not case_path.exists():
            findings.append(f"{case_name}: case file missing")
            continue
        case = load_json(case_path)
        case_id = case.get("case_id", case_name)
        kind = case.get("kind", "narrowing")

        if kind == "narrowing":
            base = rows_by_id.get(case.get("base_profile_id"))
            if base is None:
                findings.append(f"{case_id}: unknown base_profile_id {case.get('base_profile_id')}")
                continue
            row = apply_overrides(base, case.get("overrides", {}))
            stale_window = bool(case.get("stale_window", False))
            decision = narrow_row(row, stale_window)
            expected = case.get("expected", {})
            for key in ("claimed_public_claim", "effective_public_claim", "narrowed"):
                if key in expected and decision[key] != expected[key]:
                    findings.append(
                        f"{case_id}: {key} expected {expected[key]!r}, got {decision[key]!r}"
                    )
            if "active_narrowing_reasons" in expected:
                if sorted(decision["active_narrowing_reasons"]) != sorted(
                    expected["active_narrowing_reasons"]
                ):
                    findings.append(
                        f"{case_id}: reasons expected "
                        f"{sorted(expected['active_narrowing_reasons'])}, got "
                        f"{sorted(decision['active_narrowing_reasons'])}"
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
            mini_rows = {"rows": [], "verification_freshness": matrix["verification_freshness"]}
            for entry in case["entries"]:
                base = rows_by_id.get(entry["profile_id"])
                if base is not None:
                    mini_rows["rows"].append(base)
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
    out_path.write_text(
        json.dumps(profile_matrix, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
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
            print(f"voice claim narrowing failed: {finding}", file=sys.stderr)
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
            print(f"voice claim corpus failed: {finding}", file=sys.stderr)
        return 1
    print(json.dumps({"status": "pass", "corpus": CORPUS_DIR_REF}, indent=2, sort_keys=True))
    return 0


def cmd_self_test(args: argparse.Namespace) -> int:
    matrix = load_json(resolve(SOURCE_MATRIX_REF, args.source))
    findings: list[str] = []

    # 1) The generated matrix validates against the source it was built from.
    generated = build_profile_matrix(matrix, None)
    findings += [f"emit-roundtrip: {f}" for f in validate_profile_matrix(generated, matrix)]

    # 2) The checked-in artifact matches a fresh generation (no manual drift).
    committed_path = resolve(PROFILE_MATRIX_REF)
    if committed_path.exists():
        committed = load_json(committed_path)
        if committed != generated:
            findings.append("checked-in m5-voice-profile-matrix.json is stale; rerun emit-matrix")
    else:
        findings.append("m5-voice-profile-matrix.json is not checked in; run emit-matrix")

    # 3) The fixture corpus passes.
    findings += [f"corpus: {f}" for f in run_corpus(resolve(CORPUS_DIR_REF), matrix)]

    if findings:
        for finding in findings:
            print(f"voice claim self-test failed: {finding}", file=sys.stderr)
        return 1
    print(json.dumps({"status": "pass"}, indent=2, sort_keys=True))
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--source", default=None, help="Override the source matrix path.")
    sub = parser.add_subparsers(dest="command")

    p_matrix = sub.add_parser("emit-matrix", help="Regenerate the profile-claim matrix JSON.")
    p_matrix.add_argument("--as-of", default=None, help="RFC 3339 evaluation timestamp.")
    p_matrix.add_argument("--out", default=None)
    p_matrix.set_defaults(func=cmd_emit_matrix)

    p_report = sub.add_parser("emit-report", help="Regenerate the qualification report.")
    p_report.add_argument("--as-of", default=None)
    p_report.add_argument("--out", default=None)
    p_report.set_defaults(func=cmd_emit_report)

    p_validate = sub.add_parser("validate", help="Validate the checked-in profile-claim matrix.")
    p_validate.add_argument("profile_matrix", nargs="?", default=None)
    p_validate.set_defaults(func=cmd_validate)

    p_corpus = sub.add_parser("corpus", help="Run the qualification corpus.")
    p_corpus.add_argument("--corpus-dir", default=None)
    p_corpus.set_defaults(func=cmd_corpus)

    p_self = sub.add_parser("self-test", help="Emit round-trip plus corpus.")
    p_self.set_defaults(func=cmd_self_test)

    args = parser.parse_args(argv)
    if args.command is None:
        args = parser.parse_args((argv or []) + ["validate"])
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
