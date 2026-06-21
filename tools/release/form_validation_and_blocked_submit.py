#!/usr/bin/env python3
"""Freeze and certify the M5 form-validation / blocked-submit set: how a
mutation-capable form rolls field-level validity up into a form-level validation
summary without replacing the field anchors, explains cross-field dependencies
(provider/account mapping, environment selection, package source/registry auth,
import/export mode, derived field constraints) when one choice narrows or
invalidates another, and emits machine-readable blocked-submit reasons that
desktop, CLI/headless, support-export, and docs/help surfaces can all reuse to
explain the same failure state.

The canonical truth is the checked-in support export
(``artifacts/ux/m5-form-validation-and-blocked-submit/support_export.json``). Each
form binds its field anchors, form-level summary, cross-field dependencies,
blocked-submit reasons, submit gate, backing freshness, and verification proof to
one inspectable record. This tool ingests that set and, per form, **independently**
re-derives an effective claim that never reads wider than the evidence supports:

* the submit gate is closed whenever a blocked prerequisite or cross-field
  invalidation is active;
* the form-level summary is consistent with the field-level anchors and never
  replaces them;
* a cross-field invalidation is explained before submit, a derived constraint is
  disclosed, and a blocking validation is anchored to the field with exact rule
  text rather than deferred to a banner;
* a blocked-submit reason carries a stable machine code and stays reusable by the
  machine consumers (CLI/headless and support export);
* a form whose gate is open while blocked, whose summary contradicts or replaces
  the field anchors, whose invalidation is hidden, whose reason is not
  machine-readable or reusable, that lets an imported/restore review read as a
  local submit, or renders wider than its effective claim floors to form_blocked
  and falls back to a submit control that names the reason.

The Rust truth source is ``crates/aureline-ui/src/m5_form_validation_and_blocked_submit``;
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
SUPPORT_EXPORT_REF = "artifacts/ux/m5-form-validation-and-blocked-submit/support_export.json"
REPORT_REF = "artifacts/ux/m5-form-validation-and-blocked-submit/report.md"
SCHEMA_REF = "schemas/ux/m5-form-validation-and-blocked-submit.schema.json"
FIXTURE_DIR = "fixtures/ux/m5-form-validation-and-blocked-submit"

RECORD_KIND = "m5_form_validation_set_packet"
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
DEPENDENCY_KINDS = {
    "provider_account_mapping",
    "environment_selection",
    "package_source_registry_auth",
    "import_export_mode",
    "derived_field_constraint",
}
DEPENDENCY_RELATIONS = {"narrows", "invalidates", "requires", "mutually_exclusive"}
BLOCKER_CLASSES = {
    "invalid_field",
    "missing_prerequisite",
    "cross_field_conflict",
    "unresolved_policy_lock",
    "pending_validation",
    "unreviewed_side_effect",
}
BLOCKED_CONSUMERS = {"desktop", "cli_headless", "support_export", "docs_help"}
MACHINE_CONSUMERS = {"cli_headless", "support_export"}
CONSUMER_SURFACES = {
    "form_view",
    "wizard_step",
    "review_sheet",
    "diagnostics_panel",
    "support_export",
    "ai_evidence",
    "help_inline",
}

LABS_CLAIM = "form_labs_not_claimed"
CLAIM_RANK = {
    "form_blocked": 0,
    "form_review_overlay": 1,
    "form_narrowed": 2,
    "form_certified": 3,
}

# Deterministic reason ordering (mirrors the Rust order_index).
REASON_ORDER = {
    "submit_allowed_while_blocked_hidden": 0,
    "blocked_reason_unexplained": 1,
    "cross_field_invalidation_hidden": 2,
    "field_form_validation_contradicts": 3,
    "form_summary_replaces_field_anchors": 4,
    "derived_constraint_hidden": 5,
    "blocked_reason_not_machine_readable": 6,
    "blocked_reason_not_reusable": 7,
    "validation_anchor_missing": 8,
    "imported_submit_reads_as_applied": 9,
    "rendering_overclaims": 10,
    "validation_backing_missing": 11,
    "cross_field_dependency_deferred": 12,
    "resolution_hint_missing": 13,
    "validation_state_unlabeled": 14,
    "async_validation_pending": 15,
    "freshness_unlabeled": 16,
    "superseded_state_not_marked": 17,
    "form_stale": 18,
    "verification_proof_stale": 19,
    "verification_proof_missing": 20,
    "reopen_path_lost": 21,
}

# Floor reasons break the contract outright rather than merely aging out.
FLOOR_REASONS = {
    r for r, idx in REASON_ORDER.items() if idx <= REASON_ORDER["validation_backing_missing"]
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


def claimed_claim(rec: dict) -> str:
    if rec["claim_posture"] == "labs_unadvertised":
        return LABS_CLAIM
    if rec["origin"] in OVERLAY_ORIGINS:
        return "form_review_overlay"
    return "form_certified"


def any_active_blocker(rec: dict) -> bool:
    return (
        any(b["blocks_submit"] for b in rec["blocked_submit_reasons"])
        or any(d["blocks_submit"] for d in rec["dependencies"])
        or any(f["validation_state"] == "invalid_blocking" for f in rec["field_anchors"])
    )


def intrinsic_reasons(rec: dict, stale_window: bool) -> list[str]:
    overlay = rec["origin"] in OVERLAY_ORIGINS
    summary = rec["form_summary"]
    gate = rec["submit_gate"]
    integ = rec["integrity"]
    reasons: list[str] = []

    # Field-level validation anchors stay anchored and labelled.
    for f in rec["field_anchors"]:
        needs = f["validation_state"] in ("invalid_blocking", "warning")
        if needs and (not f["anchored_to_field"] or not f["exact_rule_text_present"]):
            reasons.append("validation_anchor_missing")
        if not f["state_labeled"]:
            reasons.append("validation_state_unlabeled")
        if f["validation_state"] == "pending_async":
            reasons.append("async_validation_pending")

    # Form-level summary is linked, not duplicated/contradictory, and never
    # replaces the field anchors.
    blocking_field = any(f["validation_state"] == "invalid_blocking" for f in rec["field_anchors"])
    any_blocking_reason = any(b["blocks_submit"] for b in rec["blocked_submit_reasons"])
    if not summary["consistent_with_fields"] or (blocking_field and not any_blocking_reason):
        reasons.append("field_form_validation_contradicts")
    if summary["replaces_field_anchors"] or not integ["field_anchors_preserved"]:
        reasons.append("form_summary_replaces_field_anchors")
    if not summary["summarizes_field_anchors"] or not integ["form_summary_linked"]:
        reasons.append("field_form_validation_contradicts")
    if summary["derived_constraint_count"] > 0 and (
        not summary["derived_constraints_disclosed"] or not integ["derived_constraints_visible"]
    ):
        reasons.append("derived_constraint_hidden")

    # Cross-field dependencies are explained before submit.
    for d in rec["dependencies"]:
        if not d["explained_before_submit"]:
            if d["blocks_submit"]:
                reasons.append("cross_field_invalidation_hidden")
            else:
                reasons.append("cross_field_dependency_deferred")
    if not integ["cross_field_deps_explained"]:
        reasons.append("cross_field_invalidation_hidden")

    # Blocked-submit reasons: explained, machine-readable, reusable, with a hint.
    for b in rec["blocked_submit_reasons"]:
        if b["blocks_submit"] and not b["explained_before_submit"]:
            reasons.append("blocked_reason_unexplained")
        if not present(b["machine_code"]):
            reasons.append("blocked_reason_not_machine_readable")
        if b["blocks_submit"] and any(m not in b["reusable_by"] for m in MACHINE_CONSUMERS):
            reasons.append("blocked_reason_not_reusable")
        if b["blocks_submit"] and not b["resolution_hint_present"]:
            reasons.append("resolution_hint_missing")
    if not integ["blocked_reasons_machine_readable"]:
        reasons.append("blocked_reason_not_machine_readable")
    if not integ["blocked_reasons_reusable"]:
        reasons.append("blocked_reason_not_reusable")

    # The submit gate cannot be open while a blocker is active.
    if gate["submit_allowed"] and (
        any_active_blocker(rec) or not gate["blockers_explained_before_submit"]
    ):
        reasons.append("submit_allowed_while_blocked_hidden")

    # Imported/restore overlay must stay a read-only review, never a submit.
    if overlay and (
        not integ["imported_review_read_only"]
        or gate["submit_allowed"]
        or any(not r["read_only"] for r in rec["renderings"])
    ):
        reasons.append("imported_submit_reads_as_applied")

    # Backing freshness.
    if not integ["freshness_state_visible"]:
        reasons.append("freshness_unlabeled")
    fs = rec["declared_freshness_state"]
    if fs == "missing":
        reasons.append("validation_backing_missing")
    elif fs == "superseded_by_newer_source" and not integ["superseded_state_marked"]:
        reasons.append("superseded_state_not_marked")
    elif fs == "stale_expired" and not overlay:
        reasons.append("form_stale")
    if not integ["validation_state_visible"]:
        reasons.append("validation_state_unlabeled")

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
        return "form_blocked"
    if not reasons:
        return claimed
    if claimed == "form_review_overlay":
        return "form_blocked"
    return "form_narrowed"


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
    if effective != "form_blocked":
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
    dep_kinds: set[str] = set()
    dep_relations: set[str] = set()
    blocker_classes: set[str] = set()
    blocked_consumers: set[str] = set()
    consumers: set[str] = set()
    demonstrates_narrowing = False
    stale_window = False  # canonical packet is evaluated at its own as_of

    for s in surfaces:
        sid = s.get("surface_id", "")
        if sid in seen:
            v.append("duplicate_surface_id")
        seen.add(sid)
        lanes.add(s.get("lane"))
        for d in s.get("dependencies", []):
            dep_kinds.add(d.get("dependency_kind"))
            dep_relations.add(d.get("relation"))
        for b in s.get("blocked_submit_reasons", []):
            blocker_classes.add(b.get("blocker_class"))
            for c in b.get("reusable_by", []):
                blocked_consumers.add(c)
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
        if not s.get("field_anchors"):
            v.append("surface_missing_fields")
        if not s.get("renderings"):
            v.append("surface_missing_rendering")
        for r in s.get("renderings", []):
            if not present(r.get("source_surface_ref")):
                v.append("rendering_missing_source_ref")
        for b in s.get("blocked_submit_reasons", []):
            if not present(b.get("reason_id")) or not present(b.get("label_summary")):
                v.append("blocked_reason_missing_identity")

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
    if not DEPENDENCY_KINDS.issubset(dep_kinds):
        v.append("dependency_kind_missing")
    if not DEPENDENCY_RELATIONS.issubset(dep_relations):
        v.append("dependency_relation_missing")
    if not BLOCKER_CLASSES.issubset(blocker_classes):
        v.append("blocker_class_missing")
    if not BLOCKED_CONSUMERS.issubset(blocked_consumers):
        v.append("blocked_consumer_missing")
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
        "form_certified": "certified",
        "form_narrowed": "narrowed",
        "form_review_overlay": "overlay",
        "form_blocked": "blocked",
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
    raise SystemExit(f"base form not found: {sid}")


F_PROVIDER = "form:provider-connection:0001"
F_SETTINGS = "form:settings-config:0001"
F_PROJECTS = "wizard:project-bootstrap:0001"
F_PACKAGE = "sheet:package-install:0001"
F_ADMIN = "sheet:admin-policy-rollout:0001"
F_REQUEST = "dialog:request-run:0001"
F_IMPORT = "dialog:migration-restore:0001"
F_LABS = "wizard:labs-onboarding:0001"

BLOCKED = "form_blocked"
NARROW = "form_narrowed"
CERT = "form_certified"
OVERLAY = "form_review_overlay"

# (case_id, base_id, overrides, stale_window, description, effective, narrowed, reasons)
CASES = [
    ("clean-certified", F_SETTINGS, {}, False,
     "A clean settings form whose field validity rolls up into the form summary with no active blockers certifies.",
     CERT, False, []),
    ("provider-blocker-certified", F_PROVIDER, {}, False,
     "A provider form with an explained, reusable, machine-readable missing-account blocker stays certified because its blocked-submit truth is honest and the gate is closed.",
     CERT, False, []),
    ("request-narrowed-baseline", F_REQUEST, {}, False,
     "The request-run dialog narrows while an async endpoint health check is in flight.",
     NARROW, True, ["async_validation_pending"]),
    ("import-overlay-baseline", F_IMPORT, {}, False,
     "The migration restore review stays a read-only review overlay.",
     OVERLAY, False, []),
    ("labs-not-claimed", F_LABS, {}, False,
     "A Labs onboarding form makes no public claim and is never widened or narrowed.",
     LABS_CLAIM, False, []),

    # Floors.
    ("submit-open-while-blocked", F_PROVIDER,
     {"submit_gate.submit_allowed": True, "renderings[*].rendered_claim": BLOCKED}, False,
     "Opening the submit gate while a prerequisite blocks floors the form.",
     BLOCKED, True, ["submit_allowed_while_blocked_hidden"]),
    ("blocked-reason-unexplained", F_PROVIDER,
     {"blocked_submit_reasons[0].explained_before_submit": False,
      "renderings[*].rendered_claim": BLOCKED}, False,
     "A blocking blocked-submit reason that is not explained before submit floors.",
     BLOCKED, True, ["blocked_reason_unexplained"]),
    ("cross-field-invalidation-hidden", F_PROVIDER,
     {"dependencies[0].blocks_submit": True, "dependencies[0].explained_before_submit": False,
      "renderings[*].rendered_claim": BLOCKED}, False,
     "A cross-field invalidation that blocks submit but is not explained floors.",
     BLOCKED, True, ["cross_field_invalidation_hidden"]),
    ("summary-contradicts-fields", F_PROVIDER,
     {"form_summary.consistent_with_fields": False, "renderings[*].rendered_claim": BLOCKED}, False,
     "A form-level summary that contradicts the field-level validation floors.",
     BLOCKED, True, ["field_form_validation_contradicts"]),
    ("blocking-field-without-reason", F_SETTINGS,
     {"field_anchors[0].validation_state": "invalid_blocking", "submit_gate.submit_allowed": False,
      "renderings[*].rendered_claim": BLOCKED}, False,
     "A field that is invalid-blocking with no backing blocked-submit reason is a field/form contradiction and floors.",
     BLOCKED, True, ["field_form_validation_contradicts"]),
    ("summary-replaces-anchors", F_PROVIDER,
     {"form_summary.replaces_field_anchors": True, "renderings[*].rendered_claim": BLOCKED}, False,
     "A form-level summary that replaces the field-level anchors floors.",
     BLOCKED, True, ["form_summary_replaces_field_anchors"]),
    ("derived-constraint-hidden", F_ADMIN,
     {"form_summary.derived_constraints_disclosed": False,
      "renderings[*].rendered_claim": BLOCKED}, False,
     "A derived constraint that affects submit but is not disclosed floors.",
     BLOCKED, True, ["derived_constraint_hidden"]),
    ("reason-not-machine-readable", F_PROVIDER,
     {"blocked_submit_reasons[0].machine_code": "  ", "renderings[*].rendered_claim": BLOCKED}, False,
     "A blocked-submit reason with no stable machine code floors.",
     BLOCKED, True, ["blocked_reason_not_machine_readable"]),
    ("reason-not-reusable", F_PROVIDER,
     {"blocked_submit_reasons[0].reusable_by": ["desktop"],
      "renderings[*].rendered_claim": BLOCKED}, False,
     "A blocking reason not reusable by the machine consumers (CLI/headless, support) floors.",
     BLOCKED, True, ["blocked_reason_not_reusable"]),
    ("validation-anchor-missing", F_PROVIDER,
     {"field_anchors[1].anchored_to_field": False, "field_anchors[1].exact_rule_text_present": False,
      "renderings[*].rendered_claim": BLOCKED}, False,
     "A blocking validation deferred to a banner instead of an exact field anchor floors.",
     BLOCKED, True, ["validation_anchor_missing"]),
    ("imported-submit-reads-as-applied", F_IMPORT,
     {"submit_gate.submit_allowed": True, "renderings[*].rendered_claim": BLOCKED}, False,
     "An imported/restore review that allows a local submit floors below the review overlay.",
     BLOCKED, True, ["imported_submit_reads_as_applied"]),
    ("rendering-overclaims", F_REQUEST,
     {"renderings[0].rendered_claim": CERT}, False,
     "A narrowed form whose rendering still shows certified floors as an overclaim.",
     BLOCKED, True, ["rendering_overclaims", "async_validation_pending"]),
    ("missing-backing", F_SETTINGS,
     {"declared_freshness_state": "missing", "renderings[*].rendered_claim": BLOCKED}, False,
     "Missing backing data floors the form.",
     BLOCKED, True, ["validation_backing_missing"]),

    # Narrows.
    ("dependency-deferred", F_PROVIDER,
     {"dependencies[0].explained_before_submit": False,
      "renderings[*].rendered_claim": NARROW}, False,
     "A non-blocking cross-field dependency left unexplained narrows the form.",
     NARROW, True, ["cross_field_dependency_deferred"]),
    ("resolution-hint-missing", F_PROVIDER,
     {"blocked_submit_reasons[0].resolution_hint_present": False,
      "renderings[*].rendered_claim": NARROW}, False,
     "A blocking blocked-submit reason with no resolution hint narrows the form.",
     NARROW, True, ["resolution_hint_missing"]),
    ("validation-state-unlabeled", F_SETTINGS,
     {"field_anchors[0].state_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding a field's validation state narrows the form.",
     NARROW, True, ["validation_state_unlabeled"]),
    ("freshness-unlabeled", F_SETTINGS,
     {"integrity.freshness_state_visible": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the backing freshness state narrows the form.",
     NARROW, True, ["freshness_unlabeled"]),
    ("first-party-stale", F_SETTINGS,
     {"declared_freshness_state": "stale_expired", "renderings[*].rendered_claim": NARROW}, False,
     "A first-party stale backing source narrows rather than reading as fresh.",
     NARROW, True, ["form_stale"]),
    ("superseded-unmarked", F_SETTINGS,
     {"declared_freshness_state": "superseded_by_newer_source",
      "integrity.superseded_state_marked": False, "renderings[*].rendered_claim": NARROW}, False,
     "An unmarked superseded backing source narrows the form.",
     NARROW, True, ["superseded_state_not_marked"]),
    ("superseded-marked-ok", F_SETTINGS,
     {"declared_freshness_state": "superseded_by_newer_source"}, False,
     "A marked superseded backing source stays certified because the state is visible.",
     CERT, False, []),
    ("proof-missing", F_SETTINGS,
     {"verification.proof_currency": "missing_proof", "verification.proof_ref": None,
      "renderings[*].rendered_claim": NARROW}, False,
     "A missing verification proof narrows the form.",
     NARROW, True, ["verification_proof_missing"]),
    ("proof-requires-review", F_SETTINGS,
     {"verification.proof_currency": "requires_review", "renderings[*].rendered_claim": NARROW}, False,
     "A verification proof that requires review narrows the form.",
     NARROW, True, ["verification_proof_stale"]),
    ("stale-window", F_SETTINGS,
     {"renderings[*].rendered_claim": NARROW}, True,
     "An elapsed verification window ages a current proof out to narrowed.",
     NARROW, True, ["verification_proof_stale"]),
    ("reopen-path-lost", F_SETTINGS,
     {"integrity.reopen_visible_on_demand": False, "renderings[*].rendered_claim": NARROW}, False,
     "Losing the reopen-to-origin path narrows the form.",
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
        "corpus_id": "m5-form-validation-and-blocked-submit-corpus:0001",
        "description": (
            "Perturbation corpus for the form-validation / blocked-submit narrowing engine. "
            "Each case starts from a canonical form, applies dotted-path overrides, and asserts "
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
        sys.stderr.write("form-validation set FAILED validation:\n")
        for v in violations:
            sys.stderr.write(f"  - {v}\n")
        return 1
    dist = claim_distribution(packet["surfaces"])
    sys.stdout.write(
        f"form-validation set OK: {len(packet['surfaces'])} forms, distribution {dist}\n"
    )
    return 0


def cmd_corpus(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    surfaces = packet["surfaces"]
    failures = run_corpus_from_cases(surfaces)
    failures += run_corpus_from_disk(repo_root, surfaces)
    if failures:
        sys.stderr.write("form-validation corpus FAILED:\n")
        for f in failures:
            sys.stderr.write(f"  - {f}\n")
        return 1
    sys.stdout.write(f"form-validation corpus OK: {len(CASES)} cases\n")
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
