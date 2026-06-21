#!/usr/bin/env python3
"""Freeze and certify the M5 structured-input / staged-review set: field
provenance, validation state, draft/applied state and recovery, submit blockers,
and staged-review (commit) sheets for mutation-capable forms, wizards, and review
sheets across the provider, admin, request, package, import, settings, and project
lanes.

The canonical truth is the checked-in support export
(``artifacts/ux/m5-structured-input-and-staged-review/support_export.json``). Each
surface binds its field provenance, validation, draft state, submit blockers, and
staged review to one inspectable record. This tool ingests that set and, per
surface, **independently** re-derives an effective claim that never reads wider
than the evidence supports:

* every field declares its source-of-value class (default/detected/imported/
  policy-locked/user-override/required-unset) and a user override stays distinct
  from the value it replaced;
* draft versus applied state is visibly distinct and a recoverable draft survives
  interruption/restart/reconnect;
* blocked prerequisites and cross-field conflicts are explained before submit, and
  the form never submits over an invalid-blocking field or a silently overridden
  policy lock;
* the staged review declares its target scope, omitted defaults, included/excluded/
  blocked members, side effects, and rollback/export path, and the commit action
  names that scope and effect rather than a generic Continue;
* a surface that hides a field source, blurs draft/applied, hides scope/defaults/
  prerequisites/rollback, discards a recoverable draft, lets an imported review
  read as a local apply, or renders wider than its effective claim floors to an
  explicit blocked state with a reopen/keyboard fallback rather than a
  clean-but-false submit.

The Rust truth source is
``crates/aureline-ui/src/m5_structured_input_and_staged_review``; this tool
re-derives the same effective claim and narrowing reasons so the checked-in
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
SUPPORT_EXPORT_REF = "artifacts/ux/m5-structured-input-and-staged-review/support_export.json"
REPORT_REF = "artifacts/ux/m5-structured-input-and-staged-review/report.md"
SCHEMA_REF = "schemas/ux/m5-structured-input-and-staged-review.schema.json"
FIXTURE_DIR = "fixtures/ux/m5-structured-input-and-staged-review"

RECORD_KIND = "m5_structured_input_set_packet"
SCHEMA_VERSION = 1
TAXONOMY_VERSION = 1

OVERLAY_ORIGINS = {"imported_or_restore"}

REDACTION_TOKENS = {
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
}

SURFACE_KINDS = {
    "structured_form",
    "multi_step_wizard",
    "publish_review_dialog",
    "import_restore_dialog",
    "install_review_sheet",
    "parameterized_workflow",
}
FORM_LANES = {"provider", "admin", "request", "package", "import", "settings", "projects"}
MUTATION_CLASSES = {"local", "remote", "provider_backed", "import_export", "policy_locked"}
SOURCE_CLASSES = {
    "default_value",
    "detected_value",
    "imported_value",
    "policy_locked",
    "user_override",
    "required_unset",
}
CONSUMER_SURFACES = {
    "form_view",
    "wizard_step",
    "review_sheet",
    "diagnostics_panel",
    "support_export",
    "ai_evidence",
    "help_inline",
}

LABS_CLAIM = "surface_labs_not_claimed"
CLAIM_RANK = {
    "surface_unsafe": 0,
    "surface_review_overlay": 1,
    "surface_narrowed": 2,
    "surface_certified": 3,
}

# Deterministic reason ordering (mirrors the Rust order_index).
REASON_ORDER = {
    "field_source_hidden": 0,
    "draft_applied_ambiguous": 1,
    "policy_lock_overridden_silently": 2,
    "submit_allowed_while_blocking_invalid": 3,
    "blocked_prereq_hidden": 4,
    "target_scope_hidden": 5,
    "omitted_defaults_hidden": 6,
    "side_effect_undisclosed": 7,
    "rollback_consequences_hidden": 8,
    "generic_continue_action": 9,
    "draft_recovery_lost": 10,
    "imported_state_reads_as_applied": 11,
    "reopen_path_lost": 12,
    "surface_overclaims": 13,
    "form_backing_missing": 14,
    "validation_state_unlabeled": 15,
    "cross_field_dependency_unexplained": 16,
    "excluded_members_unlabeled": 17,
    "autosave_unavailable": 18,
    "restore_prompt_missing": 19,
    "async_validation_pending": 20,
    "freshness_unlabeled": 21,
    "superseded_state_not_marked": 22,
    "surface_stale": 23,
    "verification_proof_stale": 24,
    "verification_proof_missing": 25,
}

# Floor reasons break the contract outright rather than merely aging out.
FLOOR_REASONS = {reason for reason, idx in REASON_ORDER.items() if idx <= REASON_ORDER["form_backing_missing"]}

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


def claimed_claim(surface: dict) -> str:
    if surface["claim_posture"] == "labs_unadvertised":
        return LABS_CLAIM
    if surface["origin"] in OVERLAY_ORIGINS:
        return "surface_review_overlay"
    return "surface_certified"


def intrinsic_reasons(surface: dict, stale_window: bool) -> list[str]:
    integ = surface["integrity"]
    session = surface["session"]
    review = surface["staged_review"]
    recovery = surface["draft_recovery"]
    ver = surface["verification"]
    overlay = surface["origin"] in OVERLAY_ORIGINS
    reasons: list[str] = []

    # Field provenance / source-of-value.
    for f in surface["fields"]:
        if not f["source_class_labeled"]:
            reasons.append("field_source_hidden")
        elif f["source_class"] == "user_override" and not f["override_distinct_from_default"]:
            reasons.append("field_source_hidden")
        if f["source_class"] == "policy_locked" and not f["policy_lock_respected"]:
            reasons.append("policy_lock_overridden_silently")
        if not f["validation_state_labeled"]:
            reasons.append("validation_state_unlabeled")

    has_blocking_invalid = any(
        f["validation_state"] == "invalid_blocking" for f in surface["fields"]
    )
    has_blocking_submit_blocker = any(b["blocks_submit"] for b in surface["submit_blockers"])
    if has_blocking_invalid and not has_blocking_submit_blocker:
        reasons.append("submit_allowed_while_blocking_invalid")

    # Headline integrity invariants.
    if not integ["preserves_field_provenance"]:
        reasons.append("field_source_hidden")
    if not integ["draft_applied_distinct"] or not session["draft_applied_distinct"]:
        reasons.append("draft_applied_ambiguous")
    if not integ["policy_locks_respected"]:
        reasons.append("policy_lock_overridden_silently")

    # Submit blockers.
    for b in surface["submit_blockers"]:
        if b["blocks_submit"] and not b["explained_before_submit"]:
            if b["blocker_class"] == "cross_field_conflict":
                reasons.append("cross_field_dependency_unexplained")
            elif b["blocker_class"] == "pending_validation":
                reasons.append("async_validation_pending")
            else:
                reasons.append("blocked_prereq_hidden")
    if not integ["blocked_prereqs_explained"]:
        reasons.append("blocked_prereq_hidden")

    # Staged review.
    if not review["target_scope_declared"] or not integ["target_scope_visible"]:
        reasons.append("target_scope_hidden")
    if not review["omitted_defaults_disclosed"] or not integ["omitted_defaults_visible"]:
        reasons.append("omitted_defaults_hidden")
    if not review["members_classes_labeled"]:
        reasons.append("excluded_members_unlabeled")
    if any(not s["disclosed_before_commit"] for s in review["side_effects"]) or not review[
        "side_effects_disclosed"
    ]:
        reasons.append("side_effect_undisclosed")
    if not review["rollback_path_present"] or not integ["rollback_visible"]:
        reasons.append("rollback_consequences_hidden")
    if not review["commit_action_is_specific"] or not integ["commit_action_specific"]:
        reasons.append("generic_continue_action")

    # Draft recovery + session.
    if (
        not recovery["recoverable_after_interruption"]
        or recovery["recovery_behavior"] == "no_recovery"
        or not integ["recoverable_draft_preserved"]
    ):
        reasons.append("draft_recovery_lost")
    if not session["autosave_enabled"]:
        reasons.append("autosave_unavailable")
    if not session["restore_prompt_on_reopen"]:
        reasons.append("restore_prompt_missing")

    # Validation visibility.
    if not integ["validation_state_visible"]:
        reasons.append("validation_state_unlabeled")

    # Backing freshness.
    if not integ["freshness_state_visible"]:
        reasons.append("freshness_unlabeled")
    fs = surface["declared_freshness_state"]
    if fs == "missing":
        reasons.append("form_backing_missing")
    elif fs == "superseded_by_newer_source" and not integ["superseded_state_marked"]:
        reasons.append("superseded_state_not_marked")
    elif fs == "stale_expired" and not overlay:
        reasons.append("surface_stale")

    # Verification proof.
    pc = ver["proof_currency"]
    if pc == "missing_proof":
        reasons.append("verification_proof_missing")
    elif pc in ("stale_expired", "requires_review"):
        reasons.append("verification_proof_stale")
    elif pc in ("verified_current", "cached_within_window") and stale_window:
        reasons.append("verification_proof_stale")

    # Overlay read-only.
    if overlay and not integ["imported_review_read_only"]:
        reasons.append("imported_state_reads_as_applied")

    # Reopen-to-origin.
    if not integ["reopen_visible_on_demand"] or any(
        not r["provenance_visible"] for r in surface["renderings"]
    ):
        reasons.append("reopen_path_lost")
    if surface["declared_reopen_target"] == "none_keyboard_fallback":
        reasons.append("reopen_path_lost")

    return reasons


def derive_effective(claimed: str, reasons: list[str]) -> str:
    if any(r in FLOOR_REASONS for r in reasons):
        return "surface_unsafe"
    if not reasons:
        return claimed
    if claimed == "surface_review_overlay":
        return "surface_unsafe"
    return "surface_narrowed"


def surface_reasons(surface: dict, stale_window: bool) -> list[str]:
    claimed = claimed_claim(surface)
    reasons = intrinsic_reasons(surface, stale_window)
    intrinsic = derive_effective(claimed, reasons)
    if any(overclaims(intrinsic, r["rendered_claim"]) for r in surface["renderings"]):
        reasons.append("surface_overclaims")
    return order_reasons(reasons)


def narrow(surface: dict, stale_window: bool) -> dict:
    claimed = claimed_claim(surface)
    if claimed == LABS_CLAIM:
        return {"claimed": LABS_CLAIM, "effective": LABS_CLAIM, "reasons": [], "narrowed": False}
    reasons = surface_reasons(surface, stale_window)
    effective = derive_effective(claimed, reasons)
    er = CLAIM_RANK.get(effective)
    cr = CLAIM_RANK.get(claimed)
    narrowed = er is not None and cr is not None and er < cr
    return {"claimed": claimed, "effective": effective, "reasons": reasons, "narrowed": narrowed}


def floored_keeps_fallback(surface: dict, effective: str) -> bool:
    if effective != "surface_unsafe":
        return True
    if surface["declared_reopen_target"] in ("draft_only", "none_keyboard_fallback"):
        return True
    return present(surface["lineage"].get("reopen_backlink_ref"))


def surface_overclaims(surface: dict, effective: str) -> bool:
    return any(overclaims(effective, r["rendered_claim"]) for r in surface["renderings"])


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
    kinds: set[str] = set()
    lanes: set[str] = set()
    mutations: set[str] = set()
    sources: set[str] = set()
    consumers: set[str] = set()
    demonstrates_narrowing = False
    stale_window = False  # canonical packet is evaluated at its own as_of

    for s in surfaces:
        sid = s.get("surface_id", "")
        if sid in seen:
            v.append("duplicate_surface_id")
        seen.add(sid)
        kinds.add(s.get("surface_kind"))
        lanes.add(s.get("lane"))
        mutations.add(s.get("mutation_class"))
        for f in s.get("fields", []):
            sources.add(f.get("source_class"))
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
        if not s.get("fields"):
            v.append("surface_missing_fields")
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
        if surface_overclaims(s, decision["effective"]):
            v.append("rendering_surface_overclaims")

    if kinds != SURFACE_KINDS:
        v.append("surface_kind_missing")
    if lanes != FORM_LANES:
        v.append("form_lane_missing")
    if mutations != MUTATION_CLASSES:
        v.append("mutation_class_missing")
    if sources != SOURCE_CLASSES:
        v.append("source_of_value_class_missing")
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
    dist = {"certified": 0, "narrowed": 0, "overlay": 0, "unsafe": 0, "labs": 0}
    bucket = {
        "surface_certified": "certified",
        "surface_narrowed": "narrowed",
        "surface_review_overlay": "overlay",
        "surface_unsafe": "unsafe",
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


def apply_overrides(surface: dict, overrides: dict) -> dict:
    out = json.loads(json.dumps(surface))
    for dotted, value in overrides.items():
        _set_path(out, dotted.split("."), value)
    return out


def base_surface(surfaces: list[dict], sid: str) -> dict:
    for s in surfaces:
        if s["surface_id"] == sid:
            return s
    raise SystemExit(f"base surface not found: {sid}")


S_PROVIDER = "form:provider-credentials:0001"
S_SETTINGS = "form:settings-config:0001"
S_PROJECTS = "wizard:project-bootstrap:0001"
S_PACKAGE = "sheet:package-install-review:0001"
S_ADMIN = "sheet:admin-policy-rollout:0001"
S_REQUEST = "dialog:request-workspace-run:0001"
S_IMPORT = "dialog:migration-restore-review:0001"
S_LABS = "wizard:experimental-onboarding:0001"

UNSAFE = "surface_unsafe"
NARROW = "surface_narrowed"
CERT = "surface_certified"
OVERLAY = "surface_review_overlay"

# (case_id, base_id, overrides, stale_window, description, effective, narrowed, reasons)
CASES = [
    ("clean-certified", S_SETTINGS, {}, False,
     "A clean first-party settings form with labelled source, distinct draft, and disclosed scope certifies.",
     CERT, False, []),
    ("field-source-hidden", S_SETTINGS,
     {"fields[0].source_class_labeled": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A field whose source-of-value class is hidden floors the form to an explicit blocked state.",
     UNSAFE, True, ["field_source_hidden"]),
    ("user-override-not-distinct", S_SETTINGS,
     {"fields[1].override_distinct_from_default": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A user override that is not distinct from the value it replaced floors the form.",
     UNSAFE, True, ["field_source_hidden"]),
    ("draft-applied-ambiguous", S_SETTINGS,
     {"integrity.draft_applied_distinct": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "Draft versus applied state that is not visibly distinct floors the form.",
     UNSAFE, True, ["draft_applied_ambiguous"]),
    ("policy-lock-overridden", S_PROVIDER,
     {"fields[1].policy_lock_respected": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A silently overridden policy lock floors the form.",
     UNSAFE, True, ["policy_lock_overridden_silently"]),
    ("submit-over-invalid", S_SETTINGS,
     {"fields[0].validation_state": "invalid_blocking", "renderings[*].rendered_claim": UNSAFE}, False,
     "A form reachable to submit while a field is invalid-blocking floors.",
     UNSAFE, True, ["submit_allowed_while_blocking_invalid"]),
    ("blocked-prereq-hidden", S_PROJECTS,
     {"submit_blockers[0].explained_before_submit": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A blocked prerequisite not explained before submit floors the wizard.",
     UNSAFE, True, ["blocked_prereq_hidden"]),
    ("target-scope-hidden", S_PACKAGE,
     {"staged_review.target_scope_declared": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A staged review that hides its target scope floors the sheet.",
     UNSAFE, True, ["target_scope_hidden"]),
    ("omitted-defaults-hidden", S_SETTINGS,
     {"staged_review.omitted_defaults_disclosed": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A staged review that hides omitted defaults floors the form.",
     UNSAFE, True, ["omitted_defaults_hidden"]),
    ("side-effect-undisclosed", S_PACKAGE,
     {"staged_review.side_effects[0].disclosed_before_commit": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A side effect not disclosed before commit floors the sheet.",
     UNSAFE, True, ["side_effect_undisclosed"]),
    ("rollback-hidden", S_PACKAGE,
     {"staged_review.rollback_path_present": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A hidden rollback/export consequence floors the sheet.",
     UNSAFE, True, ["rollback_consequences_hidden"]),
    ("generic-continue", S_PROJECTS,
     {"staged_review.commit_action_is_specific": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A generic Continue commit action that hides scope/effect floors the wizard.",
     UNSAFE, True, ["generic_continue_action"]),
    ("draft-recovery-lost", S_SETTINGS,
     {"draft_recovery.recoverable_after_interruption": False, "draft_recovery.recovery_behavior": "no_recovery",
      "renderings[*].rendered_claim": UNSAFE}, False,
     "Discarding a recoverable draft on interruption floors the form.",
     UNSAFE, True, ["draft_recovery_lost"]),
    ("imported-reads-as-applied", S_IMPORT,
     {"integrity.imported_review_read_only": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "An imported/restore review that reads as a local apply floors below the review overlay.",
     UNSAFE, True, ["imported_state_reads_as_applied"]),
    ("reopen-path-lost", S_SETTINGS,
     {"declared_reopen_target": "none_keyboard_fallback", "renderings[*].rendered_claim": UNSAFE}, False,
     "Losing reopen-to-origin floors the form but keeps the keyboard fallback.",
     UNSAFE, True, ["reopen_path_lost"]),
    ("surface-hides-provenance", S_SETTINGS,
     {"renderings[0].provenance_visible": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A rendering surface that cannot reveal provenance floors the form.",
     UNSAFE, True, ["reopen_path_lost"]),
    ("missing-backing", S_SETTINGS,
     {"declared_freshness_state": "missing", "renderings[*].rendered_claim": UNSAFE}, False,
     "Missing backing data floors the form.",
     UNSAFE, True, ["form_backing_missing"]),
    ("surface-overclaims", S_REQUEST,
     {"renderings[0].rendered_claim": CERT}, False,
     "A narrowed surface whose rendering still shows certified floors as an overclaim.",
     UNSAFE, True, ["surface_overclaims", "verification_proof_stale"]),
    ("autosave-unavailable", S_SETTINGS,
     {"session.autosave_enabled": False, "renderings[*].rendered_claim": NARROW}, False,
     "Missing local draft autosave narrows a first-party form but keeps it recoverable.",
     NARROW, True, ["autosave_unavailable"]),
    ("validation-unlabeled", S_SETTINGS,
     {"fields[0].validation_state_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding a field's validation state narrows the form.",
     NARROW, True, ["validation_state_unlabeled"]),
    ("restore-prompt-missing", S_SETTINGS,
     {"session.restore_prompt_on_reopen": False, "renderings[*].rendered_claim": NARROW}, False,
     "No restore prompt on reopen narrows the form while the draft stays preserved.",
     NARROW, True, ["restore_prompt_missing"]),
    ("cross-field-unexplained", S_PROJECTS,
     {"submit_blockers[0].blocker_class": "cross_field_conflict", "submit_blockers[0].explained_before_submit": False,
      "renderings[*].rendered_claim": NARROW}, False,
     "An unexplained cross-field dependency narrows the wizard.",
     NARROW, True, ["cross_field_dependency_unexplained"]),
    ("async-validation-pending", S_PROJECTS,
     {"submit_blockers[0].blocker_class": "pending_validation", "submit_blockers[0].explained_before_submit": False,
      "renderings[*].rendered_claim": NARROW}, False,
     "A pending async validation blocker narrows the wizard.",
     NARROW, True, ["async_validation_pending"]),
    ("excluded-members-unlabeled", S_PACKAGE,
     {"staged_review.members_classes_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "Unlabelled included/excluded/blocked members narrow the sheet.",
     NARROW, True, ["excluded_members_unlabeled"]),
    ("freshness-unlabeled", S_SETTINGS,
     {"integrity.freshness_state_visible": False, "renderings[*].rendered_claim": NARROW}, False,
     "Hiding the backing freshness state narrows the form.",
     NARROW, True, ["freshness_unlabeled"]),
    ("first-party-stale", S_SETTINGS,
     {"declared_freshness_state": "stale_expired", "renderings[*].rendered_claim": NARROW}, False,
     "A first-party stale backing source narrows rather than reading as fresh.",
     NARROW, True, ["surface_stale"]),
    ("superseded-unmarked", S_SETTINGS,
     {"declared_freshness_state": "superseded_by_newer_source", "integrity.superseded_state_marked": False,
      "renderings[*].rendered_claim": NARROW}, False,
     "An unmarked superseded backing source narrows the form.",
     NARROW, True, ["superseded_state_not_marked"]),
    ("superseded-marked-ok", S_SETTINGS,
     {"declared_freshness_state": "superseded_by_newer_source"}, False,
     "A marked superseded backing source stays certified because the state is visible.",
     CERT, False, []),
    ("missing-proof", S_SETTINGS,
     {"verification.proof_currency": "missing_proof", "verification.proof_ref": None,
      "renderings[*].rendered_claim": NARROW}, False,
     "A missing verification proof narrows a first-party form.",
     NARROW, True, ["verification_proof_missing"]),
    ("stale-window", S_SETTINGS,
     {"renderings[*].rendered_claim": NARROW}, True,
     "An elapsed verification window ages a current proof out to narrowed.",
     NARROW, True, ["verification_proof_stale"]),
    ("request-narrowed-baseline", S_REQUEST, {}, False,
     "The canonical remote run dialog narrows via a verification proof that requires review.",
     NARROW, True, ["verification_proof_stale"]),
    ("import-overlay-baseline", S_IMPORT, {}, False,
     "The canonical migration restore review stays a read-only review overlay.",
     OVERLAY, False, []),
    ("overlay-any-gap-floors", S_IMPORT,
     {"session.autosave_enabled": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A review overlay with any non-floor gap drops below the overlay rather than holding it.",
     UNSAFE, True, ["autosave_unavailable"]),
    ("labs-not-claimed", S_LABS, {}, False,
     "A Labs onboarding wizard makes no public claim and is never widened or narrowed.",
     LABS_CLAIM, False, []),
]


def run_corpus_from_cases(surfaces: list[dict]) -> list[str]:
    failures: list[str] = []
    for case in CASES:
        case_id, base_id, overrides, stale_window, _desc, exp_eff, exp_narrowed, exp_reasons = case
        s = apply_overrides(base_surface(surfaces, base_id), overrides)
        decision = narrow(s, stale_window)
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
        s = apply_overrides(
            base_surface(surfaces, payload["base_surface_id"]),
            payload["overrides"],
        )
        decision = narrow(s, payload["stale_window"])
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
        "corpus_id": "m5-structured-input-corpus:0001",
        "description": (
            "Perturbation corpus for the structured-input / staged-review narrowing engine. "
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
        sys.stderr.write("structured-input set FAILED validation:\n")
        for v in violations:
            sys.stderr.write(f"  - {v}\n")
        return 1
    dist = claim_distribution(packet["surfaces"])
    sys.stdout.write(
        f"structured-input set OK: {len(packet['surfaces'])} surfaces, distribution {dist}\n"
    )
    return 0


def cmd_corpus(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    surfaces = packet["surfaces"]
    failures = run_corpus_from_cases(surfaces)
    failures += run_corpus_from_disk(repo_root, surfaces)
    if failures:
        sys.stderr.write("structured-input corpus FAILED:\n")
        for f in failures:
            sys.stderr.write(f"  - {f}\n")
        return 1
    sys.stdout.write(f"structured-input corpus OK: {len(CASES)} cases\n")
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
