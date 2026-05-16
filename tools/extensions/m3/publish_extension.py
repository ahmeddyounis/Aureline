#!/usr/bin/env python3
"""Build and validate extension publication packets."""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import shutil
import sys
from pathlib import Path
from typing import Any


SCHEMA_VERSION = 1
PIPELINE_KIND = "extension_publication_pipeline_record"
SUPPORT_EXPORT_KIND = "extension_publication_support_export_record"
VALIDATOR_GENERATED_AT = "2026-05-16T18:00:00Z"


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise SystemExit(f"missing JSON file: {path}") from exc
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def write_json(path: Path, payload: Any) -> None:
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def repo_rel(path: Path, repo_root: Path) -> str:
    try:
        return path.resolve().relative_to(repo_root.resolve()).as_posix()
    except ValueError:
        return path.as_posix()


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def content_address_for(path: Path) -> dict[str, Any]:
    data = path.read_bytes()
    return {
        "digest_algorithm": "sha256",
        "digest_hex": sha256_bytes(data),
        "digest_size_bytes": len(data),
    }


def non_empty(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def as_list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def extension_slug(package_id: str) -> str:
    return package_id.rsplit(".", 1)[-1]


def extension_identity(manifest: dict[str, Any], override: str | None) -> str:
    if override:
        return override
    return f"{manifest['publisher_id']}/{extension_slug(manifest['package_id'])}"


def load_manifest_validator(repo_root: Path):
    validator_path = repo_root / "tools/extensions/m3/validator_cli/aureline_extension_validator.py"
    spec = importlib.util.spec_from_file_location("aureline_extension_validator", validator_path)
    if spec is None or spec.loader is None:
        raise SystemExit(f"cannot import manifest validator at {validator_path}")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def validate_manifest(repo_root: Path, manifest_path: Path, generated_at: str) -> dict[str, Any]:
    validator = load_manifest_validator(repo_root)
    return validator.validate_manifest_payload(
        load_json(manifest_path),
        subject_manifest_ref=repo_rel(manifest_path, repo_root),
        generated_at=generated_at,
    )


def build_input(args: argparse.Namespace, manifest: dict[str, Any], artifact_address: dict[str, Any]) -> dict[str, Any]:
    identity = extension_identity(manifest, args.extension_identity)
    version = manifest["version"]
    compat = manifest["compatibility"]
    sdk = manifest["sdk"]
    signer_ref = args.signer_ref or f"publisher_signer:{manifest['publisher_id']}:beta"
    signature_ref = args.signature_ref or f"signature:{identity}:{version}:publisher"
    provenance_ref = args.provenance_ref or f"provenance:{identity}:{version}:build"
    registry_manifest_ref = args.registry_manifest_ref or f"registry_manifest:{identity}:{version}:public"
    permission_manifest_ref = args.permission_manifest_ref or f"permission_manifest:{identity}:{version}"
    runtime_contract_ref = args.runtime_contract_ref or f"runtime_v1_beta:{identity}:{version}"
    compatibility_report_ref = (
        args.compatibility_report_ref or f"compatibility_report:extensions:{identity}:{version}"
    )
    bridge_matrix_ref = args.bridge_matrix_ref
    bridge_matrix_row_ref = args.bridge_matrix_row_ref
    previous_address = {
        "digest_algorithm": "sha256",
        "digest_hex": args.previous_digest_hex,
        "digest_size_bytes": args.previous_digest_size_bytes,
    }
    target_platforms = [str(p) for p in as_list(compat.get("platforms"))]
    host_refs = [f"host_contract_family:{manifest['host_contract_family']}"]
    world_refs = [str(ref) for ref in as_list(sdk.get("wit_world_refs"))]

    promotion_steps = [
        promotion_step(
            identity=identity,
            version=version,
            registry_manifest_ref=registry_manifest_ref,
            source="quarantine",
            target="approved",
            address=artifact_address,
            signature_ref=signature_ref,
            evidence_refs=[
                args.conformance_report_ref,
                args.security_review_ref,
            ],
            approver_refs=[
                args.ecosystem_approver_ref,
                args.security_approver_ref,
            ],
        )
    ]
    if args.target_channel == "production":
        promotion_steps.append(
            promotion_step(
                identity=identity,
                version=version,
                registry_manifest_ref=registry_manifest_ref,
                source="approved",
                target="production",
                address=artifact_address,
                signature_ref=signature_ref,
                evidence_refs=[
                    args.mirror_rehearsal_ref,
                    args.rollback_drill_ref,
                ],
                approver_refs=[
                    args.release_approver_ref,
                    args.support_approver_ref,
                ],
            )
        )

    return {
        "publication_id": args.publication_id or f"extension_publication:{identity}:{version}",
        "version_metadata": {
            "extension_identity": identity,
            "package_id": manifest["package_id"],
            "publisher_id": manifest["publisher_id"],
            "extension_version": version,
            "manifest_schema_version": manifest["manifest_version"],
            "sdk_line_id": sdk["line_id"],
            "sdk_line_semver": sdk["line_semver"],
            "aureline_version_min": compat["aureline_versions"]["min"],
            "aureline_version_max": compat["aureline_versions"]["max"],
            "support_class": compat["support_class"],
            "bridge_state": compat["bridge_state"],
        },
        "artifact_metadata": {
            "artifact_ref": args.artifact_ref or f"artifact:{identity}:{version}:package",
            "extension_identity": identity,
            "extension_version": version,
            "content_address": artifact_address,
            "registry_manifest_ref": registry_manifest_ref,
            "permission_manifest_ref": permission_manifest_ref,
            "runtime_contract_ref": runtime_contract_ref,
        },
        "signer_metadata": {
            "signer_ref": signer_ref,
            "signer_key_fingerprint": "sha256:" + sha256_bytes(signer_ref.encode("utf-8")),
            "signature_ref": signature_ref,
            "signature_class": args.signature_class,
            "transparency_log_ref": args.transparency_log_ref,
            "signed_content_address": artifact_address,
            "signed_at": args.generated_at,
        },
        "provenance_metadata": {
            "provenance_ref": provenance_ref,
            "provenance_class": args.provenance_class,
            "builder_id": args.builder_id,
            "source_manifest_ref": f"manifest:{manifest['package_id']}:{version}",
            "source_revision_ref": args.source_revision_ref,
            "build_run_ref": args.build_run_ref,
            "conformance_report_ref": args.conformance_report_ref,
            "sdk_release_bundle_ref": args.sdk_release_bundle_ref,
            "subject_content_address": artifact_address,
            "generated_at": args.generated_at,
        },
        "compatibility_metadata": {
            "compatibility_report_ref": compatibility_report_ref,
            "bridge_matrix_ref": bridge_matrix_ref,
            "bridge_matrix_row_ref": bridge_matrix_row_ref,
            "host_contract_family_refs": host_refs,
            "capability_world_refs": world_refs,
            "target_platforms": target_platforms,
            "support_class": compat["support_class"],
            "bridge_state": compat["bridge_state"],
        },
        "promotion_steps": promotion_steps,
        "rollback_plan": {
            "rollback_plan_id": args.rollback_plan_id or f"rollback_plan:{identity}:{version}-to-{args.previous_version}",
            "previous_extension_version": args.previous_version,
            "previous_registry_manifest_ref": args.previous_registry_manifest_ref,
            "previous_content_address": previous_address,
            "rollback_manifest_ref": args.rollback_manifest_ref or f"rollback_manifest:{identity}:{version}-to-{args.previous_version}",
            "preserves_prior_installable_artifact": True,
            "rollback_does_not_delete_prior_artifact": True,
        },
        "failure_atomicity_guard": {
            "staging_catalog_ref": args.staging_catalog_ref or f"catalog:extensions:staging:{identity}:{version}",
            "target_catalog_ref": args.target_catalog_ref,
            "transaction_write_class": args.transaction_write_class,
            "writes_catalog_after_artifacts_verified": True,
            "revocation_state_requires_catalog_commit": True,
            "orphaned_revocation_state_count": 0,
            "retry_idempotency_key": args.retry_idempotency_key or f"idempotency:{identity}:{version}:{args.generated_at}",
        },
        "generated_at": args.generated_at,
    }


def promotion_step(
    *,
    identity: str,
    version: str,
    registry_manifest_ref: str,
    source: str,
    target: str,
    address: dict[str, Any],
    signature_ref: str,
    evidence_refs: list[str],
    approver_refs: list[str],
) -> dict[str, Any]:
    return {
        "promotion_step_id": f"promotion:{identity}:{version}:{source}-to-{target}",
        "registry_manifest_ref": registry_manifest_ref,
        "source_channel_class": source,
        "target_channel_class": target,
        "subject_content_address": address,
        "subject_signature_ref": signature_ref,
        "preserves_artifact_identity": True,
        "required_evidence_refs": evidence_refs,
        "approver_refs": approver_refs,
    }


def evaluate_input(payload: dict[str, Any]) -> dict[str, Any]:
    promotion_steps = payload["promotion_steps"]
    rollback = payload["rollback_plan"]
    atomicity = payload["failure_atomicity_guard"]
    record = {
        "record_kind": PIPELINE_KIND,
        "extension_publication_schema_version": SCHEMA_VERSION,
        **payload,
        "promotion_step_count": len(promotion_steps),
        "required_evidence_ref_count": sum(len(as_list(s.get("required_evidence_refs"))) for s in promotion_steps),
        "approver_ref_count": sum(len(as_list(s.get("approver_refs"))) for s in promotion_steps),
        "preserves_prior_installable_artifact": bool(
            rollback.get("preserves_prior_installable_artifact")
            and rollback.get("rollback_does_not_delete_prior_artifact")
        ),
        "transactional_catalog_update": bool(
            atomicity.get("transaction_write_class") in {"atomic_catalog_swap", "dry_run_only"}
            and atomicity.get("writes_catalog_after_artifacts_verified") is True
            and atomicity.get("revocation_state_requires_catalog_commit") is True
            and atomicity.get("orphaned_revocation_state_count") == 0
        ),
        "redaction_class": "metadata_safe_default",
    }
    decision_class, reason_class, summary = decide(payload)
    record["decision_class"] = decision_class
    record["reason_class"] = reason_class
    record["decision_summary"] = summary
    return record


def decide(payload: dict[str, Any]) -> tuple[str, str, str]:
    version = payload["version_metadata"]
    artifact = payload["artifact_metadata"]
    signer = payload["signer_metadata"]
    provenance = payload["provenance_metadata"]
    compatibility = payload["compatibility_metadata"]
    promotion_steps = payload["promotion_steps"]
    rollback = payload["rollback_plan"]
    atomicity = payload["failure_atomicity_guard"]
    address = artifact.get("content_address")

    if not str(payload.get("publication_id", "")).startswith("extension_publication:"):
        return refused("refused_publication_id_unprefixed", "publication id is not in the extension publication namespace")
    version_fields = [
        "extension_identity",
        "package_id",
        "publisher_id",
        "extension_version",
        "sdk_line_id",
        "sdk_line_semver",
        "aureline_version_min",
        "aureline_version_max",
        "support_class",
        "bridge_state",
    ]
    if any(not non_empty(version.get(field)) for field in version_fields):
        return refused("refused_version_metadata_missing", "version metadata is incomplete")
    if (
        not non_empty(artifact.get("artifact_ref"))
        or artifact.get("extension_identity") != version.get("extension_identity")
        or artifact.get("extension_version") != version.get("extension_version")
        or content_address_missing(address)
    ):
        return refused("refused_artifact_metadata_missing", "artifact metadata is incomplete")
    if (
        not non_empty(signer.get("signer_ref"))
        or not non_empty(signer.get("signer_key_fingerprint"))
        or not non_empty(signer.get("signature_ref"))
        or content_address_missing(signer.get("signed_content_address"))
    ):
        return refused("refused_signer_missing", "signer metadata is incomplete")
    if signer.get("signature_class") == "unsigned_denied_on_policy":
        return refused("refused_unsigned_artifact", "unsigned artifacts cannot publish")
    if signer.get("signed_content_address") != address:
        return refused("refused_promotion_identity_mutation", "signer content address differs from artifact")
    if (
        not non_empty(provenance.get("provenance_ref"))
        or provenance.get("provenance_class") == "missing_provenance"
        or provenance.get("subject_content_address") != address
        or any(not non_empty(provenance.get(field)) for field in [
            "builder_id",
            "source_manifest_ref",
            "source_revision_ref",
            "build_run_ref",
            "conformance_report_ref",
            "sdk_release_bundle_ref",
        ])
    ):
        return refused("refused_provenance_missing", "provenance metadata is incomplete")
    if (
        not non_empty(compatibility.get("compatibility_report_ref"))
        or not non_empty(compatibility.get("bridge_matrix_ref"))
        or not non_empty(compatibility.get("bridge_matrix_row_ref"))
        or not as_list(compatibility.get("host_contract_family_refs"))
        or not as_list(compatibility.get("capability_world_refs"))
        or not as_list(compatibility.get("target_platforms"))
    ):
        return refused("refused_compatibility_missing", "compatibility metadata is incomplete")
    if not promotion_steps or any(not promotion_step_safe(step, address, signer["signature_ref"]) for step in promotion_steps):
        return refused("refused_promotion_identity_mutation", "promotion steps do not preserve artifact identity")
    if any(not step.get("required_evidence_refs") or not step.get("approver_refs") for step in promotion_steps):
        return refused("refused_promotion_evidence_missing", "promotion steps require evidence and approver refs")
    if (
        not non_empty(rollback.get("rollback_plan_id"))
        or not non_empty(rollback.get("previous_extension_version"))
        or not non_empty(rollback.get("previous_registry_manifest_ref"))
        or not non_empty(rollback.get("rollback_manifest_ref"))
        or content_address_missing(rollback.get("previous_content_address"))
        or rollback.get("preserves_prior_installable_artifact") is not True
        or rollback.get("rollback_does_not_delete_prior_artifact") is not True
    ):
        return refused("refused_rollback_target_missing", "rollback plan must preserve a prior installable artifact")
    if (
        not non_empty(atomicity.get("staging_catalog_ref"))
        or not non_empty(atomicity.get("target_catalog_ref"))
        or not non_empty(atomicity.get("retry_idempotency_key"))
        or atomicity.get("transaction_write_class") == "unsafe_partial_writes"
        or atomicity.get("writes_catalog_after_artifacts_verified") is not True
        or atomicity.get("revocation_state_requires_catalog_commit") is not True
    ):
        return refused("refused_transactional_write_guard_missing", "publication must use a guarded catalog transaction")
    if atomicity.get("orphaned_revocation_state_count") != 0:
        return refused("refused_orphaned_revocation_state", "publication leaves orphaned revocation state")
    if any(step.get("target_channel_class") == "production" for step in promotion_steps):
        return (
            "ready_for_promotion",
            "ready_signed_provenance_rollback_safe",
            "artifact is signed, provenance-bound, compatible, transaction-safe, and rollback-ready",
        )
    return (
        "held_for_review",
        "held_no_production_promotion_requested",
        "artifact is valid but no production promotion was requested",
    )


def refused(reason: str, summary: str) -> tuple[str, str, str]:
    return "refused", reason, summary


def content_address_missing(address: Any) -> bool:
    return not isinstance(address, dict) or any(
        [
            not non_empty(address.get("digest_algorithm")),
            not non_empty(address.get("digest_hex")),
            not isinstance(address.get("digest_size_bytes"), int),
            address.get("digest_size_bytes", 0) <= 0,
        ]
    )


def promotion_step_safe(step: dict[str, Any], address: dict[str, Any], signature_ref: str) -> bool:
    return (
        step.get("subject_content_address") == address
        and step.get("subject_signature_ref") == signature_ref
        and step.get("preserves_artifact_identity") is True
        and (step.get("source_channel_class"), step.get("target_channel_class"))
        in {
            ("quarantine", "approved"),
            ("approved", "production"),
            ("production", "production"),
        }
    )


def support_export(record: dict[str, Any]) -> dict[str, Any]:
    rollback_available = bool(
        record.get("preserves_prior_installable_artifact")
        and non_empty(record["rollback_plan"].get("previous_registry_manifest_ref"))
    )
    return {
        "record_kind": SUPPORT_EXPORT_KIND,
        "extension_publication_schema_version": SCHEMA_VERSION,
        "export_id": f"extension_publication_support_export:{record['publication_id']}",
        "publication_ref": record["publication_id"],
        "extension_identity": record["version_metadata"]["extension_identity"],
        "extension_version": record["version_metadata"]["extension_version"],
        "artifact_ref": record["artifact_metadata"]["artifact_ref"],
        "registry_manifest_ref": record["artifact_metadata"]["registry_manifest_ref"],
        "signature_ref": record["signer_metadata"]["signature_ref"],
        "provenance_ref": record["provenance_metadata"]["provenance_ref"],
        "compatibility_report_ref": record["compatibility_metadata"]["compatibility_report_ref"],
        "bridge_matrix_ref": record["compatibility_metadata"]["bridge_matrix_ref"],
        "bridge_matrix_row_ref": record["compatibility_metadata"]["bridge_matrix_row_ref"],
        "rollback_manifest_ref": record["rollback_plan"]["rollback_manifest_ref"],
        "decision_class": record["decision_class"],
        "reason_class": record["reason_class"],
        "blocks_catalog_mutation": record["decision_class"] == "refused",
        "rollback_available": rollback_available,
        "transactional_catalog_update": record["transactional_catalog_update"],
        "export_safe_summary": (
            f"{record['version_metadata']['extension_identity']} "
            f"{record['version_metadata']['extension_version']} decision={record['decision_class']}; "
            f"signer={record['signer_metadata']['signer_ref']}; "
            f"provenance={record['provenance_metadata']['provenance_ref']}; "
            f"bridge_row={record['compatibility_metadata']['bridge_matrix_row_ref']}; "
            f"promotion_steps={record['promotion_step_count']}; rollback={rollback_available}"
        ),
        "redaction_class": "metadata_safe_default",
    }


def registry_manifest_row(record: dict[str, Any], generated_at: str) -> dict[str, Any]:
    version = record["version_metadata"]
    artifact = record["artifact_metadata"]
    signer = record["signer_metadata"]
    compat = record["compatibility_metadata"]
    return {
        "record_kind": "registry_manifest_row",
        "registry_manifest_schema_version": 1,
        "manifest_row_id": artifact["registry_manifest_ref"],
        "extension_identity": version["extension_identity"],
        "extension_version": version["extension_version"],
        "extension_manifest_ref": f"extension_manifest:{version['package_id']}:{version['extension_version']}",
        "content_address": artifact["content_address"],
        "artifact_digest_ref": f"digest:{version['extension_identity']}:{version['extension_version']}",
        "signature_class": "publisher_signature",
        "signature_ref": signer["signature_ref"],
        "publisher_continuity_ref": f"publisher_continuity:{version['publisher_id']}",
        "registry_source_class": "public_registry",
        "source_endpoint_ref": "endpoint:public-extension-registry",
        "trust_claim_source": "origin_public_registry",
        "trust_badge_inheritance_rule": "inherits_origin_tier",
        "channel_class": "production" if record["decision_class"] == "ready_for_promotion" else "approved",
        "approval_state_class": "promoted_to_production" if record["decision_class"] == "ready_for_promotion" else "approved_on_quarantine",
        "compatibility_notes": [
            {
                "compatibility_claim_class": "compatible_on_all_declared_targets",
                "bridge_matrix_ref": compat["bridge_matrix_ref"],
                "bridge_matrix_row_ref": compat["bridge_matrix_row_ref"],
                "declared_host_contract_family_refs": compat["host_contract_family_refs"],
                "declared_capability_world_refs": compat["capability_world_refs"],
                "known_caveat_labels": [],
            }
        ],
        "mirror_provenance_ref": None,
        "offline_bundle_ref": None,
        "local_archive_ref": None,
        "revocation_snapshot_age_class": "fresh",
        "revocation_snapshot_ref": f"revocation_snapshot:{version['extension_identity']}:{version['extension_version']}",
        "anti_abuse_signals": [{"signal_class": "none"}],
        "policy_pack_applicable_refs": ["policy_pack:extension-publication-default"],
        "audit_event_refs": [
            f"audit:registry_manifest_indexed:{version['extension_identity']}:{version['extension_version']}",
            f"audit:registry_manifest_signature_reverified:{version['extension_identity']}:{version['extension_version']}",
        ],
        "indexed_at": generated_at,
        "redaction_class": "support_bundle",
    }


def promotion_rows(record: dict[str, Any], generated_at: str) -> list[dict[str, Any]]:
    rows = []
    for step in record["promotion_steps"]:
        rows.append(
            {
                "record_kind": "channel_promotion_row",
                "registry_manifest_schema_version": 1,
                "channel_promotion_row_id": step["promotion_step_id"],
                "subject_manifest_row_ref": step["registry_manifest_ref"],
                "subject_content_address": step["subject_content_address"],
                "subject_signature_ref": step["subject_signature_ref"],
                "source_channel_class": step["source_channel_class"],
                "target_channel_class": step["target_channel_class"],
                "preserves_artifact_identity": True,
                "promotion_outcome": "promoted",
                "required_evidence_refs": step["required_evidence_refs"],
                "approvers_ref": step["approver_refs"],
                "policy_pack_applicable_refs": ["policy_pack:extension-publication-default"],
                "anti_abuse_signals": [{"signal_class": "none"}],
                "audit_event_refs": [
                    f"audit:channel_promotion_submitted:{step['promotion_step_id']}",
                    f"audit:channel_promotion_approved:{step['promotion_step_id']}",
                ],
                "promoted_at": generated_at,
            }
        )
    return rows


def rollback_manifest(record: dict[str, Any]) -> dict[str, Any]:
    rollback = record["rollback_plan"]
    return {
        "record_kind": "extension_publication_rollback_manifest",
        "publication_ref": record["publication_id"],
        "rollback_manifest_ref": rollback["rollback_manifest_ref"],
        "current_extension_version": record["version_metadata"]["extension_version"],
        "previous_extension_version": rollback["previous_extension_version"],
        "previous_registry_manifest_ref": rollback["previous_registry_manifest_ref"],
        "previous_content_address": rollback["previous_content_address"],
        "preserves_prior_installable_artifact": rollback["preserves_prior_installable_artifact"],
        "rollback_does_not_delete_prior_artifact": rollback["rollback_does_not_delete_prior_artifact"],
    }


def catalog_snapshot(record: dict[str, Any]) -> dict[str, Any]:
    return {
        "record_kind": "extension_publication_catalog_snapshot",
        "catalog_ref": record["failure_atomicity_guard"]["target_catalog_ref"],
        "active_registry_manifest_ref": record["artifact_metadata"]["registry_manifest_ref"],
        "active_content_address": record["artifact_metadata"]["content_address"],
        "rollback_manifest_ref": record["rollback_plan"]["rollback_manifest_ref"],
        "previous_registry_manifest_ref": record["rollback_plan"]["previous_registry_manifest_ref"],
        "publication_ref": record["publication_id"],
        "transactional_catalog_update": record["transactional_catalog_update"],
    }


def validate_packet_payload(payload: dict[str, Any]) -> dict[str, Any]:
    source = payload.get("input", payload)
    record = evaluate_input(source) if "record_kind" not in source else source
    regenerated = evaluate_input(record) if record.get("record_kind") == PIPELINE_KIND else record
    findings = []
    for key in [
        "promotion_step_count",
        "required_evidence_ref_count",
        "approver_ref_count",
        "preserves_prior_installable_artifact",
        "transactional_catalog_update",
        "decision_class",
        "reason_class",
    ]:
        if record.get(key) != regenerated.get(key):
            findings.append(f"{key} drifted: expected {regenerated.get(key)!r}, observed {record.get(key)!r}")
    return {
        "record_kind": "extension_publication_packet_validation_report",
        "result_class": "pass" if not findings and record.get("decision_class") != "refused" else "fail",
        "decision_class": record.get("decision_class"),
        "reason_class": record.get("reason_class"),
        "findings": findings,
    }


def write_publication_outputs(out_dir: Path, record: dict[str, Any], support: dict[str, Any], generated_at: str, force: bool) -> None:
    files: dict[str, Any] = {
        "publication_pipeline_record.json": record,
        "publication_support_export.json": support,
        "registry_manifest_row.json": registry_manifest_row(record, generated_at),
        "promotion_rows.json": promotion_rows(record, generated_at),
        "rollback_manifest.json": rollback_manifest(record),
    }
    if record["decision_class"] != "refused":
        files["catalog_snapshot.json"] = catalog_snapshot(record)

    if out_dir.exists() and not force:
        existing = [name for name in files if (out_dir / name).exists()]
        if existing:
            raise SystemExit(
                f"refusing to overwrite existing publication outputs without --force: {', '.join(existing)}"
            )
    staging = out_dir.parent / f".{out_dir.name}.staging"
    if staging.exists():
        shutil.rmtree(staging)
    staging.mkdir(parents=True)
    try:
        sidecars = [name for name in files if name != "catalog_snapshot.json"]
        for name in sidecars:
            write_json(staging / name, files[name])
        if "catalog_snapshot.json" in files:
            write_json(staging / "catalog_snapshot.json", files["catalog_snapshot.json"])
        out_dir.mkdir(parents=True, exist_ok=True)
        for name in sidecars:
            (staging / name).replace(out_dir / name)
        if "catalog_snapshot.json" in files:
            (staging / "catalog_snapshot.json").replace(out_dir / "catalog_snapshot.json")
    finally:
        if staging.exists():
            shutil.rmtree(staging)


def command_build_packet(args: argparse.Namespace) -> int:
    repo_root = Path(args.repo_root).resolve()
    manifest_path = Path(args.manifest).resolve()
    artifact_path = Path(args.artifact).resolve()
    manifest_report = validate_manifest(repo_root, manifest_path, args.generated_at)
    if manifest_report["result_class"] == "fail":
        sys.stderr.write("[extension-publish] manifest conformance failed; no catalog outputs written\n")
        sys.stderr.write(json.dumps(manifest_report["summary"], sort_keys=True) + "\n")
        return 1
    manifest = load_json(manifest_path)
    packet_input = build_input(args, manifest, content_address_for(artifact_path))
    record = evaluate_input(packet_input)
    export = support_export(record)
    if args.dry_run:
        sys.stdout.write(json.dumps(record, indent=2, sort_keys=True) + "\n")
        return 0 if record["decision_class"] != "refused" else 1
    write_publication_outputs(Path(args.out_dir).resolve(), record, export, args.generated_at, args.force)
    return 0 if record["decision_class"] != "refused" else 1


def command_validate_packet(args: argparse.Namespace) -> int:
    report = validate_packet_payload(load_json(Path(args.packet)))
    sys.stdout.write(json.dumps(report, indent=2, sort_keys=True) + "\n")
    return 0 if report["result_class"] == "pass" else 1


def command_validate_fixtures(args: argparse.Namespace) -> int:
    fixtures_dir = Path(args.fixtures_dir)
    results = []
    unexpected = []
    for path in sorted(fixtures_dir.glob("*.json")):
        fixture = load_json(path)
        meta = fixture.get("__fixture__", {})
        record = evaluate_input(fixture["input"])
        matched = (
            record["decision_class"] == meta.get("expected_decision_class")
            and record["reason_class"] == meta.get("expected_reason_class")
            and record["promotion_step_count"] == meta.get("expected_promotion_step_count")
            and record["required_evidence_ref_count"] == meta.get("expected_required_evidence_ref_count")
            and record["approver_ref_count"] == meta.get("expected_approver_ref_count")
            and record["preserves_prior_installable_artifact"] == meta.get("expected_preserves_prior_installable_artifact")
            and record["transactional_catalog_update"] == meta.get("expected_transactional_catalog_update")
        )
        if not matched:
            unexpected.append(path.name)
        results.append(
            {
                "fixture": path.name,
                "decision_class": record["decision_class"],
                "reason_class": record["reason_class"],
                "matched_expectation": matched,
            }
        )
    report = {
        "record_kind": "extension_publication_fixture_report",
        "result_class": "fail" if unexpected else "pass",
        "case_count": len(results),
        "case_results": results,
        "unexpected_results": unexpected,
    }
    rendered = json.dumps(report, indent=2, sort_keys=True) + "\n"
    if args.report:
        report_path = Path(args.report)
        if args.check:
            if not report_path.exists():
                sys.stderr.write(f"[extension-publish] missing report: {report_path}\n")
                return 1
            if report_path.read_text(encoding="utf-8") != rendered:
                sys.stderr.write(f"[extension-publish] report drift detected: {report_path}\n")
                return 1
        else:
            report_path.parent.mkdir(parents=True, exist_ok=True)
            report_path.write_text(rendered, encoding="utf-8")
    else:
        sys.stdout.write(rendered)
    return 0 if report["result_class"] == "pass" else 1


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    sub = parser.add_subparsers(dest="command", required=True)

    build = sub.add_parser("build-packet", help="build publication packet outputs")
    build.add_argument("--manifest", required=True)
    build.add_argument("--artifact", required=True)
    build.add_argument("--out-dir", required=True)
    build.add_argument("--generated-at", default=VALIDATOR_GENERATED_AT)
    build.add_argument("--extension-identity")
    build.add_argument("--publication-id")
    build.add_argument("--artifact-ref")
    build.add_argument("--registry-manifest-ref")
    build.add_argument("--permission-manifest-ref")
    build.add_argument("--runtime-contract-ref")
    build.add_argument("--compatibility-report-ref")
    build.add_argument("--bridge-matrix-ref", default="artifacts/compat/m3/bridge_matrix.yaml")
    build.add_argument(
        "--bridge-matrix-row-ref",
        default="extension_bridge_row:wasm_component_native_beta",
    )
    build.add_argument("--signer-ref")
    build.add_argument("--signature-ref")
    build.add_argument("--signature-class", choices=[
        "publisher_signature",
        "attestation_bundle",
        "dual_signed_publisher_and_attestation",
        "unsigned_denied_on_policy",
    ], default="publisher_signature")
    build.add_argument("--transparency-log-ref")
    build.add_argument("--provenance-ref")
    build.add_argument("--provenance-class", choices=[
        "verified_build_provenance",
        "publisher_asserted_provenance",
        "missing_provenance",
    ], default="verified_build_provenance")
    build.add_argument("--builder-id", default="builder:extensions:local")
    build.add_argument("--source-revision-ref", default="git:unknown")
    build.add_argument("--build-run-ref", default="build_run:extensions:local")
    build.add_argument("--conformance-report-ref", default="conformance_report:extension:local:pass")
    build.add_argument("--sdk-release-bundle-ref", default="sdk_release_bundle:aureline.sdk.beta:1.0.0")
    build.add_argument("--target-channel", choices=["approved", "production"], default="production")
    build.add_argument("--security-review-ref", default="security_review:extension:local")
    build.add_argument("--mirror-rehearsal-ref", default="mirror_rehearsal:extension:local")
    build.add_argument("--rollback-drill-ref", default="rollback_drill:extension:local")
    build.add_argument("--ecosystem-approver-ref", default="approver:ecosystem:extension-review")
    build.add_argument("--security-approver-ref", default="approver:security:extension-trust")
    build.add_argument("--release-approver-ref", default="approver:release:registry-promotion")
    build.add_argument("--support-approver-ref", default="approver:support:rollback-readiness")
    build.add_argument("--previous-version", required=True)
    build.add_argument("--previous-registry-manifest-ref", required=True)
    build.add_argument("--previous-digest-hex", required=True)
    build.add_argument("--previous-digest-size-bytes", type=int, required=True)
    build.add_argument("--rollback-plan-id")
    build.add_argument("--rollback-manifest-ref")
    build.add_argument("--staging-catalog-ref")
    build.add_argument("--target-catalog-ref", default="catalog:extensions:production")
    build.add_argument("--transaction-write-class", choices=[
        "atomic_catalog_swap",
        "dry_run_only",
        "unsafe_partial_writes",
    ], default="atomic_catalog_swap")
    build.add_argument("--retry-idempotency-key")
    build.add_argument("--dry-run", action="store_true")
    build.add_argument("--force", action="store_true")
    build.set_defaults(func=command_build_packet)

    validate = sub.add_parser("validate-packet", help="validate one publication packet")
    validate.add_argument("--packet", required=True)
    validate.set_defaults(func=command_validate_packet)

    fixtures = sub.add_parser("validate-fixtures", help="validate publication fixtures")
    fixtures.add_argument("--fixtures-dir", default="fixtures/extensions/m3/publication_pipeline")
    fixtures.add_argument("--report")
    fixtures.add_argument("--check", action="store_true")
    fixtures.set_defaults(func=command_validate_fixtures)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
