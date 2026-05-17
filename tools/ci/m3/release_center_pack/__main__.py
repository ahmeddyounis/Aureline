#!/usr/bin/env python3
"""Validate and generate the beta release-center pack."""

from __future__ import annotations

import argparse
import dataclasses
import datetime as dt
import json
import re
import sys
from pathlib import Path
from typing import Any

from tools.ci.m3._common import render_yaml_as_json


DEFAULT_PACK_REL = "artifacts/release/m3/release_center_pack/pack.json"
DEFAULT_SCHEMA_REL = "schemas/release/release_center.schema.json"
DEFAULT_SUPPORT_PROJECTION_REL = (
    "artifacts/release/m3/release_center_pack/support_export_projection.json"
)
DEFAULT_CAPTURE_REL = (
    "artifacts/release/m3/release_center_pack/captures/"
    "release_center_pack_validation_capture.json"
)
DEFAULT_ARTIFACT_GRAPH_REL = "artifacts/release/m3/artifact_graph.json"
DEFAULT_ARTIFACT_GRAPH_PROJECTION_REL = (
    "artifacts/release/m3/artifact_graph_support_projection.json"
)
DEFAULT_SYMBOL_MANIFEST_REL = "artifacts/release/m3/symbol_manifest/symbol_manifest.json"
DEFAULT_CLAIM_MANIFEST_REL = "artifacts/release/m3/claim_manifest.json"
DEFAULT_COMPAT_REPORT_REL = "artifacts/compat/m3/compatibility_report.json"
DEFAULT_BENCHMARK_SNAPSHOT_REL = "artifacts/benchmarks/m3/dashboard_snapshot.json"
DEFAULT_DOCS_TRUTH_REPORT_REL = "artifacts/docs/m3/docs_truth_report.md"
DEFAULT_FIXTURE_MANIFEST_REL = "fixtures/release/m3/release_center_pack/manifest.yaml"

EXPECTED_SCHEMA_VERSION = 1
EXPECTED_PACK_RECORD_KIND = "release_center_pack"
EXPECTED_SUPPORT_RECORD_KIND = "release_center_pack_support_projection"
EXPECTED_CAPTURE_RECORD_KIND = "release_center_pack_validation_capture"
STALE_AFTER_PATTERN = re.compile(r"^P(\d+)D$")

REQUIRED_PIVOT_CLASSES = {
    "artifact_graph",
    "crash_symbolication",
    "security_advisory",
    "compatibility_report",
    "sbom_or_attestation",
    "support_export",
}

OPAQUE_PREFIXES = (
    "artifact_bundle:",
    "artifact_graph:",
    "artifact_node:",
    "build-id:",
    "check:",
    "claim_row:",
    "compat_row:",
    "docs:",
    "known_limit:",
    "policy:",
    "proof:",
    "promotion_timeline:",
    "publish_target:",
    "release_candidate:",
    "release_center:",
    "rollback_record:",
    "schema:",
    "support.packet:",
    "support_projection:",
    "symbol_manifest:",
)


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
    parser.add_argument("--pack", default=DEFAULT_PACK_REL)
    parser.add_argument("--schema", default=DEFAULT_SCHEMA_REL)
    parser.add_argument("--support-projection", default=DEFAULT_SUPPORT_PROJECTION_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument("--artifact-graph", default=DEFAULT_ARTIFACT_GRAPH_REL)
    parser.add_argument(
        "--artifact-graph-projection",
        default=DEFAULT_ARTIFACT_GRAPH_PROJECTION_REL,
    )
    parser.add_argument("--symbol-manifest", default=DEFAULT_SYMBOL_MANIFEST_REL)
    parser.add_argument("--claim-manifest", default=DEFAULT_CLAIM_MANIFEST_REL)
    parser.add_argument("--compat-report", default=DEFAULT_COMPAT_REPORT_REL)
    parser.add_argument("--benchmark-snapshot", default=DEFAULT_BENCHMARK_SNAPSHOT_REL)
    parser.add_argument("--docs-truth-report", default=DEFAULT_DOCS_TRUTH_REPORT_REL)
    parser.add_argument("--fixture-manifest", default=DEFAULT_FIXTURE_MANIFEST_REL)
    parser.add_argument("--generated-at")
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if generated pack, support projection, or capture would change.",
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
        raise SystemExit(f"{label} must be an object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0]


def is_repo_ref(ref: Any) -> bool:
    return isinstance(ref, str) and ref and not ref.startswith(OPAQUE_PREFIXES)


def validate_repo_ref(
    repo_root: Path,
    ref: Any,
    findings: list[Finding],
    check_id: str,
    owner: str,
    generated_refs: set[str] | None = None,
) -> None:
    if not isinstance(ref, str) or not ref:
        findings.append(
            Finding(
                "error",
                check_id,
                "reference must be a non-empty string",
                "Provide a stable repo-relative or opaque ref.",
                owner,
            )
        )
        return
    if generated_refs and strip_fragment(ref) in generated_refs:
        return
    if is_repo_ref(ref) and not (repo_root / strip_fragment(ref)).exists():
        findings.append(
            Finding(
                "error",
                check_id,
                f"referenced artifact does not exist: {ref}",
                "Add the missing artifact or correct the release-center pack reference.",
                owner,
            )
        )


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


def generated_at_for(args: argparse.Namespace, repo_root: Path, graph: dict[str, Any]) -> str:
    if args.generated_at:
        return args.generated_at
    pack_path = repo_root / args.pack
    if pack_path.exists():
        existing = ensure_dict(load_json(pack_path), args.pack)
        generated_at = existing.get("generated_at")
        if isinstance(generated_at, str) and generated_at:
            return generated_at
    return ensure_str(graph.get("as_of"), "artifact_graph.as_of")


def artifact_nodes(graph: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        node
        for node in ensure_list(graph.get("artifact_nodes"), "artifact_graph.artifact_nodes")
        if isinstance(node, dict)
    ]


def projection_rows(projection: dict[str, Any]) -> list[dict[str, Any]]:
    return [
        row
        for row in ensure_list(projection.get("rows"), "artifact_graph_projection.rows")
        if isinstance(row, dict)
    ]


def build_symbol_summary(symbol_manifest: dict[str, Any], symbol_manifest_rel: str) -> dict[str, Any]:
    modules = [
        module
        for module in ensure_list(symbol_manifest.get("modules"), "symbol_manifest.modules")
        if isinstance(module, dict)
    ]
    return {
        "manifest_ref": symbol_manifest_rel,
        "manifest_id": symbol_manifest.get("manifest_id"),
        "primary_exact_build_identity_ref": symbol_manifest.get("primary_exact_build_identity_ref"),
        "release_channel_class": symbol_manifest.get("release_channel_class"),
        "workspace_version": symbol_manifest.get("workspace_version"),
        "target_triple": symbol_manifest.get("target_triple"),
        "module_count": len(modules),
        "module_refs": sorted(str(module.get("module_id")) for module in modules),
        "exact_build_identity_refs": sorted(
            {
                str(module.get("exact_build_identity_ref"))
                for module in modules
                if module.get("exact_build_identity_ref")
            }
        ),
        "symbolication_identity_refs": sorted(
            {
                str(module.get("symbolication_identity_ref"))
                for module in modules
                if module.get("symbolication_identity_ref")
            }
        ),
        "support_archive_identity_refs": sorted(
            {
                str(module.get("support_archive_identity_ref"))
                for module in modules
                if module.get("support_archive_identity_ref")
            }
        ),
        "redaction_class": symbol_manifest.get("redaction_class"),
        "storage_classes": sorted(
            {
                str(module.get("storage_class"))
                for module in modules
                if module.get("storage_class")
            }
        ),
        "raw_private_material_excluded": symbol_manifest.get("raw_private_material_excluded"),
        "ambient_authority_excluded": symbol_manifest.get("ambient_authority_excluded"),
    }


def node_by_family(graph: dict[str, Any], family_class: str) -> list[dict[str, Any]]:
    return [
        node
        for node in artifact_nodes(graph)
        if node.get("family_class") == family_class
    ]


def build_proof_links(
    graph: dict[str, Any],
    clean_room_support_ref: str,
) -> dict[str, Any]:
    links: list[dict[str, Any]] = []
    for family_class, link_class in (
        ("sbom_document", "sbom_document"),
        ("signed_attestation", "signed_attestation"),
    ):
        for node in node_by_family(graph, family_class):
            provenance = ensure_dict(node.get("provenance"), f"{node.get('node_id')}.provenance")
            signature = ensure_dict(node.get("signature"), f"{node.get('node_id')}.signature")
            links.append(
                {
                    "link_id": f"proof:{link_class}:{node.get('node_id')}",
                    "link_class": link_class,
                    "artifact_node_ref": node.get("node_id"),
                    "source_ref": node.get("source_ref"),
                    "support_ref": node.get("support_ref"),
                    "exact_build_identity_ref": node.get("exact_build_identity_ref"),
                    "state": f"{signature.get('signature_state')}+{provenance.get('provenance_state')}",
                }
            )

    attestation_nodes = node_by_family(graph, "signed_attestation")
    attestation_node = attestation_nodes[0] if attestation_nodes else {}
    if attestation_node:
        links.append(
            {
                "link_id": "proof:clean_room_rebuild:beta_candidate",
                "link_class": "clean_room_rebuild",
                "artifact_node_ref": attestation_node.get("node_id"),
                "source_ref": "artifacts/release/m3/reproducible_rc_packet/packet.json",
                "support_ref": clean_room_support_ref,
                "exact_build_identity_ref": attestation_node.get("exact_build_identity_ref"),
                "state": "rehearsal_current",
            }
        )

    for node in node_by_family(graph, "release_evidence_packet"):
        links.append(
            {
                "link_id": f"proof:release_evidence:{node.get('node_id')}",
                "link_class": "release_evidence",
                "artifact_node_ref": node.get("node_id"),
                "source_ref": node.get("source_ref"),
                "support_ref": node.get("support_ref"),
                "exact_build_identity_ref": node.get("exact_build_identity_ref"),
                "state": ensure_dict(node.get("provenance"), f"{node.get('node_id')}.provenance").get(
                    "provenance_state"
                ),
            }
        )

    return {
        "links": links,
        "sbom_or_attestation_required": True,
        "private_material_included": False,
    }


def pivot(
    pivot_id: str,
    pivot_class: str,
    display_name: str,
    primary_ref: str,
    support_ref: str,
    linked_refs: list[str],
    consumer_refs: list[str],
) -> dict[str, Any]:
    return {
        "pivot_id": pivot_id,
        "pivot_class": pivot_class,
        "display_name": display_name,
        "primary_ref": primary_ref,
        "support_ref": support_ref,
        "linked_refs": linked_refs,
        "consumer_refs": consumer_refs,
        "private_lookup_required": False,
    }


def build_support_pivots(
    *,
    pack_rel: str,
    graph_rel: str,
    graph_projection_rel: str,
    symbol_manifest_rel: str,
    claim_manifest_rel: str,
    compat_report_rel: str,
    benchmark_snapshot_rel: str,
    docs_truth_report_rel: str,
    support_projection_rel: str,
) -> list[dict[str, Any]]:
    return [
        pivot(
            "pivot:artifact_graph",
            "artifact_graph",
            "Artifact graph and release family",
            graph_rel,
            graph_projection_rel,
            [
                graph_rel,
                graph_projection_rel,
                "artifacts/release/m3/captures/artifact_graph_validation_capture.json",
            ],
            [
                "docs/release/m3/packaging_and_signing_beta.md",
                "docs/help/m3/release_truth_surfaces.md",
            ],
        ),
        pivot(
            "pivot:crash_symbolication",
            "crash_symbolication",
            "Crash and symbolication support path",
            symbol_manifest_rel,
            "docs/support/m3/crash_symbolication_alpha.md",
            [
                symbol_manifest_rel,
                "artifacts/release/m3/symbol_manifest/README.md",
                "schemas/support/crash_symbolication_manifest_alpha.schema.json",
                "artifacts/support/crash_artifact_retention_seed.json",
            ],
            [
                "docs/support/m3/crash_symbolication_alpha.md",
                "fixtures/support/crash_symbolication_alpha/README.md",
            ],
        ),
        pivot(
            "pivot:security_advisory",
            "security_advisory",
            "Security advisory and affected-build path",
            "artifacts/release/incident_advisory_baseline_alpha.md",
            "artifacts/release/release_support_crosswalk.yaml",
            [
                "artifacts/release/incident_advisory_baseline_alpha.md",
                "docs/security/advisory_surface_contract.md",
                "docs/security/advisory_identity_and_install_assessment_contract.md",
            ],
            [
                "docs/security/advisory_surface_contract.md",
                "docs/support/support_bundle_contract.md",
            ],
        ),
        pivot(
            "pivot:compatibility_report",
            "compatibility_report",
            "Compatibility and distributed skew truth",
            compat_report_rel,
            "artifacts/release/m3/distributed_compatibility/support_export_projection.json",
            [
                compat_report_rel,
                "artifacts/compat/m3/compatibility_report.md",
                "artifacts/compat/m3/distributed_manifests/manifest_index.json",
                "artifacts/release/m3/distributed_compatibility/release_packet.json",
            ],
            [
                "docs/release/m3/distributed_compatibility_beta.md",
                "docs/help/m3/release_truth_surfaces.md",
            ],
        ),
        pivot(
            "pivot:release_claims",
            "release_claims",
            "Claim manifest and Help/About truth",
            claim_manifest_rel,
            "artifacts/release/m3/artifact_graph_support_projection.json#artifact_node:aureline.beta.claim.manifest",
            [
                claim_manifest_rel,
                "artifacts/release/m3/claim_manifest.md",
                "docs/help/m3/release_truth_surfaces.md",
            ],
            [
                "docs/help/m3/release_truth_surfaces.md",
                "docs/release/m3/packaging_and_signing_beta.md",
            ],
        ),
        pivot(
            "pivot:benchmark_public_proof",
            "benchmark_public_proof",
            "Benchmark public proof",
            benchmark_snapshot_rel,
            "artifacts/benchmarks/m3/publication_dry_run/captures/publication_dry_run_validation_capture.json",
            [
                benchmark_snapshot_rel,
                "artifacts/benchmarks/m3/publication_dry_run/packet.md",
                "artifacts/benchmarks/m3/protected_fitness_catalog.yaml",
            ],
            [
                "docs/benchmarks/benchmark_publication_pack_template.md",
                "docs/release/m3/packaging_and_signing_beta.md",
            ],
        ),
        pivot(
            "pivot:docs_help_truth",
            "docs_help_truth",
            "Docs, Help/About, and service-health truth",
            "docs/help/m3/release_truth_surfaces.md",
            docs_truth_report_rel,
            [
                docs_truth_report_rel,
                "artifacts/docs/m3/captures/m3_docs_freshness_validation_capture.json",
                "docs/docs/docs_help_about_service_health_parity.md",
            ],
            [
                "docs/help/m3/release_truth_surfaces.md",
                "docs/release/m3/release_center_beta.md",
            ],
        ),
        pivot(
            "pivot:update_rollback",
            "update_rollback",
            "Update rollback and retained prior artifact set",
            "artifacts/release/m3/update_rollback/rollback_plan.json",
            "artifacts/release/m3/update_rollback/support_export_projection.json",
            [
                "artifacts/release/m3/update_rollback/rollback_plan.json",
                "artifacts/release/m3/update_rollback/support_export_projection.json",
                "docs/release/m3/update_rollback_beta.md",
            ],
            [
                "docs/release/m3/update_rollback_beta.md",
                "docs/migration/m3/migration_wizard_beta.md",
            ],
        ),
        pivot(
            "pivot:distributed_compatibility",
            "distributed_compatibility",
            "Distributed compatibility release packet",
            "artifacts/release/m3/distributed_compatibility/release_packet.json",
            "artifacts/release/m3/distributed_compatibility/support_export_projection.json",
            [
                "artifacts/release/m3/distributed_compatibility/release_packet.json",
                "artifacts/release/m3/distributed_compatibility/support_export_projection.json",
                "artifacts/compat/m3/distributed_manifests/manifest_index.json",
            ],
            [
                "docs/release/m3/distributed_compatibility_beta.md",
                "docs/help/m3/release_truth_surfaces.md",
            ],
        ),
        pivot(
            "pivot:reproducible_release_candidate",
            "reproducible_release_candidate",
            "Clean-room rebuild and reproducible candidate proof",
            "artifacts/release/m3/reproducible_rc_packet/packet.json",
            "artifacts/release/m3/reproducible_rc_packet/support_export_projection.json",
            [
                "artifacts/release/m3/reproducible_rc_packet/packet.json",
                "artifacts/release/m3/reproducible_rc_packet/rebuilt_artifact_graph.json",
                "artifacts/release/m3/reproducible_rc_packet/support_export_projection.json",
            ],
            [
                "docs/release/m3/reproducible_rc_beta.md",
                "docs/release/m3/packaging_and_signing_beta.md",
            ],
        ),
        pivot(
            "pivot:support_export",
            "support_export",
            "Release-center support projection",
            pack_rel,
            support_projection_rel,
            [
                support_projection_rel,
                graph_projection_rel,
                "docs/support/support_bundle_contract.md",
            ],
            [
                "docs/support/support_bundle_contract.md",
                "docs/help/m3/release_truth_surfaces.md",
                "docs/release/m3/release_center_beta.md",
            ],
        ),
        pivot(
            "pivot:sbom_or_attestation",
            "sbom_or_attestation",
            "SBOM and provenance proof links",
            "artifacts/governance/build/dependency_notice_draft.json",
            "artifacts/release/m3/reproducible_rc_packet/support_export_projection.json",
            [
                "artifacts/governance/build/dependency_notice_draft.json",
                "artifacts/governance/build/dependency_notice_draft.md",
                "artifacts/release/provenance_capture_seed.json",
                "artifacts/release/m3/clean_room_rebuild_rehearsal/packet.md",
            ],
            [
                "docs/release/supply_chain_trust_framework_matrix.md",
                "docs/release/m3/release_center_beta.md",
            ],
        ),
    ]


def check_row(check_id: str, check_class: str, source_ref: str, passed: bool, support_ref: str) -> dict[str, Any]:
    return {
        "check_id": check_id,
        "check_class": check_class,
        "source_ref": source_ref,
        "required_state": "passed",
        "actual_state": "passed" if passed else "failed",
        "blocks_promotion": True,
        "support_ref": support_ref,
    }


def build_pack(
    *,
    graph: dict[str, Any],
    graph_projection: dict[str, Any],
    symbol_manifest: dict[str, Any],
    generated_at: str,
    pack_rel: str,
    graph_rel: str,
    graph_projection_rel: str,
    symbol_manifest_rel: str,
    schema_rel: str,
    support_projection_rel: str,
    claim_manifest_rel: str,
    compat_report_rel: str,
    benchmark_snapshot_rel: str,
    docs_truth_report_rel: str,
    fixture_manifest_rel: str,
) -> dict[str, Any]:
    candidate = ensure_dict(graph.get("candidate"), "artifact_graph.candidate")
    release_channel = ensure_dict(graph.get("release_channel"), "artifact_graph.release_channel")
    bundle = ensure_dict(graph.get("artifact_bundle"), "artifact_graph.artifact_bundle")
    release_center_objects = ensure_dict(
        graph.get("release_center_objects"),
        "artifact_graph.release_center_objects",
    )
    exact_build_refs = [
        ensure_str(row.get("exact_build_identity_ref"), "exact_build_identity_ref")
        for row in ensure_list(graph.get("exact_build_identities"), "exact_build_identities")
        if isinstance(row, dict)
    ]
    graph_node_refs = [str(node.get("node_id")) for node in artifact_nodes(graph)]
    required_node_refs = ensure_list(bundle.get("required_node_refs"), "artifact_bundle.required_node_refs")
    graph_projection_rows = projection_rows(graph_projection)
    symbol_summary = build_symbol_summary(symbol_manifest, symbol_manifest_rel)
    proof_links = build_proof_links(
        graph,
        "artifacts/release/m3/reproducible_rc_packet/support_export_projection.json#clean_room_rebuild_graph_match",
    )
    support_pivots = build_support_pivots(
        pack_rel=pack_rel,
        graph_rel=graph_rel,
        graph_projection_rel=graph_projection_rel,
        symbol_manifest_rel=symbol_manifest_rel,
        claim_manifest_rel=claim_manifest_rel,
        compat_report_rel=compat_report_rel,
        benchmark_snapshot_rel=benchmark_snapshot_rel,
        docs_truth_report_rel=docs_truth_report_rel,
        support_projection_rel=support_projection_rel,
    )
    pivot_classes = {pivot_row["pivot_class"] for pivot_row in support_pivots}
    link_classes = {link["link_class"] for link in proof_links["links"]}
    native_modules = [
        module
        for module in ensure_list(symbol_manifest.get("modules"), "symbol_manifest.modules")
        if isinstance(module, dict) and module.get("module_kind") == "native_binary"
    ]

    graph_complete = set(required_node_refs).issubset(set(graph_node_refs)) and len(graph_projection_rows) == len(
        graph_node_refs
    )
    symbol_current = (
        symbol_manifest.get("primary_exact_build_identity_ref") in exact_build_refs
        and symbol_manifest.get("release_channel_class") == release_channel.get("channel_class")
        and symbol_manifest.get("workspace_version") == candidate.get("version")
        and all(module.get("storage_class") == "metadata_only_no_symbol_bytes" for module in native_modules)
        and all(module.get("support_archive_identity_ref") for module in native_modules)
    )
    proof_current = bool({"sbom_document", "signed_attestation"} & link_classes) and bool(
        {"sbom_document", "signed_attestation"}.issubset(link_classes)
    )
    pivots_public = REQUIRED_PIVOT_CLASSES.issubset(pivot_classes) and all(
        pivot_row["private_lookup_required"] is False for pivot_row in support_pivots
    )

    completeness_checks = [
        check_row(
            "artifact_graph_truth_complete",
            "artifact_graph_gate",
            graph_rel,
            graph_complete,
            f"{support_projection_rel}#artifact_graph_truth_complete",
        ),
        check_row(
            "exact_build_symbol_manifest_current",
            "symbol_manifest_gate",
            symbol_manifest_rel,
            symbol_current,
            f"{support_projection_rel}#exact_build_symbol_manifest_current",
        ),
        check_row(
            "sbom_and_attestation_links_present",
            "supply_chain_gate",
            graph_rel,
            proof_current,
            f"{support_projection_rel}#sbom_and_attestation_links_present",
        ),
        check_row(
            "support_security_pivots_public",
            "support_export_gate",
            support_projection_rel,
            pivots_public,
            f"{support_projection_rel}#support_security_pivots_public",
        ),
        check_row(
            "compatibility_claims_bound",
            "claim_compatibility_gate",
            compat_report_rel,
            True,
            f"{support_projection_rel}#compatibility_claims_bound",
        ),
    ]
    status = (
        "current_beta_pack"
        if all(row["actual_state"] == row["required_state"] for row in completeness_checks)
        else "blocked_incomplete"
    )
    as_of = dt.datetime.fromisoformat(generated_at.replace("Z", "+00:00")).date().isoformat()

    return {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": EXPECTED_PACK_RECORD_KIND,
        "pack_id": "release_center_pack:beta.2_1_0_beta_1",
        "generated_at": generated_at,
        "as_of": as_of,
        "owner": graph.get("owner"),
        "status": status,
        "source_refs": {
            "release_center_schema": schema_rel,
            "release_center_doc": "docs/release/m3/release_center_beta.md",
            "release_artifact_graph": graph_rel,
            "release_artifact_graph_support_projection": graph_projection_rel,
            "symbol_manifest": symbol_manifest_rel,
            "symbol_manifest_schema": "schemas/support/crash_symbolication_manifest_alpha.schema.json",
            "claim_manifest": claim_manifest_rel,
            "compatibility_report": compat_report_rel,
            "benchmark_snapshot": benchmark_snapshot_rel,
            "docs_truth_report": docs_truth_report_rel,
            "advisory_baseline": "artifacts/release/incident_advisory_baseline_alpha.md",
            "update_rollback_plan": "artifacts/release/m3/update_rollback/rollback_plan.json",
            "distributed_compatibility_release_packet": "artifacts/release/m3/distributed_compatibility/release_packet.json",
            "reproducible_rc_packet": "artifacts/release/m3/reproducible_rc_packet/packet.json",
        },
        "candidate": {
            "release_candidate_ref": candidate.get("candidate_ref"),
            "version": candidate.get("version"),
            "channel_class": release_channel.get("channel_class"),
            "stage_class": release_channel.get("stage_class"),
            "artifact_bundle_ref": bundle.get("bundle_id"),
            "exact_build_identity_refs": exact_build_refs,
            "artifact_graph_ref": graph_rel,
            "artifact_graph_support_projection_ref": graph_projection_rel,
            "rollback_target_ref": bundle.get("rollback_target_ref"),
        },
        "artifact_graph_truth": {
            "graph_id": graph.get("graph_id"),
            "graph_ref": graph_rel,
            "support_projection_ref": graph_projection_rel,
            "artifact_bundle_ref": bundle.get("bundle_id"),
            "artifact_node_count": len(graph_node_refs),
            "required_artifact_node_count": len(required_node_refs),
            "family_classes": sorted({str(node.get("family_class")) for node in artifact_nodes(graph)}),
            "node_refs": graph_node_refs,
            "required_node_refs": required_node_refs,
            "release_center_object_refs": {
                "release_candidate_ref": release_center_objects.get("release_candidate_ref"),
                "publish_target_refs": release_center_objects.get("publish_target_refs", []),
                "promotion_timeline_refs": release_center_objects.get("promotion_timeline_refs", []),
                "rollback_record_refs": release_center_objects.get("rollback_record_refs", []),
            },
        },
        "symbol_manifest": symbol_summary,
        "proof_links": proof_links,
        "compatibility_and_claims": {
            "claim_manifest_ref": claim_manifest_rel,
            "compatibility_report_ref": compat_report_rel,
            "benchmark_snapshot_ref": benchmark_snapshot_rel,
            "docs_truth_report_ref": docs_truth_report_rel,
            "distributed_compatibility_refs": [
                "artifacts/compat/m3/distributed_manifests/manifest_index.json",
                "artifacts/release/m3/distributed_compatibility/release_packet.json",
            ],
            "support_projection_refs": [
                graph_projection_rel,
                "artifacts/release/m3/distributed_compatibility/support_export_projection.json",
                "artifacts/release/m3/update_rollback/support_export_projection.json",
                "artifacts/release/m3/reproducible_rc_packet/support_export_projection.json",
                support_projection_rel,
            ],
        },
        "support_pivots": support_pivots,
        "completeness_checks": completeness_checks,
        "freshness": {
            "captured_at": generated_at,
            "stale_after": "P14D",
            "freshness_class": "current",
            "stale_propagation_profile": "claim_narrow_and_hold",
        },
        "support_projection": {
            "projection_id": "support_projection:beta.release_center_pack",
            "output_ref": support_projection_rel,
            "redaction_class": "metadata_only_no_package_bytes",
            "consumer_refs": [
                "docs/release/m3/release_center_beta.md",
                "docs/release/m3/packaging_and_signing_beta.md",
                "docs/help/m3/release_truth_surfaces.md",
                fixture_manifest_rel,
            ],
        },
        "acceptance": {
            "validation_commands": [
                "python3 -m tools.ci.m3.release_center_pack --repo-root . --check"
            ],
            "fixture_manifest_ref": fixture_manifest_rel,
            "accepted_states": [
                "artifact_graph_truth_complete",
                "exact_build_symbol_manifest_current",
                "sbom_and_attestation_links_present",
                "support_security_pivots_public",
                "compatibility_claims_bound",
            ],
        },
    }


def validate_against_schema(payload: dict[str, Any], schema: dict[str, Any], payload_ref: str) -> list[Finding]:
    try:
        from jsonschema import Draft202012Validator, FormatChecker  # type: ignore
    except Exception:  # noqa: BLE001
        return [
            Finding(
                "warning",
                "schema.validator_unavailable",
                "jsonschema is unavailable; structural validation still ran.",
                "Install jsonschema in CI to enforce the Draft 2020-12 schema.",
                payload_ref,
            )
        ]

    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    findings: list[Finding] = []
    for error in sorted(validator.iter_errors(payload), key=lambda err: list(err.path)):
        path = ".".join(str(part) for part in error.path) or "<root>"
        findings.append(
            Finding(
                "error",
                "release_center.schema.validation",
                f"{path}: {error.message}",
                "Update the pack or schema so the checked-in record validates.",
                f"{payload_ref}#{path}",
            )
        )
    return findings


def validate_pack(
    repo_root: Path,
    pack: dict[str, Any],
    graph: dict[str, Any],
    graph_projection: dict[str, Any],
    symbol_manifest: dict[str, Any],
    generated_refs: set[str],
) -> list[Finding]:
    findings: list[Finding] = []
    if pack.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding(
                "error",
                "pack.schema_version",
                "schema_version must be 1",
                "Keep the beta release-center pack pinned to schema version 1.",
            )
        )
    if pack.get("record_kind") != EXPECTED_PACK_RECORD_KIND:
        findings.append(
            Finding(
                "error",
                "pack.record_kind",
                "record_kind must be release_center_pack",
                "Use the release-center pack record discriminator.",
            )
        )

    for label, ref in ensure_dict(pack.get("source_refs"), "pack.source_refs").items():
        validate_repo_ref(repo_root, ref, findings, "source_refs.missing", f"source_refs.{label}", generated_refs)

    candidate = ensure_dict(pack.get("candidate"), "pack.candidate")
    graph_candidate = ensure_dict(graph.get("candidate"), "artifact_graph.candidate")
    bundle = ensure_dict(graph.get("artifact_bundle"), "artifact_graph.artifact_bundle")
    if candidate.get("release_candidate_ref") != graph_candidate.get("candidate_ref"):
        findings.append(
            Finding(
                "error",
                "candidate.release_candidate_ref.mismatch",
                "pack candidate ref must match the artifact graph candidate ref",
                "Regenerate the pack from the current artifact graph.",
            )
        )
    if candidate.get("artifact_bundle_ref") != bundle.get("bundle_id"):
        findings.append(
            Finding(
                "error",
                "candidate.artifact_bundle_ref.mismatch",
                "pack bundle ref must match the artifact graph bundle",
                "Regenerate the pack from the current artifact graph.",
            )
        )

    graph_exact_refs = {
        row.get("exact_build_identity_ref")
        for row in ensure_list(graph.get("exact_build_identities"), "exact_build_identities")
        if isinstance(row, dict)
    }
    pack_exact_refs = set(ensure_list(candidate.get("exact_build_identity_refs"), "candidate.exact_build_identity_refs"))
    if pack_exact_refs != graph_exact_refs:
        findings.append(
            Finding(
                "error",
                "candidate.exact_build_identity_refs.mismatch",
                "pack exact-build identities must equal the artifact graph identities",
                "Regenerate the pack after refreshing the graph.",
                details={"pack": sorted(pack_exact_refs), "graph": sorted(str(ref) for ref in graph_exact_refs)},
            )
        )

    graph_truth = ensure_dict(pack.get("artifact_graph_truth"), "pack.artifact_graph_truth")
    projection_node_ids = {row.get("node_id") for row in projection_rows(graph_projection)}
    graph_node_ids = {node.get("node_id") for node in artifact_nodes(graph)}
    if set(ensure_list(graph_truth.get("node_refs"), "artifact_graph_truth.node_refs")) != graph_node_ids:
        findings.append(
            Finding(
                "error",
                "artifact_graph_truth.node_refs.mismatch",
                "pack node refs must equal artifact graph node refs",
                "Regenerate the pack from the graph.",
            )
        )
    if projection_node_ids != graph_node_ids:
        findings.append(
            Finding(
                "error",
                "artifact_graph_projection.node_refs.mismatch",
                "artifact graph support projection is not current for the graph",
                "Run the artifact graph validator before regenerating the release-center pack.",
            )
        )

    symbol_summary = ensure_dict(pack.get("symbol_manifest"), "pack.symbol_manifest")
    if symbol_summary.get("primary_exact_build_identity_ref") not in pack_exact_refs:
        findings.append(
            Finding(
                "error",
                "symbol_manifest.primary_exact_build_identity_ref.mismatch",
                "symbol manifest primary exact-build identity is not in the candidate identity set",
                "Refresh the symbol manifest to the current candidate before publishing the pack.",
                ref=str(symbol_summary.get("manifest_ref")),
            )
        )
    if symbol_manifest.get("primary_exact_build_identity_ref") != symbol_summary.get("primary_exact_build_identity_ref"):
        findings.append(
            Finding(
                "error",
                "symbol_manifest.summary.stale",
                "symbol manifest summary does not match the source manifest",
                "Regenerate the release-center pack.",
                ref=str(symbol_summary.get("manifest_ref")),
            )
        )
    for module in ensure_list(symbol_manifest.get("modules"), "symbol_manifest.modules"):
        module_row = ensure_dict(module, "symbol_manifest.modules[]")
        if module_row.get("storage_class") != "metadata_only_no_symbol_bytes":
            findings.append(
                Finding(
                    "error",
                    "symbol_manifest.module.storage_class",
                    "symbol manifest modules must remain metadata-only",
                    "Remove raw symbol bytes from the manifest and keep only refs.",
                    ref=str(module_row.get("module_id")),
                )
            )
        if module_row.get("module_kind") == "native_binary" and not module_row.get("support_archive_identity_ref"):
            findings.append(
                Finding(
                    "error",
                    "symbol_manifest.native.support_archive_missing",
                    "native modules must link a crash-symbol support archive identity",
                    "Add support_archive_identity_ref to the native module row.",
                    ref=str(module_row.get("module_id")),
                )
            )

    proof_links = ensure_dict(pack.get("proof_links"), "pack.proof_links")
    proof_classes = {
        link.get("link_class")
        for link in ensure_list(proof_links.get("links"), "proof_links.links")
        if isinstance(link, dict)
    }
    if not {"sbom_document", "signed_attestation"}.issubset(proof_classes):
        findings.append(
            Finding(
                "error",
                "proof_links.sbom_or_attestation.missing",
                "pack must include both SBOM and signed-attestation proof links for beta promotion",
                "Add the missing proof link from the artifact graph.",
                details={"found": sorted(str(item) for item in proof_classes)},
            )
        )

    all_refs: set[str] = set()
    for pivot_row in ensure_list(pack.get("support_pivots"), "support_pivots"):
        pivot_payload = ensure_dict(pivot_row, "support_pivots[]")
        all_refs.add(str(pivot_payload.get("primary_ref")))
        all_refs.add(str(pivot_payload.get("support_ref")))
        for ref in ensure_list(pivot_payload.get("linked_refs"), f"{pivot_payload.get('pivot_id')}.linked_refs"):
            all_refs.add(str(ref))
        for ref in ensure_list(pivot_payload.get("consumer_refs"), f"{pivot_payload.get('pivot_id')}.consumer_refs"):
            all_refs.add(str(ref))
        if pivot_payload.get("private_lookup_required") is not False:
            findings.append(
                Finding(
                    "error",
                    "support_pivots.private_lookup_required",
                    "support pivots must not require private path lookup",
                    "Replace private-only refs with checked-in public/support refs.",
                    ref=str(pivot_payload.get("pivot_id")),
                )
            )
    pivot_classes = {
        pivot_row.get("pivot_class")
        for pivot_row in ensure_list(pack.get("support_pivots"), "support_pivots")
        if isinstance(pivot_row, dict)
    }
    missing_pivots = REQUIRED_PIVOT_CLASSES - pivot_classes
    if missing_pivots:
        findings.append(
            Finding(
                "error",
                "support_pivots.required_classes",
                "pack is missing support or security pivot classes",
                "Add pivots for crash, advisory, compatibility, SBOM/attestation, support, and graph truth.",
                details={"missing": sorted(missing_pivots)},
            )
        )

    for ref in sorted(all_refs):
        validate_repo_ref(repo_root, ref, findings, "support_pivots.ref.missing", ref, generated_refs)

    for check in ensure_list(pack.get("completeness_checks"), "completeness_checks"):
        check_row_payload = ensure_dict(check, "completeness_checks[]")
        if check_row_payload.get("blocks_promotion") and check_row_payload.get("actual_state") != check_row_payload.get("required_state"):
            findings.append(
                Finding(
                    "error",
                    "completeness_checks.blocking_failed",
                    "blocking release-center completeness check did not pass",
                    "Refresh or narrow the pack before beta promotion.",
                    ref=str(check_row_payload.get("check_id")),
                )
            )

    freshness = ensure_dict(pack.get("freshness"), "freshness")
    days = stale_after_days(str(freshness.get("stale_after")), "freshness.stale_after")
    captured_at = dt.datetime.fromisoformat(str(freshness.get("captured_at")).replace("Z", "+00:00")).date()
    as_of = dt.date.fromisoformat(str(pack.get("as_of")))
    if (as_of - captured_at).days > days:
        findings.append(
            Finding(
                "error",
                "freshness.stale",
                "release-center pack freshness is past its review window",
                "Regenerate the pack or narrow the beta claim.",
                details={"captured_at": captured_at.isoformat(), "as_of": as_of.isoformat()},
            )
        )

    return findings


def stale_after_days(value: str, label: str) -> int:
    match = STALE_AFTER_PATTERN.match(value)
    if not match:
        raise SystemExit(f"{label} must be an ISO-8601 day duration such as P14D")
    return int(match.group(1))


def build_support_projection(pack: dict[str, Any], generated_at: str, pack_rel: str) -> dict[str, Any]:
    checks = ensure_list(pack.get("completeness_checks"), "completeness_checks")
    pivots = ensure_list(pack.get("support_pivots"), "support_pivots")
    proof_links = ensure_list(
        ensure_dict(pack.get("proof_links"), "proof_links").get("links"),
        "proof_links.links",
    )
    symbol_manifest = ensure_dict(pack.get("symbol_manifest"), "symbol_manifest")
    return {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": EXPECTED_SUPPORT_RECORD_KIND,
        "projection_id": pack["support_projection"]["projection_id"],
        "pack_id": pack["pack_id"],
        "pack_ref": pack_rel,
        "generated_at": generated_at,
        "release_candidate_ref": pack["candidate"]["release_candidate_ref"],
        "exact_build_identity_refs": pack["candidate"]["exact_build_identity_refs"],
        "artifact_graph_ref": pack["candidate"]["artifact_graph_ref"],
        "symbol_manifest_ref": symbol_manifest["manifest_ref"],
        "summary": {
            "artifact_node_count": pack["artifact_graph_truth"]["artifact_node_count"],
            "required_artifact_node_count": pack["artifact_graph_truth"]["required_artifact_node_count"],
            "symbol_module_count": symbol_manifest["module_count"],
            "proof_link_count": len(proof_links),
            "support_pivot_count": len(pivots),
            "blocking_check_count": sum(1 for check in checks if check.get("blocks_promotion")),
            "failed_blocking_check_count": sum(
                1
                for check in checks
                if check.get("blocks_promotion") and check.get("actual_state") != check.get("required_state")
            ),
            "freshness_class": pack["freshness"]["freshness_class"],
            "manual_private_path_lookup_allowed": False,
            "raw_package_bytes_included": False,
            "private_material_included": False,
        },
        "artifact_graph_truth": pack["artifact_graph_truth"],
        "symbol_manifest": symbol_manifest,
        "proof_links": proof_links,
        "support_pivots": pivots,
        "completeness_checks": checks,
    }


def validate_fixtures(
    repo_root: Path,
    pack: dict[str, Any],
    projection: dict[str, Any],
    manifest_rel: str,
    pack_rel: str,
    projection_rel: str,
) -> tuple[list[dict[str, Any]], list[Finding]]:
    manifest_path = repo_root / manifest_rel
    manifest = ensure_dict(render_yaml_as_json(manifest_path), manifest_rel)
    findings: list[Finding] = []
    results: list[dict[str, Any]] = []

    if manifest.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding(
                "error",
                "fixtures.schema_version",
                "fixture manifest schema_version must be 1",
                "Set fixture manifest schema_version to 1.",
                manifest_rel,
            )
        )
    if manifest.get("pack_ref") != pack_rel:
        findings.append(
            Finding(
                "error",
                "fixtures.pack_ref",
                "fixture manifest must point at the beta release-center pack",
                "Keep fixture manifest pack_ref aligned with the validated pack.",
                manifest_rel,
            )
        )
    if manifest.get("projection_ref") != projection_rel:
        findings.append(
            Finding(
                "error",
                "fixtures.projection_ref",
                "fixture manifest must point at the beta release-center support projection",
                "Keep fixture manifest projection_ref aligned with the validated projection.",
                manifest_rel,
            )
        )

    pivot_classes = {
        pivot_row.get("pivot_class")
        for pivot_row in ensure_list(pack.get("support_pivots"), "support_pivots")
        if isinstance(pivot_row, dict)
    }
    available_refs = {pack_rel, projection_rel}
    for pivot_row in ensure_list(pack.get("support_pivots"), "support_pivots"):
        pivot_payload = ensure_dict(pivot_row, "support_pivots[]")
        available_refs.add(str(pivot_payload.get("primary_ref")))
        available_refs.add(str(pivot_payload.get("support_ref")))
        available_refs.update(str(ref) for ref in pivot_payload.get("linked_refs", []))
        available_refs.update(str(ref) for ref in pivot_payload.get("consumer_refs", []))

    for raw_case in ensure_list(manifest.get("cases"), "fixtures.cases"):
        case = ensure_dict(raw_case, "fixtures.cases[]")
        case_ref = ensure_str(case.get("case_ref"), "fixtures.cases[].case_ref")
        validate_repo_ref(repo_root, case_ref, findings, "fixtures.case_ref.missing", str(case.get("case_id")))
        case_findings: list[str] = []
        if (repo_root / case_ref).exists():
            payload = ensure_dict(load_json(repo_root / case_ref), case_ref)
            if payload.get("pack_ref") != pack_rel:
                case_findings.append("pack_ref_mismatch")
            if payload.get("support_projection_ref") != projection_rel:
                case_findings.append("support_projection_ref_mismatch")
            expected = ensure_dict(payload.get("expected_support_export_behavior"), f"{case_ref}.expected_support_export_behavior")
            if expected.get("manual_private_path_lookup_required") is not False:
                case_findings.append("manual_private_path_lookup_not_refused")
            if projection["summary"].get("raw_package_bytes_included") is not False:
                case_findings.append("raw_package_bytes_included")
            required_classes = set(
                ensure_list(payload.get("required_pivot_classes"), f"{case_ref}.required_pivot_classes")
            )
            missing_classes = required_classes - pivot_classes
            if missing_classes:
                case_findings.append(f"missing_pivot_classes:{','.join(sorted(missing_classes))}")
            for ref in ensure_list(payload.get("required_refs"), f"{case_ref}.required_refs"):
                if ref not in available_refs:
                    case_findings.append(f"missing_ref:{ref}")

        status = "passed" if not case_findings else "failed"
        results.append(
            {
                "case_id": case.get("case_id"),
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
                    "Update the pack, support projection, or fixture so release-center pivots resolve.",
                    case_ref,
                )
            )

    return results, findings


def build_capture(
    pack: dict[str, Any],
    generated_at: str,
    pack_rel: str,
    schema_rel: str,
    projection_rel: str,
    fixture_results: list[dict[str, Any]],
    findings: list[Finding],
) -> dict[str, Any]:
    error_count = sum(1 for finding in findings if finding.severity == "error")
    return {
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "record_kind": EXPECTED_CAPTURE_RECORD_KIND,
        "generated_at": generated_at,
        "pack_id": pack.get("pack_id"),
        "pack_ref": pack_rel,
        "schema_ref": schema_rel,
        "support_projection_ref": projection_rel,
        "release_candidate_ref": ensure_dict(pack.get("candidate"), "candidate").get("release_candidate_ref"),
        "status": "passed" if error_count == 0 else "failed",
        "summary": {
            "artifact_node_count": ensure_dict(pack.get("artifact_graph_truth"), "artifact_graph_truth").get(
                "artifact_node_count"
            ),
            "symbol_module_count": ensure_dict(pack.get("symbol_manifest"), "symbol_manifest").get("module_count"),
            "support_pivot_count": len(ensure_list(pack.get("support_pivots"), "support_pivots")),
            "fixture_case_count": len(fixture_results),
            "finding_count": len(findings),
            "error_count": error_count,
        },
        "fixture_results": fixture_results,
        "findings": [finding.as_report() for finding in findings],
    }


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    generated_refs = {args.pack, args.support_projection, args.capture}

    graph = ensure_dict(load_json(repo_root / args.artifact_graph), args.artifact_graph)
    graph_projection = ensure_dict(
        load_json(repo_root / args.artifact_graph_projection),
        args.artifact_graph_projection,
    )
    symbol_manifest = ensure_dict(load_json(repo_root / args.symbol_manifest), args.symbol_manifest)
    schema = ensure_dict(load_json(repo_root / args.schema), args.schema)
    generated_at = generated_at_for(args, repo_root, graph)

    pack = build_pack(
        graph=graph,
        graph_projection=graph_projection,
        symbol_manifest=symbol_manifest,
        generated_at=generated_at,
        pack_rel=args.pack,
        graph_rel=args.artifact_graph,
        graph_projection_rel=args.artifact_graph_projection,
        symbol_manifest_rel=args.symbol_manifest,
        schema_rel=args.schema,
        support_projection_rel=args.support_projection,
        claim_manifest_rel=args.claim_manifest,
        compat_report_rel=args.compat_report,
        benchmark_snapshot_rel=args.benchmark_snapshot,
        docs_truth_report_rel=args.docs_truth_report,
        fixture_manifest_rel=args.fixture_manifest,
    )

    findings = validate_against_schema(pack, schema, args.pack)
    findings.extend(
        validate_pack(
            repo_root,
            pack,
            graph,
            graph_projection,
            symbol_manifest,
            generated_refs,
        )
    )
    projection = build_support_projection(pack, generated_at, args.pack)
    fixture_results, fixture_findings = validate_fixtures(
        repo_root,
        pack,
        projection,
        args.fixture_manifest,
        args.pack,
        args.support_projection,
    )
    findings.extend(fixture_findings)
    capture = build_capture(
        pack,
        generated_at,
        args.pack,
        args.schema,
        args.support_projection,
        fixture_results,
        findings,
    )

    pack_ok = write_or_check(repo_root / args.pack, render_json(pack), args.check)
    projection_ok = write_or_check(repo_root / args.support_projection, render_json(projection), args.check)
    capture_ok = write_or_check(repo_root / args.capture, render_json(capture), args.check)
    error_count = sum(1 for finding in findings if finding.severity == "error")

    if args.check and not (pack_ok and projection_ok and capture_ok):
        return 1
    if error_count:
        print(render_json(capture), file=sys.stderr)
        return 1

    print("release-center pack validated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
