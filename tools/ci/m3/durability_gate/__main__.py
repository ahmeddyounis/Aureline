#!/usr/bin/env python3
"""Validate the protected-beta durability packet.

The gate turns maintainer coverage, signing/quorum split-authority, and
critical-upstream sustainment into release-bearing truth. It refuses to
let a protected beta path carry durable claims when:

- a protected row is operationally single-human with no named backup and
  no active backup waiver;
- release, rollback, revocation, or registry-emergency authority is
  encoded as a single-human path or drifts from the signing-quorum
  action matrix; or
- a red-risk critical upstream on a protected path has no named
  sustainment owner, no fork/replace strategy, or a stale review.

The packet (``maintainer_coverage_matrix.json``) is the source of truth.
The critical-upstream CSV register, the signing/quorum/break-glass
markdown, and the validation capture are generated projections; the gate
regenerates them and (under ``--check``) fails if the checked-in copies
are stale, so shiproom and release packets can consume the register
without manual reassembly.
"""

from __future__ import annotations

import argparse
import copy
import csv
import dataclasses
import datetime as dt
import io
import json
import sys
from pathlib import Path
from typing import Any

from tools.ci.m3._common import render_yaml_as_json


DEFAULT_PACKET_REL = "artifacts/release/m3/maintainer_coverage_matrix.json"
DEFAULT_SCHEMA_REL = "schemas/release/beta_durability_packet.schema.json"
DEFAULT_CSV_REL = "artifacts/release/m3/critical_upstream_register.csv"
DEFAULT_SIGNING_MD_REL = "artifacts/release/m3/signing_quorum_and_breakglass.md"
DEFAULT_CAPTURE_REL = (
    "artifacts/release/m3/captures/durability_packet_validation_capture.json"
)
DEFAULT_DOC_REL = "docs/release/m3/durability_and_quorum_beta.md"
DEFAULT_SIGNING_QUORUM_REL = "artifacts/governance/signing_quorum.yaml"
DEFAULT_OWNERSHIP_MATRIX_REL = "artifacts/governance/ownership_matrix.yaml"
DEFAULT_CLAIM_MANIFEST_REL = "artifacts/release/m3/claim_manifest.json"
DEFAULT_UPSTREAM_REGISTER_REL = (
    "artifacts/governance/critical_upstream_health_register.yaml"
)
DEFAULT_FIXTURE_MANIFEST_REL = "fixtures/release/beta_durability_cases/manifest.yaml"

EXPECTED_PACKET_RECORD_KIND = "beta_durability_packet"
EXPECTED_CAPTURE_RECORD_KIND = "beta_durability_validation_capture"
EXPECTED_SCHEMA_VERSION = 1

WAIVER_ID = "single-maintainer-backup"

REQUIRED_AUTHORITIES = {
    "release_signing",
    "rollback",
    "revocation",
    "registry_emergency",
    "signer_roster_change",
}
# Authorities that may never normalize a single-responder break-glass path.
BREAK_GLASS_FORBIDDEN_AUTHORITIES = {"release_signing", "signer_roster_change"}
REQUIRED_ROSTER_ROLES = {"release_operator", "security_operator"}

PLACEHOLDER_OWNERS = {
    "",
    "tbd",
    "tba",
    "none",
    "unassigned",
    "not_yet_named",
    "not_yet_named_alpha_gap",
}

DOC_REQUIRED_TOKENS = [
    "release.beta_durability_packet.m3",
    "artifacts/release/m3/critical_upstream_register.csv",
    "artifacts/release/m3/signing_quorum_and_breakglass.md",
    "single-maintainer-backup",
    "break-glass",
    "succession",
    "release_signing",
    "rollback",
    "revocation",
    "registry_emergency",
    "signer_roster_change",
]


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    ref: str
    remediation: str

    def as_report(self) -> dict[str, str]:
        return dataclasses.asdict(self)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--packet", default=DEFAULT_PACKET_REL)
    parser.add_argument("--schema", default=DEFAULT_SCHEMA_REL)
    parser.add_argument("--csv", default=DEFAULT_CSV_REL)
    parser.add_argument("--signing-md", default=DEFAULT_SIGNING_MD_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument("--doc", default=DEFAULT_DOC_REL)
    parser.add_argument("--signing-quorum", default=DEFAULT_SIGNING_QUORUM_REL)
    parser.add_argument("--ownership-matrix", default=DEFAULT_OWNERSHIP_MATRIX_REL)
    parser.add_argument("--claim-manifest", default=DEFAULT_CLAIM_MANIFEST_REL)
    parser.add_argument("--upstream-register", default=DEFAULT_UPSTREAM_REGISTER_REL)
    parser.add_argument("--fixture-manifest", default=DEFAULT_FIXTURE_MANIFEST_REL)
    parser.add_argument(
        "--today",
        default=None,
        help="Override today's ISO date (YYYY-MM-DD); defaults to packet as_of.",
    )
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if the generated CSV, signing markdown, or capture would change.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a JSON object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a JSON array")
    return value


def as_json_text(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=False) + "\n"


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0]


def repo_ref_exists(repo_root: Path, ref: str) -> bool:
    return (repo_root / strip_fragment(ref)).exists()


def parse_iso_date(value: str) -> dt.date | None:
    try:
        return dt.date.fromisoformat(value)
    except Exception:  # noqa: BLE001
        return None


def is_placeholder(value: Any) -> bool:
    return (not isinstance(value, str)) or value.strip().lower() in PLACEHOLDER_OWNERS


def validate_against_schema(
    payload: dict[str, Any],
    schema: dict[str, Any],
    payload_ref: str,
) -> list[Finding]:
    try:
        from jsonschema import Draft202012Validator, FormatChecker  # type: ignore
    except Exception:  # noqa: BLE001
        return []

    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    findings: list[Finding] = []
    for error in sorted(validator.iter_errors(payload), key=lambda err: list(err.path)):
        path = ".".join(str(part) for part in error.path) or "<root>"
        findings.append(
            Finding(
                "error",
                "durability.schema.validation",
                f"{path}: {error.message}",
                payload_ref,
                "Update the packet or schema so the checked-in record validates.",
            )
        )
    return findings


# ---------------------------------------------------------------------------
# Deterministic projections
# ---------------------------------------------------------------------------

CSV_COLUMNS = [
    "dependency_id",
    "dependency_name",
    "health_rating",
    "health_status",
    "license_class",
    "risk_posture",
    "sustainment_owner",
    "sponsor_or_sustainment_state",
    "escalation_owner",
    "next_review_due",
    "protected_path_refs",
    "owning_packages",
    "fork_replace_strategy",
    "source_register_ref",
]


def build_critical_upstream_csv(packet: dict[str, Any]) -> str:
    buffer = io.StringIO()
    writer = csv.writer(buffer, lineterminator="\n")
    writer.writerow(CSV_COLUMNS)
    for row in ensure_list(packet.get("critical_upstreams", []), "critical_upstreams"):
        if not isinstance(row, dict):
            continue
        writer.writerow(
            [
                row.get("dependency_id", ""),
                row.get("dependency_name", ""),
                row.get("health_rating", ""),
                row.get("health_status", ""),
                row.get("license_class", ""),
                row.get("risk_posture", ""),
                row.get("sustainment_owner", ""),
                row.get("sponsor_or_sustainment_state", ""),
                row.get("escalation_owner", ""),
                row.get("next_review_due", ""),
                " | ".join(row.get("protected_path_refs", []) or []),
                " | ".join(row.get("owning_packages", []) or []),
                row.get("fork_replace_strategy", ""),
                row.get("source_register_ref", ""),
            ]
        )
    return buffer.getvalue()


def build_signing_md(packet: dict[str, Any]) -> str:
    signing = ensure_dict(packet.get("signing_authority", {}), "signing_authority")
    posture = ensure_dict(packet.get("repo_posture", {}), "repo_posture")
    lines: list[str] = []
    lines.append("# Release signing, quorum, and break-glass")
    lines.append("")
    lines.append(
        "> Generated from "
        f"`{DEFAULT_PACKET_REL}` (`{packet.get('packet_id')}`). "
        "Do not edit by hand; run the durability gate to refresh."
    )
    lines.append("")
    lines.append(
        "This is the split-authority projection of "
        f"[`{signing.get('policy_ref')}`](../../../{signing.get('policy_ref')}). "
        "Release, rollback, revocation, and registry-emergency authority cite "
        "the action ids in that matrix rather than inventing per-run quorum "
        "rules in PR comments or chat logs."
    )
    lines.append("")
    lines.append(f"- Governing principle: `{signing.get('no_single_human_principle')}`")
    lines.append(f"- Policy narrative: `{signing.get('policy_doc_ref')}`")
    lines.append(
        "- Repository posture: single-maintainer backup waiver "
        f"`{posture.get('single_maintainer_waiver_ref')}` "
        f"(expires {posture.get('waiver_expires_on')})."
    )
    lines.append("")
    lines.append("## Signer roster")
    lines.append("")
    lines.append("| Signer | Role | Named humans | Backup state | Scopes |")
    lines.append("|---|---|---|---|---|")
    for entry in ensure_list(signing.get("signer_roster", []), "signer_roster"):
        if not isinstance(entry, dict):
            continue
        lines.append(
            "| {signer} | {role} | {humans} | {backup} | {scopes} |".format(
                signer=entry.get("signer_id", ""),
                role=entry.get("role", ""),
                humans=", ".join(entry.get("named_humans", []) or []),
                backup=entry.get("backup_state", ""),
                scopes=", ".join(entry.get("scopes", []) or []),
            )
        )
    lines.append("")
    lines.append("## Split-authority actions")
    lines.append("")
    lines.append(
        "| Authority | Signing-quorum action | Quorum profile | "
        "Min distinct humans | Author-only forbidden | Break-glass |"
    )
    lines.append("|---|---|---|---|---|---|")
    for action in ensure_list(
        signing.get("split_authority_actions", []), "split_authority_actions"
    ):
        if not isinstance(action, dict):
            continue
        lines.append(
            "| {authority} | {action_id} | {profile} | {humans} | {author} | {bg} |".format(
                authority=action.get("authority", ""),
                action_id=action.get("signing_quorum_action_id", ""),
                profile=action.get("quorum_profile", ""),
                humans=action.get("min_distinct_humans", ""),
                author=str(action.get("author_only_forbidden", "")).lower(),
                bg=action.get("break_glass_profile", ""),
            )
        )
    lines.append("")
    break_glass = ensure_dict(signing.get("break_glass", {}), "break_glass")
    lines.append("## Break-glass containment")
    lines.append("")
    lines.append(f"- Profile: `{break_glass.get('profile_id')}`")
    lines.append(f"- Maximum duration (hours): {break_glass.get('max_duration_hours')}")
    lines.append(
        "- Retrospective quorum profile: "
        f"`{break_glass.get('retrospective_quorum_profile')}`"
    )
    lines.append("- Forbidden for:")
    for item in break_glass.get("forbidden_for", []) or []:
        lines.append(f"  - `{item}`")
    lines.append("- Required audit fields:")
    for item in break_glass.get("required_audit_fields", []) or []:
        lines.append(f"  - `{item}`")
    lines.append("")
    return "\n".join(lines) + "\n"


# ---------------------------------------------------------------------------
# Validator
# ---------------------------------------------------------------------------


class Validator:
    def __init__(
        self,
        *,
        repo_root: Path,
        packet: dict[str, Any],
        signing_quorum: dict[str, Any],
        ownership_matrix: dict[str, Any],
        claim_row_ids: set[str],
        upstream_register: dict[str, Any],
        today: dt.date,
        packet_rel: str,
        doc_rel: str,
        csv_rel: str,
        signing_md_rel: str,
        capture_rel: str,
    ) -> None:
        self.repo_root = repo_root
        self.packet = packet
        self.signing_quorum = signing_quorum
        self.ownership_matrix = ownership_matrix
        self.claim_row_ids = claim_row_ids
        self.upstream_register = upstream_register
        self.today = today
        self.packet_rel = packet_rel
        self.doc_rel = doc_rel
        self.csv_rel = csv_rel
        self.signing_md_rel = signing_md_rel
        self.capture_rel = capture_rel
        self.findings: list[Finding] = []

    def push(
        self,
        check_id: str,
        message: str,
        ref: str,
        remediation: str,
        severity: str = "error",
    ) -> None:
        self.findings.append(Finding(severity, check_id, message, ref, remediation))

    # -- helpers ----------------------------------------------------------

    def waiver_active(self) -> bool:
        waivers = ensure_dict(
            self.ownership_matrix.get("waivers", {}), "ownership_matrix.waivers"
        )
        waiver = waivers.get(WAIVER_ID)
        if not isinstance(waiver, dict):
            return False
        if waiver.get("closed_on"):
            return False
        expires = parse_iso_date(str(waiver.get("expires_on", "")))
        if expires is None:
            return False
        return expires >= self.today

    def signing_index(self) -> tuple[dict[str, dict], dict[str, dict], dict[str, dict]]:
        actions = {
            str(a.get("id")): a
            for a in ensure_list(
                self.signing_quorum.get("actions", []), "signing_quorum.actions"
            )
            if isinstance(a, dict)
        }
        profiles = {
            str(p.get("id")): p
            for p in ensure_list(
                self.signing_quorum.get("quorum_profiles", []),
                "signing_quorum.quorum_profiles",
            )
            if isinstance(p, dict)
        }
        break_glass = {
            str(b.get("id")): b
            for b in ensure_list(
                self.signing_quorum.get("break_glass_profiles", []),
                "signing_quorum.break_glass_profiles",
            )
            if isinstance(b, dict)
        }
        return actions, profiles, break_glass

    # -- top level --------------------------------------------------------

    def validate(self, *, include_surfaces: bool = True) -> list[Finding]:
        self.validate_header()
        if include_surfaces:
            self.validate_source_refs()
        self.validate_repo_posture()
        self.validate_protected_paths(include_surfaces=include_surfaces)
        self.validate_signing_authority()
        self.validate_critical_upstreams()
        if include_surfaces:
            self.validate_durability_doc()
        return self.findings

    def validate_header(self) -> None:
        if self.packet.get("record_kind") != EXPECTED_PACKET_RECORD_KIND:
            self.push(
                "durability.record_kind",
                f"record_kind must be {EXPECTED_PACKET_RECORD_KIND}",
                str(self.packet.get("packet_id", "<packet>")),
                "Use the beta durability packet discriminator.",
            )
        if self.packet.get("schema_version") != EXPECTED_SCHEMA_VERSION:
            self.push(
                "durability.schema_version",
                "schema_version must be 1",
                str(self.packet.get("packet_id", "<packet>")),
                "Keep the packet on schema version 1 until a governed migration exists.",
            )
        outputs = ensure_dict(
            self.packet.get("generated_outputs", {}), "generated_outputs"
        )
        expected_outputs = {
            "critical_upstream_register_csv_ref": self.csv_rel,
            "signing_quorum_and_breakglass_md_ref": self.signing_md_rel,
            "validation_capture_ref": self.capture_rel,
        }
        for field, expected in expected_outputs.items():
            if outputs.get(field) != expected:
                self.push(
                    "durability.generated_output_ref_mismatch",
                    f"{field} must match the generated output path",
                    f"generated_outputs.{field}",
                    f"Set {field} to {expected}.",
                )

    def validate_source_refs(self) -> None:
        source_refs = ensure_dict(self.packet.get("source_refs", {}), "source_refs")
        for field, ref in source_refs.items():
            if not isinstance(ref, str) or not ref:
                self.push(
                    "durability.source_ref.empty",
                    "source refs must be non-empty strings",
                    f"source_refs.{field}",
                    "Populate the source ref with a repo-relative path.",
                )
                continue
            if not repo_ref_exists(self.repo_root, ref):
                self.push(
                    "durability.source_ref.missing_on_disk",
                    "repo-relative source ref does not exist",
                    ref,
                    "Add the source artifact or correct the packet ref.",
                )

    def validate_repo_posture(self) -> None:
        posture = ensure_dict(self.packet.get("repo_posture", {}), "repo_posture")
        waiver_ref = str(posture.get("single_maintainer_waiver_ref", ""))
        if not repo_ref_exists(self.repo_root, waiver_ref):
            self.push(
                "durability.posture.waiver_ref_missing",
                "single-maintainer waiver ref does not resolve on disk",
                waiver_ref,
                "Point repo_posture.single_maintainer_waiver_ref at the ownership matrix waiver.",
            )
        waivers = ensure_dict(
            self.ownership_matrix.get("waivers", {}), "ownership_matrix.waivers"
        )
        waiver = waivers.get(WAIVER_ID)
        if not isinstance(waiver, dict):
            self.push(
                "durability.posture.waiver_unknown",
                f"waiver {WAIVER_ID} is absent from the ownership matrix",
                waiver_ref,
                "Record the waiver in the ownership matrix before citing it.",
            )
            return
        declared_expiry = parse_iso_date(str(posture.get("waiver_expires_on", "")))
        matrix_expiry = parse_iso_date(str(waiver.get("expires_on", "")))
        if declared_expiry != matrix_expiry:
            self.push(
                "durability.posture.waiver_expiry_mismatch",
                "packet waiver expiry does not match the ownership matrix",
                "repo_posture.waiver_expires_on",
                f"Set waiver_expires_on to {waiver.get('expires_on')}.",
            )
        if not self.waiver_active():
            self.push(
                "durability.posture.waiver_expired",
                "the single-maintainer waiver is closed or expired; waiver-backed "
                "coverage is no longer valid",
                waiver_ref,
                "Name backup owners or open a renewed, dated waiver with a correction plan.",
            )

    def validate_protected_paths(self, *, include_surfaces: bool) -> None:
        rows = ensure_list(self.packet.get("protected_paths", []), "protected_paths")
        seen: set[str] = set()
        waiver_active = self.waiver_active()
        for row in rows:
            if not isinstance(row, dict):
                continue
            path_id = str(row.get("path_id", ""))
            if path_id in seen:
                self.push(
                    "durability.protected_path.duplicate_id",
                    "protected path ids must be unique",
                    path_id,
                    "Give each protected path one stable path_id.",
                )
            seen.add(path_id)

            if is_placeholder(row.get("primary_maintainer")):
                self.push(
                    "durability.coverage.primary_missing",
                    "protected path has no named primary maintainer",
                    path_id,
                    "Name a real human as primary_maintainer.",
                )

            has_backup = not is_placeholder(row.get("backup_maintainer"))
            waiver_ref = row.get("backup_waiver_ref")
            cites_waiver = isinstance(waiver_ref, str) and bool(waiver_ref.strip())
            depth = row.get("standing_reviewer_depth", 0)
            declared_state = str(row.get("coverage_state", ""))

            if has_backup:
                derived_state = "covered"
                if not isinstance(depth, int) or depth < 2:
                    self.push(
                        "durability.coverage.reviewer_depth_too_low",
                        "a named backup must give standing reviewer depth of at least 2",
                        path_id,
                        "Set standing_reviewer_depth to 2 or more when a backup is named.",
                    )
            elif cites_waiver and waiver_active:
                derived_state = "waiver_backed"
            else:
                derived_state = "uncovered"

            if derived_state == "uncovered":
                self.push(
                    "durability.coverage.backup_missing_without_waiver",
                    "protected path has no named backup and no active backup waiver",
                    path_id,
                    "Name a backup maintainer or cite an active, dated backup waiver.",
                )
            if cites_waiver and not waiver_active and not has_backup:
                self.push(
                    "durability.coverage.waiver_expired",
                    "protected path cites a waiver that is closed or expired",
                    path_id,
                    "Name a backup maintainer or renew the waiver with a correction plan.",
                )
            if declared_state != derived_state:
                self.push(
                    "durability.coverage.state_inconsistent",
                    f"declared coverage_state {declared_state!r} does not match the "
                    f"derived state {derived_state!r}",
                    path_id,
                    "Set coverage_state to match the actual backup/waiver posture.",
                )

            due = parse_iso_date(str(row.get("next_review_due", "")))
            if due is None:
                self.push(
                    "durability.coverage.review_due_invalid",
                    "next_review_due is not a valid ISO date",
                    path_id,
                    "Set next_review_due to a YYYY-MM-DD date.",
                )
            elif due < self.today:
                self.push(
                    "durability.coverage.review_overdue",
                    f"coverage review for the protected path is overdue ({due.isoformat()})",
                    path_id,
                    "Run the coverage review and set the next due date.",
                )

            if include_surfaces and str(row.get("path_kind")) in {
                "beta_surface",
                "beta_archetype",
            }:
                claim_refs = ensure_list(
                    row.get("claim_row_refs", []), f"{path_id}.claim_row_refs"
                )
                if not claim_refs:
                    self.push(
                        "durability.protected_path.claim_ref_missing",
                        "beta surface/archetype rows must bind to a claim manifest row",
                        path_id,
                        "Add claim_row_refs pointing at the matching claim manifest row.",
                    )
                for ref in claim_refs:
                    row_id = str(ref).split("#", 1)[-1]
                    if row_id not in self.claim_row_ids:
                        self.push(
                            "durability.protected_path.claim_ref_unknown",
                            "claim_row_ref does not resolve to a claim manifest row",
                            f"{path_id}:{ref}",
                            "Point claim_row_refs at a real claim manifest row_id.",
                        )

    def validate_signing_authority(self) -> None:
        signing = ensure_dict(
            self.packet.get("signing_authority", {}), "signing_authority"
        )
        actions, profiles, break_glass_profiles = self.signing_index()

        # Roster split.
        roles = {
            str(e.get("role"))
            for e in ensure_list(signing.get("signer_roster", []), "signer_roster")
            if isinstance(e, dict)
        }
        missing_roles = REQUIRED_ROSTER_ROLES - roles
        if missing_roles:
            self.push(
                "durability.signing.roster_authority_not_split",
                "signer roster must split release and security operator authority",
                "signing_authority.signer_roster",
                f"Add roster roles: {', '.join(sorted(missing_roles))}.",
            )
        for entry in ensure_list(signing.get("signer_roster", []), "signer_roster"):
            if not isinstance(entry, dict):
                continue
            named = entry.get("named_humans", [])
            real = [h for h in named if not is_placeholder(h)]
            if not real:
                self.push(
                    "durability.signing.roster_entry_unnamed",
                    "signer roster entry names no real human",
                    str(entry.get("signer_id")),
                    "Name a real human signer.",
                )
            if entry.get("backup_state") == "single_maintainer_waiver":
                ref = entry.get("backup_waiver_ref")
                if not (isinstance(ref, str) and ref.strip()):
                    self.push(
                        "durability.signing.roster_waiver_ref_missing",
                        "roster entry under the single-maintainer waiver must cite it",
                        str(entry.get("signer_id")),
                        "Cite the backup_waiver_ref for the waiver-backed signer.",
                    )
                else:
                    self.push(
                        "durability.signing.roster_single_maintainer",
                        "signer role is single-maintainer under the backup waiver",
                        str(entry.get("signer_id")),
                        "Name a second signer to close the waiver for this role.",
                        severity="warning",
                    )

        # Split-authority actions.
        seen_authorities: set[str] = set()
        for action in ensure_list(
            signing.get("split_authority_actions", []), "split_authority_actions"
        ):
            if not isinstance(action, dict):
                continue
            authority = str(action.get("authority", ""))
            if authority in seen_authorities:
                self.push(
                    "durability.signing.duplicate_authority",
                    "each split authority may appear only once",
                    authority,
                    "Remove the duplicate split-authority row.",
                )
            seen_authorities.add(authority)
            action_id = str(action.get("signing_quorum_action_id", ""))
            quorum_profile = str(action.get("quorum_profile", ""))
            min_humans = action.get("min_distinct_humans", 0)
            bg_profile = str(action.get("break_glass_profile", ""))

            policy_action = actions.get(action_id)
            if policy_action is None:
                self.push(
                    "durability.signing.unknown_action_id",
                    "signing_quorum_action_id is not in the signing-quorum action matrix",
                    f"{authority}:{action_id}",
                    "Reference a real action id from signing_quorum.yaml.",
                )
            else:
                if policy_action.get("default_quorum_profile") != quorum_profile:
                    self.push(
                        "durability.signing.quorum_profile_mismatch",
                        "quorum_profile does not match the signing-quorum action",
                        f"{authority}:{action_id}",
                        "Use the action's default_quorum_profile from signing_quorum.yaml.",
                    )
                policy_bg = str(policy_action.get("break_glass_profile", "none"))
                if bg_profile != policy_bg:
                    self.push(
                        "durability.signing.break_glass_profile_mismatch",
                        "break_glass_profile does not match the signing-quorum action",
                        f"{authority}:{action_id}",
                        f"Set break_glass_profile to {policy_bg}.",
                    )
            profile = profiles.get(quorum_profile)
            if profile is None:
                self.push(
                    "durability.signing.unknown_quorum_profile",
                    "quorum_profile is not defined in signing_quorum.yaml",
                    f"{authority}:{quorum_profile}",
                    "Reference a real quorum profile id.",
                )
            elif profile.get("min_distinct_humans") != min_humans:
                self.push(
                    "durability.signing.min_humans_mismatch",
                    "min_distinct_humans does not match the quorum profile",
                    f"{authority}:{quorum_profile}",
                    f"Set min_distinct_humans to {profile.get('min_distinct_humans')}.",
                )
            if not isinstance(min_humans, int) or min_humans < 2:
                self.push(
                    "durability.signing.single_human_authority",
                    "split authority must require at least two distinct humans",
                    authority,
                    "Raise min_distinct_humans to the quorum floor (no single-human release path).",
                )
            if action.get("author_only_forbidden") is not True:
                self.push(
                    "durability.signing.author_only_allowed",
                    "split authority must forbid author-only approval",
                    authority,
                    "Set author_only_forbidden to true.",
                )
            if bg_profile != "none" and bg_profile not in break_glass_profiles:
                self.push(
                    "durability.signing.unknown_break_glass_profile",
                    "break_glass_profile is not defined in signing_quorum.yaml",
                    f"{authority}:{bg_profile}",
                    "Reference a real break-glass profile id or use none.",
                )
            if authority in BREAK_GLASS_FORBIDDEN_AUTHORITIES and bg_profile != "none":
                self.push(
                    "durability.signing.break_glass_forbidden_for_authority",
                    "release signing and signer-roster change may not use break-glass",
                    authority,
                    "Set break_glass_profile to none for this authority.",
                )

        missing_auth = REQUIRED_AUTHORITIES - seen_authorities
        if missing_auth:
            self.push(
                "durability.signing.required_authority_missing",
                "split-authority matrix is missing a required emergency authority",
                "signing_authority.split_authority_actions",
                f"Add split-authority rows for: {', '.join(sorted(missing_auth))}.",
            )

        # Break-glass contract cross-check.
        break_glass = ensure_dict(signing.get("break_glass", {}), "break_glass")
        bg_id = str(break_glass.get("profile_id", ""))
        policy_bg = break_glass_profiles.get(bg_id)
        if policy_bg is None:
            self.push(
                "durability.signing.break_glass_contract_unknown",
                "break-glass profile_id is not defined in signing_quorum.yaml",
                bg_id,
                "Reference a real break-glass profile id.",
            )
        else:
            if set(break_glass.get("forbidden_for", []) or []) != set(
                policy_bg.get("forbidden_for", []) or []
            ):
                self.push(
                    "durability.signing.break_glass_forbidden_for_mismatch",
                    "break-glass forbidden_for drifts from the signing-quorum policy",
                    bg_id,
                    "Re-project forbidden_for from signing_quorum.yaml.",
                )
            if set(break_glass.get("required_audit_fields", []) or []) != set(
                policy_bg.get("required_audit_fields", []) or []
            ):
                self.push(
                    "durability.signing.break_glass_audit_fields_mismatch",
                    "break-glass required_audit_fields drifts from the policy",
                    bg_id,
                    "Re-project required_audit_fields from signing_quorum.yaml.",
                )
            if break_glass.get("retrospective_quorum_profile") != policy_bg.get(
                "retrospective_quorum_profile"
            ):
                self.push(
                    "durability.signing.break_glass_retro_mismatch",
                    "break-glass retrospective_quorum_profile drifts from the policy",
                    bg_id,
                    "Re-project retrospective_quorum_profile from signing_quorum.yaml.",
                )

    def validate_critical_upstreams(self) -> None:
        rows = ensure_list(
            self.packet.get("critical_upstreams", []), "critical_upstreams"
        )
        path_ids = {
            str(p.get("path_id"))
            for p in ensure_list(self.packet.get("protected_paths", []), "protected_paths")
            if isinstance(p, dict)
        }
        register_reds = set(
            self.upstream_register.get("red_risk_dependency_ids", []) or []
        )
        register_rows = {
            str(r.get("dependency_id")): r
            for r in ensure_list(
                self.upstream_register.get("rows", []), "upstream_register.rows"
            )
            if isinstance(r, dict)
        }
        covered: set[str] = set()
        for row in rows:
            if not isinstance(row, dict):
                continue
            dep_id = str(row.get("dependency_id", ""))
            covered.add(dep_id)
            rating = str(row.get("health_rating", ""))

            register_row = register_rows.get(dep_id)
            if register_row is None:
                self.push(
                    "durability.upstream.not_in_register",
                    "critical upstream is absent from the critical-upstream register",
                    dep_id,
                    "Add the dependency to the critical-upstream register or correct the id.",
                )
            elif str(register_row.get("risk_state", "")) != rating:
                self.push(
                    "durability.upstream.health_rating_mismatch",
                    "health_rating drifts from the critical-upstream register risk_state",
                    dep_id,
                    f"Set health_rating to {register_row.get('risk_state')}.",
                )

            if not repo_ref_exists(self.repo_root, str(row.get("source_register_ref", ""))):
                self.push(
                    "durability.upstream.source_register_missing",
                    "source_register_ref does not resolve on disk",
                    dep_id,
                    "Point source_register_ref at the critical-upstream register anchor.",
                )

            for ref in row.get("protected_path_refs", []) or []:
                if str(ref) not in path_ids:
                    self.push(
                        "durability.upstream.unknown_protected_path_ref",
                        "protected_path_ref does not resolve to a packet path_id",
                        f"{dep_id}:{ref}",
                        "Point protected_path_refs at a real protected path_id.",
                    )

            if rating == "red":
                if is_placeholder(row.get("sustainment_owner")):
                    self.push(
                        "durability.upstream.red_risk_missing_owner",
                        "red-risk upstream has no named sustainment owner",
                        dep_id,
                        "Name a real sustainment owner for the red-risk upstream.",
                    )
                if is_placeholder(row.get("fork_replace_strategy")):
                    self.push(
                        "durability.upstream.red_risk_missing_fork_replace",
                        "red-risk upstream has no fork/replace strategy",
                        dep_id,
                        "Record a fork, replace, or sustainment strategy.",
                    )
                if is_placeholder(row.get("license_class")):
                    self.push(
                        "durability.upstream.red_risk_missing_license",
                        "red-risk upstream has no license/risk class",
                        dep_id,
                        "Record the license_class for the upstream.",
                    )
                due = parse_iso_date(str(row.get("next_review_due", "")))
                if due is None:
                    self.push(
                        "durability.upstream.review_due_invalid",
                        "next_review_due is not a valid ISO date",
                        dep_id,
                        "Set next_review_due to a YYYY-MM-DD date.",
                    )
                elif due < self.today:
                    self.push(
                        "durability.upstream.red_risk_review_stale",
                        f"red-risk upstream review is overdue ({due.isoformat()})",
                        dep_id,
                        "Refresh the upstream health review and set the next due date.",
                    )

        uncovered = register_reds - covered
        if uncovered:
            self.push(
                "durability.upstream.register_red_risk_uncovered",
                "red-risk upstreams in the register are missing from the durability packet",
                "critical_upstreams",
                f"Add critical upstream rows for: {', '.join(sorted(uncovered))}.",
            )

    def validate_durability_doc(self) -> None:
        path = self.repo_root / self.doc_rel
        if not path.exists():
            self.push(
                "durability.doc.missing",
                "durability contract doc does not exist",
                self.doc_rel,
                "Add the durability_and_quorum_beta doc.",
            )
            return
        text = path.read_text(encoding="utf-8")
        for token in DOC_REQUIRED_TOKENS:
            if token not in text:
                self.push(
                    "durability.doc.required_token_missing",
                    "durability doc does not quote required durability vocabulary",
                    self.doc_rel,
                    f"Quote {token} in the durability contract doc.",
                )


# ---------------------------------------------------------------------------
# Generated-output comparison + capture
# ---------------------------------------------------------------------------


def set_payload_path(payload: Any, path: str, value: Any) -> bool:
    segments = [segment.strip() for segment in path.split(".") if segment.strip()]
    if not segments:
        return False
    cursor = payload
    for segment in segments[:-1]:
        if isinstance(cursor, list):
            try:
                index = int(segment)
            except ValueError:
                return False
            if index < 0 or index >= len(cursor):
                return False
            cursor = cursor[index]
        elif isinstance(cursor, dict) and segment in cursor:
            cursor = cursor[segment]
        else:
            return False
    last = segments[-1]
    if isinstance(cursor, list):
        try:
            index = int(last)
        except ValueError:
            return False
        if index < 0 or index >= len(cursor):
            return False
        cursor[index] = value
        return True
    if isinstance(cursor, dict) and last in cursor:
        cursor[last] = value
        return True
    return False


def compare_or_write(
    path: Path,
    expected_text: str,
    check: bool,
    findings: list[Finding],
    check_id: str,
) -> None:
    if check:
        actual = path.read_text(encoding="utf-8") if path.exists() else ""
        if actual != expected_text:
            findings.append(
                Finding(
                    "error",
                    check_id,
                    "checked-in generated file is stale",
                    str(path),
                    "Run the durability gate without --check and commit the generated output.",
                )
            )
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(expected_text, encoding="utf-8")


def build_capture(
    *,
    packet: dict[str, Any],
    findings: list[Finding],
    fixture_results: list[dict[str, Any]],
    today: dt.date,
    packet_rel: str,
) -> dict[str, Any]:
    protected = [
        p for p in packet.get("protected_paths", []) if isinstance(p, dict)
    ]
    coverage_states = {"covered": 0, "waiver_backed": 0, "uncovered": 0}
    for row in protected:
        state = str(row.get("coverage_state", ""))
        if state in coverage_states:
            coverage_states[state] += 1
    upstreams = [u for u in packet.get("critical_upstreams", []) if isinstance(u, dict)]
    red_count = sum(1 for u in upstreams if u.get("health_rating") == "red")
    error_count = sum(1 for f in findings if f.severity == "error")
    return {
        "record_kind": EXPECTED_CAPTURE_RECORD_KIND,
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "validated_at": packet.get("as_of"),
        "packet_id": packet.get("packet_id"),
        "source_packet_ref": packet_rel,
        "passed": error_count == 0,
        "coverage": {
            "evaluated_against_date": today.isoformat(),
            "protected_path_count": len(protected),
            "coverage_state_counts": coverage_states,
            "split_authority_count": len(
                packet.get("signing_authority", {}).get("split_authority_actions", [])
            ),
            "signer_roster_count": len(
                packet.get("signing_authority", {}).get("signer_roster", [])
            ),
            "critical_upstream_count": len(upstreams),
            "red_risk_upstream_count": red_count,
            "fixture_mutation_count": len(fixture_results),
            "error_count": error_count,
            "warning_count": sum(1 for f in findings if f.severity == "warning"),
        },
        "fixture_results": fixture_results,
        "findings": [f.as_report() for f in findings],
    }


def validate_fixture_manifest(
    *,
    repo_root: Path,
    manifest_rel: str,
    packet: dict[str, Any],
    signing_quorum: dict[str, Any],
    ownership_matrix: dict[str, Any],
    claim_row_ids: set[str],
    upstream_register: dict[str, Any],
    today: dt.date,
    packet_rel: str,
    doc_rel: str,
    csv_rel: str,
    signing_md_rel: str,
    capture_rel: str,
) -> tuple[list[dict[str, Any]], list[Finding]]:
    manifest = ensure_dict(
        render_yaml_as_json(repo_root / manifest_rel), "fixture_manifest"
    )
    findings: list[Finding] = []
    results: list[dict[str, Any]] = []
    if manifest.get("record_kind") != "beta_durability_fixture_manifest":
        findings.append(
            Finding(
                "error",
                "durability.fixture_manifest.record_kind",
                "fixture manifest must use beta_durability_fixture_manifest",
                str(manifest.get("record_kind", "<fixture-manifest>")),
                "Use the durability fixture manifest discriminator.",
            )
        )
    expected_refs = {
        "canonical_packet_ref": packet_rel,
        "canonical_schema_ref": DEFAULT_SCHEMA_REL,
        "validator_ref": "tools/ci/m3/durability_gate",
    }
    for field, expected in expected_refs.items():
        if manifest.get(field) != expected:
            findings.append(
                Finding(
                    "error",
                    "durability.fixture_manifest.ref_mismatch",
                    f"{field} must match the canonical durability artifact",
                    str(manifest.get(field)),
                    f"Set {field} to {expected}.",
                )
            )
    mutations = ensure_list(
        manifest.get("failure_drill_mutations", []),
        "fixture_manifest.failure_drill_mutations",
    )
    if not mutations:
        findings.append(
            Finding(
                "error",
                "durability.fixture_manifest.mutations_missing",
                "fixture manifest must name failure-drill mutations",
                manifest_rel,
                "Add mutation rows for the expected durability rejection classes.",
            )
        )
    for mutation in mutations:
        mutation = ensure_dict(mutation, "fixture_manifest.failure_drill_mutations[]")
        mutation_id = str(mutation.get("mutation_id", ""))
        payload_path = str(mutation.get("payload_path", ""))
        expected_finding = str(mutation.get("expected_finding", ""))
        drill_today = today
        if mutation.get("today"):
            parsed = parse_iso_date(str(mutation.get("today")))
            if parsed is not None:
                drill_today = parsed
        mutated = copy.deepcopy(packet)
        case_findings: list[str] = []
        if not set_payload_path(mutated, payload_path, mutation.get("replacement")):
            case_findings.append("payload_path_unresolved")
        else:
            validator = Validator(
                repo_root=repo_root,
                packet=mutated,
                signing_quorum=signing_quorum,
                ownership_matrix=ownership_matrix,
                claim_row_ids=claim_row_ids,
                upstream_register=upstream_register,
                today=drill_today,
                packet_rel=packet_rel,
                doc_rel=doc_rel,
                csv_rel=csv_rel,
                signing_md_rel=signing_md_rel,
                capture_rel=capture_rel,
            )
            check_ids = {f.check_id for f in validator.validate(include_surfaces=False)}
            if expected_finding not in check_ids:
                case_findings.append(f"expected_finding_missing:{expected_finding}")
        status = "passed" if not case_findings else "failed"
        results.append(
            {
                "mutation_id": mutation_id,
                "target_ref": mutation.get("target_ref"),
                "expected_finding": expected_finding,
                "status": status,
                "findings": case_findings,
            }
        )
        for item in case_findings:
            findings.append(
                Finding(
                    "error",
                    "durability.fixture_manifest.mutation_failed",
                    f"durability fixture mutation failed: {item}",
                    mutation_id,
                    "Update the mutation path, expected finding, or validator rule.",
                )
            )
    return results, findings


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    packet = ensure_dict(load_json(repo_root / args.packet), "packet")
    schema = ensure_dict(load_json(repo_root / args.schema), "schema")
    signing_quorum = ensure_dict(
        render_yaml_as_json(repo_root / args.signing_quorum), "signing_quorum"
    )
    ownership_matrix = ensure_dict(
        render_yaml_as_json(repo_root / args.ownership_matrix), "ownership_matrix"
    )
    claim_manifest = ensure_dict(
        load_json(repo_root / args.claim_manifest), "claim_manifest"
    )
    upstream_register = ensure_dict(
        render_yaml_as_json(repo_root / args.upstream_register), "upstream_register"
    )
    claim_row_ids = {
        str(r.get("row_id"))
        for r in ensure_list(claim_manifest.get("rows", []), "claim_manifest.rows")
        if isinstance(r, dict)
    }

    if args.today:
        today = parse_iso_date(args.today)
        if today is None:
            raise SystemExit(f"invalid --today date: {args.today}")
    else:
        as_of = parse_iso_date(str(packet.get("as_of", ""))[:10])
        today = as_of or dt.date.today()

    findings: list[Finding] = []
    findings.extend(validate_against_schema(packet, schema, args.packet))

    validator = Validator(
        repo_root=repo_root,
        packet=packet,
        signing_quorum=signing_quorum,
        ownership_matrix=ownership_matrix,
        claim_row_ids=claim_row_ids,
        upstream_register=upstream_register,
        today=today,
        packet_rel=args.packet,
        doc_rel=args.doc,
        csv_rel=args.csv,
        signing_md_rel=args.signing_md,
        capture_rel=args.capture,
    )
    findings.extend(validator.validate())

    compare_or_write(
        repo_root / args.csv,
        build_critical_upstream_csv(packet),
        args.check,
        findings,
        "durability.generated.csv_stale",
    )
    compare_or_write(
        repo_root / args.signing_md,
        build_signing_md(packet),
        args.check,
        findings,
        "durability.generated.signing_md_stale",
    )

    fixture_results, fixture_findings = validate_fixture_manifest(
        repo_root=repo_root,
        manifest_rel=args.fixture_manifest,
        packet=packet,
        signing_quorum=signing_quorum,
        ownership_matrix=ownership_matrix,
        claim_row_ids=claim_row_ids,
        upstream_register=upstream_register,
        today=today,
        packet_rel=args.packet,
        doc_rel=args.doc,
        csv_rel=args.csv,
        signing_md_rel=args.signing_md,
        capture_rel=args.capture,
    )
    findings.extend(fixture_findings)

    capture = build_capture(
        packet=packet,
        findings=findings,
        fixture_results=fixture_results,
        today=today,
        packet_rel=args.packet,
    )
    compare_or_write(
        repo_root / args.capture,
        as_json_text(capture),
        args.check,
        findings,
        "durability.generated.capture_stale",
    )

    if any(f.severity == "error" for f in findings):
        for f in findings:
            if f.severity == "error":
                print(
                    f"{f.severity}: {f.check_id}: {f.message} [{f.ref}]",
                    file=sys.stderr,
                )
        return 1
    print("beta durability packet validation passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
