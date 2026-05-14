#!/usr/bin/env python3
"""Validate and render the external alpha launch-bundle manifests."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_TSJS_BUNDLE_REL = "artifacts/bundles/tsjs_launch_bundle_alpha.yaml"
DEFAULT_PYTHON_BUNDLE_REL = "artifacts/bundles/python_launch_bundle_alpha.yaml"
DEFAULT_CERTIFICATION_REL = "artifacts/certification/m2_archetype_seed_rows.yaml"
DEFAULT_FIXTURE_REGISTER_REL = "artifacts/benchmarks/m2_fixture_register.yaml"
DEFAULT_MATRIX_REL = "artifacts/milestones/m2/alpha_wedge_matrix.yaml"
DEFAULT_PROOF_PACKET_REL = "artifacts/milestones/m2/proof_packets/launch_bundles_and_archetypes.md"
DEFAULT_START_CENTER_REL = "crates/aureline-shell/src/start_center/mod.rs"
DEFAULT_SCORECARD_REL = "artifacts/compat/workflow_bundle_scorecard_sample.json"
DEFAULT_DRIFT_PACKET_REL = "artifacts/compat/bundle_drift_packet_sample.json"

EXPECTED_BUNDLES = {
    "launch_bundle:typescript_web_app.seed": {
        "path": DEFAULT_TSJS_BUNDLE_REL,
        "wedge_ref": "alpha_wedge:typescript_javascript",
        "archetype_row_ref": "archetype_row:ts_web_app_or_service",
        "archetype_seed_row_ref": "archetype_certification_seed:ts_web_app_or_service",
        "benchmark_fixture_register_row_ref": "fixture_register:external_alpha.ts_web_app_reference",
    },
    "launch_bundle:python_service_or_data_app.seed": {
        "path": DEFAULT_PYTHON_BUNDLE_REL,
        "wedge_ref": "alpha_wedge:python",
        "archetype_row_ref": "archetype_row:python_service_or_data_app",
        "archetype_seed_row_ref": "archetype_certification_seed:python_service_or_data_app",
        "benchmark_fixture_register_row_ref": "fixture_register:external_alpha.python_service_data_reference",
    },
}

REQUIRED_DIFF_SECTIONS = {
    "extensions",
    "settings_profile_presets",
    "keymap_mode",
    "tasks_recipes",
    "docs_tour_packs",
    "scaffold_template_refs",
    "trust_permission_notes",
    "rollback_checkpoint",
}

REQUIRED_USER_CHOICES = {
    "apply",
    "compare",
    "dismiss",
    "keep_local",
    "adopt_bundle",
    "compare_again_later",
}

REQUIRED_PRESERVATION_GUARANTEES = {
    "preserve_user_created_files",
    "preserve_imported_mappings",
    "preserve_local_history",
    "preserve_non_bundle_owned_artifacts",
}

REQUIRED_SCORECARD_STATUS_CLASSES = {
    "certified",
    "managed_approved",
    "community",
    "imported",
    "local_draft",
    "partial",
    "retest_pending",
}

REQUIRED_LIFECYCLE_ACTIONS = {
    "apply",
    "compare",
    "keep_local",
    "adopt_bundle",
    "remove_bundle",
    "rebase_to_bundle",
}

REQUIRED_DRIFT_STATES = {
    "local_override",
    "missing_artifact",
    "bundle_version_drift",
}

REQUIRED_REMOVE_SAFE_CLASSES = {
    "safe_to_remove_no_user_data",
    "safe_to_remove_user_overlay_preserved",
    "review_required_user_data_co_resident",
}

REQUIRED_MIRROR_SOURCES = {
    "public_registry",
    "approved_mirror",
    "offline_bundle",
}

ALLOWED_CURRENT_SUPPORT_CLASSES = {
    "experimental",
    "alpha_limited",
}

ALLOWED_CERTIFICATION_STATES = {
    "seed_not_certified",
    "retest_pending",
    "evidence_stale",
}

REQUIRED_GALLERY_FIELDS = {
    "bundle_id",
    "persona_label",
    "signer",
    "channel",
    "compatible_aureline_range",
    "archetype_certification_state",
    "mirror_availability",
}

REQUIRED_ACCEPTANCE_STATES = {
    "bundle_has_extensions_settings_tasks_docs_and_rollback",
    "bundle_links_one_archetype_and_one_benchmark_fixture",
    "bundle_scope_does_not_exceed_current_evidence",
    "archetype_rows_link_bundle_fixture_and_packet",
    "certification_state_is_seed_not_certified",
    "badges_open_underlying_packet",
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


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--tsjs-bundle", default=DEFAULT_TSJS_BUNDLE_REL)
    parser.add_argument("--python-bundle", default=DEFAULT_PYTHON_BUNDLE_REL)
    parser.add_argument("--certification", default=DEFAULT_CERTIFICATION_REL)
    parser.add_argument("--fixture-register", default=DEFAULT_FIXTURE_REGISTER_REL)
    parser.add_argument("--matrix", default=DEFAULT_MATRIX_REL)
    parser.add_argument("--proof-packet", default=DEFAULT_PROOF_PACKET_REL)
    parser.add_argument("--start-center", default=DEFAULT_START_CENTER_REL)
    parser.add_argument("--scorecard", default=DEFAULT_SCORECARD_REL)
    parser.add_argument("--drift-packet", default=DEFAULT_DRIFT_PACKET_REL)
    parser.add_argument("--report", default=None)
    parser.add_argument(
        "--render-gallery",
        action="store_true",
        help="Print the CLI bundle-gallery projection after validation.",
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
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), permitted_classes: [], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path}: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


def read_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


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


def ensure_int(value: Any, label: str) -> int:
    if not isinstance(value, int):
        raise SystemExit(f"{label} must be an integer")
    return value


def parse_iso_date(value: str, label: str) -> None:
    try:
        dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(f"{label} must be a YYYY-MM-DD date, got {value!r}") from exc


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    clean = strip_fragment(ref)
    return bool(clean) and (repo_root / clean).exists()


def validate_path_ref(repo_root: Path, ref: str, label: str, findings: list[Finding]) -> None:
    if not artifact_ref_exists(repo_root, ref):
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.missing_ref",
                message=f"{label} does not resolve: {ref}",
                remediation="Fix the path or seed the referenced artifact.",
                ref=ref,
            )
        )


def validate_header(payload: dict[str, Any], label: str, findings: list[Finding]) -> None:
    schema_version = ensure_int(payload.get("schema_version"), f"{label}.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.schema_version.unsupported",
                message=f"{label}.schema_version must be 1, got {schema_version}",
                remediation="Update this validator in the same change that changes the schema.",
            )
        )
    parse_iso_date(ensure_str(payload.get("as_of"), f"{label}.as_of"), f"{label}.as_of")
    ensure_str(payload.get("owner"), f"{label}.owner")


def collect_matrix_bundle_bindings(matrix: dict[str, Any]) -> dict[str, str]:
    bindings: dict[str, str] = {}
    for idx, raw_row in enumerate(ensure_list(matrix.get("wedge_rows"), "matrix.wedge_rows")):
        row = ensure_dict(raw_row, f"matrix.wedge_rows[{idx}]")
        wedge_ref = ensure_str(row.get("wedge_id"), f"matrix.wedge_rows[{idx}].wedge_id")
        for raw_bundle_ref in ensure_list(row.get("launch_bundle_refs"), f"matrix.wedge_rows[{idx}].launch_bundle_refs"):
            bindings[ensure_str(raw_bundle_ref, "matrix.launch_bundle_refs[]")] = wedge_ref
    return bindings


def collect_fixture_rows(register: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows: dict[str, dict[str, Any]] = {}
    for idx, raw_row in enumerate(ensure_list(register.get("reference_workspaces"), "fixture_register.reference_workspaces")):
        row = ensure_dict(raw_row, f"fixture_register.reference_workspaces[{idx}]")
        row_id = ensure_str(row.get("register_row_id"), f"fixture_register.reference_workspaces[{idx}].register_row_id")
        rows[row_id] = row
    return rows


def collect_certification_rows(register: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows: dict[str, dict[str, Any]] = {}
    for idx, raw_row in enumerate(ensure_list(register.get("archetype_seed_rows"), "certification.archetype_seed_rows")):
        row = ensure_dict(raw_row, f"certification.archetype_seed_rows[{idx}]")
        row_id = ensure_str(row.get("row_id"), f"certification.archetype_seed_rows[{idx}].row_id")
        rows[row_id] = row
    return rows


def validate_component_sets(bundle: dict[str, Any], bundle_id: str, findings: list[Finding]) -> None:
    extensions = ensure_dict(bundle.get("extensions"), f"{bundle_id}.extensions")
    if not ensure_list(extensions.get("required"), f"{bundle_id}.extensions.required"):
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.extensions.required.empty",
                message=f"{bundle_id} must name required extensions",
                remediation="Add at least one required extension row.",
                ref=bundle_id,
            )
        )

    settings = ensure_dict(bundle.get("settings_profile_preset"), f"{bundle_id}.settings_profile_preset")
    ensure_str(settings.get("keymap_mode"), f"{bundle_id}.settings_profile_preset.keymap_mode")
    if not ensure_list(settings.get("settings"), f"{bundle_id}.settings_profile_preset.settings"):
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.settings.empty",
                message=f"{bundle_id} must name settings/profile preset changes",
                remediation="Add settings rows and keep keymap mode explicit.",
                ref=bundle_id,
            )
        )

    if not ensure_list(bundle.get("tasks_and_recipes"), f"{bundle_id}.tasks_and_recipes"):
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.tasks.empty",
                message=f"{bundle_id} must name tasks or recipes",
                remediation="Add task or recipe rows to the bundle manifest.",
                ref=bundle_id,
            )
        )
    if not ensure_list(bundle.get("docs_tour_packs"), f"{bundle_id}.docs_tour_packs"):
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.docs.empty",
                message=f"{bundle_id} must name docs or tour packs",
                remediation="Add docs pack or tour rows to the bundle manifest.",
                ref=bundle_id,
            )
        )
    if not ensure_list(bundle.get("scaffold_template_refs"), f"{bundle_id}.scaffold_template_refs"):
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.scaffold.empty",
                message=f"{bundle_id} must name scaffold/template references",
                remediation="Add scaffold/template reference rows.",
                ref=bundle_id,
            )
        )

    rollback = ensure_dict(bundle.get("rollback_semantics"), f"{bundle_id}.rollback_semantics")
    ensure_str(rollback.get("checkpoint_creation"), f"{bundle_id}.rollback_semantics.checkpoint_creation")
    if rollback.get("restore_preview_required") is not True:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.rollback.restore_preview_required",
                message=f"{bundle_id} rollback must require preview",
                remediation="Set rollback_semantics.restore_preview_required to true.",
                ref=bundle_id,
            )
        )


def validate_review_and_drift(bundle: dict[str, Any], bundle_id: str, findings: list[Finding]) -> None:
    review = ensure_dict(bundle.get("install_update_review"), f"{bundle_id}.install_update_review")
    coherent_diff = ensure_dict(review.get("coherent_diff"), f"{bundle_id}.install_update_review.coherent_diff")
    sections = set(ensure_list(coherent_diff.get("sections"), f"{bundle_id}.install_update_review.coherent_diff.sections"))
    missing_sections = REQUIRED_DIFF_SECTIONS - sections
    if missing_sections:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.install_diff.sections.missing",
                message=f"{bundle_id} install/update diff omits required sections",
                remediation="Use one coherent diff over extensions, profile/keymap, tasks, docs, scaffold, trust, and rollback.",
                ref=bundle_id,
                details={"missing": sorted(missing_sections)},
            )
        )
    rows = ensure_dict(coherent_diff.get("rows"), f"{bundle_id}.install_update_review.coherent_diff.rows")
    for required_section in REQUIRED_DIFF_SECTIONS - {"keymap_mode"}:
        ensure_dict(rows.get(required_section), f"{bundle_id}.install_update_review.coherent_diff.rows.{required_section}")

    choices = set(ensure_list(review.get("action_vocabulary"), f"{bundle_id}.install_update_review.action_vocabulary"))
    missing_choices = REQUIRED_USER_CHOICES - choices
    if missing_choices:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.install_review.choices.missing",
                message=f"{bundle_id} install/update review omits required user choices",
                remediation="Expose Apply, Compare, Dismiss, Keep local, Adopt bundle, and Compare again later.",
                ref=bundle_id,
                details={"missing": sorted(missing_choices)},
            )
        )

    mirror = ensure_dict(review.get("mirror_offline_review"), f"{bundle_id}.install_update_review.mirror_offline_review")
    mirror_sources = set(ensure_list(mirror.get("source_vocabulary"), f"{bundle_id}.install_update_review.mirror_offline_review.source_vocabulary"))
    missing_sources = REQUIRED_MIRROR_SOURCES - mirror_sources
    if missing_sources:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.mirror_review.sources.missing",
                message=f"{bundle_id} mirror/offline review omits required source vocabulary",
                remediation="Use public_registry, approved_mirror, and offline_bundle on mirror/offline review.",
                ref=bundle_id,
                details={"missing": sorted(missing_sources)},
            )
        )
    if mirror.get("coherent_diff_sections_match_online") is not True:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.mirror_review.diff_vocab_mismatch",
                message=f"{bundle_id} mirror/offline review must preserve online diff vocabulary",
                remediation="Set coherent_diff_sections_match_online to true and keep section names aligned.",
                ref=bundle_id,
            )
        )

    drift = ensure_dict(bundle.get("recommendation_and_drift"), f"{bundle_id}.recommendation_and_drift")
    drift_choices = set(ensure_list(drift.get("user_choices"), f"{bundle_id}.recommendation_and_drift.user_choices"))
    missing_drift_choices = REQUIRED_USER_CHOICES - drift_choices
    if missing_drift_choices:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.drift.choices.missing",
                message=f"{bundle_id} drift states omit required user choices",
                remediation="Expose Apply, Compare, Dismiss, Keep local, Adopt bundle, and Compare again later.",
                ref=bundle_id,
                details={"missing": sorted(missing_drift_choices)},
            )
        )
    guarantees = set(ensure_list(drift.get("preservation_guarantees"), f"{bundle_id}.recommendation_and_drift.preservation_guarantees"))
    missing_guarantees = REQUIRED_PRESERVATION_GUARANTEES - guarantees
    if missing_guarantees:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.drift.preservation.missing",
                message=f"{bundle_id} does not preserve all required local artifact classes",
                remediation="Preserve user-created files, imported mappings, local history, and non-bundle-owned artifacts.",
                ref=bundle_id,
                details={"missing": sorted(missing_guarantees)},
            )
        )


def validate_consumer_projection(repo_root: Path, bundle: dict[str, Any], bundle_id: str, findings: list[Finding]) -> None:
    projection = ensure_dict(bundle.get("consumer_projection"), f"{bundle_id}.consumer_projection")
    gallery = ensure_dict(projection.get("start_center_gallery"), f"{bundle_id}.consumer_projection.start_center_gallery")
    validate_path_ref(
        repo_root,
        ensure_str(gallery.get("surface_ref"), f"{bundle_id}.consumer_projection.start_center_gallery.surface_ref"),
        "bundle.consumer_projection.start_center_gallery.surface_ref",
        findings,
    )
    exposed = set(ensure_list(gallery.get("exposes"), f"{bundle_id}.consumer_projection.start_center_gallery.exposes"))
    missing_fields = REQUIRED_GALLERY_FIELDS - exposed
    if missing_fields:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.consumer_projection.gallery_fields.missing",
                message=f"{bundle_id} Start Center projection omits required fields",
                remediation="Expose bundle id, persona/stack label, signer/source, channel, compatible range, certification state, and mirror availability.",
                ref=bundle_id,
                details={"missing": sorted(missing_fields)},
            )
        )
    cli = ensure_dict(projection.get("cli_summary"), f"{bundle_id}.consumer_projection.cli_summary")
    ensure_str(cli.get("command"), f"{bundle_id}.consumer_projection.cli_summary.command")
    if cli.get("exposes_same_fields_as_start_center") is not True:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.consumer_projection.cli_fields",
                message=f"{bundle_id} CLI projection must expose the same fields as Start Center",
                remediation="Set exposes_same_fields_as_start_center to true and keep the renderer in sync.",
                ref=bundle_id,
            )
        )


def validate_badges(repo_root: Path, bundle: dict[str, Any], bundle_id: str, findings: list[Finding]) -> None:
    badges = ensure_dict(bundle.get("badge_actions"), f"{bundle_id}.badge_actions")
    for key in (
        "launch_bundle_badge_opens",
        "certified_or_archetype_badge_opens",
        "compatibility_badge_opens",
    ):
        ref = ensure_str(badges.get(key), f"{bundle_id}.badge_actions.{key}")
        validate_path_ref(repo_root, ref, f"bundle.badge_actions.{key}", findings)


def validate_scorecard_packet(
    repo_root: Path,
    packet: dict[str, Any],
    bundles_by_id: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> dict[str, dict[str, Any]]:
    if ensure_str(packet.get("record_kind"), "scorecard.record_kind") != "workflow_bundle_scorecard_packet_record":
        findings.append(
            Finding(
                severity="error",
                check_id="scorecard.record_kind",
                message="workflow bundle scorecard packet has the wrong record kind",
                remediation="Use record_kind = workflow_bundle_scorecard_packet_record.",
                ref=DEFAULT_SCORECARD_REL,
            )
        )
    if ensure_int(packet.get("bundle_scorecard_schema_version"), "scorecard.bundle_scorecard_schema_version") != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="scorecard.schema_version",
                message="workflow bundle scorecard schema version must be 1",
                remediation="Update the validator alongside any schema-version change.",
                ref=DEFAULT_SCORECARD_REL,
            )
        )

    vocabulary = {
        ensure_str(raw.get("status_class"), "scorecard.status_vocabulary[].status_class")
        for raw in (ensure_dict(raw, "scorecard.status_vocabulary[]") for raw in ensure_list(packet.get("status_vocabulary"), "scorecard.status_vocabulary"))
    }
    missing_statuses = REQUIRED_SCORECARD_STATUS_CLASSES - vocabulary
    if missing_statuses:
        findings.append(
            Finding(
                severity="error",
                check_id="scorecard.status_vocabulary.missing",
                message="workflow bundle scorecard omits required explicit status classes",
                remediation="Include Certified, Managed approved, Community, Imported, Local draft, Partial, and Retest pending classes.",
                ref=DEFAULT_SCORECARD_REL,
                details={"missing": sorted(missing_statuses)},
            )
        )

    rows_by_bundle: dict[str, dict[str, Any]] = {}
    for raw_row in ensure_list(packet.get("rows"), "scorecard.rows"):
        row = ensure_dict(raw_row, "scorecard.rows[]")
        bundle = ensure_dict(row.get("bundle"), "scorecard.rows[].bundle")
        bundle_id = ensure_str(bundle.get("bundle_id"), "scorecard.rows[].bundle.bundle_id")
        rows_by_bundle[bundle_id] = row
        if bundle_id not in bundles_by_id:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scorecard.bundle_ref.unknown",
                    message=f"scorecard row cites unknown bundle {bundle_id}",
                    remediation="Scorecard rows must cite checked launch bundle ids.",
                    ref=bundle_id,
                )
            )
            continue

        status = ensure_str(row.get("compatibility_status_class"), f"{bundle_id}.scorecard.status")
        if status not in vocabulary:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scorecard.status.not_in_vocabulary",
                    message=f"{bundle_id} scorecard status is not declared in the packet vocabulary",
                    remediation="Add the status class to status_vocabulary or correct the row.",
                    ref=bundle_id,
                )
            )
        manifest_ref = ensure_str(bundle.get("manifest_ref"), f"{bundle_id}.scorecard.bundle.manifest_ref")
        validate_path_ref(repo_root, manifest_ref, "scorecard.bundle.manifest_ref", findings)

        expected = EXPECTED_BUNDLES.get(bundle_id)
        archetype_refs = {
            ensure_str(binding.get("archetype_row_ref"), f"{bundle_id}.scorecard.archetype_bindings[].archetype_row_ref")
            for binding in (ensure_dict(raw, f"{bundle_id}.scorecard.archetype_bindings[]") for raw in ensure_list(row.get("archetype_bindings"), f"{bundle_id}.scorecard.archetype_bindings"))
        }
        if expected and not any(expected["archetype_row_ref"] in ref for ref in archetype_refs):
            findings.append(
                Finding(
                    severity="error",
                    check_id="scorecard.archetype_binding.missing",
                    message=f"{bundle_id} scorecard is not tied to its expected archetype row",
                    remediation="Bind the scorecard row to the same archetype row as the launch bundle.",
                    ref=bundle_id,
                    details={"expected": expected["archetype_row_ref"]},
                )
            )

        for ref in ensure_list(row.get("evidence_refs"), f"{bundle_id}.scorecard.evidence_refs"):
            validate_path_ref(repo_root, ensure_str(ref, f"{bundle_id}.scorecard.evidence_refs[]"), "scorecard.evidence_refs", findings)

        review = ensure_dict(row.get("review_refs"), f"{bundle_id}.scorecard.review_refs")
        for key in ("install_preview_ref", "update_preview_ref", "remove_review_ref", "drift_packet_ref"):
            ensure_str(review.get(key), f"{bundle_id}.scorecard.review_refs.{key}")
        if ensure_str(review.get("rollback_checkpoint_policy"), f"{bundle_id}.scorecard.review_refs.rollback_checkpoint_policy") != "create_before_apply_update_remove_or_rebase":
            findings.append(
                Finding(
                    severity="error",
                    check_id="scorecard.rollback_checkpoint_policy",
                    message=f"{bundle_id} scorecard does not require rollback checkpoints for lifecycle review",
                    remediation="Use create_before_apply_update_remove_or_rebase.",
                    ref=bundle_id,
                )
            )

        templates = ensure_list(row.get("template_scaffold_refs"), f"{bundle_id}.scorecard.template_scaffold_refs")
        if not templates:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scorecard.template_refs.empty",
                    message=f"{bundle_id} scorecard omits template/scaffold refs",
                    remediation="Keep template/scaffold refs explicit and mirrorable.",
                    ref=bundle_id,
                )
            )
        for raw_template in templates:
            template = ensure_dict(raw_template, f"{bundle_id}.scorecard.template_scaffold_refs[]")
            validate_path_ref(
                repo_root,
                ensure_str(template.get("template_manifest_ref"), f"{bundle_id}.scorecard.template_manifest_ref"),
                "scorecard.template_manifest_ref",
                findings,
            )
            validate_path_ref(
                repo_root,
                ensure_str(template.get("generated_lineage_contract_ref"), f"{bundle_id}.scorecard.generated_lineage_contract_ref"),
                "scorecard.generated_lineage_contract_ref",
                findings,
            )
            if template.get("mirrorable") is not True or template.get("opaque_generation_behavior_allowed") is not False:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="scorecard.template_refs.not_mirrorable",
                        message=f"{bundle_id} scorecard has non-mirrorable or opaque template/scaffold behavior",
                        remediation="Set mirrorable true and opaque_generation_behavior_allowed false.",
                        ref=bundle_id,
                    )
                )

        support = ensure_dict(row.get("support_export"), f"{bundle_id}.scorecard.support_export")
        if support.get("raw_content_export_allowed") is not False:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scorecard.support_export.raw_content",
                    message=f"{bundle_id} scorecard would export raw content",
                    remediation="Keep scorecard support exports metadata-only.",
                    ref=bundle_id,
                )
            )
        if not ensure_list(support.get("export_packet_refs"), f"{bundle_id}.scorecard.support_export.export_packet_refs"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="scorecard.support_export.refs_empty",
                    message=f"{bundle_id} scorecard lacks support export refs",
                    remediation="Add support export refs that can reconstruct the row.",
                    ref=bundle_id,
                )
            )

        projection = ensure_dict(row.get("surface_projection"), f"{bundle_id}.scorecard.surface_projection")
        consumers = set(ensure_list(projection.get("surface_consumers"), f"{bundle_id}.scorecard.surface_projection.surface_consumers"))
        missing_consumers = {"start_center_bundle_detail", "migration_handoff", "docs_help_badge", "support_export"} - consumers
        if missing_consumers:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scorecard.surface_consumers.missing",
                    message=f"{bundle_id} scorecard is not projected to all required consumer surfaces",
                    remediation="Project scorecards to Start Center, migration handoff, docs/help, and support export.",
                    ref=bundle_id,
                    details={"missing": sorted(missing_consumers)},
                )
            )

    missing_rows = set(EXPECTED_BUNDLES) - set(rows_by_bundle)
    if missing_rows:
        findings.append(
            Finding(
                severity="error",
                check_id="scorecard.rows.missing_required",
                message="workflow bundle scorecard omits required launch bundles",
                remediation="Add one scorecard row for each checked launch bundle.",
                ref=DEFAULT_SCORECARD_REL,
                details={"missing": sorted(missing_rows)},
            )
        )
    return rows_by_bundle


def validate_drift_packet(
    packet: dict[str, Any],
    scorecard_rows_by_bundle: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> None:
    if ensure_str(packet.get("record_kind"), "drift_packet.record_kind") != "workflow_bundle_drift_packet_record":
        findings.append(
            Finding(
                severity="error",
                check_id="drift_packet.record_kind",
                message="workflow bundle drift packet has the wrong record kind",
                remediation="Use record_kind = workflow_bundle_drift_packet_record.",
                ref=DEFAULT_DRIFT_PACKET_REL,
            )
        )
    if ensure_int(packet.get("bundle_drift_packet_schema_version"), "drift_packet.bundle_drift_packet_schema_version") != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="drift_packet.schema_version",
                message="workflow bundle drift packet schema version must be 1",
                remediation="Update the validator alongside any schema-version change.",
                ref=DEFAULT_DRIFT_PACKET_REL,
            )
        )

    states_by_bundle: dict[str, set[str]] = {}
    actions_by_bundle: dict[str, set[str]] = {}
    for raw_row in ensure_list(packet.get("drift_rows"), "drift_packet.drift_rows"):
        row = ensure_dict(raw_row, "drift_packet.drift_rows[]")
        bundle = ensure_dict(row.get("bundle"), "drift_packet.drift_rows[].bundle")
        bundle_id = ensure_str(bundle.get("bundle_id"), "drift_packet.drift_rows[].bundle.bundle_id")
        states_by_bundle.setdefault(bundle_id, set()).add(
            ensure_str(row.get("drift_state_class"), f"{bundle_id}.drift_state_class")
        )
        actions_by_bundle.setdefault(bundle_id, set()).update(
            ensure_list(row.get("visible_actions"), f"{bundle_id}.visible_actions")
        )
        if row.get("preserves_local_artifacts") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="drift_packet.preserves_local_artifacts",
                    message=f"{bundle_id} drift row does not preserve local artifacts",
                    remediation="Set preserves_local_artifacts true and route durable changes through review.",
                    ref=bundle_id,
                )
            )
        ensure_str(row.get("support_export_ref"), f"{bundle_id}.drift_row.support_export_ref")

    ts_bundle_id = "launch_bundle:typescript_web_app.seed"
    missing_states = REQUIRED_DRIFT_STATES - states_by_bundle.get(ts_bundle_id, set())
    if missing_states:
        findings.append(
            Finding(
                severity="error",
                check_id="drift_packet.states.missing",
                message="TypeScript launch-bundle drift packet omits required drift states",
                remediation="Include local override, missing artifact, and bundle version drift rows.",
                ref=ts_bundle_id,
                details={"missing": sorted(missing_states)},
            )
        )
    missing_actions = (REQUIRED_LIFECYCLE_ACTIONS - {"apply", "remove_bundle"}) - actions_by_bundle.get(ts_bundle_id, set())
    if missing_actions:
        findings.append(
            Finding(
                severity="error",
                check_id="drift_packet.actions.missing",
                message="TypeScript launch-bundle drift packet omits required drift actions",
                remediation="Expose compare, keep local, adopt bundle, and rebase to bundle.",
                ref=ts_bundle_id,
                details={"missing": sorted(missing_actions)},
            )
        )

    for raw_review in ensure_list(packet.get("remove_reviews"), "drift_packet.remove_reviews"):
        review = ensure_dict(raw_review, "drift_packet.remove_reviews[]")
        bundle = ensure_dict(review.get("bundle"), "drift_packet.remove_reviews[].bundle")
        bundle_id = ensure_str(bundle.get("bundle_id"), "drift_packet.remove_reviews[].bundle.bundle_id")
        if bundle_id not in scorecard_rows_by_bundle:
            findings.append(
                Finding(
                    severity="error",
                    check_id="drift_packet.remove_review.bundle_unknown",
                    message=f"remove review cites unknown bundle {bundle_id}",
                    remediation="Remove reviews must cite a scorecard bundle row.",
                    ref=bundle_id,
                )
            )
        if review.get("raw_user_content_exported") is not False:
            findings.append(
                Finding(
                    severity="error",
                    check_id="drift_packet.remove_review.raw_content",
                    message=f"{bundle_id} remove review would export raw user content",
                    remediation="Keep remove-review support packet metadata-only.",
                    ref=bundle_id,
                )
            )
        safe_classes = set(ensure_list(review.get("safe_to_remove_classes"), f"{bundle_id}.remove_review.safe_to_remove_classes"))
        missing_safe_classes = REQUIRED_REMOVE_SAFE_CLASSES - safe_classes
        if bundle_id == ts_bundle_id and missing_safe_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="drift_packet.remove_review.safe_classes",
                    message="TypeScript remove review omits required safe-to-remove classes",
                    remediation="Represent no-user-data, user-overlay-preserved, and co-resident-user-data classifications.",
                    ref=bundle_id,
                    details={"missing": sorted(missing_safe_classes)},
                )
            )
        ensure_str(review.get("rollback_checkpoint_linkage_ref"), f"{bundle_id}.remove_review.rollback_checkpoint_linkage_ref")
        ensure_list(review.get("retained_local_override_refs"), f"{bundle_id}.remove_review.retained_local_override_refs")

    support = ensure_dict(packet.get("support_export"), "drift_packet.support_export")
    if support.get("raw_content_export_allowed") is not False:
        findings.append(
            Finding(
                severity="error",
                check_id="drift_packet.support_export.raw_content",
                message="drift packet would export raw content",
                remediation="Keep drift packet support exports metadata-only.",
                ref=DEFAULT_DRIFT_PACKET_REL,
            )
        )


def validate_bundle(
    repo_root: Path,
    bundle: dict[str, Any],
    bundle_path: str,
    matrix_bindings: dict[str, str],
    certification_rows: dict[str, dict[str, Any]],
    fixture_rows: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> None:
    validate_header(bundle, bundle_path, findings)
    bundle_id = ensure_str(bundle.get("bundle_id"), f"{bundle_path}.bundle_id")
    expected = EXPECTED_BUNDLES.get(bundle_id)
    if expected is None:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.unexpected_bundle_id",
                message=f"unexpected bundle id: {bundle_id}",
                remediation="Use one of the required external-alpha launch bundle ids.",
                ref=bundle_path,
            )
        )
        return
    if expected["path"] != bundle_path:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.path_mismatch",
                message=f"{bundle_id} is stored at {bundle_path}, expected {expected['path']}",
                remediation="Move the manifest to the expected canonical path or update the validator deliberately.",
                ref=bundle_id,
            )
        )

    stack = ensure_dict(bundle.get("stack_identity"), f"{bundle_id}.stack_identity")
    wedge_ref = ensure_str(stack.get("wedge_ref"), f"{bundle_id}.stack_identity.wedge_ref")
    if wedge_ref != expected["wedge_ref"] or matrix_bindings.get(bundle_id) != wedge_ref:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.wedge_binding",
                message=f"{bundle_id} does not match the alpha wedge matrix binding",
                remediation="Use the launch_bundle_ref already declared by the alpha wedge matrix.",
                ref=bundle_id,
                details={"expected_wedge": expected["wedge_ref"], "matrix_wedge": matrix_bindings.get(bundle_id)},
            )
        )
    current_support = ensure_str(stack.get("current_support_class"), f"{bundle_id}.stack_identity.current_support_class")
    current_claim = ensure_str(stack.get("current_claim_state"), f"{bundle_id}.stack_identity.current_claim_state")
    if current_support not in ALLOWED_CURRENT_SUPPORT_CLASSES or current_claim != "alpha_limited":
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.claim_overreach",
                message=f"{bundle_id} current claim exceeds seeded alpha evidence",
                remediation="Keep current support experimental or alpha-limited until compatibility evidence promotes it.",
                ref=bundle_id,
            )
        )
    for key in ("persona_label", "stack_label", "channel", "compatible_aureline_range"):
        ensure_str(stack.get(key), f"{bundle_id}.stack_identity.{key}")

    source = ensure_dict(bundle.get("source"), f"{bundle_id}.source")
    for key in ("source_label", "signer", "signer_label", "signature_state"):
        ensure_str(source.get(key), f"{bundle_id}.source.{key}")

    mirror_availability = ensure_dict(bundle.get("mirror_availability"), f"{bundle_id}.mirror_availability")
    if mirror_availability.get("offline_review_uses_same_vocabulary") is not True:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.mirror_availability.vocabulary",
                message=f"{bundle_id} must preserve install-review vocabulary when offline",
                remediation="Set offline_review_uses_same_vocabulary to true.",
                ref=bundle_id,
            )
        )

    validate_component_sets(bundle, bundle_id, findings)
    validate_review_and_drift(bundle, bundle_id, findings)
    validate_consumer_projection(repo_root, bundle, bundle_id, findings)
    validate_badges(repo_root, bundle, bundle_id, findings)

    evidence = ensure_dict(bundle.get("evidence_binding"), f"{bundle_id}.evidence_binding")
    archetype_seed_row_ref = ensure_str(evidence.get("archetype_seed_row_ref"), f"{bundle_id}.evidence_binding.archetype_seed_row_ref")
    fixture_row_ref = ensure_str(evidence.get("benchmark_fixture_register_row_ref"), f"{bundle_id}.evidence_binding.benchmark_fixture_register_row_ref")
    if archetype_seed_row_ref != expected["archetype_seed_row_ref"] or archetype_seed_row_ref not in certification_rows:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.evidence_binding.archetype_seed_row",
                message=f"{bundle_id} is not linked to the expected archetype seed row",
                remediation="Use the certification seed row for this bundle.",
                ref=bundle_id,
                details={"expected": expected["archetype_seed_row_ref"]},
            )
        )
    if fixture_row_ref != expected["benchmark_fixture_register_row_ref"] or fixture_row_ref not in fixture_rows:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.evidence_binding.fixture_row",
                message=f"{bundle_id} is not linked to the expected benchmark fixture row",
                remediation="Use the benchmark fixture register row for this bundle.",
                ref=bundle_id,
                details={"expected": expected["benchmark_fixture_register_row_ref"]},
            )
        )
    certification_state = ensure_str(evidence.get("certification_state"), f"{bundle_id}.evidence_binding.certification_state")
    if certification_state not in ALLOWED_CERTIFICATION_STATES:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.evidence_binding.certification_overreach",
                message=f"{bundle_id} certification state is stronger than seeded evidence",
                remediation="Use seed_not_certified, retest_pending, or evidence_stale until certification results exist.",
                ref=bundle_id,
            )
        )
    for key in ("archetype_evidence_packet_ref", "fixture_packet_ref", "known_limits_ref"):
        validate_path_ref(repo_root, ensure_str(evidence.get(key), f"{bundle_id}.evidence_binding.{key}"), f"bundle.evidence_binding.{key}", findings)

    acceptance_states = {
        ensure_str(row.get("exercises_state"), f"{bundle_id}.acceptance_state_coverage[].exercises_state")
        for row in (ensure_dict(raw, f"{bundle_id}.acceptance_state_coverage[]") for raw in ensure_list(bundle.get("acceptance_state_coverage"), f"{bundle_id}.acceptance_state_coverage"))
    }
    required = {
        "bundle_has_extensions_settings_tasks_docs_and_rollback",
        "bundle_links_one_archetype_and_one_benchmark_fixture",
        "bundle_scope_does_not_exceed_current_evidence",
    }
    missing_acceptance = required - acceptance_states
    if missing_acceptance:
        findings.append(
            Finding(
                severity="error",
                check_id="bundle.acceptance_state_coverage.missing",
                message=f"{bundle_id} acceptance_state_coverage omits required cases",
                remediation="Add coverage rows for required acceptance states.",
                ref=bundle_id,
                details={"missing": sorted(missing_acceptance)},
            )
        )


def validate_certification_register(
    repo_root: Path,
    register: dict[str, Any],
    bundles_by_id: dict[str, dict[str, Any]],
    fixture_rows: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> dict[str, dict[str, Any]]:
    validate_header(register, "certification", findings)
    for ref in ensure_dict(register.get("source_contract_refs"), "certification.source_contract_refs").values():
        validate_path_ref(repo_root, ensure_str(ref, "certification.source_contract_refs[]"), "certification.source_contract_refs", findings)

    rows = collect_certification_rows(register)
    for expected in EXPECTED_BUNDLES.values():
        row_ref = expected["archetype_seed_row_ref"]
        if row_ref not in rows:
            findings.append(
                Finding(
                    severity="error",
                    check_id="certification.rows.missing_required",
                    message=f"missing archetype seed row: {row_ref}",
                    remediation="Add one archetype seed row for each launch bundle.",
                    ref=row_ref,
                )
            )

    for row_id, row in rows.items():
        bundle_ref = ensure_str(row.get("bundle_ref"), f"{row_id}.bundle_ref")
        if bundle_ref not in bundles_by_id:
            findings.append(
                Finding(
                    severity="error",
                    check_id="certification.bundle_ref.unknown",
                    message=f"{row_id} cites unknown bundle ref {bundle_ref}",
                    remediation="Use one of the checked launch bundle ids.",
                    ref=row_id,
                )
            )
        validate_path_ref(repo_root, ensure_str(row.get("bundle_manifest_ref"), f"{row_id}.bundle_manifest_ref"), "certification.bundle_manifest_ref", findings)
        fixture_ref = ensure_str(row.get("benchmark_fixture_register_row_ref"), f"{row_id}.benchmark_fixture_register_row_ref")
        if fixture_ref not in fixture_rows:
            findings.append(
                Finding(
                    severity="error",
                    check_id="certification.fixture_ref.unknown",
                    message=f"{row_id} cites unknown benchmark fixture register row",
                    remediation="Use a register_row_id from the benchmark fixture register.",
                    ref=row_id,
                )
            )
        current_support = ensure_str(row.get("current_support_class"), f"{row_id}.current_support_class")
        certification_state = ensure_str(row.get("certification_state"), f"{row_id}.certification_state")
        if current_support not in ALLOWED_CURRENT_SUPPORT_CLASSES or certification_state not in ALLOWED_CERTIFICATION_STATES:
            findings.append(
                Finding(
                    severity="error",
                    check_id="certification.claim_overreach",
                    message=f"{row_id} implies a stronger state than current evidence",
                    remediation="Keep current state seed-limited until certification evidence promotes it.",
                    ref=row_id,
                )
            )
        validate_path_ref(repo_root, ensure_str(row.get("evidence_packet_ref"), f"{row_id}.evidence_packet_ref"), "certification.evidence_packet_ref", findings)
        for evidence_ref in ensure_list(row.get("evidence_refs"), f"{row_id}.evidence_refs"):
            validate_path_ref(repo_root, ensure_str(evidence_ref, f"{row_id}.evidence_refs[]"), "certification.evidence_refs", findings)

        badges = ensure_dict(row.get("badge_projection"), f"{row_id}.badge_projection")
        for key in ("launch_bundle_badge_opens", "archetype_badge_opens", "compatibility_badge_opens"):
            validate_path_ref(repo_root, ensure_str(badges.get(key), f"{row_id}.badge_projection.{key}"), f"certification.badge_projection.{key}", findings)
        mirror = ensure_dict(row.get("mirror_review"), f"{row_id}.mirror_review")
        mirror_sources = set(ensure_list(mirror.get("source_vocabulary"), f"{row_id}.mirror_review.source_vocabulary"))
        if REQUIRED_MIRROR_SOURCES - mirror_sources or mirror.get("review_vocabulary_matches_bundle_manifest") is not True:
            findings.append(
                Finding(
                    severity="error",
                    check_id="certification.mirror_review.vocabulary",
                    message=f"{row_id} mirror/offline vocabulary does not match bundle install review",
                    remediation="Use public_registry, approved_mirror, offline_bundle and set review_vocabulary_matches_bundle_manifest to true.",
                    ref=row_id,
                )
            )

    acceptance_states = {
        ensure_str(row.get("exercises_state"), "certification.acceptance_state_coverage[].exercises_state")
        for row in (ensure_dict(raw, "certification.acceptance_state_coverage[]") for raw in ensure_list(register.get("acceptance_state_coverage"), "certification.acceptance_state_coverage"))
    }
    required = {
        "archetype_rows_link_bundle_fixture_and_packet",
        "certification_state_is_seed_not_certified",
        "badges_open_underlying_packet",
    }
    missing_acceptance = required - acceptance_states
    if missing_acceptance:
        findings.append(
            Finding(
                severity="error",
                check_id="certification.acceptance_state_coverage.missing",
                message="certification seed register omits required acceptance coverage",
                remediation="Add acceptance coverage rows for binding, non-overclaiming, and badge evidence actions.",
                details={"missing": sorted(missing_acceptance)},
            )
        )
    return rows


def render_gallery_rows(bundles: list[dict[str, Any]]) -> str:
    lines = [
        "External alpha bundle gallery",
        "bundle_id | persona_or_stack | signer/source | channel | compatible_range | archetype_state | mirror",
    ]
    for bundle in sorted(bundles, key=lambda item: ensure_str(item.get("bundle_id"), "bundle.bundle_id")):
        bundle_id = ensure_str(bundle.get("bundle_id"), "bundle.bundle_id")
        stack = ensure_dict(bundle.get("stack_identity"), f"{bundle_id}.stack_identity")
        source = ensure_dict(bundle.get("source"), f"{bundle_id}.source")
        mirror = ensure_dict(bundle.get("mirror_availability"), f"{bundle_id}.mirror_availability")
        evidence = ensure_dict(bundle.get("evidence_binding"), f"{bundle_id}.evidence_binding")
        label = ensure_str(stack.get("persona_label"), f"{bundle_id}.stack_identity.persona_label")
        stack_label = ensure_str(stack.get("stack_label"), f"{bundle_id}.stack_identity.stack_label")
        signer = ensure_str(source.get("signer_label"), f"{bundle_id}.source.signer_label")
        source_label = ensure_str(source.get("source_label"), f"{bundle_id}.source.source_label")
        channel = ensure_str(stack.get("channel"), f"{bundle_id}.stack_identity.channel")
        compatible = ensure_str(stack.get("compatible_aureline_range"), f"{bundle_id}.stack_identity.compatible_aureline_range")
        certification_state = ensure_str(evidence.get("certification_state"), f"{bundle_id}.evidence_binding.certification_state")
        mirror_label = "/".join(
            [
                ensure_str(mirror.get("online_source"), f"{bundle_id}.mirror_availability.online_source"),
                ensure_str(mirror.get("approved_mirror"), f"{bundle_id}.mirror_availability.approved_mirror"),
                ensure_str(mirror.get("offline_bundle"), f"{bundle_id}.mirror_availability.offline_bundle"),
            ]
        )
        lines.append(
            f"{bundle_id} | {label} - {stack_label} | {signer} ({source_label}) | "
            f"{channel} | {compatible} | {certification_state} | {mirror_label}"
        )
    return "\n".join(lines) + "\n"


def write_report(path: Path, checked_bundle_ids: list[str], findings: list[Finding]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "status": "pass" if not any(item.severity == "error" for item in findings) else "fail",
        "generated_at": dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "summary": {
            "errors": sum(1 for item in findings if item.severity == "error"),
            "warnings": sum(1 for item in findings if item.severity == "warning"),
            "checked_bundle_ids": sorted(checked_bundle_ids),
            "acceptance_states": sorted(REQUIRED_ACCEPTANCE_STATES),
        },
        "findings": [item.as_report() for item in findings],
    }
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    paths = [args.tsjs_bundle, args.python_bundle]
    bundles = [ensure_dict(render_yaml_as_json(repo_root / path), path) for path in paths]
    certification = ensure_dict(render_yaml_as_json(repo_root / args.certification), "certification")
    fixture_register = ensure_dict(render_yaml_as_json(repo_root / args.fixture_register), "fixture_register")
    matrix = ensure_dict(render_yaml_as_json(repo_root / args.matrix), "matrix")
    scorecard = ensure_dict(read_json(repo_root / args.scorecard), "scorecard")
    drift_packet = ensure_dict(read_json(repo_root / args.drift_packet), "drift_packet")

    findings: list[Finding] = []
    for path in [
        args.proof_packet,
        args.start_center,
        args.tsjs_bundle,
        args.python_bundle,
        args.certification,
        args.fixture_register,
        args.matrix,
        args.scorecard,
        args.drift_packet,
    ]:
        validate_path_ref(repo_root, path, "primary_refs", findings)

    matrix_bindings = collect_matrix_bundle_bindings(matrix)
    fixture_rows = collect_fixture_rows(fixture_register)
    bundles_by_id = {
        ensure_str(bundle.get("bundle_id"), "bundle.bundle_id"): bundle for bundle in bundles
    }

    missing_bundles = set(EXPECTED_BUNDLES) - set(bundles_by_id)
    if missing_bundles:
        findings.append(
            Finding(
                severity="error",
                check_id="bundles.missing_required",
                message="one or more required launch bundle manifests are missing",
                remediation="Add both TypeScript/JavaScript and Python external-alpha bundle manifests.",
                details={"missing": sorted(missing_bundles)},
            )
        )

    certification_rows = validate_certification_register(
        repo_root=repo_root,
        register=certification,
        bundles_by_id=bundles_by_id,
        fixture_rows=fixture_rows,
        findings=findings,
    )
    for bundle_path, bundle in zip(paths, bundles):
        validate_bundle(
            repo_root=repo_root,
            bundle=bundle,
            bundle_path=bundle_path,
            matrix_bindings=matrix_bindings,
            certification_rows=certification_rows,
            fixture_rows=fixture_rows,
            findings=findings,
        )

    scorecard_rows_by_bundle = validate_scorecard_packet(
        repo_root=repo_root,
        packet=scorecard,
        bundles_by_id=bundles_by_id,
        findings=findings,
    )
    validate_drift_packet(
        packet=drift_packet,
        scorecard_rows_by_bundle=scorecard_rows_by_bundle,
        findings=findings,
    )

    if args.report:
        write_report(repo_root / args.report, list(bundles_by_id), findings)

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(f"[alpha-launch-bundles] {status} ({len(errors)} errors, {len(warnings)} warnings)")
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[alpha-launch-bundles] {prefix} {finding.check_id}: {finding.message}{ref_suffix}")
        print(f"[alpha-launch-bundles]   remediation: {finding.remediation}")

    if args.render_gallery and not errors:
        print()
        print(render_gallery_rows(bundles), end="")

    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
