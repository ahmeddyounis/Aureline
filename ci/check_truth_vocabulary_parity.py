#!/usr/bin/env python3
"""Enforce cross-surface truth-vocabulary parity.

This is the governance gate for the product truth vocabulary: the trust-bearing
and release-bearing state words that product UI, Help/About, docs, release
notes, admin / CLI / headless inspect output, and support bundles all quote. It
consumes the governed registry at

    artifacts/governance/product_truth_vocabulary.yaml

and proves the quiet ways one vocabulary can fork into many are caught before a
beta widens or a stable promotes:

- **Registry drift.** A class re-states a vocabulary that has moved on in its
  upstream schema or ledger. Every axis pins one `canonical_source`; the
  validator extracts it verbatim and fails closed if the registry disagrees.
- **Forbidden parallel synonym.** A surface quotes a banned synonym (`GA` for
  Stable, `service down` for unavailable) instead of the controlled word.
- **Unknown / off-vocabulary term.** A surface invents a word for a protected
  state class.
- **Cross-surface conflict.** Two surfaces describe the same protected subject
  with conflicting resolved vocabulary (Help/About says Stable, release notes
  says Beta).
- **Silent alias drift.** A deprecated alias persists past its migrate_by date
  instead of migrating forward to the canonical word.

It validates two corpora:

1. `fixtures/governance/truth_vocabulary_parity/surface_corpus.json` — the
   conforming cross-surface state map. Every subject must resolve to one
   canonical word on every surface; this corpus is expected to pass clean.
2. `fixtures/governance/truth_vocabulary_parity/conformance_corpus.json` —
   failure drills, each annotated with the finding it must produce. The
   validator re-runs its own classification on each drill and asserts the
   expected blocker / alias_migration fires, so the gate cannot rot into a
   no-op.

Outputs the diffable parity report at
artifacts/release/m3/truth_vocabulary_parity_report.md (regenerate with
--write; drift-checked otherwise) and, with --report-json PATH, a
machine-readable findings packet for CI artifact upload.

Run via scripts/ci/run_truth_vocabulary_parity.sh.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any, NoReturn

# --------------------------------------------------------------------------- #
# Paths
# --------------------------------------------------------------------------- #

REGISTRY_REL = "artifacts/governance/product_truth_vocabulary.yaml"
SURFACE_CORPUS_REL = "fixtures/governance/truth_vocabulary_parity/surface_corpus.json"
CONFORMANCE_CORPUS_REL = (
    "fixtures/governance/truth_vocabulary_parity/conformance_corpus.json"
)
REPORT_REL = "artifacts/release/m3/truth_vocabulary_parity_report.md"
PUBLIC_DOC_REL = "docs/public/m3/truth_vocabulary_reference.md"

SEVERITY_BLOCKER = "blocker"
SEVERITY_WARNING = "warning"
SEVERITY_ALIAS = "alias_migration"

ALIAS_STATUSES = {"allowed", "deprecated", "forbidden"}

REQUIRED_SEVERITY_KEYS = {
    "forbidden_alias_on_protected_surface",
    "forbidden_alias_on_advisory_surface",
    "unknown_term_on_protected_surface",
    "unknown_term_on_advisory_surface",
    "cross_surface_conflict_protected",
    "cross_surface_conflict_advisory",
    "deprecated_alias_within_window",
    "deprecated_alias_past_migrate_by",
    "registry_parity_drift",
}


# --------------------------------------------------------------------------- #
# Findings
# --------------------------------------------------------------------------- #


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


# --------------------------------------------------------------------------- #
# Loading helpers
# --------------------------------------------------------------------------- #


def fail(message: str) -> NoReturn:
    raise SystemExit(f"[truth-vocabulary] error: {message}")


def load_json(path: Path) -> Any:
    if not path.exists():
        fail(f"missing JSON file: {path}")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON at {path}: {exc}")


def load_yaml(path: Path) -> Any:
    """Decode YAML via Ruby/Psych so the lane needs no Python YAML dependency."""
    if not path.exists():
        fail(f"missing YAML file: {path}")
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), permitted_classes: [], "
                "aliases: false); STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        fail(f"failed to parse YAML at {path}: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        fail(f"Ruby/Psych emitted invalid JSON for {path}: {exc}")


def extract_pointer(payload: Any, pointer: list[str], source_ref: str) -> Any:
    cur = payload
    walked: list[str] = []
    for key in pointer:
        walked.append(key)
        if isinstance(cur, dict) and key in cur:
            cur = cur[key]
        else:
            fail(
                f"canonical_source {source_ref}: pointer {'/'.join(walked)} "
                f"does not resolve"
            )
    return cur


def extract_canonical(repo_root: Path, source: dict[str, Any]) -> list[str]:
    ref = source["ref"]
    fmt = source["format"]
    pointer = source["pointer"]
    path = repo_root / ref
    if fmt == "json":
        payload = load_json(path)
    elif fmt == "yaml":
        payload = load_yaml(path)
    else:
        fail(f"canonical_source {ref}: unsupported format {fmt!r}")
    value = extract_pointer(payload, pointer, ref)
    collect_field = source.get("collect_field")
    if collect_field is not None:
        if not isinstance(value, list):
            fail(f"canonical_source {ref}: collect_field target is not a list")
        ordered: list[str] = []
        for row in value:
            if not isinstance(row, dict) or collect_field not in row:
                fail(
                    f"canonical_source {ref}: row missing collect_field "
                    f"{collect_field!r}"
                )
            token = row[collect_field]
            if token not in ordered:
                ordered.append(token)
        return ordered
    if not isinstance(value, list):
        fail(f"canonical_source {ref}: pointer target is not a list")
    return list(value)


# --------------------------------------------------------------------------- #
# Registry model
# --------------------------------------------------------------------------- #


@dataclass
class Axis:
    class_id: str
    axis_id: str
    title: str
    canonical_terms: list[str]
    # term -> (canonical_term, status). canonical terms map to themselves with
    # status "canonical".
    resolution: dict[str, tuple[str, str]]
    aliases: list[dict[str, Any]]


@dataclass
class VocabularyClass:
    class_id: str
    title: str
    summary: str
    axes: list[Axis]


@dataclass
class Surface:
    surface_id: str
    title: str
    protected_classes: set[str]


@dataclass
class Registry:
    as_of: dt.date
    classes: dict[str, VocabularyClass]
    axes: dict[tuple[str, str], Axis]
    surfaces: dict[str, Surface]
    severity_policy: dict[str, str]


def parse_date(value: str, label: str) -> dt.date:
    try:
        return dt.date.fromisoformat(value)
    except (ValueError, TypeError):
        fail(f"{label} must be a YYYY-MM-DD date, got {value!r}")


def build_registry(
    repo_root: Path, registry: dict[str, Any], findings: list[Finding]
) -> Registry:
    if registry.get("schema_version") != 1:
        fail("registry schema_version must be 1")
    as_of = parse_date(registry.get("as_of", ""), "registry.as_of")

    severity_policy = registry.get("severity_policy") or {}
    missing_sev = REQUIRED_SEVERITY_KEYS - set(severity_policy)
    if missing_sev:
        fail(f"registry.severity_policy missing keys: {sorted(missing_sev)}")

    # Surfaces.
    surfaces: dict[str, Surface] = {}
    for raw in registry.get("surface_classes") or []:
        sid = raw["surface_id"]
        if sid in surfaces:
            fail(f"duplicate surface_id {sid!r}")
        surfaces[sid] = Surface(
            surface_id=sid,
            title=raw.get("title", sid),
            protected_classes=set(raw.get("protected_classes") or []),
        )
        for ref in raw.get("representative_refs") or []:
            if not (repo_root / ref).exists():
                findings.append(
                    Finding(
                        severity=SEVERITY_BLOCKER,
                        check_id="registry.surface_ref_missing",
                        message=f"surface {sid} representative_ref does not resolve: {ref}",
                        remediation="Fix the path or seed the referenced surface.",
                        ref=ref,
                    )
                )

    # Vocabulary classes.
    classes: dict[str, VocabularyClass] = {}
    axes: dict[tuple[str, str], Axis] = {}
    for raw_cls in registry.get("vocabulary_classes") or []:
        class_id = raw_cls["class_id"]
        if class_id in classes:
            fail(f"duplicate class_id {class_id!r}")
        axis_objs: list[Axis] = []
        for raw_axis in raw_cls.get("axes") or []:
            axis_id = raw_axis["axis_id"]
            if (class_id, axis_id) in axes:
                fail(f"duplicate axis {class_id}.{axis_id}")
            canonical_terms = list(raw_axis.get("canonical_terms") or [])
            if len(canonical_terms) != len(set(canonical_terms)):
                fail(f"{class_id}.{axis_id}: duplicate canonical_terms")

            # Registry <-> upstream parity.
            upstream = extract_canonical(repo_root, raw_axis["canonical_source"])
            if canonical_terms != upstream:
                findings.append(
                    Finding(
                        severity=registry["severity_policy"]["registry_parity_drift"],
                        check_id="registry_parity",
                        message=(
                            f"{class_id}.{axis_id} canonical_terms drift from "
                            f"{raw_axis['canonical_source']['ref']}"
                        ),
                        remediation=(
                            "Refresh the registry axis to mirror its upstream "
                            "canonical_source verbatim (or update the upstream)."
                        ),
                        ref=raw_axis["canonical_source"]["ref"],
                        details={"registry": canonical_terms, "upstream": upstream},
                    )
                )

            resolution: dict[str, tuple[str, str]] = {}
            for term in canonical_terms:
                resolution[term] = (term, "canonical")
            alias_rows = raw_axis.get("aliases") or []
            for alias in alias_rows:
                name = alias["alias"]
                status = alias.get("status")
                target = alias.get("canonical")
                if status not in ALIAS_STATUSES:
                    fail(f"{class_id}.{axis_id} alias {name!r}: bad status {status!r}")
                if target not in canonical_terms:
                    fail(
                        f"{class_id}.{axis_id} alias {name!r}: canonical {target!r} "
                        f"is not a canonical term in this axis"
                    )
                if name in resolution:
                    fail(
                        f"{class_id}.{axis_id} alias {name!r} collides with an "
                        f"existing term/alias"
                    )
                if status == "deprecated":
                    migrate_by = alias.get("migrate_by")
                    if migrate_by is None:
                        fail(
                            f"{class_id}.{axis_id} alias {name!r}: deprecated alias "
                            f"needs migrate_by"
                        )
                    parse_date(migrate_by, f"{class_id}.{axis_id} alias {name}.migrate_by")
                if status in {"deprecated", "forbidden"} and not alias.get("reason"):
                    fail(
                        f"{class_id}.{axis_id} alias {name!r}: {status} alias needs a "
                        f"reason"
                    )
                resolution[name] = (target, status)

            axis = Axis(
                class_id=class_id,
                axis_id=axis_id,
                title=raw_axis.get("title", axis_id),
                canonical_terms=canonical_terms,
                resolution=resolution,
                aliases=alias_rows,
            )
            axis_objs.append(axis)
            axes[(class_id, axis_id)] = axis

        classes[class_id] = VocabularyClass(
            class_id=class_id,
            title=raw_cls.get("title", class_id),
            summary=(raw_cls.get("summary") or "").strip(),
            axes=axis_objs,
        )

    # Surface protected_classes must reference real classes.
    for surface in surfaces.values():
        for cid in surface.protected_classes:
            if cid not in classes:
                fail(
                    f"surface {surface.surface_id} protects unknown class {cid!r}"
                )

    return Registry(
        as_of=as_of,
        classes=classes,
        axes=axes,
        surfaces=surfaces,
        severity_policy=registry["severity_policy"],
    )


# --------------------------------------------------------------------------- #
# Subject classification
# --------------------------------------------------------------------------- #


def alias_reason(axis: Axis, term: str) -> str | None:
    for alias in axis.aliases:
        if alias["alias"] == term:
            return alias.get("reason")
    return None


def alias_migrate_by(axis: Axis, term: str) -> str | None:
    for alias in axis.aliases:
        if alias["alias"] == term:
            return alias.get("migrate_by")
    return None


def classify_subject(reg: Registry, subject: dict[str, Any]) -> list[Finding]:
    """Classify one cross-surface subject; return its findings."""
    findings: list[Finding] = []
    subject_id = subject["subject_id"]
    class_id = subject["class_id"]
    axis_id = subject["axis_id"]
    expected = subject["expected_canonical"]

    axis = reg.axes.get((class_id, axis_id))
    if axis is None:
        return [
            Finding(
                severity=SEVERITY_BLOCKER,
                check_id="subject.unknown_axis",
                message=f"subject {subject_id} names unknown axis {class_id}.{axis_id}",
                remediation="Use a class_id/axis_id from the registry.",
            )
        ]
    if expected not in axis.canonical_terms:
        findings.append(
            Finding(
                severity=SEVERITY_BLOCKER,
                check_id="subject.expected_not_canonical",
                message=(
                    f"subject {subject_id} expected_canonical {expected!r} is not a "
                    f"canonical term of {class_id}.{axis_id}"
                ),
                remediation="Set expected_canonical to a canonical term.",
            )
        )

    # Per-surface resolution for conflict detection.
    resolved_by_surface: list[tuple[str, str, bool]] = []  # surface, canonical, protected
    for usage in subject.get("usages") or []:
        surface_id = usage["surface_class"]
        term = usage["term"]
        source_ref = usage.get("source_ref")
        surface = reg.surfaces.get(surface_id)
        if surface is None:
            findings.append(
                Finding(
                    severity=SEVERITY_BLOCKER,
                    check_id="subject.unknown_surface",
                    message=f"subject {subject_id} names unknown surface {surface_id!r}",
                    remediation="Use a surface_id from the registry.",
                    ref=source_ref,
                )
            )
            continue
        protected = class_id in surface.protected_classes
        resolution = axis.resolution.get(term)

        if resolution is None:
            sev = (
                reg.severity_policy["unknown_term_on_protected_surface"]
                if protected
                else reg.severity_policy["unknown_term_on_advisory_surface"]
            )
            findings.append(
                Finding(
                    severity=sev,
                    check_id="unknown_term",
                    message=(
                        f"{surface_id} describes {subject_id} ({class_id}.{axis_id}) with "
                        f"off-vocabulary term {term!r}"
                    ),
                    remediation=(
                        "Quote a canonical term or a registered alias for this axis."
                    ),
                    ref=source_ref,
                    details={
                        "subject_id": subject_id,
                        "surface": surface_id,
                        "term": term,
                        "protected": protected,
                    },
                )
            )
            continue

        canonical, status = resolution
        if status == "forbidden":
            sev = (
                reg.severity_policy["forbidden_alias_on_protected_surface"]
                if protected
                else reg.severity_policy["forbidden_alias_on_advisory_surface"]
            )
            findings.append(
                Finding(
                    severity=sev,
                    check_id="forbidden_alias",
                    message=(
                        f"{surface_id} describes {subject_id} ({class_id}.{axis_id}) with "
                        f"forbidden synonym {term!r}"
                    ),
                    remediation=(
                        alias_reason(axis, term)
                        or f"Quote the canonical term {canonical!r}."
                    ),
                    ref=source_ref,
                    details={
                        "subject_id": subject_id,
                        "surface": surface_id,
                        "term": term,
                        "canonical": canonical,
                        "protected": protected,
                    },
                )
            )
        elif status == "deprecated":
            migrate_by = alias_migrate_by(axis, term)
            past = migrate_by is not None and parse_date(
                migrate_by, "alias.migrate_by"
            ) < reg.as_of
            if past:
                findings.append(
                    Finding(
                        severity=reg.severity_policy["deprecated_alias_past_migrate_by"],
                        check_id="expired_alias_migration",
                        message=(
                            f"{surface_id} still uses deprecated alias {term!r} for "
                            f"{subject_id} ({class_id}.{axis_id}) past migrate_by "
                            f"{migrate_by}"
                        ),
                        remediation=(
                            f"Migrate to the canonical term {canonical!r}; the migration "
                            f"window has closed."
                        ),
                        ref=source_ref,
                        details={
                            "subject_id": subject_id,
                            "surface": surface_id,
                            "term": term,
                            "canonical": canonical,
                            "migrate_by": migrate_by,
                        },
                    )
                )
            else:
                findings.append(
                    Finding(
                        severity=reg.severity_policy["deprecated_alias_within_window"],
                        check_id="deprecated_alias",
                        message=(
                            f"{surface_id} uses deprecated alias {term!r} for "
                            f"{subject_id} ({class_id}.{axis_id}); migrate to "
                            f"{canonical!r} by {migrate_by}"
                        ),
                        remediation=(
                            f"Migrate forward to the canonical term {canonical!r} before "
                            f"{migrate_by}."
                        ),
                        ref=source_ref,
                        details={
                            "subject_id": subject_id,
                            "surface": surface_id,
                            "term": term,
                            "canonical": canonical,
                            "migrate_by": migrate_by,
                        },
                    )
                )
        # canonical / allowed alias resolve silently.

        resolved_by_surface.append((surface_id, canonical, protected))

    # Cross-surface conflict: surfaces that resolve to a canonical disagree.
    distinct = {canonical for _, canonical, _ in resolved_by_surface}
    distinct.add(expected)
    if len(distinct) > 1:
        any_protected = any(p for _, _, p in resolved_by_surface)
        sev = (
            reg.severity_policy["cross_surface_conflict_protected"]
            if any_protected
            else reg.severity_policy["cross_surface_conflict_advisory"]
        )
        findings.append(
            Finding(
                severity=sev,
                check_id="cross_surface_conflict",
                message=(
                    f"subject {subject_id} ({class_id}.{axis_id}) is described with "
                    f"conflicting vocabulary across surfaces: {sorted(distinct)}"
                ),
                remediation=(
                    "Surfaces describing the same subject must resolve to one "
                    "canonical term."
                ),
                details={
                    "subject_id": subject_id,
                    "expected_canonical": expected,
                    "resolved": [
                        {"surface": s, "canonical": c}
                        for s, c, _ in resolved_by_surface
                    ],
                },
            )
        )

    return findings


# --------------------------------------------------------------------------- #
# Corpus passes
# --------------------------------------------------------------------------- #


def run_surface_corpus(reg: Registry, corpus: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    seen: set[str] = set()
    for subject in corpus.get("subjects") or []:
        sid = subject["subject_id"]
        if sid in seen:
            findings.append(
                Finding(
                    severity=SEVERITY_BLOCKER,
                    check_id="surface_corpus.duplicate_subject",
                    message=f"duplicate subject_id {sid!r} in surface corpus",
                    remediation="Make every subject_id unique.",
                )
            )
        seen.add(sid)
        findings.extend(classify_subject(reg, subject))
    return findings


def run_conformance_corpus(reg: Registry, corpus: dict[str, Any]) -> list[Finding]:
    """Re-run classification per drill; assert the expected finding fires."""
    findings: list[Finding] = []
    for drill in corpus.get("drills") or []:
        drill_id = drill["drill_id"]
        expected = drill["expected_finding"]
        produced = classify_subject(reg, drill["subject"])
        match = [
            f
            for f in produced
            if f.severity == expected["severity"] and f.check_id == expected["check_id"]
        ]
        if not match:
            findings.append(
                Finding(
                    severity=SEVERITY_BLOCKER,
                    check_id="conformance.drill_did_not_fire",
                    message=(
                        f"conformance drill {drill_id} did not produce expected "
                        f"{expected['severity']}/{expected['check_id']}"
                    ),
                    remediation=(
                        "The gate no longer catches this drift; fix the validator "
                        "or the drill."
                    ),
                    details={
                        "drill_id": drill_id,
                        "expected": expected,
                        "produced": [
                            {"severity": f.severity, "check_id": f.check_id}
                            for f in produced
                        ],
                    },
                )
            )
    return findings


# --------------------------------------------------------------------------- #
# Public reference doc coverage
# --------------------------------------------------------------------------- #


def check_public_doc(repo_root: Path, reg: Registry) -> list[Finding]:
    findings: list[Finding] = []
    path = repo_root / PUBLIC_DOC_REL
    if not path.exists():
        return [
            Finding(
                severity=SEVERITY_BLOCKER,
                check_id="public_doc.missing",
                message=f"public reference doc missing: {PUBLIC_DOC_REL}",
                remediation="Author the public truth-vocabulary reference.",
                ref=PUBLIC_DOC_REL,
            )
        ]
    text = path.read_text(encoding="utf-8")
    for class_id, cls in reg.classes.items():
        if class_id not in text:
            findings.append(
                Finding(
                    severity=SEVERITY_BLOCKER,
                    check_id="public_doc.class_missing",
                    message=f"public reference doc omits class {class_id!r}",
                    remediation="Document every governed vocabulary class.",
                    ref=PUBLIC_DOC_REL,
                )
            )
        for axis in cls.axes:
            for term in axis.canonical_terms:
                if term not in text:
                    findings.append(
                        Finding(
                            severity=SEVERITY_WARNING,
                            check_id="public_doc.term_missing",
                            message=(
                                f"public reference doc omits canonical term "
                                f"{class_id}.{axis.axis_id}:{term}"
                            ),
                            remediation="List every canonical term in the reference.",
                            ref=PUBLIC_DOC_REL,
                        )
                    )
    for surface_id in reg.surfaces:
        if surface_id not in text:
            findings.append(
                Finding(
                    severity=SEVERITY_WARNING,
                    check_id="public_doc.surface_missing",
                    message=f"public reference doc omits surface {surface_id!r}",
                    remediation="Document every governed surface class.",
                    ref=PUBLIC_DOC_REL,
                )
            )
    return findings


# --------------------------------------------------------------------------- #
# Report
# --------------------------------------------------------------------------- #


def render_report(
    reg: Registry,
    surface_findings: list[Finding],
    conformance_corpus: dict[str, Any],
    conformance_findings: list[Finding],
) -> str:
    lines: list[str] = []
    lines.append("# Truth-vocabulary parity report")
    lines.append("")
    lines.append(
        "Generated by `ci/check_truth_vocabulary_parity.py`. Do not edit by hand — "
        "refresh with `python3 ci/check_truth_vocabulary_parity.py --repo-root . "
        "--write`."
    )
    lines.append("")
    lines.append("This report is the single diffable artifact shiproom, docs, support,")
    lines.append("and certification read to confirm one truth vocabulary across product")
    lines.append("UI, Help/About, docs, release notes, admin / CLI / headless inspect")
    lines.append("output, and support bundles before a beta widens or a stable promotes.")
    lines.append("")

    lines.append("## Registry")
    lines.append("")
    lines.append(f"- **Registry:** `{REGISTRY_REL}`")
    lines.append(f"- **As of:** {reg.as_of.isoformat()}")
    lines.append(f"- **Vocabulary classes:** {len(reg.classes)}")
    lines.append(f"- **Axes:** {len(reg.axes)}")
    lines.append(f"- **Surface classes:** {len(reg.surfaces)}")
    lines.append("")

    lines.append("## Vocabulary classes and upstream parity")
    lines.append("")
    lines.append("| Class | Axis | Canonical source | Canonical terms |")
    lines.append("|---|---|---|---|")
    for class_id, cls in reg.classes.items():
        for axis in cls.axes:
            terms = ", ".join(f"`{t}`" for t in axis.canonical_terms)
            lines.append(
                f"| `{class_id}` | `{axis.axis_id}` | (mirrored verbatim) | {terms} |"
            )
    lines.append("")

    lines.append("## Surface protection map")
    lines.append("")
    lines.append("| Surface | Protected vocabulary classes |")
    lines.append("|---|---|")
    for sid, surface in reg.surfaces.items():
        protected = ", ".join(f"`{c}`" for c in sorted(surface.protected_classes))
        lines.append(f"| `{sid}` | {protected} |")
    lines.append("")

    lines.append("## Controlled aliases")
    lines.append("")
    lines.append("| Class | Axis | Alias | Resolves to | Status | Migrate by |")
    lines.append("|---|---|---|---|---|---|")
    for class_id, cls in reg.classes.items():
        for axis in cls.axes:
            for alias in axis.aliases:
                if alias.get("status") == "allowed":
                    continue
                lines.append(
                    f"| `{class_id}` | `{axis.axis_id}` | `{alias['alias']}` | "
                    f"`{alias['canonical']}` | {alias['status']} | "
                    f"{alias.get('migrate_by', '—')} |"
                )
    lines.append("")

    lines.append("## Surface-corpus findings")
    lines.append("")
    counts = {SEVERITY_BLOCKER: 0, SEVERITY_WARNING: 0, SEVERITY_ALIAS: 0}
    for f in surface_findings:
        counts[f.severity] = counts.get(f.severity, 0) + 1
    lines.append(
        f"- **Blockers:** {counts[SEVERITY_BLOCKER]} · **Warnings:** "
        f"{counts[SEVERITY_WARNING]} · **Alias migrations:** {counts[SEVERITY_ALIAS]}"
    )
    if not surface_findings:
        lines.append("- All subjects resolve to one canonical term on every surface.")
    else:
        lines.append("")
        lines.append("| Severity | Check | Message |")
        lines.append("|---|---|---|")
        for f in surface_findings:
            msg = f.message.replace("|", "\\|")
            lines.append(f"| {f.severity} | `{f.check_id}` | {msg} |")
    lines.append("")

    lines.append("## Conformance drills")
    lines.append("")
    lines.append(
        "Each drill injects a known drift and asserts the gate fires. A green row "
        "means the gate still catches that class of drift."
    )
    lines.append("")
    lines.append("| Drill | Expected finding | Gate fires |")
    lines.append("|---|---|---|")
    failed_drills = {
        f.details.get("drill_id")
        for f in conformance_findings
        if f.check_id == "conformance.drill_did_not_fire"
    }
    for drill in conformance_corpus.get("drills") or []:
        exp = drill["expected_finding"]
        fires = "no" if drill["drill_id"] in failed_drills else "yes"
        lines.append(
            f"| `{drill['drill_id']}` | {exp['severity']} / `{exp['check_id']}` | "
            f"{fires} |"
        )
    lines.append("")

    lines.append("## How to refresh")
    lines.append("")
    lines.append("```bash")
    lines.append("scripts/ci/run_truth_vocabulary_parity.sh --write")
    lines.append("```")
    lines.append("")
    return "\n".join(lines)


# --------------------------------------------------------------------------- #
# Main
# --------------------------------------------------------------------------- #


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--registry", default=REGISTRY_REL)
    parser.add_argument("--report", default=REPORT_REL)
    parser.add_argument(
        "--write",
        action="store_true",
        help="Regenerate the parity report instead of drift-checking it.",
    )
    parser.add_argument(
        "--report-json",
        default=None,
        help="Write a machine-readable findings packet to this path.",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    registry_raw = load_yaml(repo_root / args.registry)
    if not isinstance(registry_raw, dict):
        fail("registry must be a YAML mapping")

    findings: list[Finding] = []
    reg = build_registry(repo_root, registry_raw, findings)

    surface_corpus = load_json(repo_root / SURFACE_CORPUS_REL)
    conformance_corpus = load_json(repo_root / CONFORMANCE_CORPUS_REL)

    surface_findings = run_surface_corpus(reg, surface_corpus)
    conformance_findings = run_conformance_corpus(reg, conformance_corpus)
    doc_findings = check_public_doc(repo_root, reg)

    findings.extend(surface_findings)
    findings.extend(conformance_findings)
    findings.extend(doc_findings)

    # Report drift / regeneration.
    report_text = render_report(
        reg, surface_findings, conformance_corpus, conformance_findings
    )
    report_path = repo_root / args.report
    if args.write:
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(report_text, encoding="utf-8")
        print(f"[truth-vocabulary] wrote {args.report}")
    else:
        on_disk = (
            report_path.read_text(encoding="utf-8") if report_path.exists() else None
        )
        if on_disk != report_text:
            findings.append(
                Finding(
                    severity=SEVERITY_BLOCKER,
                    check_id="report.drift",
                    message=(
                        f"{args.report} is stale; regenerate with --write"
                    ),
                    remediation="Run scripts/ci/run_truth_vocabulary_parity.sh --write.",
                    ref=args.report,
                )
            )

    blockers = [f for f in findings if f.severity == SEVERITY_BLOCKER]
    warnings = [f for f in findings if f.severity == SEVERITY_WARNING]
    alias_migrations = [f for f in findings if f.severity == SEVERITY_ALIAS]

    status = "fail" if blockers else "pass"
    if args.report_json:
        packet = {
            "record_kind": "truth_vocabulary_parity_report",
            "schema_version": 1,
            "status": status,
            "as_of": reg.as_of.isoformat(),
            "counts": {
                "blocker": len(blockers),
                "warning": len(warnings),
                "alias_migration": len(alias_migrations),
            },
            "findings": [f.as_report() for f in findings],
        }
        json_path = repo_root / args.report_json
        json_path.parent.mkdir(parents=True, exist_ok=True)
        json_path.write_text(json.dumps(packet, indent=2) + "\n", encoding="utf-8")

    # Console summary.
    for f in findings:
        stream = sys.stderr if f.severity == SEVERITY_BLOCKER else sys.stdout
        print(f"[{f.severity}] {f.check_id}: {f.message}", file=stream)

    print(
        f"[truth-vocabulary] {status}: "
        f"{len(blockers)} blocker(s), {len(warnings)} warning(s), "
        f"{len(alias_migrations)} alias migration(s)"
    )
    return 1 if blockers else 0


if __name__ == "__main__":
    raise SystemExit(main())
