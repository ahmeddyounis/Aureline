#!/usr/bin/env python3
"""Unattended M1 docs/help/About/service-health truth-source seed lane.

Replays every row in ``artifacts/help/m1_truth_source_examples.yaml``
against:

- ``schemas/help/provenance_badge_vocabulary.schema.json`` — the seed
  envelope schema (vocabularies, required coverage, row list);
- the row's pinned example payload under
  ``fixtures/help/m1_truth_source_examples/`` (parsed end-to-end); and
- the named runtime consumer the row binds (must exist on disk).

Per-row assertions:

- ``record_kind`` is ``m1_truth_source_row`` and
  ``truth_source_row_schema_version`` is ``1``;
- closed vocabularies are honoured (``badge_family_class``,
  ``consuming_surface_class``, ``consumer_class``,
  ``vocabulary_role_class``);
- the matrix's closed vocabularies agree with the schema's $defs
  (no drift);
- every row's ``honesty_fallback_token`` is non-empty;
- every row's ``honesty_fallback_token`` appears in
  ``vocabulary_tokens`` with role ``honesty_fallback_token``;
- every row's ``consuming_surface_classes`` includes the four
  required surfaces (``help_pane``, ``about_pane``,
  ``service_health_pane``, ``docs_browser_pane``);
- degraded-state tokens carrying the ``degraded_state_token`` role
  may not be widened to a live state by another row's claim;
- seed-placeholder tokens in the ``provenance_row_state`` and
  ``service_health_state`` rows are present and carry the
  ``seed_placeholder_token`` role;
- the row's named runtime consumer ref resolves on disk;
- the example payload pins the row's badge family class, the
  honesty-fallback token, and renders at least three consuming
  surface examples whose ``rendered_token`` values all belong to
  ``vocabulary_tokens``;
- the row's failure_drill drill_id is in the
  ``failure_drill_id_vocabulary``;
- the matrix covers every row in
  ``required_badge_family_class_coverage``.

``--force-drill <row_id>:<drill_id>`` replays the named drill on the
named row and exits 0 only when the runner reproduces the declared
``expected_check_id``. Drift in the unforced rows still fails the lane.

YAML decoding follows the repository convention: matrix files are
parsed via Ruby/Psych so this script does not require a third-party
Python YAML dependency.
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


DEFAULT_MATRIX_REL = "artifacts/help/m1_truth_source_examples.yaml"
DEFAULT_ENVELOPE_SCHEMA_REL = (
    "schemas/help/provenance_badge_vocabulary.schema.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/"
    "truth_source_seed_validation_capture.json"
)

EXPECTED_SCHEMA_VERSION = 1
EXPECTED_MATRIX_ID = "m1_truth_source_badge_vocabulary_seed"
EXPECTED_RECORD_KIND = "m1_truth_source_row"
EXPECTED_ROW_SCHEMA_VERSION = 1
EXPECTED_EXAMPLE_KIND = "m1_truth_source_row_example"

REQUIRED_SURFACES = (
    "help_pane",
    "about_pane",
    "service_health_pane",
    "docs_browser_pane",
)


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
    badge_family_class: str
    honesty_fallback_token: str
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


def load_schema_enums(repo_root: Path, ref: str, defs_key: str) -> list[str]:
    """Best-effort enum lookup from a schema's $defs."""
    schema_path = repo_root / ref
    if not schema_path.exists():
        return []
    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    defs = schema.get("$defs", {})
    entry = defs.get(defs_key, {})
    if "enum" in entry and isinstance(entry["enum"], list):
        return [str(v) for v in entry["enum"]]
    return []


def apply_forced_overrides(
    row: dict[str, Any],
    forced_overrides: dict[str, Any],
) -> dict[str, Any]:
    row = copy.deepcopy(row)
    if not forced_overrides:
        return row

    if forced_overrides.get("drop_honesty_fallback_token"):
        row["honesty_fallback_token"] = ""

    if "drop_vocabulary_token" in forced_overrides:
        target = forced_overrides["drop_vocabulary_token"]
        tokens = row.get("vocabulary_tokens") or []
        if isinstance(tokens, list):
            row["vocabulary_tokens"] = [
                t for t in tokens
                if not (isinstance(t, dict) and t.get("token") == target)
            ]

    if "inject_unknown_token" in forced_overrides:
        token = forced_overrides["inject_unknown_token"]
        tokens = row.get("vocabulary_tokens") or []
        if isinstance(tokens, list):
            tokens = list(tokens)
            tokens.append(
                {
                    "token": token,
                    "role": "live_state_token",
                    "label": f"Injected unknown token {token}",
                }
            )
            row["vocabulary_tokens"] = tokens

    if "drop_consuming_surface" in forced_overrides:
        target = forced_overrides["drop_consuming_surface"]
        surfaces = row.get("consuming_surface_classes") or []
        if isinstance(surfaces, list):
            row["consuming_surface_classes"] = [
                s for s in surfaces if s != target
            ]

    if "rewrite_token_role" in forced_overrides:
        spec = forced_overrides["rewrite_token_role"]
        if isinstance(spec, dict):
            target_token = spec.get("token")
            target_role = spec.get("role")
            tokens = row.get("vocabulary_tokens") or []
            if isinstance(tokens, list):
                new_tokens = []
                for t in tokens:
                    if (
                        isinstance(t, dict)
                        and t.get("token") == target_token
                    ):
                        t = copy.deepcopy(t)
                        t["role"] = target_role
                    new_tokens.append(t)
                row["vocabulary_tokens"] = new_tokens

    return row


def validate_row(
    row: dict[str, Any],
    *,
    repo_root: Path,
    row_id_value: str,
    badge_family_class_vocab: set[str],
    consuming_surface_class_vocab: set[str],
    consumer_class_vocab: set[str],
    vocabulary_role_class_vocab: set[str],
    failure_drill_id_vocab: set[str],
) -> RowResult:
    row_id = ensure_str(row.get("row_id"), f"{row_id_value}.row_id")
    badge_family_class = ensure_str(
        row.get("badge_family_class"),
        f"{row_id_value}.badge_family_class",
    )

    result = RowResult(
        row_id=row_id,
        badge_family_class=badge_family_class,
        honesty_fallback_token=str(row.get("honesty_fallback_token") or ""),
    )

    # --- discriminator and version pins ---------------------------------
    if row.get("record_kind") != EXPECTED_RECORD_KIND:
        fail(
            result,
            "truth_source.row.record_kind_wrong",
            (
                f"record_kind must be {EXPECTED_RECORD_KIND!r}; got "
                f"{row.get('record_kind')!r}"
            ),
        )
    if (
        row.get("truth_source_row_schema_version")
        != EXPECTED_ROW_SCHEMA_VERSION
    ):
        fail(
            result,
            "truth_source.row.schema_version_wrong",
            (
                "truth_source_row_schema_version must be"
                f" {EXPECTED_ROW_SCHEMA_VERSION}; got"
                f" {row.get('truth_source_row_schema_version')!r}"
            ),
        )

    # --- closed badge family vocabulary ---------------------------------
    if badge_family_class not in badge_family_class_vocab:
        fail(
            result,
            "truth_source.row.badge_family_class_unknown",
            (
                f"badge_family_class {badge_family_class!r} is not in"
                " badge_family_class_vocabulary"
            ),
        )

    # --- support_export_compatible must be true -------------------------
    if row.get("support_export_compatible") is not True:
        fail(
            result,
            "truth_source.row.support_export_compatible_required",
            "support_export_compatible must be true on every row",
        )

    # --- vocabulary tokens ----------------------------------------------
    tokens = row.get("vocabulary_tokens")
    if not isinstance(tokens, list) or len(tokens) < 2:
        fail(
            result,
            "truth_source.row.vocabulary_tokens_required",
            "vocabulary_tokens must be a list with at least two tokens",
        )
        tokens = []

    # --- frozen token count lock ----------------------------------------
    frozen_token_count = row.get("frozen_token_count")
    if not isinstance(frozen_token_count, int) or frozen_token_count < 2:
        fail(
            result,
            "truth_source.frozen_token_count_invalid",
            (
                "frozen_token_count must be a positive integer (>= 2);"
                f" got {frozen_token_count!r}"
            ),
        )
    elif isinstance(tokens, list) and len(tokens) != frozen_token_count:
        fail(
            result,
            "truth_source.vocabulary_token_count_mismatch",
            (
                "vocabulary_tokens has"
                f" {len(tokens)} entries; the row's"
                f" frozen_token_count is {frozen_token_count}."
                " Adding or removing a token requires bumping"
                " frozen_token_count in the same change set."
            ),
        )

    token_set: set[str] = set()
    role_by_token: dict[str, str] = {}
    for idx, t in enumerate(tokens):
        if not isinstance(t, dict):
            fail(
                result,
                "truth_source.row.vocabulary_token_invalid",
                f"vocabulary_tokens[{idx}] must be an object",
            )
            continue
        tok = t.get("token")
        role = t.get("role")
        label = t.get("label")
        if not isinstance(tok, str) or not tok.strip():
            fail(
                result,
                "truth_source.vocabulary_token_unknown",
                (
                    f"vocabulary_tokens[{idx}].token must be a non-empty"
                    " string"
                ),
            )
            continue
        if not tok.replace("_", "").isalnum() or not tok[0].islower():
            fail(
                result,
                "truth_source.vocabulary_token_unknown",
                (
                    f"vocabulary_tokens[{idx}].token {tok!r} must be a"
                    " snake_case identifier starting with a lowercase"
                    " letter"
                ),
            )
        if role not in vocabulary_role_class_vocab:
            fail(
                result,
                "truth_source.vocabulary_token_role_unknown",
                (
                    f"vocabulary_tokens[{idx}].role {role!r} is not in"
                    " vocabulary_role_class_vocabulary"
                ),
            )
        if not isinstance(label, str) or not label.strip():
            fail(
                result,
                "truth_source.vocabulary_token_label_required",
                f"vocabulary_tokens[{idx}].label must be non-empty",
            )
        if tok in token_set:
            fail(
                result,
                "truth_source.vocabulary_token_duplicate",
                f"vocabulary_tokens[{idx}].token {tok!r} is duplicated",
            )
        token_set.add(tok)
        role_by_token[tok] = role if isinstance(role, str) else ""

    # --- honesty fallback token -----------------------------------------
    fallback = row.get("honesty_fallback_token")
    if not isinstance(fallback, str) or not fallback.strip():
        fail(
            result,
            "truth_source.honesty_fallback_token_missing",
            "honesty_fallback_token must be a non-empty token",
        )
    else:
        if fallback not in token_set:
            fail(
                result,
                "truth_source.honesty_fallback_token_not_in_vocabulary",
                (
                    f"honesty_fallback_token {fallback!r} must appear in"
                    " vocabulary_tokens"
                ),
            )
        elif role_by_token.get(fallback) != "honesty_fallback_token":
            fail(
                result,
                "truth_source.honesty_fallback_token_role_wrong",
                (
                    f"vocabulary token {fallback!r} must carry role"
                    " 'honesty_fallback_token' to back the row's"
                    " honesty fallback"
                ),
            )

    # Exactly one honesty_fallback token role per row.
    honesty_fallback_count = sum(
        1 for r in role_by_token.values() if r == "honesty_fallback_token"
    )
    if honesty_fallback_count > 1:
        fail(
            result,
            "truth_source.honesty_fallback_token_role_multiple",
            (
                "vocabulary_tokens must declare exactly one token with"
                " role 'honesty_fallback_token'; found"
                f" {honesty_fallback_count}"
            ),
        )

    # --- degraded-state honesty -----------------------------------------
    # Detect attempts to widen a known degraded state token to a live
    # role. The seed forbids relabelling tokens drawn from the
    # canonical degraded-state list (stale, unverified, etc.) to
    # live_state_token without a decision row.
    canonical_degraded_tokens = {
        "stale",
        "unverified",
        "degraded_cached",
        "incompatible_drift_detected",
        "pre_release_unverified",
    }
    for tok, role in role_by_token.items():
        if tok in canonical_degraded_tokens and role == "live_state_token":
            fail(
                result,
                "truth_source.degraded_state_token_widened",
                (
                    f"token {tok!r} carries role {role!r}; the seed"
                    " forbids widening a canonical degraded-state token"
                    " to a live state without a new decision row."
                ),
            )

    # --- seed-placeholder honesty (provenance + service-health) ---------
    if badge_family_class in {"provenance_row_state", "service_health_state"}:
        if "seed_placeholder_awaiting_wiring" not in token_set:
            fail(
                result,
                "truth_source.seed_placeholder_token_required",
                (
                    f"row {row_id!r} (family {badge_family_class!r})"
                    " must declare 'seed_placeholder_awaiting_wiring' in"
                    " vocabulary_tokens with role"
                    " 'seed_placeholder_token'"
                ),
            )
        elif (
            role_by_token.get("seed_placeholder_awaiting_wiring")
            != "seed_placeholder_token"
        ):
            fail(
                result,
                "truth_source.seed_placeholder_role_widened",
                (
                    "vocabulary token 'seed_placeholder_awaiting_wiring'"
                    f" must carry role 'seed_placeholder_token' on the"
                    f" {badge_family_class!r} row"
                ),
            )

    # --- consuming surface coverage -------------------------------------
    surfaces = row.get("consuming_surface_classes")
    if not isinstance(surfaces, list):
        fail(
            result,
            "truth_source.consuming_surface_classes_required",
            "consuming_surface_classes must be a list",
        )
        surfaces = []
    for idx, s in enumerate(surfaces):
        if s not in consuming_surface_class_vocab:
            fail(
                result,
                "truth_source.consuming_surface_class_unknown",
                (
                    f"consuming_surface_classes[{idx}] {s!r} is not in"
                    " consuming_surface_class_vocabulary"
                ),
            )
    missing_surfaces = [
        s for s in REQUIRED_SURFACES if s not in surfaces
    ]
    if missing_surfaces:
        fail(
            result,
            "truth_source.required_consuming_surface_missing",
            (
                "consuming_surface_classes must include all of"
                f" {list(REQUIRED_SURFACES)}; missing:"
                f" {missing_surfaces}"
            ),
        )

    # --- named runtime consumer -----------------------------------------
    named_consumer = ensure_dict(
        row.get("named_runtime_consumer"),
        f"{row_id}.named_runtime_consumer",
    )
    consumer_ref = ensure_str(
        named_consumer.get("consumer_ref"),
        f"{row_id}.named_runtime_consumer.consumer_ref",
    )
    if not artifact_ref_exists(repo_root, consumer_ref):
        fail(
            result,
            "truth_source.named_runtime_consumer_missing",
            (
                "named_runtime_consumer.consumer_ref does not exist:"
                f" {consumer_ref}"
            ),
        )
    consumer_class = ensure_str(
        named_consumer.get("consumer_class"),
        f"{row_id}.named_runtime_consumer.consumer_class",
    )
    if consumer_class not in consumer_class_vocab:
        fail(
            result,
            "truth_source.named_runtime_consumer_consumer_class_unknown",
            (
                f"named_runtime_consumer.consumer_class"
                f" {consumer_class!r} is not in"
                " consumer_class_vocabulary"
            ),
        )
    consumed_fields = ensure_list(
        named_consumer.get("consumed_fields"),
        f"{row_id}.named_runtime_consumer.consumed_fields",
    )
    if not consumed_fields:
        fail(
            result,
            "truth_source.named_runtime_consumer_consumed_fields_empty",
            (
                "named_runtime_consumer.consumed_fields must declare at"
                " least one field"
            ),
        )

    # --- example payload ------------------------------------------------
    example_ref = ensure_str(
        row.get("example_payload_ref"),
        f"{row_id}.example_payload_ref",
    )
    example_path = repo_root / example_ref
    if not example_path.exists():
        fail(
            result,
            "truth_source.example_payload_missing",
            f"example_payload_ref does not exist: {example_ref}",
        )
        example_doc = None
    else:
        try:
            example_doc = json.loads(
                example_path.read_text(encoding="utf-8")
            )
        except json.JSONDecodeError as exc:
            fail(
                result,
                "truth_source.example_payload_invalid_json",
                (
                    f"example_payload_ref {example_ref} is not valid"
                    f" JSON: {exc}"
                ),
            )
            example_doc = None

    if isinstance(example_doc, dict):
        if example_doc.get("registry_example_kind") != EXPECTED_EXAMPLE_KIND:
            fail(
                result,
                "truth_source.example_payload_kind_wrong",
                (
                    "example_payload.registry_example_kind must be"
                    f" {EXPECTED_EXAMPLE_KIND!r}; got"
                    f" {example_doc.get('registry_example_kind')!r}"
                ),
            )
        if example_doc.get("truth_source_row_id") != row_id:
            fail(
                result,
                "truth_source.example_payload_row_id_mismatch",
                (
                    "example_payload.truth_source_row_id must match"
                    f" the row's row_id {row_id!r}; got"
                    f" {example_doc.get('truth_source_row_id')!r}"
                ),
            )
        if example_doc.get("badge_family_class") != badge_family_class:
            fail(
                result,
                "truth_source.example_payload_badge_family_mismatch",
                (
                    "example_payload.badge_family_class must match the"
                    f" row's badge_family_class {badge_family_class!r};"
                    f" got {example_doc.get('badge_family_class')!r}"
                ),
            )
        if (
            isinstance(fallback, str)
            and example_doc.get("pinned_honesty_fallback_token") != fallback
        ):
            fail(
                result,
                "truth_source.example_payload_honesty_fallback_mismatch",
                (
                    "example_payload.pinned_honesty_fallback_token must"
                    f" match the row's honesty_fallback_token"
                    f" {fallback!r}; got"
                    f" {example_doc.get('pinned_honesty_fallback_token')!r}"
                ),
            )
        surface_examples = example_doc.get("consuming_surface_examples")
        if not isinstance(surface_examples, list) or len(surface_examples) < 3:
            fail(
                result,
                "truth_source.example_payload_surfaces_too_few",
                (
                    "example_payload.consuming_surface_examples must"
                    " carry at least three rendered surface examples"
                ),
            )
        else:
            for idx, example in enumerate(surface_examples):
                if not isinstance(example, dict):
                    fail(
                        result,
                        "truth_source.example_payload_surface_invalid",
                        (
                            "example_payload.consuming_surface_examples["
                            f"{idx}] must be an object"
                        ),
                    )
                    continue
                rendered = example.get("rendered_token")
                if rendered not in token_set:
                    fail(
                        result,
                        "truth_source.example_payload_rendered_token_unknown",
                        (
                            "example_payload.consuming_surface_examples["
                            f"{idx}].rendered_token {rendered!r} is not"
                            " in the row's vocabulary_tokens"
                        ),
                    )
                surface_class = example.get("surface_class")
                if surface_class not in consuming_surface_class_vocab:
                    fail(
                        result,
                        "truth_source.example_payload_surface_class_unknown",
                        (
                            "example_payload.consuming_surface_examples["
                            f"{idx}].surface_class {surface_class!r}"
                            " is not in"
                            " consuming_surface_class_vocabulary"
                        ),
                    )

    # --- failure_drill shape --------------------------------------------
    drill = ensure_dict(
        row.get("failure_drill"), f"{row_id}.failure_drill"
    )
    drill_id = ensure_str(
        drill.get("drill_id"), f"{row_id}.failure_drill.drill_id"
    )
    if drill_id not in failure_drill_id_vocab:
        fail(
            result,
            "truth_source.failure_drill_id_unknown",
            (
                f"failure_drill.drill_id {drill_id!r} is not in"
                " failure_drill_id_vocabulary"
            ),
        )
    forced_input = ensure_dict(
        drill.get("forced_input"),
        f"{row_id}.failure_drill.forced_input",
    )
    if not forced_input:
        fail(
            result,
            "truth_source.failure_drill_forced_input_empty",
            "failure_drill.forced_input must declare at least one drift",
        )
    ensure_str(
        drill.get("expected_check_id"),
        f"{row_id}.failure_drill.expected_check_id",
    )
    ensure_str(
        drill.get("actionable_next_action"),
        f"{row_id}.failure_drill.actionable_next_action",
    )

    result.diagnostics.update(
        {
            "row_id": row_id,
            "badge_family_class": badge_family_class,
            "honesty_fallback_token": fallback
            if isinstance(fallback, str)
            else None,
            "token_count": len(token_set),
            "consuming_surfaces": sorted(set(surfaces))
            if isinstance(surfaces, list)
            else [],
            "failure_drill": {
                "drill_id": drill_id,
                "expected_check_id": drill.get("expected_check_id"),
            },
        }
    )

    if not result.failed_checks:
        pass_(result, f"row {row_id} passes")

    return result


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
    if schema_version != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding(
                severity="error",
                check_id="truth_source.envelope_schema_version_wrong",
                message=(
                    "matrix schema_version must be"
                    f" {EXPECTED_SCHEMA_VERSION}; got"
                    f" {schema_version!r}"
                ),
                remediation="Bump runner together with the envelope schema.",
            )
        )

    matrix_id = ensure_str(matrix.get("matrix_id"), "matrix.matrix_id")
    if matrix_id != EXPECTED_MATRIX_ID:
        findings.append(
            Finding(
                severity="error",
                check_id="truth_source.envelope_matrix_id_wrong",
                message=(
                    f"matrix_id must be {EXPECTED_MATRIX_ID!r}; got"
                    f" {matrix_id!r}"
                ),
                remediation="Restore the canonical envelope matrix id.",
            )
        )

    ensure_str(matrix.get("status"), "matrix.status")
    ensure_str(matrix.get("owner_dri"), "matrix.owner_dri")

    overview_page = ensure_str(
        matrix.get("overview_page"), "matrix.overview_page"
    )
    if not artifact_ref_exists(repo_root, overview_page):
        findings.append(
            Finding(
                severity="error",
                check_id="truth_source.envelope_overview_page_missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation=(
                    "Create the reviewer landing page or fix the path."
                ),
                ref=overview_page,
            )
        )

    badge_draft = ensure_str(
        matrix.get("badge_vocabulary_draft_ref"),
        "matrix.badge_vocabulary_draft_ref",
    )
    if not artifact_ref_exists(repo_root, badge_draft):
        findings.append(
            Finding(
                severity="error",
                check_id="truth_source.envelope_badge_vocabulary_draft_missing",
                message=(
                    "badge_vocabulary_draft_ref does not exist:"
                    f" {badge_draft}"
                ),
                remediation=(
                    "Land docs/help/badge_vocabulary_draft.md or fix the"
                    " path."
                ),
                ref=badge_draft,
            )
        )

    for key in ("build_identity_ref",):
        ref = ensure_str(matrix.get(key), f"matrix.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"truth_source.envelope_{key}_missing",
                    message=f"{key} does not resolve: {ref}",
                    remediation=(
                        "Fix the path or land the referenced artifact."
                    ),
                    ref=ref,
                )
            )

    validation_lane_ref = ensure_str(
        matrix.get("validation_lane_ref"),
        "matrix.validation_lane_ref",
    )
    if not artifact_ref_exists(repo_root, validation_lane_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="truth_source.envelope_validation_lane_ref_missing",
                message=(
                    "validation_lane_ref base does not exist:"
                    f" {validation_lane_ref}"
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

    badge_family_class_vocab = load_vocab("badge_family_class_vocabulary")
    vocabulary_role_class_vocab = load_vocab("vocabulary_role_class_vocabulary")
    consuming_surface_class_vocab = load_vocab(
        "consuming_surface_class_vocabulary"
    )
    consumer_class_vocab = load_vocab("consumer_class_vocabulary")
    failure_drill_id_vocab = load_vocab("failure_drill_id_vocabulary")
    required_badge_family_class_coverage = load_vocab(
        "required_badge_family_class_coverage"
    )
    required_consuming_surface_class_coverage = load_vocab(
        "required_consuming_surface_class_coverage"
    )

    envelope_schema_rel = args.envelope_schema

    def assert_vocab_matches_schema(
        matrix_vocab: set[str], defs_key: str, name: str
    ) -> None:
        schema_enum = set(
            load_schema_enums(repo_root, envelope_schema_rel, defs_key)
        )
        if not schema_enum:
            return
        diff = matrix_vocab.symmetric_difference(schema_enum)
        if diff:
            findings.append(
                Finding(
                    severity="error",
                    check_id=(
                        f"truth_source.envelope_{name}_disagrees_with_schema"
                    ),
                    message=(
                        f"matrix.{name} disagrees with"
                        f" {envelope_schema_rel}#$defs.{defs_key}; "
                        f"matrix-only:"
                        f" {sorted(matrix_vocab - schema_enum)};"
                        f" schema-only:"
                        f" {sorted(schema_enum - matrix_vocab)}"
                    ),
                    remediation=(
                        "Keep the matrix vocabulary in lock-step with"
                        " the envelope schema; the schema is canonical."
                    ),
                )
            )

    assert_vocab_matches_schema(
        badge_family_class_vocab, "badge_family_class", "badge_family_class_vocabulary"
    )
    assert_vocab_matches_schema(
        vocabulary_role_class_vocab,
        "vocabulary_role_class",
        "vocabulary_role_class_vocabulary",
    )
    assert_vocab_matches_schema(
        consuming_surface_class_vocab,
        "consuming_surface_class",
        "consuming_surface_class_vocabulary",
    )
    assert_vocab_matches_schema(
        consumer_class_vocab, "consumer_class", "consumer_class_vocabulary"
    )

    # Required consuming-surface coverage must include the four named surfaces.
    missing_required_coverage = set(REQUIRED_SURFACES) - (
        required_consuming_surface_class_coverage
    )
    if missing_required_coverage:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "truth_source.envelope_required_consuming_surfaces_missing"
                ),
                message=(
                    "required_consuming_surface_class_coverage must"
                    f" include all of {list(REQUIRED_SURFACES)};"
                    f" missing: {sorted(missing_required_coverage)}"
                ),
                remediation=(
                    "Restore the four required surfaces to"
                    " required_consuming_surface_class_coverage."
                ),
            )
        )

    # --force-drill plumbing.
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

    rows = ensure_list(matrix.get("rows"), "matrix.rows")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="truth_source.envelope_rows_empty",
                message="matrix.rows must declare at least one row",
                remediation="Seed the required rows.",
            )
        )

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_family_classes: set[str] = set()
    forced_replay_record: dict[str, Any] | None = None

    for idx, raw_row in enumerate(rows):
        raw_row = ensure_dict(raw_row, f"matrix.rows[{idx}]")
        row_id_local = ensure_str(
            raw_row.get("row_id"), f"matrix.rows[{idx}].row_id"
        )
        original_row = copy.deepcopy(raw_row)
        drill = ensure_dict(
            raw_row.get("failure_drill"),
            f"{row_id_local}.failure_drill",
        )
        drill_id_local = ensure_str(
            drill.get("drill_id"),
            f"{row_id_local}.failure_drill.drill_id",
        )

        applied_overrides: dict[str, Any] = {}
        replay_row_payload = raw_row
        if forced_row_id is not None and row_id_local == forced_row_id:
            if drill_id_local != forced_drill_id:
                raise SystemExit(
                    f"--force-drill drill_id {forced_drill_id!r} does"
                    f" not match the row's failure_drill.drill_id"
                    f" {drill_id_local!r}"
                )
            applied_overrides = ensure_dict(
                drill.get("forced_input"),
                f"{row_id_local}.failure_drill.forced_input",
            )
            replay_row_payload = apply_forced_overrides(
                raw_row, applied_overrides
            )

        result = validate_row(
            replay_row_payload,
            repo_root=repo_root,
            row_id_value=row_id_local,
            badge_family_class_vocab=badge_family_class_vocab,
            consuming_surface_class_vocab=consuming_surface_class_vocab,
            consumer_class_vocab=consumer_class_vocab,
            vocabulary_role_class_vocab=vocabulary_role_class_vocab,
            failure_drill_id_vocab=failure_drill_id_vocab,
        )
        if applied_overrides:
            result.diagnostics["forced_overrides_applied"] = applied_overrides
        row_results.append(result)

        if result.row_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="truth_source.rows_duplicate_id",
                    message=f"duplicate row_id: {result.row_id}",
                    remediation="row_ids must be unique.",
                    ref=result.row_id,
                )
            )
        seen_ids.add(result.row_id)
        seen_family_classes.add(
            ensure_str(
                original_row.get("badge_family_class"),
                f"{result.row_id}.badge_family_class",
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

    missing_families = (
        required_badge_family_class_coverage - seen_family_classes
    )
    if missing_families:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "truth_source.coverage_missing_required_badge_family_classes"
                ),
                message=(
                    "matrix must seed at least one row for each required"
                    f" badge_family_class:"
                    f" {sorted(required_badge_family_class_coverage)};"
                    f" missing: {sorted(missing_families)}"
                ),
                remediation=(
                    "Add the missing rows so every required badge"
                    " family is exercised."
                ),
            )
        )

    # Promote per-row failures into findings, skipping the targeted row
    # under --force-drill so the runner's exit can reflect the drill verdict.
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
                        "check_id", "truth_source.row_failed_check"
                    ),
                    message=(
                        f"{result.row_id}: {failure.get('message', '')}"
                    ),
                    remediation=(
                        "Re-align the row with the truth-source seed"
                        " contract or fix the drift in the seed;"
                        " failures are reported with the precise"
                        " actionable check_id."
                    ),
                    ref=result.row_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "truth_source_seed_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": ensure_str(matrix.get("owner_dri"), "matrix.owner_dri"),
        "matrix_ref": matrix_rel,
        "envelope_schema_ref": args.envelope_schema,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/help/m1_truth_source_seed_lane/"
            "run_m1_truth_source_seed_lane.py --repo-root ."
        ),
        "status": status,
        "required_badge_family_class_coverage": sorted(
            required_badge_family_class_coverage
        ),
        "observed_badge_family_classes": sorted(seen_family_classes),
        "rows": [
            {
                "row_id": r.row_id,
                "badge_family_class": r.badge_family_class,
                "honesty_fallback_token": r.honesty_fallback_token,
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
        json.dumps(capture, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )

    label = "truth-source-seed"
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
                f"[{label}] forced drill"
                f" {forced_replay_record['drill_id']} on"
                f" {forced_replay_record['row_id']} reproduced"
                f" {forced_replay_record['expected_check_id']}"
            )
            return 0
        print(
            f"[{label}] forced drill {forced_replay_record['drill_id']}"
            f" on {forced_replay_record['row_id']} did NOT reproduce"
            f" {forced_replay_record['expected_check_id']};"
            f" observed: {forced_replay_record['observed_failed_check_ids']}"
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[truth-source-seed] interrupted", file=sys.stderr)
        sys.exit(130)
