#!/usr/bin/env python3
"""Validate UI copy lint rules + worked fixtures without external deps."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
RULES_PATH = REPO_ROOT / "artifacts/copy/ui_copy_lint_rules.yaml"
CASE_DIR = REPO_ROOT / "fixtures/copy/ui_copy_cases"
CASE_PATHS = sorted(path for path in CASE_DIR.glob("*.yaml") if path.name != "README.md")

CMD_ID_RE = re.compile(r"^cmd:[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)+$")


class ValidationError(Exception):
    """Raised when an instance fails validation."""


def render_yaml_as_json(path: Path) -> object:
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


def expect(condition: bool, message: str) -> None:
    if not condition:
        raise ValidationError(message)


def as_list(value: object, where: str) -> list[object]:
    expect(isinstance(value, list), f"{where}: expected array")
    return value


def as_dict(value: object, where: str) -> dict[str, object]:
    expect(isinstance(value, dict), f"{where}: expected object")
    return value


def as_str(value: object, where: str) -> str:
    expect(isinstance(value, str) and value, f"{where}: expected non-empty string")
    return value


def load_rules() -> dict[str, object]:
    expect(RULES_PATH.exists(), f"missing UI copy lint rules: {RULES_PATH}")
    rules = as_dict(render_yaml_as_json(RULES_PATH), "ui_copy_lint_rules")
    expect(rules.get("record_kind") == "ui_copy_lint_rules_record", "ui_copy_lint_rules.record_kind mismatch")
    expect(rules.get("schema_version") == 1, "ui_copy_lint_rules.schema_version must be 1")

    rule_sets = as_dict(rules.get("rule_sets"), "ui_copy_lint_rules.rule_sets")
    action_label = as_dict(rule_sets.get("action_label"), "ui_copy_lint_rules.rule_sets.action_label")
    error_message = as_dict(rule_sets.get("error_message"), "ui_copy_lint_rules.rule_sets.error_message")
    ai_copy = as_dict(rule_sets.get("ai_copy"), "ui_copy_lint_rules.rule_sets.ai_copy")

    forbidden_labels = as_list(action_label.get("forbidden_exact_labels"), "action_label.forbidden_exact_labels")
    required_sections = as_list(error_message.get("required_sections"), "error_message.required_sections")
    forbidden_failure_summaries = as_list(
        error_message.get("forbidden_exact_failure_summaries"), "error_message.forbidden_exact_failure_summaries"
    )
    forbidden_next_actions = as_list(
        error_message.get("forbidden_exact_next_actions"), "error_message.forbidden_exact_next_actions"
    )

    forbidden_registry_ref = as_str(ai_copy.get("forbidden_terms_registry_ref"), "ai_copy.forbidden_terms_registry_ref")
    forbidden_registry_path = (REPO_ROOT / forbidden_registry_ref).resolve()
    expect(forbidden_registry_path.exists(), f"ai_copy.forbidden_terms_registry_ref missing: {forbidden_registry_ref}")
    forbidden_registry = as_dict(render_yaml_as_json(forbidden_registry_path), "ai_copy.forbidden_terms_registry")
    forbidden_terms = as_list(forbidden_registry.get("forbidden_terms"), "ai_copy.forbidden_terms")

    softening_terms = as_list(
        ai_copy.get("required_softening_terms_for_unvalidated_output"),
        "ai_copy.required_softening_terms_for_unvalidated_output",
    )

    return {
        "action_label_rule_id": as_str(action_label.get("rule_id"), "action_label.rule_id"),
        "error_message_rule_id": as_str(error_message.get("rule_id"), "error_message.rule_id"),
        "ai_copy_rule_id": as_str(ai_copy.get("rule_id"), "ai_copy.rule_id"),
        "forbidden_action_labels": {as_str(item, "action_label.forbidden_exact_labels[]") for item in forbidden_labels},
        "required_error_sections": {as_str(item, "error_message.required_sections[]") for item in required_sections},
        "forbidden_failure_summaries": {
            as_str(item, "error_message.forbidden_exact_failure_summaries[]") for item in forbidden_failure_summaries
        },
        "forbidden_next_action_labels": {
            as_str(item, "error_message.forbidden_exact_next_actions[]") for item in forbidden_next_actions
        },
        "forbidden_ai_phrase_patterns": extract_phrase_patterns(forbidden_terms),
        "required_ai_softening_terms": {
            as_str(item, "ai_copy.required_softening_terms_for_unvalidated_output[]") for item in softening_terms
        },
    }


def extract_phrase_patterns(forbidden_terms: list[object]) -> list[str]:
    patterns: list[str] = []
    for idx, item in enumerate(forbidden_terms):
        row = as_dict(item, f"ai_copy.forbidden_terms[{idx}]")
        phrase_patterns = as_list(row.get("phrase_patterns"), f"ai_copy.forbidden_terms[{idx}].phrase_patterns")
        for phrase in phrase_patterns:
            phrase_str = as_str(phrase, f"ai_copy.forbidden_terms[{idx}].phrase_patterns[]").strip().lower()
            if phrase_str:
                patterns.append(phrase_str)
    # Prefer longer phrases first to reduce noisy matches.
    return sorted(set(patterns), key=lambda p: (-len(p), p))


def find_ai_forbidden_matches(text: str, phrase_patterns: list[str]) -> list[str]:
    haystack = text.lower()
    return [pattern for pattern in phrase_patterns if pattern in haystack]


def validate_action_label_case(case: dict[str, object], rules: dict[str, object], where: str) -> None:
    copy = case.get("copy_under_review")
    if copy is None:
        return
    copy_obj = as_dict(copy, f"{where}.copy_under_review")
    action_labels = copy_obj.get("action_labels")
    if action_labels is None:
        return
    for idx, item in enumerate(as_list(action_labels, f"{where}.copy_under_review.action_labels")):
        row = as_dict(item, f"{where}.copy_under_review.action_labels[{idx}]")
        label = as_str(row.get("rendered_label"), f"{where}.action_labels[{idx}].rendered_label")
        decision = as_str(row.get("expected_decision"), f"{where}.action_labels[{idx}].expected_decision")
        expected_rule_ids = row.get("expected_rule_ids")
        if expected_rule_ids is None:
            expected_rules: list[str] = []
        else:
            expected_rules = [
                as_str(value, f"{where}.action_labels[{idx}].expected_rule_ids[]")
                for value in as_list(expected_rule_ids, f"{where}.action_labels[{idx}].expected_rule_ids")
            ]

        rule_id = rules["action_label_rule_id"]
        forbidden = rules["forbidden_action_labels"]

        violates = label in forbidden
        if decision == "approved":
            expect(
                not violates,
                f"{where}: action label {label!r} is forbidden but marked approved",
            )
        elif decision == "rejected":
            if rule_id in expected_rules:
                expect(violates, f"{where}: expected {rule_id} but label {label!r} is not in forbidden list")
        else:
            raise ValidationError(f"{where}: unknown expected_decision {decision!r} for action label {label!r}")


def validate_error_message_candidate(
    candidate: dict[str, object], rules: dict[str, object], where: str, *, require_schema_like_next_action: bool
) -> bool:
    required = rules["required_error_sections"]
    forbidden_summaries = rules["forbidden_failure_summaries"]
    forbidden_next = rules["forbidden_next_action_labels"]

    def present(key: str) -> bool:
        return key in candidate and candidate[key] not in (None, "")

    missing = sorted(key for key in required if not present(key))
    if missing:
        return False

    what_failed = candidate.get("what_failed")
    if isinstance(what_failed, str) and what_failed.strip() in forbidden_summaries:
        return False

    next_action = candidate.get("next_safe_action")
    if require_schema_like_next_action:
        if not isinstance(next_action, dict):
            return False
        action_label = next_action.get("action_label")
        summary = next_action.get("summary")
        if not isinstance(action_label, str) or not action_label.strip():
            return False
        if not isinstance(summary, str) or not summary.strip():
            return False
        if action_label.strip() in forbidden_next:
            return False
    else:
        if next_action in (None, ""):
            return False

    return True


def validate_error_message_case(case: dict[str, object], rules: dict[str, object], where: str) -> None:
    copy = case.get("copy_under_review")
    if copy is None:
        return
    copy_obj = as_dict(copy, f"{where}.copy_under_review")
    error_messages = copy_obj.get("error_messages")
    if error_messages is not None:
        rule_id = rules["error_message_rule_id"]
        for idx, item in enumerate(as_list(error_messages, f"{where}.copy_under_review.error_messages")):
            row = as_dict(item, f"{where}.copy_under_review.error_messages[{idx}]")
            decision = as_str(row.get("expected_decision"), f"{where}.error_messages[{idx}].expected_decision")
            expected_rule_ids = [
                as_str(value, f"{where}.error_messages[{idx}].expected_rule_ids[]")
                for value in as_list(row.get("expected_rule_ids", []), f"{where}.error_messages[{idx}].expected_rule_ids")
            ]

            passes = validate_error_message_candidate(row, rules, where, require_schema_like_next_action=True)
            if decision == "approved":
                expect(passes, f"{where}: error message candidate marked approved but fails required structure")
            elif decision == "rejected":
                expect(rule_id in expected_rule_ids, f"{where}: rejected error message must cite {rule_id} in expected_rule_ids")
                expect(not passes, f"{where}: error message candidate marked rejected but appears to satisfy required structure")
            else:
                raise ValidationError(f"{where}: unknown expected_decision {decision!r} for error message candidate")

    approved_record = copy_obj.get("approved_error_message_record")
    if approved_record is not None:
        record = as_dict(approved_record, f"{where}.copy_under_review.approved_error_message_record")
        expect(record.get("record_kind") == "error_message_record", f"{where}: approved_error_message_record.record_kind mismatch")
        expect(
            record.get("error_message_schema_version") == 1,
            f"{where}: approved_error_message_record.error_message_schema_version must be 1",
        )
        expect(isinstance(record.get("message_id"), str) and record.get("message_id"), f"{where}: message_id required")
        expect(isinstance(record.get("surface_family"), str) and record.get("surface_family"), f"{where}: surface_family required")
        expect(isinstance(record.get("severity"), str) and record.get("severity"), f"{where}: severity required")

        for field in ("what_failed", "likely_cause", "what_still_works"):
            expect(isinstance(record.get(field), str) and record.get(field), f"{where}: {field} required on approved record")

        next_action = as_dict(record.get("next_safe_action"), f"{where}: approved_error_message_record.next_safe_action")
        action_label = as_str(next_action.get("action_label"), f"{where}: approved_error_message_record.next_safe_action.action_label")
        expect(
            action_label not in rules["forbidden_action_labels"],
            f"{where}: approved error message next_safe_action.action_label uses forbidden standalone label {action_label!r}",
        )
        summary = as_str(next_action.get("summary"), f"{where}: approved_error_message_record.next_safe_action.summary")
        _ = summary
        cmd_ref = next_action.get("command_id_ref")
        if cmd_ref is not None:
            cmd_str = as_str(cmd_ref, f"{where}: approved_error_message_record.next_safe_action.command_id_ref")
            expect(CMD_ID_RE.match(cmd_str) is not None, f"{where}: command_id_ref {cmd_str!r} does not match cmd: pattern")


def validate_ai_copy_case(case: dict[str, object], rules: dict[str, object], where: str) -> None:
    copy = case.get("copy_under_review")
    if copy is None:
        return
    copy_obj = as_dict(copy, f"{where}.copy_under_review")
    ai_copy = copy_obj.get("ai_copy")
    if ai_copy is None:
        return
    rule_id = rules["ai_copy_rule_id"]
    patterns = rules["forbidden_ai_phrase_patterns"]
    softening = rules["required_ai_softening_terms"]

    for idx, item in enumerate(as_list(ai_copy, f"{where}.copy_under_review.ai_copy")):
        row = as_dict(item, f"{where}.copy_under_review.ai_copy[{idx}]")
        text = as_str(row.get("rendered_copy"), f"{where}.ai_copy[{idx}].rendered_copy")
        decision = as_str(row.get("expected_decision"), f"{where}.ai_copy[{idx}].expected_decision")
        expected_rule_ids = [
            as_str(value, f"{where}.ai_copy[{idx}].expected_rule_ids[]")
            for value in as_list(row.get("expected_rule_ids", []), f"{where}.ai_copy[{idx}].expected_rule_ids")
        ]

        matches = find_ai_forbidden_matches(text, patterns)
        has_softening = any(term.lower() in text.lower() for term in softening)

        if decision == "approved":
            expect(has_softening, f"{where}: approved ai_copy must include one of {sorted(softening)!r}")
            expect(not matches, f"{where}: approved ai_copy contains forbidden phrases: {matches!r}")
        elif decision == "rejected":
            expect(rule_id in expected_rule_ids, f"{where}: rejected ai_copy must cite {rule_id} in expected_rule_ids")
            expect(matches or not has_softening, f"{where}: rejected ai_copy must either match forbidden patterns or omit softening terms")
        else:
            raise ValidationError(f"{where}: unknown expected_decision {decision!r} for ai_copy candidate")


def main() -> int:
    if not CASE_DIR.exists():
        print(f"missing ui copy case dir: {CASE_DIR}", file=sys.stderr)
        return 1

    if not CASE_PATHS:
        print(f"no ui copy cases found in {CASE_DIR}", file=sys.stderr)
        return 1

    rules = load_rules()
    print(RULES_PATH.relative_to(REPO_ROOT))

    for path in CASE_PATHS:
        instance = render_yaml_as_json(path)
        case = as_dict(instance, str(path.relative_to(REPO_ROOT)))
        expect(case.get("record_kind") == "ui_copy_case_record", f"{path}: record_kind must be ui_copy_case_record")
        expect(case.get("ui_copy_lint_rules_version") == 1, f"{path}: ui_copy_lint_rules_version must be 1")
        expect(isinstance(case.get("case_id"), str) and case.get("case_id"), f"{path}: case_id required")
        expect(isinstance(case.get("surface_family"), str) and case.get("surface_family"), f"{path}: surface_family required")

        where = str(path.relative_to(REPO_ROOT))
        validate_action_label_case(case, rules, where)
        validate_error_message_case(case, rules, where)
        validate_ai_copy_case(case, rules, where)

        print(where)

    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ValidationError as exc:
        print(f"ui copy validation failed: {exc}", file=sys.stderr)
        raise SystemExit(1)

