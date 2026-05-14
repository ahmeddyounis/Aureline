#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
"""Validate alpha repository-compliance import and notice evidence."""

from __future__ import annotations

import argparse
import dataclasses
import json
import re
import sys
from pathlib import Path
from typing import Any

import yaml


DEFAULT_GRAPH = "artifacts/release/alpha_artifact_graph.yaml"
DEFAULT_MANIFEST = "artifacts/governance/third_party_import_manifest.yaml"
DEFAULT_HEALTH = "artifacts/governance/critical_upstream_health_register.yaml"
DEFAULT_NOTICE_DELTA = "artifacts/release/reuse_spdx_notice_delta_alpha.md"
DEFAULT_DCO_AUDIT = "artifacts/governance/dco_merge_audit_alpha.md"
DEFAULT_REPORT = "target/release-evidence/repository_compliance_alpha_validation.json"

OWNER_RE = re.compile(r"^@[A-Za-z0-9_-]+$")


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
    parser.add_argument("--graph", default=DEFAULT_GRAPH)
    parser.add_argument("--manifest", default=DEFAULT_MANIFEST)
    parser.add_argument("--health-register", default=DEFAULT_HEALTH)
    parser.add_argument("--notice-delta", default=DEFAULT_NOTICE_DELTA)
    parser.add_argument("--dco-audit", default=DEFAULT_DCO_AUDIT)
    parser.add_argument("--report", default=DEFAULT_REPORT)
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
    if not resolve(repo_root, ref).exists():
        findings.append(
            Finding(
                "error",
                check_id,
                f"referenced artifact does not exist: {ref}",
                ref=owner_ref,
                remediation="Correct the reference or add the missing artifact.",
            )
        )


def collect_graph_nodes(graph: dict[str, Any]) -> dict[str, dict[str, Any]]:
    nodes: dict[str, dict[str, Any]] = {}
    families = graph.get("artifact_families", {})
    if not isinstance(families, dict):
        return nodes
    for family_key, raw_rows in families.items():
        for row in list_of_dicts(raw_rows):
            node_id = row.get("node_id")
            if isinstance(node_id, str) and node_id:
                node = dict(row)
                node["_family_key"] = family_key
                nodes[node_id] = node
    return nodes


def collect_graph_family_keys(graph: dict[str, Any]) -> set[str]:
    families = graph.get("artifact_families", {})
    if not isinstance(families, dict):
        return set()
    keys: set[str] = set()
    for family_key, raw_rows in families.items():
        if any(row.get("required_for_candidate") for row in list_of_dicts(raw_rows)):
            keys.add(str(family_key))
    return keys


def collect_register_ids(register: dict[str, Any]) -> set[str]:
    return {
        str(row["id"])
        for row in list_of_dicts(register.get("rows"))
        if isinstance(row.get("id"), str)
    }


def collect_notice_source_ids(notice_seed: dict[str, Any]) -> set[str]:
    return {
        str(row["source_id"])
        for row in list_of_dicts(notice_seed.get("rows"))
        if isinstance(row.get("source_id"), str)
    }


def collect_critical_dependency_source_ids(critical_register: dict[str, Any]) -> set[str]:
    ids: set[str] = set()
    for row in list_of_dicts(critical_register.get("entries")):
        if (
            row.get("source_register") == "dependency_register"
            and row.get("criticality_class") == "protected_path_release_critical"
            and isinstance(row.get("source_id"), str)
        ):
            ids.add(row["source_id"])
    return ids


def collect_high_risk_scorecard_ids(scorecard: dict[str, Any]) -> set[str]:
    ids: set[str] = set()
    for row in list_of_dicts(scorecard.get("rows")):
        if (
            row.get("assessment_state") == "provisional_risk_only"
            and row.get("provisional_risk_class") == "high"
            and isinstance(row.get("dependency_id"), str)
        ):
            ids.add(row["dependency_id"])
    return ids


def validate_manifest(
    repo_root: Path,
    manifest: dict[str, Any],
    graph: dict[str, Any],
    dependency_ids: set[str],
    import_ids: set[str],
    notice_source_ids: set[str],
    notice_text: str,
    dco_text: str,
    findings: list[Finding],
) -> dict[str, Any]:
    manifest_ref = DEFAULT_MANIFEST
    if manifest.get("schema_version") != 1:
        findings.append(Finding("error", "manifest.schema_version", "schema_version must be 1", ref=manifest_ref))
    if manifest.get("record_kind") != "alpha_repository_compliance_import_manifest":
        findings.append(
            Finding(
                "error",
                "manifest.record_kind",
                "record_kind must be alpha_repository_compliance_import_manifest",
                ref=manifest_ref,
            )
        )

    for label, ref in (manifest.get("source_contract_refs") or {}).items():
        validate_ref(repo_root, ref, findings, "manifest.source_contract_refs.missing", f"source_contract_refs.{label}")

    protected_keys = set(str(item) for item in manifest.get("protected_artifact_family_keys", []))
    graph_keys = collect_graph_family_keys(graph)
    if graph_keys - protected_keys:
        findings.append(
            Finding(
                "error",
                "manifest.protected_artifact_family_keys.missing",
                "manifest does not name every required alpha artifact family",
                ref=manifest_ref,
                remediation="Add every required artifact family key from the alpha artifact graph.",
                details={"missing": sorted(graph_keys - protected_keys)},
            )
        )

    nodes = collect_graph_nodes(graph)
    required_node_ids = {
        node_id for node_id, node in nodes.items() if node.get("required_for_candidate")
    }
    covered_family_keys: set[str] = set()
    covered_node_ids: set[str] = set()
    row_ids: set[str] = set()
    rows = list_of_dicts(manifest.get("rows"))

    required_fields = set(str(item) for item in manifest.get("required_row_fields", []))
    dco_states = set(str(item) for item in manifest.get("dco_signoff_state_vocabulary", []))
    reuse_states = set(str(item) for item in manifest.get("reuse_spdx_state_vocabulary", []))
    source_classes = set(str(item) for item in manifest.get("source_class_vocabulary", []))

    for row in rows:
        row_id = row.get("row_id")
        row_ref = f"{manifest_ref}#{row_id or '<missing>'}"
        if not isinstance(row_id, str) or not row_id:
            findings.append(Finding("error", "manifest.rows.row_id", "row_id is required", ref=row_ref))
            continue
        if row_id in row_ids:
            findings.append(Finding("error", "manifest.rows.duplicate", f"duplicate row_id: {row_id}", ref=row_ref))
        row_ids.add(row_id)

        missing_fields = sorted(field for field in required_fields if not row.get(field))
        if missing_fields:
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.required_fields",
                    "manifest row is missing required fields",
                    ref=row_ref,
                    remediation="Populate every required row field.",
                    details={"missing": missing_fields},
                )
            )

        if row.get("source_class") not in source_classes:
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.source_class",
                    f"unknown source_class: {row.get('source_class')}",
                    ref=row_ref,
                )
            )

        update_owner = row.get("update_owner")
        if not isinstance(update_owner, str) or not OWNER_RE.fullmatch(update_owner):
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.update_owner",
                    "update_owner must be an @handle",
                    ref=row_ref,
                )
            )

        dco_state = row.get("dco_signoff_state")
        if dco_state not in dco_states:
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.dco_signoff_state",
                    f"unknown dco_signoff_state: {dco_state}",
                    ref=row_ref,
                )
            )
        if dco_state != "signed_or_required_for_new_commits" and "dco_audit_ref" not in row:
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.dco_exception_missing",
                    "non-standard DCO state must cite dco_audit_ref",
                    ref=row_ref,
                )
            )

        reuse_state = row.get("reuse_spdx_state")
        if reuse_state not in reuse_states:
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.reuse_spdx_state",
                    f"unknown reuse_spdx_state: {reuse_state}",
                    ref=row_ref,
                )
            )

        for origin_ref in row.get("origin_refs", []) or []:
            validate_ref(repo_root, origin_ref, findings, "manifest.rows.origin_refs.missing", row_ref)

        source_register = row.get("source_register")
        source_id = row.get("source_id")
        if source_register == "dependency_register" and source_id not in dependency_ids:
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.source_id.unresolved_dependency",
                    f"dependency source_id does not resolve: {source_id}",
                    ref=row_ref,
                )
            )
        if source_register == "third_party_import_register" and source_id not in import_ids:
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.source_id.unresolved_import",
                    f"import source_id does not resolve: {source_id}",
                    ref=row_ref,
                )
            )

        if row.get("notice_delta_required") and row_id not in notice_text:
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.notice_delta_missing_row",
                    "notice delta packet does not mention a notice-bearing manifest row",
                    ref=row_ref,
                    remediation="Add the row id to the notice delta packet.",
                )
            )
        for notice_ref in row.get("notice_source_refs", []) or []:
            validate_ref(repo_root, notice_ref, findings, "manifest.rows.notice_source_refs.missing", row_ref)
        if source_id and row.get("notice_source_refs") and source_id not in notice_source_ids:
            findings.append(
                Finding(
                    "error",
                    "manifest.rows.notice_source_id_missing_seed",
                    f"release notice seed has no row for source_id {source_id}",
                    ref=row_ref,
                )
            )

        for linkage in list_of_dicts(row.get("artifact_family_linkage")):
            family_key = linkage.get("family_key")
            if family_key not in graph_keys:
                findings.append(
                    Finding(
                        "error",
                        "manifest.rows.artifact_family_linkage.unknown_family",
                        f"unknown or non-required artifact family: {family_key}",
                        ref=row_ref,
                    )
                )
            else:
                covered_family_keys.add(str(family_key))
            for node_ref in linkage.get("artifact_node_refs", []) or []:
                if node_ref not in nodes:
                    findings.append(
                        Finding(
                            "error",
                            "manifest.rows.artifact_node_refs.unknown",
                            f"artifact node does not exist in graph: {node_ref}",
                            ref=row_ref,
                        )
                    )
                    continue
                covered_node_ids.add(node_ref)
                if nodes[node_ref].get("_family_key") != family_key:
                    findings.append(
                        Finding(
                            "error",
                            "manifest.rows.artifact_node_refs.family_mismatch",
                            "artifact node belongs to a different family key",
                            ref=row_ref,
                            details={
                                "node": node_ref,
                                "expected_family": family_key,
                                "actual_family": nodes[node_ref].get("_family_key"),
                            },
                        )
                    )

    missing_family_coverage = graph_keys - covered_family_keys
    if missing_family_coverage:
        findings.append(
            Finding(
                "error",
                "manifest.coverage.family_missing",
                "one or more required artifact families have no manifest row",
                ref=manifest_ref,
                remediation="Add a row with artifact_family_linkage for each missing family.",
                details={"missing": sorted(missing_family_coverage)},
            )
        )
    missing_node_coverage = required_node_ids - covered_node_ids
    if missing_node_coverage:
        findings.append(
            Finding(
                "error",
                "manifest.coverage.required_nodes_missing",
                "one or more candidate artifact nodes have no manifest linkage",
                ref=manifest_ref,
                remediation="Link every required candidate node from the alpha artifact graph.",
                details={"missing": sorted(missing_node_coverage)},
            )
        )

    for family_key in protected_keys:
        if family_key not in dco_text:
            findings.append(
                Finding(
                    "error",
                    "dco_audit.family_missing",
                    "DCO audit does not mention a protected family key",
                    ref=family_key,
                )
            )

    return {
        "manifest_rows": len(rows),
        "covered_family_keys": sorted(covered_family_keys),
        "covered_required_nodes": sorted(covered_node_ids & required_node_ids),
    }


def validate_health_register(
    repo_root: Path,
    health: dict[str, Any],
    dependency_ids: set[str],
    critical_dependency_source_ids: set[str],
    high_risk_scorecard_ids: set[str],
    protected_family_keys: set[str],
    findings: list[Finding],
) -> dict[str, Any]:
    health_ref = DEFAULT_HEALTH
    if health.get("schema_version") != 1:
        findings.append(Finding("error", "health.schema_version", "schema_version must be 1", ref=health_ref))
    if health.get("record_kind") != "alpha_critical_upstream_health_register":
        findings.append(
            Finding(
                "error",
                "health.record_kind",
                "record_kind must be alpha_critical_upstream_health_register",
                ref=health_ref,
            )
        )
    for label, ref in (health.get("source_contract_refs") or {}).items():
        validate_ref(repo_root, ref, findings, "health.source_contract_refs.missing", f"source_contract_refs.{label}")

    red_ids = set(str(item) for item in health.get("red_risk_dependency_ids", []))
    rows = list_of_dicts(health.get("rows"))
    rows_by_id = {
        str(row["dependency_id"]): row
        for row in rows
        if isinstance(row.get("dependency_id"), str)
    }

    for missing_id in sorted((critical_dependency_source_ids | high_risk_scorecard_ids) - red_ids):
        findings.append(
            Finding(
                "error",
                "health.red_risk_dependency_ids.missing",
                "protected or high-risk dependency is missing from red_risk_dependency_ids",
                ref=missing_id,
                remediation="Add the dependency to the health register or downgrade the source register with evidence.",
            )
        )
    for missing_id in sorted(red_ids - set(rows_by_id)):
        findings.append(
            Finding(
                "error",
                "health.rows.missing_red_row",
                "red-risk dependency has no health row",
                ref=missing_id,
            )
        )

    for dependency_id, row in rows_by_id.items():
        row_ref = f"{health_ref}#{dependency_id}"
        if dependency_id not in dependency_ids:
            findings.append(
                Finding(
                    "error",
                    "health.rows.dependency_id.unresolved",
                    f"dependency_id does not resolve: {dependency_id}",
                    ref=row_ref,
                )
            )
        if row.get("risk_state") == "red":
            owner = row.get("owner_dri")
            if not isinstance(owner, str) or not OWNER_RE.fullmatch(owner):
                findings.append(
                    Finding(
                        "error",
                        "health.rows.red.owner_missing",
                        "red-risk upstream must have an owner_dri @handle",
                        ref=row_ref,
                    )
                )
            if not row.get("health_status"):
                findings.append(
                    Finding(
                        "error",
                        "health.rows.red.health_status_missing",
                        "red-risk upstream must publish a health_status",
                        ref=row_ref,
                    )
                )
            if not row.get("fork_replace_escalate_trigger"):
                findings.append(
                    Finding(
                        "error",
                        "health.rows.red.trigger_missing",
                        "red-risk upstream must have a fork/replace/escalate trigger",
                        ref=row_ref,
                    )
                )
        for family_key in row.get("artifact_family_keys", []) or []:
            if family_key not in protected_family_keys:
                findings.append(
                    Finding(
                        "error",
                        "health.rows.artifact_family_keys.unknown",
                        f"health row cites an unknown protected artifact family: {family_key}",
                        ref=row_ref,
                    )
                )
        validate_ref(repo_root, row.get("source_dependency_ref"), findings, "health.rows.source_dependency_ref.missing", row_ref)
        for entry_ref in row.get("critical_dependency_entry_refs", []) or []:
            validate_ref(repo_root, entry_ref, findings, "health.rows.critical_dependency_entry_refs.missing", row_ref)
        for evidence_ref in row.get("evidence_refs", []) or []:
            validate_ref(repo_root, evidence_ref, findings, "health.rows.evidence_refs.missing", row_ref)

    return {
        "red_risk_dependency_ids": sorted(red_ids),
        "health_rows": len(rows_by_id),
    }


def validate_notice_and_dco_packets(
    notice_text: str,
    dco_text: str,
    findings: list[Finding],
) -> None:
    if "artifacts/governance/third_party_import_manifest.yaml" not in notice_text:
        findings.append(
            Finding(
                "error",
                "notice_delta.source_of_truth_missing",
                "notice delta packet does not name the import manifest as source of truth",
                ref=DEFAULT_NOTICE_DELTA,
            )
        )
    for marker in (
        "SPDX SBOM state",
        "Third-Party Notice Projection",
        "Explicit Exceptions",
    ):
        if marker not in notice_text:
            findings.append(
                Finding(
                    "error",
                    "notice_delta.marker_missing",
                    f"notice delta packet is missing marker: {marker}",
                    ref=DEFAULT_NOTICE_DELTA,
                )
            )
    for marker in (
        "Developer Certificate of Origin 1.1",
        "ci/release/check_dco_signoff.sh",
        "baseline.repository_history_before_dco_ci",
        "dco-baseline-cutoff",
    ):
        if marker not in dco_text:
            findings.append(
                Finding(
                    "error",
                    "dco_audit.marker_missing",
                    f"DCO audit packet is missing marker: {marker}",
                    ref=DEFAULT_DCO_AUDIT,
                )
            )


def write_report(path: Path, payload: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a git repository: {repo_root}")

    graph = load_yaml(repo_root / args.graph)
    manifest = load_yaml(repo_root / args.manifest)
    health = load_yaml(repo_root / args.health_register)
    dependency_register = load_yaml(repo_root / "artifacts/governance/dependency_register.yaml")
    import_register = load_yaml(repo_root / "artifacts/governance/third_party_import_register.yaml")
    critical_register = load_yaml(repo_root / "artifacts/governance/critical_dependency_register.yaml")
    notice_seed = load_yaml(repo_root / "artifacts/governance/release_notice_seed.yaml")
    scorecard = load_yaml(repo_root / "artifacts/governance/upstream_health_scorecard.yaml")
    notice_text = (repo_root / args.notice_delta).read_text(encoding="utf-8")
    dco_text = (repo_root / args.dco_audit).read_text(encoding="utf-8")

    findings: list[Finding] = []
    dependency_ids = collect_register_ids(dependency_register)
    import_ids = collect_register_ids(import_register)
    notice_source_ids = collect_notice_source_ids(notice_seed)
    critical_dependency_source_ids = collect_critical_dependency_source_ids(critical_register)
    high_risk_scorecard_ids = collect_high_risk_scorecard_ids(scorecard)

    manifest_summary = validate_manifest(
        repo_root,
        manifest,
        graph,
        dependency_ids,
        import_ids,
        notice_source_ids,
        notice_text,
        dco_text,
        findings,
    )
    health_summary = validate_health_register(
        repo_root,
        health,
        dependency_ids,
        critical_dependency_source_ids,
        high_risk_scorecard_ids,
        set(manifest.get("protected_artifact_family_keys", [])),
        findings,
    )
    validate_notice_and_dco_packets(notice_text, dco_text, findings)

    status = "PASS" if not any(finding.severity == "error" for finding in findings) else "FAIL"
    report = {
        "schema_version": 1,
        "record_kind": "alpha_repository_compliance_validation_report",
        "status": status,
        "graph_ref": args.graph,
        "manifest_ref": args.manifest,
        "health_register_ref": args.health_register,
        "notice_delta_ref": args.notice_delta,
        "dco_audit_ref": args.dco_audit,
        "manifest_summary": manifest_summary,
        "health_summary": health_summary,
        "finding_count": len(findings),
        "findings": [finding.as_dict() for finding in findings],
    }

    if args.report and not args.validate_only:
        write_report(repo_root / args.report, report)

    if status != "PASS":
        for finding in findings:
            print(f"{finding.severity.upper()} {finding.check_id}: {finding.message}", file=sys.stderr)
            if finding.ref:
                print(f"  ref: {finding.ref}", file=sys.stderr)
            if finding.remediation:
                print(f"  remediation: {finding.remediation}", file=sys.stderr)
        return 1

    print(
        "repository compliance manifest validation PASS "
        f"({manifest_summary['manifest_rows']} rows, "
        f"{len(manifest_summary['covered_family_keys'])} families, "
        f"{len(health_summary['red_risk_dependency_ids'])} red-risk upstreams)"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
