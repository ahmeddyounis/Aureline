#!/usr/bin/env python3
"""M3 protected-example stale-example checker.

This gate is the M3 sibling to the M1 stale-example detection
pipeline. It reads the M3 source map at
``artifacts/ci/m3_docs_truth_source_map.yaml`` and the M3 governed
claim manifest, and for every protected example listed in the source
map verifies that:

- the payload exists on disk, parses, and validates against the
  pinned schema (after stripping fixture-only metadata keys);
- every declared ``vocabulary_pins`` resolves to a string on the
  payload and that string is still present in the manifest's
  ``vocabularies[<vocabulary_id>]`` list; and
- the named protected-example failure drill is reproducible: under
  ``--force-drill`` the named drill is replayed and the gate exits 0
  only when the declared ``expected_check_id`` is observed.

The runner emits a deterministic human summary on stdout and a
durable JSON capture (``--capture`` path) for proof archives.
"""

from __future__ import annotations

import copy
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

if __package__ in (None, ""):
    HERE = Path(__file__).resolve().parent
    sys.path.insert(0, str(HERE.parent.parent))
    from ci.m3._common import (  # type: ignore
        Finding,
        base_argument_parser,
        ensure_dict,
        ensure_list,
        ensure_str,
        load_claim_manifest,
        load_source_map,
        now_iso_z,
        parse_payload,
        resolve_payload_path,
        resolve_today,
        set_payload_path,
        validate_payload_against_schema,
    )
else:
    from ._common import (
        Finding,
        base_argument_parser,
        ensure_dict,
        ensure_list,
        ensure_str,
        load_claim_manifest,
        load_source_map,
        now_iso_z,
        parse_payload,
        resolve_payload_path,
        resolve_today,
        set_payload_path,
        validate_payload_against_schema,
    )


DEFAULT_CAPTURE_REL = (
    "artifacts/docs/m3/captures/m3_stale_example_validation_capture.json"
)


@dataclass
class ExampleResult:
    example_id: str
    payload_ref: str
    schema_ref: str
    passed_checks: list[str] = field(default_factory=list)
    failed_checks: list[dict[str, Any]] = field(default_factory=list)
    diagnostics: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        return asdict(self)


def fail_example(
    result: ExampleResult,
    findings: list[Finding],
    check_id: str,
    message: str,
    remediation: str,
    *,
    details: dict[str, Any] | None = None,
) -> None:
    entry: dict[str, Any] = {"check_id": check_id, "message": message}
    if details:
        entry["details"] = details
    result.failed_checks.append(entry)
    findings.append(
        Finding(
            severity="error",
            check_id=check_id,
            message=f"{result.example_id}: {message}",
            remediation=remediation,
            ref=result.example_id,
            details=details or {},
        )
    )


def find_forced_drill(
    source_map: dict[str, Any],
    drill_id: str,
) -> dict[str, Any]:
    drills = ensure_list(
        source_map.get("failure_drills", []),
        "source_map.failure_drills",
    )
    for drill in drills:
        if (
            isinstance(drill, dict)
            and drill.get("drill_id") == drill_id
        ):
            return drill
    raise SystemExit(
        f"--force-drill {drill_id!r} does not match any "
        "failure_drills[].drill_id in the source map."
    )


def apply_protected_example_drill(
    payload: Any,
    forced_input: dict[str, Any],
) -> tuple[Any, dict[str, Any]]:
    payload = copy.deepcopy(payload)
    applied: dict[str, Any] = {}
    rewrite_path = forced_input.get("rewrite_path")
    replacement = forced_input.get("replacement_value")
    if isinstance(rewrite_path, str) and rewrite_path:
        if set_payload_path(payload, rewrite_path, replacement):
            applied["rewrite_path"] = rewrite_path
            applied["replacement_value"] = replacement
    return payload, applied


def validate_example(
    *,
    repo_root: Path,
    example: dict[str, Any],
    manifest_vocabs: dict[str, list[str]],
    findings: list[Finding],
    forced_input: dict[str, Any] | None,
) -> ExampleResult:
    example_id = ensure_str(
        example.get("example_id"), "protected_examples[].example_id"
    )
    payload_ref = ensure_str(
        example.get("payload_ref"), f"{example_id}.payload_ref"
    )
    schema_ref = ensure_str(
        example.get("schema_ref"), f"{example_id}.schema_ref"
    )
    result = ExampleResult(
        example_id=example_id,
        payload_ref=payload_ref,
        schema_ref=schema_ref,
    )
    payload_path = repo_root / payload_ref
    schema_path = repo_root / schema_ref
    if not payload_path.exists():
        fail_example(
            result,
            findings,
            "stale_examples.payload_missing",
            f"payload file does not exist: {payload_ref}",
            "Restore the payload or update the source map path in the "
            "same change set.",
        )
        return result
    if not schema_path.exists():
        fail_example(
            result,
            findings,
            "stale_examples.schema_missing",
            f"schema file does not exist: {schema_ref}",
            "Restore the schema or update the source map path in the "
            "same change set.",
        )
        return result
    try:
        payload = parse_payload(payload_path)
    except Exception as exc:  # noqa: BLE001
        fail_example(
            result,
            findings,
            "stale_examples.payload_parse_failed",
            f"payload {payload_ref} failed to parse: {exc}",
            "Fix the payload syntax or the YAML/JSON encoding.",
        )
        return result

    # Apply drill input if any.
    if forced_input is not None:
        payload, applied = apply_protected_example_drill(
            payload, forced_input
        )
        if applied:
            result.diagnostics["forced_overrides_applied"] = applied

    # Schema validity (after fixture metadata strip).
    ok, schema_errors = validate_payload_against_schema(
        payload=payload,
        schema_ref=schema_ref,
        schema_path=schema_path,
    )
    if not ok:
        fail_example(
            result,
            findings,
            "stale_examples.example_payload_schema_invalid",
            (
                f"payload {payload_ref} does not validate against "
                f"{schema_ref}: {schema_errors[:3]}"
            ),
            "Restore the payload so it validates, or land a superseding "
            "schema in the same change set.",
        )
    elif schema_errors == ["schema_validator_unavailable"]:
        result.diagnostics["schema_validator"] = "unavailable"
    else:
        result.passed_checks.append("payload_schema_valid")

    # Vocabulary pins resolve through the live manifest vocabularies.
    pins = ensure_list(
        example.get("vocabulary_pins", []),
        f"{example_id}.vocabulary_pins",
    )
    if not pins:
        fail_example(
            result,
            findings,
            "stale_examples.vocabulary_pins_empty",
            f"protected_examples[{example_id}].vocabulary_pins must "
            "list at least one pin.",
            "Add at least one vocabulary_pin entry so the gate can "
            "verify the payload's vocabulary against the manifest.",
        )
        return result
    for pin in pins:
        pin = ensure_dict(pin, f"{example_id}.vocabulary_pins[]")
        vocab_id = ensure_str(
            pin.get("vocabulary_id"), f"{example_id}.vocabulary_id"
        )
        payload_path_value = ensure_str(
            pin.get("payload_path"), f"{example_id}.payload_path"
        )
        if vocab_id not in manifest_vocabs:
            fail_example(
                result,
                findings,
                "stale_examples.vocabulary_pin_unknown_id",
                (
                    f"vocabulary_pin {vocab_id!r} is not present in the "
                    f"claim manifest vocabularies"
                ),
                "Pin the vocabulary to a name still listed in "
                "manifest.vocabularies.",
                details={"vocabulary_id": vocab_id},
            )
            continue
        found, value = resolve_payload_path(payload, payload_path_value)
        if not found:
            fail_example(
                result,
                findings,
                "stale_examples.vocabulary_pin_path_unresolved",
                (
                    f"vocabulary_pin {vocab_id!r} payload_path "
                    f"{payload_path_value!r} did not resolve in "
                    f"{payload_ref}"
                ),
                "Fix the payload_path or the payload so the field "
                "exists.",
                details={"vocabulary_id": vocab_id, "payload_path": payload_path_value},
            )
            continue
        if not isinstance(value, str) or not value:
            fail_example(
                result,
                findings,
                "stale_examples.vocabulary_pin_value_not_string",
                (
                    f"vocabulary_pin {vocab_id!r} at {payload_path_value!r} "
                    f"resolved to a non-string or empty value"
                ),
                "Pin the vocabulary to a string-valued payload field.",
                details={"vocabulary_id": vocab_id, "payload_path": payload_path_value},
            )
            continue
        if value not in manifest_vocabs[vocab_id]:
            fail_example(
                result,
                findings,
                "stale_examples.vocabulary_pin_not_in_manifest",
                (
                    f"vocabulary_pin {vocab_id!r} token {value!r} is no "
                    f"longer present in manifest.vocabularies."
                    f"{vocab_id}"
                ),
                "Restore the payload's pinned token to a value still "
                "listed in the manifest vocabulary, or land a "
                "superseding manifest vocabulary in the same change "
                "set.",
                details={
                    "vocabulary_id": vocab_id,
                    "payload_path": payload_path_value,
                    "stale_token": value,
                },
            )
        else:
            result.passed_checks.append(
                f"vocabulary_pin:{vocab_id}={value}"
            )
    return result


def main() -> int:
    parser = base_argument_parser(description=__doc__)
    parser.add_argument(
        "--capture", default=DEFAULT_CAPTURE_REL,
        help="Where to write the durable JSON capture.",
    )
    args = parser.parse_args()

    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    source_map = load_source_map(repo_root, args.source_map)
    manifest_ref = ensure_str(
        source_map.get("claim_manifest_ref"),
        "source_map.claim_manifest_ref",
    )
    manifest = load_claim_manifest(repo_root, manifest_ref)
    manifest_vocabs_raw = ensure_dict(
        manifest.get("vocabularies"), "claim_manifest.vocabularies"
    )
    manifest_vocabs: dict[str, list[str]] = {}
    for name, values in manifest_vocabs_raw.items():
        if not isinstance(values, list):
            continue
        manifest_vocabs[name] = [
            v for v in values if isinstance(v, str)
        ]

    findings: list[Finding] = []
    today = resolve_today(args.today)

    examples = ensure_list(
        source_map.get("protected_examples", []),
        "source_map.protected_examples",
    )

    # Force drill handling for protected_example target.
    forced_drill: dict[str, Any] | None = None
    forced_example_id: str | None = None
    forced_input: dict[str, Any] | None = None
    if args.force_drill:
        forced_drill = find_forced_drill(source_map, args.force_drill)
        target = forced_drill.get("target")
        if target != "protected_example":
            raise SystemExit(
                f"--force-drill {args.force_drill!r} targets {target!r}; "
                "the stale-example checker only replays "
                "protected_example drills."
            )
        forced_input = ensure_dict(
            forced_drill.get("forced_input", {}),
            f"{args.force_drill}.forced_input",
        )
        forced_example_id = ensure_str(
            forced_input.get("example_id"),
            f"{args.force_drill}.forced_input.example_id",
        )

    results: list[ExampleResult] = []
    for raw in examples:
        example = ensure_dict(raw, "protected_examples[]")
        example_id = ensure_str(
            example.get("example_id"), "protected_examples[].example_id"
        )
        drill_input_for_this: dict[str, Any] | None = None
        if (
            forced_drill is not None
            and forced_example_id == example_id
            and forced_input is not None
        ):
            drill_input_for_this = forced_input
        result = validate_example(
            repo_root=repo_root,
            example=example,
            manifest_vocabs=manifest_vocabs,
            findings=findings,
            forced_input=drill_input_for_this,
        )
        results.append(result)

    if forced_drill is not None and forced_example_id is not None:
        if forced_example_id not in {r.example_id for r in results}:
            raise SystemExit(
                f"--force-drill targeted example_id "
                f"{forced_example_id!r} which is not in "
                "protected_examples[]."
            )

    generated_at = now_iso_z()
    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "m3_stale_example_validation_capture",
        "captured_at": generated_at,
        "today": today.isoformat(),
        "source_map_ref": args.source_map,
        "claim_manifest_ref": manifest_ref,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tools/ci/m3/stale_example_checker.py --repo-root ."
        ),
        "status": "pass"
        if not [f for f in findings if f.severity == "error"]
        else "fail",
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(
                1 for f in findings if f.severity == "warning"
            ),
        },
        "manifest_vocabularies_checked": sorted(manifest_vocabs.keys()),
        "examples": [r.as_report() for r in results],
        "findings": [f.as_report() for f in findings],
    }

    if forced_drill is not None:
        observed = [
            f.check_id for f in findings if f.severity == "error"
        ]
        capture["forced_drill_replay"] = {
            "drill_id": forced_drill.get("drill_id"),
            "expected_check_id": forced_drill.get("expected_check_id"),
            "actionable_next_action": forced_drill.get(
                "actionable_next_action"
            ),
            "observed_failed_check_ids": observed,
            "reproduced": forced_drill.get("expected_check_id")
            in observed,
        }

    capture_path = repo_root / args.capture
    capture_path.parent.mkdir(parents=True, exist_ok=True)
    capture_path.write_text(
        json.dumps(capture, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )

    label = "m3-stale-examples"
    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[{label}] {status} ({len(errors)} errors, "
        f"{len(findings) - len(errors)} warnings) — capture: "
        f"{args.capture}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[{label}] {prefix} {finding.check_id}: "
            f"{finding.message}{ref_suffix}"
        )
        print(f"[{label}]   remediation: {finding.remediation}")

    if forced_drill is not None:
        reproduced = capture["forced_drill_replay"]["reproduced"]
        expected = forced_drill.get("expected_check_id")
        if reproduced:
            print(
                f"[{label}] forced drill "
                f"{forced_drill.get('drill_id')!r} reproduced "
                f"{expected!r}"
            )
            return 0
        print(
            f"[{label}] forced drill "
            f"{forced_drill.get('drill_id')!r} did NOT reproduce "
            f"{expected!r}; observed: "
            f"{capture['forced_drill_replay']['observed_failed_check_ids']}"
        )
        return 2

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[m3-stale-examples] interrupted", file=sys.stderr)
        sys.exit(130)
