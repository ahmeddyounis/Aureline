#!/usr/bin/env python3
"""Validate the beta release artifact graph and render its support projection."""

from __future__ import annotations

import argparse
import dataclasses
import hashlib
import json
import subprocess
import sys
from pathlib import Path
from typing import Any


DEFAULT_GRAPH_REL = "artifacts/release/m3/artifact_graph.json"
DEFAULT_SCHEMA_REL = "schemas/release/artifact_graph.schema.json"
DEFAULT_PROJECTION_REL = "artifacts/release/m3/artifact_graph_support_projection.json"
DEFAULT_CAPTURE_REL = "artifacts/release/m3/captures/artifact_graph_validation_capture.json"
DEFAULT_FIXTURE_MANIFEST_REL = "fixtures/release/artifact_graph_cases/manifest.yaml"

OPAQUE_PREFIXES = (
    "artifact_bundle:",
    "artifact_graph:",
    "artifact_node:",
    "build-id:",
    "commit:",
    "docs:",
    "policy:",
    "promotion_timeline:",
    "publish_target:",
    "release_candidate:",
    "rollback_record:",
    "schema:",
    "support.packet:",
    "support_projection:",
)

REQUIRED_FAMILIES = {
    "ide_binary",
    "cli_binary",
    "remote_agent_tarball",
    "update_metadata",
    "policy_bundle",
    "schema_export",
    "docs_pack",
    "compatibility_report",
    "benchmark_snapshot",
    "debug_symbol_manifest",
    "sbom_document",
    "signed_attestation",
    "support_runbook_bundle",
    "release_evidence_packet",
}


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    ref: str | None = None
    details: dict[str, Any] = dataclasses.field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = dataclasses.asdict(self)
        if payload["ref"] is None:
            payload.pop("ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--graph", default=DEFAULT_GRAPH_REL)
    parser.add_argument("--schema", default=DEFAULT_SCHEMA_REL)
    parser.add_argument("--projection", default=DEFAULT_PROJECTION_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument("--fixture-manifest", default=DEFAULT_FIXTURE_MANIFEST_REL)
    parser.add_argument("--generated-at")
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if the generated support projection or validation capture would change.",
    )
    return parser.parse_args()


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0]


def is_repo_ref(ref: Any) -> bool:
    return isinstance(ref, str) and ref and not ref.startswith(OPAQUE_PREFIXES)


def resolve_ref(repo_root: Path, ref: str) -> Path:
    return repo_root / strip_fragment(ref)


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def load_yaml(path: Path) -> Any:
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
        raise SystemExit(f"failed to parse YAML at {path}: {stderr}")
    return json.loads(ruby.stdout)


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be an object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list")
    return value


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return "sha256:" + digest.hexdigest()


def validate_against_schema(payload: Any, schema: dict[str, Any]) -> list[Finding]:
    try:
        from jsonschema import Draft202012Validator, FormatChecker  # type: ignore
    except Exception:  # noqa: BLE001
        return [
            Finding(
                severity="warning",
                check_id="schema.validator_unavailable",
                message="jsonschema is unavailable; structural validation still ran.",
                remediation="Install jsonschema in CI to enforce the Draft 2020-12 schema.",
            )
        ]

    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    findings: list[Finding] = []
    for error in sorted(validator.iter_errors(payload), key=lambda err: list(err.path)):
        path = ".".join(str(part) for part in error.path) or "<root>"
        findings.append(
            Finding(
                severity="error",
                check_id="schema.validation",
                message=f"{path}: {error.message}",
                remediation="Update the graph or schema so the checked-in graph validates.",
                ref=path,
            )
        )
    return findings


def validate_repo_ref(repo_root: Path, ref: Any, findings: list[Finding], check_id: str, owner: str) -> None:
    if not isinstance(ref, str) or not ref:
        findings.append(
            Finding(
                severity="error",
                check_id=check_id,
                message="reference must be a non-empty string",
                remediation="Provide a stable repo-relative or opaque ref.",
                ref=owner,
            )
        )
        return
    if is_repo_ref(ref) and not resolve_ref(repo_root, ref).exists():
        findings.append(
            Finding(
                severity="error",
                check_id=check_id,
                message=f"referenced artifact does not exist: {ref}",
                remediation="Add the missing artifact or correct the graph reference.",
                ref=owner,
            )
        )


def artifact_nodes(graph: dict[str, Any]) -> list[dict[str, Any]]:
    return [node for node in ensure_list(graph.get("artifact_nodes"), "artifact_nodes") if isinstance(node, dict)]


def validate_graph(repo_root: Path, graph: dict[str, Any], projection_rel: str) -> list[Finding]:
    findings: list[Finding] = []
    if graph.get("artifact_graph_schema_version") != 1:
        findings.append(
            Finding(
                "error",
                "graph.schema_version",
                "artifact_graph_schema_version must be 1",
                "Keep the graph pinned to schema version 1 until a governed migration exists.",
            )
        )
    if graph.get("record_kind") != "release_artifact_graph":
        findings.append(
            Finding(
                "error",
                "graph.record_kind",
                "record_kind must be release_artifact_graph",
                "Use the release artifact graph record discriminator.",
            )
        )

    source_refs = ensure_dict(graph.get("source_contract_refs"), "source_contract_refs")
    for label, ref in source_refs.items():
        validate_repo_ref(repo_root, ref, findings, "source_contract_refs.missing", f"source_contract_refs.{label}")

    governance = ensure_dict(graph.get("policy_and_schema_governance"), "policy_and_schema_governance")
    validate_repo_ref(repo_root, governance.get("graph_schema_ref"), findings, "governance.graph_schema_ref.missing", "policy_and_schema_governance.graph_schema_ref")

    policy_ids: set[str] = set()
    for row in ensure_list(governance.get("policy_refs"), "policy_refs"):
        policy = ensure_dict(row, "policy_refs[]")
        policy_id = str(policy.get("policy_id", ""))
        if policy_id in policy_ids:
            findings.append(
                Finding("error", "policy_refs.duplicate", "policy_id must be unique", "Remove the duplicate policy row.", ref=policy_id)
            )
        policy_ids.add(policy_id)
        validate_repo_ref(repo_root, policy.get("policy_ref"), findings, "policy_refs.policy_ref.missing", policy_id)
        if not policy.get("governs_refs"):
            findings.append(
                Finding("error", "policy_refs.governs_refs.empty", "policy row must govern at least one artifact ref", "List the refs governed by the policy.", ref=policy_id)
            )

    schema_ids: set[str] = set()
    for row in ensure_list(governance.get("schema_refs"), "schema_refs"):
        schema = ensure_dict(row, "schema_refs[]")
        schema_id = str(schema.get("schema_id", ""))
        if schema_id in schema_ids:
            findings.append(
                Finding("error", "schema_refs.duplicate", "schema_id must be unique", "Remove the duplicate schema row.", ref=schema_id)
            )
        schema_ids.add(schema_id)
        validate_repo_ref(repo_root, schema.get("schema_ref"), findings, "schema_refs.schema_ref.missing", schema_id)
        if not schema.get("governs_refs"):
            findings.append(
                Finding("error", "schema_refs.governs_refs.empty", "schema row must govern at least one artifact ref", "List the refs governed by the schema.", ref=schema_id)
            )

    exact_refs = {
        row.get("exact_build_identity_ref")
        for row in ensure_list(graph.get("exact_build_identities"), "exact_build_identities")
        if isinstance(row, dict)
    }
    for row in ensure_list(graph.get("exact_build_identities"), "exact_build_identities"):
        identity = ensure_dict(row, "exact_build_identities[]")
        validate_repo_ref(repo_root, identity.get("build_identity_source_ref"), findings, "exact_build_identity.source_ref.missing", str(identity.get("exact_build_identity_ref")))

    nodes = artifact_nodes(graph)
    node_ids = {str(node.get("node_id")) for node in nodes}
    family_classes = {str(node.get("family_class")) for node in nodes}
    duplicate_node_ids = {node_id for node_id in node_ids if sum(1 for node in nodes if node.get("node_id") == node_id) > 1}
    for node_id in sorted(duplicate_node_ids):
        findings.append(
            Finding("error", "artifact_nodes.duplicate", "artifact node ids must be unique", "Give each graph node one stable id.", ref=node_id)
        )

    missing_families = REQUIRED_FAMILIES - family_classes
    if missing_families:
        findings.append(
            Finding(
                "error",
                "artifact_nodes.required_families",
                "graph is missing required beta artifact families",
                "Add nodes for every required beta artifact family.",
                details={"missing": sorted(missing_families)},
            )
        )

    projection_output_rel = ensure_dict(graph.get("support_projection"), "support_projection").get("output_ref", projection_rel)
    for node in nodes:
        node_id = str(node.get("node_id"))
        if node.get("exact_build_identity_ref") not in exact_refs:
            findings.append(
                Finding(
                    "error",
                    "artifact_node.exact_build_identity_ref.unknown",
                    "artifact node points at an unknown exact build identity",
                    "Add the exact build identity row or correct the node.",
                    ref=node_id,
                )
            )

        for field in ("source_ref",):
            ref = node.get(field)
            if ref == projection_output_rel:
                continue
            validate_repo_ref(repo_root, ref, findings, f"artifact_node.{field}.missing", node_id)

        digest = ensure_dict(node.get("digest"), f"{node_id}.digest")
        material_ref = digest.get("material_ref")
        if material_ref != projection_output_rel:
            validate_repo_ref(repo_root, material_ref, findings, "artifact_node.digest.material_ref.missing", node_id)
            if isinstance(material_ref, str) and is_repo_ref(material_ref):
                path = resolve_ref(repo_root, material_ref)
                if path.exists() and digest.get("value") not in (None, sha256_file(path)):
                    findings.append(
                        Finding(
                            "error",
                            "artifact_node.digest.value_mismatch",
                            "declared digest does not match digest material",
                            "Refresh the digest value or leave it null for validator computation.",
                            ref=node_id,
                        )
                    )

        for policy_id in ensure_list(node.get("policy_refs"), f"{node_id}.policy_refs"):
            if policy_id not in policy_ids:
                findings.append(
                    Finding(
                        "error",
                        "artifact_node.policy_refs.unknown",
                        "artifact node cites an unknown policy ref",
                        "Add the policy row to policy_and_schema_governance.policy_refs or correct the node.",
                        ref=f"{node_id}#{policy_id}",
                    )
                )
        for schema_id in ensure_list(node.get("schema_refs"), f"{node_id}.schema_refs"):
            if schema_id not in schema_ids:
                findings.append(
                    Finding(
                        "error",
                        "artifact_node.schema_refs.unknown",
                        "artifact node cites an unknown schema ref",
                        "Add the schema row to policy_and_schema_governance.schema_refs or correct the node.",
                        ref=f"{node_id}#{schema_id}",
                    )
                )

        signature = ensure_dict(node.get("signature"), f"{node_id}.signature")
        if node.get("required_for_candidate") is True and signature.get("signature_state") == "missing_blocking":
            findings.append(
                Finding(
                    "error",
                    "artifact_node.signature.missing_blocking",
                    "required artifact node cannot carry missing_blocking signature state",
                    "Attach a signature hook or remove the node from candidate-required scope.",
                    ref=node_id,
                )
            )

        provenance = ensure_dict(node.get("provenance"), f"{node_id}.provenance")
        if node.get("required_for_candidate") is True and provenance.get("provenance_state") == "missing_blocking":
            findings.append(
                Finding(
                    "error",
                    "artifact_node.provenance.missing_blocking",
                    "required artifact node cannot carry missing_blocking provenance state",
                    "Attach provenance evidence or remove the node from candidate-required scope.",
                    ref=node_id,
                )
            )

    bundle = ensure_dict(graph.get("artifact_bundle"), "artifact_bundle")
    bundle_refs = set(ensure_list(bundle.get("node_refs"), "artifact_bundle.node_refs"))
    required_bundle_refs = set(ensure_list(bundle.get("required_node_refs"), "artifact_bundle.required_node_refs"))
    required_nodes = {str(node.get("node_id")) for node in nodes if node.get("required_for_candidate") is True}
    missing_from_bundle = required_nodes - bundle_refs
    missing_required = required_nodes - required_bundle_refs
    unknown_bundle_refs = bundle_refs - node_ids
    if missing_from_bundle:
        findings.append(
            Finding("error", "artifact_bundle.node_refs.missing_required", "candidate-required nodes are missing from bundle node_refs", "Add every candidate-required node to artifact_bundle.node_refs.", details={"missing": sorted(missing_from_bundle)})
        )
    if missing_required:
        findings.append(
            Finding("error", "artifact_bundle.required_node_refs.missing_required", "candidate-required nodes are missing from required_node_refs", "Add every candidate-required node to artifact_bundle.required_node_refs.", details={"missing": sorted(missing_required)})
        )
    if unknown_bundle_refs:
        findings.append(
            Finding("error", "artifact_bundle.node_refs.unknown", "bundle references unknown artifact nodes", "Correct the node refs or add the missing graph nodes.", details={"unknown": sorted(unknown_bundle_refs)})
        )

    known_graph_refs = node_ids | exact_refs | {graph.get("graph_id"), bundle.get("bundle_id")}
    for row in ensure_list(graph.get("artifact_edges"), "artifact_edges"):
        edge = ensure_dict(row, "artifact_edges[]")
        for field in ("source_ref", "target_ref"):
            ref = edge.get(field)
            if ref not in known_graph_refs:
                findings.append(
                    Finding(
                        "error",
                        f"artifact_edges.{field}.unknown",
                        "artifact edge endpoint does not resolve inside this graph",
                        "Edges must point at graph ids, bundle ids, exact-build ids, or artifact node ids.",
                        ref=f"{edge.get('edge_id')}#{ref}",
                    )
                )

    release_objects = ensure_dict(graph.get("release_center_objects"), "release_center_objects")
    candidate = ensure_dict(graph.get("candidate"), "candidate")
    if release_objects.get("release_candidate_ref") != candidate.get("candidate_ref"):
        findings.append(
            Finding(
                "error",
                "release_center_objects.release_candidate_ref.mismatch",
                "release-center candidate ref must match candidate.candidate_ref",
                "Use one release candidate id across graph, bundle, support projection, and release-center refs.",
            )
        )

    support_projection = ensure_dict(graph.get("support_projection"), "support_projection")
    required_fields = set(ensure_list(support_projection.get("required_fields"), "support_projection.required_fields"))
    missing_projection_fields = {
        "node_id",
        "family_class",
        "computed_digest",
        "signature_state",
        "provenance_state",
        "policy_refs",
        "schema_refs",
        "rollback_target_ref",
        "support_ref",
    } - required_fields
    if missing_projection_fields:
        findings.append(
            Finding(
                "error",
                "support_projection.required_fields.missing",
                "support projection omits required reconstruction fields",
                "Keep the support projection contract sufficient to reconstruct graph relationships.",
                details={"missing": sorted(missing_projection_fields)},
            )
        )
    for ref in ensure_list(support_projection.get("consumer_refs"), "support_projection.consumer_refs"):
        validate_repo_ref(repo_root, ref, findings, "support_projection.consumer_refs.missing", "support_projection.consumer_refs")

    return findings


def compute_node_digest(repo_root: Path, node: dict[str, Any], projection_rel: str) -> tuple[str | None, str]:
    digest = ensure_dict(node.get("digest"), f"{node.get('node_id')}.digest")
    material_ref = digest.get("material_ref")
    if material_ref == projection_rel:
        return None, "self_referential_projection"
    if isinstance(material_ref, str) and is_repo_ref(material_ref):
        path = resolve_ref(repo_root, material_ref)
        if path.exists():
            return sha256_file(path), "computed"
    if isinstance(digest.get("value"), str):
        return digest["value"], "declared_only"
    return None, "missing_material"


def build_projection(
    repo_root: Path,
    graph: dict[str, Any],
    generated_at: str,
    projection_rel: str,
    graph_rel: str,
) -> dict[str, Any]:
    rows: list[dict[str, Any]] = []
    for node in artifact_nodes(graph):
        signature = ensure_dict(node.get("signature"), f"{node.get('node_id')}.signature")
        provenance = ensure_dict(node.get("provenance"), f"{node.get('node_id')}.provenance")
        rollback = ensure_dict(node.get("rollback"), f"{node.get('node_id')}.rollback")
        computed_digest, digest_state = compute_node_digest(repo_root, node, projection_rel)
        rows.append(
            {
                "node_id": node.get("node_id"),
                "family_class": node.get("family_class"),
                "artifact_role": node.get("artifact_role"),
                "display_name": node.get("display_name"),
                "required_for_candidate": node.get("required_for_candidate"),
                "source_ref": node.get("source_ref"),
                "material_state": node.get("material_state"),
                "computed_digest": computed_digest,
                "digest_state": digest_state,
                "signature_state": signature.get("signature_state"),
                "verification_refs": signature.get("verification_refs", []),
                "trust_root_ref": signature.get("trust_root_ref"),
                "provenance_state": provenance.get("provenance_state"),
                "attestation_refs": provenance.get("attestation_refs", []),
                "sbom_refs": provenance.get("sbom_refs", []),
                "producer_lane_ref": provenance.get("producer_lane_ref"),
                "policy_refs": node.get("policy_refs", []),
                "schema_refs": node.get("schema_refs", []),
                "rollback_target_ref": rollback.get("rollback_target_ref"),
                "rollback_state": rollback.get("rollback_state"),
                "support_ref": node.get("support_ref"),
                "compatibility_refs": node.get("compatibility_refs", []),
            }
        )

    required_rows = [row for row in rows if row["required_for_candidate"] is True]
    signature_summary: dict[str, int] = {}
    provenance_summary: dict[str, int] = {}
    for row in rows:
        signature_summary[row["signature_state"]] = signature_summary.get(row["signature_state"], 0) + 1
        provenance_summary[row["provenance_state"]] = provenance_summary.get(row["provenance_state"], 0) + 1

    return {
        "schema_version": 1,
        "record_kind": "release_artifact_graph_support_projection",
        "projection_id": graph["support_projection"]["projection_id"],
        "graph_id": graph["graph_id"],
        "graph_ref": graph_rel,
        "generated_at": generated_at,
        "release_candidate_ref": graph["candidate"]["candidate_ref"],
        "artifact_bundle_ref": graph["artifact_bundle"]["bundle_id"],
        "rollback_target_ref": graph["artifact_bundle"]["rollback_target_ref"],
        "exact_build_identity_refs": [
            row["exact_build_identity_ref"]
            for row in ensure_list(graph.get("exact_build_identities"), "exact_build_identities")
            if isinstance(row, dict)
        ],
        "summary": {
            "node_count": len(rows),
            "required_node_count": len(required_rows),
            "family_classes": sorted({str(row["family_class"]) for row in rows}),
            "signature_states": signature_summary,
            "provenance_states": provenance_summary,
            "manual_relationship_reconstruction_allowed": False,
            "raw_package_bytes_included": False,
        },
        "rows": rows,
    }


def validate_fixtures(
    repo_root: Path,
    graph: dict[str, Any],
    projection: dict[str, Any],
    manifest_rel: str,
    graph_rel: str,
) -> tuple[list[dict[str, Any]], list[Finding]]:
    manifest_path = repo_root / manifest_rel
    manifest = ensure_dict(load_yaml(manifest_path), manifest_rel)
    findings: list[Finding] = []
    results: list[dict[str, Any]] = []
    if manifest.get("schema_version") != 1:
        findings.append(
            Finding("error", "fixtures.schema_version", "fixture manifest schema_version must be 1", "Set fixture manifest schema_version to 1.", ref=manifest_rel)
        )
    if manifest.get("graph_ref") != graph_rel:
        findings.append(
            Finding("error", "fixtures.graph_ref", "fixture manifest must point at the beta artifact graph", "Keep fixture manifest graph_ref aligned with the validated graph.", ref=manifest_rel)
        )

    projection_node_ids = {row.get("node_id") for row in ensure_list(projection.get("rows"), "projection.rows") if isinstance(row, dict)}
    graph_node_ids = {node.get("node_id") for node in artifact_nodes(graph)}
    policy_ids = {
        row.get("policy_id")
        for row in ensure_list(graph["policy_and_schema_governance"].get("policy_refs"), "policy_refs")
        if isinstance(row, dict)
    }
    schema_ids = {
        row.get("schema_id")
        for row in ensure_list(graph["policy_and_schema_governance"].get("schema_refs"), "schema_refs")
        if isinstance(row, dict)
    }

    for raw_case in ensure_list(manifest.get("cases"), "fixtures.cases"):
        case = ensure_dict(raw_case, "fixtures.cases[]")
        case_ref = str(case.get("case_ref", ""))
        validate_repo_ref(repo_root, case_ref, findings, "fixtures.case_ref.missing", str(case.get("case_id")))
        if not case_ref or not (repo_root / case_ref).exists():
            continue
        payload = ensure_dict(load_json(repo_root / case_ref), case_ref)
        case_findings: list[str] = []
        if payload.get("graph_id") != graph.get("graph_id"):
            case_findings.append("graph_id_mismatch")
        if payload.get("release_candidate_ref") != graph["candidate"]["candidate_ref"]:
            case_findings.append("release_candidate_ref_mismatch")
        if payload.get("support_projection_ref") != graph["support_projection"]["output_ref"]:
            case_findings.append("support_projection_ref_mismatch")

        refs = ensure_dict(payload.get("support_reconstruction_refs"), f"{case_ref}.support_reconstruction_refs")
        for node_ref in ensure_list(refs.get("artifact_node_refs"), f"{case_ref}.artifact_node_refs"):
            if node_ref not in graph_node_ids:
                case_findings.append(f"unknown_graph_node:{node_ref}")
            if node_ref not in projection_node_ids:
                case_findings.append(f"missing_projection_node:{node_ref}")
        for policy_ref in ensure_list(refs.get("policy_refs"), f"{case_ref}.policy_refs"):
            if policy_ref not in policy_ids:
                case_findings.append(f"unknown_policy_ref:{policy_ref}")
        for schema_ref in ensure_list(refs.get("schema_refs"), f"{case_ref}.schema_refs"):
            if schema_ref not in schema_ids:
                case_findings.append(f"unknown_schema_ref:{schema_ref}")

        behavior = ensure_dict(payload.get("expected_support_export_behavior"), f"{case_ref}.expected_support_export_behavior")
        if behavior.get("manual_relationship_reconstruction_allowed") is not False:
            case_findings.append("manual_relationship_reconstruction_not_refused")
        if behavior.get("rollback_target_ref") != graph["artifact_bundle"]["rollback_target_ref"]:
            case_findings.append("rollback_target_ref_mismatch")

        status = "passed" if not case_findings else "failed"
        results.append(
            {
                "case_id": payload.get("case_id", case.get("case_id")),
                "case_ref": case_ref,
                "status": status,
                "findings": case_findings,
            }
        )
        for item in case_findings:
            findings.append(
                Finding(
                    "error",
                    "fixtures.case.failed",
                    f"fixture case failed: {item}",
                    "Update the graph, projection, or fixture so the support-export backreference resolves.",
                    ref=case_ref,
                )
            )
    return results, findings


def build_capture(
    graph: dict[str, Any],
    generated_at: str,
    graph_rel: str,
    schema_rel: str,
    projection_rel: str,
    fixture_results: list[dict[str, Any]],
    findings: list[Finding],
) -> dict[str, Any]:
    error_count = sum(1 for finding in findings if finding.severity == "error")
    return {
        "schema_version": 1,
        "record_kind": "artifact_graph_validation_capture",
        "generated_at": generated_at,
        "graph_id": graph.get("graph_id"),
        "graph_ref": graph_rel,
        "schema_ref": schema_rel,
        "support_projection_ref": projection_rel,
        "release_candidate_ref": graph.get("candidate", {}).get("candidate_ref"),
        "artifact_bundle_ref": graph.get("artifact_bundle", {}).get("bundle_id"),
        "status": "passed" if error_count == 0 else "failed",
        "summary": {
            "artifact_node_count": len(artifact_nodes(graph)),
            "required_node_count": sum(1 for node in artifact_nodes(graph) if node.get("required_for_candidate") is True),
            "fixture_case_count": len(fixture_results),
            "finding_count": len(findings),
            "error_count": error_count,
        },
        "fixture_results": fixture_results,
        "findings": [finding.as_report() for finding in findings],
    }


def render_json(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=True) + "\n"


def write_or_check(path: Path, content: str, check: bool) -> bool:
    if check:
        if not path.exists() or path.read_text(encoding="utf-8") != content:
            print(f"would update {path}", file=sys.stderr)
            return False
        return True
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")
    return True


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    graph_rel = args.graph
    schema_rel = args.schema
    projection_rel = args.projection
    capture_rel = args.capture

    graph = ensure_dict(load_json(repo_root / graph_rel), graph_rel)
    schema = ensure_dict(load_json(repo_root / schema_rel), schema_rel)
    generated_at = args.generated_at or str(graph.get("as_of"))

    findings = validate_against_schema(graph, schema)
    findings.extend(validate_graph(repo_root, graph, projection_rel))
    projection = build_projection(repo_root, graph, generated_at, projection_rel, graph_rel)
    fixture_results, fixture_findings = validate_fixtures(repo_root, graph, projection, args.fixture_manifest, graph_rel)
    findings.extend(fixture_findings)
    capture = build_capture(graph, generated_at, graph_rel, schema_rel, projection_rel, fixture_results, findings)

    projection_ok = write_or_check(repo_root / projection_rel, render_json(projection), args.check)
    capture_ok = write_or_check(repo_root / capture_rel, render_json(capture), args.check)
    error_count = sum(1 for finding in findings if finding.severity == "error")

    if args.check and not (projection_ok and capture_ok):
        return 1
    if error_count:
        print(render_json(capture), file=sys.stderr)
        return 1

    print("release artifact graph validated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
