#!/usr/bin/env python3
"""Validate the v1.0 support-class ledger, certified-archetype manifest, and
downgrade automation.

Public-facing support wording regresses when the decision to call a surface
"Certified" or "Supported" lives in prose, side spreadsheets, or optimistic
badges. This gate makes that decision explicit, enforceable, and *ingested* from
the stable claim matrix. It reads the checked-in ledger at
``artifacts/release/support_class_ledger.json`` and:

  - asserts the closed vocabularies (support classes, ledger states, downgrade
    reasons, actions, evidence paths) and the certified cutline are canonical;
  - asserts every entry put forward as Certified references a certified-archetype
    manifest entry, and that a published Certified claim points at a certified
    (not decertified) archetype;
  - asserts every entry that is not qualified, has stale evidence, or relied on
    an expired waiver narrows strictly below its claimed class rather than
    inheriting a stronger neighbour, and that a published class carries current,
    proof-backed, owner-signed evidence with no active downgrade reason;
  - performs the downgrade automation the typed model cannot:
      * the cross-artifact backing-claim check — for each entry that names a
        ``backing_stable_claim_ref`` it reads the stable claim matrix and fails
        when the entry still publishes a class while its backing stable claim is
        narrowed below the stable cutline, or narrows on a backing claim that
        still holds, or names a backing claim the matrix does not carry; and
      * date arithmetic — against the ledger ``as_of`` date it recomputes waiver
        expiry and evidence/archetype staleness and fails when an entry
        overstates its posture;
  - recomputes the publication verdict from the entries and downgrade rules and
    fails when the declared decision or blocking sets drift;
  - recomputes the summary block;
  - runs negative drills proving the narrowing, certified-linkage, waiver-expiry,
    backing-claim, and publication rejections all fire; and
  - runs the checked-in fixture cases under
    ``fixtures/release/support_class_ledger`` and fails when a case the manifest
    marks as rejected validates clean (or trips the wrong check).

With ``--require-proceed`` the gate additionally fails (exit code 2) when the
recomputed publication verdict is ``hold``, so shiproom and release tooling can
block v1.0 support publication directly from this artifact.

The typed Rust consumer
(``aureline_release::support_class_ledger::current_support_class_ledger``) reads
the same ledger and runs the same structural cross-check, so this gate and
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


DEFAULT_LEDGER_REL = "artifacts/release/support_class_ledger.json"
DEFAULT_REPORT_REL = "artifacts/release/captures/support_class_ledger_validation_capture.json"
DEFAULT_FIXTURES_REL = "fixtures/release/support_class_ledger"

EXPECTED_SCHEMA_VERSION = 1
LEDGER_RECORD_KIND = "support_class_ledger"

SUPPORT_CLASSES = (
    "certified",
    "supported",
    "community",
    "experimental",
    "not_certified_in_this_mode",
    "not_configured",
    "disabled_by_policy",
    "not_supported",
)
POSITIVE_CLASSES = ("certified", "supported", "community", "experimental")
REFUSAL_CLASSES = (
    "not_certified_in_this_mode",
    "not_configured",
    "disabled_by_policy",
    "not_supported",
)
LEDGER_STATES = (
    "published",
    "provisional_on_waiver",
    "narrowed_unqualified",
    "narrowed_stale",
    "narrowed_waiver_expired",
)
DOWNGRADE_REASONS = (
    "certified_archetype_unmanifested",
    "certified_archetype_evidence_stale",
    "certified_archetype_decertified",
    "support_evidence_missing",
    "support_evidence_stale",
    "backing_stable_claim_narrowed",
    "waiver_expired",
    "owner_signoff_missing",
)
DOWNGRADE_ACTIONS = (
    "narrow_published_class",
    "refresh_certified_archetype",
    "hold_publication",
    "request_owner_signoff",
    "route_to_supported_mode",
)
EVIDENCE_PATH_CLASSES = (
    "certified_archetype_report",
    "compatibility_report_supported",
    "community_evidence_or_partner_attestation",
    "experimental_runtime_observation_only",
    "no_evidence_required_refusal_state",
)
CERTIFICATION_STATUSES = ("certified", "decertified")
HOLDING_STATES = ("published", "provisional_on_waiver")
NARROWING_STATES = ("narrowed_unqualified", "narrowed_stale", "narrowed_waiver_expired")

CLASS_RANK = {
    "certified": 4,
    "supported": 3,
    "community": 2,
    "experimental": 1,
    "not_certified_in_this_mode": 0,
    "not_configured": 0,
    "disabled_by_policy": 0,
    "not_supported": 0,
}

# Stable claim matrix levels at or above the stable cutline.
STABLE_HOLDING_LEVELS = ("lts", "stable")


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
    parser.add_argument("--ledger", default=DEFAULT_LEDGER_REL)
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


def parse_date(value: Any) -> dt.date | None:
    if not isinstance(value, str):
        return None
    try:
        return dt.date.fromisoformat(value)
    except ValueError:
        return None


def is_positive(klass: Any) -> bool:
    return isinstance(klass, str) and CLASS_RANK.get(klass, -1) > 0


def rank(klass: Any) -> int:
    return CLASS_RANK.get(klass, -1) if isinstance(klass, str) else -1


def downgrade_rule_fires(rule: dict[str, Any], entries: list[dict[str, Any]]) -> bool:
    trigger = rule.get("trigger_reason")
    classes = rule.get("applies_to_classes", [])
    if not isinstance(classes, list):
        return False
    for entry in entries:
        if entry.get("claimed_class") in classes and trigger in entry.get(
            "active_downgrade_reasons", []
        ):
            return True
    return False


def computed_publication(ledger: dict[str, Any]) -> tuple[str, list[str], list[str]]:
    entries = ledger.get("entries", [])
    rules = ledger.get("downgrade_rules", [])
    blocking_rules = [
        rule
        for rule in rules
        if rule.get("blocks_publication") is True and downgrade_rule_fires(rule, entries)
    ]
    rule_ids = sorted(str(rule.get("rule_id")) for rule in blocking_rules)
    blocking_triggers = {rule.get("trigger_reason") for rule in blocking_rules}
    entry_ids: set[str] = set()
    for entry in entries:
        if is_positive(entry.get("claimed_class")) and blocking_triggers.intersection(
            entry.get("active_downgrade_reasons", [])
        ):
            entry_ids.add(str(entry.get("entry_id")))
    decision = "hold" if blocking_rules else "proceed"
    return decision, rule_ids, sorted(entry_ids)


def computed_summary(ledger: dict[str, Any]) -> dict[str, int]:
    entries = ledger.get("entries", [])
    rules = ledger.get("downgrade_rules", [])
    archetypes = ledger.get("certified_archetypes", [])
    holding = [e for e in entries if e.get("ledger_state") in HOLDING_STATES]
    return {
        "total_entries": len(entries),
        "entries_published_as_claimed": len(holding),
        "entries_narrowed": len(entries) - len(holding),
        "entries_on_active_waiver": sum(
            1 for e in entries if e.get("ledger_state") == "provisional_on_waiver"
        ),
        "certified_entries": sum(1 for e in entries if e.get("effective_class") == "certified"),
        "total_active_downgrade_reasons": sum(
            len(e.get("active_downgrade_reasons", []))
            for e in entries
            if isinstance(e.get("active_downgrade_reasons"), list)
        ),
        "downgrade_rules_firing": sum(
            1 for rule in rules if downgrade_rule_fires(rule, entries)
        ),
        "certified_archetypes_total": len(archetypes),
        "certified_archetypes_decertified": sum(
            1 for a in archetypes if a.get("certification_status") == "decertified"
        ),
    }


def validate_envelope(ledger: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    ledger_id = str(ledger.get("ledger_id", "<ledger>"))

    if ledger.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(Finding("error", "ledger.schema_version", "schema_version must be 1", ledger_id))
    if ledger.get("record_kind") != LEDGER_RECORD_KIND:
        findings.append(Finding("error", "ledger.record_kind", "record_kind is not supported", ledger_id))
    for field in (
        "ledger_id",
        "status",
        "overview_page",
        "as_of",
        "release_train",
        "claim_matrix_ref",
        "taxonomy_ref",
    ):
        if not is_str(ledger.get(field)):
            findings.append(
                Finding("error", "ledger.empty_field", f"{field} must be a non-empty string", ledger_id)
            )
    if parse_date(ledger.get("as_of")) is None:
        findings.append(Finding("error", "ledger.as_of_invalid", "as_of must be an ISO date", ledger_id))

    for key, expected in (
        ("support_classes", list(SUPPORT_CLASSES)),
        ("ledger_states", list(LEDGER_STATES)),
        ("downgrade_reasons", list(DOWNGRADE_REASONS)),
        ("downgrade_actions", list(DOWNGRADE_ACTIONS)),
        ("evidence_path_classes", list(EVIDENCE_PATH_CLASSES)),
    ):
        if list(ledger.get(key, [])) != expected:
            findings.append(
                Finding("error", "ledger.vocabulary", f"ledger.{key} is not the closed vocabulary", key)
            )

    cutline = ledger.get("certified_cutline")
    if not isinstance(cutline, dict):
        findings.append(Finding("error", "cutline.missing", "ledger must carry a certified_cutline", ledger_id))
    else:
        if cutline.get("cutline_class") != "certified":
            findings.append(Finding("error", "cutline.class", "cutline_class must be certified", ledger_id))
        if list(cutline.get("positive_classes", [])) != list(POSITIVE_CLASSES):
            findings.append(Finding("error", "cutline.positive", "positive_classes is not canonical", ledger_id))
        if list(cutline.get("refusal_classes", [])) != list(REFUSAL_CLASSES):
            findings.append(Finding("error", "cutline.refusal", "refusal_classes is not canonical", ledger_id))
        if not is_str(cutline.get("description")):
            findings.append(Finding("error", "cutline.description", "cutline description must be non-empty", ledger_id))
    return findings


def validate_archetypes(ledger: dict[str, Any], repo_root: Path) -> tuple[list[Finding], dict[str, dict]]:
    findings: list[Finding] = []
    archetypes = ledger.get("certified_archetypes")
    by_id: dict[str, dict] = {}
    if not isinstance(archetypes, list) or not archetypes:
        findings.append(
            Finding("error", "archetypes.empty", "ledger must enumerate at least one certified archetype", "<ledger>")
        )
        return findings, by_id

    as_of = parse_date(ledger.get("as_of"))
    seen: set[str] = set()
    for archetype in archetypes:
        archetype = ensure_dict(archetype, "certified_archetypes[]")
        archetype_id = str(archetype.get("archetype_id", "<archetype>"))
        by_id[archetype_id] = archetype
        for field in (
            "archetype_id",
            "title",
            "client_class",
            "os_family",
            "deployment_mode",
            "locality_mode",
            "scope_summary",
            "rationale",
        ):
            if not is_str(archetype.get(field)):
                findings.append(
                    Finding("error", "archetype.empty_field", f"archetype {field} must be non-empty", archetype_id)
                )
        if archetype_id in seen:
            findings.append(Finding("error", "archetype.duplicate_id", "archetype ids must be unique", archetype_id))
        seen.add(archetype_id)
        status = archetype.get("certification_status")
        if status not in CERTIFICATION_STATUSES:
            findings.append(
                Finding("error", "archetype.status_invalid", "certification_status is outside the vocabulary", archetype_id)
            )
        cert = archetype.get("certification")
        if not isinstance(cert, dict):
            findings.append(Finding("error", "archetype.certification_missing", "archetype must carry a certification block", archetype_id))
            continue
        if not is_str(cert.get("report_ref")):
            findings.append(Finding("error", "archetype.report_ref_missing", "certification.report_ref must be non-empty", archetype_id))
        elif not (repo_root / cert["report_ref"].split("#", 1)[0]).exists():
            findings.append(
                Finding("error", "archetype.report_ref_missing_file", f"certified-archetype report does not exist: {cert['report_ref']}", archetype_id)
            )
        if not is_str(cert.get("owner_ref")):
            findings.append(Finding("error", "archetype.owner_missing", "certification.owner_ref must be non-empty", archetype_id))
        window = cert.get("freshness_window_days")
        if not isinstance(window, int) or isinstance(window, bool) or window < 1:
            findings.append(Finding("error", "archetype.window_invalid", "freshness_window_days must be an integer >= 1", archetype_id))
        if status == "certified":
            if not (cert.get("signed_off") is True and is_str(cert.get("signed_at")) and is_str(cert.get("captured_at"))):
                findings.append(
                    Finding("error", "archetype.certified_without_signoff", "a certified archetype must be captured and owner-signed", archetype_id)
                )
            captured = parse_date(cert.get("captured_at"))
            if as_of is not None and captured is not None and isinstance(window, int) and not isinstance(window, bool):
                if as_of > captured + dt.timedelta(days=window):
                    findings.append(
                        Finding("error", "archetype.certified_but_stale", "a certified archetype rides a report past its freshness window against as_of", archetype_id)
                    )
    return findings, by_id


def validate_rules(ledger: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    rules = ledger.get("downgrade_rules")
    if not isinstance(rules, list) or not rules:
        findings.append(Finding("error", "rules.empty", "ledger must enumerate at least one downgrade rule", "<ledger>"))
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
        if rule.get("trigger_reason") not in DOWNGRADE_REASONS:
            findings.append(Finding("error", "rule.trigger_invalid", "trigger_reason is outside the vocabulary", rule_id))
        else:
            covered.add(rule["trigger_reason"])
        if rule.get("default_action") not in DOWNGRADE_ACTIONS:
            findings.append(Finding("error", "rule.action_invalid", "default_action is outside the vocabulary", rule_id))
        classes = rule.get("applies_to_classes")
        if not isinstance(classes, list) or not classes:
            findings.append(Finding("error", "rule.classes_empty", "rule must watch at least one class", rule_id))
        elif any(klass not in SUPPORT_CLASSES for klass in classes):
            findings.append(Finding("error", "rule.classes_invalid", "applies_to_classes has an unknown class", rule_id))
        if not isinstance(rule.get("blocks_publication"), bool):
            findings.append(Finding("error", "rule.blocks_invalid", "blocks_publication must be a boolean", rule_id))

    for reason in DOWNGRADE_REASONS:
        if reason not in covered:
            findings.append(
                Finding("error", "rule.reason_uncovered", "downgrade reason has no rule watching for it", reason)
            )
    return findings


def validate_entry(entry: dict[str, Any], archetypes: dict[str, dict], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    entry_id = str(entry.get("entry_id", "<entry>"))

    for field in ("entry_id", "title", "subject_family", "rationale"):
        if not is_str(entry.get(field)):
            findings.append(Finding("error", "entry.empty_field", f"entry {field} must be non-empty", entry_id))

    claimed = entry.get("claimed_class")
    effective = entry.get("effective_class")
    state = entry.get("ledger_state")
    reasons = entry.get("active_downgrade_reasons", [])
    if not isinstance(reasons, list):
        reasons = []

    if claimed not in SUPPORT_CLASSES:
        findings.append(Finding("error", "entry.claimed_invalid", "claimed_class is invalid", entry_id))
    elif not is_positive(claimed):
        findings.append(Finding("error", "entry.claimed_not_positive", "claimed_class must be a positive support class", entry_id))
    if effective not in SUPPORT_CLASSES:
        findings.append(Finding("error", "entry.effective_invalid", "effective_class is invalid", entry_id))
    if state not in LEDGER_STATES:
        findings.append(Finding("error", "entry.state_invalid", "ledger_state is invalid", entry_id))
    if any(reason not in DOWNGRADE_REASONS for reason in reasons):
        findings.append(Finding("error", "entry.reason_invalid", "active_downgrade_reasons has an unknown reason", entry_id))

    if claimed in CLASS_RANK and effective in CLASS_RANK and rank(effective) > rank(claimed):
        findings.append(Finding("error", "entry.effective_wider_than_claimed", "effective_class is wider than claimed_class", entry_id))

    evidence = entry.get("evidence")
    if not isinstance(evidence, dict):
        findings.append(Finding("error", "entry.evidence_missing", "entry must carry an evidence block", entry_id))
        evidence = {}
    if not is_str(evidence.get("proof_index_ref")):
        findings.append(Finding("error", "entry.proof_index_missing", "evidence.proof_index_ref must be non-empty", entry_id))
    if evidence.get("required_evidence_path") not in EVIDENCE_PATH_CLASSES:
        findings.append(Finding("error", "entry.evidence_path_invalid", "required_evidence_path is outside the vocabulary", entry_id))
    window = evidence.get("freshness_window_days")
    if not isinstance(window, int) or isinstance(window, bool) or window < 1:
        findings.append(Finding("error", "entry.window_invalid", "freshness_window_days must be an integer >= 1", entry_id))

    holds = state in HOLDING_STATES

    # Acceptance core: a narrowing state must drop strictly below the claimed
    # class and name a reason; a published class must be clean, proof-backed,
    # and owner-signed.
    if state in NARROWING_STATES:
        if claimed in CLASS_RANK and effective in CLASS_RANK and rank(effective) >= rank(claimed):
            findings.append(
                Finding("error", "entry.effective_not_narrowed", "an entry that is not qualified must narrow below its claimed class", entry_id)
            )
        if not reasons:
            findings.append(Finding("error", "entry.narrowing_without_reason", "a narrowing entry must name an active downgrade reason", entry_id))
    if holds:
        if claimed != effective:
            findings.append(Finding("error", "entry.held_class_not_equal_claimed", "a published entry must publish its claimed class", entry_id))
        if reasons:
            findings.append(Finding("error", "entry.held_with_active_downgrade", "a published entry must carry no active downgrade reason", entry_id))
        if not evidence.get("evidence_refs"):
            findings.append(Finding("error", "entry.held_without_evidence", "a published entry must name qualification evidence", entry_id))
        signoff = entry.get("owner_signoff", {})
        if not (isinstance(signoff, dict) and signoff.get("signed_off") is True and is_str(signoff.get("signed_at"))):
            findings.append(Finding("error", "entry.held_without_signoff", "a published entry must carry an owner sign-off with a date", entry_id))

    findings.extend(validate_certified_linkage(entry_id, entry, archetypes))
    findings.extend(validate_state_reason_coherence(entry_id, state, reasons, entry))
    findings.extend(validate_entry_refs(entry_id, evidence, repo_root))
    return findings


def validate_certified_linkage(entry_id: str, entry: dict[str, Any], archetypes: dict[str, dict]) -> list[Finding]:
    findings: list[Finding] = []
    if entry.get("claimed_class") != "certified":
        return findings
    archetype_ref = entry.get("certified_archetype_ref")
    if not is_str(archetype_ref):
        findings.append(
            Finding("error", "entry.certified_without_archetype_ref", "a Certified claim must reference a certified-archetype manifest entry", entry_id)
        )
        return findings
    archetype = archetypes.get(archetype_ref)
    if archetype is None:
        findings.append(
            Finding("error", "entry.certified_archetype_not_in_manifest", f"referenced archetype is not in the manifest: {archetype_ref}", entry_id)
        )
        return findings
    if entry.get("effective_class") == "certified" and archetype.get("certification_status") != "certified":
        findings.append(
            Finding("error", "entry.certified_on_decertified_archetype", f"a published Certified claim points at a decertified archetype: {archetype_ref}", entry_id)
        )
    return findings


def validate_state_reason_coherence(entry_id: str, state: Any, reasons: list[str], entry: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    if state == "narrowed_unqualified":
        allowed = {
            "support_evidence_missing",
            "certified_archetype_unmanifested",
            "certified_archetype_decertified",
            "backing_stable_claim_narrowed",
            "owner_signoff_missing",
        }
        if not allowed.intersection(reasons):
            findings.append(
                Finding("error", "entry.state_reason_incoherent", "narrowed_unqualified requires a qualification-gap reason", entry_id)
            )
    if state == "narrowed_stale" and not (
        "support_evidence_stale" in reasons or "certified_archetype_evidence_stale" in reasons
    ):
        findings.append(Finding("error", "entry.state_reason_incoherent", "narrowed_stale requires a staleness reason", entry_id))
    if state == "narrowed_waiver_expired":
        if "waiver_expired" not in reasons:
            findings.append(Finding("error", "entry.state_reason_incoherent", "narrowed_waiver_expired requires the waiver_expired reason", entry_id))
        if not isinstance(entry.get("waiver"), dict):
            findings.append(Finding("error", "entry.waiver_state_without_waiver", "narrowed_waiver_expired state must name a waiver", entry_id))
    if state == "provisional_on_waiver":
        waiver = entry.get("waiver")
        if not isinstance(waiver, dict) or not is_str(waiver.get("waiver_ref")) or not is_str(waiver.get("expires_at")):
            findings.append(Finding("error", "entry.waiver_state_without_waiver", "provisional_on_waiver state must name a waiver", entry_id))
    return findings


def validate_entry_refs(entry_id: str, evidence: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings: list[Finding] = []
    refs: list[str] = []
    proof_ref = evidence.get("proof_index_ref")
    if is_str(proof_ref):
        refs.append(proof_ref.split("#", 1)[0])
    for ref in evidence.get("evidence_refs", []) or []:
        if is_str(ref):
            refs.append(ref.split("#", 1)[0])
    for ref in refs:
        if not (repo_root / ref).exists():
            findings.append(Finding("error", "entry.ref_missing", f"referenced artifact does not exist: {ref}", entry_id))
    return findings


def validate_dates(ledger: dict[str, Any], archetypes: dict[str, dict]) -> list[Finding]:
    """Date arithmetic the typed model cannot do: against as_of, fail when an
    entry overstates its posture on an expired waiver, stale evidence, or a stale
    certified-archetype report."""
    findings: list[Finding] = []
    as_of = parse_date(ledger.get("as_of"))
    if as_of is None:
        return findings
    for entry in ledger.get("entries", []):
        entry_id = str(entry.get("entry_id", "<entry>"))
        state = entry.get("ledger_state")

        waiver = entry.get("waiver")
        if isinstance(waiver, dict):
            expires = parse_date(waiver.get("expires_at"))
            if expires is None:
                findings.append(Finding("error", "entry.waiver_expiry_invalid", "waiver expires_at must be an ISO date", entry_id))
            else:
                expired = as_of >= expires
                if expired and state == "provisional_on_waiver":
                    findings.append(
                        Finding("error", "entry.provisional_on_expired_waiver", "a provisional class relies on a waiver that has expired against as_of", entry_id)
                    )
                if not expired and state == "narrowed_waiver_expired":
                    findings.append(
                        Finding("error", "entry.waiver_expired_but_active", "an entry is marked waiver-expired but the waiver is still active against as_of", entry_id)
                    )

        evidence = entry.get("evidence", {})
        captured = parse_date(evidence.get("captured_at")) if isinstance(evidence, dict) else None
        window = evidence.get("freshness_window_days") if isinstance(evidence, dict) else None
        if captured is not None and isinstance(window, int) and not isinstance(window, bool):
            if as_of > captured + dt.timedelta(days=window) and state in HOLDING_STATES:
                findings.append(
                    Finding("error", "entry.evidence_stale_but_claimed", "a published class rides evidence past its freshness window against as_of", entry_id)
                )

        # A held Certified entry may not ride a stale certified-archetype report.
        if state in HOLDING_STATES and entry.get("effective_class") == "certified":
            archetype = archetypes.get(entry.get("certified_archetype_ref"))
            if isinstance(archetype, dict):
                cert = archetype.get("certification", {})
                cap = parse_date(cert.get("captured_at")) if isinstance(cert, dict) else None
                win = cert.get("freshness_window_days") if isinstance(cert, dict) else None
                if cap is not None and isinstance(win, int) and not isinstance(win, bool):
                    if as_of > cap + dt.timedelta(days=win):
                        findings.append(
                            Finding("error", "entry.certified_on_stale_archetype", "a published Certified class rides a certified-archetype report past its freshness window", entry_id)
                        )
    return findings


def validate_backing_claims(ledger: dict[str, Any], repo_root: Path) -> list[Finding]:
    """Cross-artifact downgrade automation: ingest the stable claim matrix and
    fail when an entry's posture disagrees with its backing stable claim."""
    findings: list[Finding] = []
    matrix_ref = ledger.get("claim_matrix_ref")
    if not is_str(matrix_ref):
        return findings
    matrix_path = repo_root / matrix_ref
    if not matrix_path.exists():
        findings.append(Finding("error", "backing.matrix_missing", f"claim_matrix_ref does not exist: {matrix_ref}", "<ledger>"))
        return findings
    try:
        matrix = json.loads(matrix_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        findings.append(Finding("error", "backing.matrix_invalid", f"stable claim matrix is not valid JSON: {exc}", matrix_ref))
        return findings

    rows = matrix.get("rows", []) if isinstance(matrix, dict) else []
    effective_by_claim = {
        str(row.get("claim_id")): row.get("effective_level")
        for row in rows
        if isinstance(row, dict)
    }

    for entry in ledger.get("entries", []):
        entry_id = str(entry.get("entry_id", "<entry>"))
        backing_ref = entry.get("backing_stable_claim_ref")
        if not is_str(backing_ref):
            continue
        if backing_ref not in effective_by_claim:
            findings.append(
                Finding("error", "backing.claim_unknown", f"backing_stable_claim_ref is not a row in the stable claim matrix: {backing_ref}", entry_id)
            )
            continue
        backing_holds = effective_by_claim[backing_ref] in STABLE_HOLDING_LEVELS
        state = entry.get("ledger_state")
        reasons = entry.get("active_downgrade_reasons", []) or []
        if not backing_holds:
            if state in HOLDING_STATES:
                findings.append(
                    Finding("error", "backing.holds_on_narrowed_backing", "an entry publishes its class while its backing stable claim is narrowed below the stable cutline", entry_id)
                )
            if "backing_stable_claim_narrowed" not in reasons:
                findings.append(
                    Finding("error", "backing.narrowing_reason_missing", "an entry whose backing stable claim is narrowed must carry the backing_stable_claim_narrowed reason", entry_id)
                )
        else:
            if "backing_stable_claim_narrowed" in reasons:
                findings.append(
                    Finding("error", "backing.reason_without_narrowed_claim", "an entry names backing_stable_claim_narrowed but its backing stable claim still holds", entry_id)
                )
    return findings


def validate_publication(ledger: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    publication = ledger.get("publication")
    if not isinstance(publication, dict):
        findings.append(Finding("error", "publication.missing", "ledger must carry a publication block", "<ledger>"))
        return findings
    for field in ("publication_gate", "rationale"):
        if not is_str(publication.get(field)):
            findings.append(Finding("error", "publication.empty_field", f"publication.{field} must be non-empty", "<ledger>"))
    decision, rule_ids, entry_ids = computed_publication(ledger)
    if publication.get("decision") != decision:
        findings.append(
            Finding("error", "publication.decision_inconsistent", f"declared decision {publication.get('decision')!r} disagrees with computed {decision!r}", "<ledger>")
        )
    if list(publication.get("blocking_rule_ids", [])) != rule_ids:
        findings.append(Finding("error", "publication.blocking_rules_mismatch", "blocking_rule_ids disagrees with the firing downgrade rules", "<ledger>"))
    if list(publication.get("blocking_entry_ids", [])) != entry_ids:
        findings.append(Finding("error", "publication.blocking_entries_mismatch", "blocking_entry_ids disagrees with the firing downgrade rules", "<ledger>"))
    return findings


def validate_summary(ledger: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    summary = ledger.get("summary")
    if not isinstance(summary, dict):
        findings.append(Finding("error", "summary.missing", "ledger must carry a summary block", "<ledger>"))
        return findings
    expected = computed_summary(ledger)
    for key, value in expected.items():
        if summary.get(key) != value:
            findings.append(Finding("error", "summary.count_mismatch", f"summary.{key} must equal {value}", key))
    return findings


def validate_ledger(ledger: dict[str, Any], repo_root: Path) -> list[Finding]:
    findings = validate_envelope(ledger)
    archetype_findings, archetypes = validate_archetypes(ledger, repo_root)
    findings.extend(archetype_findings)
    findings.extend(validate_rules(ledger))

    entries = ledger.get("entries")
    if not isinstance(entries, list) or not entries:
        findings.append(Finding("error", "ledger.entries_empty", "ledger must enumerate at least one entry", "<ledger>"))
        return findings

    seen: set[str] = set()
    for raw in entries:
        entry = ensure_dict(raw, "ledger.entries[]")
        findings.extend(validate_entry(entry, archetypes, repo_root))
        entry_id = str(entry.get("entry_id", "<entry>"))
        if entry_id in seen:
            findings.append(Finding("error", "entry.duplicate_id", "entry ids must be unique", entry_id))
        seen.add(entry_id)

    findings.extend(validate_dates(ledger, archetypes))
    findings.extend(validate_backing_claims(ledger, repo_root))
    findings.extend(validate_publication(ledger))
    findings.extend(validate_summary(ledger))
    return findings


def recompute_derived(ledger: dict[str, Any]) -> None:
    """Recompute summary and publication blocks in place so a mutated copy stays
    self-consistent except for the targeted flaw."""
    ledger["summary"] = computed_summary(ledger)
    decision, rule_ids, entry_ids = computed_publication(ledger)
    if isinstance(ledger.get("publication"), dict):
        ledger["publication"].update(
            {"decision": decision, "blocking_rule_ids": rule_ids, "blocking_entry_ids": entry_ids}
        )


def run_negative_drills(ledger: dict[str, Any], repo_root: Path) -> tuple[list[dict[str, str]], list[Finding]]:
    results: list[dict[str, str]] = []
    findings: list[Finding] = []

    def record(drill_id: str, expected_check_id: str, fired: bool) -> None:
        results.append({"drill_id": drill_id, "expected_check_id": expected_check_id, "status": "passed" if fired else "failed"})
        if not fired:
            findings.append(Finding("error", "negative_drill.not_rejected", f"negative drill {drill_id} did not fire", drill_id))

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        return {f.check_id for f in validate_ledger(candidate, repo_root)}

    # A narrowing entry that still publishes its claimed class must be flagged.
    mutated = copy.deepcopy(ledger)
    target = next((e for e in mutated["entries"] if e.get("ledger_state") in NARROWING_STATES), None)
    if target is not None:
        target["effective_class"] = target["claimed_class"]
        recompute_derived(mutated)
        record("effective_not_narrowed_rejected", "entry.effective_not_narrowed", "entry.effective_not_narrowed" in check_ids(mutated))

    # A published entry carrying an active downgrade reason must be flagged.
    mutated = copy.deepcopy(ledger)
    target = next((e for e in mutated["entries"] if e.get("ledger_state") in HOLDING_STATES), None)
    if target is not None:
        target["active_downgrade_reasons"] = ["backing_stable_claim_narrowed"]
        recompute_derived(mutated)
        record("held_with_active_downgrade_rejected", "entry.held_with_active_downgrade", "entry.held_with_active_downgrade" in check_ids(mutated))

    # A Certified claim referencing a missing archetype must be flagged.
    mutated = copy.deepcopy(ledger)
    target = next((e for e in mutated["entries"] if e.get("claimed_class") == "certified"), None)
    if target is not None:
        target["certified_archetype_ref"] = "certified_archetype:does_not_exist"
        record("certified_archetype_not_in_manifest_rejected", "entry.certified_archetype_not_in_manifest", "entry.certified_archetype_not_in_manifest" in check_ids(mutated))

    # A provisional entry whose waiver expired against as_of must be flagged.
    mutated = copy.deepcopy(ledger)
    target = next((e for e in mutated["entries"] if e.get("ledger_state") == "provisional_on_waiver"), None)
    if target is not None and isinstance(target.get("waiver"), dict):
        target["waiver"]["expires_at"] = "2000-01-01"
        record("provisional_on_expired_waiver_rejected", "entry.provisional_on_expired_waiver", "entry.provisional_on_expired_waiver" in check_ids(mutated))

    # An entry that holds while its backing stable claim is narrowed must be flagged.
    mutated = copy.deepcopy(ledger)
    target = next(
        (e for e in mutated["entries"] if e.get("ledger_state") in HOLDING_STATES and is_str(e.get("backing_stable_claim_ref"))),
        None,
    )
    if target is not None:
        target["backing_stable_claim_ref"] = "stable_claim:export_and_offboarding_support"
        record("holds_on_narrowed_backing_rejected", "backing.holds_on_narrowed_backing", "backing.holds_on_narrowed_backing" in check_ids(mutated))

    # A publication decision that disagrees with the firing rules must be flagged.
    mutated = copy.deepcopy(ledger)
    mutated["publication"]["decision"] = "proceed" if mutated["publication"].get("decision") == "hold" else "hold"
    record("publication_decision_inconsistent_rejected", "publication.decision_inconsistent", "publication.decision_inconsistent" in check_ids(mutated))

    return results, findings


def run_fixture_cases(repo_root: Path, fixtures_dir: str) -> tuple[list[dict[str, str]], list[Finding]]:
    """Run the checked-in fixture cases. Each case the manifest marks as rejected
    must fail validation with its expected check id."""
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
        ids = {f.check_id for f in validate_ledger(ensure_dict(candidate, "fixture"), repo_root)}
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
    ledger: dict[str, Any],
    findings: list[Finding],
    drill_results: list[dict[str, str]],
    fixture_results: list[dict[str, str]],
) -> None:
    decision, rule_ids, entry_ids = computed_publication(ledger)
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": "support_class_ledger_validation_capture",
        "status": "pass" if not any(f.severity == "error" for f in findings) else "fail",
        "generated_at": generated_at_now(),
        "ledger_id": ledger.get("ledger_id"),
        "release_train": ledger.get("release_train"),
        "as_of": ledger.get("as_of"),
        "summary": ledger.get("summary"),
        "publication": {
            "decision": decision,
            "blocking_rule_ids": rule_ids,
            "blocking_entry_ids": entry_ids,
        },
        "negative_drills": drill_results,
        "fixture_cases": fixture_results,
        "findings": [f.as_report() for f in findings],
    }
    path.write_text(json_text(payload), encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    ledger = ensure_dict(load_json(repo_root / args.ledger, "support-class ledger"), "support-class ledger")

    findings = validate_ledger(ledger, repo_root)
    drill_results, drill_findings = run_negative_drills(ledger, repo_root)
    findings.extend(drill_findings)
    fixture_results, fixture_findings = run_fixture_cases(repo_root, args.fixtures_dir)
    findings.extend(fixture_findings)

    report_rel = args.report
    if report_rel is None and not args.no_capture:
        report_rel = DEFAULT_REPORT_REL
    if report_rel:
        write_report(repo_root / report_rel, ledger, findings, drill_results, fixture_results)

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        for item in errors:
            print(f"ERROR [{item.check_id}] ({item.ref}): {item.message}", file=sys.stderr)
        return 1

    summary = ledger.get("summary", {})
    decision, rule_ids, _entry_ids = computed_publication(ledger)
    print(
        "support-class ledger validated "
        f"({summary.get('total_entries')} entries, "
        f"{summary.get('entries_published_as_claimed')} published as claimed, "
        f"{summary.get('entries_narrowed')} narrowed; "
        f"{summary.get('certified_archetypes_total')} archetypes "
        f"({summary.get('certified_archetypes_decertified')} decertified); "
        f"{summary.get('downgrade_rules_firing')} downgrade rules firing; "
        f"publication={decision}; {len(drill_results)} negative drills, {len(fixture_results)} fixture cases)"
    )

    if args.require_proceed and decision == "hold":
        print(
            f"PUBLICATION HELD: v1.0 support line blocked by {len(rule_ids)} rule(s): {', '.join(rule_ids)}",
            file=sys.stderr,
        )
        return 2

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
