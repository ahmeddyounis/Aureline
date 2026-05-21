#!/usr/bin/env python3
"""Validate the claim-narrowing automation for optional surfaces that lack a stable
qualification packet.

The stable claim manifest decides the single canonical lifecycle label each *subject*
publishes; the stable qualification matrix and the stable proof index ground the surfaces
that are *meant* to ship at the cutline. This register is the inverse layer: it governs the
*optional* surfaces — opt-in capabilities, optional integrations, secondary platforms, and
shipped-but-experimental previews — whose default is *narrowed*. For each surface it asks: is
there a fresh, complete, owner-signed stable qualification packet behind it, or is the surface
riding on optimism and an adjacent qualified surface? It reads the checked-in register at
``artifacts/release/optional_surface_qualification.json`` and:

  - asserts the closed vocabularies (lifecycle labels, surface kinds, surface states, narrow
    reasons, narrow actions) and the launch cutline are canonical;
  - enforces the absent-packet rule — a surface with no ``qualification_packet`` must be
    ``narrowed_no_packet``, must name ``qualification_packet_absent``, may never render at or
    above the cutline, and never inherits an adjacent qualified surface;
  - asserts every surface that is narrowed (for an absent packet, a breached packet, an absent
    capability, incomplete evidence, an expired waiver, a missing owner sign-off, or a public
    claim whose canonical label is itself below the cutline) drops below the cutline rather than
    rendering a label wider than the public claim, and that a qualified surface renders the
    public claim's canonical label cleanly with a captured within-SLO packet and an owner
    sign-off;
  - asserts opt-in/integration/platform/preview coverage: every surface kind is represented,
    every declared release-relevant surface ref has exactly one covering release-relevant row,
    every release-relevant row is declared, and no surface ref repeats;
  - performs the ceiling cross-check the typed model cannot — it reads the stable claim manifest
    named by ``claim_manifest_ref`` and fails when a surface's ``claim_label`` is not the label
    the claim manifest publishes for the entry named by ``claim_ref``, or names an entry the
    claim manifest does not carry;
  - performs the packet-freshness SLO automation and the waiver-expiry date arithmetic against
    the register ``as_of`` date the typed model cannot;
  - recomputes the publication verdict and the summary block, and fails on any drift;
  - runs negative drills proving the absent-packet, narrowing, ceiling, freshness, and
    publication rejections all fire; and
  - runs the checked-in fixture cases under ``fixtures/release/optional_surface_qualification``
    and fails when a case the register marks as rejected validates clean.

With ``--require-proceed`` the gate additionally fails (exit code 2) when the recomputed
publication verdict is ``hold``, so shiproom and release tooling can fail promotion directly
from this artifact.

The typed Rust consumer
(``aureline_release::optional_surface_qualification::current_optional_surface_qualification``)
reads the same register and runs the same structural cross-check, so this gate and
``cargo test -p aureline-release`` agree without a cargo build in CI.
"""

from __future__ import annotations

import argparse
import copy
import dataclasses
import datetime as dt
import json
import sys
from pathlib import Path
from typing import Any


DEFAULT_REGISTER_REL = "artifacts/release/optional_surface_qualification.json"
DEFAULT_REPORT_REL = (
    "artifacts/release/captures/optional_surface_qualification_validation_capture.json"
)
DEFAULT_FIXTURES_REL = "fixtures/release/optional_surface_qualification"

EXPECTED_SCHEMA_VERSION = 1
REGISTER_RECORD_KIND = "optional_surface_qualification"

LIFECYCLE_LABELS = ("lts", "stable", "beta", "preview", "withdrawn")
ABOVE_CUTLINE = ("lts", "stable")
BELOW_CUTLINE = ("beta", "preview", "withdrawn")
SURFACE_KINDS = (
    "opt_in_capability",
    "optional_integration",
    "secondary_platform",
    "experimental_preview",
)
FRESHNESS_SLO_STATES = ("current", "due_for_refresh", "breached", "missing")
SURFACE_STATES = (
    "qualified_stable",
    "qualified_on_waiver",
    "narrowed_no_packet",
    "narrowed_incomplete",
    "narrowed_stale",
    "narrowed_claim_narrowed",
    "narrowed_waiver_expired",
)
NARROW_REASONS = (
    "claim_label_narrowed",
    "qualification_packet_absent",
    "surface_capability_absent",
    "surface_evidence_incomplete",
    "qualification_packet_breached",
    "waiver_expired",
    "owner_signoff_missing",
)
NARROW_ACTIONS = (
    "hold_promotion",
    "narrow_surface_label",
    "author_qualification_packet",
    "refresh_qualification_packet",
    "recapture_surface_evidence",
    "request_owner_signoff",
)
QUALIFIED_STATES = ("qualified_stable", "qualified_on_waiver")
NARROWING_STATES = (
    "narrowed_no_packet",
    "narrowed_incomplete",
    "narrowed_stale",
    "narrowed_claim_narrowed",
    "narrowed_waiver_expired",
)

LEVEL_RANK = {"lts": 4, "stable": 3, "beta": 2, "preview": 1, "withdrawn": 0}
CUTLINE_RANK = LEVEL_RANK["stable"]
FRESHNESS_RANK = {"current": 3, "due_for_refresh": 2, "breached": 1, "missing": 0}


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    ref: str

    def as_report(self) -> dict[str, str]:
        return dataclasses.asdict(self)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--register", default=DEFAULT_REGISTER_REL)
    parser.add_argument("--fixtures-dir", default=DEFAULT_FIXTURES_REL)
    parser.add_argument("--report", default=None, help="Optional JSON validation capture path.")
    parser.add_argument(
        "--check",
        action="store_true",
        help="CI mode: validate and write the validation capture (default report path).",
    )
    parser.add_argument(
        "--no-capture",
        action="store_true",
        help="Validate without writing the validation capture.",
    )
    parser.add_argument(
        "--require-proceed",
        action="store_true",
        help="Publication gate: also fail (exit 2) when the recomputed verdict is hold.",
    )
    return parser.parse_args()


def load_json(path: Path, label: str) -> Any:
    if not path.exists():
        raise SystemExit(f"missing {label}: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"{label} is not valid JSON: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a mapping/object")
    return value


def generated_at_now() -> str:
    return dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def json_text(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


def is_str(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def is_int(value: Any) -> bool:
    return isinstance(value, int) and not isinstance(value, bool)


def parse_date(value: Any) -> dt.date | None:
    if not isinstance(value, str):
        return None
    try:
        return dt.date.fromisoformat(value)
    except ValueError:
        return None


def is_above_cutline(level: Any) -> bool:
    return isinstance(level, str) and LEVEL_RANK.get(level, -1) >= CUTLINE_RANK


def packet_of(surface: dict[str, Any]) -> dict[str, Any] | None:
    packet = surface.get("qualification_packet")
    return packet if isinstance(packet, dict) else None


def has_packet(surface: dict[str, Any]) -> bool:
    return packet_of(surface) is not None


def has_capture(packet: dict[str, Any]) -> bool:
    return bool(packet.get("captured_at")) and bool(packet.get("evidence_refs"))


def slo_state_of(surface: dict[str, Any]) -> Any:
    packet = packet_of(surface)
    return packet.get("slo_state") if packet is not None else None


def reasons_of(surface: dict[str, Any]) -> list[str]:
    reasons = surface.get("active_narrow_reasons", [])
    return reasons if isinstance(reasons, list) else []


def stop_rule_fires(rule: dict[str, Any], surfaces: list[dict[str, Any]]) -> bool:
    trigger = rule.get("trigger_reason")
    labels = rule.get("applies_to_labels", [])
    if not isinstance(labels, list):
        return False
    for surface in surfaces:
        if surface.get("claim_label") in labels and trigger in reasons_of(surface):
            return True
    return False


def computed_publication(register: dict[str, Any]) -> tuple[str, list[str], list[str]]:
    surfaces = register.get("surfaces", [])
    rules = register.get("stop_rules", [])
    blocking_rules = [
        rule
        for rule in rules
        if rule.get("blocks_promotion") is True and stop_rule_fires(rule, surfaces)
    ]
    rule_ids = sorted(str(rule.get("rule_id")) for rule in blocking_rules)
    blocking_triggers = {rule.get("trigger_reason") for rule in blocking_rules}
    surface_ids: set[str] = set()
    for surface in surfaces:
        if is_above_cutline(surface.get("claim_label")) and blocking_triggers.intersection(
            reasons_of(surface)
        ):
            surface_ids.add(str(surface.get("surface_id")))
    decision = "hold" if blocking_rules else "proceed"
    return decision, rule_ids, sorted(surface_ids)


def computed_summary(register: dict[str, Any]) -> dict[str, Any]:
    surfaces = register.get("surfaces", [])
    rules = register.get("stop_rules", [])

    def kind_count(kind: str) -> int:
        return sum(1 for s in surfaces if s.get("surface_kind") == kind)

    def packet_count(state: str) -> int:
        return sum(1 for s in surfaces if slo_state_of(s) == state)

    qualified = [s for s in surfaces if is_above_cutline(s.get("displayed_label"))]
    claims = {str(s.get("claim_ref")) for s in surfaces}
    with_packet = [s for s in surfaces if has_packet(s)]
    release_relevant = [s for s in surfaces if s.get("release_relevant") is True]
    rr_qualified = [s for s in release_relevant if is_above_cutline(s.get("displayed_label"))]
    return {
        "total_surfaces": len(surfaces),
        "total_claims": len(claims),
        "surfaces_qualified_stable": len(qualified),
        "surfaces_narrowed_below_cutline": len(surfaces) - len(qualified),
        "surfaces_on_active_waiver": sum(
            1 for s in surfaces if s.get("surface_state") == "qualified_on_waiver"
        ),
        "surfaces_with_packet": len(with_packet),
        "surfaces_without_packet": len(surfaces) - len(with_packet),
        "release_relevant_total": len(release_relevant),
        "release_relevant_qualified": len(rr_qualified),
        "release_relevant_narrowed": len(release_relevant) - len(rr_qualified),
        "opt_in_capability_surfaces": kind_count("opt_in_capability"),
        "optional_integration_surfaces": kind_count("optional_integration"),
        "secondary_platform_surfaces": kind_count("secondary_platform"),
        "experimental_preview_surfaces": kind_count("experimental_preview"),
        "packets_current": packet_count("current"),
        "packets_due_for_refresh": packet_count("due_for_refresh"),
        "packets_breached": packet_count("breached"),
        "total_active_narrow_reasons": sum(len(reasons_of(s)) for s in surfaces),
        "stop_rules_firing": sum(1 for rule in rules if stop_rule_fires(rule, surfaces)),
    }


def computed_slo_state(packet: dict[str, Any], as_of: dt.date | None) -> str | None:
    captured = parse_date(packet.get("captured_at"))
    if captured is None:
        return "missing" if packet.get("captured_at") is None else None
    slo = packet.get("freshness_slo")
    if not isinstance(slo, dict):
        return None
    target = slo.get("target_max_age_days")
    warn = slo.get("warn_within_days")
    if not (is_int(target) and target >= 1):
        return None
    if not (is_int(warn) and warn >= 0):
        return None
    if as_of is None:
        return None
    age = (as_of - captured).days
    if age > target:
        return "breached"
    if (target - age) <= warn:
        return "due_for_refresh"
    return "current"


def validate_envelope(register: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    register_id = str(register.get("register_id", "<register>"))

    if register.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("error", "register.schema_version", "schema_version must be 1", register_id))
    if register.get("record_kind") != REGISTER_RECORD_KIND:
        findings.append(Finding("error", "register.record_kind", "record_kind is not supported", register_id))
    for field in (
        "register_id",
        "status",
        "overview_page",
        "as_of",
        "claim_manifest_ref",
        "freshness_slo_register_ref",
    ):
        if not is_str(register.get(field)):
            findings.append(
                Finding("error", "register.empty_field", f"{field} must be a non-empty string", register_id)
            )
    if parse_date(register.get("as_of")) is None:
        findings.append(Finding("error", "register.as_of_invalid", "as_of must be an ISO date", register_id))

    for field in ("claim_manifest_ref", "freshness_slo_register_ref"):
        ref = register.get(field)
        if is_str(ref) and not (repo_root / ref.split("#", 1)[0]).exists():
            findings.append(
                Finding("error", "register.ref_missing", f"{field} does not exist: {ref}", register_id)
            )

    for key, expected in (
        ("lifecycle_labels", list(LIFECYCLE_LABELS)),
        ("surface_kinds", list(SURFACE_KINDS)),
        ("surface_states", list(SURFACE_STATES)),
        ("narrow_reasons", list(NARROW_REASONS)),
        ("narrow_actions", list(NARROW_ACTIONS)),
    ):
        if list(register.get(key, [])) != expected:
            findings.append(
                Finding("error", "register.vocabulary", f"register.{key} is not the closed vocabulary", key)
            )

    refs = register.get("release_relevant_surface_refs")
    if not isinstance(refs, list) or not refs or any(not is_str(r) for r in refs):
        findings.append(
            Finding(
                "error",
                "register.release_relevant_refs",
                "release_relevant_surface_refs must be a non-empty list of non-empty strings",
                register_id,
            )
        )

    cutline = register.get("launch_cutline")
    if not isinstance(cutline, dict):
        findings.append(Finding("error", "cutline.missing", "register must carry a launch_cutline", register_id))
    else:
        if cutline.get("cutline_level") != "stable":
            findings.append(Finding("error", "cutline.level", "cutline_level must be stable", register_id))
        if list(cutline.get("above_cutline_levels", [])) != list(ABOVE_CUTLINE):
            findings.append(Finding("error", "cutline.above", "above_cutline_levels is not canonical", register_id))
        if list(cutline.get("below_cutline_levels", [])) != list(BELOW_CUTLINE):
            findings.append(Finding("error", "cutline.below", "below_cutline_levels is not canonical", register_id))
        if not is_str(cutline.get("description")):
            findings.append(Finding("error", "cutline.description", "cutline description must be non-empty", register_id))
    return findings


def validate_rules(register: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    rules = register.get("stop_rules")
    if not isinstance(rules, list) or not rules:
        findings.append(Finding("error", "rules.empty", "register must enumerate at least one stop rule", "<register>"))
        return findings

    seen: set[str] = set()
    covered: set[str] = set()
    for rule in rules:
        rule_id = str(rule.get("rule_id", "<rule>"))
        for field in ("rule_id", "title", "rationale"):
            if not is_str(rule.get(field)):
                findings.append(Finding("error", "rule.empty_field", f"rule {field} must be non-empty", rule_id))
        if rule_id in seen:
            findings.append(Finding("error", "rule.duplicate_id", "rule ids must be unique", rule_id))
        seen.add(rule_id)
        if rule.get("trigger_reason") not in NARROW_REASONS:
            findings.append(Finding("error", "rule.trigger_invalid", "trigger_reason is outside the vocabulary", rule_id))
        else:
            covered.add(rule["trigger_reason"])
        if rule.get("default_action") not in NARROW_ACTIONS:
            findings.append(Finding("error", "rule.action_invalid", "default_action is outside the vocabulary", rule_id))
        labels = rule.get("applies_to_labels")
        if not isinstance(labels, list) or not labels:
            findings.append(Finding("error", "rule.labels_empty", "rule must watch at least one label", rule_id))
        elif any(label not in LIFECYCLE_LABELS for label in labels):
            findings.append(Finding("error", "rule.labels_invalid", "applies_to_labels has an unknown label", rule_id))
        if not isinstance(rule.get("blocks_promotion"), bool):
            findings.append(Finding("error", "rule.blocks_invalid", "blocks_promotion must be a boolean", rule_id))

    for reason in NARROW_REASONS:
        if reason not in covered:
            findings.append(
                Finding("error", "rule.reason_uncovered", "narrow reason has no rule watching for it", reason)
            )
    return findings


def validate_packet_block(surface_id: str, packet: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    for field in ("packet_id", "packet_ref", "proof_index_ref"):
        if not is_str(packet.get(field)):
            findings.append(Finding("error", "packet.empty_field", f"qualification_packet.{field} must be non-empty", surface_id))
    if packet.get("slo_state") not in FRESHNESS_SLO_STATES:
        findings.append(Finding("error", "packet.state_invalid", "slo_state is outside the vocabulary", surface_id))
    slo = packet.get("freshness_slo")
    if not isinstance(slo, dict):
        findings.append(Finding("error", "packet.slo_missing", "qualification_packet must carry a freshness_slo block", surface_id))
        return findings
    if not is_str(slo.get("slo_register_ref")):
        findings.append(Finding("error", "packet.slo_register_missing", "freshness_slo.slo_register_ref must be non-empty", surface_id))
    target = slo.get("target_max_age_days")
    warn = slo.get("warn_within_days")
    if not (is_int(target) and target >= 1):
        findings.append(Finding("error", "packet.target_invalid", "target_max_age_days must be an integer >= 1", surface_id))
    if not (is_int(warn) and warn >= 0):
        findings.append(Finding("error", "packet.warn_invalid", "warn_within_days must be an integer >= 0", surface_id))
    if is_int(target) and is_int(warn) and warn > target:
        findings.append(Finding("error", "packet.slo_inconsistent", "warn_within_days may not exceed target_max_age_days", surface_id))
    # A present packet must be a real captured packet — absence is expressed by omitting the
    # whole packet, not by a degenerate block.
    if packet.get("slo_state") == "missing" or not has_capture(packet):
        findings.append(Finding("error", "packet.present_without_capture", "a present packet must carry a capture and evidence; omit the packet to express absence", surface_id))
    return findings


def validate_surface(surface: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    surface_id = str(surface.get("surface_id", "<surface>"))

    for field in (
        "surface_id",
        "title",
        "surface_ref",
        "surface_summary",
        "claim_ref",
        "source_ref",
        "rationale",
    ):
        if not is_str(surface.get(field)):
            findings.append(Finding("error", "surface.empty_field", f"surface {field} must be non-empty", surface_id))

    if surface.get("surface_kind") not in SURFACE_KINDS:
        findings.append(Finding("error", "surface.kind_invalid", "surface_kind is outside the vocabulary", surface_id))
    if not isinstance(surface.get("release_relevant"), bool):
        findings.append(Finding("error", "surface.release_relevant_invalid", "release_relevant must be a boolean", surface_id))

    claim_label = surface.get("claim_label")
    displayed = surface.get("displayed_label")
    state = surface.get("surface_state")
    reasons = reasons_of(surface)

    if claim_label not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "surface.claim_label_invalid", "claim_label is invalid", surface_id))
    if displayed not in LIFECYCLE_LABELS:
        findings.append(Finding("error", "surface.displayed_invalid", "displayed_label is invalid", surface_id))
    if state not in SURFACE_STATES:
        findings.append(Finding("error", "surface.state_invalid", "surface_state is invalid", surface_id))
    if any(reason not in NARROW_REASONS for reason in reasons):
        findings.append(Finding("error", "surface.reason_invalid", "active_narrow_reasons has an unknown reason", surface_id))

    # The ceiling: no surface may render a label wider than the public claim's label.
    if claim_label in LEVEL_RANK and displayed in LEVEL_RANK and LEVEL_RANK[displayed] > LEVEL_RANK[claim_label]:
        findings.append(Finding("error", "surface.displayed_wider_than_claim", "displayed_label is wider than the claim ceiling", surface_id))

    packet = packet_of(surface)
    if packet is not None:
        findings.extend(validate_packet_block(surface_id, packet))
    slo_state = packet.get("slo_state") if packet is not None else None

    qualified = state in QUALIFIED_STATES
    claim_holds = is_above_cutline(claim_label)

    # The absent-packet rule: a surface with no qualification packet must be narrowed, must
    # name qualification_packet_absent, and may never render qualified.
    if packet is None:
        if qualified:
            findings.append(Finding("error", "surface.qualified_without_packet", "a surface renders qualified while it has no stable qualification packet", surface_id))
        if "qualification_packet_absent" not in reasons:
            findings.append(Finding("error", "surface.absent_packet_without_reason", "a surface with no qualification packet must name qualification_packet_absent", surface_id))
    elif "qualification_packet_absent" in reasons:
        findings.append(Finding("error", "surface.packet_present_but_reason_absent", "a surface carrying a packet may not name qualification_packet_absent", surface_id))

    # A claim canonically below the cutline forces the surface to inherit the ceiling and
    # narrow.
    if claim_label in LEVEL_RANK and not claim_holds:
        if qualified:
            findings.append(Finding("error", "surface.qualified_on_narrowed_claim", "a surface renders qualified while the public claim label is below the cutline", surface_id))
        if "claim_label_narrowed" not in reasons:
            findings.append(Finding("error", "surface.claim_narrowed_without_reason", "a surface whose claim is narrowed must name claim_label_narrowed", surface_id))

    if state in NARROWING_STATES:
        if is_above_cutline(displayed):
            findings.append(Finding("error", "surface.displayed_not_narrowed", "a surface that is not qualified must narrow below the cutline", surface_id))
        if not reasons:
            findings.append(Finding("error", "surface.narrowing_without_reason", "a narrowing surface must name an active narrow reason", surface_id))
        if slo_state == "breached" and "qualification_packet_breached" not in reasons:
            findings.append(Finding("error", "surface.breached_without_reason", "a breached packet must name qualification_packet_breached", surface_id))
    if qualified:
        if claim_label != displayed:
            findings.append(Finding("error", "surface.qualified_label_not_equal_claim", "a qualified surface must render the public claim's canonical label", surface_id))
        if reasons:
            findings.append(Finding("error", "surface.qualified_with_active_reason", "a qualified surface must carry no active narrow reason", surface_id))
        if packet is not None:
            if not has_capture(packet):
                findings.append(Finding("error", "surface.qualified_without_fresh_packet", "a qualified surface must ride a captured, evidence-backed packet", surface_id))
            if slo_state in ("breached", "missing"):
                findings.append(Finding("error", "surface.qualified_on_stale_packet", "a qualified surface must ride a packet within its freshness SLO", surface_id))
        signoff = surface.get("owner_signoff", {})
        if not (isinstance(signoff, dict) and signoff.get("signed_off") is True and is_str(signoff.get("signed_at"))):
            findings.append(Finding("error", "surface.qualified_without_signoff", "a qualified surface must carry an owner sign-off with a date", surface_id))
    signoff = surface.get("owner_signoff", {})
    if not (isinstance(signoff, dict) and is_str(signoff.get("owner_ref"))):
        findings.append(Finding("error", "surface.signoff_owner_missing", "owner_signoff.owner_ref must be non-empty", surface_id))

    findings.extend(validate_state_reason_coherence(surface_id, state, reasons, surface))
    findings.extend(validate_surface_refs(surface_id, surface, repo_root))
    return findings


def validate_state_reason_coherence(surface_id: str, state: Any, reasons: list[str], surface: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    if state == "narrowed_no_packet":
        if "qualification_packet_absent" not in reasons:
            findings.append(Finding("error", "surface.state_reason_incoherent", "narrowed_no_packet requires the qualification_packet_absent reason", surface_id))
        if has_packet(surface):
            findings.append(Finding("error", "surface.no_packet_state_with_packet", "narrowed_no_packet state must carry no qualification packet", surface_id))
    if state == "narrowed_incomplete":
        allowed = {"surface_capability_absent", "surface_evidence_incomplete", "owner_signoff_missing"}
        if not allowed.intersection(reasons):
            findings.append(Finding("error", "surface.state_reason_incoherent", "narrowed_incomplete requires a capability/evidence/signoff reason", surface_id))
    if state == "narrowed_stale" and "qualification_packet_breached" not in reasons:
        findings.append(Finding("error", "surface.state_reason_incoherent", "narrowed_stale requires the qualification_packet_breached reason", surface_id))
    if state == "narrowed_claim_narrowed" and "claim_label_narrowed" not in reasons:
        findings.append(Finding("error", "surface.state_reason_incoherent", "narrowed_claim_narrowed requires the claim_label_narrowed reason", surface_id))
    if state == "narrowed_waiver_expired":
        if "waiver_expired" not in reasons:
            findings.append(Finding("error", "surface.state_reason_incoherent", "narrowed_waiver_expired requires the waiver_expired reason", surface_id))
        if not isinstance(surface.get("waiver"), dict):
            findings.append(Finding("error", "surface.waiver_state_without_waiver", "narrowed_waiver_expired state must name a waiver", surface_id))
    if state == "qualified_on_waiver":
        waiver = surface.get("waiver")
        if not isinstance(waiver, dict) or not is_str(waiver.get("waiver_ref")) or not is_str(waiver.get("expires_at")):
            findings.append(Finding("error", "surface.waiver_state_without_waiver", "qualified_on_waiver state must name a waiver", surface_id))
    return findings


def validate_surface_refs(surface_id: str, surface: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Path-existence checks for the source ref and the packet refs (not the cross-artifact
    ids, which resolve against the claim manifest)."""
    findings: list[Finding] = []
    refs: list[str] = []
    source = surface.get("source_ref")
    if is_str(source):
        refs.append(source.split("#", 1)[0])
    packet = packet_of(surface)
    if packet is not None:
        for key in ("packet_ref", "proof_index_ref"):
            ref = packet.get(key)
            if is_str(ref):
                refs.append(ref.split("#", 1)[0])
        slo = packet.get("freshness_slo")
        if isinstance(slo, dict) and is_str(slo.get("slo_register_ref")):
            refs.append(slo["slo_register_ref"].split("#", 1)[0])
        for ref in packet.get("evidence_refs", []) or []:
            if is_str(ref):
                refs.append(ref.split("#", 1)[0])
    for ref in refs:
        if not (repo_root / ref).exists():
            findings.append(Finding("error", "surface.ref_missing", f"referenced artifact does not exist: {ref}", surface_id))
    return findings


def validate_coverage(register: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    surfaces = register.get("surfaces", [])

    seen: set[str] = set()
    for surface in surfaces:
        ref = surface.get("surface_ref")
        if not is_str(ref):
            continue
        if ref in seen:
            findings.append(Finding("error", "coverage.surface_duplicate", f"surface ref appears more than once: {ref}", ref))
        seen.add(ref)

    declared = register.get("release_relevant_surface_refs", [])
    declared_set = {r for r in declared if is_str(r)} if isinstance(declared, list) else set()
    covered = {
        str(surface.get("surface_ref"))
        for surface in surfaces
        if surface.get("release_relevant") is True and is_str(surface.get("surface_ref"))
    }
    for ref in sorted(declared_set):
        if ref not in covered:
            findings.append(Finding("error", "coverage.release_relevant_uncovered", f"declared release-relevant surface has no covering row: {ref}", ref))
    for surface in surfaces:
        if surface.get("release_relevant") is True and is_str(surface.get("surface_ref")) and surface["surface_ref"] not in declared_set:
            findings.append(Finding("error", "coverage.release_relevant_not_declared", "a release-relevant surface is not in release_relevant_surface_refs", str(surface.get("surface_id"))))

    present_kinds = {surface.get("surface_kind") for surface in surfaces}
    for kind in SURFACE_KINDS:
        if kind not in present_kinds:
            findings.append(Finding("error", "coverage.surface_kind_absent", f"surface kind has no row: {kind}", kind))
    return findings


def validate_freshness(register: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(register.get("as_of"))
    if as_of is None:
        return findings
    for surface in register.get("surfaces", []):
        surface_id = str(surface.get("surface_id", "<surface>"))
        packet = packet_of(surface)
        if packet is None:
            continue
        declared = packet.get("slo_state")
        computed = computed_slo_state(packet, as_of)
        if computed is None or declared not in FRESHNESS_RANK or computed not in FRESHNESS_RANK:
            continue
        if FRESHNESS_RANK[declared] > FRESHNESS_RANK[computed]:
            findings.append(
                Finding("error", "surface.packet_freshness_overstated", f"declared slo_state {declared!r} is fresher than the {computed!r} the as_of date allows", surface_id)
            )
        if surface.get("surface_state") in QUALIFIED_STATES and computed in ("breached", "missing"):
            findings.append(
                Finding("error", "surface.qualified_on_breached_packet", "a qualified surface rides a packet past its SLO against as_of", surface_id)
            )
    return findings


def validate_waivers(register: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    as_of = parse_date(register.get("as_of"))
    if as_of is None:
        return findings
    for surface in register.get("surfaces", []):
        surface_id = str(surface.get("surface_id", "<surface>"))
        state = surface.get("surface_state")
        waiver = surface.get("waiver")
        if not isinstance(waiver, dict):
            continue
        expires = parse_date(waiver.get("expires_at"))
        if expires is None:
            findings.append(Finding("error", "surface.waiver_expiry_invalid", "waiver expires_at must be an ISO date", surface_id))
            continue
        expired = as_of >= expires
        if expired and state == "qualified_on_waiver":
            findings.append(Finding("error", "surface.qualified_on_expired_waiver", "a surface renders qualified on a waiver that has expired against as_of", surface_id))
        if not expired and state == "narrowed_waiver_expired":
            findings.append(Finding("error", "surface.waiver_expired_but_active", "a surface is marked waiver-expired but the waiver is still active against as_of", surface_id))
    return findings


def validate_ceiling(register: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Cross-artifact ingestion: read the stable claim manifest and fail when a surface's
    claim_label is not the label the claim manifest publishes for the entry named by
    claim_ref, or names an entry the claim manifest does not carry."""
    findings: list[Finding] = []
    ref = register.get("claim_manifest_ref")
    if not is_str(ref):
        return findings
    path = repo_root / ref.split("#", 1)[0]
    if not path.exists():
        findings.append(Finding("error", "ceiling.artifact_missing", f"claim_manifest_ref does not exist: {ref}", "<register>"))
        return findings
    try:
        claim_manifest = json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        findings.append(Finding("error", "ceiling.artifact_invalid", f"claim_manifest_ref is not valid JSON: {exc}", ref))
        return findings

    published_by_entry = {}
    if isinstance(claim_manifest, dict):
        published_by_entry = {
            str(e.get("entry_id")): e.get("published_label")
            for e in claim_manifest.get("entries", [])
            if isinstance(e, dict)
        }

    for surface in register.get("surfaces", []):
        surface_id = str(surface.get("surface_id", "<surface>"))
        claim_ref = surface.get("claim_ref")
        if not is_str(claim_ref):
            continue
        if claim_ref not in published_by_entry:
            findings.append(Finding("error", "ceiling.claim_entry_unknown", f"claim_ref is not an entry in the stable claim manifest: {claim_ref}", surface_id))
            continue
        canonical = published_by_entry[claim_ref]
        if surface.get("claim_label") != canonical:
            findings.append(Finding("error", "ceiling.claim_label_mismatch", f"claim_label {surface.get('claim_label')!r} does not match the claim manifest published_label {canonical!r}", surface_id))
    return findings


def validate_publication(register: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    publication = register.get("publication")
    if not isinstance(publication, dict):
        findings.append(Finding("error", "publication.missing", "register must carry a publication block", "<register>"))
        return findings
    for field in ("publication_gate", "rationale"):
        if not is_str(publication.get(field)):
            findings.append(Finding("error", "publication.empty_field", f"publication.{field} must be non-empty", "<register>"))
    decision, rule_ids, surface_ids = computed_publication(register)
    if publication.get("decision") != decision:
        findings.append(
            Finding("error", "publication.decision_inconsistent", f"declared decision {publication.get('decision')!r} disagrees with computed {decision!r}", "<register>")
        )
    if list(publication.get("blocking_rule_ids", [])) != rule_ids:
        findings.append(Finding("error", "publication.blocking_rules_mismatch", "blocking_rule_ids disagrees with the firing stop rules", "<register>"))
    if list(publication.get("blocking_surface_ids", [])) != surface_ids:
        findings.append(Finding("error", "publication.blocking_surfaces_mismatch", "blocking_surface_ids disagrees with the firing stop rules", "<register>"))
    return findings


def validate_summary(register: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    summary = register.get("summary")
    if not isinstance(summary, dict):
        findings.append(Finding("error", "summary.missing", "register must carry a summary block", "<register>"))
        return findings
    expected = computed_summary(register)
    for key, value in expected.items():
        if summary.get(key) != value:
            findings.append(Finding("error", "summary.count_mismatch", f"summary.{key} must equal the recomputed value", key))
    return findings


def validate_register(register: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings = validate_envelope(register, repo_root)
    findings.extend(validate_rules(register))

    surfaces = register.get("surfaces")
    if not isinstance(surfaces, list) or not surfaces:
        findings.append(Finding("error", "register.surfaces_empty", "register must enumerate at least one surface", "<register>"))
        return findings

    seen: set[str] = set()
    for raw in surfaces:
        surface = ensure_dict(raw, "register.surfaces[]")
        findings.extend(validate_surface(surface, repo_root))
        surface_id = str(surface.get("surface_id", "<surface>"))
        if surface_id in seen:
            findings.append(Finding("error", "surface.duplicate_id", "surface ids must be unique", surface_id))
        seen.add(surface_id)

    findings.extend(validate_coverage(register))
    findings.extend(validate_freshness(register))
    findings.extend(validate_waivers(register))
    findings.extend(validate_ceiling(register, repo_root))
    findings.extend(validate_publication(register))
    findings.extend(validate_summary(register))
    return findings


def recompute_derived(register: dict[str, Any]) -> None:
    register["summary"] = computed_summary(register)
    decision, rule_ids, surface_ids = computed_publication(register)
    if isinstance(register.get("publication"), dict):
        register["publication"].update(
            {"decision": decision, "blocking_rule_ids": rule_ids, "blocking_surface_ids": surface_ids}
        )


def run_negative_drills(register: dict[str, Any], repo_root: Path) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []

    def record(drill_id: str, expected_check_id: str, fired: bool) -> None:
        results.append({"drill_id": drill_id, "expected_check_id": expected_check_id, "status": "passed" if fired else "failed"})
        if not fired:
            findings.append(Finding("error", "negative_drill.not_rejected", f"negative drill {drill_id} did not fire", drill_id))

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        return {f.check_id for f in validate_register(candidate, repo_root)}

    # A surface lacking a packet that is rendered qualified must be flagged.
    mutated = copy.deepcopy(register)
    target = next((s for s in mutated["surfaces"] if not has_packet(s)), None)
    if target is not None:
        target["surface_state"] = "qualified_stable"
        target["displayed_label"] = target["claim_label"]
        target["active_narrow_reasons"] = []
        recompute_derived(mutated)
        record("qualified_without_packet_rejected", "surface.qualified_without_packet", "surface.qualified_without_packet" in check_ids(mutated))

    # A narrowing surface that still renders a stable label must be flagged.
    mutated = copy.deepcopy(register)
    target = next((s for s in mutated["surfaces"] if s.get("surface_state") in NARROWING_STATES), None)
    if target is not None:
        target["displayed_label"] = "stable"
        recompute_derived(mutated)
        record("displayed_not_narrowed_rejected", "surface.displayed_not_narrowed", "surface.displayed_not_narrowed" in check_ids(mutated))

    # A qualified surface whose packet is breached against as_of must be flagged.
    mutated = copy.deepcopy(register)
    target = next((s for s in mutated["surfaces"] if s.get("surface_state") in QUALIFIED_STATES and has_packet(s)), None)
    if target is not None:
        target["qualification_packet"]["captured_at"] = "2000-01-01"
        record("qualified_on_breached_packet_rejected", "surface.qualified_on_breached_packet", "surface.qualified_on_breached_packet" in check_ids(mutated))

    # A surface whose claim_label disagrees with the claim manifest must be flagged.
    mutated = copy.deepcopy(register)
    target = next((s for s in mutated["surfaces"] if s.get("claim_label") == "beta"), None)
    if target is not None:
        target["claim_label"] = "stable"
        record("claim_label_mismatch_rejected", "ceiling.claim_label_mismatch", "ceiling.claim_label_mismatch" in check_ids(mutated))

    # A surface rendered wider than its public claim's ceiling must be flagged.
    mutated = copy.deepcopy(register)
    target = next(
        (s for s in mutated["surfaces"] if s.get("claim_label") == "beta" and not is_above_cutline(s.get("displayed_label"))),
        None,
    )
    if target is not None:
        target["displayed_label"] = "lts"
        recompute_derived(mutated)
        record("displayed_wider_than_claim_rejected", "surface.displayed_wider_than_claim", "surface.displayed_wider_than_claim" in check_ids(mutated))

    # Dropping a declared release-relevant surface's row must be flagged.
    mutated = copy.deepcopy(register)
    declared = mutated.get("release_relevant_surface_refs", [])
    if declared:
        dropped = declared[0]
        mutated["surfaces"] = [s for s in mutated["surfaces"] if s.get("surface_ref") != dropped]
        recompute_derived(mutated)
        record("release_relevant_uncovered_rejected", "coverage.release_relevant_uncovered", "coverage.release_relevant_uncovered" in check_ids(mutated))

    # Removing every surface of a kind must be flagged.
    mutated = copy.deepcopy(register)
    mutated["surfaces"] = [s for s in mutated["surfaces"] if s.get("surface_kind") != "experimental_preview"]
    recompute_derived(mutated)
    record("surface_kind_absent_rejected", "coverage.surface_kind_absent", "coverage.surface_kind_absent" in check_ids(mutated))

    # A publication decision that disagrees with the firing rules must be flagged.
    mutated = copy.deepcopy(register)
    mutated["publication"]["decision"] = "proceed" if mutated["publication"].get("decision") == "hold" else "hold"
    record("publication_decision_inconsistent_rejected", "publication.decision_inconsistent", "publication.decision_inconsistent" in check_ids(mutated))

    return results, findings


def run_fixture_cases(repo_root: Path, fixtures_dir: str) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []
    manifest_path = repo_root / fixtures_dir / "cases.json"
    if not manifest_path.exists():
        return results, findings
    try:
        manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        findings.append(Finding("error", "fixture.manifest_invalid", f"fixture manifest is not valid JSON: {exc}", str(manifest_path)))
        return results, findings

    for case in manifest.get("cases", []):
        case_id = str(case.get("case_id", "<case>"))
        rel = case.get("file")
        expected = case.get("expected_check_id")
        outcome = case.get("expected_outcome", "rejected")
        case_path = repo_root / fixtures_dir / rel if is_str(rel) else None
        if case_path is None or not case_path.exists():
            findings.append(Finding("error", "fixture.missing", f"fixture file does not exist: {rel}", case_id))
            continue
        try:
            candidate = json.loads(case_path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            findings.append(Finding("error", "fixture.invalid_json", f"fixture is not valid JSON: {exc}", case_id))
            continue
        ids = {f.check_id for f in validate_register(ensure_dict(candidate, "fixture"), repo_root)}
        if outcome == "rejected":
            fired = (expected in ids) if is_str(expected) else bool(ids)
            results.append({"case_id": case_id, "expected_check_id": expected or "<any>", "status": "passed" if fired else "failed"})
            if not fired:
                findings.append(Finding("error", "fixture.not_rejected", f"fixture case {case_id} should be rejected with {expected!r} but was not", case_id))
        else:
            results.append({"case_id": case_id, "expected_check_id": "<none>", "status": "passed" if not ids else "failed"})
            if ids:
                findings.append(Finding("error", "fixture.unexpected_rejection", f"fixture case {case_id} should validate clean but was rejected", case_id))
    return results, findings


def write_report(
    path: Path,
    register: dict[str, Any],
    findings: list[Finding],
    drill_results: list[dict[str, str]],
    fixture_results: list[dict[str, str]],
) -> None:
    decision, rule_ids, surface_ids = computed_publication(register)
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "optional_surface_qualification_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "register_id": register.get("register_id"),
        "as_of": register.get("as_of"),
        "summary": register.get("summary"),
        "publication": {
            "decision": decision,
            "blocking_rule_ids": rule_ids,
            "blocking_surface_ids": surface_ids,
        },
        "negative_drills": drill_results,
        "fixture_cases": fixture_results,
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(json_text(payload), encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    register = ensure_dict(
        load_json(repo_root / args.register, "optional-surface qualification register"),
        "optional-surface qualification register",
    )

    findings = validate_register(register, repo_root)
    drill_results, drill_findings = run_negative_drills(register, repo_root)
    findings.extend(drill_findings)
    fixture_results, fixture_findings = run_fixture_cases(repo_root, args.fixtures_dir)
    findings.extend(fixture_findings)

    report_rel = args.report
    if report_rel is None and not args.no_capture:
        report_rel = DEFAULT_REPORT_REL
    if report_rel:
        write_report(repo_root / report_rel, register, findings, drill_results, fixture_results)

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            print(f"ERROR [{item.check_id}] ({item.ref}): {item.message}", file=sys.stderr)
        return 1

    summary = register.get("summary", {})
    decision, rule_ids, _surface_ids = computed_publication(register)
    print(
        "optional-surface qualification register validated "
        f"({summary.get('total_surfaces')} surfaces across {summary.get('total_claims')} claims, "
        f"{summary.get('surfaces_qualified_stable')} qualified stable, "
        f"{summary.get('surfaces_narrowed_below_cutline')} narrowed; "
        f"{summary.get('surfaces_without_packet')} lack a qualification packet; "
        f"kinds opt_in={summary.get('opt_in_capability_surfaces')} integration={summary.get('optional_integration_surfaces')} "
        f"platform={summary.get('secondary_platform_surfaces')} preview={summary.get('experimental_preview_surfaces')}; "
        f"release-relevant {summary.get('release_relevant_qualified')}/{summary.get('release_relevant_total')} qualified; "
        f"packets {summary.get('packets_current')} current / "
        f"{summary.get('packets_due_for_refresh')} due / "
        f"{summary.get('packets_breached')} breached; "
        f"{summary.get('stop_rules_firing')} rules firing; "
        f"publication={decision}; {len(drill_results)} negative drills, {len(fixture_results)} fixture cases)"
    )

    if args.require_proceed and decision == "hold":
        print(
            f"PUBLICATION HELD: optional-surface promotion blocked by {len(rule_ids)} rule(s): {', '.join(rule_ids)}",
            file=sys.stderr,
        )
        return 2

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
