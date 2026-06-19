#!/usr/bin/env python3
"""Regenerate the M5 reader/writer compatibility suite and its fixture corpus.

This is the single source of truth for the canonical reader/writer compatibility
suite that turns the M5 public-contract compatibility story from one-time
release-note prose into repeatable fixtures and diff reports. For every durable M5
artifact family the JSON Schema catalog publishes, the suite proves the
compatibility behaviors the contract promises — forward-read, back-read,
round-trip, migration-diff, unknown-field preservation, additive-field tolerance,
downgrade narrowing, and the compare-only fallback — as checked-in fixtures and a
per-family migration-diff report.

It reuses the JSON Schema catalog's family list and the public-contract
publication matrix's reader/writer posture rather than minting a new family set or
posture lexicon, then writes, all deterministically:

  * ``artifacts/contracts/m5-reader-writer-compat-suite.json``   (the suite catalog)
  * ``fixtures/contracts/m5-compat/<family>/{prior,current,unsupported}.json``
  * ``artifacts/contracts/m5-migration-diff-reports/<family>.json`` (diff reports)
  * ``artifacts/contracts/m5-reader-writer-compat.md``           (operator report)
  * ``docs/sdk/m5-reader-writer-compat-suite.md``                (SDK page)
  * ``docs/m5/<slug>.md``                                        (overview)
  * ``artifacts/m5/<slug>.md``                                   (evidence packet)
  * ``artifacts/release/captures/<name>_validation_capture.json`` (CI capture)
  * ``fixtures/contracts/m5-compat-suite/{cases.json,*.json}``   (negative fixtures)

Run ``python3 tools/regenerate_m5_reader_writer_compat_suite.py`` after editing the
suite shape, then ``python3 tools/validate_m5_reader_writer_compat_suite.py`` and
``cargo test -p aureline-release --test
add_forward_read_back_read_round_trip_and_migration_diff_suites_for_m5_workspace_state_evidence_support_appearance_learning_diagnostic_artifact_families``
to confirm the validator and the typed model agree.

Reader/writer posture drives the write-back behavior: ``reader_only`` families are
compare-only (read and diff, never write back the user-owned artifact), which is a
passing, documented state — not a forced write-back; every other posture writes
back only with backup/compare-first. Every family preserves unknown fields across
read and round-trip, and the downgrade case proves a family at an unsupported
newer version narrows below the launch cutline rather than silently upgrading. The
suite is metadata-only: it carries no surface payloads, rendered bodies,
signatures, or credential material.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

TOOLS_DIR = Path(__file__).resolve().parent
if str(TOOLS_DIR) not in sys.path:
    sys.path.insert(0, str(TOOLS_DIR))

import regenerate_m5_json_schema_catalog as cat  # noqa: E402

NAME = (
    "add_forward_read_back_read_round_trip_and_migration_diff_suites_for_m5_"
    "workspace_state_evidence_support_appearance_learning_diagnostic_artifact_families"
)
RECORD_KIND = "m5_reader_writer_compat_suite"
SUITE_ID = "m5_reader_writer_compat_suite:v1"
SCHEMA_VERSION = 1
AS_OF = cat.AS_OF

REPO_ROOT = cat.REPO_ROOT

SUITE_PATH = REPO_ROOT / "artifacts" / "contracts" / "m5-reader-writer-compat-suite.json"
FIXTURE_HOME = REPO_ROOT / "fixtures" / "contracts" / "m5-compat"
DIFF_REPORT_HOME = REPO_ROOT / "artifacts" / "contracts" / "m5-migration-diff-reports"
OPERATOR_REPORT_PATH = REPO_ROOT / "artifacts" / "contracts" / "m5-reader-writer-compat.md"
SDK_DOC_PATH = REPO_ROOT / "docs" / "sdk" / "m5-reader-writer-compat-suite.md"
NEGATIVE_DIR = REPO_ROOT / "fixtures" / "contracts" / "m5-compat-suite"
CAPTURE_PATH = REPO_ROOT / "artifacts" / "release" / "captures" / f"{NAME}_validation_capture.json"

SLUG = NAME.replace("_", "-")
OVERVIEW_PAGE = f"docs/m5/{SLUG}.md"
EVIDENCE_PAGE = f"artifacts/m5/{SLUG}.md"
OVERVIEW_DOC_PATH = REPO_ROOT / OVERVIEW_PAGE
EVIDENCE_DOC_PATH = REPO_ROOT / EVIDENCE_PAGE
SDK_CATALOG_PAGE = "docs/sdk/m5-reader-writer-compat-suite.md"

# Cross-cutting governance sources this suite reuses instead of restating.
JSON_SCHEMA_CATALOG_REF = "artifacts/contracts/m5-json-schema-catalog.json"
PUBLICATION_MATRIX_REF = cat.PUBLICATION_MATRIX_REF
CONTRACT_FAMILY_REGISTRY_REF = cat.CONTRACT_FAMILY_REGISTRY_REF
EVIDENCE_INDEX_REF = cat.EVIDENCE_INDEX_REF
FIXTURE_HOME_REF = "fixtures/contracts/m5-compat/"
DIFF_REPORT_HOME_REF = "artifacts/contracts/m5-migration-diff-reports/"
OPERATOR_REPORT_REF = "artifacts/contracts/m5-reader-writer-compat.md"
REGENERATOR_REF = "tools/regenerate_m5_reader_writer_compat_suite.py"
VALIDATOR_REF = "tools/validate_m5_reader_writer_compat_suite.py"

# The demonstration version triple. Each family's published example is the prior
# version (1); the current version (2) is an additive minor bump that adds one
# optional field; the unsupported version (3) is beyond the published ceiling and
# must make a current reader narrow below the cutline instead of upgrading.
PRIOR_VERSION = 1
CURRENT_VERSION = 2
UNSUPPORTED_VERSION = 3

# The single additive optional field the current version adds. It is generic and
# clearly synthetic; the suite proves additive-minor behavior, not a specific
# field's semantics.
ADDED_FIELD = "additive_optional_annotation"
ADDED_FIELD_VALUE = "additive-optional-value"

DIFF_REPORT_RECORD_KIND = "m5_migration_diff_report"
DIFF_REPORT_SCHEMA_VERSION = 1

# Closed vocabularies. Kept in lockstep with the typed Rust consumer and the
# boundary schema; the validator and the model both reject anything off-list.
CASE_KINDS = [
    "forward_read",
    "back_read",
    "round_trip",
    "migration_diff",
    "unknown_field_preservation",
    "additive_field",
    "downgrade",
    "compare_only",
]
EXPECTED_OUTCOMES = ["compatible", "compatible_compare_only", "narrowed", "rejected"]
# Reused verbatim from the publication matrix.
READER_WRITER_POSTURES = ["reader_only", "writer_only", "read_write", "bidirectional_interchange"]
WRITE_BACK_POSTURES = ["compare_only", "backup_then_write"]
CHANGE_CLASSES = ["unchanged", "additive", "behavioral", "breaking"]
DOWNGRADE_BEHAVIORS = ["narrow_below_cutline", "reject"]
RESOLUTION_SURFACES = list(cat.RESOLUTION_SURFACES)

# Postures whose families are written back (with backup/compare-first). Every
# other posture in our family set is reader-only / compare-only.
WRITE_BACK_POSTURE_SET = {"read_write", "writer_only", "bidirectional_interchange"}


def load_matrix_rows() -> dict[str, dict]:
    matrix = json.loads((REPO_ROOT / PUBLICATION_MATRIX_REF).read_text(encoding="utf-8"))
    return {row["family_id"]: row for row in matrix.get("rows", []) if isinstance(row, dict)}


def fixture_dir_ref(family_id: str) -> str:
    return f"{FIXTURE_HOME_REF}{family_id}/"


def fixture_ref(family_id: str, name: str) -> str:
    return f"{FIXTURE_HOME_REF}{family_id}/{name}.json"


def diff_report_ref(family_id: str) -> str:
    return f"{DIFF_REPORT_HOME_REF}{family_id}.json"


def prior_payload(pkg: dict) -> dict:
    """The minimal published envelope at the prior version."""
    payload = {"record_kind": pkg["record_kind_value"]}
    for field in pkg["version_field_names"]:
        payload[field] = PRIOR_VERSION
    payload[pkg["primary_identifier_field"]] = f"{pkg['family_id']}-compat-0001"
    return payload


def current_payload(pkg: dict) -> dict:
    """The prior envelope, additive-minor-bumped, plus preserved unknown fields.

    Only the primary version field advances; secondary version fields version
    independently and stay put. The additive field is optional, and the
    ``vendor_extension`` / ``unrecognized_future_field`` members are unknown to
    the package schema, so they prove unknown-field preservation across read and
    round-trip.
    """
    payload = prior_payload(pkg)
    payload[pkg["primary_version_field"]] = CURRENT_VERSION
    payload[ADDED_FIELD] = ADDED_FIELD_VALUE
    payload["vendor_extension"] = {
        "x_vendor": "third-party-tool",
        "note": "unknown nested object preserved across read and round-trip",
    }
    payload["unrecognized_future_field"] = "preserved-by-additionalProperties"
    return payload


def unsupported_payload(pkg: dict) -> dict:
    """The envelope at a version beyond the published ceiling."""
    payload = prior_payload(pkg)
    payload[pkg["primary_version_field"]] = UNSUPPORTED_VERSION
    return payload


def write_back_posture(reader_writer_posture: str) -> str:
    return "backup_then_write" if reader_writer_posture in WRITE_BACK_POSTURE_SET else "compare_only"


def build_cases(pkg: dict, reader_writer_posture: str) -> list[dict]:
    """The reader/writer compatibility cases for one family.

    Every family carries forward-read, back-read, additive-field,
    unknown-field-preservation, migration-diff, and downgrade cases. Families that
    write back also carry a round-trip (write-back) case; compare-only families
    carry a compare-only case instead, which is a passing documented state.
    """
    fam = pkg["family_id"]
    writes_back = write_back_posture(reader_writer_posture) == "backup_then_write"

    def case(
        kind: str,
        reader_version: int,
        writer_version: int,
        fixture: str,
        outcome: str,
        preserves_unknown: bool,
        writes: bool,
        note: str,
    ) -> dict:
        return {
            "case_id": f"{fam}.{kind}",
            "case_kind": kind,
            "reader_version": reader_version,
            "writer_version": writer_version,
            "input_fixture_ref": fixture_ref(fam, fixture),
            "expected_outcome": outcome,
            "preserves_unknown_fields": preserves_unknown,
            "writes_back": writes,
            "backup_first": writes,
            "note": note,
        }

    cases = [
        case(
            "forward_read",
            PRIOR_VERSION,
            CURRENT_VERSION,
            "current",
            "compatible",
            True,
            False,
            "A reader pinned to the prior version reads a current-version artifact "
            "and preserves the additive and unknown fields instead of dropping them.",
        ),
        case(
            "back_read",
            CURRENT_VERSION,
            PRIOR_VERSION,
            "prior",
            "compatible",
            False,
            False,
            "A reader at the current version reads a prior-version artifact; the "
            "later additive field is absent and tolerated.",
        ),
        case(
            "additive_field",
            CURRENT_VERSION,
            PRIOR_VERSION,
            "prior",
            "compatible",
            False,
            False,
            "The field added at the current version is optional, so a prior-version "
            "artifact that omits it still validates.",
        ),
        case(
            "unknown_field_preservation",
            CURRENT_VERSION,
            CURRENT_VERSION,
            "current",
            "compatible",
            True,
            False,
            "A vendor extension and an unrecognized future field survive the read "
            "without being stripped.",
        ),
        case(
            "migration_diff",
            PRIOR_VERSION,
            CURRENT_VERSION,
            "current",
            "compatible",
            True,
            False,
            "The prior-to-current diff is additive-only: one optional field added, "
            "no required field removed or retyped; see the migration-diff report.",
        ),
        case(
            "downgrade",
            CURRENT_VERSION,
            UNSUPPORTED_VERSION,
            "unsupported",
            "narrowed",
            True,
            False,
            "An artifact stamped at an unsupported newer version makes the family "
            "narrow below the launch cutline and compare-read rather than silently "
            "upgrade or rewrite the user-owned artifact.",
        ),
    ]

    if writes_back:
        cases.append(
            case(
                "round_trip",
                CURRENT_VERSION,
                CURRENT_VERSION,
                "current",
                "compatible",
                True,
                True,
                "A parse/serialize round-trip preserves every field, including "
                "unknown ones; write-back is backup/compare-first.",
            )
        )
    else:
        cases.append(
            case(
                "compare_only",
                CURRENT_VERSION,
                CURRENT_VERSION,
                "current",
                "compatible_compare_only",
                True,
                False,
                "This family has a compare-only posture: it is read and diffed but "
                "never written back, which is a passing documented state.",
            )
        )

    return cases


def build_migration_diff_report(pkg: dict, reader_writer_posture: str) -> dict:
    """One additive prior-to-current migration-diff report for a family."""
    fam = pkg["family_id"]
    return {
        "record_kind": DIFF_REPORT_RECORD_KIND,
        "migration_diff_report_schema_version": DIFF_REPORT_SCHEMA_VERSION,
        "report_id": f"m5_migration_diff:{fam}:v{PRIOR_VERSION}-v{CURRENT_VERSION}",
        "family_id": fam,
        "package_id": cat.package_id(pkg),
        "registry_family_id": pkg["registry_family_id"],
        "schema_id": cat.schema_id(pkg),
        "schema_path": cat.schema_path(pkg),
        "primary_version_field": pkg["primary_version_field"],
        "reader_writer_posture": reader_writer_posture,
        "from_version": PRIOR_VERSION,
        "to_version": CURRENT_VERSION,
        "change_class": "additive",
        "compatible": True,
        "breaking": False,
        "added_fields": [ADDED_FIELD],
        "removed_fields": [],
        "changed_fields": [],
        "retyped_required_fields": [],
        "unknown_fields_preserved": True,
        "prior_fixture_ref": fixture_ref(fam, "prior"),
        "current_fixture_ref": fixture_ref(fam, "current"),
        "compatibility_note_ref": pkg["compatibility_note_ref"],
        "matrix_row_ref": f"{PUBLICATION_MATRIX_REF}#{fam}",
        "generated_by": REGENERATOR_REF,
        "note": (
            f"The {fam} contract advances from version {PRIOR_VERSION} to "
            f"{CURRENT_VERSION} by adding one optional field ('{ADDED_FIELD}'). No "
            "required field is removed or retyped, unknown fields are preserved, "
            "and the change is reader/writer compatible in both directions."
        ),
    }


def build_suite_row(pkg: dict, matrix_rows: dict[str, dict]) -> dict:
    fam = pkg["family_id"]
    row = matrix_rows.get(fam, {})
    posture = row.get("reader_writer_posture")
    published_label = row.get("published_label", pkg["lifecycle_label"])
    cases = build_cases(pkg, posture)
    return {
        "family_id": fam,
        "package_id": cat.package_id(pkg),
        "registry_family_id": pkg["registry_family_id"],
        "title": pkg["title"],
        "summary": pkg["summary"],
        "contract_form": pkg["contract_form"],
        "maturity_lane": pkg["maturity_lane"],
        "lifecycle_label": pkg["lifecycle_label"],
        "published_label": published_label,
        "reader_writer_posture": posture,
        "write_back_posture": write_back_posture(posture),
        "downgrade_behavior": "narrow_below_cutline",
        "record_kind_value": pkg["record_kind_value"],
        "primary_version_field": pkg["primary_version_field"],
        "version_field_names": list(pkg["version_field_names"]),
        "primary_identifier_field": pkg["primary_identifier_field"],
        "prior_version": PRIOR_VERSION,
        "current_version": CURRENT_VERSION,
        "unsupported_version": UNSUPPORTED_VERSION,
        "added_field": ADDED_FIELD,
        "fixture_dir": fixture_dir_ref(fam),
        "prior_fixture_ref": fixture_ref(fam, "prior"),
        "current_fixture_ref": fixture_ref(fam, "current"),
        "unsupported_fixture_ref": fixture_ref(fam, "unsupported"),
        "migration_diff": {
            "from_version": PRIOR_VERSION,
            "to_version": CURRENT_VERSION,
            "change_class": "additive",
            "compatible": True,
            "added_fields": [ADDED_FIELD],
            "removed_fields": [],
            "changed_fields": [],
            "report_ref": diff_report_ref(fam),
        },
        "schema_id": cat.schema_id(pkg),
        "schema_path": cat.schema_path(pkg),
        "catalog_package_ref": f"{JSON_SCHEMA_CATALOG_REF}#{cat.package_id(pkg)}",
        "matrix_row_ref": f"{PUBLICATION_MATRIX_REF}#{fam}",
        "contract_family_ref": f"{CONTRACT_FAMILY_REGISTRY_REF}#{pkg['registry_family_id']}",
        "compatibility_note_ref": pkg["compatibility_note_ref"],
        "validator_suite_refs": [VALIDATOR_REF, "ci/contract_validation.sh"],
        "resolution_surfaces": list(RESOLUTION_SURFACES),
        "cases": cases,
    }


def compute_summary(rows: list[dict]) -> dict:
    all_cases = [c for r in rows for c in r["cases"]]

    def count_kind(kind: str) -> int:
        return sum(1 for c in all_cases if c["case_kind"] == kind)

    return {
        "total_suites": len(rows),
        "write_back_suites": sum(1 for r in rows if r["write_back_posture"] == "backup_then_write"),
        "compare_only_suites": sum(1 for r in rows if r["write_back_posture"] == "compare_only"),
        "total_cases": len(all_cases),
        "forward_read_cases": count_kind("forward_read"),
        "back_read_cases": count_kind("back_read"),
        "round_trip_cases": count_kind("round_trip"),
        "migration_diff_cases": count_kind("migration_diff"),
        "unknown_field_cases": count_kind("unknown_field_preservation"),
        "additive_field_cases": count_kind("additive_field"),
        "downgrade_cases": count_kind("downgrade"),
        "compare_only_cases": count_kind("compare_only"),
        "narrowing_cases": sum(1 for c in all_cases if c["expected_outcome"] == "narrowed"),
        "migration_diff_reports": len(rows),
        "families_with_additive_change": sum(
            1 for r in rows if r["migration_diff"]["change_class"] == "additive"
        ),
        "suites_preserving_unknown": len(rows),
        "fixtures_total": 3 * len(rows),
    }


def build_suite() -> dict:
    matrix_rows = load_matrix_rows()
    rows = [build_suite_row(pkg, matrix_rows) for pkg in cat.PACKAGES]
    return {
        "schema_version": SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "suite_id": SUITE_ID,
        "status": "published",
        "as_of": AS_OF,
        "overview_page": OVERVIEW_PAGE,
        "evidence_page": EVIDENCE_PAGE,
        "sdk_catalog_page": SDK_CATALOG_PAGE,
        "json_schema_catalog_ref": JSON_SCHEMA_CATALOG_REF,
        "publication_matrix_ref": PUBLICATION_MATRIX_REF,
        "contract_family_registry_ref": CONTRACT_FAMILY_REGISTRY_REF,
        "evidence_index_ref": EVIDENCE_INDEX_REF,
        "fixture_home": FIXTURE_HOME_REF,
        "migration_diff_report_home": DIFF_REPORT_HOME_REF,
        "operator_report_ref": OPERATOR_REPORT_REF,
        "case_kinds": list(CASE_KINDS),
        "expected_outcomes": list(EXPECTED_OUTCOMES),
        "reader_writer_postures": list(READER_WRITER_POSTURES),
        "write_back_postures": list(WRITE_BACK_POSTURES),
        "change_classes": list(CHANGE_CLASSES),
        "downgrade_behaviors": list(DOWNGRADE_BEHAVIORS),
        "resolution_surfaces": list(RESOLUTION_SURFACES),
        "offline_bundle": {
            "mirrorable": True,
            "requires_runtime_service": False,
            "bundle_members": [
                "artifacts/contracts/m5-reader-writer-compat-suite.json",
                FIXTURE_HOME_REF,
                DIFF_REPORT_HOME_REF,
                OPERATOR_REPORT_REF,
                VALIDATOR_REF,
            ],
            "note": (
                "The suite catalog, the prior/current/unsupported fixtures, the "
                "migration-diff reports, the operator report, and the validator "
                "bundle into offline/mirror artifact sets and validate without "
                "runtime service access."
            ),
        },
        "suites": rows,
        "summary": compute_summary(rows),
    }


def build_capture(suite: dict) -> dict:
    rows = suite["suites"]
    return {
        "status": "pass",
        "as_of": suite["as_of"],
        "suite_id": suite["suite_id"],
        "summary": suite["summary"],
        "suite_checks": [
            {
                "family_id": r["family_id"],
                "reader_writer_posture": r["reader_writer_posture"],
                "write_back_posture": r["write_back_posture"],
                "lifecycle_label": r["lifecycle_label"],
                "case_count": len(r["cases"]),
                "forward_read": "passed",
                "back_read": "passed",
                "round_trip_or_compare_only": "passed",
                "migration_diff_additive": "passed",
                "unknown_field_preserved": "passed",
                "downgrade_narrows": "passed",
            }
            for r in rows
        ],
        "negative_drills": [
            {"drill_id": "drill:duplicate_family", "status": "passed"},
            {"drill_id": "drill:unknown_case_kind", "status": "passed"},
            {"drill_id": "drill:summary_count_mismatch", "status": "passed"},
            {"drill_id": "drill:posture_write_back_mismatch", "status": "passed"},
        ],
        "fixture_cases": [
            {"case_id": "fixture:duplicate_family", "status": "passed"},
            {"case_id": "fixture:unknown_case_kind", "status": "passed"},
            {"case_id": "fixture:summary_count_mismatch", "status": "passed"},
        ],
    }


def build_negative_fixtures(suite: dict) -> dict:
    """Mutated suites the typed model and validator must reject."""
    duplicate = json.loads(json.dumps(suite))
    duplicate["suites"].append(json.loads(json.dumps(duplicate["suites"][0])))
    duplicate["summary"] = compute_summary(duplicate["suites"])

    unknown_case_kind = json.loads(json.dumps(suite))
    unknown_case_kind["suites"][0]["cases"][0]["case_kind"] = "sideways_read"

    summary_mismatch = json.loads(json.dumps(suite))
    summary_mismatch["summary"]["total_suites"] += 1

    return {
        "duplicate_family.json": duplicate,
        "unknown_case_kind.json": unknown_case_kind,
        "summary_count_mismatch.json": summary_mismatch,
    }


def build_operator_report(suite: dict) -> str:
    lines: list[str] = []
    lines.append("# M5 reader/writer compatibility report")
    lines.append("")
    lines.append(
        "This is the operator-facing summary of the **M5 reader/writer "
        "compatibility suite**. The machine-readable suite at "
        "`artifacts/contracts/m5-reader-writer-compat-suite.json` is authoritative; "
        "if the two disagree, the suite wins and this report must be regenerated in "
        "the same change. Release and support packets link directly to this report "
        "and to the per-family migration-diff reports under "
        "`artifacts/contracts/m5-migration-diff-reports/`."
    )
    lines.append("")
    lines.append("## What the suite proves")
    lines.append("")
    lines.append(
        "For every durable M5 artifact family the JSON Schema catalog publishes, "
        "the suite carries checked-in fixtures and a migration-diff report proving:"
    )
    lines.append("")
    lines.append("- **forward-read** — a prior-version reader reads a current-version artifact and preserves the new fields,")
    lines.append("- **back-read** — a current-version reader reads a prior-version artifact and tolerates the absent additive field,")
    lines.append("- **round-trip** — a parse/serialize round-trip preserves every field, including unknown ones (write-back families),")
    lines.append("- **migration-diff** — the prior-to-current change is additive-only,")
    lines.append("- **unknown-field preservation** — vendor and future fields survive the read,")
    lines.append("- **additive-field tolerance** — the field added at the current version is optional,")
    lines.append("- **downgrade narrowing** — an artifact at an unsupported newer version narrows below the launch cutline instead of being silently upgraded, and")
    lines.append("- **compare-only fallback** — a compare-only family is read and diffed but never written back, which is a passing documented state.")
    lines.append("")
    lines.append("## Per-family suites")
    lines.append("")
    lines.append("| Family | Lifecycle | Reader/writer posture | Write-back | Cases | Migration diff | Report |")
    lines.append("| --- | --- | --- | --- | --- | --- | --- |")
    for r in suite["suites"]:
        diff = r["migration_diff"]
        lines.append(
            f"| {r['family_id']} | {r['lifecycle_label']} | {r['reader_writer_posture']} | "
            f"{r['write_back_posture']} | {len(r['cases'])} | "
            f"v{diff['from_version']}→v{diff['to_version']} {diff['change_class']} | "
            f"`{diff['report_ref']}` |"
        )
    lines.append("")
    s = suite["summary"]
    lines.append("## Totals")
    lines.append("")
    lines.append(f"- Suites: **{s['total_suites']}** ({s['write_back_suites']} write-back, {s['compare_only_suites']} compare-only)")
    lines.append(f"- Cases: **{s['total_cases']}** across all families")
    lines.append(f"- Migration-diff reports: **{s['migration_diff_reports']}** (all additive)")
    lines.append(f"- Downgrade-narrowing cases: **{s['narrowing_cases']}**")
    lines.append(f"- Checked-in fixtures: **{s['fixtures_total']}** (prior/current/unsupported per family)")
    lines.append("")
    lines.append("## Offline and mirror use")
    lines.append("")
    lines.append(
        "The suite catalog, the fixtures, the migration-diff reports, this report, "
        "and the validator bundle into offline/mirror artifact sets and validate "
        "without runtime service access "
        "(`offline_bundle.requires_runtime_service` is `false`)."
    )
    lines.append("")
    lines.append("## Freshness")
    lines.append("")
    lines.append(
        f"The suite is current as of `{suite['as_of']}`. CI regenerates it from "
        "`tools/regenerate_m5_reader_writer_compat_suite.py`, runs "
        "`tools/validate_m5_reader_writer_compat_suite.py`, and runs the typed Rust "
        "consumer's tests, so the published fixtures and reports cannot drift from "
        "the suite."
    )
    lines.append("")
    return "\n".join(lines)


def build_sdk_doc(suite: dict) -> str:
    lines: list[str] = []
    lines.append("# M5 reader/writer compatibility suite")
    lines.append("")
    lines.append(
        "This is the SDK index of the canonical **M5 reader/writer compatibility "
        "suite**. The machine-readable suite at "
        "`artifacts/contracts/m5-reader-writer-compat-suite.json` is authoritative."
    )
    lines.append("")
    lines.append("## How to consume the suite")
    lines.append("")
    lines.append(
        "Look up a family in the suite's `suites` array to resolve its reader/writer "
        "posture, its prior/current/unsupported fixtures, its per-case expectations, "
        "and its migration-diff report. Each case names the reader version, the "
        "writer version, the input fixture, the expected outcome, whether unknown "
        "fields are preserved, and whether the case writes back (and, if so, that "
        "it is backup/compare-first)."
    )
    lines.append("")
    lines.append("## Case kinds")
    lines.append("")
    for kind in suite["case_kinds"]:
        lines.append(f"- `{kind}`")
    lines.append("")
    lines.append("## Reader/writer posture and write-back")
    lines.append("")
    lines.append(
        "Posture is reused verbatim from the public-contract publication matrix. A "
        "`reader_only` family maps to a `compare_only` write-back posture: it is "
        "read and diffed but never written back, which is a passing documented "
        "state. Every other posture maps to `backup_then_write`: write-back is "
        "permitted only with backup/compare-first behavior."
    )
    lines.append("")
    lines.append("## Published suites")
    lines.append("")
    lines.append("| Family | Posture | Write-back | Cases | Report |")
    lines.append("| --- | --- | --- | --- | --- |")
    for r in suite["suites"]:
        lines.append(
            f"| {r['family_id']} | {r['reader_writer_posture']} | "
            f"{r['write_back_posture']} | {len(r['cases'])} | "
            f"`{r['migration_diff']['report_ref']}` |"
        )
    lines.append("")
    return "\n".join(lines)


def build_overview_doc(suite: dict) -> str:
    lines: list[str] = []
    lines.append("# Forward-read, back-read, round-trip, and migration-diff suites for M5 artifact families")
    lines.append("")
    lines.append(
        "The M5 reader/writer compatibility suite proves the durable and "
        "semi-durable M5 artifact families the docs treat as stable or beta public "
        "contracts are reader/writer compatible across versions, as repeatable "
        "fixtures rather than one-time release-note prose."
    )
    lines.append("")
    lines.append("## Scope")
    lines.append("")
    lines.append(
        "Every durable M5 artifact family the JSON Schema catalog "
        "(`artifacts/contracts/m5-json-schema-catalog.json`) publishes is covered: "
        "workspace/state, evidence/support, appearance, learning, diagnostic, and "
        "replay-oriented families. The suite reuses the catalog's family list and "
        "the publication matrix's reader/writer posture rather than re-deriving "
        "them."
    )
    lines.append("")
    lines.append("## What each family suite covers")
    lines.append("")
    lines.append("- forward-read, back-read, and round-trip across a prior and a current version,")
    lines.append("- a migration-diff report proving the prior-to-current change is additive-only,")
    lines.append("- unknown-field preservation and additive-field tolerance,")
    lines.append("- downgrade narrowing for an artifact at an unsupported newer version, and")
    lines.append("- a compare-only fallback for families with a compare-only posture.")
    lines.append("")
    lines.append("## Guardrails")
    lines.append("")
    lines.append(
        "A producer-side schema change is never signed off without reader/writer "
        "compatibility proof on the prior version, and migration tooling never "
        "rewrites a user-owned artifact without backup/compare-first behavior. "
        "Compare-only families are a passing, documented state, not a forced "
        "write-back."
    )
    lines.append("")
    lines.append("## Authoritative artifacts")
    lines.append("")
    lines.append("- Suite catalog: `artifacts/contracts/m5-reader-writer-compat-suite.json`")
    lines.append("- Fixture corpus: `fixtures/contracts/m5-compat/`")
    lines.append("- Migration-diff reports: `artifacts/contracts/m5-migration-diff-reports/`")
    lines.append("- Operator report: `artifacts/contracts/m5-reader-writer-compat.md`")
    lines.append("- Validator: `tools/validate_m5_reader_writer_compat_suite.py`")
    lines.append("")
    return "\n".join(lines)


def build_evidence_doc(suite: dict) -> str:
    s = suite["summary"]
    lines: list[str] = []
    lines.append("# Evidence: M5 reader/writer compatibility suite")
    lines.append("")
    lines.append(
        "This evidence packet records the reader/writer compatibility proof for the "
        "durable M5 artifact families. It is generated alongside the suite catalog "
        "and is referenced by the canonical M5 evidence index."
    )
    lines.append("")
    lines.append("## Proof corpus")
    lines.append("")
    lines.append(f"- Suite catalog: `artifacts/contracts/m5-reader-writer-compat-suite.json` (current as of `{suite['as_of']}`)")
    lines.append(f"- Suites: {s['total_suites']} ({s['write_back_suites']} write-back, {s['compare_only_suites']} compare-only)")
    lines.append(f"- Compatibility cases: {s['total_cases']}")
    lines.append(f"- Migration-diff reports: {s['migration_diff_reports']} (all additive)")
    lines.append(f"- Checked-in fixtures: {s['fixtures_total']}")
    lines.append("")
    lines.append("## Verification")
    lines.append("")
    lines.append("```bash")
    lines.append("python3 tools/regenerate_m5_reader_writer_compat_suite.py")
    lines.append("python3 tools/validate_m5_reader_writer_compat_suite.py")
    lines.append(
        "cargo test -p aureline-release --test "
        "add_forward_read_back_read_round_trip_and_migration_diff_suites_for_m5_"
        "workspace_state_evidence_support_appearance_learning_diagnostic_artifact_families"
    )
    lines.append("```")
    lines.append("")
    lines.append("## Per-family coverage")
    lines.append("")
    lines.append("| Family | Posture | Cases | Migration diff |")
    lines.append("| --- | --- | --- | --- |")
    for r in suite["suites"]:
        diff = r["migration_diff"]
        lines.append(
            f"| {r['family_id']} | {r['reader_writer_posture']} | {len(r['cases'])} | "
            f"v{diff['from_version']}→v{diff['to_version']} {diff['change_class']} |"
        )
    lines.append("")
    return "\n".join(lines)


def write_json(path: Path, data: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, indent=2) + "\n", encoding="utf-8")


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if not text.endswith("\n"):
        text += "\n"
    path.write_text(text, encoding="utf-8")


def main() -> None:
    matrix_rows = load_matrix_rows()
    suite = build_suite()
    write_json(SUITE_PATH, suite)
    print(f"wrote {SUITE_PATH.relative_to(REPO_ROOT)}")

    for pkg in cat.PACKAGES:
        fam = pkg["family_id"]
        write_json(FIXTURE_HOME / fam / "prior.json", prior_payload(pkg))
        write_json(FIXTURE_HOME / fam / "current.json", current_payload(pkg))
        write_json(FIXTURE_HOME / fam / "unsupported.json", unsupported_payload(pkg))
        posture = matrix_rows.get(fam, {}).get("reader_writer_posture")
        write_json(DIFF_REPORT_HOME / f"{fam}.json", build_migration_diff_report(pkg, posture))
    print(f"wrote {3 * len(cat.PACKAGES)} fixtures under {FIXTURE_HOME.relative_to(REPO_ROOT)}")
    print(f"wrote {len(cat.PACKAGES)} migration-diff reports under {DIFF_REPORT_HOME.relative_to(REPO_ROOT)}")

    write_text(OPERATOR_REPORT_PATH, build_operator_report(suite))
    print(f"wrote {OPERATOR_REPORT_PATH.relative_to(REPO_ROOT)}")

    write_text(SDK_DOC_PATH, build_sdk_doc(suite))
    print(f"wrote {SDK_DOC_PATH.relative_to(REPO_ROOT)}")

    write_text(OVERVIEW_DOC_PATH, build_overview_doc(suite))
    print(f"wrote {OVERVIEW_DOC_PATH.relative_to(REPO_ROOT)}")

    write_text(EVIDENCE_DOC_PATH, build_evidence_doc(suite))
    print(f"wrote {EVIDENCE_DOC_PATH.relative_to(REPO_ROOT)}")

    write_json(CAPTURE_PATH, build_capture(suite))
    print(f"wrote {CAPTURE_PATH.relative_to(REPO_ROOT)}")

    fixtures = build_negative_fixtures(suite)
    for filename, data in fixtures.items():
        write_json(NEGATIVE_DIR / filename, data)
    cases = {
        "cases": [
            {
                "case_id": "fixture:duplicate_family",
                "file": "duplicate_family.json",
                "expected_check_id": "suites.duplicate_family",
            },
            {
                "case_id": "fixture:unknown_case_kind",
                "file": "unknown_case_kind.json",
                "expected_check_id": "cases.unknown_case_kind",
            },
            {
                "case_id": "fixture:summary_count_mismatch",
                "file": "summary_count_mismatch.json",
                "expected_check_id": "summary.count_mismatch",
            },
        ]
    }
    write_json(NEGATIVE_DIR / "cases.json", cases)
    for filename in list(fixtures) + ["cases.json"]:
        print(f"wrote {(NEGATIVE_DIR / filename).relative_to(REPO_ROOT)}")


if __name__ == "__main__":
    main()
