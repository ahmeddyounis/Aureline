#!/usr/bin/env python3
"""Freeze and certify the M5 draft-state / autosave / recover-draft set: how a
mutation-capable surface autosaves its edits to a local draft journal, keeps
draft-versus-applied state explicit, and recovers a draft after a crash, restart,
reconnect, or missing-dependency condition — without ever implying that local
draft state reached a remote target, provider, or protected file.

The canonical truth is the checked-in support export
(``artifacts/ux/m5-draft-state-and-autosave/support_export.json``). Each surface
binds its autosave journal, draft-versus-applied state, recover-draft semantics,
submit gate, backing freshness, and verification proof to one inspectable record.
This tool ingests that set and, per surface, **independently** re-derives an
effective claim that never reads wider than the evidence supports:

* the autosave indicator never claims a remote/synced target while only local
  draft state was saved;
* draft and applied state are distinguished, a local (draft-tier) value never
  reads as applied, and an applied state names its target;
* a recover-draft action stays available while a journal exists, never implies a
  remote write, never deletes unrelated workspace/profile state, and the
  crash-recovery surface can enumerate the affected forms/sheets;
* submit cannot proceed from an ambiguous draft/applied state, an imported/restore
  review never reads as a local submit, and no rendering surface overclaims;
* a surface whose autosave indicator overclaims, whose draft/applied state is
  ambiguous, whose local draft reads as applied, whose applied target is unnamed,
  whose recovery implies a remote write or deletes unrelated state, that loses the
  recover action while a journal exists, that cannot enumerate affected surfaces,
  that submits from ambiguous state, that lets an imported review submit, or that
  renders wider than its effective claim floors to draft_blocked and falls back to
  an explicit blocked state that names the reason.

The Rust truth source is ``crates/aureline-ui/src/m5_draft_state_and_autosave``;
this tool re-derives the same effective claim and narrowing reasons so the
checked-in artifacts can never imply a wider claim than the current evidence backs.

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
SUPPORT_EXPORT_REF = "artifacts/ux/m5-draft-state-and-autosave/support_export.json"
REPORT_REF = "artifacts/ux/m5-draft-state-and-autosave/report.md"
SCHEMA_REF = "schemas/ux/m5-draft-state-and-autosave.schema.json"
FIXTURE_DIR = "fixtures/ux/m5-draft-state-and-autosave"

RECORD_KIND = "m5_draft_state_set_packet"
SCHEMA_VERSION = 1
TAXONOMY_VERSION = 1

OVERLAY_ORIGINS = {"imported_or_restore"}

REDACTION_TOKENS = {
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
}

FORM_LANES = {"provider", "admin", "request", "package", "import", "settings", "projects"}
PERSISTENCE_TIERS = {
    "unsaved_in_memory",
    "local_journal",
    "local_durable_checkpoint",
    "committed_local",
    "committed_remote",
}
RECOVERY_AVAILABILITIES = {"recoverable", "recovered", "no_journal"}
INTERRUPTION_KINDS = {"none", "crash", "restart", "reconnect", "missing_dependency"}
AUTOSAVE_CLAIM_SCOPES = {"claims_local_only", "claims_remote_synced", "claims_none"}
CONSUMER_SURFACES = {
    "form_view",
    "wizard_step",
    "review_sheet",
    "diagnostics_panel",
    "support_export",
    "ai_evidence",
    "help_inline",
}

DRAFT_TIERS = {"unsaved_in_memory", "local_journal", "local_durable_checkpoint"}

LABS_CLAIM = "draft_labs_not_claimed"
CLAIM_RANK = {
    "draft_blocked": 0,
    "draft_review_overlay": 1,
    "draft_narrowed": 2,
    "draft_certified": 3,
}

# Deterministic reason ordering (mirrors the Rust order_index).
REASON_ORDER = {
    "autosave_overclaims_remote": 0,
    "draft_applied_ambiguous": 1,
    "local_draft_reads_as_applied": 2,
    "recover_implies_remote_write": 3,
    "submit_from_ambiguous_state": 4,
    "recovery_deletes_unrelated_state": 5,
    "applied_target_unnamed": 6,
    "recover_action_lost": 7,
    "affected_surfaces_unenumerable": 8,
    "imported_draft_reads_as_applied": 9,
    "rendering_overclaims": 10,
    "journal_backing_missing": 11,
    "autosave_state_unlabeled": 12,
    "autosave_pending": 13,
    "draft_unsaved_pending": 14,
    "freshness_unlabeled": 15,
    "superseded_state_not_marked": 16,
    "draft_stale": 17,
    "verification_proof_stale": 18,
    "verification_proof_missing": 19,
    "reopen_path_lost": 20,
}

# Floor reasons break the contract outright rather than merely aging out.
FLOOR_REASONS = {
    r for r, idx in REASON_ORDER.items() if idx <= REASON_ORDER["journal_backing_missing"]
}

FORBIDDEN_SUBSTRINGS = ("api_key", "password", "secret", "bearer ")


def present(value) -> bool:
    return isinstance(value, str) and value.strip() != ""


def is_local_only(tier: str) -> bool:
    return tier != "committed_remote"


def is_draft_tier(tier: str) -> bool:
    return tier in DRAFT_TIERS


def asserts_applied(state: str) -> bool:
    return state in ("partially_applied", "applied")


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


def claimed_claim(rec: dict) -> str:
    if rec["claim_posture"] == "labs_unadvertised":
        return LABS_CLAIM
    if rec["origin"] in OVERLAY_ORIGINS:
        return "draft_review_overlay"
    return "draft_certified"


def intrinsic_reasons(rec: dict, stale_window: bool) -> list[str]:
    j = rec["journal"]
    ds = rec["draft_state"]
    recov = rec["recovery"]
    gate = rec["submit_gate"]
    integ = rec["integrity"]
    overlay = rec["origin"] in OVERLAY_ORIGINS
    tier = j["persistence_tier"]
    reasons: list[str] = []

    # The autosave indicator can never claim a remote/synced target while only
    # local draft state was saved.
    if j["autosave_claim_scope"] == "claims_remote_synced" and is_local_only(tier):
        reasons.append("autosave_overclaims_remote")
    if not integ["autosave_scope_truthful"]:
        reasons.append("autosave_overclaims_remote")

    # Draft and applied state must be distinguished.
    if (
        ds["draft_applied_state"] == "not_distinguished"
        or not ds["draft_distinct_from_applied"]
        or not integ["draft_applied_distinct"]
    ):
        reasons.append("draft_applied_ambiguous")

    # A draft-tier value can never read as applied.
    if is_draft_tier(tier) and ds["draft_applied_state"] == "applied":
        reasons.append("local_draft_reads_as_applied")
    if not integ["local_draft_not_remote"]:
        reasons.append("local_draft_reads_as_applied")

    # An applied state must name its target.
    if asserts_applied(ds["draft_applied_state"]) and (
        not ds["applied_target_named"] or not integ["applied_target_disclosed"]
    ):
        reasons.append("applied_target_unnamed")

    # Recovering a draft must never imply a remote write.
    if recov["recover_implies_remote_write"]:
        reasons.append("recover_implies_remote_write")

    # Submit cannot proceed from an ambiguous draft/applied state.
    if gate["submit_allowed"] and (
        ds["draft_applied_state"] == "not_distinguished"
        or not ds["draft_distinct_from_applied"]
        or not gate["draft_applied_disambiguated_before_submit"]
    ):
        reasons.append("submit_from_ambiguous_state")

    # Recovery preserves unrelated workspace/profile state.
    if not recov["recover_preserves_unrelated_state"] or not integ["recovery_preserves_unrelated"]:
        reasons.append("recovery_deletes_unrelated_state")

    # A journal that exists must keep its recover-draft action.
    if recov["availability"] == "recoverable" and (
        not recov["recover_action_present"] or not integ["recovery_available_when_journal"]
    ):
        reasons.append("recover_action_lost")

    # Crash-recovery surfaces must enumerate the affected forms/sheets.
    if not recov["enumerates_affected_surfaces"] or not integ["affected_surfaces_enumerable"]:
        reasons.append("affected_surfaces_unenumerable")

    # Imported/restore overlay must stay a read-only review, never a submit.
    if overlay and (
        gate["submit_allowed"]
        or ds["draft_applied_state"] == "applied"
        or any(not r["read_only"] for r in rec["renderings"])
    ):
        reasons.append("imported_draft_reads_as_applied")

    # Backing freshness → journal backing.
    if not integ["freshness_state_visible"]:
        reasons.append("freshness_unlabeled")
    fs = rec["declared_freshness_state"]
    if fs == "missing":
        reasons.append("journal_backing_missing")
    elif fs == "superseded_by_newer_source" and not integ["superseded_state_marked"]:
        reasons.append("superseded_state_not_marked")
    elif fs == "stale_expired" and not overlay:
        reasons.append("draft_stale")

    # Autosave indicator labelling and pending states.
    if not j["indicator_labeled"]:
        reasons.append("autosave_state_unlabeled")
    if j["autosave_status"] == "saving":
        reasons.append("autosave_pending")
    if tier == "unsaved_in_memory" and ds["unsaved_change_count"] > 0:
        reasons.append("draft_unsaved_pending")

    # Verification proof.
    pc = rec["verification"]["proof_currency"]
    if pc == "missing_proof":
        reasons.append("verification_proof_missing")
    elif pc in ("stale_expired", "requires_review"):
        reasons.append("verification_proof_stale")
    elif pc in ("verified_current", "cached_within_window") and stale_window:
        reasons.append("verification_proof_stale")

    # Reopen-to-origin.
    if not integ["reopen_visible_on_demand"] or any(
        not r["provenance_visible"] for r in rec["renderings"]
    ):
        reasons.append("reopen_path_lost")

    return reasons


def derive_effective(claimed: str, reasons: list[str]) -> str:
    if any(r in FLOOR_REASONS for r in reasons):
        return "draft_blocked"
    if not reasons:
        return claimed
    if claimed == "draft_review_overlay":
        return "draft_blocked"
    return "draft_narrowed"


def record_reasons(rec: dict, stale_window: bool) -> list[str]:
    claimed = claimed_claim(rec)
    reasons = intrinsic_reasons(rec, stale_window)
    intrinsic = derive_effective(claimed, reasons)
    if any(overclaims(intrinsic, r["rendered_claim"]) for r in rec["renderings"]):
        reasons.append("rendering_overclaims")
    return order_reasons(reasons)


def narrow(rec: dict, stale_window: bool) -> dict:
    claimed = claimed_claim(rec)
    if claimed == LABS_CLAIM:
        return {"claimed": LABS_CLAIM, "effective": LABS_CLAIM, "reasons": [], "narrowed": False}
    reasons = record_reasons(rec, stale_window)
    effective = derive_effective(claimed, reasons)
    er = CLAIM_RANK.get(effective)
    cr = CLAIM_RANK.get(claimed)
    narrowed = er is not None and cr is not None and er < cr
    return {"claimed": claimed, "effective": effective, "reasons": reasons, "narrowed": narrowed}


def floored_keeps_fallback(rec: dict, effective: str) -> bool:
    if effective != "draft_blocked":
        return True
    return rec["declared_blocked_fallback"] in ("shows_reason_on_submit", "disabled_with_hint")


def record_overclaims(rec: dict, effective: str) -> bool:
    return any(overclaims(effective, r["rendered_claim"]) for r in rec["renderings"])


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
    surfaces = packet.get("surfaces", [])
    if not surfaces:
        v.append("empty_surfaces")

    seen: set[str] = set()
    lanes: set[str] = set()
    tiers: set[str] = set()
    availabilities: set[str] = set()
    interruptions: set[str] = set()
    claim_scopes: set[str] = set()
    consumers: set[str] = set()
    demonstrates_narrowing = False
    stale_window = False  # canonical packet is evaluated at its own as_of

    for s in surfaces:
        sid = s.get("surface_id", "")
        if sid in seen:
            v.append("duplicate_surface_id")
        seen.add(sid)
        lanes.add(s.get("lane"))
        tiers.add(s.get("journal", {}).get("persistence_tier"))
        availabilities.add(s.get("recovery", {}).get("availability"))
        interruptions.add(s.get("recovery", {}).get("interruption_kind"))
        claim_scopes.add(s.get("journal", {}).get("autosave_claim_scope"))
        for r in s.get("renderings", []):
            consumers.add(r.get("surface"))

        if (
            not present(s.get("surface_id"))
            or not present(s.get("label_summary"))
            or not present(s.get("lineage", {}).get("session_ref"))
        ):
            v.append("surface_missing_identity")
        if s.get("origin") in OVERLAY_ORIGINS and not (
            present(s.get("lineage", {}).get("provider_ref"))
            or present(s.get("lineage", {}).get("source_artifact_ref"))
        ):
            v.append("overlay_missing_provenance_ref")
        if not s.get("renderings"):
            v.append("surface_missing_rendering")
        for r in s.get("renderings", []):
            if not present(r.get("source_surface_ref")):
                v.append("rendering_missing_source_ref")

        decision = narrow(s, stale_window)
        if decision["narrowed"]:
            demonstrates_narrowing = True
            if not decision["reasons"]:
                v.append("narrowed_surface_missing_label_or_trigger")
        if not floored_keeps_fallback(s, decision["effective"]):
            v.append("floored_surface_loses_fallback")
        if record_overclaims(s, decision["effective"]):
            v.append("rendering_surface_overclaims")

    if not FORM_LANES.issubset(lanes):
        v.append("form_lane_missing")
    if not PERSISTENCE_TIERS.issubset(tiers):
        v.append("persistence_tier_missing")
    if not RECOVERY_AVAILABILITIES.issubset(availabilities):
        v.append("recovery_availability_missing")
    if not INTERRUPTION_KINDS.issubset(interruptions):
        v.append("interruption_kind_missing")
    if not AUTOSAVE_CLAIM_SCOPES.issubset(claim_scopes):
        v.append("autosave_claim_scope_missing")
    if not CONSUMER_SURFACES.issubset(consumers):
        v.append("consumer_surface_missing")
    if not demonstrates_narrowing:
        v.append("downgraded_surface_case_missing")
    if contains_forbidden(packet):
        v.append("raw_boundary_material_in_export")

    out: list[str] = []
    for item in v:
        if item not in out:
            out.append(item)
    return out


def claim_distribution(surfaces: list[dict]) -> dict:
    dist = {"certified": 0, "narrowed": 0, "overlay": 0, "blocked": 0, "labs": 0}
    bucket = {
        "draft_certified": "certified",
        "draft_narrowed": "narrowed",
        "draft_review_overlay": "overlay",
        "draft_blocked": "blocked",
        LABS_CLAIM: "labs",
    }
    for s in surfaces:
        dist[bucket[narrow(s, False)["effective"]]] += 1
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


def apply_overrides(rec: dict, overrides: dict) -> dict:
    out = json.loads(json.dumps(rec))
    for dotted, value in overrides.items():
        _set_path(out, dotted.split("."), value)
    return out


def base_record(surfaces: list[dict], sid: str) -> dict:
    for s in surfaces:
        if s["surface_id"] == sid:
            return s
    raise SystemExit(f"base surface not found: {sid}")


F_PROVIDER = "form:provider-connection:0001"
F_SETTINGS = "form:settings-config:0001"
F_PROJECTS = "wizard:project-bootstrap:0001"
F_PACKAGE = "sheet:package-install:0001"
F_ADMIN = "sheet:admin-policy:0001"
F_REQUEST = "dialog:request-run:0001"
F_IMPORT = "dialog:migration-restore:0001"
F_LABS = "wizard:labs-onboarding:0001"

BLOCKED = "draft_blocked"
NARROW = "draft_narrowed"
CERT = "draft_certified"
OVERLAY = "draft_review_overlay"

# (case_id, base_id, overrides, stale_window, description, effective, narrowed, reasons)
CASES = [
    ("clean-certified", F_SETTINGS, {}, False,
     "A settings editor applied to the local store with an honest local-only autosave indicator certifies.",
     CERT, False, []),
    ("provider-draft-certified", F_PROVIDER, {}, False,
     "A provider form whose edits autosave to a recoverable local journal and claim a local draft certifies.",
     CERT, False, []),
    ("request-narrowed-baseline", F_REQUEST, {}, False,
     "The request composer narrows while an autosave write is in flight.",
     NARROW, True, ["autosave_pending"]),
    ("import-overlay-baseline", F_IMPORT, {}, False,
     "The migration restore review stays a read-only review overlay.",
     OVERLAY, False, []),
    ("labs-not-claimed", F_LABS, {}, False,
     "A Labs onboarding surface makes no public claim and is never widened or narrowed.",
     LABS_CLAIM, False, []),

    # Floors.
    ("autosave-overclaims-remote", F_PROVIDER,
     {"journal.autosave_claim_scope": "claims_remote_synced", "renderings[*].rendered_claim": BLOCKED}, False,
     "An autosave indicator that claims a remote sync for a local-only draft floors.",
     BLOCKED, True, ["autosave_overclaims_remote"]),
    ("draft-applied-ambiguous", F_PROVIDER,
     {"draft_state.draft_distinct_from_applied": False, "submit_gate.submit_allowed": False,
      "renderings[*].rendered_claim": BLOCKED}, False,
     "A surface that does not distinguish draft from applied state floors.",
     BLOCKED, True, ["draft_applied_ambiguous"]),
    ("local-draft-reads-as-applied", F_PROVIDER,
     {"draft_state.draft_applied_state": "applied", "renderings[*].rendered_claim": BLOCKED}, False,
     "A draft-tier value labelled fully applied floors.",
     BLOCKED, True, ["local_draft_reads_as_applied"]),
    ("recover-implies-remote-write", F_PROVIDER,
     {"recovery.recover_implies_remote_write": True, "renderings[*].rendered_claim": BLOCKED}, False,
     "A recover-draft action that implies a remote write floors.",
     BLOCKED, True, ["recover_implies_remote_write"]),
    ("submit-from-ambiguous-state", F_PROVIDER,
     {"submit_gate.draft_applied_disambiguated_before_submit": False,
      "renderings[*].rendered_claim": BLOCKED}, False,
     "An open submit gate that does not disambiguate draft from applied first floors.",
     BLOCKED, True, ["submit_from_ambiguous_state"]),
    ("recovery-deletes-unrelated-state", F_PROVIDER,
     {"recovery.recover_preserves_unrelated_state": False, "renderings[*].rendered_claim": BLOCKED}, False,
     "Recovery that deletes unrelated workspace/profile state floors.",
     BLOCKED, True, ["recovery_deletes_unrelated_state"]),
    ("applied-target-unnamed", F_SETTINGS,
     {"draft_state.applied_target_named": False, "renderings[*].rendered_claim": BLOCKED}, False,
     "An applied state that does not name its target floors.",
     BLOCKED, True, ["applied_target_unnamed"]),
    ("recover-action-lost", F_PROVIDER,
     {"recovery.recover_action_present": False, "renderings[*].rendered_claim": BLOCKED}, False,
     "Losing the recover-draft action while a journal exists floors.",
     BLOCKED, True, ["recover_action_lost"]),
    ("affected-surfaces-unenumerable", F_PROVIDER,
     {"recovery.enumerates_affected_surfaces": False, "renderings[*].rendered_claim": BLOCKED}, False,
     "A crash-recovery surface that cannot enumerate the affected forms/sheets floors.",
     BLOCKED, True, ["affected_surfaces_unenumerable"]),
    ("imported-draft-reads-as-applied", F_IMPORT,
     {"submit_gate.submit_allowed": True, "renderings[*].rendered_claim": BLOCKED}, False,
     "An imported/restore review that allows a local submit floors below the review overlay.",
     BLOCKED, True, ["imported_draft_reads_as_applied"]),
    ("rendering-overclaims", F_REQUEST,
     {"renderings[0].rendered_claim": CERT}, False,
     "A narrowed surface whose rendering still shows certified floors as an overclaim.",
     BLOCKED, True, ["rendering_overclaims", "autosave_pending"]),
    ("missing-journal-backing", F_SETTINGS,
     {"declared_freshness_state": "missing", "renderings[*].rendered_claim": BLOCKED}, False,
     "Missing journal backing data floors the surface.",
     BLOCKED, True, ["journal_backing_missing"]),

    # Narrows.
    ("unlabeled-autosave-indicator", F_SETTINGS,
     {"journal.indicator_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the autosave indicator state narrows the surface.",
     NARROW, True, ["autosave_state_unlabeled"]),
    ("unsaved-in-memory-edits", F_PROJECTS,
     {"draft_state.unsaved_change_count": 2, "renderings[*].rendered_claim": NARROW}, False,
     "Unsaved in-memory edits not yet journaled narrow the surface.",
     NARROW, True, ["draft_unsaved_pending"]),
    ("unlabeled-freshness", F_SETTINGS,
     {"integrity.freshness_state_visible": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the backing freshness state narrows the surface.",
     NARROW, True, ["freshness_unlabeled"]),
    ("first-party-stale", F_SETTINGS,
     {"declared_freshness_state": "stale_expired", "renderings[*].rendered_claim": NARROW}, False,
     "A first-party stale backing source narrows rather than reading as fresh.",
     NARROW, True, ["draft_stale"]),
    ("superseded-unmarked", F_SETTINGS,
     {"declared_freshness_state": "superseded_by_newer_source",
      "integrity.superseded_state_marked": False, "renderings[*].rendered_claim": NARROW}, False,
     "An unmarked superseded backing source narrows the surface.",
     NARROW, True, ["superseded_state_not_marked"]),
    ("superseded-marked-ok", F_SETTINGS,
     {"declared_freshness_state": "superseded_by_newer_source"}, False,
     "A marked superseded backing source stays certified because the state is visible.",
     CERT, False, []),
    ("proof-missing", F_SETTINGS,
     {"verification.proof_currency": "missing_proof", "verification.proof_ref": None,
      "renderings[*].rendered_claim": NARROW}, False,
     "A missing verification proof narrows the surface.",
     NARROW, True, ["verification_proof_missing"]),
    ("proof-requires-review", F_SETTINGS,
     {"verification.proof_currency": "requires_review", "renderings[*].rendered_claim": NARROW}, False,
     "A verification proof that requires review narrows the surface.",
     NARROW, True, ["verification_proof_stale"]),
    ("stale-window", F_SETTINGS,
     {"renderings[*].rendered_claim": NARROW}, True,
     "An elapsed verification window ages a current proof out to narrowed.",
     NARROW, True, ["verification_proof_stale"]),
    ("reopen-path-lost", F_SETTINGS,
     {"integrity.reopen_visible_on_demand": False, "renderings[*].rendered_claim": NARROW}, False,
     "Losing the reopen-to-origin path narrows the surface.",
     NARROW, True, ["reopen_path_lost"]),
    ("overlay-any-gap-floors", F_IMPORT,
     {"integrity.freshness_state_visible": False, "renderings[*].rendered_claim": BLOCKED}, False,
     "A review overlay with any non-floor gap drops below the overlay rather than holding it.",
     BLOCKED, True, ["freshness_unlabeled"]),
]


def run_corpus_from_cases(surfaces: list[dict]) -> list[str]:
    failures: list[str] = []
    for case in CASES:
        case_id, base_id, overrides, stale_window, _desc, exp_eff, exp_narrowed, exp_reasons = case
        r = apply_overrides(base_record(surfaces, base_id), overrides)
        decision = narrow(r, stale_window)
        if decision["effective"] != exp_eff:
            failures.append(f"{case_id}: effective {decision['effective']} != {exp_eff}")
        if decision["narrowed"] != exp_narrowed:
            failures.append(f"{case_id}: narrowed {decision['narrowed']} != {exp_narrowed}")
        if decision["reasons"] != exp_reasons:
            failures.append(f"{case_id}: reasons {decision['reasons']} != {exp_reasons}")
    return failures


def run_corpus_from_disk(repo_root: Path, surfaces: list[dict]) -> list[str]:
    out_dir = repo_root / FIXTURE_DIR
    index_path = out_dir / "index.json"
    if not index_path.exists():
        return [f"missing corpus index: {index_path}"]
    index = json.loads(index_path.read_text(encoding="utf-8"))
    failures: list[str] = []
    for filename in index["cases"]:
        payload = json.loads((out_dir / filename).read_text(encoding="utf-8"))
        case_id = payload["case_id"]
        r = apply_overrides(base_record(surfaces, payload["base_surface_id"]), payload["overrides"])
        decision = narrow(r, payload["stale_window"])
        exp = payload["expected"]
        if decision["effective"] != exp["effective_claim"]:
            failures.append(
                f"{case_id}: effective {decision['effective']} != {exp['effective_claim']}"
            )
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
            "base_surface_id": base_id,
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
        "corpus_id": "m5-draft-state-and-autosave-corpus:0001",
        "description": (
            "Perturbation corpus for the draft-state / autosave / recover-draft narrowing engine. "
            "Each case starts from a canonical surface, applies dotted-path overrides, and asserts "
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
        sys.stderr.write("draft-state set FAILED validation:\n")
        for v in violations:
            sys.stderr.write(f"  - {v}\n")
        return 1
    dist = claim_distribution(packet["surfaces"])
    sys.stdout.write(
        f"draft-state set OK: {len(packet['surfaces'])} surfaces, distribution {dist}\n"
    )
    return 0


def cmd_corpus(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    surfaces = packet["surfaces"]
    failures = run_corpus_from_cases(surfaces)
    failures += run_corpus_from_disk(repo_root, surfaces)
    if failures:
        sys.stderr.write("draft-state corpus FAILED:\n")
        for f in failures:
            sys.stderr.write(f"  - {f}\n")
        return 1
    sys.stdout.write(f"draft-state corpus OK: {len(CASES)} cases\n")
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
    parser.add_argument("command", choices=["validate", "corpus", "emit-corpus", "self-test"])
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
