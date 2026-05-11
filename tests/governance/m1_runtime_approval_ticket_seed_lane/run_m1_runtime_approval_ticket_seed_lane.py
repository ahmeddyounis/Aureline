#!/usr/bin/env python3
"""Unattended M1 runtime approval-ticket seed validation lane.

Replays every row in ``artifacts/runtime/m1_runtime_approval_ticket_seed.yaml``
against:

- ``schemas/runtime/m1_runtime_approval_ticket_seed.schema.json`` —
  the envelope schema (vocabularies, required coverage, named
  consumers);
- ``schemas/runtime/approval_ticket.schema.json`` — the row
  vocabulary and the conditional invariants (authority-class /
  side-effect agreement, inner provider tickets, projection mode,
  attach target, step-up flag, preview / rollback refs,
  bounded-reuse counter, session-scoped forbiddens, requester-origin
  surface ref); and
- the canonical landing page at
  ``docs/runtime/m1_authority_model.md`` plus the upstream governance
  authority-ticket contract at
  ``docs/governance/runtime_authority_contract.md`` so the seed
  cannot quietly outlive its upstream.

Per-row assertions (every row):

- ``approval_ticket_profile_id`` is unique, non-empty, and matches
  the envelope pattern.
- ``authority_class``, ``side_effect_class``, ``issuer_class``,
  ``request_origin_class``, ``use_posture``, and
  ``revocation_posture_class`` are in their closed vocabularies.
- ``example_payload_ref`` resolves on disk under
  ``fixtures/runtime/approval_ticket_examples/`` and the loaded
  payload:
    - has ``record_kind == 'runtime_approval_ticket_record'`` and
      ``runtime_approval_ticket_schema_version == 1``;
    - pins the row's ``authority_class``, ``side_effect_class``,
      ``issuer_class``, ``request_origin_class``, ``use_posture``,
      and ``revocation_posture.posture_class`` (no drift between the
      seed row and its example payload);
    - obeys the row schema's conditional invariants:
        * ``authority_class = local_mutation`` ⇒ side-effect class in
          {``local_reversible_edit``, ``local_destructive_edit``};
        * ``authority_class = external_mutation`` ⇒ side-effect class
          in {``external_reversible_comment``,
          ``external_irreversible_publish``} AND
          ``inner_provider_approval_ticket_refs`` non-empty;
        * ``authority_class = credential_projection`` ⇒ side-effect
          class is ``credential_handle_projection`` AND
          ``projection_mode_ref`` non-empty;
        * ``authority_class = privileged_attach`` ⇒ side-effect class
          is ``privileged_inspection_attach`` AND
          ``attach_target_ref`` non-empty AND
          ``step_up_required_flag = true``;
        * ``side_effect_class`` in
          {``local_destructive_edit``,
          ``external_irreversible_publish``} ⇒ ``preview_ref``
          non-empty;
        * ``side_effect_class = local_destructive_edit`` ⇒
          ``rollback_checkpoint_ref`` non-empty;
        * ``use_posture = bounded_reuse`` ⇔ ``bounded_reuse_counter``
          present;
        * ``use_posture = session_scoped`` ⇒ side-effect class is not
          in {``external_irreversible_publish``,
          ``local_destructive_edit``};
        * ``request_origin_class`` outside the three issuer seats ⇒
          ``requesting_surface_ref`` non-empty.
- ``owner_dri`` is a non-empty ``@handle``.
- ``failure_drill`` is a non-null object whose ``drill_id`` is in
  ``failure_drill_id_vocabulary``, whose ``forced_input`` declares
  at least one drift, and whose ``expected_check_id`` and
  ``actionable_next_action`` are non-empty.

Envelope assertions:

- ``schema_version = 1``, ``matrix_id =
  m1_runtime_approval_ticket_seed``, ``status`` is non-empty,
  ``owner_dri`` is a ``@handle``.
- ``overview_page``, ``upstream_authority_contract_ref``,
  ``upstream_integration_approval_ticket_schema_ref``,
  ``row_schema_ref``, ``build_identity_ref``, and
  ``validation_lane_ref`` resolve on disk.
- Closed envelope vocabularies match the row schema $defs verbatim
  across the eleven shared axes (authority_class, side_effect_class,
  issuer_class, request_origin_class, actor_class, use_posture,
  revocation_posture_class, redaction_class, high_risk_flag,
  audit_event_id, denial_reason, invalidation_reason).
- Required coverage lists (authority classes, side-effect classes,
  request origins, revocation postures) are satisfied by the union
  of the seeded rows' payloads.
- Every ``named_runtime_consumers[].consumer_ref`` resolves on disk,
  the consumer_class is from the closed envelope vocabulary, and
  ``consumed_fields`` is non-empty.

``--force-drill <approval_ticket_profile_id>:<drill_id>`` replays the
named drill on the named row's example payload and exits 0 only
when the runner reproduces the declared ``expected_check_id``.
Drift in the unforced rows still fails the lane.

YAML decoding follows the repository convention: matrix files are
parsed via Ruby/Psych so this script does not require a third-party
Python YAML dependency.
"""

from __future__ import annotations

import argparse
import copy
import datetime as dt
import json
import re
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_MATRIX_REL = "artifacts/runtime/m1_runtime_approval_ticket_seed.yaml"
DEFAULT_ENVELOPE_SCHEMA_REL = (
    "schemas/runtime/m1_runtime_approval_ticket_seed.schema.json"
)
DEFAULT_ROW_SCHEMA_REL = "schemas/runtime/approval_ticket.schema.json"
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/"
    "runtime_approval_ticket_seed_validation_capture.json"
)

EXPECTED_RECORD_KIND = "runtime_approval_ticket_record"
EXPECTED_ROW_SCHEMA_VERSION = 1
EXPECTED_MATRIX_ID = "m1_runtime_approval_ticket_seed"
EXPECTED_OVERVIEW_PAGE = "docs/runtime/m1_authority_model.md"
EXPECTED_ROW_SCHEMA_REF = "schemas/runtime/approval_ticket.schema.json"

PROFILE_ID_PATTERN = re.compile(r"^[a-z0-9]+(?:[._-][a-z0-9]+)*$")
OWNER_DRI_PATTERN = re.compile(r"^@[a-zA-Z0-9_-]+$")

ISSUER_SEATS = frozenset({
    "user_shell_prompt",
    "policy_decision",
    "supervisor_control_path",
})

LOCAL_SIDE_EFFECTS = frozenset({
    "local_reversible_edit",
    "local_destructive_edit",
})
EXTERNAL_SIDE_EFFECTS = frozenset({
    "external_reversible_comment",
    "external_irreversible_publish",
})
SESSION_SCOPED_FORBIDDEN_SIDE_EFFECTS = frozenset({
    "external_irreversible_publish",
    "local_destructive_edit",
})
PREVIEW_REQUIRED_SIDE_EFFECTS = frozenset({
    "local_destructive_edit",
    "external_irreversible_publish",
})


@dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if payload["ref"] is None:
            payload.pop("ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


@dataclass
class RowResult:
    approval_ticket_profile_id: str
    authority_class: str
    side_effect_class: str
    issuer_class: str
    request_origin_class: str
    use_posture: str
    revocation_posture_class: str
    passed_checks: list[str] = field(default_factory=list)
    failed_checks: list[dict[str, str]] = field(default_factory=list)
    diagnostics: dict[str, Any] = field(default_factory=dict)


def fail(result: RowResult, check_id: str, message: str) -> None:
    result.failed_checks.append({"check_id": check_id, "message": message})


def pass_(result: RowResult, message: str) -> None:
    result.passed_checks.append(message)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Repository root (must contain a .git directory).",
    )
    parser.add_argument(
        "--matrix",
        default=DEFAULT_MATRIX_REL,
        help="Seed YAML path, repo-relative.",
    )
    parser.add_argument(
        "--envelope-schema",
        default=DEFAULT_ENVELOPE_SCHEMA_REL,
        help="Envelope schema JSON path, repo-relative.",
    )
    parser.add_argument(
        "--row-schema",
        default=DEFAULT_ROW_SCHEMA_REL,
        help="Row schema JSON path, repo-relative.",
    )
    parser.add_argument(
        "--build-identity",
        default=DEFAULT_BUILD_IDENTITY_REL,
        help="Build-identity record path the capture embeds.",
    )
    parser.add_argument(
        "--report",
        default=DEFAULT_REPORT_REL,
        help="Where to write the durable JSON capture.",
    )
    parser.add_argument(
        "--force-drill",
        default=None,
        help=(
            "Replay a named failure drill on a named row in the form "
            "'<approval_ticket_profile_id>:<drill_id>'. The runner exits 0 "
            "only when the row's failure drill reproduces the exact "
            "expected_check_id."
        ),
    )
    return parser.parse_args()


def render_yaml_as_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-rdate",
            "-rtime",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), "
                "permitted_classes: [Time, Date, DateTime], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(
            f"failed to parse YAML at {path} via Ruby/Psych: {stderr}"
        )
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(
            f"Ruby/Psych emitted invalid JSON for {path}: {exc}"
        ) from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a YAML mapping/object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a YAML list/array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    ref = ref.strip()
    path = ref.split("#", 1)[0].strip()
    if not path:
        return False
    return (repo_root / path).exists()


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def load_schema_enum(repo_root: Path, ref: str, defs_key: str) -> list[str]:
    schema_path = repo_root / ref
    if not schema_path.exists():
        return []
    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    defs = schema.get("$defs", {})
    entry = defs.get(defs_key, {})
    if "enum" in entry and isinstance(entry["enum"], list):
        return [str(v) for v in entry["enum"]]
    return []


def load_payload(repo_root: Path, ref: str) -> dict[str, Any]:
    path = repo_root / ref
    if not path.exists():
        raise FileNotFoundError(ref)
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise ValueError(f"invalid JSON at {ref}: {exc}") from exc


def apply_forced_overrides(
    payload: dict[str, Any], forced_overrides: dict[str, Any]
) -> dict[str, Any]:
    payload = copy.deepcopy(payload)
    if not forced_overrides:
        return payload

    if forced_overrides.get("clear_preview_ref"):
        payload["preview_ref"] = ""

    if forced_overrides.get("clear_rollback_checkpoint_ref"):
        payload["rollback_checkpoint_ref"] = ""

    if forced_overrides.get("clear_inner_provider_approval_ticket_refs"):
        payload["inner_provider_approval_ticket_refs"] = []

    if forced_overrides.get("clear_projection_mode_ref"):
        payload["projection_mode_ref"] = ""

    if forced_overrides.get("clear_attach_target_ref"):
        payload["attach_target_ref"] = ""

    if "rewrite_step_up_required_flag_to_false" in forced_overrides:
        payload["step_up_required_flag"] = False

    if forced_overrides.get("clear_requesting_surface_ref"):
        payload["requesting_surface_ref"] = ""

    if "rewrite_use_posture" in forced_overrides:
        payload["use_posture"] = forced_overrides["rewrite_use_posture"]
        if payload["use_posture"] != "bounded_reuse":
            payload.pop("bounded_reuse_counter", None)

    return payload


def validate_payload(
    payload: dict[str, Any],
    *,
    row_authority_class: str,
    row_side_effect_class: str,
    row_issuer_class: str,
    row_request_origin_class: str,
    row_use_posture: str,
    row_revocation_posture_class: str,
    authority_class_vocab: set[str],
    side_effect_class_vocab: set[str],
    issuer_class_vocab: set[str],
    request_origin_class_vocab: set[str],
    actor_class_vocab: set[str],
    use_posture_vocab: set[str],
    revocation_posture_vocab: set[str],
    redaction_class_vocab: set[str],
    high_risk_flag_vocab: set[str],
    audit_event_id_vocab: set[str],
    denial_reason_vocab: set[str],
    invalidation_reason_vocab: set[str],
    result: RowResult,
) -> None:
    # ---- discriminator + version pin ------------------------------------
    if payload.get("record_kind") != EXPECTED_RECORD_KIND:
        fail(
            result,
            "runtime_approval_ticket.record_kind_wrong",
            (
                f"record_kind must be {EXPECTED_RECORD_KIND!r}; got "
                f"{payload.get('record_kind')!r}"
            ),
        )
    if (
        payload.get("runtime_approval_ticket_schema_version")
        != EXPECTED_ROW_SCHEMA_VERSION
    ):
        fail(
            result,
            "runtime_approval_ticket.schema_version_wrong",
            (
                "runtime_approval_ticket_schema_version must be "
                f"{EXPECTED_ROW_SCHEMA_VERSION}; got "
                f"{payload.get('runtime_approval_ticket_schema_version')!r}"
            ),
        )

    # ---- closed scalar axes ---------------------------------------------
    closed_scalar_axes = [
        ("authority_class", authority_class_vocab),
        ("side_effect_class", side_effect_class_vocab),
        ("issuer_class", issuer_class_vocab),
        ("request_origin_class", request_origin_class_vocab),
        ("actor_class", actor_class_vocab),
        ("use_posture", use_posture_vocab),
        ("redaction_class", redaction_class_vocab),
    ]
    for field_name, vocab in closed_scalar_axes:
        value = payload.get(field_name)
        if not isinstance(value, str) or not value.strip():
            fail(
                result,
                f"runtime_approval_ticket.{field_name}_required",
                (
                    f"{field_name} must be a non-empty member of the "
                    "closed vocabulary"
                ),
            )
        elif value not in vocab:
            fail(
                result,
                f"runtime_approval_ticket.{field_name}_unknown",
                (
                    f"{field_name} {value!r} is not in the row schema's "
                    f"{field_name} enum"
                ),
            )

    # ---- payload-vs-row agreement ---------------------------------------
    pinned_axes = [
        ("authority_class", row_authority_class),
        ("side_effect_class", row_side_effect_class),
        ("issuer_class", row_issuer_class),
        ("request_origin_class", row_request_origin_class),
        ("use_posture", row_use_posture),
    ]
    for axis, expected in pinned_axes:
        observed = payload.get(axis)
        if observed != expected:
            fail(
                result,
                f"runtime_approval_ticket.payload_pinned_{axis}_mismatch",
                (
                    f"payload {axis} {observed!r} disagrees with the seed "
                    f"row's pinned value {expected!r}; the example payload "
                    "MUST agree with the row it lives under"
                ),
            )

    # ---- revocation_posture agreement -----------------------------------
    rp = payload.get("revocation_posture")
    if not isinstance(rp, dict):
        fail(
            result,
            "runtime_approval_ticket.revocation_posture_required",
            "revocation_posture must be an object",
        )
    else:
        posture_class = rp.get("posture_class")
        if not isinstance(posture_class, str) or not posture_class.strip():
            fail(
                result,
                "runtime_approval_ticket.revocation_posture_class_required",
                "revocation_posture.posture_class must be non-empty",
            )
        elif posture_class not in revocation_posture_vocab:
            fail(
                result,
                "runtime_approval_ticket.revocation_posture_class_unknown",
                (
                    f"revocation_posture.posture_class {posture_class!r} is "
                    "not in the row schema's revocation_posture_class enum"
                ),
            )
        elif posture_class != row_revocation_posture_class:
            fail(
                result,
                "runtime_approval_ticket.payload_pinned_revocation_posture_class_mismatch",
                (
                    "payload revocation_posture.posture_class "
                    f"{posture_class!r} disagrees with the seed row's "
                    f"pinned value {row_revocation_posture_class!r}"
                ),
            )
        last_reason = rp.get("last_invalidation_reason", "")
        if isinstance(last_reason, str) and last_reason:
            if last_reason not in invalidation_reason_vocab:
                fail(
                    result,
                    "runtime_approval_ticket.last_invalidation_reason_unknown",
                    (
                        "revocation_posture.last_invalidation_reason "
                        f"{last_reason!r} is not in the row schema's "
                        "invalidation_reason enum"
                    ),
                )

    # ---- high_risk_flags / audit_event_id -------------------------------
    flags = payload.get("high_risk_flags", [])
    if isinstance(flags, list):
        for flag in flags:
            if not isinstance(flag, str) or flag not in high_risk_flag_vocab:
                fail(
                    result,
                    "runtime_approval_ticket.high_risk_flag_unknown",
                    (
                        f"high_risk_flags entry {flag!r} is not in the "
                        "row schema's high_risk_flag enum"
                    ),
                )
    audit_metadata = payload.get("audit_metadata")
    if not isinstance(audit_metadata, dict):
        fail(
            result,
            "runtime_approval_ticket.audit_metadata_required",
            "audit_metadata must be an object",
        )
    else:
        aei = audit_metadata.get("audit_event_id")
        if aei not in audit_event_id_vocab:
            fail(
                result,
                "runtime_approval_ticket.audit_event_id_unknown",
                (
                    f"audit_metadata.audit_event_id {aei!r} is not in the "
                    "row schema's audit_event_id enum"
                ),
            )

    # ---- denial_reason --------------------------------------------------
    denial_reason = payload.get("denial_reason", "")
    if isinstance(denial_reason, str) and denial_reason:
        if denial_reason not in denial_reason_vocab:
            fail(
                result,
                "runtime_approval_ticket.denial_reason_unknown",
                (
                    f"denial_reason {denial_reason!r} is not in the row "
                    "schema's denial_reason enum"
                ),
            )

    # ---- conditional invariants -----------------------------------------
    ac = payload.get("authority_class")
    sec = payload.get("side_effect_class")

    if ac == "local_mutation":
        if sec not in LOCAL_SIDE_EFFECTS:
            fail(
                result,
                "runtime_approval_ticket.local_mutation_requires_local_side_effect",
                (
                    "authority_class = local_mutation forces side_effect_class "
                    f"in {sorted(LOCAL_SIDE_EFFECTS)}; got {sec!r}"
                ),
            )
        if payload.get("projection_mode_ref"):
            fail(
                result,
                "runtime_approval_ticket.projection_mode_ref_forbidden_for_non_credential_projection",
                (
                    "projection_mode_ref must be empty unless "
                    "authority_class is credential_projection"
                ),
            )
        if payload.get("attach_target_ref"):
            fail(
                result,
                "runtime_approval_ticket.attach_target_ref_forbidden_for_non_privileged_attach",
                (
                    "attach_target_ref must be empty unless "
                    "authority_class is privileged_attach"
                ),
            )
        inner = payload.get("inner_provider_approval_ticket_refs", [])
        if isinstance(inner, list) and inner:
            fail(
                result,
                "runtime_approval_ticket.inner_provider_approval_ticket_refs_forbidden_for_non_external_mutation",
                (
                    "inner_provider_approval_ticket_refs must be empty "
                    "unless authority_class is external_mutation"
                ),
            )

    elif ac == "external_mutation":
        if sec not in EXTERNAL_SIDE_EFFECTS:
            fail(
                result,
                "runtime_approval_ticket.external_mutation_requires_external_side_effect",
                (
                    "authority_class = external_mutation forces "
                    "side_effect_class in "
                    f"{sorted(EXTERNAL_SIDE_EFFECTS)}; got {sec!r}"
                ),
            )
        inner = payload.get("inner_provider_approval_ticket_refs")
        if not isinstance(inner, list) or not inner:
            fail(
                result,
                "runtime_approval_ticket.inner_provider_approval_ticket_required_for_external_mutation",
                (
                    "authority_class = external_mutation requires at least "
                    "one inner_provider_approval_ticket_refs entry so the "
                    "provider-plane approval ticket is admitted under the "
                    "runtime authority ticket"
                ),
            )

    elif ac == "credential_projection":
        if sec != "credential_handle_projection":
            fail(
                result,
                "runtime_approval_ticket.credential_projection_requires_credential_handle_projection",
                (
                    "authority_class = credential_projection forces "
                    "side_effect_class = 'credential_handle_projection'; "
                    f"got {sec!r}"
                ),
            )
        proj = payload.get("projection_mode_ref")
        if not isinstance(proj, str) or not proj.strip():
            fail(
                result,
                "runtime_approval_ticket.projection_mode_ref_required_for_credential_projection",
                (
                    "authority_class = credential_projection requires a "
                    "non-empty projection_mode_ref so the projection mode "
                    "is named, scoped, and auditable"
                ),
            )

    elif ac == "privileged_attach":
        if sec != "privileged_inspection_attach":
            fail(
                result,
                "runtime_approval_ticket.privileged_attach_requires_privileged_inspection_attach",
                (
                    "authority_class = privileged_attach forces "
                    "side_effect_class = 'privileged_inspection_attach'; "
                    f"got {sec!r}"
                ),
            )
        attach_target = payload.get("attach_target_ref")
        if not isinstance(attach_target, str) or not attach_target.strip():
            fail(
                result,
                "runtime_approval_ticket.attach_target_ref_required_for_privileged_attach",
                (
                    "authority_class = privileged_attach requires a "
                    "non-empty attach_target_ref"
                ),
            )
        if not payload.get("step_up_required_flag"):
            fail(
                result,
                "runtime_approval_ticket.step_up_required_flag_must_be_true_for_privileged_attach",
                (
                    "authority_class = privileged_attach requires "
                    "step_up_required_flag = true so a stale prompt "
                    "cannot mint a debugger attach"
                ),
            )

    # ---- preview_ref / rollback_checkpoint_ref --------------------------
    if sec in PREVIEW_REQUIRED_SIDE_EFFECTS:
        pv = payload.get("preview_ref")
        if not isinstance(pv, str) or not pv.strip():
            fail(
                result,
                "runtime_approval_ticket.preview_ref_required_for_destructive_or_irreversible_side_effect",
                (
                    "side_effect_class "
                    f"{sec!r} requires a non-empty preview_ref so the "
                    "user re-confirms the preview before the destructive "
                    "or irreversible action is spendable"
                ),
            )

    if sec == "local_destructive_edit":
        rc = payload.get("rollback_checkpoint_ref")
        if not isinstance(rc, str) or not rc.strip():
            fail(
                result,
                "runtime_approval_ticket.rollback_checkpoint_ref_required_for_local_destructive_edit",
                (
                    "side_effect_class = 'local_destructive_edit' requires "
                    "a non-empty rollback_checkpoint_ref"
                ),
            )

    # ---- use_posture / session_scoped forbiddens ------------------------
    up = payload.get("use_posture")
    if up == "bounded_reuse":
        if not isinstance(payload.get("bounded_reuse_counter"), dict):
            fail(
                result,
                "runtime_approval_ticket.bounded_reuse_counter_required_for_bounded_reuse",
                (
                    "use_posture = 'bounded_reuse' requires a "
                    "bounded_reuse_counter object"
                ),
            )
    else:
        if payload.get("bounded_reuse_counter") is not None and (
            isinstance(payload.get("bounded_reuse_counter"), dict)
        ):
            fail(
                result,
                "runtime_approval_ticket.bounded_reuse_counter_forbidden_outside_bounded_reuse",
                (
                    "bounded_reuse_counter is only allowed when "
                    "use_posture = 'bounded_reuse'"
                ),
            )

    if up == "session_scoped" and sec in SESSION_SCOPED_FORBIDDEN_SIDE_EFFECTS:
        fail(
            result,
            "runtime_approval_ticket.session_scoped_forbidden_for_destructive_or_irreversible_side_effect",
            (
                "use_posture = 'session_scoped' is forbidden when "
                f"side_effect_class is {sec!r}"
            ),
        )

    # ---- requester-origin surface ref -----------------------------------
    roc = payload.get("request_origin_class")
    if isinstance(roc, str) and roc not in ISSUER_SEATS:
        rsr = payload.get("requesting_surface_ref")
        if not isinstance(rsr, str) or not rsr.strip():
            fail(
                result,
                "runtime_approval_ticket.requesting_surface_ref_required_for_non_issuer_origin",
                (
                    "request_origin_class "
                    f"{roc!r} requires a non-empty requesting_surface_ref "
                    "so the requesting surface's lineage is named, "
                    "scoped, and auditable"
                ),
            )


def validate_row(
    row: dict[str, Any],
    *,
    repo_root: Path,
    authority_class_vocab: set[str],
    side_effect_class_vocab: set[str],
    issuer_class_vocab: set[str],
    request_origin_class_vocab: set[str],
    actor_class_vocab: set[str],
    use_posture_vocab: set[str],
    revocation_posture_vocab: set[str],
    redaction_class_vocab: set[str],
    high_risk_flag_vocab: set[str],
    audit_event_id_vocab: set[str],
    denial_reason_vocab: set[str],
    invalidation_reason_vocab: set[str],
    failure_drill_id_vocab: set[str],
    forced_overrides: dict[str, Any] | None = None,
) -> tuple[RowResult, dict[str, Any] | None]:
    profile_id = ensure_str(
        row.get("approval_ticket_profile_id"),
        "matrix.entries[].approval_ticket_profile_id",
    )

    ac = (
        row.get("authority_class")
        if isinstance(row.get("authority_class"), str)
        else ""
    )
    sec = (
        row.get("side_effect_class")
        if isinstance(row.get("side_effect_class"), str)
        else ""
    )
    ic = (
        row.get("issuer_class")
        if isinstance(row.get("issuer_class"), str)
        else ""
    )
    roc = (
        row.get("request_origin_class")
        if isinstance(row.get("request_origin_class"), str)
        else ""
    )
    up = (
        row.get("use_posture")
        if isinstance(row.get("use_posture"), str)
        else ""
    )
    rpc = (
        row.get("revocation_posture_class")
        if isinstance(row.get("revocation_posture_class"), str)
        else ""
    )

    result = RowResult(
        approval_ticket_profile_id=profile_id,
        authority_class=ac,
        side_effect_class=sec,
        issuer_class=ic,
        request_origin_class=roc,
        use_posture=up,
        revocation_posture_class=rpc,
    )

    # --- profile_id pattern ---------------------------------------------
    if not PROFILE_ID_PATTERN.match(profile_id):
        fail(
            result,
            "runtime_approval_ticket.approval_ticket_profile_id_pattern_invalid",
            (
                f"approval_ticket_profile_id {profile_id!r} does not match "
                f"{PROFILE_ID_PATTERN.pattern!r}"
            ),
        )

    # --- closed scalar axes on the row ----------------------------------
    closed_axes = [
        ("authority_class", ac, authority_class_vocab),
        ("side_effect_class", sec, side_effect_class_vocab),
        ("issuer_class", ic, issuer_class_vocab),
        ("request_origin_class", roc, request_origin_class_vocab),
        ("use_posture", up, use_posture_vocab),
        ("revocation_posture_class", rpc, revocation_posture_vocab),
    ]
    for axis, value, vocab in closed_axes:
        if not value:
            fail(
                result,
                f"runtime_approval_ticket.row_{axis}_required",
                (
                    f"row {axis} must be a non-empty member of the "
                    "closed vocabulary"
                ),
            )
        elif value not in vocab:
            fail(
                result,
                f"runtime_approval_ticket.row_{axis}_unknown",
                (
                    f"row {axis} {value!r} is not in the seed envelope's "
                    f"{axis} vocabulary"
                ),
            )

    # --- owner_dri ------------------------------------------------------
    owner_dri = row.get("owner_dri")
    if not isinstance(owner_dri, str) or not owner_dri.strip():
        fail(
            result,
            "runtime_approval_ticket.owner_dri_required",
            "owner_dri must be a non-empty @handle",
        )
    elif not OWNER_DRI_PATTERN.match(owner_dri):
        fail(
            result,
            "runtime_approval_ticket.owner_dri_pattern_invalid",
            (
                f"owner_dri {owner_dri!r} must match "
                f"{OWNER_DRI_PATTERN.pattern!r}"
            ),
        )

    # --- failure_drill --------------------------------------------------
    drill = row.get("failure_drill")
    if not isinstance(drill, dict):
        fail(
            result,
            "runtime_approval_ticket.failure_drill_required",
            "failure_drill must be a non-null object on every row",
        )
    else:
        drill_id = drill.get("drill_id")
        if not isinstance(drill_id, str) or not drill_id.strip():
            fail(
                result,
                "runtime_approval_ticket.failure_drill_drill_id_required",
                "failure_drill.drill_id must be a non-empty string",
            )
        elif drill_id not in failure_drill_id_vocab:
            fail(
                result,
                "runtime_approval_ticket.failure_drill_drill_id_unknown",
                (
                    f"failure_drill.drill_id {drill_id!r} is not in "
                    "failure_drill_id_vocabulary"
                ),
            )
        forced_input = drill.get("forced_input")
        if not isinstance(forced_input, dict) or not forced_input:
            fail(
                result,
                "runtime_approval_ticket.failure_drill_forced_input_empty",
                "failure_drill.forced_input must declare at least one drift",
            )
        expected_check = drill.get("expected_check_id")
        if (
            not isinstance(expected_check, str)
            or not expected_check.strip()
        ):
            fail(
                result,
                "runtime_approval_ticket.failure_drill_expected_check_id_required",
                "failure_drill.expected_check_id must be non-empty",
            )
        actionable = drill.get("actionable_next_action")
        if not isinstance(actionable, str) or not actionable.strip():
            fail(
                result,
                "runtime_approval_ticket.failure_drill_actionable_next_action_required",
                "failure_drill.actionable_next_action must be non-empty",
            )

    # --- example_payload_ref --------------------------------------------
    payload_ref = row.get("example_payload_ref")
    if not isinstance(payload_ref, str) or not payload_ref.strip():
        fail(
            result,
            "runtime_approval_ticket.example_payload_ref_required",
            "example_payload_ref must be a non-empty string",
        )
        result.diagnostics["row_summary"] = {
            "authority_class": ac,
            "side_effect_class": sec,
            "issuer_class": ic,
            "request_origin_class": roc,
            "use_posture": up,
            "revocation_posture_class": rpc,
        }
        if not result.failed_checks:
            pass_(result, f"row {profile_id} passes")
        return result, None

    if not payload_ref.startswith("fixtures/runtime/approval_ticket_examples/"):
        fail(
            result,
            "runtime_approval_ticket.example_payload_ref_outside_governed_root",
            (
                "example_payload_ref must live under "
                "fixtures/runtime/approval_ticket_examples/; got "
                f"{payload_ref!r}"
            ),
        )

    try:
        payload = load_payload(repo_root, payload_ref)
    except FileNotFoundError:
        fail(
            result,
            "runtime_approval_ticket.example_payload_missing",
            f"example_payload_ref does not resolve on disk: {payload_ref}",
        )
        if not result.failed_checks:
            pass_(result, f"row {profile_id} passes")
        return result, None
    except ValueError as exc:
        fail(
            result,
            "runtime_approval_ticket.example_payload_invalid_json",
            str(exc),
        )
        if not result.failed_checks:
            pass_(result, f"row {profile_id} passes")
        return result, None

    replay_payload = (
        apply_forced_overrides(payload, forced_overrides)
        if forced_overrides
        else payload
    )

    validate_payload(
        replay_payload,
        row_authority_class=ac,
        row_side_effect_class=sec,
        row_issuer_class=ic,
        row_request_origin_class=roc,
        row_use_posture=up,
        row_revocation_posture_class=rpc,
        authority_class_vocab=authority_class_vocab,
        side_effect_class_vocab=side_effect_class_vocab,
        issuer_class_vocab=issuer_class_vocab,
        request_origin_class_vocab=request_origin_class_vocab,
        actor_class_vocab=actor_class_vocab,
        use_posture_vocab=use_posture_vocab,
        revocation_posture_vocab=revocation_posture_vocab,
        redaction_class_vocab=redaction_class_vocab,
        high_risk_flag_vocab=high_risk_flag_vocab,
        audit_event_id_vocab=audit_event_id_vocab,
        denial_reason_vocab=denial_reason_vocab,
        invalidation_reason_vocab=invalidation_reason_vocab,
        result=result,
    )

    result.diagnostics["row_summary"] = {
        "authority_class": ac,
        "side_effect_class": sec,
        "issuer_class": ic,
        "request_origin_class": roc,
        "use_posture": up,
        "revocation_posture_class": rpc,
        "example_payload_ref": payload_ref,
    }
    if forced_overrides:
        result.diagnostics["forced_overrides_applied"] = forced_overrides

    if not result.failed_checks:
        pass_(result, f"row {profile_id} passes")

    return result, payload


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    matrix_rel = args.matrix
    matrix = ensure_dict(
        render_yaml_as_json(repo_root / matrix_rel), matrix_rel
    )

    findings: list[Finding] = []

    schema_version = matrix.get("schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="runtime_approval_ticket.envelope_schema_version_wrong",
                message=(
                    f"matrix schema_version must be 1; got {schema_version!r}"
                ),
                remediation="Bump runner together with the envelope schema.",
            )
        )

    matrix_id = ensure_str(matrix.get("matrix_id"), "matrix.matrix_id")
    if matrix_id != EXPECTED_MATRIX_ID:
        findings.append(
            Finding(
                severity="error",
                check_id="runtime_approval_ticket.envelope_matrix_id_wrong",
                message=(
                    f"matrix_id must be {EXPECTED_MATRIX_ID!r}; got "
                    f"{matrix_id!r}"
                ),
                remediation="Restore the canonical envelope matrix id.",
            )
        )

    ensure_str(matrix.get("status"), "matrix.status")
    owner_dri = ensure_str(matrix.get("owner_dri"), "matrix.owner_dri")
    if not OWNER_DRI_PATTERN.match(owner_dri):
        findings.append(
            Finding(
                severity="error",
                check_id="runtime_approval_ticket.envelope_owner_dri_pattern_invalid",
                message=(
                    f"owner_dri {owner_dri!r} must match "
                    f"{OWNER_DRI_PATTERN.pattern!r}"
                ),
                remediation="Use an @handle for the owner DRI.",
            )
        )

    overview_page = ensure_str(
        matrix.get("overview_page"), "matrix.overview_page"
    )
    if overview_page != EXPECTED_OVERVIEW_PAGE:
        findings.append(
            Finding(
                severity="error",
                check_id="runtime_approval_ticket.envelope_overview_page_wrong",
                message=(
                    f"overview_page must be {EXPECTED_OVERVIEW_PAGE!r}; "
                    f"got {overview_page!r}"
                ),
                remediation="Restore the canonical landing page path.",
            )
        )
    if not artifact_ref_exists(repo_root, overview_page):
        findings.append(
            Finding(
                severity="error",
                check_id="runtime_approval_ticket.envelope_overview_page_missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation=(
                    "Create the reviewer landing page or fix the path."
                ),
                ref=overview_page,
            )
        )

    upstream_authority_contract_ref = ensure_str(
        matrix.get("upstream_authority_contract_ref"),
        "matrix.upstream_authority_contract_ref",
    )
    if not artifact_ref_exists(repo_root, upstream_authority_contract_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="runtime_approval_ticket.envelope_upstream_authority_contract_ref_missing",
                message=(
                    "upstream_authority_contract_ref does not resolve: "
                    f"{upstream_authority_contract_ref}"
                ),
                remediation=(
                    "Fix the path or land the upstream authority-ticket "
                    "contract."
                ),
                ref=upstream_authority_contract_ref,
            )
        )

    upstream_provider_ticket_ref = ensure_str(
        matrix.get("upstream_integration_approval_ticket_schema_ref"),
        "matrix.upstream_integration_approval_ticket_schema_ref",
    )
    if not artifact_ref_exists(repo_root, upstream_provider_ticket_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="runtime_approval_ticket.envelope_upstream_provider_ticket_ref_missing",
                message=(
                    "upstream_integration_approval_ticket_schema_ref does "
                    f"not resolve: {upstream_provider_ticket_ref}"
                ),
                remediation=(
                    "Fix the path or land the upstream provider-plane "
                    "approval-ticket schema."
                ),
                ref=upstream_provider_ticket_ref,
            )
        )

    for key in ("row_schema_ref", "build_identity_ref"):
        ref = ensure_str(matrix.get(key), f"matrix.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"runtime_approval_ticket.envelope_{key}_missing",
                    message=f"{key} does not resolve: {ref}",
                    remediation=(
                        "Fix the path or land the referenced artifact."
                    ),
                    ref=ref,
                )
            )

    row_schema_ref = ensure_str(
        matrix.get("row_schema_ref"), "matrix.row_schema_ref"
    )
    if row_schema_ref != EXPECTED_ROW_SCHEMA_REF:
        findings.append(
            Finding(
                severity="error",
                check_id="runtime_approval_ticket.envelope_row_schema_ref_wrong",
                message=(
                    f"row_schema_ref must be {EXPECTED_ROW_SCHEMA_REF!r}; "
                    f"got {row_schema_ref!r}"
                ),
                remediation="Restore the canonical row schema path.",
            )
        )

    validation_lane_ref = ensure_str(
        matrix.get("validation_lane_ref"), "matrix.validation_lane_ref"
    )
    if not artifact_ref_exists(repo_root, validation_lane_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="runtime_approval_ticket.envelope_validation_lane_ref_missing",
                message=(
                    "validation_lane_ref base does not exist: "
                    f"{validation_lane_ref}"
                ),
                remediation="Point at a seeded QE lane registry entry.",
                ref=validation_lane_ref,
            )
        )

    def load_vocab(key: str) -> set[str]:
        return {
            ensure_str(item, f"matrix.{key}[]")
            for item in ensure_list(matrix.get(key), f"matrix.{key}")
        }

    authority_class_vocab = load_vocab("authority_class_vocabulary")
    side_effect_class_vocab = load_vocab("side_effect_class_vocabulary")
    issuer_class_vocab = load_vocab("issuer_class_vocabulary")
    request_origin_class_vocab = load_vocab(
        "request_origin_class_vocabulary"
    )
    actor_class_vocab = load_vocab("actor_class_vocabulary")
    use_posture_vocab = load_vocab("use_posture_class_vocabulary")
    revocation_posture_vocab = load_vocab(
        "revocation_posture_class_vocabulary"
    )
    redaction_class_vocab = load_vocab("redaction_class_vocabulary")
    high_risk_flag_vocab = load_vocab("high_risk_flag_vocabulary")
    denial_reason_vocab = load_vocab("denial_reason_vocabulary")
    invalidation_reason_vocab = load_vocab(
        "invalidation_reason_vocabulary"
    )
    audit_event_id_vocab = load_vocab("audit_event_id_vocabulary")
    failure_drill_id_vocab = load_vocab("failure_drill_id_vocabulary")
    required_authority_class_coverage = load_vocab(
        "required_authority_class_coverage"
    )
    required_side_effect_class_coverage = load_vocab(
        "required_side_effect_class_coverage"
    )
    required_request_origin_class_coverage = load_vocab(
        "required_request_origin_class_coverage"
    )
    required_revocation_posture_class_coverage = load_vocab(
        "required_revocation_posture_class_coverage"
    )

    vocab_agreements = [
        ("authority_class_vocabulary", "authority_class", authority_class_vocab),
        ("side_effect_class_vocabulary", "side_effect_class", side_effect_class_vocab),
        ("issuer_class_vocabulary", "issuer_class", issuer_class_vocab),
        (
            "request_origin_class_vocabulary",
            "request_origin_class",
            request_origin_class_vocab,
        ),
        ("actor_class_vocabulary", "actor_class", actor_class_vocab),
        ("use_posture_class_vocabulary", "use_posture", use_posture_vocab),
        (
            "revocation_posture_class_vocabulary",
            "revocation_posture_class",
            revocation_posture_vocab,
        ),
        ("redaction_class_vocabulary", "redaction_class", redaction_class_vocab),
        ("high_risk_flag_vocabulary", "high_risk_flag", high_risk_flag_vocab),
        ("audit_event_id_vocabulary", "audit_event_id", audit_event_id_vocab),
        ("denial_reason_vocabulary", "denial_reason", denial_reason_vocab),
        (
            "invalidation_reason_vocabulary",
            "invalidation_reason",
            invalidation_reason_vocab,
        ),
    ]
    for envelope_key, defs_key, envelope_vocab in vocab_agreements:
        schema_enum = set(
            load_schema_enum(repo_root, row_schema_ref, defs_key)
        )
        if not schema_enum:
            continue
        diff = envelope_vocab.symmetric_difference(schema_enum)
        if diff:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        "runtime_approval_ticket.envelope_"
                        f"{envelope_key}_disagrees_with_row_schema"
                    ),
                    message=(
                        f"matrix.{envelope_key} disagrees with "
                        f"{row_schema_ref}#$defs.{defs_key}; "
                        f"matrix-only: {sorted(envelope_vocab - schema_enum)}; "
                        f"schema-only: {sorted(schema_enum - envelope_vocab)}"
                    ),
                    remediation=(
                        "Keep the matrix vocabulary in lock-step with the "
                        "row schema; the schema is canonical."
                    ),
                )
            )

    # --- named runtime consumers ----------------------------------------
    consumers = ensure_list(
        matrix.get("named_runtime_consumers"),
        "matrix.named_runtime_consumers",
    )
    if not consumers:
        findings.append(
            Finding(
                severity="error",
                check_id="runtime_approval_ticket.envelope_named_runtime_consumers_empty",
                message=(
                    "named_runtime_consumers must declare at least one "
                    "consumer"
                ),
                remediation=(
                    "Add at least one named runtime consumer that reads "
                    "the seed."
                ),
            )
        )
    for idx, consumer in enumerate(consumers):
        consumer = ensure_dict(
            consumer, f"matrix.named_runtime_consumers[{idx}]"
        )
        ensure_str(
            consumer.get("consumer_id"),
            f"matrix.named_runtime_consumers[{idx}].consumer_id",
        )
        consumer_ref = ensure_str(
            consumer.get("consumer_ref"),
            f"matrix.named_runtime_consumers[{idx}].consumer_ref",
        )
        if not artifact_ref_exists(repo_root, consumer_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="runtime_approval_ticket.named_runtime_consumer_ref_missing",
                    message=(
                        f"named_runtime_consumers[{idx}].consumer_ref does "
                        f"not exist: {consumer_ref}"
                    ),
                    remediation=(
                        "Fix the path or land the referenced consumer "
                        "before claiming it as live."
                    ),
                    ref=consumer_ref,
                )
            )
        consumed_fields = consumer.get("consumed_fields")
        if (
            not isinstance(consumed_fields, list)
            or not consumed_fields
        ):
            findings.append(
                Finding(
                    severity="error",
                    check_id="runtime_approval_ticket.named_runtime_consumer_consumed_fields_empty",
                    message=(
                        f"named_runtime_consumers[{idx}].consumed_fields "
                        "must be a non-empty list"
                    ),
                    remediation=(
                        "Name at least one field the consumer reads so "
                        "the consumer cannot regress to mentioned-but-"
                        "unread."
                    ),
                )
            )

    # --- --force-drill plumbing -----------------------------------------
    forced_profile_id: str | None = None
    forced_drill_id: str | None = None
    if args.force_drill:
        if ":" not in args.force_drill:
            raise SystemExit(
                "--force-drill must be of the form "
                "'<approval_ticket_profile_id>:<drill_id>'"
            )
        forced_profile_id, forced_drill_id = args.force_drill.rsplit(":", 1)
        forced_profile_id = forced_profile_id.strip()
        forced_drill_id = forced_drill_id.strip()

    rows = ensure_list(matrix.get("entries"), "matrix.entries")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="runtime_approval_ticket.envelope_entries_empty",
                message="matrix.entries must declare at least one row",
                remediation="Seed at least one approval-ticket profile row.",
            )
        )

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_authority_classes: set[str] = set()
    seen_side_effect_classes: set[str] = set()
    seen_request_origin_classes: set[str] = set()
    seen_revocation_postures: set[str] = set()
    forced_replay_record: dict[str, Any] | None = None

    for idx, raw_row in enumerate(rows):
        raw_row = ensure_dict(raw_row, f"matrix.entries[{idx}]")
        profile_id_local = ensure_str(
            raw_row.get("approval_ticket_profile_id"),
            f"matrix.entries[{idx}].approval_ticket_profile_id",
        )

        forced_overrides: dict[str, Any] = {}
        drill_local: dict[str, Any] | None = None
        if (
            forced_profile_id is not None
            and profile_id_local == forced_profile_id
        ):
            drill_local = raw_row.get("failure_drill")
            if not isinstance(drill_local, dict):
                raise SystemExit(
                    f"--force-drill targeted approval_ticket_profile_id "
                    f"{forced_profile_id!r} but the row has no failure_drill"
                )
            drill_id_local = drill_local.get("drill_id")
            if drill_id_local != forced_drill_id:
                raise SystemExit(
                    f"--force-drill drill_id {forced_drill_id!r} does not "
                    f"match the row's failure_drill.drill_id "
                    f"{drill_id_local!r}"
                )
            forced_input_local = drill_local.get("forced_input")
            if not isinstance(forced_input_local, dict):
                raise SystemExit(
                    f"failure_drill.forced_input must be an object on row "
                    f"{forced_profile_id!r}"
                )
            forced_overrides = forced_input_local

        result, payload = validate_row(
            raw_row,
            repo_root=repo_root,
            authority_class_vocab=authority_class_vocab,
            side_effect_class_vocab=side_effect_class_vocab,
            issuer_class_vocab=issuer_class_vocab,
            request_origin_class_vocab=request_origin_class_vocab,
            actor_class_vocab=actor_class_vocab,
            use_posture_vocab=use_posture_vocab,
            revocation_posture_vocab=revocation_posture_vocab,
            redaction_class_vocab=redaction_class_vocab,
            high_risk_flag_vocab=high_risk_flag_vocab,
            audit_event_id_vocab=audit_event_id_vocab,
            denial_reason_vocab=denial_reason_vocab,
            invalidation_reason_vocab=invalidation_reason_vocab,
            failure_drill_id_vocab=failure_drill_id_vocab,
            forced_overrides=forced_overrides if forced_overrides else None,
        )
        row_results.append(result)

        if result.approval_ticket_profile_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="runtime_approval_ticket.entries_duplicate_approval_ticket_profile_id",
                    message=(
                        f"duplicate approval_ticket_profile_id: "
                        f"{result.approval_ticket_profile_id}"
                    ),
                    remediation=(
                        "approval_ticket_profile_id values must be unique."
                    ),
                    ref=result.approval_ticket_profile_id,
                )
            )
        seen_ids.add(result.approval_ticket_profile_id)

        if result.authority_class:
            seen_authority_classes.add(result.authority_class)
        if result.side_effect_class:
            seen_side_effect_classes.add(result.side_effect_class)
        if result.request_origin_class:
            seen_request_origin_classes.add(result.request_origin_class)
        if isinstance(payload, dict):
            rp = payload.get("revocation_posture")
            if isinstance(rp, dict) and isinstance(
                rp.get("posture_class"), str
            ):
                seen_revocation_postures.add(rp["posture_class"])

        if (
            forced_profile_id is not None
            and result.approval_ticket_profile_id == forced_profile_id
            and forced_overrides
            and isinstance(drill_local, dict)
        ):
            expected_check = ensure_str(
                drill_local.get("expected_check_id"),
                f"{forced_profile_id}.failure_drill.expected_check_id",
            )
            observed = [
                fc.get("check_id")
                for fc in result.failed_checks
                if isinstance(fc, dict)
            ]
            forced_replay_record = {
                "approval_ticket_profile_id": forced_profile_id,
                "drill_id": forced_drill_id,
                "expected_check_id": expected_check,
                "observed_failed_check_ids": observed,
                "reproduced": expected_check in observed,
            }

    # --- coverage --------------------------------------------------------
    missing_ac = required_authority_class_coverage - seen_authority_classes
    if missing_ac:
        findings.append(
            Finding(
                severity="error",
                check_id="runtime_approval_ticket.coverage_missing_required_authority_classes",
                message=(
                    "matrix must seed at least one row for each required "
                    "authority_class: "
                    f"{sorted(required_authority_class_coverage)}; "
                    f"missing: {sorted(missing_ac)}"
                ),
                remediation=(
                    "Add the missing rows so every M1 protected authority "
                    "class is reachable from the seed."
                ),
            )
        )

    missing_sec = (
        required_side_effect_class_coverage - seen_side_effect_classes
    )
    if missing_sec:
        findings.append(
            Finding(
                severity="error",
                check_id="runtime_approval_ticket.coverage_missing_required_side_effect_classes",
                message=(
                    "matrix must seed at least one row for each required "
                    "side_effect_class: "
                    f"{sorted(required_side_effect_class_coverage)}; "
                    f"missing: {sorted(missing_sec)}"
                ),
                remediation=(
                    "Add the missing rows so every protected side-effect "
                    "class is exercised."
                ),
            )
        )

    missing_roc = (
        required_request_origin_class_coverage
        - seen_request_origin_classes
    )
    if missing_roc:
        findings.append(
            Finding(
                severity="error",
                check_id="runtime_approval_ticket.coverage_missing_required_request_origin_classes",
                message=(
                    "matrix must seed at least one row for each required "
                    "request_origin_class: "
                    f"{sorted(required_request_origin_class_coverage)}; "
                    f"missing: {sorted(missing_roc)}"
                ),
                remediation=(
                    "Add the missing rows so the seed covers the issuer "
                    "seats it claims to cover."
                ),
            )
        )

    missing_rpc = (
        required_revocation_posture_class_coverage
        - seen_revocation_postures
    )
    if missing_rpc:
        findings.append(
            Finding(
                severity="error",
                check_id="runtime_approval_ticket.coverage_missing_required_revocation_postures",
                message=(
                    "the union of every row's revocation_posture.posture_class "
                    "MUST cover the required revocation postures: "
                    f"{sorted(required_revocation_posture_class_coverage)}; "
                    f"missing: {sorted(missing_rpc)}"
                ),
                remediation=(
                    "Add the missing posture to one of the example "
                    "payloads."
                ),
            )
        )

    # Promote per-row failures into findings, skipping the targeted row
    # under --force-drill so the runner's exit reflects the drill verdict.
    for result in row_results:
        if (
            forced_profile_id is not None
            and result.approval_ticket_profile_id == forced_profile_id
            and forced_replay_record is not None
        ):
            continue
        for failure in result.failed_checks:
            findings.append(
                Finding(
                    severity="error",
                    check_id=failure.get(
                        "check_id", "runtime_approval_ticket.row_failed_check"
                    ),
                    message=(
                        f"{result.approval_ticket_profile_id}: "
                        f"{failure.get('message', '')}"
                    ),
                    remediation=(
                        "Re-align the row or its example payload with the "
                        "runtime approval-ticket contract; failures are "
                        "reported with the precise actionable check_id."
                    ),
                    ref=result.approval_ticket_profile_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "runtime_approval_ticket_seed_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": owner_dri,
        "matrix_ref": matrix_rel,
        "envelope_schema_ref": args.envelope_schema,
        "row_schema_ref": args.row_schema,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/governance/m1_runtime_approval_ticket_seed_lane/"
            "run_m1_runtime_approval_ticket_seed_lane.py --repo-root ."
        ),
        "status": status,
        "required_authority_class_coverage": sorted(
            required_authority_class_coverage
        ),
        "observed_authority_classes": sorted(seen_authority_classes),
        "required_side_effect_class_coverage": sorted(
            required_side_effect_class_coverage
        ),
        "observed_side_effect_classes": sorted(seen_side_effect_classes),
        "required_request_origin_class_coverage": sorted(
            required_request_origin_class_coverage
        ),
        "observed_request_origin_classes": sorted(
            seen_request_origin_classes
        ),
        "required_revocation_posture_class_coverage": sorted(
            required_revocation_posture_class_coverage
        ),
        "observed_revocation_postures": sorted(seen_revocation_postures),
        "rows": [
            {
                "approval_ticket_profile_id": r.approval_ticket_profile_id,
                "authority_class": r.authority_class,
                "side_effect_class": r.side_effect_class,
                "issuer_class": r.issuer_class,
                "request_origin_class": r.request_origin_class,
                "use_posture": r.use_posture,
                "revocation_posture_class": r.revocation_posture_class,
                "passed_checks": r.passed_checks,
                "failed_checks": r.failed_checks,
                "diagnostics": r.diagnostics,
            }
            for r in row_results
        ],
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(
                1 for f in findings if f.severity == "warning"
            ),
        },
        "findings": [f.as_report() for f in findings],
    }

    if forced_replay_record is not None:
        capture["forced_drill_replay"] = forced_replay_record

    report_path = repo_root / args.report
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        json.dumps(capture, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )

    label = "runtime-approval-ticket-seed"
    print(
        f"[{label}] {status} ({len(errors)} errors, "
        f"{len(findings) - len(errors)} warnings) — capture: {args.report}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[{label}] {prefix} {finding.check_id}: {finding.message}"
            f"{ref_suffix}"
        )
        print(f"[{label}]   remediation: {finding.remediation}")

    if forced_replay_record is not None:
        if forced_replay_record["reproduced"]:
            print(
                f"[{label}] forced drill {forced_replay_record['drill_id']}"
                f" on {forced_replay_record['approval_ticket_profile_id']} "
                f"reproduced {forced_replay_record['expected_check_id']}"
            )
            return 0
        print(
            f"[{label}] forced drill {forced_replay_record['drill_id']} on"
            f" {forced_replay_record['approval_ticket_profile_id']} did NOT"
            f" reproduce {forced_replay_record['expected_check_id']};"
            f" observed: {forced_replay_record['observed_failed_check_ids']}"
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print(
            "[runtime-approval-ticket-seed] interrupted", file=sys.stderr
        )
        sys.exit(130)
