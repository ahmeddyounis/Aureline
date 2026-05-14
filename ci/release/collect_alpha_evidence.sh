#!/usr/bin/env bash
set -euo pipefail

python3 - "$@" <<'PY'
from __future__ import annotations

import argparse
import datetime as dt
import hashlib
import json
import sys
from pathlib import Path
from typing import Any

import yaml


DEFAULT_GRAPH = "artifacts/release/alpha_artifact_graph.yaml"
DEFAULT_OUT = "target/release-evidence/alpha_evidence_packet.json"
REQUIRED_RECONSTRUCTION_FIELDS = {
    "candidate_version",
    "target_class",
    "digest_set",
    "rollout_ring",
    "auth_source_class",
    "rollback_target_ref",
}
REQUIRED_FAMILY_KEYS = {
    "binaries",
    "symbols",
    "docs_help_packs",
    "schemas",
    "support_exports",
    "release_evidence",
}


class Finding:
    def __init__(self, check_id: str, message: str, ref: str | None = None) -> None:
        self.check_id = check_id
        self.message = message
        self.ref = ref

    def as_dict(self) -> dict[str, str]:
        payload = {"check_id": self.check_id, "message": self.message}
        if self.ref:
            payload["ref"] = self.ref
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Collect the alpha release evidence packet from the artifact graph."
    )
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--graph", default=DEFAULT_GRAPH)
    parser.add_argument("--out", default=DEFAULT_OUT)
    parser.add_argument("--generated-at", default=None)
    parser.add_argument("--validate-only", action="store_true")
    return parser.parse_args()


def load_yaml(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        payload = yaml.safe_load(handle)
    if not isinstance(payload, dict):
        raise SystemExit(f"graph must be a YAML mapping: {path}")
    return payload


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0]


def resolve(repo_root: Path, ref: str) -> Path:
    return repo_root / strip_fragment(ref)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return "sha256:" + digest.hexdigest()


def now_utc() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def list_of_dicts(value: Any) -> list[dict[str, Any]]:
    if not isinstance(value, list):
        return []
    return [item for item in value if isinstance(item, dict)]


def collect_artifact_nodes(graph: dict[str, Any]) -> dict[str, dict[str, Any]]:
    nodes: dict[str, dict[str, Any]] = {}
    families = graph.get("artifact_families", {})
    if not isinstance(families, dict):
        return nodes
    for family_key, rows in families.items():
        for row in list_of_dicts(rows):
            node_id = row.get("node_id")
            if isinstance(node_id, str) and node_id:
                item = dict(row)
                item["_family_key"] = family_key
                nodes[node_id] = item
    return nodes


def first_matching(rows: list[dict[str, Any]], key: str, value: str) -> dict[str, Any] | None:
    for row in rows:
        if row.get(key) == value:
            return row
    return None


def validate_graph(repo_root: Path, graph_path: Path, graph: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    if graph.get("schema_version") != 1:
        findings.append(Finding("graph.schema_version", "schema_version must be 1"))
    if graph.get("record_kind") != "alpha_artifact_graph":
        findings.append(Finding("graph.record_kind", "record_kind must be alpha_artifact_graph"))
    if not str(graph.get("exact_build_identity_ref", "")).startswith("build-id:aureline:"):
        findings.append(Finding("graph.exact_build_identity_ref", "exact-build identity ref is missing or invalid"))

    source_refs = graph.get("source_contract_refs", {})
    if not isinstance(source_refs, dict) or not source_refs:
        findings.append(Finding("source_contract_refs", "source_contract_refs must be a non-empty mapping"))
    else:
        for label, ref in source_refs.items():
            if not isinstance(ref, str) or not resolve(repo_root, ref).exists():
                findings.append(Finding("source_contract_refs.missing", f"{label} does not resolve", str(ref)))

    families = graph.get("artifact_families", {})
    if not isinstance(families, dict):
        findings.append(Finding("artifact_families", "artifact_families must be a mapping"))
        return findings

    missing_family_keys = REQUIRED_FAMILY_KEYS - set(families)
    for family_key in sorted(missing_family_keys):
        findings.append(Finding("artifact_families.missing", f"required artifact family missing: {family_key}"))

    nodes = collect_artifact_nodes(graph)
    for node_id, node in nodes.items():
        for field in ("family_class", "exact_build_identity_ref", "source_ref", "digest_source_ref", "trust_domain"):
            if not node.get(field):
                findings.append(Finding(f"artifact_node.{field}", f"artifact node missing {field}", node_id))
        if node.get("exact_build_identity_ref") != graph.get("exact_build_identity_ref"):
            findings.append(Finding("artifact_node.exact_build_mismatch", "artifact node exact-build ref does not match graph", node_id))
        for field in ("source_ref", "digest_source_ref"):
            ref = node.get(field)
            if isinstance(ref, str) and ref and not resolve(repo_root, ref).exists():
                findings.append(Finding(f"artifact_node.{field}.missing", f"{field} does not resolve", ref))

    release_objects = graph.get("release_center_objects", {})
    if not isinstance(release_objects, dict):
        findings.append(Finding("release_center_objects", "release_center_objects must be a mapping"))
        return findings

    candidates = list_of_dicts(release_objects.get("release_candidate_descriptors"))
    targets = list_of_dicts(release_objects.get("publish_target_descriptors"))
    timelines = list_of_dicts(release_objects.get("promotion_timeline_descriptors"))
    bundles = list_of_dicts(release_objects.get("artifact_bundle_descriptors"))
    if not candidates:
        findings.append(Finding("release_candidate_descriptors", "at least one release candidate descriptor is required"))
    if not targets:
        findings.append(Finding("publish_target_descriptors", "at least one publish target descriptor is required"))
    if not timelines:
        findings.append(Finding("promotion_timeline_descriptors", "at least one promotion timeline descriptor is required"))
    if not bundles:
        findings.append(Finding("artifact_bundle_descriptors", "at least one artifact bundle descriptor is required"))

    required_fields = set(graph.get("evidence_collection", {}).get("required_reconstruction_fields", []))
    missing_fields = REQUIRED_RECONSTRUCTION_FIELDS - required_fields
    for field in sorted(missing_fields):
        findings.append(Finding("evidence_collection.required_reconstruction_fields", f"required reconstruction field missing: {field}"))

    if candidates and targets and bundles:
        candidate = candidates[0]
        for field in ("candidate_id", "candidate_version", "publish_target_refs", "artifact_bundle_refs", "rollback_target_ref"):
            if not candidate.get(field):
                findings.append(Finding(f"release_candidate.{field}", f"candidate missing {field}", str(candidate.get("candidate_id"))))

        target_ids = {target.get("publish_target_id") for target in targets}
        for ref in candidate.get("publish_target_refs", []):
            if ref not in target_ids:
                findings.append(Finding("release_candidate.publish_target_refs", "candidate references an unknown publish target", str(ref)))

        bundle_ids = {bundle.get("bundle_id") for bundle in bundles}
        for ref in candidate.get("artifact_bundle_refs", []):
            if ref not in bundle_ids:
                findings.append(Finding("release_candidate.artifact_bundle_refs", "candidate references an unknown artifact bundle", str(ref)))

        for target in targets:
            for field in ("target_class", "rollout_ring", "auth_source_class", "rollback_target_ref"):
                if not target.get(field):
                    findings.append(Finding(f"publish_target.{field}", f"publish target missing {field}", str(target.get("publish_target_id"))))

        for bundle in bundles:
            digest_set = list_of_dicts(bundle.get("digest_set"))
            if not digest_set:
                findings.append(Finding("artifact_bundle.digest_set", "artifact bundle digest_set must be non-empty", str(bundle.get("bundle_id"))))
            for entry in digest_set:
                ref = entry.get("artifact_node_ref")
                if ref not in nodes:
                    findings.append(Finding("artifact_bundle.digest_set.artifact_node_ref", "digest entry references an unknown artifact node", str(ref)))
                material_ref = entry.get("digest_material_ref")
                if not isinstance(material_ref, str) or not resolve(repo_root, material_ref).exists():
                    findings.append(Finding("artifact_bundle.digest_set.digest_material_ref", "digest material does not resolve", str(material_ref)))

    return findings


def build_packet(repo_root: Path, graph_path: Path, graph: dict[str, Any], generated_at: str) -> dict[str, Any]:
    release_objects = graph["release_center_objects"]
    candidates = list_of_dicts(release_objects.get("release_candidate_descriptors"))
    targets = list_of_dicts(release_objects.get("publish_target_descriptors"))
    bundles = list_of_dicts(release_objects.get("artifact_bundle_descriptors"))
    candidate = candidates[0]
    target_ref = candidate["publish_target_refs"][0]
    bundle_ref = candidate["artifact_bundle_refs"][0]
    target = first_matching(targets, "publish_target_id", target_ref) or targets[0]
    bundle = first_matching(bundles, "bundle_id", bundle_ref) or bundles[0]
    nodes = collect_artifact_nodes(graph)

    digest_set: list[dict[str, Any]] = []
    for entry in list_of_dicts(bundle.get("digest_set")):
        node = nodes[entry["artifact_node_ref"]]
        material_ref = entry["digest_material_ref"]
        digest_set.append(
            {
                "digest_id": entry["digest_id"],
                "artifact_node_ref": entry["artifact_node_ref"],
                "family_class": node["family_class"],
                "family_key": node["_family_key"],
                "material_state": node.get("material_state", ""),
                "source_ref": node["source_ref"],
                "digest_material_ref": material_ref,
                "algorithm": entry.get("algorithm", "sha256"),
                "digest": sha256_file(resolve(repo_root, material_ref)),
            }
        )

    trust_domain_refs = sorted(
        {
            row.get("trust_domain")
            for row in list_of_dicts(graph.get("build_roots"))
            if row.get("trust_domain")
        }
        | {
            row.get("trust_domain")
            for row in list_of_dicts(graph.get("provenance_inputs"))
            if row.get("trust_domain")
        }
        | {
            node.get("trust_domain")
            for node in nodes.values()
            if node.get("trust_domain")
        }
    )

    reconstruction = {
        "candidate_version": candidate["candidate_version"],
        "target_class": target["target_class"],
        "digest_set": [entry["digest"] for entry in digest_set],
        "rollout_ring": target["rollout_ring"],
        "auth_source_class": target["auth_source_class"],
        "rollback_target_ref": target["rollback_target_ref"],
    }

    return {
        "schema_version": 1,
        "record_kind": graph["evidence_collection"]["output_record_kind"],
        "packet_id": "release.evidence.alpha.seed.preview",
        "generated_at": generated_at,
        "graph_ref": str(graph_path),
        "graph_id": graph["graph_id"],
        "exact_build_identity_ref": graph["exact_build_identity_ref"],
        "release_candidate": {
            "candidate_id": candidate["candidate_id"],
            "candidate_version": candidate["candidate_version"],
            "channel_family": candidate["channel_family"],
            "current_stage_class": candidate["current_stage_class"],
            "rollback_target_ref": candidate["rollback_target_ref"],
            "evidence_refs": candidate.get("evidence_refs", []),
            "blockers": candidate.get("blockers", []),
        },
        "publish_target": {
            "publish_target_id": target["publish_target_id"],
            "target_class": target["target_class"],
            "destination_class": target["destination_class"],
            "visibility_class": target["visibility_class"],
            "rollout_ring": target["rollout_ring"],
            "auth_source_class": target["auth_source_class"],
            "rollback_target_ref": target["rollback_target_ref"],
            "evidence_freshness_class": target["evidence_freshness_class"],
        },
        "artifact_bundle": {
            "bundle_id": bundle["bundle_id"],
            "bundle_class": bundle["bundle_class"],
            "signature_state": bundle["signature_state"],
            "attestation_state": bundle["attestation_state"],
            "artifact_node_refs": bundle.get("artifact_node_refs", []),
        },
        "digest_set": digest_set,
        "trust_domain_refs": trust_domain_refs,
        "release_center_reconstruction": reconstruction,
        "raw_private_material_excluded": True,
    }


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    graph_rel = Path(args.graph)
    graph_path = graph_rel if graph_rel.is_absolute() else repo_root / graph_rel
    graph = load_yaml(graph_path)
    findings = validate_graph(repo_root, graph_path, graph)
    if findings:
        report = {"status": "failed", "findings": [finding.as_dict() for finding in findings]}
        print(json.dumps(report, indent=2, sort_keys=True), file=sys.stderr)
        return 1

    generated_at = args.generated_at or now_utc()
    packet = build_packet(repo_root, Path(args.graph), graph, generated_at)
    if args.validate_only:
        print(
            "alpha evidence graph OK: "
            f"{packet['release_candidate']['candidate_version']} -> "
            f"{packet['publish_target']['target_class']} "
            f"({len(packet['digest_set'])} digests)"
        )
        return 0

    out_rel = Path(args.out)
    out_path = out_rel if out_rel.is_absolute() else repo_root / out_rel
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(packet, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(f"wrote {out_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
PY
