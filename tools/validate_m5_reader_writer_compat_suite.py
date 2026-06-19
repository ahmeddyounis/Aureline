#!/usr/bin/env python3
"""Validate the M5 reader/writer compatibility suite and its fixture corpus.

Validates:
- ``artifacts/contracts/m5-reader-writer-compat-suite.json`` against
  ``schemas/public/m5-contracts/m5_reader_writer_compat_suite.schema.json``
- the suite's semantic invariants (duplicate families, closed-vocabulary
  membership, posture/write-back consistency, monotone version triple, per-family
  required case kinds, additive migration-diff, and the summary recomputed from
  the suites), mirroring the typed Rust consumer
- that the checked-in suite, fixtures, migration-diff reports, operator report,
  SDK doc, overview doc, evidence packet, capture, and negative fixtures match the
  regenerator (no hand-edit drift)
- for every family: the prior/current/unsupported fixtures validate against the
  family's published JSON Schema package, the current fixture carries the additive
  field and at least one unknown field that survives a parse/serialize round-trip,
  the unsupported fixture is stamped beyond the published ceiling, and the
  embedded migration-diff matches the standalone per-family report
- that each suite's reader/writer posture agrees with the publication matrix and
  its family resolves in the JSON Schema catalog
- that every referenced repo-relative path exists
- that the checked-in negative fixtures under ``fixtures/contracts/m5-compat-suite/``
  are rejected by the semantic invariants

The validator imports the builders from the regenerator so the two cannot drift.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any

TOOLS_DIR = Path(__file__).resolve().parent
if str(TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(TOOLS_DIR))

import regenerate_m5_reader_writer_compat_suite as gen  # noqa: E402

REPO_ROOT = gen.REPO_ROOT
SCHEMA_PATH = REPO_ROOT / "schemas/public/m5-contracts/m5_reader_writer_compat_suite.schema.json"
CATALOG_PATH = REPO_ROOT / "artifacts/contracts/m5-json-schema-catalog.json"
MATRIX_PATH = REPO_ROOT / gen.PUBLICATION_MATRIX_REF


def load_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def is_path_ref(value: str) -> bool:
    if not value:
        return False
    if "://" in value:
        return False
    return "/" in value or value.endswith(
        (".json", ".yaml", ".yml", ".md", ".py", ".sh", ".toml", ".wit")
    )


def candidate_path(ref: str) -> str:
    return ref.split("#", 1)[0] if "#" in ref else ref


def collect_refs(suite: dict) -> list[str]:
    refs: list[str] = []
    for key in (
        "overview_page",
        "evidence_page",
        "sdk_catalog_page",
        "json_schema_catalog_ref",
        "publication_matrix_ref",
        "contract_family_registry_ref",
        "evidence_index_ref",
        "fixture_home",
        "migration_diff_report_home",
        "operator_report_ref",
    ):
        value = suite.get(key)
        if isinstance(value, str):
            refs.append(value)
    refs.extend(suite.get("offline_bundle", {}).get("bundle_members", []))
    for row in suite.get("suites", []):
        for key in (
            "fixture_dir",
            "prior_fixture_ref",
            "current_fixture_ref",
            "unsupported_fixture_ref",
            "schema_path",
            "catalog_package_ref",
            "matrix_row_ref",
            "contract_family_ref",
            "compatibility_note_ref",
        ):
            value = row.get(key)
            if isinstance(value, str):
                refs.append(value)
        refs.extend(v for v in row.get("validator_suite_refs", []) if isinstance(v, str))
        refs.append(row.get("migration_diff", {}).get("report_ref"))
        for case in row.get("cases", []):
            refs.append(case.get("input_fixture_ref"))
    return [r for r in refs if isinstance(r, str)]


def semantic_violations(suite: dict) -> list[str]:
    """Recompute the suite's derived state and report disagreements.

    Mirrors the typed Rust consumer's `validate()`. The canonical suite must
    return no violations; each negative fixture must return at least one.
    """
    violations: list[str] = []

    if suite.get("record_kind") != gen.RECORD_KIND:
        violations.append("record_kind mismatch")
    if suite.get("schema_version") != gen.SCHEMA_VERSION:
        violations.append("schema_version mismatch")
    if suite.get("suite_id") != gen.SUITE_ID:
        violations.append("suite_id mismatch")

    for field, expected in [
        ("case_kinds", gen.CASE_KINDS),
        ("expected_outcomes", gen.EXPECTED_OUTCOMES),
        ("reader_writer_postures", gen.READER_WRITER_POSTURES),
        ("write_back_postures", gen.WRITE_BACK_POSTURES),
        ("change_classes", gen.CHANGE_CLASSES),
        ("downgrade_behaviors", gen.DOWNGRADE_BEHAVIORS),
        ("resolution_surfaces", gen.RESOLUTION_SURFACES),
    ]:
        if suite.get(field) != expected:
            violations.append(f"closed vocabulary mismatch: {field}")

    suites = suite.get("suites", [])
    seen_family: set[str] = set()
    for row in suites:
        fid = row.get("family_id", "<unknown>")
        if fid in seen_family:
            violations.append(f"duplicate family_id: {fid}")
        seen_family.add(fid)

        if row.get("package_id") != f"m5.{fid}":
            violations.append(f"{fid}: package_id must be 'm5.<family_id>'")

        posture = row.get("reader_writer_posture")
        if posture not in gen.READER_WRITER_POSTURES:
            violations.append(f"{fid}: reader_writer_posture not in vocabulary")
        if row.get("write_back_posture") != gen.write_back_posture(posture):
            violations.append(
                f"{fid}: write_back_posture disagrees with reader_writer_posture"
            )
        if row.get("downgrade_behavior") not in gen.DOWNGRADE_BEHAVIORS:
            violations.append(f"{fid}: downgrade_behavior not in vocabulary")

        prior = row.get("prior_version")
        current = row.get("current_version")
        unsupported = row.get("unsupported_version")
        if not (isinstance(prior, int) and isinstance(current, int) and isinstance(unsupported, int)):
            violations.append(f"{fid}: version triple must be integers")
        elif not (prior < current < unsupported):
            violations.append(f"{fid}: version triple must be strictly increasing")

        version_fields = row.get("version_field_names", [])
        if not version_fields:
            violations.append(f"{fid}: empty version_field_names")
        if row.get("primary_version_field") not in version_fields:
            violations.append(f"{fid}: primary_version_field not in version_field_names")

        cases = row.get("cases", [])
        if not cases:
            violations.append(f"{fid}: no cases")
        kinds = [c.get("case_kind") for c in cases]
        for kind in kinds:
            if kind not in gen.CASE_KINDS:
                violations.append(f"{fid}: case_kind '{kind}' not in vocabulary")
        for required_kind in (
            "forward_read",
            "back_read",
            "additive_field",
            "unknown_field_preservation",
            "migration_diff",
            "downgrade",
        ):
            if required_kind not in kinds:
                violations.append(f"{fid}: missing required case kind '{required_kind}'")

        writes_back = row.get("write_back_posture") == "backup_then_write"
        if writes_back and "round_trip" not in kinds:
            violations.append(f"{fid}: write-back family must carry a round_trip case")
        if (not writes_back) and "compare_only" not in kinds:
            violations.append(f"{fid}: compare-only family must carry a compare_only case")
        if writes_back and "compare_only" in kinds:
            violations.append(f"{fid}: write-back family must not carry a compare_only case")
        if (not writes_back) and "round_trip" in kinds:
            violations.append(f"{fid}: compare-only family must not carry a round_trip case")

        # Compare-only families never write back; backup_first implies writes_back.
        for case in cases:
            if not writes_back and case.get("writes_back"):
                violations.append(f"{fid}: compare-only family case writes back")
            if case.get("backup_first") and not case.get("writes_back"):
                violations.append(f"{fid}: backup_first set without writes_back")
            if case.get("case_kind") == "downgrade" and case.get("expected_outcome") != "narrowed":
                violations.append(f"{fid}: downgrade case must expect 'narrowed'")

        diff = row.get("migration_diff", {})
        if diff.get("change_class") != "additive":
            violations.append(f"{fid}: migration_diff change_class must be 'additive'")
        if diff.get("compatible") is not True:
            violations.append(f"{fid}: migration_diff must be compatible")
        if diff.get("removed_fields"):
            violations.append(f"{fid}: additive migration_diff must remove no fields")
        if diff.get("changed_fields"):
            violations.append(f"{fid}: additive migration_diff must change no fields")
        if not diff.get("added_fields"):
            violations.append(f"{fid}: additive migration_diff must add at least one field")

    if suite.get("summary") != gen.compute_summary(suites):
        violations.append("summary counts disagree with the suites")

    return violations


def load_matrix_postures() -> dict[str, str]:
    if not MATRIX_PATH.exists():
        return {}
    matrix = load_json(MATRIX_PATH)
    return {
        row.get("family_id"): row.get("reader_writer_posture")
        for row in matrix.get("rows", [])
        if isinstance(row, dict)
    }


def load_catalog_families() -> set[str]:
    if not CATALOG_PATH.exists():
        return set()
    catalog = load_json(CATALOG_PATH)
    return {p.get("family_id") for p in catalog.get("packages", [])}


def main() -> int:
    try:
        from jsonschema import Draft202012Validator
    except Exception as exc:  # pragma: no cover
        print(f"[m5-compat-suite] error: python jsonschema is required: {exc}", file=sys.stderr)
        return 2

    failures: list[str] = []

    if not SCHEMA_PATH.exists():
        print(f"[m5-compat-suite] error: missing schema {SCHEMA_PATH}", file=sys.stderr)
        return 2
    if not gen.SUITE_PATH.exists():
        print(f"[m5-compat-suite] error: missing suite {gen.SUITE_PATH}", file=sys.stderr)
        return 2

    schema = load_json(SCHEMA_PATH)
    validator = Draft202012Validator(schema)
    suite = load_json(gen.SUITE_PATH)

    # 1) Schema validation of the canonical suite.
    for err in sorted(validator.iter_errors(suite), key=lambda e: list(e.path)):
        loc = "/".join(str(p) for p in err.path) or "<root>"
        failures.append(f"schema: {loc}: {err.message}")

    # 2) Semantic invariants on the canonical suite.
    for msg in semantic_violations(suite):
        failures.append(f"semantic: {msg}")

    # 3) Regenerator drift: the suite must match what the regenerator builds.
    if suite != gen.build_suite():
        failures.append(
            "drift: artifacts/contracts/m5-reader-writer-compat-suite.json is stale; "
            "run tools/regenerate_m5_reader_writer_compat_suite.py"
        )

    # 4) Generated companion artifacts must match the regenerator.
    capture_path = gen.CAPTURE_PATH
    if capture_path.exists():
        if load_json(capture_path) != gen.build_capture(suite):
            failures.append("drift: validation capture is stale; run the regenerator")
    else:
        failures.append("missing validation capture")
    for path, builder in [
        (gen.OPERATOR_REPORT_PATH, gen.build_operator_report),
        (gen.SDK_DOC_PATH, gen.build_sdk_doc),
        (gen.OVERVIEW_DOC_PATH, gen.build_overview_doc),
        (gen.EVIDENCE_DOC_PATH, gen.build_evidence_doc),
    ]:
        expected = builder(suite)
        if not expected.endswith("\n"):
            expected += "\n"
        if not path.exists():
            failures.append(f"missing generated doc {path.relative_to(REPO_ROOT)}")
        elif path.read_text(encoding="utf-8") != expected:
            failures.append(f"drift: {path.relative_to(REPO_ROOT)} is stale; run the regenerator")

    # 5) Per-family: fixtures validate against the package schema; the embedded
    #    migration-diff matches the standalone report; the round-trip preserves
    #    unknown fields.
    catalog = load_json(CATALOG_PATH) if CATALOG_PATH.exists() else {"packages": []}
    pkg_by_family = {p["family_id"]: p for p in catalog.get("packages", [])}
    gen_pkg_by_family = {p["family_id"]: p for p in gen.cat.PACKAGES}

    for row in suite.get("suites", []):
        fid = row.get("family_id")
        cat_pkg = pkg_by_family.get(fid)
        if cat_pkg is None:
            failures.append(f"{fid}: family not found in the JSON Schema catalog")
            continue
        schema_file = REPO_ROOT / cat_pkg.get("schema_path", "")
        if not schema_file.exists():
            failures.append(f"{fid}: missing package schema {cat_pkg.get('schema_path')}")
            continue
        pkg_validator = Draft202012Validator(load_json(schema_file))

        gen_pkg = gen_pkg_by_family.get(fid)
        fixtures = {
            "prior": (REPO_ROOT / row.get("prior_fixture_ref", ""), gen.prior_payload(gen_pkg)),
            "current": (REPO_ROOT / row.get("current_fixture_ref", ""), gen.current_payload(gen_pkg)),
            "unsupported": (
                REPO_ROOT / row.get("unsupported_fixture_ref", ""),
                gen.unsupported_payload(gen_pkg),
            ),
        }
        for name, (fpath, expected) in fixtures.items():
            if not fpath_exists(fpath):
                failures.append(f"{fid}: missing {name} fixture {fpath}")
                continue
            actual = load_json(fpath)
            if actual != expected:
                failures.append(f"{fid}: {name} fixture is stale; run the regenerator")
            for err in pkg_validator.iter_errors(actual):
                failures.append(f"{fid}: {name} fixture fails its package schema: {err.message}")

        # The current fixture must advance the primary version, carry the additive
        # field, and preserve at least one unknown field on round-trip.
        current = load_json(fixtures["current"][0]) if fpath_exists(fixtures["current"][0]) else {}
        if current.get(row.get("primary_version_field")) != row.get("current_version"):
            failures.append(f"{fid}: current fixture does not advance the primary version")
        if row.get("added_field") not in current:
            failures.append(f"{fid}: current fixture is missing the additive field")
        declared = set(load_json(schema_file).get("properties", {}))
        unknown = [k for k in current if k not in declared and k != row.get("added_field")]
        if not unknown:
            failures.append(f"{fid}: current fixture carries no unknown field to preserve")
        else:
            preserved = json.loads(json.dumps(current))
            if any(k not in preserved for k in unknown):
                failures.append(f"{fid}: current fixture dropped an unknown field on round-trip")

        # The unsupported fixture must be stamped beyond the published ceiling.
        unsupported = (
            load_json(fixtures["unsupported"][0]) if fpath_exists(fixtures["unsupported"][0]) else {}
        )
        if unsupported.get(row.get("primary_version_field")) != row.get("unsupported_version"):
            failures.append(f"{fid}: unsupported fixture is not stamped at the unsupported version")

        # The embedded migration-diff must match the standalone per-family report.
        report_path = REPO_ROOT / row.get("migration_diff", {}).get("report_ref", "")
        if not report_path.exists():
            failures.append(f"{fid}: missing migration-diff report {report_path}")
        else:
            report = load_json(report_path)
            if report != gen.build_migration_diff_report(gen_pkg, row.get("reader_writer_posture")):
                failures.append(f"{fid}: migration-diff report is stale; run the regenerator")
            diff = row.get("migration_diff", {})
            if report.get("from_version") != diff.get("from_version"):
                failures.append(f"{fid}: report from_version disagrees with the suite")
            if report.get("to_version") != diff.get("to_version"):
                failures.append(f"{fid}: report to_version disagrees with the suite")
            if report.get("change_class") != diff.get("change_class"):
                failures.append(f"{fid}: report change_class disagrees with the suite")
            if report.get("added_fields") != diff.get("added_fields"):
                failures.append(f"{fid}: report added_fields disagree with the suite")

    # 6) Cross-matrix posture consistency and catalog family coverage.
    postures = load_matrix_postures()
    catalog_families = load_catalog_families()
    suite_families = {r.get("family_id") for r in suite.get("suites", [])}
    if catalog_families and suite_families != catalog_families:
        missing = catalog_families - suite_families
        extra = suite_families - catalog_families
        if missing:
            failures.append(f"families missing a compatibility suite: {sorted(missing)}")
        if extra:
            failures.append(f"compatibility suites with no catalog family: {sorted(extra)}")
    for row in suite.get("suites", []):
        fid = row.get("family_id")
        if postures:
            if fid not in postures:
                failures.append(f"{fid}: no matching row in the publication matrix")
            elif postures[fid] != row.get("reader_writer_posture"):
                failures.append(
                    f"{fid}: reader_writer_posture {row.get('reader_writer_posture')} disagrees "
                    f"with matrix posture {postures[fid]}"
                )

    # 7) Path existence.
    for ref in sorted(set(collect_refs(suite))):
        if not is_path_ref(ref):
            continue
        if not (REPO_ROOT / candidate_path(ref)).exists():
            failures.append(f"missing referenced path: {ref}")

    # 8) Negative fixtures must be rejected by the semantic invariants.
    cases_path = gen.NEGATIVE_DIR / "cases.json"
    if cases_path.exists():
        cases = load_json(cases_path).get("cases", [])
        if not cases:
            failures.append("fixtures: cases.json lists no cases")
        for case in cases:
            file = case.get("file")
            fixture_path = gen.NEGATIVE_DIR / file
            if not fixture_path.exists():
                failures.append(f"fixtures: missing {file}")
                continue
            fixture = load_json(fixture_path)
            if not semantic_violations(fixture):
                failures.append(f"fixtures: {file} was not rejected by the semantic invariants")
    else:
        failures.append("fixtures: missing cases.json")

    if failures:
        print("[m5-compat-suite] FAIL", file=sys.stderr)
        for msg in failures[:200]:
            print(f"  - {msg}", file=sys.stderr)
        if len(failures) > 200:
            print(f"  ... ({len(failures) - 200} more)", file=sys.stderr)
        return 1

    print(
        "[m5-compat-suite] OK: suite, fixtures, migration-diff reports, "
        "matrix/catalog consistency, generated docs, paths, and negative fixtures validate"
    )
    return 0


def fpath_exists(path: Path) -> bool:
    return path.exists()


if __name__ == "__main__":
    raise SystemExit(main())
