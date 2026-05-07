#!/usr/bin/env python3
"""Scan docs- and guidance-bearing artifacts for snippet freshness drift.

This tool is the executable companion to:

- docs/docs/stale_example_ci.md
- artifacts/docs/snippet_freshness_ledger.yaml
- ci/check_docs_freshness.yml

It walks configured docs-pack manifests, guided-step fixtures, migration
examples, and browser/provider handoff packets, then validates that each
observed snippet/example:

- has a stable snippet id;
- carries the required source/version and compatibility anchors; and
- matches the expected values recorded in the snippet freshness ledger,
  or is covered by an explicit exception window.

The scanner is intentionally lightweight: it uses Python stdlib for
control flow and Ruby's Psych YAML parser for YAML decoding (matching
other repository validation tools).
"""

from __future__ import annotations

import argparse
import datetime as dt
import fnmatch
import hashlib
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any, Iterable


@dataclass(frozen=True)
class Observation:
    snippet_id: str
    snippet_kind: str
    source_path: str
    locator: str | None
    metadata: dict[str, Any] = field(default_factory=dict)


@dataclass
class Finding:
    severity: str
    failure_class: str
    message: str
    snippet_id: str | None = None
    source_path: str | None = None
    locator: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if not payload["details"]:
            payload.pop("details")
        if payload["snippet_id"] is None:
            payload.pop("snippet_id")
        if payload["source_path"] is None:
            payload.pop("source_path")
        if payload["locator"] is None:
            payload.pop("locator")
        return payload


FAILURE_UNTRACKED_SNIPPET = "untracked_snippet"
FAILURE_MISSING_SOURCE_ANCHOR = "missing_source_anchor"
FAILURE_VERSION_MISMATCH = "version_mismatch"
FAILURE_DRIFTED_COMMAND_ID = "drifted_command_id"
FAILURE_STALE_SCREENSHOT_SAFE_COPY = "stale_screenshot_safe_copy"
FAILURE_STALE_MIGRATION_EXAMPLE = "stale_migration_example"
FAILURE_STALE_PROVIDER_HANDOFF_GUIDANCE = "stale_provider_browser_handoff_guidance"

FAILURE_INVALID_CONFIG = "invalid_config"
FAILURE_INVALID_LEDGER = "invalid_ledger"
FAILURE_PARSE_ERROR = "parse_error"

POSTURE_VERIFIED = "verified"
POSTURE_ILLUSTRATIVE = "illustrative"
POSTURE_RETEST_PENDING = "retest_pending"


def render_yaml_as_json(path: Path) -> Any:
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), permitted_classes: [], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path} via Ruby/Psych: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


def parse_rfc3339(value: str) -> dt.datetime:
    raw = value.strip()
    if raw.endswith("Z"):
        raw = raw[:-1] + "+00:00"
    parsed = dt.datetime.fromisoformat(raw)
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=dt.timezone.utc)
    return parsed.astimezone(dt.timezone.utc)


def sha256_text(text: str) -> str:
    digest = hashlib.sha256(text.encode("utf-8")).hexdigest()
    return f"sha256:{digest}"


def relpath(repo_root: Path, path: Path) -> str:
    try:
        return path.relative_to(repo_root).as_posix()
    except ValueError:
        return str(path)


def expand_scan_paths(repo_root: Path, patterns: list[str]) -> list[Path]:
    paths: list[Path] = []
    for pattern in patterns:
        raw = pattern.strip()
        if not raw:
            continue
        if any(ch in raw for ch in "*?[]"):
            paths.extend(sorted(repo_root.glob(raw)))
        else:
            path = repo_root / raw
            if path.exists():
                paths.append(path)
    unique: dict[str, Path] = {}
    for path in paths:
        unique[str(path)] = path
    return sorted(unique.values(), key=lambda p: str(p))


def iter_root_files(
    repo_root: Path,
    roots: list[str],
    include_suffixes: set[str],
    exclude_globs: list[str],
) -> Iterable[Path]:
    excludes = [g for g in (exclude_globs or []) if isinstance(g, str) and g.strip()]
    for root in roots:
        path = repo_root / root
        if not path.exists():
            continue
        for candidate in sorted(path.rglob("*")):
            if not candidate.is_file():
                continue
            if include_suffixes and candidate.suffix not in include_suffixes:
                continue
            rel = relpath(repo_root, candidate)
            if any(fnmatch.fnmatch(rel, glob) for glob in excludes):
                continue
            yield candidate


def extract_from_docs_pack_manifest(path: Path, payload: dict[str, Any], repo_root: Path) -> list[Observation]:
    pack_id = payload.get("pack_id")
    pack_revision_ref = payload.get("pack_revision_ref")
    display_version = payload.get("display_version")
    semver_version = payload.get("semver_version")
    target_running_build_identity_ref = payload.get("target_running_build_identity_ref")
    compat_window_semver = payload.get("compat_window_semver")
    example_summary = payload.get("example_summary") or {}
    observations: list[Observation] = []
    for label in ("stable", "stale", "needs_review", "quarantined"):
        entries = example_summary.get(label) or []
        if not isinstance(entries, list):
            continue
        for idx, entry in enumerate(entries):
            if not isinstance(entry, dict):
                continue
            example_id = entry.get("example_id")
            if not example_id:
                continue
            locator = f"docs_pack_manifest_record.example_summary.{label}[{idx}]"
            observations.append(
                Observation(
                    snippet_id=str(example_id),
                    snippet_kind="docs_pack_example",
                    source_path=relpath(repo_root, path),
                    locator=locator,
                    metadata={
                        "record_kind": "docs_pack_manifest_record",
                        "pack_id": pack_id,
                        "pack_revision_ref": pack_revision_ref,
                        "display_version": display_version,
                        "semver_version": semver_version,
                        "target_running_build_identity_ref": target_running_build_identity_ref,
                        "compat_window_semver": compat_window_semver,
                        "label_class": entry.get("label_class"),
                        "stale_reason": entry.get("stale_reason"),
                        "target_symbol_or_anchor_ref": entry.get("target_symbol_or_anchor_ref"),
                        "superseding_example_id": entry.get("superseding_example_id"),
                    },
                )
            )
    return observations


def extract_from_guided_tour_records(path: Path, payload: dict[str, Any], repo_root: Path) -> list[Observation]:
    records = payload.get("records")
    if not isinstance(records, list):
        return []
    observations: list[Observation] = []
    for idx, record in enumerate(records):
        if not isinstance(record, dict):
            continue
        if record.get("record_kind") != "tour_waypoint_record":
            continue
        waypoint_id = record.get("waypoint_id")
        if not waypoint_id:
            continue
        anchor = record.get("anchor") or {}
        if not isinstance(anchor, dict):
            anchor = {}
        locator = f"guided_tour.records[{idx}]"
        observations.append(
            Observation(
                snippet_id=str(waypoint_id),
                snippet_kind="guided_step",
                source_path=relpath(repo_root, path),
                locator=locator,
                metadata={
                    "record_kind": "tour_waypoint_record",
                    "anchor_kind": anchor.get("anchor_kind"),
                    "command_id_ref": anchor.get("command_id_ref"),
                    "upstream_citation_anchor_refs": record.get("upstream_citation_anchor_refs") or [],
                    "docs_pack_ref": record.get("docs_pack_ref"),
                    "docs_pack_revision_ref": record.get("docs_pack_revision_ref"),
                    "guided_surface_ref": record.get("guided_surface_ref"),
                    "freshness_class": record.get("freshness_class"),
                    "version_match_state_at_mint": record.get("version_match_state_at_mint"),
                    "minted_at": record.get("minted_at"),
                },
            )
        )
    return observations


def extract_from_rollback_checkpoint_example(path: Path, payload: dict[str, Any], repo_root: Path) -> list[Observation]:
    if payload.get("record_kind") != "rollback_checkpoint_example_record":
        return []
    example_id = payload.get("example_id")
    if not example_id:
        return []
    return [
        Observation(
            snippet_id=str(example_id),
            snippet_kind="migration_example",
            source_path=relpath(repo_root, path),
            locator="rollback_checkpoint_example_record",
            metadata={
                "record_kind": "rollback_checkpoint_example_record",
                "schema_version": payload.get("schema_version"),
                "compatibility_report_ref": payload.get("compatibility_report_ref"),
                "compatibility_row_refs": payload.get("compatibility_row_refs") or [],
                "freshness_class": payload.get("freshness_class"),
            },
        )
    ]


def extract_from_assist_handoff(path: Path, payload: dict[str, Any], repo_root: Path) -> list[Observation]:
    if payload.get("record_kind") != "assist_help_handoff_record":
        return []
    handoff_id = payload.get("handoff_id")
    if not handoff_id:
        return []
    source_snapshot = payload.get("source_snapshot") or {}
    if not isinstance(source_snapshot, dict):
        source_snapshot = {}
    return [
        Observation(
            snippet_id=str(handoff_id),
            snippet_kind="provider_handoff",
            source_path=relpath(repo_root, path),
            locator="assist_help_handoff_record",
            metadata={
                "record_kind": "assist_help_handoff_record",
                "pack_id": source_snapshot.get("pack_id"),
                "pack_revision_ref": source_snapshot.get("pack_revision_ref"),
                "citation_anchor_refs": source_snapshot.get("citation_anchor_refs") or [],
                "target_build_identity_ref": source_snapshot.get("target_build_identity_ref"),
                "freshness_class_at_mint": source_snapshot.get("freshness_class_at_mint"),
                "version_match_state_at_mint": source_snapshot.get("version_match_state_at_mint"),
                "browser_handoff_reason": payload.get("browser_handoff_reason"),
            },
        )
    ]


def extract_from_assist_reference(path: Path, payload: dict[str, Any], repo_root: Path) -> list[Observation]:
    if payload.get("record_kind") != "assist_reference_record":
        return []
    reference_id = payload.get("assist_reference_id")
    if not reference_id:
        return []
    return [
        Observation(
            snippet_id=str(reference_id),
            snippet_kind="provider_handoff",
            source_path=relpath(repo_root, path),
            locator="assist_reference_record",
            metadata={
                "record_kind": "assist_reference_record",
                "pack_id": payload.get("pack_id"),
                "pack_revision_ref": payload.get("pack_revision_ref"),
                "citation_anchor_refs": payload.get("citation_anchor_refs") or [],
                "target_build_identity_ref": payload.get("target_build_identity_ref"),
                "freshness_class_at_mint": payload.get("freshness_class_at_mint"),
                "version_match_state_at_mint": payload.get("version_match_state_at_mint"),
                "browser_handoff_reason": payload.get("browser_handoff_reason"),
            },
        )
    ]


def extract_from_markdown(path: Path, text: str, repo_root: Path) -> list[Observation]:
    observations: list[Observation] = []
    lines = text.splitlines()
    idx = 0
    while idx < len(lines):
        line = lines[idx].strip()
        if not line.startswith("<!-- aureline-snippet:"):
            idx += 1
            continue
        header = line.removeprefix("<!-- aureline-snippet:").removesuffix("-->").strip()
        attrs: dict[str, str] = {}
        for chunk in header.split():
            if "=" not in chunk:
                continue
            key, value = chunk.split("=", 1)
            attrs[key.strip()] = value.strip()
        snippet_id = attrs.get("id")
        snippet_kind = attrs.get("kind") or "markdown_snippet"
        start_idx = idx + 1
        end_idx = start_idx
        while end_idx < len(lines) and lines[end_idx].strip() != "<!-- /aureline-snippet -->":
            end_idx += 1
        content = "\n".join(lines[start_idx:end_idx]).strip() + "\n"
        if snippet_id:
            observations.append(
                Observation(
                    snippet_id=snippet_id,
                    snippet_kind=snippet_kind,
                    source_path=relpath(repo_root, path),
                    locator=f"markdown:{idx + 1}",
                    metadata={"content_sha256": sha256_text(content)},
                )
            )
        idx = end_idx + 1
    return observations


def extract_observations(repo_root: Path, path: Path) -> tuple[list[Observation], list[Finding]]:
    findings: list[Finding] = []
    suffix = path.suffix.lower()
    try:
        if suffix == ".json":
            payload = json.loads(path.read_text(encoding="utf-8"))
            if not isinstance(payload, dict):
                return ([], [])
            record_kind = payload.get("record_kind")
            if record_kind == "docs_pack_manifest_record":
                return (extract_from_docs_pack_manifest(path, payload, repo_root), [])
            if record_kind == "assist_help_handoff_record":
                return (extract_from_assist_handoff(path, payload, repo_root), [])
            if record_kind == "assist_reference_record":
                return (extract_from_assist_reference(path, payload, repo_root), [])
            return ([], [])
        if suffix in (".yaml", ".yml"):
            payload = render_yaml_as_json(path)
            if not isinstance(payload, dict):
                return ([], [])
            observations: list[Observation] = []
            observations.extend(extract_from_guided_tour_records(path, payload, repo_root))
            observations.extend(extract_from_rollback_checkpoint_example(path, payload, repo_root))
            return (observations, [])
        if suffix == ".md":
            text = path.read_text(encoding="utf-8")
            return (extract_from_markdown(path, text, repo_root), [])
        return ([], [])
    except Exception as exc:  # noqa: BLE001
        findings.append(
            Finding(
                severity="error",
                failure_class=FAILURE_PARSE_ERROR,
                message=f"failed to parse {relpath(repo_root, path)}: {exc}",
                source_path=relpath(repo_root, path),
                details={"exception": type(exc).__name__},
            )
        )
        return ([], findings)


def as_dict(value: Any, name: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{name} must be a YAML mapping/object")
    return value


def as_list(value: Any, name: str) -> list[Any]:
    if value is None:
        return []
    if not isinstance(value, list):
        raise SystemExit(f"{name} must be a YAML array/list")
    return value


def load_config(repo_root: Path, config_path: Path) -> dict[str, Any]:
    payload = render_yaml_as_json(config_path)
    config = as_dict(payload, "check_docs_freshness config")
    ledger_path = config.get("ledger_path")
    if not isinstance(ledger_path, str) or not ledger_path.strip():
        raise SystemExit("check_docs_freshness config must set ledger_path")
    config["ledger_path"] = ledger_path.strip()
    scan_paths = config.get("scan_paths")
    scan_roots = config.get("scan_roots")
    if scan_paths is None and scan_roots is None:
        raise SystemExit("check_docs_freshness config must set scan_paths or scan_roots")
    if scan_paths is not None:
        config["scan_paths"] = [str(p) for p in as_list(scan_paths, "scan_paths")]
    if scan_roots is not None:
        config["scan_roots"] = [str(p) for p in as_list(scan_roots, "scan_roots")]
    include_suffixes = config.get("include_suffixes") or [".json", ".yaml", ".yml", ".md"]
    if not isinstance(include_suffixes, list) or not include_suffixes:
        raise SystemExit("include_suffixes must be a non-empty list")
    config["include_suffixes"] = [str(s) for s in include_suffixes]
    config["exclude_globs"] = [str(s) for s in as_list(config.get("exclude_globs"), "exclude_globs")]
    require_kinds = config.get("require_ledger_for_kinds")
    if require_kinds is None:
        config["require_ledger_for_kinds"] = []
    else:
        config["require_ledger_for_kinds"] = [str(s) for s in as_list(require_kinds, "require_ledger_for_kinds")]
    return config


def load_ledger(repo_root: Path, ledger_path: Path) -> dict[str, Any]:
    payload = render_yaml_as_json(ledger_path)
    ledger = as_dict(payload, "snippet freshness ledger")
    entries = ledger.get("entries")
    if not isinstance(entries, list):
        raise SystemExit("snippet freshness ledger must contain entries: []")
    index: dict[str, dict[str, Any]] = {}
    duplicates: list[str] = []
    for entry in entries:
        if not isinstance(entry, dict):
            continue
        snippet_id = entry.get("snippet_id")
        if not snippet_id:
            continue
        sid = str(snippet_id)
        if sid in index:
            duplicates.append(sid)
        index[sid] = entry
    if duplicates:
        raise SystemExit(
            "snippet freshness ledger contains duplicate snippet_id values: " + ", ".join(sorted(set(duplicates)))
        )
    ledger["__index__"] = index
    return ledger


def active_exception(entry: dict[str, Any], failure_class: str, now: dt.datetime) -> dict[str, Any] | None:
    windows = entry.get("exception_windows") or []
    if not isinstance(windows, list):
        return None
    for window in windows:
        if not isinstance(window, dict):
            continue
        until_raw = window.get("until")
        allowed = window.get("allowed_failure_classes") or []
        if not until_raw or not isinstance(allowed, list):
            continue
        if failure_class not in {str(a) for a in allowed}:
            continue
        try:
            until = parse_rfc3339(str(until_raw))
        except ValueError:
            continue
        if now <= until:
            return window
    return None


def severity_for(entry: dict[str, Any], failure_class: str, now: dt.datetime) -> str:
    window = active_exception(entry, failure_class, now)
    if window is not None:
        return "warn"
    posture = str(entry.get("posture_class") or POSTURE_VERIFIED)
    if posture == POSTURE_ILLUSTRATIVE and failure_class == FAILURE_VERSION_MISMATCH:
        return "warn"
    return "error"


def require_non_empty(value: Any) -> bool:
    if value is None:
        return False
    if isinstance(value, str):
        return bool(value.strip())
    if isinstance(value, list):
        return len(value) > 0
    return True


def validate_observation(
    obs: Observation,
    ledger_index: dict[str, dict[str, Any]],
    require_ledger_for_kinds: set[str],
    now: dt.datetime,
) -> list[Finding]:
    findings: list[Finding] = []
    entry = ledger_index.get(obs.snippet_id)
    if obs.snippet_kind in require_ledger_for_kinds and entry is None:
        findings.append(
            Finding(
                severity="error",
                failure_class=FAILURE_UNTRACKED_SNIPPET,
                message="snippet is present in scanned surfaces but missing from the freshness ledger",
                snippet_id=obs.snippet_id,
                source_path=obs.source_path,
                locator=obs.locator,
                details={"snippet_kind": obs.snippet_kind},
            )
        )
        return findings
    if entry is None:
        return findings

    expected_kind = entry.get("snippet_kind")
    if expected_kind and str(expected_kind) != obs.snippet_kind:
        findings.append(
            Finding(
                severity=severity_for(entry, FAILURE_INVALID_LEDGER, now),
                failure_class=FAILURE_INVALID_LEDGER,
                message="ledger snippet_kind does not match observed snippet_kind",
                snippet_id=obs.snippet_id,
                source_path=obs.source_path,
                locator=obs.locator,
                details={"expected": expected_kind, "observed": obs.snippet_kind},
            )
        )

    posture = str(entry.get("posture_class") or POSTURE_VERIFIED)
    if posture == POSTURE_RETEST_PENDING:
        sev = severity_for(entry, POSTURE_RETEST_PENDING, now)
        findings.append(
            Finding(
                severity=sev,
                failure_class=POSTURE_RETEST_PENDING,
                message="snippet is marked retest_pending and blocks promotion unless covered by an exception window",
                snippet_id=obs.snippet_id,
                source_path=obs.source_path,
                locator=obs.locator,
            )
        )

    if obs.snippet_kind == "docs_pack_example":
        for required_key in ("pack_id", "pack_revision_ref", "semver_version", "target_running_build_identity_ref"):
            if not require_non_empty(obs.metadata.get(required_key)):
                findings.append(
                    Finding(
                        severity=severity_for(entry, FAILURE_VERSION_MISMATCH, now),
                        failure_class=FAILURE_VERSION_MISMATCH,
                        message=f"missing required docs-pack version anchor field: {required_key}",
                        snippet_id=obs.snippet_id,
                        source_path=obs.source_path,
                        locator=obs.locator,
                    )
                )
        if obs.metadata.get("label_class") == "stable_example" and not require_non_empty(
            obs.metadata.get("target_symbol_or_anchor_ref")
        ):
            findings.append(
                Finding(
                    severity=severity_for(entry, FAILURE_MISSING_SOURCE_ANCHOR, now),
                    failure_class=FAILURE_MISSING_SOURCE_ANCHOR,
                    message="stable docs-pack example is missing target_symbol_or_anchor_ref",
                    snippet_id=obs.snippet_id,
                    source_path=obs.source_path,
                    locator=obs.locator,
                )
            )

        expected_pack_revision_ref = entry.get("expected_pack_revision_ref")
        if expected_pack_revision_ref and obs.metadata.get("pack_revision_ref") != expected_pack_revision_ref:
            findings.append(
                Finding(
                    severity=severity_for(entry, FAILURE_VERSION_MISMATCH, now),
                    failure_class=FAILURE_VERSION_MISMATCH,
                    message="docs-pack revision does not match ledger expectation",
                    snippet_id=obs.snippet_id,
                    source_path=obs.source_path,
                    locator=obs.locator,
                    details={
                        "expected_pack_revision_ref": expected_pack_revision_ref,
                        "observed_pack_revision_ref": obs.metadata.get("pack_revision_ref"),
                    },
                )
            )

    if obs.snippet_kind == "guided_step":
        anchor_kind = obs.metadata.get("anchor_kind")
        command_id_ref = obs.metadata.get("command_id_ref")
        upstream_refs = obs.metadata.get("upstream_citation_anchor_refs") or []
        if anchor_kind == "command_id_anchor" and not require_non_empty(command_id_ref):
            findings.append(
                Finding(
                    severity=severity_for(entry, FAILURE_DRIFTED_COMMAND_ID, now),
                    failure_class=FAILURE_DRIFTED_COMMAND_ID,
                    message="guided step is command anchored but command_id_ref is missing",
                    snippet_id=obs.snippet_id,
                    source_path=obs.source_path,
                    locator=obs.locator,
                )
            )
        if not isinstance(upstream_refs, list) or len(upstream_refs) == 0:
            findings.append(
                Finding(
                    severity=severity_for(entry, FAILURE_MISSING_SOURCE_ANCHOR, now),
                    failure_class=FAILURE_MISSING_SOURCE_ANCHOR,
                    message="guided step is missing upstream_citation_anchor_refs",
                    snippet_id=obs.snippet_id,
                    source_path=obs.source_path,
                    locator=obs.locator,
                )
            )
        if not require_non_empty(obs.metadata.get("docs_pack_revision_ref")):
            findings.append(
                Finding(
                    severity=severity_for(entry, FAILURE_VERSION_MISMATCH, now),
                    failure_class=FAILURE_VERSION_MISMATCH,
                    message="guided step is missing docs_pack_revision_ref",
                    snippet_id=obs.snippet_id,
                    source_path=obs.source_path,
                    locator=obs.locator,
                )
            )
        if not require_non_empty(obs.metadata.get("version_match_state_at_mint")):
            findings.append(
                Finding(
                    severity=severity_for(entry, FAILURE_VERSION_MISMATCH, now),
                    failure_class=FAILURE_VERSION_MISMATCH,
                    message="guided step is missing version_match_state_at_mint",
                    snippet_id=obs.snippet_id,
                    source_path=obs.source_path,
                    locator=obs.locator,
                )
            )
        expected_command_id_ref = entry.get("expected_command_id_ref")
        if expected_command_id_ref and command_id_ref != expected_command_id_ref:
            findings.append(
                Finding(
                    severity=severity_for(entry, FAILURE_DRIFTED_COMMAND_ID, now),
                    failure_class=FAILURE_DRIFTED_COMMAND_ID,
                    message="guided step command_id_ref drifted from ledger expectation",
                    snippet_id=obs.snippet_id,
                    source_path=obs.source_path,
                    locator=obs.locator,
                    details={"expected": expected_command_id_ref, "observed": command_id_ref},
                )
            )

    if obs.snippet_kind == "migration_example":
        compat_refs = obs.metadata.get("compatibility_row_refs") or []
        if not isinstance(compat_refs, list) or len(compat_refs) == 0:
            findings.append(
                Finding(
                    severity=severity_for(entry, FAILURE_STALE_MIGRATION_EXAMPLE, now),
                    failure_class=FAILURE_STALE_MIGRATION_EXAMPLE,
                    message="migration example is missing compatibility_row_refs",
                    snippet_id=obs.snippet_id,
                    source_path=obs.source_path,
                    locator=obs.locator,
                )
            )
        expected_compat_refs = entry.get("expected_compatibility_row_refs")
        if isinstance(expected_compat_refs, list):
            observed = [str(v) for v in compat_refs if v is not None]
            expected = [str(v) for v in expected_compat_refs]
            if observed != expected:
                findings.append(
                    Finding(
                        severity=severity_for(entry, FAILURE_STALE_MIGRATION_EXAMPLE, now),
                        failure_class=FAILURE_STALE_MIGRATION_EXAMPLE,
                        message="migration example compatibility anchors drifted from ledger expectation",
                        snippet_id=obs.snippet_id,
                        source_path=obs.source_path,
                        locator=obs.locator,
                        details={"expected": expected, "observed": observed},
                    )
                )

    if obs.snippet_kind == "provider_handoff":
        for key in ("pack_id", "pack_revision_ref", "target_build_identity_ref", "version_match_state_at_mint"):
            if not require_non_empty(obs.metadata.get(key)):
                findings.append(
                    Finding(
                        severity=severity_for(entry, FAILURE_STALE_PROVIDER_HANDOFF_GUIDANCE, now),
                        failure_class=FAILURE_STALE_PROVIDER_HANDOFF_GUIDANCE,
                        message=f"provider/browser handoff is missing required field: {key}",
                        snippet_id=obs.snippet_id,
                        source_path=obs.source_path,
                        locator=obs.locator,
                    )
                )
        citation_refs = obs.metadata.get("citation_anchor_refs") or []
        if not isinstance(citation_refs, list) or len(citation_refs) == 0:
            findings.append(
                Finding(
                    severity=severity_for(entry, FAILURE_MISSING_SOURCE_ANCHOR, now),
                    failure_class=FAILURE_MISSING_SOURCE_ANCHOR,
                    message="provider/browser handoff is missing citation_anchor_refs",
                    snippet_id=obs.snippet_id,
                    source_path=obs.source_path,
                    locator=obs.locator,
                )
            )
        expected_pack_revision_ref = entry.get("expected_pack_revision_ref")
        if expected_pack_revision_ref and obs.metadata.get("pack_revision_ref") != expected_pack_revision_ref:
            findings.append(
                Finding(
                    severity=severity_for(entry, FAILURE_STALE_PROVIDER_HANDOFF_GUIDANCE, now),
                    failure_class=FAILURE_STALE_PROVIDER_HANDOFF_GUIDANCE,
                    message="provider/browser handoff pack revision drifted from ledger expectation",
                    snippet_id=obs.snippet_id,
                    source_path=obs.source_path,
                    locator=obs.locator,
                    details={
                        "expected_pack_revision_ref": expected_pack_revision_ref,
                        "observed_pack_revision_ref": obs.metadata.get("pack_revision_ref"),
                    },
                )
            )
        expected_browser_handoff_reason = entry.get("expected_browser_handoff_reason")
        if expected_browser_handoff_reason and obs.metadata.get("browser_handoff_reason") != expected_browser_handoff_reason:
            findings.append(
                Finding(
                    severity=severity_for(entry, FAILURE_STALE_PROVIDER_HANDOFF_GUIDANCE, now),
                    failure_class=FAILURE_STALE_PROVIDER_HANDOFF_GUIDANCE,
                    message="provider/browser handoff reason drifted from ledger expectation",
                    snippet_id=obs.snippet_id,
                    source_path=obs.source_path,
                    locator=obs.locator,
                    details={
                        "expected_browser_handoff_reason": expected_browser_handoff_reason,
                        "observed_browser_handoff_reason": obs.metadata.get("browser_handoff_reason"),
                    },
                )
            )
        expected_version_match_state = entry.get("expected_version_match_state_at_mint")
        if expected_version_match_state and obs.metadata.get("version_match_state_at_mint") != expected_version_match_state:
            findings.append(
                Finding(
                    severity=severity_for(entry, FAILURE_VERSION_MISMATCH, now),
                    failure_class=FAILURE_VERSION_MISMATCH,
                    message="provider/browser handoff version_match_state_at_mint drifted from ledger expectation",
                    snippet_id=obs.snippet_id,
                    source_path=obs.source_path,
                    locator=obs.locator,
                    details={
                        "expected_version_match_state_at_mint": expected_version_match_state,
                        "observed_version_match_state_at_mint": obs.metadata.get("version_match_state_at_mint"),
                    },
                )
            )

    if obs.snippet_kind == "screenshot_safe_copy":
        expected_sha = entry.get("expected_content_sha256")
        observed_sha = obs.metadata.get("content_sha256")
        if expected_sha and observed_sha != expected_sha:
            findings.append(
                Finding(
                    severity=severity_for(entry, FAILURE_STALE_SCREENSHOT_SAFE_COPY, now),
                    failure_class=FAILURE_STALE_SCREENSHOT_SAFE_COPY,
                    message="screenshot-safe copy drifted from ledger expectation",
                    snippet_id=obs.snippet_id,
                    source_path=obs.source_path,
                    locator=obs.locator,
                    details={"expected_content_sha256": expected_sha, "observed_content_sha256": observed_sha},
                )
            )

    return findings


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo-root", type=Path, default=None)
    parser.add_argument("--config", type=Path, default=Path("ci/check_docs_freshness.yml"))
    parser.add_argument("--report", type=Path, default=None)
    args = parser.parse_args()

    repo_root = args.repo_root or Path(__file__).resolve().parents[3]
    repo_root = repo_root.resolve()

    config_path = args.config if args.config.is_absolute() else repo_root / args.config
    if not config_path.exists():
        print(f"[docs-freshness] error: config not found: {config_path}", file=sys.stderr)
        return 2

    now = dt.datetime.now(dt.timezone.utc)

    try:
        config = load_config(repo_root, config_path)
    except SystemExit as exc:
        print(f"[docs-freshness] error: {exc}", file=sys.stderr)
        return 2

    ledger_path = repo_root / config["ledger_path"]
    if not ledger_path.exists():
        print(f"[docs-freshness] error: ledger not found: {ledger_path}", file=sys.stderr)
        return 2

    try:
        ledger = load_ledger(repo_root, ledger_path)
    except SystemExit as exc:
        print(f"[docs-freshness] error: {exc}", file=sys.stderr)
        return 2

    ledger_index = ledger.get("__index__") or {}
    if not isinstance(ledger_index, dict):
        print("[docs-freshness] error: ledger index corruption", file=sys.stderr)
        return 2

    include_suffixes = {str(s) for s in config.get("include_suffixes") or []}
    exclude_globs = config.get("exclude_globs") or []
    require_kinds = {str(k) for k in (config.get("require_ledger_for_kinds") or [])}
    if not require_kinds:
        require_kinds = {"docs_pack_example", "guided_step", "migration_example", "provider_handoff", "screenshot_safe_copy"}

    scan_paths: list[Path] = []
    if "scan_paths" in config:
        scan_paths.extend(expand_scan_paths(repo_root, list(config["scan_paths"])))
    if "scan_roots" in config:
        scan_paths.extend(
            list(
                iter_root_files(
                    repo_root=repo_root,
                    roots=list(config["scan_roots"]),
                    include_suffixes=include_suffixes,
                    exclude_globs=list(exclude_globs),
                )
            )
        )
    unique: dict[str, Path] = {}
    for p in scan_paths:
        unique[str(p)] = p
    scan_paths = sorted(unique.values(), key=lambda p: str(p))

    observations: list[Observation] = []
    findings: list[Finding] = []
    for path in scan_paths:
        if include_suffixes and path.suffix not in include_suffixes:
            continue
        obs, parse_findings = extract_observations(repo_root, path)
        observations.extend(obs)
        findings.extend(parse_findings)

    for obs in observations:
        findings.extend(validate_observation(obs, ledger_index, require_kinds, now))

    error_count = sum(1 for f in findings if f.severity == "error")
    warn_count = sum(1 for f in findings if f.severity == "warn")

    report: dict[str, Any] = {
        "scanner": "docs_freshness_scan",
        "scanned_at": now.isoformat().replace("+00:00", "Z"),
        "config_path": relpath(repo_root, config_path),
        "ledger_path": relpath(repo_root, ledger_path),
        "scan_count": len(scan_paths),
        "observation_count": len(observations),
        "finding_count": len(findings),
        "error_count": error_count,
        "warn_count": warn_count,
        "findings": [f.as_report() for f in findings],
    }

    if args.report is not None:
        report_path = args.report if args.report.is_absolute() else repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    for finding in findings:
        prefix = f"[docs-freshness] {finding.severity}: {finding.failure_class}: {finding.message}"
        tail_bits = []
        if finding.source_path:
            tail_bits.append(str(finding.source_path))
        if finding.snippet_id:
            tail_bits.append(str(finding.snippet_id))
        if finding.locator:
            tail_bits.append(str(finding.locator))
        tail = " | ".join(tail_bits)
        if tail:
            prefix = f"{prefix} ({tail})"
        stream = sys.stderr if finding.severity == "error" else sys.stdout
        print(prefix, file=stream)

    if error_count:
        print(f"[docs-freshness] FAILED ({error_count} errors, {warn_count} warnings)", file=sys.stderr)
        return 1

    print(f"[docs-freshness] OK ({warn_count} warnings, {len(observations)} observations)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

