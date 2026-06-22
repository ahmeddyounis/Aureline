#!/usr/bin/env python3
"""Freeze and certify the M5 staged-review (commit) sheet set: the commit sheet
itself, made the first-class object every consequential M5 mutation flow stops at
before it changes remote/provider/admin/package/request/import state.

One review model is reused across provider publish-later, admin/source-management,
request replay/mutation, package install/update/remove, and import/export/publish
flows. The canonical truth is the checked-in support export
(``artifacts/ux/m5-staged-review-sheets/support_export.json``). Each sheet binds
its target scope, its disclosed omitted defaults, a reconciled
included/excluded/blocked/hidden count block, a disclosed side-effect summary with
a rollback/export path, a scope-and-effect-specific commit action, its backing
freshness, and its verification proof to one inspectable record. This tool ingests
that set and, per sheet, **independently** re-derives an effective claim that never
reads wider than the evidence supports:

* the target scope is declared and visible (single object, an explicit
  multi-object selection, a query-backed selection, or a workspace-wide action);
* the included/excluded/blocked/hidden counts reconcile with the total matched, a
  query-backed/broad action that collapses members discloses a hidden count, and a
  multi-object action surfaces its counts;
* omitted defaults are disclosed, every side effect is disclosed before commit, a
  blocked prerequisite is explained, and the rollback/export consequence is visible;
* the commit action names the scope/effect rather than a generic Continue, an
  imported/restore review never reads as a local apply, and no rendering surface
  overclaims;
* a sheet whose scope is hidden, whose counts disagree, whose collapsed members are
  uncounted, whose included/excluded/blocked counts are hidden, whose omitted
  defaults or side effects are hidden, that buries a blocked prerequisite or
  rollback consequence behind a generic Continue, that lets an imported review read
  as an apply, that loses its reopen path, or that renders wider than its claim
  floors to sheet_unsafe and falls back to an explicit blocked state with a
  reopen/keyboard recovery path.

The Rust truth source is ``crates/aureline-ui/src/m5_staged_review_sheets``; this
tool re-derives the same effective claim and narrowing reasons so the checked-in
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
SUPPORT_EXPORT_REF = "artifacts/ux/m5-staged-review-sheets/support_export.json"
REPORT_REF = "artifacts/ux/m5-staged-review-sheets/report.md"
SCHEMA_REF = "schemas/ux/m5-staged-review-sheets.schema.json"
FIXTURE_DIR = "fixtures/ux/m5-staged-review-sheets"

RECORD_KIND = "m5_staged_review_sheet_set_packet"
SCHEMA_VERSION = 1
TAXONOMY_VERSION = 1

OVERLAY_ORIGINS = {"imported_review"}
PROVIDER_OR_REMOTE_ORIGINS = {"remote_commit", "provider_commit"}
PROVIDER_OR_REMOTE_FLOWS = {
    "provider_publish_later",
    "request_replay_mutation",
    "import_export_publish",
}

REDACTION_TOKENS = {
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
}

MUTATION_FLOWS = {
    "provider_publish_later",
    "admin_source_management",
    "request_replay_mutation",
    "package_lifecycle",
    "import_export_publish",
    "settings_bulk_apply",
}
FLOW_LANES = {"provider", "admin", "request", "package", "import", "settings"}
SCOPE_KINDS = {"single_object", "multi_object_explicit", "query_backed", "workspace_wide"}
MULTI_OBJECT_SCOPES = {"multi_object_explicit", "query_backed", "workspace_wide"}
CAN_HIDE_SCOPES = {"query_backed", "workspace_wide"}
MEMBER_CLASSES = {
    "included",
    "excluded_by_default",
    "excluded_by_user",
    "blocked_prerequisite",
    "hidden_collapsed",
}
SIDE_EFFECT_CLASSES = {
    "reversible_local",
    "reversible_with_export",
    "irreversible_confirmed",
    "external_publish",
    "policy_governed",
}
EXPORT_BEARING_EFFECTS = {"irreversible_confirmed", "external_publish"}
RECOVERABILITY_REQUIRES_EXPORT = {"reversible_via_export", "irreversible"}
RECOVERABILITY_DESTRUCTIVE = {"partially_reversible", "irreversible"}
CONSUMER_SURFACES = {
    "review_sheet",
    "batch_selection_bar",
    "diagnostics_panel",
    "support_export",
    "ai_evidence",
    "help_inline",
    "cli_confirmation",
}

LABS_CLAIM = "sheet_labs_not_claimed"
CLAIM_RANK = {
    "sheet_unsafe": 0,
    "sheet_review_overlay": 1,
    "sheet_narrowed": 2,
    "sheet_certified": 3,
}

# Deterministic reason ordering (mirrors the Rust order_index).
REASON_ORDER = {
    "target_scope_hidden": 0,
    "member_counts_inconsistent": 1,
    "hidden_members_uncounted": 2,
    "included_excluded_blocked_counts_hidden": 3,
    "omitted_defaults_hidden": 4,
    "side_effect_undisclosed": 5,
    "blocked_prereq_hidden": 6,
    "rollback_consequences_hidden": 7,
    "generic_continue_action": 8,
    "imported_review_reads_as_apply": 9,
    "reopen_path_lost": 10,
    "sheet_overclaims": 11,
    "sheet_backing_missing": 12,
    "member_classes_unlabeled": 13,
    "side_effect_summary_unlabeled": 14,
    "cancel_action_unlabeled": 15,
    "recoverability_class_unlabeled": 16,
    "freshness_unlabeled": 17,
    "superseded_scope_not_marked": 18,
    "scope_stale": 19,
    "verification_proof_stale": 20,
    "verification_proof_missing": 21,
}

# Floor reasons break the contract outright rather than merely aging out.
FLOOR_REASONS = {
    r for r, idx in REASON_ORDER.items() if idx <= REASON_ORDER["sheet_backing_missing"]
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


def reconciles(counts: dict) -> bool:
    return (
        counts["included"] + counts["excluded"] + counts["blocked"] + counts["hidden"]
        == counts["total_matched"]
    )


def requires_specific_commit(rec: dict) -> bool:
    s = rec["sheet"]
    return (
        s["scope"]["scope_kind"] in MULTI_OBJECT_SCOPES
        or s["recoverability"]["recoverability_class"] in RECOVERABILITY_DESTRUCTIVE
        or any(e["effect_class"] in EXPORT_BEARING_EFFECTS for e in s["side_effects"])
        or rec["origin"] in PROVIDER_OR_REMOTE_ORIGINS
        or rec["flow"] in PROVIDER_OR_REMOTE_FLOWS
    )


def needs_export_path(rec: dict) -> bool:
    s = rec["sheet"]
    return (
        s["recoverability"]["recoverability_class"] in RECOVERABILITY_REQUIRES_EXPORT
        or any(e["effect_class"] in EXPORT_BEARING_EFFECTS for e in s["side_effects"])
    )


def claimed_claim(rec: dict) -> str:
    if rec["claim_posture"] == "labs_unadvertised":
        return LABS_CLAIM
    if rec["origin"] in OVERLAY_ORIGINS:
        return "sheet_review_overlay"
    return "sheet_certified"


def intrinsic_reasons(rec: dict, stale_window: bool) -> list[str]:
    s = rec["sheet"]
    integ = rec["integrity"]
    scope_kind = s["scope"]["scope_kind"]
    overlay = rec["origin"] in OVERLAY_ORIGINS
    counts = s["counts"]
    members = s["members"]
    recov = s["recoverability"]
    reasons: list[str] = []

    # Target scope.
    if not s["scope"]["scope_declared"] or not integ["target_scope_visible"]:
        reasons.append("target_scope_hidden")

    # Member counts.
    if not reconciles(counts):
        reasons.append("member_counts_inconsistent")
    has_hidden_member = any(m["member_class"] == "hidden_collapsed" for m in members)
    if scope_kind in CAN_HIDE_SCOPES and has_hidden_member and counts["hidden"] == 0:
        reasons.append("hidden_members_uncounted")
    if scope_kind in MULTI_OBJECT_SCOPES and (
        not counts["counts_visible"] or not integ["counts_visible"]
    ):
        reasons.append("included_excluded_blocked_counts_hidden")

    # Omitted defaults.
    if not s["omitted_defaults_disclosed"] or not integ["omitted_defaults_visible"]:
        reasons.append("omitted_defaults_hidden")

    # Side effects.
    if (
        any(not e["disclosed_before_commit"] for e in s["side_effects"])
        or not s["side_effects_disclosed"]
        or not integ["side_effects_disclosed"]
    ):
        reasons.append("side_effect_undisclosed")

    # Blocked prerequisites.
    blocked_member_unlabeled = any(
        m["member_class"] == "blocked_prerequisite" and not m["reason_labeled"]
        for m in members
    )
    if blocked_member_unlabeled or not integ["blocked_prereqs_explained"]:
        reasons.append("blocked_prereq_hidden")

    # Rollback / export consequence.
    recovery_path_present = recov["rollback_path_present"] or recov["export_path_present"]
    if (
        not recovery_path_present
        or (needs_export_path(rec) and not recov["export_path_present"])
        or not integ["rollback_visible"]
    ):
        reasons.append("rollback_consequences_hidden")

    # Commit action.
    if requires_specific_commit(rec) and (
        not s["commit"]["commit_action_is_specific"] or not integ["commit_action_specific"]
    ):
        reasons.append("generic_continue_action")

    # Member class labelling (non-floor).
    non_blocked_member_unlabeled = any(
        m["member_class"] != "blocked_prerequisite" and not m["reason_labeled"]
        for m in members
    )
    if (
        not s["members_classes_labeled"]
        or not integ["member_classes_labeled"]
        or non_blocked_member_unlabeled
    ):
        reasons.append("member_classes_unlabeled")

    # Side-effect summary (non-floor).
    if not s["side_effect_summary_labeled"]:
        reasons.append("side_effect_summary_unlabeled")

    # Cancel action (non-floor).
    if not s["commit"]["cancel_action_is_specific"]:
        reasons.append("cancel_action_unlabeled")

    # Recoverability label (non-floor).
    if not recov["recoverability_class_labeled"] or not integ["recoverability_labeled"]:
        reasons.append("recoverability_class_unlabeled")

    # Scope freshness.
    if not integ["freshness_state_visible"]:
        reasons.append("freshness_unlabeled")
    fs = rec["declared_freshness_state"]
    if fs == "missing":
        reasons.append("sheet_backing_missing")
    elif fs == "superseded_by_newer_source" and not integ["superseded_state_marked"]:
        reasons.append("superseded_scope_not_marked")
    elif fs == "stale_expired" and not overlay:
        reasons.append("scope_stale")

    # Verification proof.
    pc = rec["verification"]["proof_currency"]
    if pc == "missing_proof":
        reasons.append("verification_proof_missing")
    elif pc in ("stale_expired", "requires_review"):
        reasons.append("verification_proof_stale")
    elif pc in ("verified_current", "cached_within_window") and stale_window:
        reasons.append("verification_proof_stale")

    # Overlay read-only.
    if overlay and not integ["imported_review_read_only"]:
        reasons.append("imported_review_reads_as_apply")

    # Reopen-to-scope.
    if not integ["reopen_visible_on_demand"] or any(
        not r["scope_visible"] for r in rec["renderings"]
    ):
        reasons.append("reopen_path_lost")
    if rec["declared_reopen_target"] == "none_keyboard_fallback":
        reasons.append("reopen_path_lost")

    return reasons


def derive_effective(claimed: str, reasons: list[str]) -> str:
    if any(r in FLOOR_REASONS for r in reasons):
        return "sheet_unsafe"
    if not reasons:
        return claimed
    if claimed == "sheet_review_overlay":
        return "sheet_unsafe"
    return "sheet_narrowed"


def record_reasons(rec: dict, stale_window: bool) -> list[str]:
    claimed = claimed_claim(rec)
    reasons = intrinsic_reasons(rec, stale_window)
    intrinsic = derive_effective(claimed, reasons)
    if any(overclaims(intrinsic, r["rendered_claim"]) for r in rec["renderings"]):
        reasons.append("sheet_overclaims")
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
    if effective != "sheet_unsafe":
        return True
    return rec["declared_reopen_target"] in ("scope_only", "none_keyboard_fallback") or present(
        rec["lineage"]["reopen_backlink_ref"]
    )


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
    sheets = packet.get("sheets", [])
    if not sheets:
        v.append("empty_sheets")

    seen: set[str] = set()
    flows: set[str] = set()
    lanes: set[str] = set()
    scope_kinds: set[str] = set()
    member_classes: set[str] = set()
    side_effect_classes: set[str] = set()
    consumers: set[str] = set()
    demonstrates_narrowing = False
    stale_window = False  # canonical packet is evaluated at its own as_of

    for s in sheets:
        sid = s.get("sheet_id", "")
        if sid in seen:
            v.append("duplicate_sheet_id")
        seen.add(sid)
        flows.add(s.get("flow"))
        lanes.add(s.get("lane"))
        scope_kinds.add(s.get("sheet", {}).get("scope", {}).get("scope_kind"))
        for m in s.get("sheet", {}).get("members", []):
            member_classes.add(m.get("member_class"))
        for e in s.get("sheet", {}).get("side_effects", []):
            side_effect_classes.add(e.get("effect_class"))
        for r in s.get("renderings", []):
            consumers.add(r.get("surface"))

        if (
            not present(s.get("sheet_id"))
            or not present(s.get("label_summary"))
            or not present(s.get("lineage", {}).get("session_ref"))
        ):
            v.append("sheet_missing_identity")
        if s.get("origin") in OVERLAY_ORIGINS and not (
            present(s.get("lineage", {}).get("provider_ref"))
            or present(s.get("lineage", {}).get("source_artifact_ref"))
        ):
            v.append("overlay_missing_provenance_ref")
        if not s.get("sheet", {}).get("members"):
            v.append("sheet_missing_members")
        if not s.get("renderings"):
            v.append("sheet_missing_rendering")
        for r in s.get("renderings", []):
            if not present(r.get("source_sheet_ref")):
                v.append("rendering_missing_source_ref")

        decision = narrow(s, stale_window)
        if decision["narrowed"]:
            demonstrates_narrowing = True
            if not decision["reasons"]:
                v.append("narrowed_sheet_missing_label_or_trigger")
        if not floored_keeps_fallback(s, decision["effective"]):
            v.append("floored_sheet_loses_fallback")
        if record_overclaims(s, decision["effective"]):
            v.append("rendering_sheet_overclaims")

    if not MUTATION_FLOWS.issubset(flows):
        v.append("mutation_flow_missing")
    if not FLOW_LANES.issubset(lanes):
        v.append("flow_lane_missing")
    if not SCOPE_KINDS.issubset(scope_kinds):
        v.append("scope_kind_missing")
    if not MEMBER_CLASSES.issubset(member_classes):
        v.append("member_class_missing")
    if not SIDE_EFFECT_CLASSES.issubset(side_effect_classes):
        v.append("side_effect_class_missing")
    if not CONSUMER_SURFACES.issubset(consumers):
        v.append("consumer_surface_missing")
    if not demonstrates_narrowing:
        v.append("downgraded_sheet_case_missing")
    if contains_forbidden(packet):
        v.append("raw_boundary_material_in_export")

    out: list[str] = []
    for item in v:
        if item not in out:
            out.append(item)
    return out


def claim_distribution(sheets: list[dict]) -> dict:
    dist = {"certified": 0, "narrowed": 0, "overlay": 0, "unsafe": 0, "labs": 0}
    bucket = {
        "sheet_certified": "certified",
        "sheet_narrowed": "narrowed",
        "sheet_review_overlay": "overlay",
        "sheet_unsafe": "unsafe",
        LABS_CLAIM: "labs",
    }
    for s in sheets:
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


def base_record(sheets: list[dict], sid: str) -> dict:
    for s in sheets:
        if s["sheet_id"] == sid:
            return s
    raise SystemExit(f"base sheet not found: {sid}")


S_PROVIDER = "sheet:provider-publish-later:0001"
S_SETTINGS = "sheet:settings-bulk-apply:0001"
S_PACKAGE = "sheet:package-lifecycle:0001"
S_ADMIN = "sheet:admin-source-management:0001"
S_REQUEST = "sheet:request-replay:0001"
S_IMPORT = "sheet:import-export-publish:0001"
S_LABS = "sheet:experimental-quick-apply:0001"

UNSAFE = "sheet_unsafe"
NARROW = "sheet_narrowed"
CERT = "sheet_certified"
OVERLAY = "sheet_review_overlay"

# (case_id, base_id, overrides, stale_window, description, effective, narrowed, reasons)
CASES = [
    ("clean-certified", S_SETTINGS, {}, False,
     "An explicit multi-object settings apply, fully reversible from local history, certifies.",
     CERT, False, []),
    ("provider-publish-certified", S_PROVIDER, {}, False,
     "A single provider object published with an external-publish effect, reversible via export, certifies.",
     CERT, False, []),
    ("package-lifecycle-certified", S_PACKAGE, {}, False,
     "An install/update/remove set with one blocked prerequisite and a labelled irreversible removal certifies.",
     CERT, False, []),
    ("admin-workspace-wide-certified", S_ADMIN, {}, False,
     "A workspace-wide policy rotation whose 230 collapsed sources are covered by a hidden count certifies.",
     CERT, False, []),
    ("request-narrowed-baseline", S_REQUEST, {}, False,
     "A query-backed remote replay whose verification proof requires review narrows.",
     NARROW, True, ["verification_proof_stale"]),
    ("import-overlay-baseline", S_IMPORT, {}, False,
     "An imported migration-bundle review stays a read-only review overlay, never a local apply.",
     OVERLAY, False, []),
    ("labs-not-claimed", S_LABS, {}, False,
     "A Labs quick-apply sheet makes no public claim and is never widened or narrowed.",
     LABS_CLAIM, False, []),

    # Floors.
    ("target-scope-hidden", S_SETTINGS,
     {"sheet.scope.scope_declared": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A sheet that does not declare its target scope floors.",
     UNSAFE, True, ["target_scope_hidden"]),
    ("member-counts-inconsistent", S_SETTINGS,
     {"sheet.counts.total_matched": 99, "renderings[*].rendered_claim": UNSAFE}, False,
     "Member counts that do not reconcile with the declared total floor.",
     UNSAFE, True, ["member_counts_inconsistent"]),
    ("hidden-members-uncounted", S_ADMIN,
     {"sheet.counts.hidden": 0, "sheet.counts.total_matched": 12,
      "renderings[*].rendered_claim": UNSAFE}, False,
     "A workspace-wide action that collapses members but reports zero hidden floors.",
     UNSAFE, True, ["hidden_members_uncounted"]),
    ("included-excluded-blocked-counts-hidden", S_SETTINGS,
     {"sheet.counts.counts_visible": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A multi-object action that hides the included/excluded/blocked counts floors.",
     UNSAFE, True, ["included_excluded_blocked_counts_hidden"]),
    ("single-object-counts-hidden-ok", S_PROVIDER,
     {"sheet.counts.counts_visible": False}, False,
     "A single-object sheet needs no counts breakdown, so hiding it stays certified.",
     CERT, False, []),
    ("omitted-defaults-hidden", S_SETTINGS,
     {"sheet.omitted_defaults_disclosed": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A sheet that hides omitted defaults floors.",
     UNSAFE, True, ["omitted_defaults_hidden"]),
    ("undisclosed-side-effect", S_PACKAGE,
     {"sheet.side_effects[0].disclosed_before_commit": False,
      "renderings[*].rendered_claim": UNSAFE}, False,
     "A side effect not disclosed before commit floors.",
     UNSAFE, True, ["side_effect_undisclosed"]),
    ("blocked-prereq-hidden", S_PACKAGE,
     {"sheet.members[3].reason_labeled": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A blocked prerequisite whose reason is not labelled floors.",
     UNSAFE, True, ["blocked_prereq_hidden"]),
    ("hidden-rollback", S_SETTINGS,
     {"sheet.recoverability.rollback_path_present": False,
      "renderings[*].rendered_claim": UNSAFE}, False,
     "A reversible action with no rollback or export path hides the recovery consequence and floors.",
     UNSAFE, True, ["rollback_consequences_hidden"]),
    ("irreversible-without-export", S_PACKAGE,
     {"sheet.recoverability.export_path_present": False,
      "renderings[*].rendered_claim": UNSAFE}, False,
     "An irreversible removal with no export/backup path floors.",
     UNSAFE, True, ["rollback_consequences_hidden"]),
    ("generic-continue", S_PACKAGE,
     {"sheet.commit.commit_action_is_specific": False,
      "renderings[*].rendered_claim": UNSAFE}, False,
     "A consequential commit behind a generic Continue floors.",
     UNSAFE, True, ["generic_continue_action"]),
    ("imported-review-reads-as-apply", S_IMPORT,
     {"integrity.imported_review_read_only": False,
      "renderings[*].rendered_claim": UNSAFE}, False,
     "An imported/restore review that reads as a local apply floors below the review overlay.",
     UNSAFE, True, ["imported_review_reads_as_apply"]),
    ("reopen-path-lost-keeps-fallback", S_SETTINGS,
     {"declared_reopen_target": "none_keyboard_fallback",
      "renderings[*].rendered_claim": UNSAFE}, False,
     "Losing the reopen-to-scope path floors but keeps a keyboard fallback.",
     UNSAFE, True, ["reopen_path_lost"]),
    ("missing-backing", S_SETTINGS,
     {"declared_freshness_state": "missing", "renderings[*].rendered_claim": UNSAFE}, False,
     "A missing scope snapshot floors the sheet.",
     UNSAFE, True, ["sheet_backing_missing"]),
    ("overlay-any-gap-floors", S_IMPORT,
     {"sheet.recoverability.recoverability_class_labeled": False,
      "renderings[*].rendered_claim": UNSAFE}, False,
     "A review overlay with any non-floor gap drops below the overlay rather than holding it.",
     UNSAFE, True, ["recoverability_class_unlabeled"]),
    ("rendering-overclaims", S_REQUEST,
     {"renderings[0].rendered_claim": CERT}, False,
     "A narrowed sheet whose rendering still shows certified floors as an overclaim.",
     UNSAFE, True, ["sheet_overclaims", "verification_proof_stale"]),

    # Narrows.
    ("member-classes-unlabeled", S_SETTINGS,
     {"sheet.members_classes_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the member-class labels narrows the sheet.",
     NARROW, True, ["member_classes_unlabeled"]),
    ("side-effect-summary-unlabeled", S_SETTINGS,
     {"sheet.side_effect_summary_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the aggregate side-effect summary narrows the sheet.",
     NARROW, True, ["side_effect_summary_unlabeled"]),
    ("cancel-action-unlabeled", S_SETTINGS,
     {"sheet.commit.cancel_action_is_specific": False, "renderings[*].rendered_claim": NARROW}, False,
     "A non-specific cancel action narrows the sheet.",
     NARROW, True, ["cancel_action_unlabeled"]),
    ("recoverability-unlabeled", S_SETTINGS,
     {"sheet.recoverability.recoverability_class_labeled": False,
      "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the aggregate reversibility posture narrows the sheet.",
     NARROW, True, ["recoverability_class_unlabeled"]),
    ("freshness-unlabeled", S_SETTINGS,
     {"integrity.freshness_state_visible": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the scope freshness state narrows the sheet.",
     NARROW, True, ["freshness_unlabeled"]),
    ("superseded-unmarked", S_SETTINGS,
     {"declared_freshness_state": "superseded_by_newer_source",
      "integrity.superseded_state_marked": False, "renderings[*].rendered_claim": NARROW}, False,
     "An unmarked superseded scope snapshot narrows the sheet.",
     NARROW, True, ["superseded_scope_not_marked"]),
    ("superseded-marked-ok", S_SETTINGS,
     {"declared_freshness_state": "superseded_by_newer_source"}, False,
     "A marked superseded scope snapshot stays certified because the state is visible.",
     CERT, False, []),
    ("first-party-stale", S_SETTINGS,
     {"declared_freshness_state": "stale_expired", "renderings[*].rendered_claim": NARROW}, False,
     "A first-party stale scope snapshot narrows rather than reading as fresh.",
     NARROW, True, ["scope_stale"]),
    ("proof-missing", S_SETTINGS,
     {"verification.proof_currency": "missing_proof", "verification.proof_ref": None,
      "renderings[*].rendered_claim": NARROW}, False,
     "A missing verification proof narrows the sheet.",
     NARROW, True, ["verification_proof_missing"]),
    ("proof-requires-review", S_SETTINGS,
     {"verification.proof_currency": "requires_review", "renderings[*].rendered_claim": NARROW}, False,
     "A verification proof that requires review narrows the sheet.",
     NARROW, True, ["verification_proof_stale"]),
    ("stale-window", S_SETTINGS,
     {"renderings[*].rendered_claim": NARROW}, True,
     "An elapsed verification window ages a current proof out to narrowed.",
     NARROW, True, ["verification_proof_stale"]),
]


def run_corpus_from_cases(sheets: list[dict]) -> list[str]:
    failures: list[str] = []
    for case in CASES:
        case_id, base_id, overrides, stale_window, _desc, exp_eff, exp_narrowed, exp_reasons = case
        r = apply_overrides(base_record(sheets, base_id), overrides)
        decision = narrow(r, stale_window)
        if decision["effective"] != exp_eff:
            failures.append(f"{case_id}: effective {decision['effective']} != {exp_eff}")
        if decision["narrowed"] != exp_narrowed:
            failures.append(f"{case_id}: narrowed {decision['narrowed']} != {exp_narrowed}")
        if decision["reasons"] != exp_reasons:
            failures.append(f"{case_id}: reasons {decision['reasons']} != {exp_reasons}")
    return failures


def run_corpus_from_disk(repo_root: Path, sheets: list[dict]) -> list[str]:
    out_dir = repo_root / FIXTURE_DIR
    index_path = out_dir / "index.json"
    if not index_path.exists():
        return [f"missing corpus index: {index_path}"]
    index = json.loads(index_path.read_text(encoding="utf-8"))
    failures: list[str] = []
    for filename in index["cases"]:
        payload = json.loads((out_dir / filename).read_text(encoding="utf-8"))
        case_id = payload["case_id"]
        r = apply_overrides(base_record(sheets, payload["base_sheet_id"]), payload["overrides"])
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
            "base_sheet_id": base_id,
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
        "corpus_id": "m5-staged-review-sheets-corpus:0001",
        "description": (
            "Perturbation corpus for the staged-review (commit) sheet narrowing engine. "
            "Each case starts from a canonical sheet, applies dotted-path overrides, and asserts "
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
        sys.stderr.write("staged-review set FAILED validation:\n")
        for v in violations:
            sys.stderr.write(f"  - {v}\n")
        return 1
    dist = claim_distribution(packet["sheets"])
    sys.stdout.write(
        f"staged-review set OK: {len(packet['sheets'])} sheets, distribution {dist}\n"
    )
    return 0


def cmd_corpus(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    sheets = packet["sheets"]
    failures = run_corpus_from_cases(sheets)
    failures += run_corpus_from_disk(repo_root, sheets)
    if failures:
        sys.stderr.write("staged-review corpus FAILED:\n")
        for f in failures:
            sys.stderr.write(f"  - {f}\n")
        return 1
    sys.stdout.write(f"staged-review corpus OK: {len(CASES)} cases\n")
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
