#!/usr/bin/env python3
"""Freeze and certify the M5 keyboard, assistive-tech, reduced-motion, and
interruption-safe continuity set: the accessibility and interruption-safety contract
every M5 dense multi-step form, inline validation-link group, and batch/staged-review
sheet must hold so the shared structured-input model stays fully usable under
keyboard-only, assistive-tech, reduced-motion, reconnect, and restart conditions.

One contract is reused across the provider, admin, request, package, settings, import,
and project lanes. The canonical truth is the checked-in support export
(``artifacts/ux/m5-accessibility-and-continuity/support_export.json``). Each surface
binds its keyboard reachability, its assistive-tech reachability, its reduced-motion
behavior (bound to the shared substitution class), and its interruption-safe continuity
(a recovery journal that preserves the current step, blocked fields, and draft). This
tool ingests that set and, per surface, **independently** re-derives an effective claim
that never reads wider than the evidence supports:

* every interactive control is keyboard-reachable, the focus order is deterministic, the
  focus trap is escapable, and batch-review actions have keyboard parity;
* every control carries a permanent screen-reader label, inline validation links are
  announced to assistive tech, blocked-submit reasons are read from a live region, and
  the step position is announced;
* state is conveyed without depending on motion under the shared reduced-motion class;
* an interrupted flow resumes on the correct step with blocked fields and draft intact
  across reconnect, restart, missing dependency, and crash, backed by a recovery
  journal;
* an imported/migration review stays read-only, the keyboard recovery path is kept, and
  no rendering surface overclaims;
* a surface that breaks any of these floors to continuity_unsafe and falls back to an
  explicit blocked-submit state with a keyboard recovery path, while a labelled
  recoverable gap holds a first-party surface at continuity_narrowed.

The Rust truth source is ``crates/aureline-ui/src/m5_accessibility_and_continuity``;
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
SUPPORT_EXPORT_REF = "artifacts/ux/m5-accessibility-and-continuity/support_export.json"
REPORT_REF = "artifacts/ux/m5-accessibility-and-continuity/report.md"
SCHEMA_REF = "schemas/ux/m5-accessibility-and-continuity.schema.json"
FIXTURE_DIR = "fixtures/ux/m5-accessibility-and-continuity"

RECORD_KIND = "m5_accessibility_continuity_set_packet"
SCHEMA_VERSION = 1
TAXONOMY_VERSION = 1

OVERLAY_ORIGINS = {"imported_review"}

REDACTION_TOKENS = {
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
}

SURFACE_KINDS = {
    "multi_step_form",
    "inline_validation_links",
    "batch_review_sheet",
    "staged_review_sheet",
    "config_editor",
}
BATCH_KINDS = {"batch_review_sheet", "staged_review_sheet"}
VALIDATION_LINK_KINDS = {
    "multi_step_form",
    "inline_validation_links",
    "staged_review_sheet",
    "config_editor",
}
SURFACE_LANES = {"provider", "admin", "request", "package", "settings", "import", "projects"}
SURFACE_ORIGINS = {"local_form", "remote_form", "provider_form", "imported_review"}
REDUCED_MOTION_CLASSES = {
    "crossfade_only",
    "maintain_essential_keep_simplified",
    "suppress_entirely",
    "collapse_to_instant",
    "non_motion_state_marker",
}
CONSUMER_SURFACES = {
    "live_surface",
    "review_sheet",
    "diagnostics_panel",
    "support_export",
    "accessibility_audit",
    "help_inline",
    "cli_headless",
}
INTERRUPTION_FLAGS = {
    "reconnect": "resume_on_reconnect",
    "restart": "resume_on_restart",
    "missing_dependency": "resume_on_missing_dependency",
    "crash_recovery": "resume_on_crash",
}

LABS_CLAIM = "continuity_labs_not_claimed"
CLAIM_RANK = {
    "continuity_unsafe": 0,
    "continuity_review_overlay": 1,
    "continuity_narrowed": 2,
    "continuity_certified": 3,
}

# Deterministic reason ordering (mirrors the Rust order_index).
REASON_ORDER = {
    "keyboard_path_incomplete": 0,
    "focus_order_undefined": 1,
    "batch_actions_keyboard_unreachable": 2,
    "screen_reader_labels_missing": 3,
    "validation_links_not_announced": 4,
    "blocked_submit_not_announced": 5,
    "motion_only_state": 6,
    "current_step_lost": 7,
    "blocked_fields_lost": 8,
    "draft_state_lost": 9,
    "imported_review_mutable": 10,
    "recovery_path_lost": 11,
    "surface_overclaims": 12,
    "continuity_journal_missing": 13,
    "step_position_unannounced": 14,
    "focus_trap_escape_unlabeled": 15,
    "reduced_motion_substitution_unlabeled": 16,
    "progress_marker_unlabeled": 17,
    "journal_partial": 18,
    "continuity_proof_stale": 19,
    "continuity_proof_missing": 20,
}

# Floor reasons break the contract outright rather than merely aging out.
FLOOR_REASONS = {
    r for r, idx in REASON_ORDER.items() if idx <= REASON_ORDER["continuity_journal_missing"]
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
        return "continuity_review_overlay"
    return "continuity_certified"


def intrinsic_reasons(rec: dict, stale_window: bool) -> list[str]:
    acc = rec["accessibility"]
    kb = acc["keyboard"]
    at = acc["assistive_tech"]
    rm = acc["reduced_motion"]
    cont = rec["continuity"]
    integ = rec["integrity"]
    overlay = rec["origin"] in OVERLAY_ORIGINS
    batch = rec["surface_kind"] in BATCH_KINDS
    has_validation = rec["surface_kind"] in VALIDATION_LINK_KINDS
    reasons: list[str] = []

    # Keyboard completeness.
    if (
        not kb["all_controls_reachable"]
        or not kb["focus_trap_escapable"]
        or not integ["keyboard_complete"]
    ):
        reasons.append("keyboard_path_incomplete")

    # Deterministic focus order.
    if not kb["focus_order_defined"] or not integ["focus_order_deterministic"]:
        reasons.append("focus_order_undefined")

    # Batch-review action keyboard parity.
    if batch and (
        not kb["batch_actions_keyboard_parity"] or not integ["batch_actions_keyboard_reachable"]
    ):
        reasons.append("batch_actions_keyboard_unreachable")

    # Screen-reader labels.
    if not at["screen_reader_labels_present"] or not integ["screen_reader_reachable"]:
        reasons.append("screen_reader_labels_missing")

    # Inline validation links announced.
    if has_validation and (
        not at["validation_links_announced"] or not integ["validation_links_in_at"]
    ):
        reasons.append("validation_links_not_announced")

    # Blocked-submit live region (mutation-capable, non-overlay).
    if not overlay and (
        not at["blocked_submit_live_region"] or not integ["blocked_submit_in_live_region"]
    ):
        reasons.append("blocked_submit_not_announced")

    # Motion-only state (the reduced-motion guardrail).
    if not rm["state_conveyed_without_motion"] or not integ["state_without_motion"]:
        reasons.append("motion_only_state")

    # Interruption-safe continuity (mutation-capable, non-overlay).
    if not overlay:
        if not cont["current_step_preserved"] or not integ["step_resumes_correctly"]:
            reasons.append("current_step_lost")
        if not cont["blocked_fields_preserved"] or not integ["blocked_fields_retained"]:
            reasons.append("blocked_fields_lost")
        if not cont["draft_state_preserved"] or not integ["draft_state_retained"]:
            reasons.append("draft_state_lost")

    # Imported overlay read-only.
    if overlay and not integ["imported_review_read_only"]:
        reasons.append("imported_review_mutable")

    # Keyboard recovery path.
    if any(not r["recovery_visible"] for r in rec["renderings"]) or (
        rec["declared_recovery_target"] == "none_keyboard_fallback"
    ):
        reasons.append("recovery_path_lost")

    # Continuity journal backing (mutation-capable, non-overlay).
    if not overlay:
        js = cont["journal_state"]
        if js == "missing":
            reasons.append("continuity_journal_missing")
        elif js in ("partial", "stale"):
            reasons.append("journal_partial")

    # Step position announced (non-floor).
    if not at["step_position_announced"] or not integ["step_position_announced"]:
        reasons.append("step_position_unannounced")

    # Focus-trap escape labelling (non-floor).
    if not kb["focus_trap_escape_labeled"]:
        reasons.append("focus_trap_escape_unlabeled")

    # Reduced-motion substitution labelling (non-floor).
    if not rm["substitution_labeled"]:
        reasons.append("reduced_motion_substitution_unlabeled")

    # Progress non-motion marker labelling (non-floor; only when a marker is used).
    if rm["progress_non_motion_marker"] and not rm["progress_marker_labeled"]:
        reasons.append("progress_marker_unlabeled")

    # Verification proof.
    pc = rec["verification"]["proof_currency"]
    if pc == "missing_proof":
        reasons.append("continuity_proof_missing")
    elif pc in ("stale_expired", "requires_review"):
        reasons.append("continuity_proof_stale")
    elif pc in ("verified_current", "cached_within_window") and stale_window:
        reasons.append("continuity_proof_stale")

    return reasons


def derive_effective(claimed: str, reasons: list[str]) -> str:
    if any(r in FLOOR_REASONS for r in reasons):
        return "continuity_unsafe"
    if not reasons:
        return claimed
    if claimed == "continuity_review_overlay":
        return "continuity_unsafe"
    return "continuity_narrowed"


def record_reasons(rec: dict, stale_window: bool) -> list[str]:
    claimed = claimed_claim(rec)
    reasons = intrinsic_reasons(rec, stale_window)
    intrinsic = derive_effective(claimed, reasons)
    if any(overclaims(intrinsic, r["rendered_claim"]) for r in rec["renderings"]):
        reasons.append("surface_overclaims")
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
    if effective != "continuity_unsafe":
        return True
    return rec["declared_recovery_target"] in ("step_only", "none_keyboard_fallback") or present(
        rec["lineage"]["recovery_backlink_ref"]
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


def preserved_paths(rec: dict) -> set[str]:
    cont = rec["continuity"]
    return {path for path, flag in INTERRUPTION_FLAGS.items() if cont.get(flag)}


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
    origins: set[str] = set()
    motion_classes: set[str] = set()
    consumers: set[str] = set()
    paths: set[str] = set()
    demonstrates_narrowing = False
    stale_window = False  # canonical packet is evaluated at its own as_of

    for s in surfaces:
        sid = s.get("surface_id", "")
        if sid in seen:
            v.append("duplicate_surface_id")
        seen.add(sid)
        kinds.add(s.get("surface_kind"))
        lanes.add(s.get("lane"))
        origins.add(s.get("origin"))
        motion_classes.add(s.get("accessibility", {}).get("reduced_motion", {}).get("substitution_class"))
        paths |= preserved_paths(s)
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

    if not SURFACE_KINDS.issubset(kinds):
        v.append("surface_kind_missing")
    if not SURFACE_LANES.issubset(lanes):
        v.append("surface_lane_missing")
    if not SURFACE_ORIGINS.issubset(origins):
        v.append("surface_origin_missing")
    if set(INTERRUPTION_FLAGS).issubset(paths) is False:
        v.append("interruption_path_missing")
    if not REDUCED_MOTION_CLASSES.issubset(motion_classes):
        v.append("reduced_motion_class_missing")
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
        "continuity_certified": "certified",
        "continuity_narrowed": "narrowed",
        "continuity_review_overlay": "overlay",
        "continuity_unsafe": "unsafe",
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


S_PROVIDER = "surface:provider-connect-wizard:0001"
S_ADMIN = "surface:admin-source-batch-review:0001"
S_REQUEST = "surface:request-environment-validation:0001"
S_PACKAGE = "surface:package-install-review:0001"
S_SETTINGS = "surface:settings-config-editor:0001"
S_IMPORT = "surface:import-migration-review:0001"
S_LABS = "surface:project-bootstrap-wizard:0001"

UNSAFE = "continuity_unsafe"
NARROW = "continuity_narrowed"
CERT = "continuity_certified"
OVERLAY = "continuity_review_overlay"

# (case_id, base_id, overrides, stale_window, description, effective, narrowed, reasons)
CASES = [
    ("clean-certified", S_SETTINGS, {}, False,
     "A keyboard-complete, AT-reachable, reduced-motion-safe, crash-safe config editor certifies.",
     CERT, False, []),
    ("provider-multistep-certified", S_PROVIDER, {}, False,
     "A dense multi-step wizard that resumes on the correct step after a reconnect certifies.",
     CERT, False, []),
    ("admin-batch-certified", S_ADMIN, {}, False,
     "A batch-review sheet whose actions all have keyboard parity certifies.",
     CERT, False, []),
    ("package-staged-certified", S_PACKAGE, {}, False,
     "A staged review sheet whose scope and rollback path are keyboard-reachable and announced certifies.",
     CERT, False, []),
    ("request-narrowed-baseline", S_REQUEST, {}, False,
     "An inline validation-links group whose recovery journal is partial narrows.",
     NARROW, True, ["journal_partial"]),
    ("import-overlay-baseline", S_IMPORT, {}, False,
     "A read-only import/migration review stays a review overlay, never an apply.",
     OVERLAY, False, []),
    ("labs-not-claimed", S_LABS, {}, False,
     "A Labs project-bootstrap wizard makes no public claim and is never widened or narrowed.",
     LABS_CLAIM, False, []),

    # Floors.
    ("keyboard-path-incomplete", S_SETTINGS,
     {"accessibility.keyboard.all_controls_reachable": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A surface where not every control is keyboard-reachable floors.",
     UNSAFE, True, ["keyboard_path_incomplete"]),
    ("focus-trap-inescapable", S_PROVIDER,
     {"accessibility.keyboard.focus_trap_escapable": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A surface whose focus trap cannot be escaped floors.",
     UNSAFE, True, ["keyboard_path_incomplete"]),
    ("focus-order-undefined", S_SETTINGS,
     {"accessibility.keyboard.focus_order_defined": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A surface with no deterministic focus order floors.",
     UNSAFE, True, ["focus_order_undefined"]),
    ("batch-actions-keyboard-unreachable", S_ADMIN,
     {"accessibility.keyboard.batch_actions_keyboard_parity": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A batch-review action with no keyboard parity floors.",
     UNSAFE, True, ["batch_actions_keyboard_unreachable"]),
    ("batch-parity-exempt-non-batch", S_SETTINGS,
     {"accessibility.keyboard.batch_actions_keyboard_parity": False}, False,
     "A non-batch surface ignores an unset batch-parity flag and stays certified.",
     CERT, False, []),
    ("screen-reader-labels-missing", S_SETTINGS,
     {"accessibility.assistive_tech.screen_reader_labels_present": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A control missing a permanent screen-reader label floors.",
     UNSAFE, True, ["screen_reader_labels_missing"]),
    ("validation-links-not-announced", S_PACKAGE,
     {"accessibility.assistive_tech.validation_links_announced": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "An inline validation link not announced to assistive tech floors.",
     UNSAFE, True, ["validation_links_not_announced"]),
    ("validation-links-exempt-batch", S_ADMIN,
     {"accessibility.assistive_tech.validation_links_announced": False}, False,
     "A batch-review sheet has no inline validation links, so an unset announce flag stays certified.",
     CERT, False, []),
    ("blocked-submit-not-announced", S_SETTINGS,
     {"accessibility.assistive_tech.blocked_submit_live_region": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A blocked-submit reason not surfaced through a live region floors.",
     UNSAFE, True, ["blocked_submit_not_announced"]),
    ("motion-only-state", S_SETTINGS,
     {"accessibility.reduced_motion.state_conveyed_without_motion": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A state conveyed only by motion floors because reduced motion loses it.",
     UNSAFE, True, ["motion_only_state"]),
    ("current-step-lost", S_PROVIDER,
     {"continuity.current_step_preserved": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "An interrupted flow that does not resume on the correct step floors.",
     UNSAFE, True, ["current_step_lost"]),
    ("blocked-fields-lost", S_PROVIDER,
     {"continuity.blocked_fields_preserved": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "Losing blocked-field context across an interruption floors.",
     UNSAFE, True, ["blocked_fields_lost"]),
    ("draft-state-lost", S_PROVIDER,
     {"continuity.draft_state_preserved": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "Losing draft-state continuity across an interruption floors.",
     UNSAFE, True, ["draft_state_lost"]),
    ("imported-review-mutable", S_IMPORT,
     {"integrity.imported_review_read_only": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "An imported/migration review that is mutable / reads as an apply floors below the overlay.",
     UNSAFE, True, ["imported_review_mutable"]),
    ("recovery-path-lost-keeps-fallback", S_SETTINGS,
     {"declared_recovery_target": "none_keyboard_fallback", "renderings[*].rendered_claim": UNSAFE}, False,
     "Losing the keyboard recovery path floors but keeps a keyboard fallback.",
     UNSAFE, True, ["recovery_path_lost"]),
    ("continuity-journal-missing", S_SETTINGS,
     {"continuity.journal_state": "missing", "renderings[*].rendered_claim": UNSAFE}, False,
     "A mutation-capable surface with no recovery journal floors.",
     UNSAFE, True, ["continuity_journal_missing"]),
    ("overlay-any-gap-floors", S_IMPORT,
     {"accessibility.reduced_motion.substitution_labeled": False, "renderings[*].rendered_claim": UNSAFE}, False,
     "A review overlay with any non-floor gap drops below the overlay rather than holding it.",
     UNSAFE, True, ["reduced_motion_substitution_unlabeled"]),
    ("surface-overclaims", S_REQUEST,
     {"renderings[0].rendered_claim": CERT}, False,
     "A narrowed surface whose rendering still shows certified floors as an overclaim.",
     UNSAFE, True, ["surface_overclaims", "journal_partial"]),

    # Narrows.
    ("step-position-unannounced", S_SETTINGS,
     {"accessibility.assistive_tech.step_position_announced": False,
      "integrity.step_position_announced": False, "renderings[*].rendered_claim": NARROW}, False,
     "Not announcing the current step position narrows the surface.",
     NARROW, True, ["step_position_unannounced"]),
    ("focus-trap-escape-unlabeled", S_SETTINGS,
     {"accessibility.keyboard.focus_trap_escape_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "An unlabelled focus-trap escape affordance narrows the surface.",
     NARROW, True, ["focus_trap_escape_unlabeled"]),
    ("reduced-motion-substitution-unlabeled", S_SETTINGS,
     {"accessibility.reduced_motion.substitution_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "A generic/unlabelled reduced-motion substitution narrows the surface.",
     NARROW, True, ["reduced_motion_substitution_unlabeled"]),
    ("progress-marker-unlabeled", S_SETTINGS,
     {"accessibility.reduced_motion.progress_marker_labeled": False, "renderings[*].rendered_claim": NARROW}, False,
     "An unlabelled step-progress non-motion marker narrows the surface.",
     NARROW, True, ["progress_marker_unlabeled"]),
    ("absent-progress-marker-ok", S_SETTINGS,
     {"accessibility.reduced_motion.progress_non_motion_marker": False,
      "accessibility.reduced_motion.progress_marker_labeled": False}, False,
     "With no progress marker in use, an unset marker label is exempt and the surface stays certified.",
     CERT, False, []),
    ("journal-stale", S_SETTINGS,
     {"continuity.journal_state": "stale", "renderings[*].rendered_claim": NARROW}, False,
     "A stale recovery journal narrows the surface rather than reading as complete.",
     NARROW, True, ["journal_partial"]),
    ("proof-missing", S_SETTINGS,
     {"verification.proof_currency": "missing_proof", "verification.proof_ref": None,
      "renderings[*].rendered_claim": NARROW}, False,
     "A missing verification proof narrows the surface.",
     NARROW, True, ["continuity_proof_missing"]),
    ("proof-requires-review", S_PROVIDER,
     {"verification.proof_currency": "requires_review", "renderings[*].rendered_claim": NARROW}, False,
     "A verification proof that requires review narrows the surface.",
     NARROW, True, ["continuity_proof_stale"]),
    ("stale-window", S_SETTINGS,
     {"renderings[*].rendered_claim": NARROW}, True,
     "An elapsed verification window ages a current proof out to narrowed.",
     NARROW, True, ["continuity_proof_stale"]),
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
        "corpus_id": "m5-accessibility-and-continuity-corpus:0001",
        "description": (
            "Perturbation corpus for the accessibility-and-continuity narrowing engine. Each "
            "case starts from a canonical surface, applies dotted-path overrides, and asserts "
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
        sys.stderr.write("accessibility-continuity set FAILED validation:\n")
        for v in violations:
            sys.stderr.write(f"  - {v}\n")
        return 1
    dist = claim_distribution(packet["surfaces"])
    sys.stdout.write(
        f"accessibility-continuity set OK: {len(packet['surfaces'])} surfaces, distribution {dist}\n"
    )
    return 0


def cmd_corpus(repo_root: Path) -> int:
    packet = load_support_export(repo_root)
    surfaces = packet["surfaces"]
    failures = run_corpus_from_cases(surfaces)
    failures += run_corpus_from_disk(repo_root, surfaces)
    if failures:
        sys.stderr.write("accessibility-continuity corpus FAILED:\n")
        for f in failures:
            sys.stderr.write(f"  - {f}\n")
        return 1
    sys.stdout.write(f"accessibility-continuity corpus OK: {len(CASES)} cases\n")
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
