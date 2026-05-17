#!/usr/bin/env python3
"""Validate beta update rollback plans and render their support projection."""

from __future__ import annotations

import argparse
import copy
import dataclasses
import json
import sys
from pathlib import Path
from typing import Any

from tools.ci.m3._common import render_yaml_as_json


DEFAULT_PLAN_REL = "artifacts/release/m3/update_rollback/rollback_plan.json"
DEFAULT_SCHEMA_REL = "schemas/release/rollback_plan.schema.json"
DEFAULT_SUPPORT_PROJECTION_REL = (
    "artifacts/release/m3/update_rollback/support_export_projection.json"
)
DEFAULT_CAPTURE_REL = (
    "artifacts/release/m3/captures/update_rollback_validation_capture.json"
)
DEFAULT_ARTIFACT_GRAPH_REL = "artifacts/release/m3/artifact_graph.json"
DEFAULT_RING_ROLLOUT_REL = "artifacts/release/m3/ring_rollout/packet.json"
DEFAULT_INSTALL_DIAGNOSTICS_REL = (
    "artifacts/release/m3/install_diagnostics/install_diagnostics_packet.json"
)
DEFAULT_FIXTURE_MANIFEST_REL = "fixtures/release/update_rollback_plan_cases/manifest.yaml"

EXPECTED_SCHEMA_VERSION = 1
EXPECTED_PLAN_RECORD_KIND = "update_rollback_plan_record"
EXPECTED_SUPPORT_RECORD_KIND = "update_rollback_support_export"
EXPECTED_CAPTURE_RECORD_KIND = "update_rollback_validation_capture"
REQUIRED_RETENTION_STATE = "retained_exact_build"
REQUIRED_VERIFICATION_STATE = "verified"
REQUIRED_RETAINED_FAMILIES = {
    "ide_binary",
    "cli_binary",
    "remote_agent_tarball",
    "update_metadata",
    "policy_bundle",
    "schema_export",
    "docs_pack",
    "support_runbook_bundle",
    "release_evidence_packet",
}
REQUIRED_VOCABULARY_TERMS = {
    "retained_prior_artifact_set",
    "schema_rollback_hook",
    "downgrade_eligibility_state",
    "exact_build_identity_ref",
}
ADMITTED_DOWNGRADE_STATES = {
    "auto_eligible",
    "eligible_with_review",
    "manual_review_required",
}
AUTOMATIC_SCHEMA_COMPATIBILITY = {
    "backward_readable",
    "additive_compatible",
}
OPAQUE_PREFIXES = (
    "artifact_bundle:",
    "backup_snapshot:",
    "build-id:",
    "checkpoint.",
    "digest:",
    "migration_journal:",
    "policy:",
    "release_candidate:",
    "repair_transaction:",
    "retained_artifact:",
    "review:",
    "schema_hook:",
    "state.",
    "support_bundle:",
    "update_manifest:",
    "update_ready_review:",
)


@dataclasses.dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    ref: str
    remediation: str

    def as_report(self) -> dict[str, str]:
        return dataclasses.asdict(self)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--plan", default=DEFAULT_PLAN_REL)
    parser.add_argument("--schema", default=DEFAULT_SCHEMA_REL)
    parser.add_argument("--support-projection", default=DEFAULT_SUPPORT_PROJECTION_REL)
    parser.add_argument("--capture", default=DEFAULT_CAPTURE_REL)
    parser.add_argument("--artifact-graph", default=DEFAULT_ARTIFACT_GRAPH_REL)
    parser.add_argument("--ring-rollout", default=DEFAULT_RING_ROLLOUT_REL)
    parser.add_argument("--install-diagnostics", default=DEFAULT_INSTALL_DIAGNOSTICS_REL)
    parser.add_argument("--fixture-manifest", default=DEFAULT_FIXTURE_MANIFEST_REL)
    parser.add_argument(
        "--check",
        action="store_true",
        help="Fail if generated support projection or validation capture would change.",
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
        raise SystemExit(f"{label} must be a JSON object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a JSON array")
    return value


def as_json_text(payload: dict[str, Any]) -> str:
    return json.dumps(payload, indent=2, sort_keys=False) + "\n"


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0]


def is_repo_ref(ref: Any) -> bool:
    return isinstance(ref, str) and ref and not ref.startswith(OPAQUE_PREFIXES)


def repo_ref_exists(repo_root: Path, ref: str) -> bool:
    return (repo_root / strip_fragment(ref)).exists()


def validate_against_schema(
    payload: dict[str, Any],
    schema: dict[str, Any],
    payload_ref: str,
) -> list[Finding]:
    try:
        from jsonschema import Draft202012Validator, FormatChecker  # type: ignore
    except Exception:  # noqa: BLE001
        return []

    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    findings: list[Finding] = []
    for error in sorted(validator.iter_errors(payload), key=lambda err: list(err.path)):
        path = ".".join(str(part) for part in error.path) or "<root>"
        findings.append(
            Finding(
                "error",
                "rollback_plan.schema.validation",
                f"{path}: {error.message}",
                payload_ref,
                "Update the rollback plan, support projection, or schema so the checked-in records validate.",
            )
        )
    return findings


def build_support_projection(plan: dict[str, Any]) -> dict[str, Any]:
    current = ensure_dict(plan.get("current_build"), "plan.current_build")
    target = ensure_dict(plan.get("rollback_target"), "plan.rollback_target")
    downgrade = ensure_dict(plan.get("downgrade_truth"), "plan.downgrade_truth")
    support = ensure_dict(plan.get("support_projection"), "plan.support_projection")
    return {
        "record_kind": EXPECTED_SUPPORT_RECORD_KIND,
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "plan_id": plan.get("plan_id"),
        "generated_at": plan.get("generated_at"),
        "source_plan_ref": DEFAULT_PLAN_REL,
        "current_release_candidate_ref": current.get("release_candidate_ref"),
        "current_exact_build_identity_ref": current.get("exact_build_identity_ref"),
        "rollback_target_ref": target.get("release_candidate_ref"),
        "rollback_exact_build_identity_ref": target.get("exact_build_identity_ref"),
        "downgrade_eligibility_state": downgrade.get("eligibility_state"),
        "migration_caveats": downgrade.get("migration_caveats", []),
        "retained_artifacts": [
            {
                "artifact_ref": artifact.get("artifact_ref"),
                "family_class": artifact.get("family_class"),
                "exact_build_identity_ref": artifact.get("exact_build_identity_ref"),
                "retention_state": artifact.get("retention_state"),
                "verification_state": artifact.get("verification_state"),
                "rollback_atom_member": artifact.get("rollback_atom_member"),
                "support_ref": artifact.get("support_ref"),
                "caveat": artifact.get("caveat"),
            }
            for artifact in ensure_list(
                plan.get("retained_prior_artifacts", []),
                "plan.retained_prior_artifacts",
            )
            if isinstance(artifact, dict)
        ],
        "schema_hooks": [
            {
                "hook_id": hook.get("hook_id"),
                "state_root_ref": hook.get("state_root_ref"),
                "compatibility_class": hook.get("compatibility_class"),
                "reviewed_flow_class": hook.get("reviewed_flow_class"),
                "invoked_checkpoint_id": hook.get("invoked_checkpoint_id"),
                "hook_state": hook.get("hook_state"),
                "caveat": hook.get("caveat"),
            }
            for hook in ensure_list(
                plan.get("schema_rollback_hooks", []),
                "plan.schema_rollback_hooks",
            )
            if isinstance(hook, dict)
        ],
        "support_bundle_refs": support.get("support_bundle_refs", []),
        "vocabulary_terms": support.get("vocabulary_terms", []),
        "redaction_class": support.get("redaction_class"),
    }


class Validator:
    def __init__(
        self,
        repo_root: Path,
        plan: dict[str, Any],
        support_projection: dict[str, Any],
        artifact_graph: dict[str, Any],
        ring_rollout: dict[str, Any],
        install_diagnostics: dict[str, Any],
        plan_rel: str,
        support_projection_rel: str,
        artifact_graph_rel: str,
        ring_rollout_rel: str,
        install_diagnostics_rel: str,
    ) -> None:
        self.repo_root = repo_root
        self.plan = plan
        self.support_projection = support_projection
        self.artifact_graph = artifact_graph
        self.ring_rollout = ring_rollout
        self.install_diagnostics = install_diagnostics
        self.plan_rel = plan_rel
        self.support_projection_rel = support_projection_rel
        self.artifact_graph_rel = artifact_graph_rel
        self.ring_rollout_rel = ring_rollout_rel
        self.install_diagnostics_rel = install_diagnostics_rel
        self.findings: list[Finding] = []
        self.install_state_roots = self.collect_install_state_roots()
        self.graph_exact_build_refs = {
            row.get("exact_build_identity_ref")
            for row in ensure_list(
                artifact_graph.get("exact_build_identities", []),
                "artifact_graph.exact_build_identities",
            )
            if isinstance(row, dict)
        }

    def collect_install_state_roots(self) -> set[str]:
        roots: set[str] = set()
        for row in ensure_list(self.install_diagnostics.get("rows", []), "install_diagnostics.rows"):
            if not isinstance(row, dict):
                continue
            for root in ensure_list(row.get("durable_state_roots", []), "row.durable_state_roots"):
                if isinstance(root, dict) and isinstance(root.get("state_root_ref"), str):
                    roots.add(root["state_root_ref"])
        return roots

    def push(
        self,
        check_id: str,
        message: str,
        ref: str,
        remediation: str,
        severity: str = "error",
    ) -> None:
        self.findings.append(Finding(severity, check_id, message, ref, remediation))

    def validate(self, *, include_surfaces: bool = True) -> list[Finding]:
        self.validate_header()
        self.validate_source_refs()
        self.validate_build_links()
        self.validate_retained_artifacts()
        self.validate_schema_hooks()
        self.validate_downgrade_truth()
        self.validate_support_contract()
        if include_surfaces:
            self.validate_consuming_surfaces()
        return self.findings

    def validate_header(self) -> None:
        if self.plan.get("record_kind") != EXPECTED_PLAN_RECORD_KIND:
            self.push(
                "rollback_plan.record_kind",
                f"record_kind must be {EXPECTED_PLAN_RECORD_KIND}",
                str(self.plan.get("plan_id", "<plan>")),
                "Use the update rollback plan record discriminator.",
            )
        if self.plan.get("schema_version") != EXPECTED_SCHEMA_VERSION:
            self.push(
                "rollback_plan.schema_version",
                "schema_version must be 1",
                str(self.plan.get("plan_id", "<plan>")),
                "Keep the plan on schema version 1 until a governed migration exists.",
            )

    def validate_source_refs(self) -> None:
        refs = ensure_dict(self.plan.get("source_refs", {}), "plan.source_refs")
        expected_paths = {
            "artifact_graph_ref": self.artifact_graph_rel,
            "ring_rollout_ref": self.ring_rollout_rel,
            "install_diagnostics_ref": self.install_diagnostics_rel,
        }
        for field, expected in expected_paths.items():
            if refs.get(field) != expected:
                self.push(
                    "rollback_plan.source_ref.mismatch",
                    f"{field} must match the validated beta artifact",
                    f"source_refs.{field}",
                    f"Set {field} to {expected}.",
                )
        for field, ref in refs.items():
            if not isinstance(ref, str) or not ref:
                self.push(
                    "rollback_plan.source_ref.empty",
                    "source refs must be non-empty strings",
                    f"source_refs.{field}",
                    "Populate the source ref with a repo-relative path or stable opaque ref.",
                )
                continue
            if is_repo_ref(ref) and not repo_ref_exists(self.repo_root, ref):
                self.push(
                    "rollback_plan.source_ref.missing_on_disk",
                    "repo-relative source ref does not exist",
                    ref,
                    "Add the source artifact or correct the rollback plan ref.",
                )

    def validate_build_links(self) -> None:
        current = ensure_dict(self.plan.get("current_build", {}), "plan.current_build")
        target = ensure_dict(self.plan.get("rollback_target", {}), "plan.rollback_target")
        graph_candidate = ensure_dict(self.artifact_graph.get("candidate", {}), "artifact_graph.candidate")
        current_release = current.get("release_candidate_ref")
        target_release = target.get("release_candidate_ref")
        current_exact = current.get("exact_build_identity_ref")
        target_exact = target.get("exact_build_identity_ref")

        if current_release != graph_candidate.get("candidate_ref"):
            self.push(
                "rollback_plan.current_candidate.not_in_artifact_graph",
                "current build must match the artifact graph candidate",
                str(current_release),
                "Refresh current_build.release_candidate_ref from the artifact graph candidate.",
            )
        if target_release != graph_candidate.get("rollback_target_ref"):
            self.push(
                "rollback_plan.target_candidate.not_in_artifact_graph",
                "rollback target must match the artifact graph rollback target",
                str(target_release),
                "Refresh rollback_target.release_candidate_ref from the artifact graph rollback target.",
            )
        if current_release != self.ring_rollout.get("release_candidate_ref"):
            self.push(
                "rollback_plan.current_candidate.not_in_ring_rollout",
                "current build must match the ring rollout release candidate",
                str(current_release),
                "Keep the rollback plan and rollout packet on the same beta candidate.",
            )
        if target_release != self.ring_rollout.get("rollback_target_ref"):
            self.push(
                "rollback_plan.target_candidate.not_in_ring_rollout",
                "rollback target must match the ring rollout rollback target",
                str(target_release),
                "Keep rollback target refs aligned between rollout and rollback packets.",
            )
        if current_exact not in self.graph_exact_build_refs:
            self.push(
                "rollback_plan.current_exact.not_in_artifact_graph",
                "current exact-build ref is absent from the artifact graph",
                str(current_exact),
                "Add the exact-build identity to the graph or correct current_build.",
            )
        if current_exact not in set(self.ring_rollout.get("exact_build_identity_refs", [])):
            self.push(
                "rollback_plan.current_exact.not_in_ring_rollout",
                "current exact-build ref is absent from ring rollout",
                str(current_exact),
                "Refresh ring rollout exact-build refs or correct current_build.",
            )
        if current_exact not in set(self.install_diagnostics.get("exact_build_identity_refs", [])):
            self.push(
                "rollback_plan.current_exact.not_in_install_diagnostics",
                "current exact-build ref is absent from install diagnostics",
                str(current_exact),
                "Refresh install diagnostics so update rollback evidence shares exact-build truth.",
            )
        for label, exact in [
            ("current_build.exact_build_identity_ref", current_exact),
            ("rollback_target.exact_build_identity_ref", target_exact),
        ]:
            if not isinstance(exact, str) or not exact.startswith("build-id:aureline:"):
                self.push(
                    "rollback_plan.exact_build.invalid",
                    "exact-build refs must use the build-id:aureline namespace",
                    label,
                    "Use a stable exact-build identity ref.",
                )
        if current_exact == target_exact:
            self.push(
                "rollback_plan.same_current_and_target_exact_build",
                "current and rollback target exact-build refs must differ",
                str(current_exact),
                "Point rollback_target at the retained prior build, not the current build.",
            )

    def validate_retained_artifacts(self) -> None:
        target = ensure_dict(self.plan.get("rollback_target", {}), "plan.rollback_target")
        target_exact = target.get("exact_build_identity_ref")
        target_release = target.get("release_candidate_ref")
        artifacts = ensure_list(
            self.plan.get("retained_prior_artifacts", []),
            "plan.retained_prior_artifacts",
        )
        if not artifacts:
            self.push(
                "retained_artifacts.empty",
                "rollback plan must retain prior artifacts",
                str(self.plan.get("plan_id", "<plan>")),
                "Add retained prior artifact rows for the coordinated rollback atom.",
            )
            return

        seen: set[str] = set()
        families: set[str] = set()
        for artifact in artifacts:
            if not isinstance(artifact, dict):
                self.push(
                    "retained_artifacts.row_shape",
                    "retained artifact rows must be objects",
                    str(artifact),
                    "Replace the row with a retained prior artifact object.",
                )
                continue
            artifact_ref = str(artifact.get("artifact_ref", ""))
            if artifact_ref in seen:
                self.push(
                    "retained_artifacts.duplicate_artifact_ref",
                    "retained artifact refs must be unique",
                    artifact_ref,
                    "Give each retained artifact one stable ref.",
                )
            seen.add(artifact_ref)
            families.add(str(artifact.get("family_class", "")))
            if artifact.get("exact_build_identity_ref") != target_exact:
                self.push(
                    "retained_artifacts.exact_build_mismatch",
                    "retained artifacts must use the rollback target exact-build ref",
                    artifact_ref,
                    "Set the retained artifact exact-build ref to the rollback target exact-build ref.",
                )
            if artifact.get("prior_release_candidate_ref") != target_release:
                self.push(
                    "retained_artifacts.release_candidate_mismatch",
                    "retained artifacts must belong to the rollback target release candidate",
                    artifact_ref,
                    "Set prior_release_candidate_ref to the rollback target candidate.",
                )
            if artifact.get("retention_state") != REQUIRED_RETENTION_STATE:
                self.push(
                    "retained_artifacts.not_exact_build_retained",
                    "retained prior artifacts must keep exact-build bytes",
                    artifact_ref,
                    "Use retained_exact_build only after the artifact bytes are retained and addressable.",
                )
            if artifact.get("verification_state") != REQUIRED_VERIFICATION_STATE:
                self.push(
                    "retained_artifacts.not_verified",
                    "retained prior artifacts must have verified trust state",
                    artifact_ref,
                    "Verify the digest/signature state before admitting the rollback atom.",
                )
            if artifact.get("rollback_atom_member") is not True:
                self.push(
                    "retained_artifacts.not_in_rollback_atom",
                    "retained prior artifact must be part of the coordinated rollback atom",
                    artifact_ref,
                    "Set rollback_atom_member only when the artifact moves with the rollback set.",
                )
            support_ref = artifact.get("support_ref")
            if not isinstance(support_ref, str) or not support_ref.startswith(
                self.support_projection_rel + "#"
            ):
                self.push(
                    "retained_artifacts.support_ref_mismatch",
                    "retained artifact support refs must point into the rollback support projection",
                    artifact_ref,
                    "Point support_ref at the generated rollback support projection.",
                )

        missing = REQUIRED_RETAINED_FAMILIES - families
        if missing:
            self.push(
                "retained_artifacts.required_family_missing",
                "rollback plan is missing required retained artifact families",
                str(self.plan.get("plan_id", "<plan>")),
                f"Add retained artifact rows for: {', '.join(sorted(missing))}.",
            )

    def validate_schema_hooks(self) -> None:
        hooks = ensure_list(
            self.plan.get("schema_rollback_hooks", []),
            "plan.schema_rollback_hooks",
        )
        if not hooks:
            self.push(
                "schema_hooks.empty",
                "rollback plan must declare schema rollback hooks",
                str(self.plan.get("plan_id", "<plan>")),
                "Add hook rows for every affected schema/state root.",
            )
            return

        seen: set[str] = set()
        for hook in hooks:
            if not isinstance(hook, dict):
                self.push(
                    "schema_hooks.row_shape",
                    "schema hook rows must be objects",
                    str(hook),
                    "Replace the row with a schema rollback hook object.",
                )
                continue
            hook_id = str(hook.get("hook_id", ""))
            if hook_id in seen:
                self.push(
                    "schema_hooks.duplicate_hook_id",
                    "schema hook ids must be unique",
                    hook_id,
                    "Give each schema rollback hook one stable id.",
                )
            seen.add(hook_id)
            checkpoint = hook.get("invoked_checkpoint_id")
            if not isinstance(checkpoint, str) or not checkpoint.startswith("checkpoint.update."):
                self.push(
                    "schema_hooks.invoked_checkpoint_not_update_sequence",
                    "schema rollback hooks must bind to reviewed update sequence checkpoints",
                    hook_id,
                    "Use a checkpoint.update.* id admitted by the rollback review flow.",
                )
            if not hook.get("reviewed_flow_ref"):
                self.push(
                    "schema_hooks.reviewed_flow_missing",
                    "schema rollback hooks must carry a reviewed flow ref",
                    hook_id,
                    "Add the review ref that admitted this hook.",
                )
            if hook.get("state_root_ref") not in self.install_state_roots:
                self.push(
                    "schema_hooks.state_root_not_in_install_diagnostics",
                    "schema hook state root is absent from install diagnostics",
                    hook_id,
                    "Use a durable state-root ref from install diagnostics.",
                )
            if hook.get("hook_state") == "blocked":
                self.push(
                    "schema_hooks.blocked",
                    "blocked schema hooks cannot be part of an admitted rollback plan",
                    hook_id,
                    "Remove the hook or downgrade the rollback truth to blocked.",
                )
            compatibility = hook.get("compatibility_class")
            if compatibility == "destructive_blocked":
                self.push(
                    "schema_hooks.destructive_compatibility_blocked",
                    "destructive schema rollback compatibility is not admitted",
                    hook_id,
                    "Replace the rollback path with repair/export review or mark downgrade blocked.",
                )
            if (
                compatibility not in AUTOMATIC_SCHEMA_COMPATIBILITY
                and not hook.get("repair_transaction_ref")
                and ensure_dict(self.plan.get("downgrade_truth", {}), "plan.downgrade_truth").get(
                    "eligibility_state"
                )
                == "auto_eligible"
            ):
                self.push(
                    "schema_hooks.repair_ref_missing",
                    "non-automatic schema rollback must carry repair evidence or require review",
                    hook_id,
                    "Add repair_transaction_ref or change downgrade truth to a reviewed state.",
                )

    def validate_downgrade_truth(self) -> None:
        current = ensure_dict(self.plan.get("current_build", {}), "plan.current_build")
        target = ensure_dict(self.plan.get("rollback_target", {}), "plan.rollback_target")
        truth = ensure_dict(self.plan.get("downgrade_truth", {}), "plan.downgrade_truth")
        plan_id = str(self.plan.get("plan_id", "<plan>"))
        if truth.get("source_build_ref") != current.get("release_candidate_ref"):
            self.push(
                "downgrade_truth.source_build_ref_mismatch",
                "downgrade source build must match current release candidate",
                plan_id,
                "Set downgrade_truth.source_build_ref to current_build.release_candidate_ref.",
            )
        if truth.get("target_build_ref") != target.get("release_candidate_ref"):
            self.push(
                "downgrade_truth.target_build_ref_mismatch",
                "downgrade target build must match rollback target release candidate",
                plan_id,
                "Set downgrade_truth.target_build_ref to rollback_target.release_candidate_ref.",
            )
        eligibility = truth.get("eligibility_state")
        if eligibility not in ADMITTED_DOWNGRADE_STATES:
            self.push(
                "downgrade_truth.not_admitted",
                "blocked or unsupported downgrade truth cannot back a beta rollback guarantee",
                plan_id,
                "Narrow the claim or restore the evidence required for an admitted rollback state.",
            )
        if not truth.get("migration_caveats"):
            self.push(
                "downgrade_truth.caveats_missing",
                "downgrade truth must expose migration caveats",
                plan_id,
                "Add explicit caveats for user/admin/support surfaces.",
            )
        if eligibility in {"eligible_with_review", "manual_review_required"} and not truth.get(
            "manual_review_reason_classes"
        ):
            self.push(
                "downgrade_truth.review_reasons_missing",
                "review-gated downgrade truth must name review reason classes",
                plan_id,
                "Add manual review reason classes so support and UI explain the gate.",
            )
        for field in ["preserved_state_root_refs", "not_restored_state_root_refs"]:
            for root_ref in ensure_list(truth.get(field, []), f"downgrade_truth.{field}"):
                if root_ref not in self.install_state_roots:
                    self.push(
                        "downgrade_truth.state_root_not_in_install_diagnostics",
                        "downgrade truth names a state root absent from install diagnostics",
                        str(root_ref),
                        "Use state-root refs from install diagnostics.",
                    )

    def validate_support_contract(self) -> None:
        support = ensure_dict(self.plan.get("support_projection", {}), "plan.support_projection")
        if support.get("support_projection_ref") != self.support_projection_rel:
            self.push(
                "support_projection.output_ref_mismatch",
                "plan support projection ref must match the generated projection path",
                str(support.get("support_projection_ref")),
                f"Set support_projection_ref to {self.support_projection_rel}.",
            )
        if not support.get("support_bundle_refs"):
            self.push(
                "support_projection.support_bundle_refs_missing",
                "rollback plan must project into at least one support bundle ref",
                str(self.plan.get("plan_id", "<plan>")),
                "Add the support bundle ref that quotes this rollback plan.",
            )
        terms = set(support.get("vocabulary_terms", []))
        missing = REQUIRED_VOCABULARY_TERMS - terms
        if missing:
            self.push(
                "support_projection.required_vocabulary_missing",
                "support projection is missing required rollback vocabulary",
                str(self.plan.get("plan_id", "<plan>")),
                f"Add vocabulary terms: {', '.join(sorted(missing))}.",
            )
        expected = build_support_projection(self.plan)
        if self.support_projection != expected:
            self.push(
                "support_projection.generated_payload_stale",
                "checked-in support projection does not match the rollback plan",
                self.support_projection_rel,
                "Run the update rollback validator without --check and commit the generated projection.",
            )

    def validate_consuming_surfaces(self) -> None:
        support = ensure_dict(self.plan.get("support_projection", {}), "plan.support_projection")
        current = ensure_dict(self.plan.get("current_build", {}), "plan.current_build")
        target = ensure_dict(self.plan.get("rollback_target", {}), "plan.rollback_target")
        required_tokens = [
            current.get("exact_build_identity_ref"),
            target.get("exact_build_identity_ref"),
            target.get("release_candidate_ref"),
            *support.get("vocabulary_terms", []),
        ]
        for surface_ref in ensure_list(
            support.get("consuming_surface_refs", []),
            "support_projection.consuming_surface_refs",
        ):
            if not isinstance(surface_ref, str) or not surface_ref:
                self.push(
                    "support_surface.ref_empty",
                    "support surface refs must be non-empty strings",
                    str(surface_ref),
                    "Populate consuming_surface_refs with repo-relative paths.",
                )
                continue
            surface_path = self.repo_root / strip_fragment(surface_ref)
            if not surface_path.exists():
                self.push(
                    "support_surface.missing_on_disk",
                    "declared consuming surface does not exist",
                    surface_ref,
                    "Add the docs/help/support artifact or remove the consuming surface ref.",
                )
                continue
            if surface_path.suffix not in {".md", ".json"}:
                continue
            text = surface_path.read_text(encoding="utf-8")
            for token in required_tokens:
                if isinstance(token, str) and token and token not in text:
                    self.push(
                        "support_surface.rollback_truth_token_missing",
                        "consuming surface does not quote the shared rollback build id or vocabulary",
                        surface_ref,
                        f"Quote {token} from the rollback plan or generated support projection.",
                    )


def set_payload_path(payload: Any, path: str, value: Any) -> bool:
    segments = [segment.strip() for segment in path.split(".") if segment.strip()]
    if not segments:
        return False
    cursor = payload
    for segment in segments[:-1]:
        if isinstance(cursor, list):
            try:
                index = int(segment)
            except ValueError:
                return False
            if index < 0 or index >= len(cursor):
                return False
            cursor = cursor[index]
        elif isinstance(cursor, dict) and segment in cursor:
            cursor = cursor[segment]
        else:
            return False
    last = segments[-1]
    if isinstance(cursor, list):
        try:
            index = int(last)
        except ValueError:
            return False
        if index < 0 or index >= len(cursor):
            return False
        cursor[index] = value
        return True
    if isinstance(cursor, dict) and last in cursor:
        cursor[last] = value
        return True
    return False


def validate_fixture_manifest(
    *,
    repo_root: Path,
    manifest_rel: str,
    plan: dict[str, Any],
    artifact_graph: dict[str, Any],
    ring_rollout: dict[str, Any],
    install_diagnostics: dict[str, Any],
    plan_rel: str,
    support_projection_rel: str,
    schema_rel: str,
    artifact_graph_rel: str,
    ring_rollout_rel: str,
    install_diagnostics_rel: str,
) -> tuple[list[dict[str, Any]], list[Finding]]:
    manifest = ensure_dict(render_yaml_as_json(repo_root / manifest_rel), "fixture_manifest")
    findings: list[Finding] = []
    results: list[dict[str, Any]] = []
    if manifest.get("record_kind") != "update_rollback_fixture_manifest":
        findings.append(
            Finding(
                "error",
                "update_rollback.fixture_manifest.record_kind",
                "fixture manifest must use update_rollback_fixture_manifest",
                str(manifest.get("record_kind", "<fixture-manifest>")),
                "Use the rollback fixture manifest discriminator.",
            )
        )
    expected_refs = {
        "canonical_plan_ref": plan_rel,
        "canonical_schema_ref": schema_rel,
        "canonical_support_projection_ref": support_projection_rel,
        "validator_ref": "tools/ci/m3/update_rollback",
    }
    for field, expected in expected_refs.items():
        ref = manifest.get(field)
        if ref != expected:
            findings.append(
                Finding(
                    "error",
                    "update_rollback.fixture_manifest.ref_mismatch",
                    f"{field} must match the canonical rollback artifact",
                    str(ref),
                    f"Set {field} to {expected}.",
                )
            )
        if isinstance(ref, str) and is_repo_ref(ref) and not repo_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    "error",
                    "update_rollback.fixture_manifest.ref_missing_on_disk",
                    "fixture manifest ref does not exist",
                    ref,
                    "Add the referenced file or correct the fixture manifest.",
                )
            )
    mutations = ensure_list(
        manifest.get("failure_drill_mutations", []),
        "fixture_manifest.failure_drill_mutations",
    )
    if not mutations:
        findings.append(
            Finding(
                "error",
                "update_rollback.fixture_manifest.mutations_missing",
                "fixture manifest must name failure-drill mutations",
                manifest_rel,
                "Add mutation rows for expected rollback rejection classes.",
            )
        )
    for mutation in mutations:
        mutation = ensure_dict(mutation, "fixture_manifest.failure_drill_mutations[]")
        mutation_id = str(mutation.get("mutation_id", ""))
        payload_path = str(mutation.get("payload_path", ""))
        expected_finding = str(mutation.get("expected_finding", ""))
        mutated_plan = copy.deepcopy(plan)
        set_ok = set_payload_path(mutated_plan, payload_path, mutation.get("replacement"))
        case_findings: list[str] = []
        if not set_ok:
            case_findings.append("payload_path_unresolved")
        else:
            validator = Validator(
                repo_root,
                mutated_plan,
                build_support_projection(mutated_plan),
                artifact_graph,
                ring_rollout,
                install_diagnostics,
                plan_rel,
                support_projection_rel,
                artifact_graph_rel,
                ring_rollout_rel,
                install_diagnostics_rel,
            )
            mutation_findings = validator.validate(include_surfaces=False)
            check_ids = {finding.check_id for finding in mutation_findings}
            if expected_finding not in check_ids:
                case_findings.append(f"expected_finding_missing:{expected_finding}")
        status = "passed" if not case_findings else "failed"
        results.append(
            {
                "mutation_id": mutation_id,
                "target_ref": mutation.get("target_ref"),
                "expected_finding": expected_finding,
                "status": status,
                "findings": case_findings,
            }
        )
        for item in case_findings:
            findings.append(
                Finding(
                    "error",
                    "update_rollback.fixture_manifest.mutation_failed",
                    f"rollback fixture mutation failed: {item}",
                    mutation_id,
                    "Update the mutation path, expected finding, or validator rule.",
                )
            )
    return results, findings


def compare_or_write(
    path: Path,
    expected_text: str,
    check: bool,
    findings: list[Finding],
    check_id: str,
) -> None:
    if check:
        actual = path.read_text(encoding="utf-8") if path.exists() else ""
        if actual != expected_text:
            findings.append(
                Finding(
                    "error",
                    check_id,
                    "checked-in generated file is stale",
                    str(path),
                    "Run the update rollback validator without --check and commit the generated output.",
                )
            )
        return
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(expected_text, encoding="utf-8")


def build_capture(
    *,
    plan: dict[str, Any],
    findings: list[Finding],
    fixture_results: list[dict[str, Any]],
    plan_rel: str,
    schema_rel: str,
    support_projection_rel: str,
) -> dict[str, Any]:
    retained = [
        artifact
        for artifact in plan.get("retained_prior_artifacts", [])
        if isinstance(artifact, dict)
    ]
    hooks = [
        hook for hook in plan.get("schema_rollback_hooks", []) if isinstance(hook, dict)
    ]
    error_count = sum(1 for finding in findings if finding.severity == "error")
    return {
        "record_kind": EXPECTED_CAPTURE_RECORD_KIND,
        "schema_version": EXPECTED_SCHEMA_VERSION,
        "validated_at": plan.get("generated_at"),
        "plan_id": plan.get("plan_id"),
        "source_plan_ref": plan_rel,
        "schema_ref": schema_rel,
        "support_projection_ref": support_projection_rel,
        "passed": error_count == 0,
        "coverage": {
            "current_release_candidate_ref": plan.get("current_build", {}).get(
                "release_candidate_ref"
            ),
            "current_exact_build_identity_ref": plan.get("current_build", {}).get(
                "exact_build_identity_ref"
            ),
            "rollback_target_ref": plan.get("rollback_target", {}).get(
                "release_candidate_ref"
            ),
            "rollback_exact_build_identity_ref": plan.get("rollback_target", {}).get(
                "exact_build_identity_ref"
            ),
            "retained_artifact_families": sorted(
                {artifact.get("family_class") for artifact in retained}
            ),
            "retained_artifact_count": len(retained),
            "schema_hook_count": len(hooks),
            "downgrade_eligibility_state": plan.get("downgrade_truth", {}).get(
                "eligibility_state"
            ),
            "fixture_mutation_count": len(fixture_results),
            "error_count": error_count,
        },
        "fixture_results": fixture_results,
        "findings": [finding.as_report() for finding in findings],
    }


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    plan = ensure_dict(load_json(repo_root / args.plan), "plan")
    support_projection = ensure_dict(
        load_json(repo_root / args.support_projection),
        "support_projection",
    )
    schema = ensure_dict(load_json(repo_root / args.schema), "schema")
    artifact_graph = ensure_dict(load_json(repo_root / args.artifact_graph), "artifact_graph")
    ring_rollout = ensure_dict(load_json(repo_root / args.ring_rollout), "ring_rollout")
    install_diagnostics = ensure_dict(
        load_json(repo_root / args.install_diagnostics),
        "install_diagnostics",
    )

    findings: list[Finding] = []
    findings.extend(validate_against_schema(plan, schema, args.plan))
    findings.extend(validate_against_schema(support_projection, schema, args.support_projection))

    validator = Validator(
        repo_root,
        plan,
        support_projection,
        artifact_graph,
        ring_rollout,
        install_diagnostics,
        args.plan,
        args.support_projection,
        args.artifact_graph,
        args.ring_rollout,
        args.install_diagnostics,
    )
    findings.extend(validator.validate())

    expected_support = build_support_projection(plan)
    compare_or_write(
        repo_root / args.support_projection,
        as_json_text(expected_support),
        args.check,
        findings,
        "update_rollback.generated.support_projection_stale",
    )

    fixture_results, fixture_findings = validate_fixture_manifest(
        repo_root=repo_root,
        manifest_rel=args.fixture_manifest,
        plan=plan,
        artifact_graph=artifact_graph,
        ring_rollout=ring_rollout,
        install_diagnostics=install_diagnostics,
        plan_rel=args.plan,
        support_projection_rel=args.support_projection,
        schema_rel=args.schema,
        artifact_graph_rel=args.artifact_graph,
        ring_rollout_rel=args.ring_rollout,
        install_diagnostics_rel=args.install_diagnostics,
    )
    findings.extend(fixture_findings)
    capture = build_capture(
        plan=plan,
        findings=findings,
        fixture_results=fixture_results,
        plan_rel=args.plan,
        schema_rel=args.schema,
        support_projection_rel=args.support_projection,
    )
    compare_or_write(
        repo_root / args.capture,
        as_json_text(capture),
        args.check,
        findings,
        "update_rollback.generated.capture_stale",
    )

    if any(finding.severity == "error" for finding in findings):
        for finding in findings:
            print(
                f"{finding.severity}: {finding.check_id}: {finding.message} [{finding.ref}]",
                file=sys.stderr,
            )
        return 1
    print("update rollback validation passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
