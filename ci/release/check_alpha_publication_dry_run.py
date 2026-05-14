#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Validate the alpha publication dry-run manifest."""

from __future__ import annotations

import argparse
import dataclasses
import json
import sys
from pathlib import Path
from typing import Any

import yaml


DEFAULT_MANIFEST = "artifacts/release/alpha_publication_manifest.yaml"
DEFAULT_GRAPH = "artifacts/release/alpha_artifact_graph.yaml"
DEFAULT_REPORT = "target/release-evidence/alpha_publication_dry_run_validation.json"
REQUIRED_FAMILIES = {
    "binaries",
    "docs_help_packs",
    "policy_bundles",
    "symbols",
    "support_schemas",
    "notices",
    "sbom_provenance",
}
REQUIRED_POSTURES = {"mirror_only", "deny_all", "offline_verification"}
REQUIRED_FRESHNESS_LIMITS = {
    "docs_help_packs",
    "service_health_snapshot",
    "notice_pack",
    "advisory_snapshot",
    "revocation_snapshot",
}
REQUIRED_DEGRADATION_TRUTH = {"live_service_health", "advisory", "revocation"}
OPAQUE_PREFIXES = (
    "artifact_node:",
    "artifact_bundle:",
    "artifact_family:",
    "artifact_verification_row:",
    "blocker.",
    "build-id:",
    "channel.",
    "digest.",
    "import_instruction:",
    "mirror_integrity_packet:",
    "offline_verification_packet:",
    "policy_pack:",
    "publication_posture:",
    "receipt:",
    "release_candidate:",
)


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    ref: str | None = None
    remediation: str | None = None
    details: dict[str, Any] = dataclasses.field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        payload = dataclasses.asdict(self)
        if payload["ref"] is None:
            payload.pop("ref")
        if payload["remediation"] is None:
            payload.pop("remediation")
        if not payload["details"]:
            payload.pop("details")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--manifest", default=DEFAULT_MANIFEST)
    parser.add_argument("--graph", default=DEFAULT_GRAPH)
    parser.add_argument("--report", default=DEFAULT_REPORT)
    parser.add_argument(
        "--posture",
        choices=sorted(REQUIRED_POSTURES),
        help="Validate one publication posture in addition to the full manifest.",
    )
    parser.add_argument("--validate-only", action="store_true")
    return parser.parse_args()


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0]


def resolve(repo_root: Path, ref: str) -> Path:
    return repo_root / strip_fragment(ref)


def load_yaml(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        payload = yaml.safe_load(handle)
    if not isinstance(payload, dict):
        raise SystemExit(f"YAML file must contain a mapping: {path}")
    return payload


def list_of_dicts(value: Any) -> list[dict[str, Any]]:
    if not isinstance(value, list):
        return []
    return [item for item in value if isinstance(item, dict)]


def string_set(value: Any) -> set[str]:
    if not isinstance(value, list):
        return set()
    return {item for item in value if isinstance(item, str) and item}


def is_repo_ref(ref: Any) -> bool:
    return isinstance(ref, str) and ref and not ref.startswith(OPAQUE_PREFIXES)


def validate_ref(
    repo_root: Path,
    ref: Any,
    findings: list[Finding],
    check_id: str,
    owner_ref: str,
) -> None:
    if not isinstance(ref, str) or not ref.strip():
        findings.append(
            Finding(
                "error",
                check_id,
                "reference must be a non-empty string",
                ref=owner_ref,
                remediation="Set a repo-relative artifact reference.",
            )
        )
        return
    if is_repo_ref(ref) and not resolve(repo_root, ref).exists():
        findings.append(
            Finding(
                "error",
                check_id,
                f"referenced artifact does not exist: {ref}",
                ref=owner_ref,
                remediation="Correct the reference or add the missing artifact.",
            )
        )


def collect_graph_node_ids(graph: dict[str, Any]) -> set[str]:
    node_ids: set[str] = set()
    families = graph.get("artifact_families")
    if not isinstance(families, dict):
        return node_ids
    for rows in families.values():
        for row in list_of_dicts(rows):
            node_id = row.get("node_id")
            if isinstance(node_id, str):
                node_ids.add(node_id)
    return node_ids


def validate_manifest_shape(
    manifest: dict[str, Any],
    findings: list[Finding],
    manifest_ref: str,
) -> None:
    if manifest.get("schema_version") != 1:
        findings.append(Finding("error", "manifest.schema_version", "schema_version must be 1", ref=manifest_ref))
    if manifest.get("record_kind") != "alpha_publication_manifest":
        findings.append(
            Finding(
                "error",
                "manifest.record_kind",
                "record_kind must be alpha_publication_manifest",
                ref=manifest_ref,
            )
        )
    if manifest.get("broader_publication_allowed") is not False:
        findings.append(
            Finding(
                "error",
                "manifest.broader_publication_allowed",
                "dry-run manifest must not allow broader publication",
                ref=manifest_ref,
                remediation="Keep broader_publication_allowed false until blockers are closed.",
            )
        )
    if not str(manifest.get("exact_build_identity_ref", "")).startswith("build-id:aureline:"):
        findings.append(
            Finding(
                "error",
                "manifest.exact_build_identity_ref",
                "manifest must carry an Aureline exact-build identity ref",
                ref=manifest_ref,
            )
        )


def validate_source_refs(repo_root: Path, manifest: dict[str, Any], findings: list[Finding]) -> None:
    for label, ref in (manifest.get("source_contract_refs") or {}).items():
        validate_ref(
            repo_root,
            ref,
            findings,
            "manifest.source_contract_refs.missing",
            f"source_contract_refs.{label}",
        )

    for row in list_of_dicts(manifest.get("artifact_family_rows")):
        row_ref = f"{DEFAULT_MANIFEST}#{row.get('family_key', '<missing>')}"
        for field in ("source_refs", "digest_material_refs"):
            for ref in row.get(field, []) or []:
                validate_ref(repo_root, ref, findings, f"artifact_family_rows.{field}.missing", row_ref)

    for receipt in list_of_dicts(manifest.get("verification_receipts")):
        receipt_ref = f"{DEFAULT_MANIFEST}#{receipt.get('receipt_id', '<missing>')}"
        for ref in receipt.get("evidence_refs", []) or []:
            validate_ref(repo_root, ref, findings, "verification_receipts.evidence_refs.missing", receipt_ref)

    for blocker in list_of_dicts(manifest.get("blockers")):
        blocker_ref = f"{DEFAULT_MANIFEST}#{blocker.get('blocker_id', '<missing>')}"
        for ref in blocker.get("evidence_refs", []) or []:
            validate_ref(repo_root, ref, findings, "blockers.evidence_refs.missing", blocker_ref)


def validate_family_coverage(
    manifest: dict[str, Any],
    graph_node_ids: set[str],
    findings: list[Finding],
) -> dict[str, dict[str, Any]]:
    manifest_ref = DEFAULT_MANIFEST
    required = string_set(manifest.get("required_artifact_family_keys"))
    missing_required = REQUIRED_FAMILIES - required
    if missing_required:
        findings.append(
            Finding(
                "error",
                "manifest.required_artifact_family_keys",
                "manifest does not declare every required alpha publication family",
                ref=manifest_ref,
                remediation="Add every required family key to required_artifact_family_keys.",
                details={"missing": sorted(missing_required)},
            )
        )

    rows_by_family: dict[str, dict[str, Any]] = {}
    for row in list_of_dicts(manifest.get("artifact_family_rows")):
        family_key = row.get("family_key")
        row_ref = f"{manifest_ref}#{family_key or '<missing>'}"
        if not isinstance(family_key, str) or not family_key:
            findings.append(Finding("error", "artifact_family_rows.family_key", "family_key is required", ref=row_ref))
            continue
        if family_key in rows_by_family:
            findings.append(Finding("error", "artifact_family_rows.duplicate", "family row is duplicated", ref=row_ref))
        rows_by_family[family_key] = row

        for field in (
            "display_name",
            "live_truth_degradation",
        ):
            if not row.get(field):
                findings.append(Finding("error", f"artifact_family_rows.{field}", f"{field} is required", ref=row_ref))
        for field in (
            "subject_classes",
            "artifact_family_classes",
            "source_refs",
            "digest_material_refs",
            "verification_receipt_refs",
            "import_instruction_refs",
        ):
            if not row.get(field):
                findings.append(Finding("error", f"artifact_family_rows.{field}", f"{field} must be non-empty", ref=row_ref))
        if row.get("verifiable_without_vendor_reachability") is not True:
            findings.append(
                Finding(
                    "error",
                    "artifact_family_rows.verifiable_without_vendor_reachability",
                    "family must declare whether it remains verifiable without vendor reachability",
                    ref=row_ref,
                    remediation="Set verifiable_without_vendor_reachability true for the dry-run-covered families.",
                )
            )

        graph_refs = string_set(row.get("graph_node_refs"))
        unknown_graph_refs = sorted(graph_refs - graph_node_ids)
        if family_key != "policy_bundles" and unknown_graph_refs:
            findings.append(
                Finding(
                    "error",
                    "artifact_family_rows.graph_node_refs",
                    "family references graph nodes that are absent from the alpha artifact graph",
                    ref=row_ref,
                    details={"unknown": unknown_graph_refs},
                )
            )
        if family_key == "policy_bundles" and graph_refs:
            findings.append(
                Finding(
                    "error",
                    "artifact_family_rows.policy_graph_gap",
                    "policy bundle row must keep graph linkage gap explicit in this dry run",
                    ref=row_ref,
                    remediation="Leave graph_node_refs empty until policy bundle nodes land in the graph.",
                )
            )
        if not row.get("blocker_refs"):
            findings.append(
                Finding(
                    "error",
                    "artifact_family_rows.blocker_refs",
                    "family must name blockers or explicit review-required gaps",
                    ref=row_ref,
                )
            )

    missing_rows = REQUIRED_FAMILIES - set(rows_by_family)
    if missing_rows:
        findings.append(
            Finding(
                "error",
                "artifact_family_rows.missing",
                "manifest is missing required artifact family rows",
                ref=manifest_ref,
                details={"missing": sorted(missing_rows)},
            )
        )

    return rows_by_family


def validate_postures(
    manifest: dict[str, Any],
    rows_by_family: dict[str, dict[str, Any]],
    requested_posture: str | None,
    findings: list[Finding],
) -> dict[str, dict[str, Any]]:
    postures_by_class: dict[str, dict[str, Any]] = {}
    receipt_ids = {
        receipt["receipt_id"]
        for receipt in list_of_dicts(manifest.get("verification_receipts"))
        if isinstance(receipt.get("receipt_id"), str)
    }

    for posture in list_of_dicts(manifest.get("publication_postures")):
        posture_class = posture.get("posture_class")
        posture_ref = f"{DEFAULT_MANIFEST}#{posture.get('posture_id', '<missing>')}"
        if not isinstance(posture_class, str) or not posture_class:
            findings.append(Finding("error", "publication_postures.posture_class", "posture_class is required", ref=posture_ref))
            continue
        postures_by_class[posture_class] = posture
        if posture.get("publication_mutations_allowed") is not False:
            findings.append(
                Finding(
                    "error",
                    "publication_postures.publication_mutations_allowed",
                    "dry-run postures must not allow publication mutations",
                    ref=posture_ref,
                )
            )
        if posture.get("vendor_reachability_required") is not False:
            findings.append(
                Finding(
                    "error",
                    "publication_postures.vendor_reachability_required",
                    "mirror/offline dry-run postures must not require vendor reachability",
                    ref=posture_ref,
                )
            )
        for field in ("connectivity_class", "freshness_limit"):
            if not posture.get(field):
                findings.append(Finding("error", f"publication_postures.{field}", f"{field} is required", ref=posture_ref))
        if not list_of_dicts(posture.get("import_instructions")):
            findings.append(
                Finding(
                    "error",
                    "publication_postures.import_instructions",
                    "posture must carry import instructions",
                    ref=posture_ref,
                )
            )
        if not posture.get("degraded_truth"):
            findings.append(
                Finding(
                    "error",
                    "publication_postures.degraded_truth",
                    "posture must explain live-truth degradation",
                    ref=posture_ref,
                )
            )

        covered_by_instructions: set[str] = set()
        for instruction in list_of_dicts(posture.get("import_instructions")):
            instruction_ref = f"{posture_ref}#{instruction.get('instruction_id', '<missing>')}"
            instruction_families = string_set(instruction.get("artifact_family_keys"))
            covered_by_instructions |= instruction_families
            for receipt_ref in instruction.get("expected_receipt_refs", []) or []:
                if receipt_ref not in receipt_ids:
                    findings.append(
                        Finding(
                            "error",
                            "publication_postures.import_instructions.expected_receipt_refs",
                            "import instruction references a missing receipt",
                            ref=instruction_ref,
                            details={"receipt_ref": receipt_ref},
                        )
                    )
            if not instruction.get("command_ref"):
                findings.append(
                    Finding(
                        "error",
                        "publication_postures.import_instructions.command_ref",
                        "import instruction must include a validation command ref",
                        ref=instruction_ref,
                    )
                )
        missing_instruction_families = REQUIRED_FAMILIES - covered_by_instructions
        if missing_instruction_families:
            findings.append(
                Finding(
                    "error",
                    "publication_postures.import_instructions.coverage",
                    "posture import instructions do not cover every required family",
                    ref=posture_ref,
                    details={"missing": sorted(missing_instruction_families)},
                )
            )

        posture_receipts = string_set(posture.get("verification_receipt_refs"))
        for family_key, row in rows_by_family.items():
            row_receipts = string_set(row.get("verification_receipt_refs"))
            if not any(receipt.startswith(f"receipt:alpha.publication.{posture_class}.{family_key}") for receipt in row_receipts):
                findings.append(
                    Finding(
                        "error",
                        "artifact_family_rows.verification_receipt_refs",
                        "family row is missing a receipt for a required posture",
                        ref=f"{DEFAULT_MANIFEST}#{family_key}",
                        details={"posture_class": posture_class},
                    )
                )
            if not row_receipts & posture_receipts:
                findings.append(
                    Finding(
                        "error",
                        "publication_postures.verification_receipt_refs",
                        "posture does not include the family row receipt",
                        ref=posture_ref,
                        details={"family_key": family_key},
                    )
                )

    missing_postures = REQUIRED_POSTURES - set(postures_by_class)
    if missing_postures:
        findings.append(
            Finding(
                "error",
                "publication_postures.missing",
                "manifest is missing required dry-run postures",
                ref=DEFAULT_MANIFEST,
                details={"missing": sorted(missing_postures)},
            )
        )
    if requested_posture and requested_posture not in postures_by_class:
        findings.append(
            Finding(
                "error",
                "publication_postures.requested_missing",
                "requested posture is not present in the manifest",
                ref=DEFAULT_MANIFEST,
                details={"posture": requested_posture},
            )
        )
    return postures_by_class


def validate_receipts(manifest: dict[str, Any], findings: list[Finding]) -> None:
    postures = {
        posture.get("posture_id")
        for posture in list_of_dicts(manifest.get("publication_postures"))
        if isinstance(posture.get("posture_id"), str)
    }
    receipt_ids: set[str] = set()
    coverage: dict[tuple[str, str], set[str]] = {}
    for receipt in list_of_dicts(manifest.get("verification_receipts")):
        receipt_id = receipt.get("receipt_id")
        receipt_ref = f"{DEFAULT_MANIFEST}#{receipt_id or '<missing>'}"
        if not isinstance(receipt_id, str) or not receipt_id:
            findings.append(Finding("error", "verification_receipts.receipt_id", "receipt_id is required", ref=receipt_ref))
            continue
        if receipt_id in receipt_ids:
            findings.append(Finding("error", "verification_receipts.duplicate", "receipt is duplicated", ref=receipt_ref))
        receipt_ids.add(receipt_id)

        posture_ref = receipt.get("posture_ref")
        if posture_ref not in postures:
            findings.append(
                Finding(
                    "error",
                    "verification_receipts.posture_ref",
                    "receipt references a missing posture",
                    ref=receipt_ref,
                    details={"posture_ref": posture_ref},
                )
            )
        posture_class = str(posture_ref).replace("publication_posture:", "")
        for family_key in string_set(receipt.get("artifact_family_keys")):
            coverage.setdefault((posture_class, family_key), set()).add(receipt_id)

        for field in ("receipt_class", "result_class", "freshness_limit", "notes"):
            if not receipt.get(field):
                findings.append(Finding("error", f"verification_receipts.{field}", f"{field} is required", ref=receipt_ref))
        if receipt.get("vendor_reachability_required") is not False:
            findings.append(
                Finding(
                    "error",
                    "verification_receipts.vendor_reachability_required",
                    "receipt must stay verifiable without vendor reachability",
                    ref=receipt_ref,
                )
            )
        if receipt.get("publication_mutations_allowed") is not False:
            findings.append(
                Finding(
                    "error",
                    "verification_receipts.publication_mutations_allowed",
                    "receipt must not allow publication mutations",
                    ref=receipt_ref,
                )
            )
        if not receipt.get("evidence_refs"):
            findings.append(
                Finding(
                    "error",
                    "verification_receipts.evidence_refs",
                    "receipt must cite evidence refs",
                    ref=receipt_ref,
                )
            )

    for posture in REQUIRED_POSTURES:
        for family in REQUIRED_FAMILIES:
            if not coverage.get((posture, family)):
                findings.append(
                    Finding(
                        "error",
                        "verification_receipts.coverage",
                        "missing receipt for posture and artifact family",
                        ref=DEFAULT_MANIFEST,
                        details={"posture": posture, "family": family},
                    )
                )


def validate_freshness_and_degradation(manifest: dict[str, Any], findings: list[Finding]) -> None:
    freshness_limits = manifest.get("freshness_limits") or {}
    if not isinstance(freshness_limits, dict):
        findings.append(Finding("error", "freshness_limits", "freshness_limits must be a mapping", ref=DEFAULT_MANIFEST))
        return
    missing_limits = REQUIRED_FRESHNESS_LIMITS - set(freshness_limits)
    if missing_limits:
        findings.append(
            Finding(
                "error",
                "freshness_limits.missing",
                "manifest is missing required freshness limits",
                ref=DEFAULT_MANIFEST,
                details={"missing": sorted(missing_limits)},
            )
        )

    degradation_truth = {
        row.get("truth_class")
        for row in list_of_dicts(manifest.get("live_truth_degradation_rules"))
        if isinstance(row.get("truth_class"), str)
    }
    missing_truth = REQUIRED_DEGRADATION_TRUTH - degradation_truth
    if missing_truth:
        findings.append(
            Finding(
                "error",
                "live_truth_degradation_rules.missing",
                "manifest does not declare required live-truth degradation classes",
                ref=DEFAULT_MANIFEST,
                details={"missing": sorted(missing_truth)},
            )
        )
    for row in list_of_dicts(manifest.get("live_truth_degradation_rules")):
        row_ref = f"{DEFAULT_MANIFEST}#{row.get('rule_id', '<missing>')}"
        for field in ("offline_or_mirror_state", "freshness_limit_ref", "required_surface_copy"):
            if not row.get(field):
                findings.append(Finding("error", f"live_truth_degradation_rules.{field}", f"{field} is required", ref=row_ref))
        if row.get("freshness_limit_ref") not in freshness_limits:
            findings.append(
                Finding(
                    "error",
                    "live_truth_degradation_rules.freshness_limit_ref",
                    "degradation rule references an unknown freshness limit",
                    ref=row_ref,
                )
            )


def validate_blockers(manifest: dict[str, Any], findings: list[Finding]) -> None:
    blockers = list_of_dicts(manifest.get("blockers"))
    if not blockers:
        findings.append(Finding("error", "blockers.empty", "dry-run manifest must name blockers", ref=DEFAULT_MANIFEST))
        return
    if not any(blocker.get("blocks_broader_publication") is True for blocker in blockers):
        findings.append(
            Finding(
                "error",
                "blockers.blocks_broader_publication",
                "at least one blocker must block broader publication while dry-run gaps remain",
                ref=DEFAULT_MANIFEST,
            )
        )
    blocker_ids = {blocker.get("blocker_id") for blocker in blockers}
    for row in list_of_dicts(manifest.get("artifact_family_rows")):
        row_ref = f"{DEFAULT_MANIFEST}#{row.get('family_key', '<missing>')}"
        for blocker_ref in row.get("blocker_refs", []) or []:
            if blocker_ref not in blocker_ids:
                findings.append(
                    Finding(
                        "error",
                        "artifact_family_rows.blocker_refs.unknown",
                        "family row references an unknown blocker",
                        ref=row_ref,
                        details={"blocker_ref": blocker_ref},
                    )
                )
    for blocker in blockers:
        blocker_ref = f"{DEFAULT_MANIFEST}#{blocker.get('blocker_id', '<missing>')}"
        for field in ("blocker_id", "severity", "summary", "closure_condition"):
            if not blocker.get(field):
                findings.append(Finding("error", f"blockers.{field}", f"{field} is required", ref=blocker_ref))


def build_report(
    manifest: dict[str, Any],
    findings: list[Finding],
    requested_posture: str | None,
) -> dict[str, Any]:
    rows = list_of_dicts(manifest.get("artifact_family_rows"))
    postures = list_of_dicts(manifest.get("publication_postures"))
    receipts = list_of_dicts(manifest.get("verification_receipts"))
    blockers = list_of_dicts(manifest.get("blockers"))
    status = "passed" if not findings else "failed"
    return {
        "schema_version": 1,
        "record_kind": "alpha_publication_dry_run_validation_capture",
        "status": status,
        "manifest_id": manifest.get("manifest_id", ""),
        "exact_build_identity_ref": manifest.get("exact_build_identity_ref", ""),
        "requested_posture": requested_posture,
        "coverage": {
            "artifact_family_keys": sorted(
                row.get("family_key") for row in rows if isinstance(row.get("family_key"), str)
            ),
            "posture_classes": sorted(
                posture.get("posture_class")
                for posture in postures
                if isinstance(posture.get("posture_class"), str)
            ),
            "receipt_count": len(receipts),
            "blocking_blocker_count": sum(1 for blocker in blockers if blocker.get("blocks_broader_publication") is True),
            "vendor_unreachable_families": sorted(
                row.get("family_key")
                for row in rows
                if row.get("verifiable_without_vendor_reachability") is True
            ),
        },
        "findings": [finding.as_dict() for finding in findings],
    }


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    manifest_path = repo_root / args.manifest
    graph_path = repo_root / args.graph
    manifest = load_yaml(manifest_path)
    graph = load_yaml(graph_path)
    findings: list[Finding] = []

    validate_manifest_shape(manifest, findings, args.manifest)
    validate_source_refs(repo_root, manifest, findings)
    rows_by_family = validate_family_coverage(manifest, collect_graph_node_ids(graph), findings)
    validate_postures(manifest, rows_by_family, args.posture, findings)
    validate_receipts(manifest, findings)
    validate_freshness_and_degradation(manifest, findings)
    validate_blockers(manifest, findings)

    report = build_report(manifest, findings, args.posture)
    if args.validate_only:
        if findings:
            print(json.dumps(report, indent=2, sort_keys=True), file=sys.stderr)
            return 1
        posture_note = f" posture={args.posture}" if args.posture else ""
        print(
            "alpha publication dry-run OK: "
            f"{len(report['coverage']['artifact_family_keys'])} families, "
            f"{len(report['coverage']['posture_classes'])} postures, "
            f"{report['coverage']['receipt_count']} receipts"
            f"{posture_note}"
        )
        return 0

    out_path = Path(args.report)
    if not out_path.is_absolute():
        out_path = repo_root / out_path
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    if findings:
        print(json.dumps(report, indent=2, sort_keys=True), file=sys.stderr)
        return 1
    print(f"wrote {out_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
