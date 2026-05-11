#!/usr/bin/env python3
"""Unattended M1 connected-provider seed validation lane.

Replays every row in
``fixtures/providers/connected_provider_seed_rows/m1_connected_provider_seed_rows.yaml``
against the connected-account-registry seed schema and the provider-
entry browser-handoff packet seed schema:

- ``schemas/providers/connected_account_registry.schema.json`` — the
  registry-entry, provider-descriptor, and publish-later object-seed
  vocabulary.
- ``schemas/providers/browser_handoff_packet.schema.json`` — the
  provider-entry browser-handoff packet seed vocabulary.

Per-row assertions:

- ``row_id`` is unique and namespaced under ``provider_entry:``;
- the row's ``entry`` block, ``descriptor`` block, every
  ``publish_later_seeds`` entry, and (when present) ``packet_seed``
  carry the matching ``entry_kind`` / ``record_kind`` discriminator;
- closed vocabularies are honored:
  ``mutation_disposition_class``, ``freshness_class``,
  ``registry_entry_status``, ``publish_later_object_class``,
  ``provider_actor_class``, ``provider_capability_class``;
- the entry's ``mutation_disposition_class`` is in the descriptor's
  ``supported_capabilities`` (browser_handoff_required requires
  supports_browser_handoff, publish_later_required requires
  supports_publish_later, immediate_mutation_in_product requires
  supports_immediate_mutation, inspect_only_no_mutation requires
  supports_inspect_only);
- the entry's ``actor_scope.primary_actor_class`` is in the descriptor's
  ``supported_actor_classes``;
- publish-later coverage rules:
    - publish_later_required and browser_handoff_required entries cite
      at least one ``publish_later_object_seed_ref``,
    - inspect_only_no_mutation entries cite none,
    - browser_handoff_required entries cite a non-empty
      ``browser_handoff_routing_summary_ref``;
- publish-later seed rules:
    - local_draft_only and imported_read_only_snapshot seeds carry the
      allowed dispositions only,
    - queued_publish_pending and deferred_publish_in_queue seeds cite a
      non-empty ``publish_later_queue_item_ref``,
    - browser_handoff_pending seeds carry browser_handoff_required;
- never_observed entries carry registry_entry_status in
  {seeded, unobserved_pending};
- live_observed entries cannot carry primary_actor_class
  unknown_actor_class;
- the matrix covers every entry in
  ``required_mutation_disposition_coverage`` and
  ``required_actor_coverage``;
- every named external schema, build identity, and consumer ref exists
  on disk.

The runner emits a durable, machine-readable capture (``--report``) and
exits non-zero if any row fails. ``--force-drill <row_id>:<drill_id>``
replays the named drill on the named row and exits 0 only when the
runner reproduces the declared ``expected_check_id``.

YAML decoding follows the existing repository convention: matrix and
fixture files are parsed via Ruby/Psych so this script does not require
a third-party Python YAML dependency.
"""

from __future__ import annotations

import argparse
import copy
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_MATRIX_REL = (
    "fixtures/providers/connected_provider_seed_rows/"
    "m1_connected_provider_seed_rows.yaml"
)
DEFAULT_REGISTRY_SCHEMA_REL = (
    "schemas/providers/connected_account_registry.schema.json"
)
DEFAULT_PACKET_SCHEMA_REL = (
    "schemas/providers/browser_handoff_packet.schema.json"
)
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/"
    "connected_provider_seed_validation_capture.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"

ROW_ID_PREFIX = "provider_entry:"

# Capability requirement for each mutation_disposition_class.
DISPOSITION_REQUIRED_CAPABILITY = {
    "immediate_mutation_in_product": "supports_immediate_mutation",
    "browser_handoff_required": "supports_browser_handoff",
    "publish_later_required": "supports_publish_later",
    "inspect_only_no_mutation": "supports_inspect_only",
}


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
    row_id: str
    mutation_disposition_class: str
    primary_actor_class: str
    registry_entry_status: str
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
        help="Seed-row matrix YAML path, repo-relative.",
    )
    parser.add_argument(
        "--registry-schema",
        default=DEFAULT_REGISTRY_SCHEMA_REL,
        help="Connected-account-registry schema JSON path, repo-relative.",
    )
    parser.add_argument(
        "--packet-schema",
        default=DEFAULT_PACKET_SCHEMA_REL,
        help="Provider-entry browser-handoff packet schema JSON path, repo-relative.",
    )
    parser.add_argument(
        "--report",
        default=DEFAULT_REPORT_REL,
        help="Where to write the durable JSON capture (repo-relative).",
    )
    parser.add_argument(
        "--build-identity",
        default=DEFAULT_BUILD_IDENTITY_REL,
        help="Path to the build-identity record to embed in the capture.",
    )
    parser.add_argument(
        "--force-drill",
        default=None,
        help=(
            "Replay a named failure drill on a named row in the form "
            "'<row_id>:<drill_id>'. The runner exits 0 only when the "
            "row's failure drill reproduces the exact expected_check_id."
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
        raise SystemExit(f"failed to parse YAML at {path} via Ruby/Psych: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


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


def load_schema_enums(repo_root: Path, ref: str, defs_key: str) -> list[str]:
    """Best-effort enum lookup from a schema's $defs."""
    schema = json.loads((repo_root / ref).read_text(encoding="utf-8"))
    defs = schema.get("$defs", {})
    entry = defs.get(defs_key, {})
    if "enum" in entry and isinstance(entry["enum"], list):
        return [str(v) for v in entry["enum"]]
    return []


def apply_forced_overrides(
    row: dict[str, Any], forced_overrides: dict[str, Any]
) -> dict[str, Any]:
    """Return a deep-copied row with the forced overrides applied."""
    row = copy.deepcopy(row)
    if not forced_overrides:
        return row

    entry = row["entry"]
    descriptor = row["descriptor"]

    if "strip_capability" in forced_overrides:
        cap = forced_overrides["strip_capability"]
        descriptor["supported_capabilities"] = [
            c for c in descriptor.get("supported_capabilities", []) if c != cap
        ]

    if forced_overrides.get("clear_publish_later_object_seed_refs"):
        entry["publish_later_object_seed_refs"] = []
        row["publish_later_seeds"] = []

    if forced_overrides.get("clear_browser_handoff_routing_summary_ref"):
        entry["browser_handoff_routing_summary_ref"] = ""

    if "inject_publish_later_object_seed_ref" in forced_overrides:
        ref = forced_overrides["inject_publish_later_object_seed_ref"]
        entry.setdefault("publish_later_object_seed_refs", []).append(ref)

    if forced_overrides.get("clear_publish_later_queue_item_ref"):
        for seed in row.get("publish_later_seeds", []):
            seed["publish_later_queue_item_ref"] = ""

    if "rewrite_registry_entry_status" in forced_overrides:
        entry["registry_entry_status"] = forced_overrides[
            "rewrite_registry_entry_status"
        ]

    return row


def validate_row(
    row: dict[str, Any],
    *,
    mutation_disposition_class_vocab: set[str],
    freshness_class_vocab: set[str],
    registry_entry_status_vocab: set[str],
    publish_later_object_class_vocab: set[str],
    provider_actor_class_vocab: set[str],
    provider_capability_class_vocab: set[str],
) -> RowResult:
    row_id = ensure_str(row.get("row_id"), "row.row_id")
    descriptor = ensure_dict(row.get("descriptor"), f"{row_id}.descriptor")
    entry = ensure_dict(row.get("entry"), f"{row_id}.entry")
    publish_later_seeds = ensure_list(
        row.get("publish_later_seeds", []),
        f"{row_id}.publish_later_seeds",
    )
    packet_seed = row.get("packet_seed")

    mutation_disposition_class = ensure_str(
        entry.get("mutation_disposition_class"),
        f"{row_id}.entry.mutation_disposition_class",
    )
    actor_scope = ensure_dict(
        entry.get("actor_scope"), f"{row_id}.entry.actor_scope"
    )
    primary_actor_class = ensure_str(
        actor_scope.get("primary_actor_class"),
        f"{row_id}.entry.actor_scope.primary_actor_class",
    )
    registry_entry_status = ensure_str(
        entry.get("registry_entry_status"),
        f"{row_id}.entry.registry_entry_status",
    )

    result = RowResult(
        row_id=row_id,
        mutation_disposition_class=mutation_disposition_class,
        primary_actor_class=primary_actor_class,
        registry_entry_status=registry_entry_status,
    )

    # row_id prefix.
    if not row_id.startswith(ROW_ID_PREFIX):
        fail(
            result,
            "connected_provider_seed.row_id_unprefixed",
            f"row_id '{row_id}' must start with '{ROW_ID_PREFIX}'",
        )

    # Discriminator checks.
    if descriptor.get("entry_kind") != "provider_descriptor_record":
        fail(
            result,
            "connected_provider_seed.descriptor_entry_kind_wrong",
            (
                "descriptor.entry_kind must be 'provider_descriptor_record'; "
                f"got {descriptor.get('entry_kind')!r}"
            ),
        )
    if entry.get("entry_kind") != "connected_account_registry_entry_record":
        fail(
            result,
            "connected_provider_seed.entry_entry_kind_wrong",
            (
                "entry.entry_kind must be "
                "'connected_account_registry_entry_record'; "
                f"got {entry.get('entry_kind')!r}"
            ),
        )

    # Closed-vocab membership.
    if mutation_disposition_class not in mutation_disposition_class_vocab:
        fail(
            result,
            "connected_provider_seed.mutation_disposition_class_unknown",
            (
                f"entry.mutation_disposition_class '{mutation_disposition_class}'"
                " is not in mutation_disposition_class_vocabulary"
            ),
        )
    else:
        pass_(
            result,
            f"mutation_disposition_class '{mutation_disposition_class}' is in vocabulary",
        )

    if primary_actor_class not in provider_actor_class_vocab:
        fail(
            result,
            "connected_provider_seed.primary_actor_class_unknown",
            (
                f"entry.actor_scope.primary_actor_class '{primary_actor_class}'"
                " is not in provider_actor_class_vocabulary"
            ),
        )

    if registry_entry_status not in registry_entry_status_vocab:
        fail(
            result,
            "connected_provider_seed.registry_entry_status_unknown",
            (
                f"entry.registry_entry_status '{registry_entry_status}' is not"
                " in registry_entry_status_vocabulary"
            ),
        )

    freshness = ensure_dict(entry.get("freshness"), f"{row_id}.entry.freshness")
    freshness_class = ensure_str(
        freshness.get("freshness_class"),
        f"{row_id}.entry.freshness.freshness_class",
    )
    if freshness_class not in freshness_class_vocab:
        fail(
            result,
            "connected_provider_seed.freshness_class_unknown",
            (
                f"entry.freshness.freshness_class '{freshness_class}' is not"
                " in freshness_class_vocabulary"
            ),
        )

    supported_capabilities = ensure_list(
        descriptor.get("supported_capabilities"),
        f"{row_id}.descriptor.supported_capabilities",
    )
    for cap in supported_capabilities:
        if cap not in provider_capability_class_vocab:
            fail(
                result,
                "connected_provider_seed.provider_capability_class_unknown",
                (
                    f"descriptor.supported_capabilities contains '{cap}'"
                    " which is not in provider_capability_class_vocabulary"
                ),
            )

    supported_actor_classes = ensure_list(
        descriptor.get("supported_actor_classes"),
        f"{row_id}.descriptor.supported_actor_classes",
    )
    for cls in supported_actor_classes:
        if cls not in provider_actor_class_vocab:
            fail(
                result,
                "connected_provider_seed.descriptor_actor_class_unknown",
                (
                    f"descriptor.supported_actor_classes contains '{cls}'"
                    " which is not in provider_actor_class_vocabulary"
                ),
            )

    # The entry's primary actor class must be in the descriptor's set.
    if primary_actor_class not in supported_actor_classes:
        fail(
            result,
            "connected_provider_seed.actor_not_supported_by_descriptor",
            (
                f"entry.actor_scope.primary_actor_class '{primary_actor_class}'"
                " is not in descriptor.supported_actor_classes"
            ),
        )

    # The entry's mutation disposition must be admitted by the descriptor's
    # capabilities.
    required_cap = DISPOSITION_REQUIRED_CAPABILITY.get(mutation_disposition_class)
    if required_cap is not None and required_cap not in supported_capabilities:
        fail(
            result,
            "connected_provider_seed.mutation_disposition_not_supported_by_descriptor",
            (
                f"entry.mutation_disposition_class '{mutation_disposition_class}'"
                f" requires capability '{required_cap}' on the descriptor;"
                f" descriptor capabilities: {sorted(supported_capabilities)}"
            ),
        )
    else:
        pass_(
            result,
            f"descriptor admits mutation_disposition_class '{mutation_disposition_class}'",
        )

    # browser-handoff routing summary.
    browser_handoff_routing_summary_ref = entry.get(
        "browser_handoff_routing_summary_ref", ""
    )
    if mutation_disposition_class == "browser_handoff_required":
        if (
            not isinstance(browser_handoff_routing_summary_ref, str)
            or not browser_handoff_routing_summary_ref.strip()
        ):
            fail(
                result,
                "connected_provider_seed.browser_handoff_routing_summary_required",
                (
                    "entry.browser_handoff_routing_summary_ref must be a"
                    " non-empty string when mutation_disposition_class is"
                    " browser_handoff_required"
                ),
            )

    # Publish-later seed refs.
    publish_later_object_seed_refs = entry.get(
        "publish_later_object_seed_refs", []
    )
    if not isinstance(publish_later_object_seed_refs, list):
        fail(
            result,
            "connected_provider_seed.publish_later_object_seed_refs_must_be_list",
            (
                "entry.publish_later_object_seed_refs must be a list of"
                " opaque ids"
            ),
        )
        publish_later_object_seed_refs = []

    if (
        mutation_disposition_class in {"publish_later_required", "browser_handoff_required"}
        and not publish_later_object_seed_refs
    ):
        fail(
            result,
            "connected_provider_seed.publish_later_seed_required",
            (
                f"entry.mutation_disposition_class '{mutation_disposition_class}'"
                " requires at least one publish_later_object_seed_ref"
            ),
        )
    if (
        mutation_disposition_class == "inspect_only_no_mutation"
        and publish_later_object_seed_refs
    ):
        fail(
            result,
            "connected_provider_seed.inspect_only_must_have_no_publish_later_seed",
            (
                "inspect_only_no_mutation entries MUST NOT cite any"
                " publish_later_object_seed_ref"
            ),
        )

    # never_observed entries cannot claim a live observation.
    if freshness_class == "never_observed" and registry_entry_status not in {
        "seeded",
        "unobserved_pending",
    }:
        fail(
            result,
            "connected_provider_seed.never_observed_must_not_be_live",
            (
                "never_observed entries MUST carry registry_entry_status"
                " seeded or unobserved_pending"
            ),
        )

    # live_observed entries MUST NOT carry unknown_actor_class.
    if (
        registry_entry_status == "live_observed"
        and primary_actor_class == "unknown_actor_class"
    ):
        fail(
            result,
            "connected_provider_seed.live_observed_must_not_carry_unknown_actor",
            (
                "live_observed entries MUST NOT carry primary_actor_class"
                " unknown_actor_class; route to repair instead"
            ),
        )

    # Cross-check that every seed ref referenced by the entry is materialized
    # as a publish_later_seeds entry (when the row has any seeds inline).
    declared_seed_ids = {
        ensure_str(s.get("seed_id"), f"{row_id}.publish_later_seeds[].seed_id")
        for s in publish_later_seeds
    }
    for ref in publish_later_object_seed_refs:
        if not isinstance(ref, str) or not ref.strip():
            fail(
                result,
                "connected_provider_seed.publish_later_object_seed_ref_must_be_non_empty",
                (
                    "entry.publish_later_object_seed_refs entries must be"
                    " non-empty opaque ids"
                ),
            )

    # Validate each publish-later seed in turn.
    for idx, seed in enumerate(publish_later_seeds):
        seed = ensure_dict(seed, f"{row_id}.publish_later_seeds[{idx}]")
        seed_kind = seed.get("entry_kind")
        if seed_kind != "publish_later_object_seed_record":
            fail(
                result,
                "connected_provider_seed.publish_later_seed_entry_kind_wrong",
                (
                    f"publish_later_seeds[{idx}].entry_kind must be"
                    " 'publish_later_object_seed_record'"
                ),
            )
        object_class = ensure_str(
            seed.get("publish_later_object_class"),
            f"{row_id}.publish_later_seeds[{idx}].publish_later_object_class",
        )
        if object_class not in publish_later_object_class_vocab:
            fail(
                result,
                "connected_provider_seed.publish_later_object_class_unknown",
                (
                    f"publish_later_seeds[{idx}].publish_later_object_class"
                    f" '{object_class}' is not in"
                    " publish_later_object_class_vocabulary"
                ),
            )

        seed_disposition = ensure_str(
            seed.get("mutation_disposition_class"),
            f"{row_id}.publish_later_seeds[{idx}].mutation_disposition_class",
        )

        # Rules per publish_later_object_class.
        if (
            object_class == "local_draft_only"
            and seed_disposition
            not in {"immediate_mutation_in_product", "inspect_only_no_mutation"}
        ):
            fail(
                result,
                "connected_provider_seed.local_draft_only_disposition_invalid",
                (
                    "local_draft_only seeds MUST carry mutation_disposition_class"
                    " immediate_mutation_in_product or inspect_only_no_mutation"
                ),
            )
        if (
            object_class == "imported_read_only_snapshot"
            and seed_disposition != "inspect_only_no_mutation"
        ):
            fail(
                result,
                "connected_provider_seed.imported_snapshot_must_be_inspect_only",
                (
                    "imported_read_only_snapshot seeds MUST carry"
                    " mutation_disposition_class inspect_only_no_mutation"
                ),
            )
        if (
            object_class == "browser_handoff_pending"
            and seed_disposition != "browser_handoff_required"
        ):
            fail(
                result,
                "connected_provider_seed.browser_handoff_pending_disposition_invalid",
                (
                    "browser_handoff_pending seeds MUST carry"
                    " mutation_disposition_class browser_handoff_required"
                ),
            )
        if object_class in {
            "queued_publish_pending",
            "deferred_publish_in_queue",
        }:
            qref = seed.get("publish_later_queue_item_ref")
            if not isinstance(qref, str) or not qref.strip():
                fail(
                    result,
                    "connected_provider_seed.publish_later_queue_item_required",
                    (
                        f"publish_later_seeds[{idx}] of class '{object_class}'"
                        " MUST cite a non-empty publish_later_queue_item_ref"
                    ),
                )

    # Validate the optional provider-entry browser-handoff packet seed.
    if packet_seed is not None:
        packet_seed = ensure_dict(packet_seed, f"{row_id}.packet_seed")
        if (
            packet_seed.get("record_kind")
            != "provider_entry_browser_handoff_packet_record"
        ):
            fail(
                result,
                "connected_provider_seed.packet_seed_record_kind_wrong",
                (
                    "packet_seed.record_kind must be"
                    " 'provider_entry_browser_handoff_packet_record'"
                ),
            )
        packet_disposition = ensure_str(
            packet_seed.get("mutation_disposition_class"),
            f"{row_id}.packet_seed.mutation_disposition_class",
        )
        if packet_disposition not in {
            "browser_handoff_required",
            "publish_later_required",
        }:
            fail(
                result,
                "connected_provider_seed.packet_seed_disposition_invalid",
                (
                    "packet_seed.mutation_disposition_class MUST be"
                    " browser_handoff_required or publish_later_required"
                ),
            )
        if packet_disposition != mutation_disposition_class:
            fail(
                result,
                "connected_provider_seed.packet_seed_disposition_mismatch",
                (
                    "packet_seed.mutation_disposition_class must match the"
                    " entry's mutation_disposition_class; "
                    f"entry={mutation_disposition_class}, "
                    f"packet={packet_disposition}"
                ),
            )

    # Failure drill shape.
    drill = ensure_dict(row.get("failure_drill"), f"{row_id}.failure_drill")
    ensure_str(drill.get("drill_id"), f"{row_id}.failure_drill.drill_id")
    ensure_str(
        drill.get("expected_check_id"),
        f"{row_id}.failure_drill.expected_check_id",
    )
    ensure_str(
        drill.get("actionable_owner_ref"),
        f"{row_id}.failure_drill.actionable_owner_ref",
    )
    ensure_str(drill.get("next_action"), f"{row_id}.failure_drill.next_action")
    forced_input = ensure_dict(
        drill.get("forced_input"), f"{row_id}.failure_drill.forced_input"
    )
    if not forced_input:
        fail(
            result,
            "connected_provider_seed.failure_drill_forced_input_empty",
            "failure_drill.forced_input must declare at least one drift",
        )

    result.diagnostics.update(
        {
            "row_id": row_id,
            "mutation_disposition_class": mutation_disposition_class,
            "primary_actor_class": primary_actor_class,
            "registry_entry_status": registry_entry_status,
            "freshness_class": freshness_class,
            "publish_later_seed_count": len(publish_later_seeds),
            "publish_later_object_seed_refs": list(
                publish_later_object_seed_refs
            ),
            "has_packet_seed": packet_seed is not None,
            "descriptor_capabilities": list(supported_capabilities),
            "failure_drill": {
                "drill_id": drill.get("drill_id"),
                "expected_check_id": drill.get("expected_check_id"),
            },
        }
    )

    return result


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    matrix_rel = args.matrix
    matrix_path = repo_root / matrix_rel
    matrix = ensure_dict(render_yaml_as_json(matrix_path), matrix_rel)

    findings: list[Finding] = []

    schema_version = matrix.get("schema_version")
    if not isinstance(schema_version, int) or schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.schema_version",
                message=(
                    f"matrix schema_version must be the integer 1, got "
                    f"{schema_version!r}"
                ),
                remediation=(
                    "Bump the runner together with the matrix when its shape"
                    " changes."
                ),
            )
        )

    ensure_str(matrix.get("status"), "matrix.status")
    ensure_str(matrix.get("matrix_id"), "matrix.matrix_id")
    ensure_str(matrix.get("owner_dri"), "matrix.owner_dri")

    overview_page = ensure_str(
        matrix.get("overview_page"), "matrix.overview_page"
    )
    if not artifact_ref_exists(repo_root, overview_page):
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.overview_page.missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation=(
                    "Create the reviewer-facing landing page or fix the path."
                ),
                ref=overview_page,
            )
        )

    for key in (
        "registry_schema_ref",
        "packet_schema_ref",
        "connected_account_record_schema_ref",
        "publish_later_record_schema_ref",
        "integration_browser_handoff_packet_schema_ref",
        "build_identity_ref",
    ):
        ref = ensure_str(matrix.get(key), f"matrix.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"matrix.{key}.missing",
                    message=f"{key} does not resolve: {ref}",
                    remediation=(
                        "Fix the path or land the referenced artifact."
                    ),
                    ref=ref,
                )
            )

    validation_lane_ref = ensure_str(
        matrix.get("validation_lane_ref"), "matrix.validation_lane_ref"
    )
    fragment = validation_lane_ref.split("#", 1)[0]
    if not (repo_root / fragment).exists():
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.validation_lane_ref.missing",
                message=(
                    f"validation_lane_ref base does not exist: "
                    f"{validation_lane_ref}"
                ),
                remediation="Point at a seeded QE lane registry entry.",
                ref=validation_lane_ref,
            )
        )

    # Closed vocabularies.
    def load_vocab(key: str) -> set[str]:
        return {
            ensure_str(item, f"matrix.{key}[]")
            for item in ensure_list(matrix.get(key), f"matrix.{key}")
        }

    mutation_disposition_class_vocab = load_vocab(
        "mutation_disposition_class_vocabulary"
    )
    freshness_class_vocab = load_vocab("freshness_class_vocabulary")
    registry_entry_status_vocab = load_vocab(
        "registry_entry_status_vocabulary"
    )
    publish_later_object_class_vocab = load_vocab(
        "publish_later_object_class_vocabulary"
    )
    provider_actor_class_vocab = load_vocab("provider_actor_class_vocabulary")
    provider_capability_class_vocab = load_vocab(
        "provider_capability_class_vocabulary"
    )
    failure_drill_id_vocab = load_vocab("failure_drill_id_vocabulary")

    required_disposition_coverage = load_vocab(
        "required_mutation_disposition_coverage"
    )
    required_actor_coverage = load_vocab("required_actor_coverage")

    # Cross-check the matrix's closed vocabularies against the schemas.
    registry_schema_rel = ensure_str(
        matrix.get("registry_schema_ref"), "matrix.registry_schema_ref"
    )
    packet_schema_rel = ensure_str(
        matrix.get("packet_schema_ref"), "matrix.packet_schema_ref"
    )

    def assert_vocab_matches_schema(
        matrix_vocab: set[str], schema_rel: str, defs_key: str, name: str
    ) -> None:
        if not (repo_root / schema_rel).exists():
            return
        schema_enum = set(load_schema_enums(repo_root, schema_rel, defs_key))
        if not schema_enum:
            return
        diff = matrix_vocab.symmetric_difference(schema_enum)
        if diff:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"matrix.{name}.mismatch_schema",
                    message=(
                        f"matrix.{name} disagrees with"
                        f" {schema_rel}#$defs.{defs_key}; "
                        f"matrix-only: {sorted(matrix_vocab - schema_enum)};"
                        f" schema-only: {sorted(schema_enum - matrix_vocab)}"
                    ),
                    remediation=(
                        "Keep the matrix's closed vocabulary in lock-step"
                        f" with {schema_rel}; the schema is canonical."
                    ),
                )
            )

    assert_vocab_matches_schema(
        mutation_disposition_class_vocab,
        registry_schema_rel,
        "mutation_disposition_class",
        "mutation_disposition_class_vocabulary",
    )
    assert_vocab_matches_schema(
        freshness_class_vocab,
        registry_schema_rel,
        "freshness_class",
        "freshness_class_vocabulary",
    )
    assert_vocab_matches_schema(
        registry_entry_status_vocab,
        registry_schema_rel,
        "registry_entry_status",
        "registry_entry_status_vocabulary",
    )
    assert_vocab_matches_schema(
        publish_later_object_class_vocab,
        registry_schema_rel,
        "publish_later_object_class",
        "publish_later_object_class_vocabulary",
    )
    assert_vocab_matches_schema(
        provider_actor_class_vocab,
        registry_schema_rel,
        "provider_actor_class",
        "provider_actor_class_vocabulary",
    )
    assert_vocab_matches_schema(
        provider_capability_class_vocab,
        registry_schema_rel,
        "provider_capability_class",
        "provider_capability_class_vocabulary",
    )

    # Consumer bindings.
    consumer_bindings = ensure_dict(
        matrix.get("consumer_bindings"), "matrix.consumer_bindings"
    )
    named_consumer = ensure_dict(
        consumer_bindings.get("named_runtime_consumer"),
        "matrix.consumer_bindings.named_runtime_consumer",
    )
    consumer_ref = ensure_str(
        named_consumer.get("consumer_ref"),
        "matrix.consumer_bindings.named_runtime_consumer.consumer_ref",
    )
    if not artifact_ref_exists(repo_root, consumer_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.named_runtime_consumer.missing",
                message=(
                    "named_runtime_consumer.consumer_ref does not exist: "
                    f"{consumer_ref}"
                ),
                remediation=(
                    "Point at a real downstream consumer or seed the surface"
                    " before claiming a runtime consumer exists."
                ),
                ref=consumer_ref,
            )
        )
    consumed_fields = ensure_list(
        named_consumer.get("consumed_fields"),
        "matrix.consumer_bindings.named_runtime_consumer.consumed_fields",
    )
    if not consumed_fields:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.named_runtime_consumer.consumed_fields_empty",
                message=(
                    "named_runtime_consumer.consumed_fields must declare at"
                    " least one truth field"
                ),
                remediation=(
                    "Name the truth fields the runtime consumer reads from"
                    " the matrix."
                ),
            )
        )

    rows = ensure_list(matrix.get("rows"), "matrix.rows")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.rows.empty",
                message="matrix.rows must declare at least one capability row",
                remediation="Seed the required rows.",
            )
        )

    # Resolve --force-drill if requested.
    forced_row_id: str | None = None
    forced_drill_id: str | None = None
    if args.force_drill:
        if ":" not in args.force_drill:
            raise SystemExit(
                "--force-drill must be of the form '<row_id>:<drill_id>'"
            )
        forced_row_id, forced_drill_id = args.force_drill.rsplit(":", 1)
        forced_row_id = forced_row_id.strip()
        forced_drill_id = forced_drill_id.strip()

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_dispositions: set[str] = set()
    seen_actors: set[str] = set()
    forced_replay_record: dict[str, Any] | None = None

    for row in rows:
        row = ensure_dict(row, "matrix.rows[]")
        original_row = copy.deepcopy(row)
        row_id_local = ensure_str(row.get("row_id"), "row.row_id")
        drill = ensure_dict(
            row.get("failure_drill"), f"{row_id_local}.failure_drill"
        )
        drill_id_local = ensure_str(
            drill.get("drill_id"), f"{row_id_local}.failure_drill.drill_id"
        )

        if drill_id_local not in failure_drill_id_vocab:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.failure_drill_id_unknown",
                    message=(
                        f"{row_id_local}: failure_drill.drill_id"
                        f" '{drill_id_local}' is not in"
                        " failure_drill_id_vocabulary"
                    ),
                    remediation=(
                        "Add the drill id to failure_drill_id_vocabulary or"
                        " rename the drill."
                    ),
                    ref=row_id_local,
                )
            )

        applied_overrides: dict[str, Any] = {}
        replay_row_payload = row
        if forced_row_id is not None and row_id_local == forced_row_id:
            if drill_id_local != forced_drill_id:
                raise SystemExit(
                    f"--force-drill drill_id '{forced_drill_id}' does not"
                    f" match the row's failure_drill.drill_id"
                    f" '{drill_id_local}'"
                )
            applied_overrides = ensure_dict(
                drill.get("forced_input"),
                f"{row_id_local}.failure_drill.forced_input",
            )
            replay_row_payload = apply_forced_overrides(
                row, applied_overrides
            )

        result = validate_row(
            replay_row_payload,
            mutation_disposition_class_vocab=mutation_disposition_class_vocab,
            freshness_class_vocab=freshness_class_vocab,
            registry_entry_status_vocab=registry_entry_status_vocab,
            publish_later_object_class_vocab=publish_later_object_class_vocab,
            provider_actor_class_vocab=provider_actor_class_vocab,
            provider_capability_class_vocab=provider_capability_class_vocab,
        )
        if applied_overrides:
            result.diagnostics["forced_overrides_applied"] = applied_overrides
        row_results.append(result)

        if result.row_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.rows.duplicate_id",
                    message=f"duplicate row_id: {result.row_id}",
                    remediation="row_ids must be unique.",
                    ref=result.row_id,
                )
            )
        seen_ids.add(result.row_id)

        # Coverage is computed from the original row (not the drill-forced
        # replay), so drills cannot accidentally satisfy or violate coverage.
        expected_coverage = ensure_dict(
            original_row.get("expected_coverage"),
            f"{result.row_id}.expected_coverage",
        )
        seen_dispositions.add(
            ensure_str(
                expected_coverage.get("mutation_disposition_class"),
                f"{result.row_id}.expected_coverage.mutation_disposition_class",
            )
        )
        seen_actors.add(
            ensure_str(
                expected_coverage.get("actor_class"),
                f"{result.row_id}.expected_coverage.actor_class",
            )
        )

        if (
            forced_row_id is not None
            and result.row_id == forced_row_id
            and applied_overrides
        ):
            expected_check = ensure_str(
                drill.get("expected_check_id"),
                f"{forced_row_id}.failure_drill.expected_check_id",
            )
            observed = [
                fc.get("check_id")
                for fc in result.failed_checks
                if isinstance(fc, dict)
            ]
            forced_replay_record = {
                "row_id": forced_row_id,
                "drill_id": forced_drill_id,
                "expected_check_id": expected_check,
                "observed_failed_check_ids": observed,
                "reproduced": expected_check in observed,
            }

    missing_dispositions = required_disposition_coverage - seen_dispositions
    if missing_dispositions:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.coverage.missing_required_mutation_dispositions",
                message=(
                    "matrix must seed at least one row for each"
                    f" required mutation_disposition_class:"
                    f" {sorted(required_disposition_coverage)}; missing:"
                    f" {sorted(missing_dispositions)}"
                ),
                remediation=(
                    "Add the missing rows so every required disposition class"
                    " is exercised."
                ),
            )
        )

    missing_actors = required_actor_coverage - seen_actors
    if missing_actors:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.coverage.missing_required_actors",
                message=(
                    "matrix must seed at least one row for each"
                    f" required provider_actor_class:"
                    f" {sorted(required_actor_coverage)}; missing:"
                    f" {sorted(missing_actors)}"
                ),
                remediation=(
                    "Add the missing rows so every required actor class is"
                    " exercised."
                ),
            )
        )

    # Promote per-row failures into findings. Skip the targeted row under
    # --force-drill so the runner's exit can reflect the reproduce verdict.
    for result in row_results:
        if (
            forced_row_id is not None
            and result.row_id == forced_row_id
            and forced_replay_record is not None
        ):
            continue
        for failure in result.failed_checks:
            findings.append(
                Finding(
                    severity="error",
                    check_id=failure.get(
                        "check_id", "matrix.row.failed_check"
                    ),
                    message=f"{result.row_id}: {failure.get('message', '')}",
                    remediation=(
                        "Re-align the row with the registry / packet schemas"
                        " or fix the drift in the matrix; failures are"
                        " reported with the precise actionable check_id."
                    ),
                    ref=result.row_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "connected_provider_seed_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": ensure_str(matrix.get("owner_dri"), "matrix.owner_dri"),
        "matrix_ref": matrix_rel,
        "registry_schema_ref": args.registry_schema,
        "packet_schema_ref": args.packet_schema,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/providers/m1_connected_provider_seed_lane/"
            "run_m1_connected_provider_seed_lane.py --repo-root ."
        ),
        "status": status,
        "required_mutation_disposition_coverage": sorted(
            required_disposition_coverage
        ),
        "observed_mutation_dispositions": sorted(seen_dispositions),
        "required_actor_coverage": sorted(required_actor_coverage),
        "observed_actors": sorted(seen_actors),
        "rows": [
            {
                "row_id": r.row_id,
                "mutation_disposition_class": r.mutation_disposition_class,
                "primary_actor_class": r.primary_actor_class,
                "registry_entry_status": r.registry_entry_status,
                "passed_checks": r.passed_checks,
                "failed_checks": r.failed_checks,
                "diagnostics": r.diagnostics,
            }
            for r in row_results
        ],
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
    }

    if forced_replay_record is not None:
        capture["forced_drill_replay"] = forced_replay_record

    report_path = repo_root / args.report
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        json.dumps(capture, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )

    label = "connected-provider-seed"
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
                f" on {forced_replay_record['row_id']} reproduced"
                f" {forced_replay_record['expected_check_id']}"
            )
            return 0
        print(
            f"[{label}] forced drill {forced_replay_record['drill_id']} on"
            f" {forced_replay_record['row_id']} did NOT reproduce"
            f" {forced_replay_record['expected_check_id']};"
            f" observed: {forced_replay_record['observed_failed_check_ids']}"
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[connected-provider-seed] interrupted", file=sys.stderr)
        sys.exit(130)
