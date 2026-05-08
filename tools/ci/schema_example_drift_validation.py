#!/usr/bin/env python3
"""Validate drift between protected schemas, canonical examples, and docs snippets.

This validator is intentionally scoped by a small, reviewable source map:
`artifacts/ci/contract_example_sources.yaml`.

Exit code is 0 when every check passes and 1 when any finding is at severity
`error`.
"""

from __future__ import annotations

import datetime as dt
import hashlib
import json
import os
import re
import subprocess
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any, Iterable


SOURCE_MAP_REL = "artifacts/ci/contract_example_sources.yaml"
EXAMPLE_PACK_INDEX_REL = "artifacts/contracts/example_pack_index.yaml"

IGNORED_CHANGED_FILE_PREFIXES = ("target/",)
IGNORED_CHANGED_FILE_PARTS = ("__pycache__",)
IGNORED_CHANGED_FILE_SUFFIXES = (".pyc",)

AURELINE_SCHEMA_PREFIX = "https://aureline.dev/schemas/"
SENTINEL_REFS = {"not_yet_seeded", "outline_only", "contract_not_yet_seeded"}


@dataclass
class Finding:
    severity: str
    check_id: str
    artifact_ref: str
    message: str
    remediation: str
    row_ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if not payload["details"]:
            payload.pop("details")
        if payload["row_ref"] is None:
            payload.pop("row_ref")
        return payload


def now_utc() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def strip_fixture_metadata(value: Any) -> Any:
    if isinstance(value, dict):
        stripped: dict[str, Any] = {}
        for key, item in value.items():
            if key in {"$schema", "__fixture__"}:
                continue
            stripped[key] = strip_fixture_metadata(item)
        return stripped
    if isinstance(value, list):
        return [strip_fixture_metadata(item) for item in value]
    return value


def iter_instances(payload: Any) -> Iterable[tuple[Any, str]]:
    if isinstance(payload, dict):
        records = payload.get("records")
        if isinstance(records, list):
            for idx, record in enumerate(records):
                yield record, f"records[{idx}]"
            return
    if isinstance(payload, list):
        for idx, item in enumerate(payload):
            yield item, f"[{idx}]"
        return
    yield payload, "<root>"


def run_git_lines(repo_root: Path, args: list[str]) -> list[str]:
    proc = subprocess.run(
        ["git", "-C", str(repo_root), *args],
        capture_output=True,
        text=True,
    )
    if proc.returncode != 0:
        return []
    return sorted({line.strip() for line in proc.stdout.splitlines() if line.strip()})


def filter_changed_files(paths: list[str]) -> list[str]:
    filtered: set[str] = set()
    for raw_path in paths:
        path = Path(raw_path).as_posix()
        if path.startswith(IGNORED_CHANGED_FILE_PREFIXES):
            continue
        if path.endswith(IGNORED_CHANGED_FILE_SUFFIXES):
            continue
        if any(part in IGNORED_CHANGED_FILE_PARTS for part in Path(path).parts):
            continue
        filtered.add(path)
    return sorted(filtered)


def detect_changed_files(repo_root: Path, scenario: dict[str, Any] | None) -> list[str]:
    if scenario is not None and scenario.get("changed_files"):
        changed_files = scenario.get("changed_files")
        if not isinstance(changed_files, list) or not all(
            isinstance(item, str) and item.strip() for item in changed_files
        ):
            raise SystemExit("scenario changed_files must be a list of non-empty strings")
        return filter_changed_files([Path(path).as_posix() for path in changed_files])

    changed: set[str] = set()
    for args in (
        ["diff", "--name-only", "--relative", "--"],
        ["diff", "--name-only", "--relative", "--cached", "--"],
        ["ls-files", "--others", "--exclude-standard"],
    ):
        changed.update(run_git_lines(repo_root, args))

    if changed:
        return filter_changed_files(sorted(changed))

    if os.getenv("GITHUB_ACTIONS") != "true" and os.getenv("CI") != "true":
        return []

    head_parent = subprocess.run(
        ["git", "-C", str(repo_root), "rev-parse", "--verify", "HEAD^1"],
        capture_output=True,
        text=True,
    )
    if head_parent.returncode != 0:
        return []

    changed.update(run_git_lines(repo_root, ["diff", "--name-only", "--relative", "HEAD^1", "HEAD", "--"]))
    return filter_changed_files(sorted(changed))


def load_yaml(path: Path, *, scenario: dict[str, Any] | None, repo_root: Path) -> Any:
    try:
        import yaml  # type: ignore
    except Exception as exc:  # pragma: no cover
        raise SystemExit(f"python PyYAML is required to parse {path.relative_to(repo_root)}: {exc}") from exc

    rel = path.relative_to(repo_root).as_posix()
    overrides = (scenario or {}).get("file_overrides") or {}
    if isinstance(overrides, dict) and isinstance(overrides.get(rel), str):
        return yaml.safe_load(overrides[rel])
    return yaml.safe_load(path.read_text(encoding="utf-8"))


def load_json(path: Path, *, scenario: dict[str, Any] | None, repo_root: Path) -> Any:
    rel = path.relative_to(repo_root).as_posix()
    overrides = (scenario or {}).get("file_overrides") or {}
    if isinstance(overrides, dict) and isinstance(overrides.get(rel), str):
        return json.loads(overrides[rel])
    return json.loads(path.read_text(encoding="utf-8"))


def read_bytes(path: Path, *, scenario: dict[str, Any] | None, repo_root: Path) -> bytes:
    rel = path.relative_to(repo_root).as_posix()
    overrides = (scenario or {}).get("file_overrides") or {}
    if isinstance(overrides, dict) and isinstance(overrides.get(rel), str):
        return overrides[rel].encode("utf-8")
    return path.read_bytes()


def is_path_ref(value: str) -> bool:
    if not value:
        return False
    if value in SENTINEL_REFS:
        return False
    if "://" in value:
        return False
    return "/" in value or value.endswith((".json", ".yaml", ".yml", ".md"))


def parse_snippet_blocks(markdown: str) -> list[tuple[dict[str, str], str, int]]:
    blocks: list[tuple[dict[str, str], str, int]] = []
    lines = markdown.splitlines()
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
        start_idx = idx + 1
        end_idx = start_idx
        while end_idx < len(lines) and lines[end_idx].strip() != "<!-- /aureline-snippet -->":
            end_idx += 1
        content = "\n".join(lines[start_idx:end_idx]).strip() + "\n"
        blocks.append((attrs, content, idx + 1))
        idx = end_idx + 1
    return blocks


def extract_first_fenced_block(snippet: str) -> tuple[str | None, str]:
    lines = snippet.splitlines()
    fence_start = None
    fence_lang = None
    for idx, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith("```"):
            fence_start = idx
            fence_lang = stripped.removeprefix("```").strip() or None
            break
    if fence_start is None:
        return None, snippet.strip()
    for end_idx in range(fence_start + 1, len(lines)):
        if lines[end_idx].strip().startswith("```"):
            body = "\n".join(lines[fence_start + 1 : end_idx]).strip()
            return fence_lang, body
    return fence_lang, "\n".join(lines[fence_start + 1 :]).strip()


def find_snippet_payload(
    repo_root: Path,
    doc_ref: str,
    snippet_id: str,
    *,
    scenario: dict[str, Any] | None,
) -> tuple[Any, str | None, int] | None:
    doc_path = repo_root / doc_ref
    if not doc_path.exists():
        return None
    markdown = (scenario or {}).get("file_overrides", {}).get(doc_ref)
    if isinstance(markdown, str):
        text = markdown
    else:
        text = doc_path.read_text(encoding="utf-8")
    for attrs, content, lineno in parse_snippet_blocks(text):
        if attrs.get("id") != snippet_id:
            continue
        lang, body = extract_first_fenced_block(content)
        try:
            if (lang or "").lower() == "json":
                return json.loads(body), lang, lineno
            import yaml  # type: ignore

            return yaml.safe_load(body), lang, lineno
        except Exception:
            return None, lang, lineno
    return None


def retrieve_aureline_schema(uri: str, repo_root: Path) -> Any:
    from referencing import Resource  # type: ignore
    from referencing.exceptions import NoSuchResource  # type: ignore
    from referencing.jsonschema import DRAFT202012  # type: ignore

    if not uri.startswith(AURELINE_SCHEMA_PREFIX):
        raise NoSuchResource(ref=uri)
    rel = uri.removeprefix(AURELINE_SCHEMA_PREFIX)
    candidate = repo_root / "schemas" / rel
    if not candidate.exists():
        raise NoSuchResource(ref=uri)
    contents = json.loads(candidate.read_text(encoding="utf-8"))
    return Resource.from_contents(contents, default_specification=DRAFT202012)


def validate_examples_against_schemas(
    repo_root: Path,
    example_rows: dict[str, dict[str, Any]],
    protected_examples: list[dict[str, Any]],
    *,
    scenario: dict[str, Any] | None,
) -> list[Finding]:
    try:
        from jsonschema import Draft202012Validator  # type: ignore
        from referencing import Registry  # type: ignore
    except Exception as exc:  # pragma: no cover
        raise SystemExit(f"python jsonschema + referencing are required: {exc}") from exc

    registry = Registry(retrieve=lambda uri: retrieve_aureline_schema(uri, repo_root))
    validator_cache: dict[str, Draft202012Validator] = {}
    findings: list[Finding] = []

    def clip(message: str, limit: int = 480) -> str:
        if len(message) <= limit:
            return message
        return message[:limit].rstrip() + "… (truncated)"

    for entry in protected_examples:
        example_id = entry.get("example_id")
        if not isinstance(example_id, str) or not example_id:
            continue
        row = example_rows.get(example_id)
        if row is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id="schema_example_drift.example_id_resolves",
                    artifact_ref=SOURCE_MAP_REL,
                    message=f"protected example_id does not resolve in {EXAMPLE_PACK_INDEX_REL}: {example_id}",
                    remediation="Add the example to the example pack index or remove it from the protected source map.",
                    row_ref=example_id,
                )
            )
            continue

        payload_ref = row.get("payload_ref")
        schema_ref = row.get("schema_ref")
        if not isinstance(payload_ref, str) or not is_path_ref(payload_ref):
            continue
        if not isinstance(schema_ref, str) or not is_path_ref(schema_ref):
            continue

        payload_path = repo_root / payload_ref
        schema_path = repo_root / schema_ref
        if not payload_path.exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id="schema_example_drift.payload_exists",
                    artifact_ref=payload_ref,
                    message=f"example payload file is missing: {payload_ref}",
                    remediation="Restore the payload file or update the example pack index.",
                    row_ref=example_id,
                )
            )
            continue
        if not schema_path.exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id="schema_example_drift.schema_exists",
                    artifact_ref=schema_ref,
                    message=f"example schema file is missing: {schema_ref}",
                    remediation="Restore the schema file or update the example pack index.",
                    row_ref=example_id,
                )
            )
            continue

        try:
            if payload_path.suffix.lower() == ".json":
                payload = load_json(payload_path, scenario=scenario, repo_root=repo_root)
            else:
                payload = load_yaml(payload_path, scenario=scenario, repo_root=repo_root)
        except Exception as exc:  # noqa: BLE001
            findings.append(
                Finding(
                    severity="error",
                    check_id="schema_example_drift.payload_parses",
                    artifact_ref=payload_ref,
                    message=f"failed to parse payload: {exc}",
                    remediation="Fix the payload so it parses as JSON/YAML.",
                    row_ref=example_id,
                    details={"payload_ref": payload_ref},
                )
            )
            continue

        try:
            schema = load_json(schema_path, scenario=scenario, repo_root=repo_root)
        except Exception as exc:  # noqa: BLE001
            findings.append(
                Finding(
                    severity="error",
                    check_id="schema_example_drift.schema_parses",
                    artifact_ref=schema_ref,
                    message=f"failed to parse schema JSON: {exc}",
                    remediation="Fix the schema so it parses as JSON.",
                    row_ref=example_id,
                    details={"schema_ref": schema_ref},
                )
            )
            continue

        cache_key = schema_ref
        validator = validator_cache.get(cache_key)
        if validator is None:
            validator = Draft202012Validator(schema, registry=registry)
            validator_cache[cache_key] = validator

        for instance, where in iter_instances(payload):
            instance = strip_fixture_metadata(instance)
            errors = sorted(validator.iter_errors(instance), key=lambda e: list(e.path))
            for err in errors[:50]:
                loc = ".".join(map(str, err.path)) or "<root>"
                err_msg = clip(str(err.message))
                findings.append(
                    Finding(
                        severity="error",
                        check_id="schema_example_drift.example_payload_schema_valid",
                        artifact_ref=payload_ref,
                        message=f"{payload_ref}:{where}:{loc}: {example_id}: {err_msg}",
                        remediation="Update the example payload (or schema) so the protected example remains valid.",
                        row_ref=example_id,
                        details={
                            "schema_ref": schema_ref,
                            "validator": getattr(err, "validator", None),
                            "instance_path": list(err.path),
                            "schema_path": list(err.schema_path),
                        },
                    )
                )

    return findings


def render_human_summary(findings: list[Finding], analysis: dict[str, Any]) -> str:
    lines: list[str] = []
    lines.append("[schema-example-drift] summary")
    lines.append(f"  observed_at: {analysis.get('observed_at')}")
    lines.append(f"  changed_file_count: {analysis.get('changed_file_count')}")
    if analysis.get("changed_files"):
        lines.append("  changed_files:")
        for path in analysis["changed_files"]:
            lines.append(f"    - {path}")
    if not findings:
        lines.append("[schema-example-drift] OK")
        return "\n".join(lines) + "\n"
    lines.append("[schema-example-drift] FAIL")
    for finding in findings[:200]:
        row = f" ({finding.row_ref})" if finding.row_ref else ""
        lines.append(f"  - {finding.check_id}{row}: {finding.message}")
        if finding.remediation:
            lines.append(f"    remediation: {finding.remediation}")
    if len(findings) > 200:
        lines.append(f"  ... ({len(findings) - 200} more)")
    return "\n".join(lines) + "\n"


def validate_schema_example_drift(
    repo_root: Path,
    *,
    source_map_path: str | None = None,
    scenario: dict[str, Any] | None = None,
) -> tuple[list[Finding], dict[str, Any]]:
    findings: list[Finding] = []
    changed_files = detect_changed_files(repo_root, scenario)

    effective_source_map_rel = source_map_path or (scenario or {}).get("source_map_path") or SOURCE_MAP_REL
    source_map_path_abs = repo_root / str(effective_source_map_rel)
    if not source_map_path_abs.exists():
        raise SystemExit(f"source map not found: {effective_source_map_rel}")

    source_map = load_yaml(source_map_path_abs, scenario=scenario, repo_root=repo_root)
    if not isinstance(source_map, dict):
        raise SystemExit(f"{effective_source_map_rel} must parse as a YAML mapping/object")

    example_pack_ref = source_map.get("example_pack_index_ref") or EXAMPLE_PACK_INDEX_REL
    if not isinstance(example_pack_ref, str) or not example_pack_ref:
        example_pack_ref = EXAMPLE_PACK_INDEX_REL
    example_pack_path = repo_root / example_pack_ref
    if not example_pack_path.exists():
        raise SystemExit(f"example pack index not found: {example_pack_ref}")
    example_pack = load_yaml(example_pack_path, scenario=scenario, repo_root=repo_root)
    if not isinstance(example_pack, dict):
        raise SystemExit(f"{example_pack_ref} must parse as a YAML mapping/object")

    example_rows: dict[str, dict[str, Any]] = {}
    for row in example_pack.get("examples") or []:
        if not isinstance(row, dict):
            continue
        example_id = row.get("example_id")
        if isinstance(example_id, str) and example_id:
            example_rows[example_id] = row

    protected_schema_refs: list[dict[str, Any]] = []
    if isinstance(source_map.get("protected_schema_refs"), list):
        protected_schema_refs = [row for row in source_map["protected_schema_refs"] if isinstance(row, dict)]

    protected_example_entries: list[dict[str, Any]] = []
    protected_payload_refs: set[str] = set()
    protected_family_rows = source_map.get("protected_families") or []
    if isinstance(protected_family_rows, list):
        for family in protected_family_rows:
            if not isinstance(family, dict):
                continue
            examples = family.get("protected_examples") or []
            if not isinstance(examples, list):
                continue
            for example in examples:
                if not isinstance(example, dict):
                    continue
                protected_example_entries.append(example)
                example_id = example.get("example_id")
                if isinstance(example_id, str) and example_id and example_id in example_rows:
                    payload_ref = example_rows[example_id].get("payload_ref")
                    if isinstance(payload_ref, str) and is_path_ref(payload_ref):
                        protected_payload_refs.add(payload_ref)

    protected_schema_paths = {
        str(row.get("schema_ref")) for row in protected_schema_refs if isinstance(row.get("schema_ref"), str)
    }

    map_rel = str(effective_source_map_rel)
    if protected_schema_paths.intersection(changed_files) and map_rel not in changed_files:
        findings.append(
            Finding(
                severity="error",
                check_id="schema_example_drift.protected_schema_requires_source_map_update",
                artifact_ref=map_rel,
                message="protected schema file changed without updating the contract-example source map",
                remediation=f"Update {map_rel} in the same change so the example-impact review is explicit.",
                details={"protected_schema_changes": sorted(protected_schema_paths.intersection(changed_files))},
            )
        )

    if protected_payload_refs.intersection(changed_files) and map_rel not in changed_files:
        findings.append(
            Finding(
                severity="error",
                check_id="schema_example_drift.protected_payload_requires_source_map_update",
                artifact_ref=map_rel,
                message="protected canonical example payload changed without updating the contract-example source map",
                remediation=f"Update {map_rel} in the same change so the example-impact review is explicit.",
                details={"protected_payload_changes": sorted(protected_payload_refs.intersection(changed_files))},
            )
        )

    for row in protected_schema_refs:
        schema_ref = row.get("schema_ref")
        expected = row.get("sha256")
        if not isinstance(schema_ref, str) or not schema_ref:
            continue
        schema_path = repo_root / schema_ref
        if not schema_path.exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id="schema_example_drift.protected_schema_exists",
                    artifact_ref=schema_ref,
                    message=f"protected schema_ref does not exist: {schema_ref}",
                    remediation="Fix the schema_ref path or restore the schema file.",
                    row_ref=schema_ref,
                )
            )
            continue
        actual = sha256_bytes(read_bytes(schema_path, scenario=scenario, repo_root=repo_root))
        if isinstance(expected, str) and expected and actual != expected:
            findings.append(
                Finding(
                    severity="error",
                    check_id="schema_example_drift.protected_schema_digest_matches",
                    artifact_ref=schema_ref,
                    message=f"protected schema digest drifted: {schema_ref}",
                    remediation="Update the source map digest (and review example/doc impact) in the same change.",
                    row_ref=schema_ref,
                    details={"expected_sha256": expected, "actual_sha256": actual},
                )
            )

    for entry in protected_example_entries:
        example_id = entry.get("example_id")
        expected = entry.get("payload_sha256")
        if not isinstance(example_id, str) or not example_id:
            continue
        row = example_rows.get(example_id)
        if row is None:
            continue
        payload_ref = row.get("payload_ref")
        if not isinstance(payload_ref, str) or not is_path_ref(payload_ref):
            continue
        payload_path = repo_root / payload_ref
        if not payload_path.exists():
            continue
        actual = sha256_bytes(read_bytes(payload_path, scenario=scenario, repo_root=repo_root))
        if isinstance(expected, str) and expected and actual != expected:
            findings.append(
                Finding(
                    severity="error",
                    check_id="schema_example_drift.protected_payload_digest_matches",
                    artifact_ref=payload_ref,
                    message=f"protected payload digest drifted: {payload_ref}",
                    remediation="Update the source map digest (and review redaction/meaning impact) in the same change.",
                    row_ref=example_id,
                    details={"expected_sha256": expected, "actual_sha256": actual},
                )
            )

    findings.extend(
        validate_examples_against_schemas(
            repo_root,
            example_rows,
            protected_example_entries,
            scenario=scenario,
        )
    )

    for family in protected_family_rows if isinstance(protected_family_rows, list) else []:
        if not isinstance(family, dict):
            continue
        examples = family.get("protected_examples") or []
        if not isinstance(examples, list):
            continue
        for example in examples:
            if not isinstance(example, dict):
                continue
            docs_snippets = example.get("docs_snippets") or []
            if not isinstance(docs_snippets, list):
                continue
            example_id = example.get("example_id")
            if not isinstance(example_id, str) or example_id not in example_rows:
                continue
            canonical_payload_ref = example_rows[example_id].get("payload_ref")
            if not isinstance(canonical_payload_ref, str) or not is_path_ref(canonical_payload_ref):
                continue
            canonical_path = repo_root / canonical_payload_ref
            try:
                canonical_payload = (
                    load_json(canonical_path, scenario=scenario, repo_root=repo_root)
                    if canonical_path.suffix.lower() == ".json"
                    else load_yaml(canonical_path, scenario=scenario, repo_root=repo_root)
                )
            except Exception:
                continue
            canonical_payload = strip_fixture_metadata(canonical_payload)
            for snippet in docs_snippets:
                if not isinstance(snippet, dict):
                    continue
                doc_ref = snippet.get("doc_ref")
                snippet_id = snippet.get("snippet_id")
                if not isinstance(doc_ref, str) or not isinstance(snippet_id, str):
                    continue
                found = find_snippet_payload(
                    repo_root,
                    doc_ref,
                    snippet_id,
                    scenario=scenario,
                )
                if found is None:
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="schema_example_drift.docs_snippet_resolves",
                            artifact_ref=doc_ref,
                            message=f"docs snippet id not found: {snippet_id} in {doc_ref}",
                            remediation="Ensure the doc contains the snippet block and that the source map points at the correct id.",
                            row_ref=snippet_id,
                            details={"example_id": example_id},
                        )
                    )
                    continue
                payload, lang, lineno = found
                if payload is None:
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="schema_example_drift.docs_snippet_parses",
                            artifact_ref=doc_ref,
                            message=f"docs snippet payload failed to parse (snippet id {snippet_id})",
                            remediation="Fix the snippet payload so it parses as JSON/YAML.",
                            row_ref=snippet_id,
                            details={"example_id": example_id, "locator": f"{doc_ref}:line:{lineno}", "lang": lang},
                        )
                    )
                    continue
                if payload != canonical_payload:
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="schema_example_drift.docs_snippet_matches_canonical_payload",
                            artifact_ref=doc_ref,
                            message=f"docs snippet drifted from canonical example payload: {snippet_id}",
                            remediation="Update the snippet content to match the canonical payload (or update the canonical payload + source map).",
                            row_ref=snippet_id,
                            details={"example_id": example_id, "canonical_payload_ref": canonical_payload_ref},
                        )
                    )

    analysis = {
        "observed_at": now_utc(),
        "source_map_ref": map_rel,
        "example_pack_index_ref": str(example_pack_ref),
        "changed_file_count": len(changed_files),
        "changed_files": changed_files,
        "finding_count": len(findings),
        "findings": [finding.as_report() for finding in findings],
    }
    return findings, analysis
