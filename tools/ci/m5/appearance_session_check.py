#!/usr/bin/env python3
"""M5 appearance-session runtime audit CI gate.

This gate enforces that the checked-in M5 appearance-session runtime audit
stays fresh and honest: one live appearance-session object records what is
active right now, every appearance change flows through one explicit checkpoint,
the preview / apply / cancel / validation-failure / revert / OS-signal
transitions are legal edges of one checkpoint-aware state machine, every change
a surface cannot apply live discloses its restart-or-reload posture, and every
claimed M5 surface (notebook, data/result surface, preview/browser pane,
docs/help pane, companion surface, extension-hosted surface) rides the shared
session instead of painting its own appearance. It reads:

- the audit fixture at ``fixtures/ux/m5/live-appearance-change/report.json``;
- the support-export fixture at
  ``fixtures/ux/m5/live-appearance-change/support_export.json``;
- the boundary schema at ``schemas/ux/appearance-session.schema.json``;
- the canonical appearance-session / checkpoint record schema at
  ``schemas/ux/appearance_checkpoint.schema.json`` (referenced by the audit);
  and
- (when present) the published markdown at
  ``artifacts/ux/m5/appearance-session-checkpoints/m5_appearance_session_runtime_audit.md``
  and the companion doc at ``docs/m5/appearance-session-runtime.md``.

For the audit the gate verifies that:

- every transition flows through exactly one checkpoint that resolves in the
  ledger;
- every transition is a legal edge of the state machine (its ``from`` state is
  a legal predecessor for the operation, and its ``to`` and apply states match
  the operation), and a validation failure auto-reverts;
- a change that requires a reload or restart discloses it through its atomicity
  class, and a surface whose capability is not ``applies_live`` discloses the
  requirement;
- every checkpoint is reversible from a single checkpoint and carries a usable
  rollback path;
- the live session is self-consistent (a live or committed preview cites a
  current checkpoint, a rolled-back session cites a rollback ref, and any
  current checkpoint ref resolves);
- every surface rides the shared session, consumes the live session ref, and
  carries a canonical appearance anchor and a non-empty accessibility note;
- the report carries no blocking finding and at least one transition
  demonstrates a live appearance change;
- the support-export wrapper quotes the report id, the session ref, every
  checkpoint ref, every transition ref, and every surface id and descriptor
  revision; and
- the published markdown audit and the companion doc are present and back-link
  the canonical schema, fixtures, and CLI gate.

Exit codes:

- ``0`` -- audit is clean (no blockers).
- ``1`` -- one or more findings.
- ``2`` -- usage error or missing input file.
"""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

REPORT_REL = Path("fixtures/ux/m5/live-appearance-change/report.json")
SUPPORT_EXPORT_REL = Path("fixtures/ux/m5/live-appearance-change/support_export.json")
SCHEMA_REL = Path("schemas/ux/appearance-session.schema.json")
CANONICAL_SCHEMA_REL = Path("schemas/ux/appearance_checkpoint.schema.json")
MARKDOWN_REL = Path(
    "artifacts/ux/m5/appearance-session-checkpoints/m5_appearance_session_runtime_audit.md"
)
DOC_REL = Path("docs/m5/appearance-session-runtime.md")

EXPECTED_RECORD_KIND_REPORT = "shell_m5_appearance_session_runtime_report_record"
EXPECTED_RECORD_KIND_SESSION = "shell_m5_appearance_session_record"
EXPECTED_RECORD_KIND_CHECKPOINT = "shell_m5_appearance_session_checkpoint_record"
EXPECTED_RECORD_KIND_TRANSITION = "shell_m5_appearance_session_transition_record"
EXPECTED_RECORD_KIND_SURFACE = "shell_m5_appearance_session_surface_binding_record"
EXPECTED_RECORD_KIND_SUPPORT = "shell_m5_appearance_session_runtime_support_export_record"
EXPECTED_SHARED_CONTRACT_REF = "shell:m5_appearance_session:v1"
EXPECTED_SCHEMA_VERSION = 1

PREVIEW_STATES = {
    "not_previewing",
    "preview_pending_validation",
    "preview_live",
    "preview_failed_reverted",
    "preview_committed",
    "rollback_applied",
}
PREVIEW_NEEDS_CHECKPOINT = {
    "preview_pending_validation",
    "preview_live",
    "preview_committed",
}
LIVE_ATOMICITY = "single_checkpoint_atomic"
LIVE_ROLLBACK_PATH = "single_checkpoint_revert"

# The legal edges of the state machine: op -> (legal from states, to state,
# apply state). Mirrors TransitionOp in
# crates/aureline-shell/src/appearance_session/mod.rs.
STATE_MACHINE: dict[str, tuple[set[str], str, str]] = {
    "open_preview": ({"not_previewing"}, "preview_pending_validation", "checkpoint_created"),
    "preflight_passed": ({"preview_pending_validation"}, "preview_live", "preview_live"),
    "commit_preview": ({"preview_live"}, "preview_committed", "committed"),
    "cancel_preview": (
        {"preview_pending_validation", "preview_live"},
        "not_previewing",
        "reverted",
    ),
    "validation_failed": (
        {"preview_pending_validation", "preview_live"},
        "preview_failed_reverted",
        "preflight_failed",
    ),
    "revert_committed": ({"preview_committed"}, "rollback_applied", "reverted"),
    "os_signal_applied": (
        {"not_previewing", "preview_committed"},
        "preview_committed",
        "committed",
    ),
}


@dataclass
class Finding:
    """One blocking finding emitted by the gate."""

    code: str
    message: str
    subject_ref: str | None = None
    detail: dict[str, Any] = field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        out: dict[str, Any] = {"code": self.code, "message": self.message}
        if self.subject_ref is not None:
            out["subject_ref"] = self.subject_ref
        if self.detail:
            out["detail"] = self.detail
        return out


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Path to the repository root (default: cwd).",
    )
    parser.add_argument(
        "--format",
        choices=("text", "json"),
        default="text",
        help="Output format for the findings report.",
    )
    return parser.parse_args()


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing required input: {path}")
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


def check_report_envelope(report: dict[str, Any], findings: list[Finding]) -> None:
    if report.get("record_kind") != EXPECTED_RECORD_KIND_REPORT:
        findings.append(
            Finding(
                "report_record_kind_mismatch",
                f"report.record_kind must be {EXPECTED_RECORD_KIND_REPORT}",
                detail={"record_kind": report.get("record_kind")},
            )
        )
    if report.get("schema_version") != EXPECTED_SCHEMA_VERSION:
        findings.append(
            Finding(
                "report_schema_version_mismatch",
                f"report.schema_version must be {EXPECTED_SCHEMA_VERSION}",
                detail={"schema_version": report.get("schema_version")},
            )
        )
    if report.get("shared_contract_ref") != EXPECTED_SHARED_CONTRACT_REF:
        findings.append(
            Finding(
                "report_shared_contract_ref_mismatch",
                f"report.shared_contract_ref must be {EXPECTED_SHARED_CONTRACT_REF}",
                detail={"shared_contract_ref": report.get("shared_contract_ref")},
            )
        )
    for ref_field in ("published_report_ref", "published_doc_ref"):
        ref = report.get(ref_field)
        if not isinstance(ref, str) or not ref.strip():
            findings.append(
                Finding(
                    "publication_ref_missing",
                    f"report.{ref_field} must be a non-empty string",
                    detail={ref_field: ref},
                )
            )
    if report.get("report_clean") is not True:
        findings.append(
            Finding(
                "report_not_clean",
                "report.report_clean must be true",
                detail={"report_clean": report.get("report_clean")},
            )
        )
    if report.get("live_change_demonstrated") is not True:
        findings.append(
            Finding(
                "no_live_change_demonstrated",
                "report.live_change_demonstrated must be true",
            )
        )
    declared = ensure_list(report.get("blocking_findings", []), "report.blocking_findings")
    for blocker in declared:
        blocker = ensure_dict(blocker, "blocking_finding")
        findings.append(
            Finding(
                "declared_blocking_finding",
                "report carries a declared blocking finding",
                detail={"class": blocker.get("class")},
            )
        )
    if not ensure_list(report.get("checkpoints", []), "report.checkpoints"):
        findings.append(Finding("no_checkpoints", "report.checkpoints must be non-empty"))
    if not ensure_list(report.get("transitions", []), "report.transitions"):
        findings.append(Finding("no_transitions", "report.transitions must be non-empty"))
    if not ensure_list(report.get("surfaces", []), "report.surfaces"):
        findings.append(Finding("no_surfaces", "report.surfaces must be non-empty"))


def checkpoint_index(report: dict[str, Any]) -> dict[str, dict[str, Any]]:
    out: dict[str, dict[str, Any]] = {}
    for checkpoint in ensure_list(report.get("checkpoints", []), "report.checkpoints"):
        checkpoint = ensure_dict(checkpoint, "checkpoint")
        checkpoint_ref = checkpoint.get("checkpoint_ref")
        if isinstance(checkpoint_ref, str):
            out[checkpoint_ref] = checkpoint
    return out


def check_session(
    session: dict[str, Any], checkpoints: dict[str, dict[str, Any]], findings: list[Finding]
) -> None:
    session_ref = session.get("session_ref")
    if session.get("record_kind") != EXPECTED_RECORD_KIND_SESSION:
        findings.append(
            Finding(
                "session_record_kind_mismatch",
                f"session.record_kind must be {EXPECTED_RECORD_KIND_SESSION}",
                subject_ref=session_ref,
            )
        )
    preview_state = session.get("preview_state")
    current = session.get("current_checkpoint_ref")
    if preview_state in PREVIEW_NEEDS_CHECKPOINT and not current:
        findings.append(
            Finding(
                "session_preview_without_checkpoint",
                "a live or committed preview must cite a current checkpoint",
                subject_ref=session_ref,
                detail={"preview_state": preview_state},
            )
        )
    if preview_state == "rollback_applied" and not session.get("rollback_ref"):
        findings.append(
            Finding(
                "session_rollback_without_ref",
                "a rolled-back session must cite a rollback ref",
                subject_ref=session_ref,
            )
        )
    if isinstance(current, str) and current and current not in checkpoints:
        findings.append(
            Finding(
                "session_unknown_current_checkpoint",
                "session current_checkpoint_ref does not resolve in the ledger",
                subject_ref=session_ref,
                detail={"checkpoint_ref": current},
            )
        )


def check_checkpoint(checkpoint: dict[str, Any], findings: list[Finding]) -> None:
    checkpoint_ref = checkpoint.get("checkpoint_ref")
    if checkpoint.get("record_kind") != EXPECTED_RECORD_KIND_CHECKPOINT:
        findings.append(
            Finding(
                "checkpoint_record_kind_mismatch",
                f"checkpoint.record_kind must be {EXPECTED_RECORD_KIND_CHECKPOINT}",
                subject_ref=checkpoint_ref,
            )
        )
    if checkpoint.get("reversible_from_single_checkpoint") is not True:
        findings.append(
            Finding(
                "checkpoint_non_reversible",
                "checkpoint must be reversible from a single checkpoint",
                subject_ref=checkpoint_ref,
            )
        )
    rollback = ensure_dict(checkpoint.get("rollback_path", {}), "checkpoint.rollback_path")
    if (
        not isinstance(rollback.get("rollback_ref"), str)
        or not rollback.get("rollback_ref", "").strip()
        or not isinstance(rollback.get("user_visible_action_id"), str)
        or not rollback.get("user_visible_action_id", "").strip()
        or not ensure_list(rollback.get("restores_axes", []), "rollback.restores_axes")
    ):
        findings.append(
            Finding(
                "checkpoint_missing_rollback_path",
                "checkpoint must carry a usable rollback path",
                subject_ref=checkpoint_ref,
            )
        )
    atomicity = checkpoint.get("atomicity_class")
    if atomicity != LIVE_ATOMICITY and rollback.get("rollback_path_class") == LIVE_ROLLBACK_PATH:
        findings.append(
            Finding(
                "checkpoint_restart_reload_undisclosed",
                "a reload/restart checkpoint must not hide the requirement in its rollback path",
                subject_ref=checkpoint_ref,
                detail={"atomicity_class": atomicity},
            )
        )


def check_transition(
    transition: dict[str, Any], checkpoints: dict[str, dict[str, Any]], findings: list[Finding]
) -> None:
    transition_ref = transition.get("transition_ref")
    if transition.get("record_kind") != EXPECTED_RECORD_KIND_TRANSITION:
        findings.append(
            Finding(
                "transition_record_kind_mismatch",
                f"transition.record_kind must be {EXPECTED_RECORD_KIND_TRANSITION}",
                subject_ref=transition_ref,
            )
        )
    checkpoint_ref = transition.get("checkpoint_ref")
    if not isinstance(checkpoint_ref, str) or not checkpoint_ref.strip():
        findings.append(
            Finding(
                "transition_without_checkpoint",
                "transition must flow through one checkpoint",
                subject_ref=transition_ref,
            )
        )
    elif checkpoint_ref not in checkpoints:
        findings.append(
            Finding(
                "transition_unknown_checkpoint",
                "transition names a checkpoint not in the ledger",
                subject_ref=transition_ref,
                detail={"checkpoint_ref": checkpoint_ref},
            )
        )

    op = transition.get("op")
    edge = STATE_MACHINE.get(op)
    if edge is None:
        findings.append(
            Finding(
                "transition_unknown_op",
                "transition op is not a known state-machine edge",
                subject_ref=transition_ref,
                detail={"op": op},
            )
        )
    else:
        legal_from, to_state, apply_state = edge
        if (
            transition.get("from_preview_state") not in legal_from
            or transition.get("to_preview_state") != to_state
            or transition.get("resulting_apply_state") != apply_state
        ):
            findings.append(
                Finding(
                    "transition_illegal_state",
                    "transition is not a legal edge of the state machine",
                    subject_ref=transition_ref,
                    detail={
                        "op": op,
                        "from": transition.get("from_preview_state"),
                        "to": transition.get("to_preview_state"),
                        "apply": transition.get("resulting_apply_state"),
                    },
                )
            )
        if op == "validation_failed" and transition.get("to_preview_state") != "preview_failed_reverted":
            findings.append(
                Finding(
                    "validation_failure_not_reverted",
                    "a validation failure must auto-revert",
                    subject_ref=transition_ref,
                )
            )

    if transition.get("reversible_from_single_checkpoint") is not True:
        findings.append(
            Finding(
                "transition_non_reversible",
                "transition must be reversible from a single checkpoint",
                subject_ref=transition_ref,
            )
        )

    atomicity = transition.get("atomicity_class")
    requires = transition.get("requires_restart_or_reload") is True
    if requires and atomicity == LIVE_ATOMICITY:
        findings.append(
            Finding(
                "transition_restart_reload_undisclosed",
                "a reload/restart transition must not claim a live atomicity class",
                subject_ref=transition_ref,
            )
        )
    if not requires and atomicity != LIVE_ATOMICITY:
        findings.append(
            Finding(
                "transition_atomicity_mismatch",
                "a live transition must use the single-checkpoint atomicity class",
                subject_ref=transition_ref,
                detail={"atomicity_class": atomicity},
            )
        )


def check_surface(
    surface: dict[str, Any],
    session_ref: str | None,
    checkpoints: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> None:
    surface_id = surface.get("surface_id")
    if not isinstance(surface_id, str) or not surface_id.strip():
        findings.append(Finding("missing_surface_id", "surface.surface_id must be non-empty"))
        return
    if surface.get("record_kind") != EXPECTED_RECORD_KIND_SURFACE:
        findings.append(
            Finding(
                "surface_record_kind_mismatch",
                f"surface.record_kind must be {EXPECTED_RECORD_KIND_SURFACE}",
                subject_ref=surface_id,
            )
        )
    if surface.get("registered_on_session") is not True:
        findings.append(
            Finding(
                "surface_not_on_session",
                "surface.registered_on_session must be true",
                subject_ref=surface_id,
            )
        )
    anchor = surface.get("appearance_anchor_ref")
    if not isinstance(anchor, str) or not anchor.strip():
        findings.append(
            Finding(
                "surface_missing_appearance_anchor",
                "surface.appearance_anchor_ref must be non-empty",
                subject_ref=surface_id,
            )
        )
    note = surface.get("accessibility_note")
    if not isinstance(note, str) or not note.strip():
        findings.append(
            Finding(
                "surface_missing_accessibility_note",
                "surface.accessibility_note must be non-empty",
                subject_ref=surface_id,
            )
        )
    consumes = surface.get("consumes_session_ref")
    if consumes != session_ref:
        findings.append(
            Finding(
                "surface_session_ref_mismatch",
                "surface must consume the live session ref",
                subject_ref=surface_id,
                detail={"consumes_session_ref": consumes},
            )
        )
    capability = surface.get("live_apply_capability")
    if capability != "applies_live" and surface.get("restart_or_reload_disclosed") is not True:
        findings.append(
            Finding(
                "surface_restart_reload_undisclosed",
                "a surface that cannot apply live must disclose its restart/reload requirement",
                subject_ref=surface_id,
                detail={"live_apply_capability": capability},
            )
        )
    last = surface.get("last_observed_checkpoint_ref")
    if isinstance(last, str) and last.strip() and last not in checkpoints:
        findings.append(
            Finding(
                "surface_unknown_checkpoint",
                "surface last_observed_checkpoint_ref does not resolve in the ledger",
                subject_ref=surface_id,
                detail={"checkpoint_ref": last},
            )
        )


def check_support_export(
    report: dict[str, Any], export: dict[str, Any], findings: list[Finding]
) -> None:
    if export.get("record_kind") != EXPECTED_RECORD_KIND_SUPPORT:
        findings.append(
            Finding(
                "support_record_kind_mismatch",
                f"support_export.record_kind must be {EXPECTED_RECORD_KIND_SUPPORT}",
                detail={"record_kind": export.get("record_kind")},
            )
        )
    case_ids = export.get("case_ids")
    if not isinstance(case_ids, list):
        findings.append(
            Finding("support_case_ids_missing", "support_export.case_ids must be an array")
        )
        return
    case_set = set(case_ids)
    if report.get("report_id") not in case_set:
        findings.append(
            Finding(
                "support_missing_report_id",
                "support_export.case_ids must quote the report id",
                detail={"report_id": report.get("report_id")},
            )
        )
    session_ref = ensure_dict(report.get("session", {}), "report.session").get("session_ref")
    if session_ref not in case_set:
        findings.append(
            Finding(
                "support_missing_session_ref",
                "support_export.case_ids must quote the session ref",
                detail={"session_ref": session_ref},
            )
        )
    for checkpoint in ensure_list(report.get("checkpoints", []), "report.checkpoints"):
        ref = ensure_dict(checkpoint, "checkpoint").get("checkpoint_ref")
        if ref not in case_set:
            findings.append(
                Finding(
                    "support_missing_checkpoint_ref",
                    "support_export.case_ids must quote every checkpoint ref",
                    detail={"checkpoint_ref": ref},
                )
            )
    for transition in ensure_list(report.get("transitions", []), "report.transitions"):
        ref = ensure_dict(transition, "transition").get("transition_ref")
        if ref not in case_set:
            findings.append(
                Finding(
                    "support_missing_transition_ref",
                    "support_export.case_ids must quote every transition ref",
                    detail={"transition_ref": ref},
                )
            )
    for surface in ensure_list(report.get("surfaces", []), "report.surfaces"):
        surface = ensure_dict(surface, "surface")
        surface_id = surface.get("surface_id")
        revision = surface.get("descriptor_revision_ref")
        if surface_id not in case_set:
            findings.append(
                Finding(
                    "support_missing_surface_id",
                    "support_export.case_ids must quote every surface id",
                    subject_ref=surface_id,
                )
            )
        if revision not in case_set:
            findings.append(
                Finding(
                    "support_missing_descriptor_revision",
                    "support_export.case_ids must quote every descriptor revision",
                    subject_ref=surface_id,
                    detail={"descriptor_revision_ref": revision},
                )
            )


DOC_BACKLINKS = (
    "artifacts/ux/m5/appearance-session-checkpoints/m5_appearance_session_runtime_audit.md",
    "fixtures/ux/m5/live-appearance-change/report.json",
    "schemas/ux/appearance-session.schema.json",
    "schemas/ux/appearance_checkpoint.schema.json",
    "tools/ci/m5/appearance_session_check.py",
)


def check_publications(repo_root: Path, findings: list[Finding]) -> None:
    markdown = repo_root / MARKDOWN_REL
    if not markdown.exists():
        findings.append(
            Finding("published_markdown_missing", f"missing published markdown: {MARKDOWN_REL}")
        )
    doc = repo_root / DOC_REL
    if not doc.exists():
        findings.append(Finding("published_doc_missing", f"missing companion doc: {DOC_REL}"))
        return
    body = doc.read_text(encoding="utf-8")
    for backlink in DOC_BACKLINKS:
        if backlink not in body:
            findings.append(
                Finding(
                    "doc_missing_backlink",
                    "companion doc must back-link the canonical artifacts and gate",
                    detail={"backlink": backlink},
                )
            )


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    report = ensure_dict(load_json(repo_root / REPORT_REL), "report")
    export = ensure_dict(load_json(repo_root / SUPPORT_EXPORT_REL), "support_export")
    for schema_rel in (SCHEMA_REL, CANONICAL_SCHEMA_REL):
        if not (repo_root / schema_rel).exists():
            raise SystemExit(f"missing required input: {schema_rel}")

    findings: list[Finding] = []
    check_report_envelope(report, findings)
    checkpoints = checkpoint_index(report)
    session = ensure_dict(report.get("session", {}), "report.session")
    session_ref = session.get("session_ref")
    check_session(session, checkpoints, findings)
    for checkpoint in ensure_list(report.get("checkpoints", []), "report.checkpoints"):
        check_checkpoint(ensure_dict(checkpoint, "checkpoint"), findings)
    for transition in ensure_list(report.get("transitions", []), "report.transitions"):
        check_transition(ensure_dict(transition, "transition"), checkpoints, findings)
    for surface in ensure_list(report.get("surfaces", []), "report.surfaces"):
        check_surface(ensure_dict(surface, "surface"), session_ref, checkpoints, findings)
    check_support_export(report, export, findings)
    check_publications(repo_root, findings)

    if args.format == "json":
        print(json.dumps({"findings": [f.as_dict() for f in findings]}, indent=2))
    else:
        if not findings:
            print("m5 appearance-session runtime audit: clean")
        else:
            for finding in findings:
                location = finding.subject_ref or "report"
                print(f"FAIL [{finding.code}] {location}: {finding.message}")

    return 1 if findings else 0


if __name__ == "__main__":
    sys.exit(main())
