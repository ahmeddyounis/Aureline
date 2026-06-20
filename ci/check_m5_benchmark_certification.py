#!/usr/bin/env python3
"""Enforce the M5 benchmark-certification proof packet.

This gate turns benchmark-corpus governance, protected thresholds, and
public-comparison integrity into a single promotion-grade certification lane. It
validates the proof packet against its schema, resolves every certification row
to the upstream truth that backs it -- the benchmark-governance matrix, the
shiproom freshness ledger, the threshold-change ledger, the corpus-intake
ledger, the public-comparison reproducibility register, and the
publication-ingestion register -- and recomputes for each claimed performance,
compatibility, and qualification row the certification gaps that fire, the
narrowed effective claim, and the certification state directly from those
bindings. It fails closed when a stored value drifts from the recompute, when a
claimed row cannot prove current corpus identity, reference-hardware basis,
threshold lineage, or reproducibility-pack completeness, and when a green tile
would mask a fired gap; it holds promotion when a claim-bearing row narrows below
the posture it asserts; and it replays the certification fixtures and negative
drills that prove each fail-closed path.

Exit code is 0 when every check passes and 1 when any finding is at severity
``error``. With ``--require-proceed`` the gate also fails (exit code 2) when the
recomputed promotion verdict is ``hold``.
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

try:
    from jsonschema import Draft202012Validator
except Exception as exc:  # pragma: no cover - dependency guard
    raise SystemExit(
        "python jsonschema is required: pip install jsonschema"
    ) from exc


PACKET_REL = "artifacts/benchmarks/m5-benchmark-proof-packet.json"
PACKET_SCHEMA_REL = "schemas/benchmarks/m5-benchmark-proof-packet.schema.json"
FIXTURE_SCHEMA_REL = "schemas/benchmarks/m5-benchmark-certification-fixture.schema.json"
FIXTURE_REGISTER_REL = "fixtures/benchmarks/m5-benchmark-certification/manifest.yaml"

MATRIX_REL = "artifacts/benchmarks/m5-benchmark-governance.json"
FRESHNESS_REL = "artifacts/benchmarks/shiproom-benchmark-freshness.json"
REPRO_REGISTER_REL = "artifacts/benchmarks/public-comparison-pack-register.json"
THRESHOLD_LEDGER_REL = "artifacts/benchmarks/threshold-change-ledger.json"
CORPUS_INTAKE_REL = "artifacts/benchmarks/corpus-intake-ledger.json"
PUBLICATION_INGESTION_REL = "artifacts/benchmarks/publication-ingestion-register.json"

CLAIM_BEARING_LEVELS = {"aureline_only_claim", "public_head_to_head_comparison"}
ADMITTED_CI = {"admitted_real_data", "admitted_synthetic_only"}
SELF_CAPTURE_MARKER = ".self_capture"

PILLARS = (
    "corpus_identity",
    "hardware_basis",
    "threshold_lineage",
    "reproducibility_pack",
    "freshness_comparability",
    "publication_propagation",
)

# Each pillar's gap value, so the cross-check of stored evidence_bindings can map
# a fired gap back to the pillar that owns it.
GAP_PILLAR = {
    "uncertified_corpus_intake": "corpus_identity",
    "missing_hardware_basis": "hardware_basis",
    "missing_threshold_lineage": "threshold_lineage",
    "expired_threshold_waiver": "threshold_lineage",
    "missing_reproducibility_pack": "reproducibility_pack",
    "incomplete_reproducibility_pack": "reproducibility_pack",
    "stale_reproducibility_pack": "reproducibility_pack",
    "stale_freshness_evidence": "freshness_comparability",
    "incomparable_freshness_evidence": "freshness_comparability",
    "missing_publication_propagation": "publication_propagation",
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
    parser.add_argument(
        "--report",
        default=None,
        help="Write the machine-readable JSON report to this repo-relative path.",
    )
    parser.add_argument(
        "--require-proceed",
        action="store_true",
        help="Publication gate: also fail (exit 2) when the recomputed verdict is hold.",
    )
    return parser.parse_args()


def add_finding(
    findings: list[Finding],
    check_id: str,
    message: str,
    remediation: str,
    *,
    ref: str | None = None,
    severity: str = "error",
    details: dict[str, Any] | None = None,
) -> None:
    findings.append(
        Finding(
            severity=severity,
            check_id=check_id,
            message=message,
            remediation=remediation,
            ref=ref,
            details=details or {},
        )
    )


def load_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


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
                "permitted_classes: [Date, Time, DateTime], aliases: false); "
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


def parse_date(value: Any) -> dt.date | None:
    if not isinstance(value, str) or not value:
        return None
    try:
        return dt.date.fromisoformat(value[:10])
    except ValueError:
        return None


def is_self_capture(ref: Any) -> bool:
    return isinstance(ref, str) and SELF_CAPTURE_MARKER in ref


# --------------------------------------------------------------------------- #
# Upstream truth resolver. The same shape backs the canonical packet (built from
# the checked-in ledgers) and each fixture (built from its embedded snapshot), so
# the recompute is identical in both paths.
# --------------------------------------------------------------------------- #


@dataclass
class Upstream:
    freshness_entries: dict[str, dict[str, Any]]
    corpus_intake: dict[str, dict[str, Any]]
    governance_metrics: dict[str, dict[str, Any]]
    threshold_changes: dict[str, list[dict[str, Any]]]
    repro_packs: dict[str, dict[str, Any]]
    publication_surfaces: dict[str, list[str]]

    @classmethod
    def from_repo(cls, repo_root: Path) -> "Upstream":
        freshness = load_json(repo_root / FRESHNESS_REL)
        freshness_entries = {
            e.get("entry_id"): {
                "effective_claim": e.get("effective_claim"),
                "freshness_state": e.get("freshness_state"),
            }
            for e in freshness.get("entries", [])
            if isinstance(e, dict) and isinstance(e.get("entry_id"), str)
        }

        intake = load_json(repo_root / CORPUS_INTAKE_REL)
        corpus_intake = {
            r.get("corpus_ref"): {
                "decision_status": (r.get("intake_decision") or {}).get("status"),
                "ci_admissibility": r.get("ci_admissibility"),
            }
            for r in intake.get("records", [])
            if isinstance(r, dict) and isinstance(r.get("corpus_ref"), str)
        }

        matrix = load_json(repo_root / MATRIX_REL)
        governance_metrics = {
            m.get("metric_ref"): {
                "hardware_profile_ref": m.get("hardware_profile_ref"),
                "lab_image_ref": m.get("lab_image_ref"),
            }
            for m in matrix.get("protected_metrics", [])
            if isinstance(m, dict) and isinstance(m.get("metric_ref"), str)
        }

        threshold = load_json(repo_root / THRESHOLD_LEDGER_REL)
        threshold_changes: dict[str, list[dict[str, Any]]] = {}
        for c in threshold.get("changes", []):
            if not isinstance(c, dict):
                continue
            metric_ref = c.get("metric_ref")
            if not isinstance(metric_ref, str):
                continue
            waiver = c.get("waiver") or {}
            threshold_changes.setdefault(metric_ref, []).append(
                {
                    "status": c.get("status"),
                    "waiver_class": waiver.get("class"),
                    "waiver_expires_on": waiver.get("expires_on"),
                }
            )

        repro = load_json(repo_root / REPRO_REGISTER_REL)
        repro_packs = {
            p.get("governance_pack_ref"): {
                "status": p.get("status"),
                "raw_run_metadata_retained": p.get("raw_run_metadata_retained"),
                "disclosed_fields": p.get("disclosed_fields", []),
                "freshness_expires_on": (p.get("freshness") or {}).get("expires_on"),
            }
            for p in repro.get("packs", [])
            if isinstance(p, dict) and isinstance(p.get("governance_pack_ref"), str)
        }

        ingestion = load_json(repo_root / PUBLICATION_INGESTION_REL)
        publication_surfaces: dict[str, list[str]] = {}
        for b in ingestion.get("bindings", []):
            if not isinstance(b, dict):
                continue
            entry_id = b.get("consumes_entry_id")
            surface = b.get("surface")
            if isinstance(entry_id, str) and isinstance(surface, str):
                publication_surfaces.setdefault(entry_id, []).append(surface)

        return cls(
            freshness_entries=freshness_entries,
            corpus_intake=corpus_intake,
            governance_metrics=governance_metrics,
            threshold_changes=threshold_changes,
            repro_packs=repro_packs,
            publication_surfaces=publication_surfaces,
        )

    @classmethod
    def from_snapshot(cls, snapshot: dict[str, Any]) -> "Upstream":
        return cls(
            freshness_entries=snapshot.get("freshness_entries", {}),
            corpus_intake=snapshot.get("corpus_intake", {}),
            governance_metrics=snapshot.get("governance_metrics", {}),
            threshold_changes=snapshot.get("threshold_changes", {}),
            repro_packs=snapshot.get("repro_packs", {}),
            publication_surfaces=snapshot.get("publication_surfaces", {}),
        )


# --------------------------------------------------------------------------- #
# Certification engine.
# --------------------------------------------------------------------------- #


class CertEngine:
    """Recompute certification gaps, the narrowed claim, and the state."""

    def __init__(self, packet: dict[str, Any], required_surfaces: list[str]) -> None:
        self.rank = {
            row["level"]: row["rank"] for row in packet.get("claim_levels", [])
        }
        self.narrows_to = {
            row["gap"]: row["narrows_to"]
            for row in packet.get("certification_gaps", [])
        }
        self.required_surfaces = list(required_surfaces)

    def pillar_gaps(
        self, row: dict[str, Any], up: Upstream, as_of: dt.date
    ) -> dict[str, str | None]:
        gap: dict[str, str | None] = {p: None for p in PILLARS}
        posture = row.get("posture")
        claim_bearing = posture in CLAIM_BEARING_LEVELS

        # 1. Corpus identity: every bound corpus must carry an approved, CI-admitted
        #    intake decision.
        for corpus_ref in row.get("corpus_refs", []):
            rec = up.corpus_intake.get(corpus_ref)
            if (
                rec is None
                or rec.get("decision_status") != "approved"
                or rec.get("ci_admissibility") not in ADMITTED_CI
            ):
                gap["corpus_identity"] = "uncertified_corpus_intake"
                break

        # 2. Reference-hardware basis: every metric must bind a non-empty hardware
        #    profile and lab image, and a claim-bearing row may not ride self-capture.
        for metric_ref in row.get("metric_refs", []):
            mrow = up.governance_metrics.get(metric_ref)
            hw = (mrow or {}).get("hardware_profile_ref")
            lab = (mrow or {}).get("lab_image_ref")
            if mrow is None or not str(hw or "").strip() or not str(lab or "").strip():
                gap["hardware_basis"] = "missing_hardware_basis"
                break
            if claim_bearing and (is_self_capture(hw) or is_self_capture(lab)):
                gap["hardware_basis"] = "missing_hardware_basis"
                break

        # 3. Threshold lineage: every metric must carry an in-force (active) change
        #    record, and an active waiver may not be past its expiry.
        for metric_ref in row.get("metric_refs", []):
            changes = up.threshold_changes.get(metric_ref, [])
            active = [c for c in changes if c.get("status") == "active"]
            if not active:
                gap["threshold_lineage"] = "missing_threshold_lineage"
                break
            for c in active:
                waiver_class = c.get("waiver_class")
                expires = parse_date(c.get("waiver_expires_on"))
                if (
                    waiver_class not in (None, "none")
                    and expires is not None
                    and expires < as_of
                ):
                    gap["threshold_lineage"] = "expired_threshold_waiver"

        # 4. Reproducibility pack completeness (posture aware).
        pack = up.repro_packs.get(row.get("governance_pack_ref"))
        if pack is None:
            gap["reproducibility_pack"] = "missing_reproducibility_pack"
        elif not pack.get("raw_run_metadata_retained"):
            gap["reproducibility_pack"] = "incomplete_reproducibility_pack"
        elif posture == "quarantined_not_comparable":
            if pack.get("status") != "quarantined":
                gap["reproducibility_pack"] = "incomplete_reproducibility_pack"
        elif pack.get("status") != "publishable":
            gap["reproducibility_pack"] = "incomplete_reproducibility_pack"
        elif not set(row.get("required_repro_disclosure_fields", [])).issubset(
            set(pack.get("disclosed_fields", []))
        ):
            gap["reproducibility_pack"] = "incomplete_reproducibility_pack"
        else:
            expires = parse_date(pack.get("freshness_expires_on"))
            if expires is not None and expires < as_of:
                gap["reproducibility_pack"] = "stale_reproducibility_pack"

        # 5. Freshness and comparability (delegated to the freshness ledger entry).
        fe = up.freshness_entries.get(row.get("freshness_entry_ref"))
        fstate = fe.get("freshness_state") if fe else "missing"
        if fstate == "stale":
            gap["freshness_comparability"] = "stale_freshness_evidence"
        elif fstate in ("incomparable", "missing"):
            gap["freshness_comparability"] = "incomparable_freshness_evidence"

        # 6. Publication propagation: the entry must reach every required surface.
        surfaces = set(up.publication_surfaces.get(row.get("freshness_entry_ref"), []))
        if not set(self.required_surfaces).issubset(surfaces):
            gap["publication_propagation"] = "missing_publication_propagation"

        return gap

    def recompute(
        self, row: dict[str, Any], up: Upstream, as_of: dt.date
    ) -> dict[str, Any]:
        gap = self.pillar_gaps(row, up, as_of)
        fired = sorted({g for g in gap.values() if g})

        ceiling = row.get("published_claim_ceiling", "")
        candidates = [ceiling]
        fe = up.freshness_entries.get(row.get("freshness_entry_ref"))
        if fe and fe.get("effective_claim") is not None:
            candidates.append(fe["effective_claim"])
        candidates.extend(self.narrows_to[g] for g in fired if g in self.narrows_to)
        effective = min(candidates, key=lambda lv: self.rank.get(lv, 0))

        if self.rank.get(effective, 0) == 0:
            state = "quarantined"
        elif self.rank.get(effective, 0) < self.rank.get(ceiling, 0):
            state = "narrowed"
        else:
            state = "certified"

        posture = row.get("posture")
        blocks = (
            posture in CLAIM_BEARING_LEVELS
            and self.rank.get(effective, 0) < self.rank.get(posture, 0)
        )
        blocker_gaps = fired if blocks else []
        return {
            "pillar_gaps": gap,
            "fired": fired,
            "effective": effective,
            "state": state,
            "blocks": blocks,
            "blocker_gaps": blocker_gaps,
        }


# --------------------------------------------------------------------------- #
# Packet validation.
# --------------------------------------------------------------------------- #


def validate_packet_schema(
    repo_root: Path, packet: dict[str, Any], findings: list[Finding]
) -> None:
    schema = load_json(repo_root / PACKET_SCHEMA_REL)
    validator = Draft202012Validator(schema)
    for error in sorted(validator.iter_errors(packet), key=lambda e: list(e.path)):
        location = "/".join(str(p) for p in error.path) or "<root>"
        add_finding(
            findings,
            "packet.schema",
            f"packet fails its schema at {location}: {error.message}",
            "Bring the proof packet back into conformance with its boundary schema.",
            ref=PACKET_REL,
        )


def validate_source_refs(
    repo_root: Path, packet: dict[str, Any], findings: list[Finding]
) -> None:
    if PACKET_SCHEMA_REL not in packet.get("source_refs", []):
        add_finding(
            findings,
            "packet.source_refs.schema",
            "packet source_refs must cite its own schema",
            f"Add {PACKET_SCHEMA_REL} to source_refs.",
            ref=PACKET_REL,
        )
    refs = list(packet.get("source_refs", []))
    for key in (
        "governance_matrix_ref",
        "freshness_ledger_ref",
        "public_comparison_register_ref",
        "threshold_change_ledger_ref",
        "corpus_intake_ledger_ref",
        "publication_ingestion_register_ref",
        "evidence_index_ref",
    ):
        ref = packet.get(key)
        if isinstance(ref, str) and not (repo_root / ref).exists():
            add_finding(
                findings,
                f"packet.{key}.missing",
                f"packet cites a missing artifact for {key}: {ref}",
                "Publish the referenced artifact or correct the ref.",
                ref=ref,
            )
    for ref in refs:
        # Refs may carry a #fragment locator; only the file portion is on disk.
        file_part = ref.split("#", 1)[0] if isinstance(ref, str) else ref
        if isinstance(file_part, str) and "/" in file_part and not (repo_root / file_part).exists():
            add_finding(
                findings,
                "packet.source_refs.missing",
                f"packet cites a missing source artifact: {ref}",
                "Seed the referenced artifact or correct the source ref.",
                ref=ref,
            )


def validate_gap_table(packet: dict[str, Any], findings: list[Finding]) -> None:
    declared = {row.get("gap"): row.get("pillar") for row in packet.get("certification_gaps", [])}
    for gap, pillar in GAP_PILLAR.items():
        if gap not in declared:
            add_finding(
                findings,
                "packet.gap_table.missing_gap",
                f"certification_gaps omits a gap the engine can fire: {gap}",
                "Declare every fireable certification gap with its pillar and narrows_to.",
                ref=PACKET_REL,
            )
        elif declared[gap] != pillar:
            add_finding(
                findings,
                "packet.gap_table.pillar_mismatch",
                f"gap {gap} is declared under pillar {declared[gap]} but the engine owns it under {pillar}",
                "Keep each certification gap under the pillar that detects it.",
                ref=PACKET_REL,
            )


def validate_consumer_bindings(packet: dict[str, Any], findings: list[Finding]) -> None:
    consumers = {
        row.get("consumer", "")
        for row in packet.get("consumer_bindings", [])
        if isinstance(row, dict)
    }
    needs = {
        "a release surface": lambda c: c.startswith("release"),
        "a support/export surface": lambda c: "support" in c or "export" in c,
        "a docs or help surface": lambda c: "docs" in c or "help" in c,
    }
    for label, predicate in needs.items():
        if not any(predicate(c) for c in consumers):
            add_finding(
                findings,
                "packet.consumer_bindings.missing",
                f"packet has no consumer binding for {label}",
                "Bind release, support/export, and docs/help to the same packet ids.",
                ref=PACKET_REL,
            )


def validate_claim_class_coverage(packet: dict[str, Any], findings: list[Finding]) -> None:
    declared = {
        row.get("claim_class")
        for row in packet.get("claim_classes", [])
        if isinstance(row, dict)
    }
    covered = {row.get("claim_class") for row in packet.get("rows", [])}
    for claim_class in declared:
        if claim_class not in covered:
            add_finding(
                findings,
                "packet.claim_class.uncovered",
                f"declared claim class {claim_class} has no certification row",
                "Certify at least one row for every declared claim class.",
                ref=PACKET_REL,
            )


def validate_row(
    row: dict[str, Any],
    engine: CertEngine,
    up: Upstream,
    as_of: dt.date,
    findings: list[Finding],
) -> dict[str, Any]:
    row_id = row.get("row_id", "<row>")
    result = engine.recompute(row, up, as_of)
    fired = result["fired"]
    effective = result["effective"]
    state = result["state"]

    stored_fired = sorted(row.get("fired_certification_gaps", []))
    if stored_fired != fired:
        add_finding(
            findings,
            "row.fired_gaps_mismatch",
            f"row {row_id} stored gaps {stored_fired} != recompute {fired}",
            "Record exactly the certification gaps the bindings fire.",
            ref=row_id,
            details={"recomputed": fired, "stored": stored_fired},
        )

    if row.get("effective_claim") != effective:
        add_finding(
            findings,
            "row.effective_claim_mismatch",
            f"row {row_id} effective_claim {row.get('effective_claim')} != recompute {effective}",
            "Recompute the effective claim as the lowest of the ceiling, the freshness claim, and each fired gap.",
            ref=row_id,
            details={"fired": fired},
        )

    if row.get("certification_state") != state:
        add_finding(
            findings,
            "row.certification_state_mismatch",
            f"row {row_id} certification_state {row.get('certification_state')} != recompute {state}",
            "Derive the state: quarantined when the claim floors, narrowed when below the ceiling, else certified.",
            ref=row_id,
            details={"recomputed": state},
        )

    # Per-pillar evidence binding cross-check: a binding's stored status and gap
    # must equal what the engine derives for that pillar.
    bindings = row.get("evidence_bindings", {})
    for pillar in PILLARS:
        binding = bindings.get(pillar, {})
        expected_gap = result["pillar_gaps"].get(pillar)
        expected_status = "gap" if expected_gap else "certified"
        if binding.get("certification_status") != expected_status:
            add_finding(
                findings,
                "row.binding_status_mismatch",
                f"row {row_id} pillar {pillar} status {binding.get('certification_status')} != recompute {expected_status}",
                "A pillar reports a gap only when its evidence cannot prove currency.",
                ref=row_id,
            )
        if binding.get("gap") != expected_gap:
            add_finding(
                findings,
                "row.binding_gap_mismatch",
                f"row {row_id} pillar {pillar} gap {binding.get('gap')} != recompute {expected_gap}",
                "Record the exact gap the pillar fires, or null when it is clean.",
                ref=row_id,
            )

    # Guardrail: a certified (green) row may carry no fired gap.
    if state == "certified" and fired:
        add_finding(
            findings,
            "row.green_tile_masks_gap",
            f"row {row_id} renders certified while {fired} fired",
            "A certified row must carry no fired certification gap.",
            ref=row_id,
        )

    # Guardrail: a claim-bearing effective claim may not ride a non-current freshness entry.
    fe = up.freshness_entries.get(row.get("freshness_entry_ref"))
    fstate = fe.get("freshness_state") if fe else "missing"
    if effective in CLAIM_BEARING_LEVELS and fstate in {"stale", "incomparable", "missing"}:
        add_finding(
            findings,
            "row.claim_bearing_on_non_current_evidence",
            f"row {row_id} keeps claim-bearing {effective} on a {fstate} freshness entry",
            "A claim-bearing certified row must ride a current, comparable run.",
            ref=row_id,
        )

    # Blocker cross-check.
    blocker = row.get("promotion_blocker", {})
    if blocker.get("blocks_promotion") != result["blocks"]:
        add_finding(
            findings,
            "row.blocker_state_mismatch",
            f"row {row_id} blocks_promotion {blocker.get('blocks_promotion')} != recompute {result['blocks']}",
            "A row blocks promotion only when a claim-bearing posture narrows below itself.",
            ref=row_id,
        )
    if sorted(blocker.get("blocker_gaps", [])) != sorted(result["blocker_gaps"]):
        add_finding(
            findings,
            "row.blocker_gaps_mismatch",
            f"row {row_id} blocker_gaps {sorted(blocker.get('blocker_gaps', []))} != recompute {sorted(result['blocker_gaps'])}",
            "Surface exactly the fired gaps as blocker gaps when the row holds promotion.",
            ref=row_id,
        )

    # Release-packet alignment: a packet may not publish a fresher claim than the recompute.
    packet_row = row.get("release_packet", {})
    if packet_row.get("declared_certification_state") != state:
        add_finding(
            findings,
            "row.release_packet_state_mismatch",
            f"row {row_id} release packet state {packet_row.get('declared_certification_state')} != recompute {state}",
            "Keep the release packet certification state aligned with the recompute.",
            ref=row_id,
        )
    if packet_row.get("declared_effective_claim") != effective:
        add_finding(
            findings,
            "row.release_packet_claim_mismatch",
            f"row {row_id} release packet claim {packet_row.get('declared_effective_claim')} != recompute {effective}",
            "Keep the release packet effective claim aligned with the recompute.",
            ref=row_id,
        )

    # Freshness input cross-check: the row must mirror the ledger entry it cites.
    declared_input = row.get("freshness_input", {})
    if fe is not None:
        if declared_input.get("freshness_state") != fe.get("freshness_state"):
            add_finding(
                findings,
                "row.freshness_input_state_mismatch",
                f"row {row_id} freshness_input state {declared_input.get('freshness_state')} != ledger {fe.get('freshness_state')}",
                "Mirror the freshness ledger entry's state in the row's freshness_input.",
                ref=row_id,
            )
        if declared_input.get("effective_claim") != fe.get("effective_claim"):
            add_finding(
                findings,
                "row.freshness_input_claim_mismatch",
                f"row {row_id} freshness_input claim {declared_input.get('effective_claim')} != ledger {fe.get('effective_claim')}",
                "Mirror the freshness ledger entry's effective claim in the row's freshness_input.",
                ref=row_id,
            )
    else:
        add_finding(
            findings,
            "row.freshness_entry_unresolved",
            f"row {row_id} cites freshness entry {row.get('freshness_entry_ref')} absent from the ledger",
            "Bind every row to a freshness ledger entry.",
            ref=row_id,
        )

    return {"row_id": row_id, "fired": fired, "blocks": result["blocks"], "blocker_gaps": result["blocker_gaps"]}


def validate_projection(
    packet: dict[str, Any], row_views: list[dict[str, Any]], findings: list[Finding]
) -> str:
    blocking = [v for v in row_views if v["blocks"]]
    decision = "hold" if blocking else "proceed"
    blocking_row_ids = sorted(v["row_id"] for v in blocking)
    blocking_gaps = sorted({g for v in blocking for g in v["blocker_gaps"]})

    gate = packet.get("promotion_gate", {})
    if gate.get("decision") != decision:
        add_finding(
            findings,
            "projection.decision_mismatch",
            f"promotion gate decision {gate.get('decision')} != recompute {decision}",
            "Hold promotion when any claim-bearing row narrows below its posture.",
            ref=PACKET_REL,
        )
    if sorted(gate.get("blocking_row_ids", [])) != blocking_row_ids:
        add_finding(
            findings,
            "projection.blocking_rows_mismatch",
            "promotion gate blocking_row_ids disagrees with the narrowed rows",
            "List exactly the claim-bearing rows narrowed below their posture.",
            ref=PACKET_REL,
        )
    if sorted(gate.get("blocking_gaps", [])) != blocking_gaps:
        add_finding(
            findings,
            "projection.blocking_gaps_mismatch",
            "promotion gate blocking_gaps disagrees with the narrowed rows",
            "List exactly the certification gaps behind the blocking rows.",
            ref=PACKET_REL,
        )
    return decision


def validate_packet(
    repo_root: Path,
    packet: dict[str, Any],
    up: Upstream,
    findings: list[Finding],
) -> str:
    validate_packet_schema(repo_root, packet, findings)
    validate_source_refs(repo_root, packet, findings)
    validate_gap_table(packet, findings)
    validate_consumer_bindings(packet, findings)
    validate_claim_class_coverage(packet, findings)

    as_of = parse_date(packet.get("as_of"))
    if as_of is None:
        add_finding(
            findings,
            "packet.as_of_invalid",
            "packet as_of is not a parseable ISO date",
            "Set as_of to the evaluation date.",
            ref=PACKET_REL,
        )
        return "proceed"

    required_surfaces = packet.get("required_publication_surfaces", [])
    engine = CertEngine(packet, required_surfaces)
    row_views: list[dict[str, Any]] = []
    seen: set[str] = set()
    for row in packet.get("rows", []):
        row_id = row.get("row_id", "<row>")
        if row_id in seen:
            add_finding(
                findings,
                "row.duplicate_id",
                f"duplicate row id {row_id}",
                "Row ids must be unique.",
                ref=row_id,
            )
        seen.add(row_id)
        row_views.append(validate_row(row, engine, up, as_of, findings))

    return validate_projection(packet, row_views, findings)


# --------------------------------------------------------------------------- #
# Fixture replay.
# --------------------------------------------------------------------------- #


def replay_fixtures(
    repo_root: Path, packet: dict[str, Any], findings: list[Finding]
) -> int:
    register_path = repo_root / FIXTURE_REGISTER_REL
    if not register_path.exists():
        add_finding(
            findings,
            "fixtures.register_missing",
            f"certification fixture register is missing: {FIXTURE_REGISTER_REL}",
            "Seed the fixture register so the detection paths are proven.",
            ref=FIXTURE_REGISTER_REL,
        )
        return 0

    register = render_yaml_as_json(register_path)
    fixture_schema = load_json(repo_root / FIXTURE_SCHEMA_REL)
    validator = Draft202012Validator(fixture_schema)
    required_surfaces = packet.get("required_publication_surfaces", [])
    engine = CertEngine(packet, required_surfaces)

    count = 0
    for rel in register.get("fixtures", []):
        fixture = load_json(repo_root / rel)
        count += 1

        if list(validator.iter_errors(fixture)):
            add_finding(
                findings,
                "fixture.schema",
                f"fixture {rel} fails the certification-fixture schema",
                "Bring the fixture into conformance with the fixture-record schema.",
                ref=rel,
            )
            continue

        as_of = parse_date(fixture.get("as_of"))
        if as_of is None:
            add_finding(
                findings,
                "fixture.as_of_invalid",
                f"fixture {rel} has no parseable as_of",
                "Set a fixture as_of so date-sensitive gaps are deterministic.",
                ref=rel,
            )
            continue

        up = Upstream.from_snapshot(fixture.get("upstream", {}))
        result = engine.recompute(fixture.get("row", {}), up, as_of)

        expected_fired = sorted(fixture.get("expected_fired_gaps", []))
        if result["fired"] != expected_fired:
            add_finding(
                findings,
                "fixture.fired_gaps_mismatch",
                f"fixture {fixture.get('fixture_id')} expects {expected_fired} but recompute fires {result['fired']}",
                "Align the fixture upstream snapshot or its expectation.",
                ref=rel,
            )
        if result["effective"] != fixture.get("expected_effective_claim"):
            add_finding(
                findings,
                "fixture.effective_claim_mismatch",
                (
                    f"fixture {fixture.get('fixture_id')} expects claim "
                    f"{fixture.get('expected_effective_claim')} but recompute yields {result['effective']}"
                ),
                "Align the fixture expectation with the certification engine.",
                ref=rel,
            )
        if result["state"] != fixture.get("expected_certification_state"):
            add_finding(
                findings,
                "fixture.state_mismatch",
                (
                    f"fixture {fixture.get('fixture_id')} expects state "
                    f"{fixture.get('expected_certification_state')} but recompute yields {result['state']}"
                ),
                "Align the fixture expectation with the certification engine.",
                ref=rel,
            )
        if "expected_blocks_promotion" in fixture and result["blocks"] != fixture.get(
            "expected_blocks_promotion"
        ):
            add_finding(
                findings,
                "fixture.blocks_mismatch",
                (
                    f"fixture {fixture.get('fixture_id')} expects blocks_promotion "
                    f"{fixture.get('expected_blocks_promotion')} but recompute yields {result['blocks']}"
                ),
                "Align the fixture expectation with the promotion gate.",
                ref=rel,
            )
    return count


# --------------------------------------------------------------------------- #
# Negative drills.
# --------------------------------------------------------------------------- #


def run_negative_drills(
    repo_root: Path, packet: dict[str, Any], up: Upstream, findings: list[Finding]
) -> list[dict[str, str]]:
    results: list[dict[str, str]] = []

    def check_ids(candidate: dict[str, Any]) -> set[str]:
        local: list[Finding] = []
        validate_packet(repo_root, candidate, up, local)
        return {f.check_id for f in local}

    def record(drill_id: str, expected: str, fired: bool) -> None:
        results.append(
            {
                "drill_id": drill_id,
                "expected_check_id": expected,
                "status": "passed" if fired else "failed",
            }
        )
        if not fired:
            add_finding(
                findings,
                "negative_drill.not_rejected",
                f"negative drill {drill_id} did not fire {expected}",
                "The recompute must reject this mutation.",
                ref=drill_id,
            )

    def certified_row(candidate: dict[str, Any]) -> dict[str, Any] | None:
        return next(
            (r for r in candidate["rows"] if r.get("certification_state") == "certified"),
            None,
        )

    # 1. Hiding a fired gap behind a certified tile must be rejected.
    mutated = copy.deepcopy(packet)
    target = next((r for r in mutated["rows"] if r.get("fired_certification_gaps")), None)
    if target is not None:
        target["fired_certification_gaps"] = []
        record("hidden_gap_rejected", "row.fired_gaps_mismatch", "row.fired_gaps_mismatch" in check_ids(mutated))

    # 2. Overstating a narrowed row's effective claim must be rejected.
    mutated = copy.deepcopy(packet)
    target = certified_row(mutated)
    if target is not None:
        target["effective_claim"] = "public_head_to_head_comparison"
        if target.get("published_claim_ceiling") != "public_head_to_head_comparison":
            record("overstated_claim_rejected", "row.effective_claim_mismatch", "row.effective_claim_mismatch" in check_ids(mutated))

    # 3. A binding that claims certified while its pillar gap fires must be rejected.
    mutated = copy.deepcopy(packet)
    target = next(
        (r for r in mutated["rows"] if r.get("certification_state") == "quarantined"),
        None,
    )
    if target is not None:
        target["evidence_bindings"]["freshness_comparability"] = {
            "certification_status": "certified",
            "gap": None,
            "bound_refs": target["evidence_bindings"]["freshness_comparability"].get("bound_refs", []),
        }
        record("binding_overstated_rejected", "row.binding_status_mismatch", "row.binding_status_mismatch" in check_ids(mutated))

    # 4. A proceed verdict that hides a blocked claim-bearing row must be rejected.
    mutated = copy.deepcopy(packet)
    target = next(
        (
            r
            for r in mutated["rows"]
            if r.get("posture") in CLAIM_BEARING_LEVELS
            and isinstance(r.get("release_packet"), dict)
        ),
        None,
    )
    if target is not None:
        # Make the cited freshness entry look stale by retargeting it to a
        # nonexistent entry; the recompute then quarantines the claim-bearing row.
        target["freshness_entry_ref"] = "publication_pack.__nonexistent__"
        record("blocked_row_hidden_by_proceed", "projection.decision_mismatch", "projection.decision_mismatch" in check_ids(mutated))

    # 5. A release packet that publishes a fresher claim than the recompute must be rejected.
    mutated = copy.deepcopy(packet)
    target = certified_row(mutated)
    if target is not None:
        target["release_packet"]["declared_effective_claim"] = "public_head_to_head_comparison"
        if target.get("effective_claim") != "public_head_to_head_comparison":
            record("release_packet_overstated_rejected", "row.release_packet_claim_mismatch", "row.release_packet_claim_mismatch" in check_ids(mutated))

    return results


# --------------------------------------------------------------------------- #
# Entry point.
# --------------------------------------------------------------------------- #


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    findings: list[Finding] = []
    packet = load_json(repo_root / PACKET_REL)
    if not isinstance(packet, dict):
        raise SystemExit("proof packet must be a JSON object")

    up = Upstream.from_repo(repo_root)
    decision = validate_packet(repo_root, packet, up, findings)
    fixture_count = replay_fixtures(repo_root, packet, findings)
    drill_results = run_negative_drills(repo_root, packet, up, findings)

    errors = [f for f in findings if f.severity == "error"]
    warnings = [f for f in findings if f.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[m5-benchmark-certification] {status} "
        f"({len(errors)} errors, {len(warnings)} warnings) -- "
        f"rows: {len(packet.get('rows', []))}, "
        f"fixtures: {fixture_count}, drills: {len(drill_results)}, "
        f"promotion: {decision}, as_of: {packet.get('as_of')}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        suffix = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[m5-benchmark-certification] {prefix} {finding.check_id}: {finding.message}{suffix}"
        )
        print(f"[m5-benchmark-certification]   remediation: {finding.remediation}")

    if args.report:
        report = {
            "schema_version": 1,
            "check_id": "m5_benchmark_certification",
            "evaluated_on": packet.get("as_of"),
            "status": "pass" if not errors else "fail",
            "packet_ref": PACKET_REL,
            "row_count": len(packet.get("rows", [])),
            "fixture_count": fixture_count,
            "drill_count": len(drill_results),
            "promotion_decision": decision,
            "negative_drills": drill_results,
            "finding_counts": {"error": len(errors), "warning": len(warnings)},
            "findings": [f.as_report() for f in findings],
        }
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(
            json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )

    if errors:
        return 1
    if args.require_proceed and decision == "hold":
        print(
            "[m5-benchmark-certification] PUBLICATION HELD: certification promotion blocked",
            file=sys.stderr,
        )
        return 2
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[m5-benchmark-certification] interrupted", file=sys.stderr)
        sys.exit(130)
