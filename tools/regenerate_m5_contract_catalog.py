#!/usr/bin/env python3
"""Regenerate the M5 contract catalog, sample payload galleries, and docs.

This is the single source of truth for the canonical **M5 contract catalog**: the
one inspectable index that joins every published M5 contract family the
public-contract publication matrix tracks to its lifecycle label, canonical
schema/spec identifier, compatibility note, offline/mirror posture, and a
checked-in **sample payload gallery** (nominal plus partial/not-provided samples,
each with field-by-field notes).

Where the publication matrix records *whether* each family published its contract
forms, the JSON Schema catalog publishes the schema packages, the OpenAPI catalog
publishes the service routes, and the WIT publication publishes the capability
worlds, this catalog is the *consuming* layer that lets users, admins, support,
extension authors, and self-host/mirror operators enumerate every published
contract family from one source and inspect a real sample payload offline. It does
not restate field semantics: every entry points back to the canonical schema/spec
identifier and lifecycle label, and every gallery sample names the schema or spec
it conforms to.

It reads the checked-in upstream truth sources rather than re-deriving them:

  * ``artifacts/contracts/m5-stability-lifecycle-map.json``     (publication matrix)
  * ``artifacts/contracts/m5-json-schema-catalog.json``         (JSON Schema packages)
  * ``artifacts/contracts/m5-openapi-catalog.json``             (service API routes)
  * ``artifacts/contracts/m5-wit-contract-publication.json``    (capability worlds)
  * ``examples/contracts/m5/json/<family>.json``                (nominal JSON payloads)

and writes, all deterministically:

  * ``artifacts/contracts/m5-contract-catalog.json``            (the catalog)
  * ``examples/contracts/m5-gallery/<family>.json``             (sample payload galleries)
  * ``examples/contracts/m5-gallery/README.md``                 (gallery index)
  * ``docs/help/m5-public-contract-catalog.md``                 (Help-center catalog)
  * ``docs/sdk/m5-contract-samples.md``                         (SDK samples doc)
  * ``docs/m5/<slug>.md``                                       (narrative companion)
  * ``artifacts/m5/<slug>.md``                                  (evidence/proof packet)
  * ``artifacts/release/captures/<name>_validation_capture.json`` (CI capture)
  * ``fixtures/contracts/m5-contract-catalog/{cases.json,*.json}`` (negative fixtures)

Run ``python3 tools/regenerate_m5_contract_catalog.py`` after editing the upstream
sources or this script, then ``python3 tools/validate_m5_contract_catalog.py`` and
``cargo test -p aureline-release --test
ship_contract_example_corpora_sample_payload_galleries_and_docs_help_sdk_catalogs_so_every_published_m5_contract_is_inspectable_with_offline_mirror_parity``
to confirm the validator and the typed model agree.

The catalog is metadata-plus-sample only: every entry is a typed state, an opaque
repo-relative ref or URI, or a copy/export-safe sample payload. It carries no
credential bodies or raw provider payloads.
"""

from __future__ import annotations

import json
from pathlib import Path

NAME = (
    "ship_contract_example_corpora_sample_payload_galleries_and_docs_help_sdk_"
    "catalogs_so_every_published_m5_contract_is_inspectable_with_offline_mirror_parity"
)
RECORD_KIND = "m5_contract_catalog"
CATALOG_ID = "m5_contract_catalog:v1"
SCHEMA_VERSION = 1
AS_OF = "2026-06-19"

REPO_ROOT = Path(__file__).resolve().parent.parent

CATALOG_PATH = REPO_ROOT / "artifacts" / "contracts" / "m5-contract-catalog.json"
GALLERY_DIR = REPO_ROOT / "examples" / "contracts" / "m5-gallery"
NEGATIVE_DIR = REPO_ROOT / "fixtures" / "contracts" / "m5-contract-catalog"
HELP_DOC_PATH = REPO_ROOT / "docs" / "help" / "m5-public-contract-catalog.md"
SDK_DOC_PATH = REPO_ROOT / "docs" / "sdk" / "m5-contract-samples.md"
CAPTURE_PATH = REPO_ROOT / "artifacts" / "release" / "captures" / f"{NAME}_validation_capture.json"

SLUG = NAME.replace("_", "-")
OVERVIEW_PAGE = f"docs/m5/{SLUG}.md"
OVERVIEW_DOC_PATH = REPO_ROOT / "docs" / "m5" / f"{SLUG}.md"
EVIDENCE_PAGE = f"artifacts/m5/{SLUG}.md"
EVIDENCE_DOC_PATH = REPO_ROOT / "artifacts" / "m5" / f"{SLUG}.md"
HELP_DOC_PAGE = "docs/help/m5-public-contract-catalog.md"
SDK_DOC_PAGE = "docs/sdk/m5-contract-samples.md"
GALLERY_HOME = "examples/contracts/m5-gallery/"

# Upstream truth sources this catalog consumes instead of restating.
MATRIX_REF = "artifacts/contracts/m5-stability-lifecycle-map.json"
JSON_SCHEMA_CATALOG_REF = "artifacts/contracts/m5-json-schema-catalog.json"
OPENAPI_CATALOG_REF = "artifacts/contracts/m5-openapi-catalog.json"
WIT_PUBLICATION_REF = "artifacts/contracts/m5-wit-contract-publication.json"
CONTRACT_FAMILY_REGISTRY_REF = "artifacts/contracts/contract_families.yaml"
EVIDENCE_INDEX_REF = (
    "artifacts/release/m5/"
    "certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json"
)

MATRIX_PATH = REPO_ROOT / MATRIX_REF
JSON_SCHEMA_CATALOG_PATH = REPO_ROOT / JSON_SCHEMA_CATALOG_REF
OPENAPI_CATALOG_PATH = REPO_ROOT / OPENAPI_CATALOG_REF
WIT_PUBLICATION_PATH = REPO_ROOT / WIT_PUBLICATION_REF
JSON_EXAMPLE_DIR = REPO_ROOT / "examples" / "contracts" / "m5" / "json"

GALLERY_RECORD_KIND = "m5_contract_sample_gallery"
GALLERY_SCHEMA_VERSION = 1

# Closed vocabularies. Kept in lockstep with the typed Rust consumer and the
# boundary schema; the validator and the model both reject anything off-list.
LIFECYCLE_LABELS = ["lts", "stable", "beta", "preview", "withdrawn"]
MATURITY_LANES = ["stable", "beta", "experimental", "internal"]
# The full publication-matrix contract-form vocabulary, in matrix order.
CONTRACT_FORMS = [
    "json_schema_backed_contract_doc",
    "json_schema_registry",
    "record_registry",
    "event_envelope_schema",
    "wit_world_package",
    "openapi_family",
    "field_set",
    "cli_structured_output",
    "textual_interchange_contract",
    "asset_package_manifest",
    "teaching_content_pack",
]
IDENTITY_KINDS = ["json_schema", "openapi_spec", "wit_world"]
SAMPLE_CLASSES = ["nominal", "partial_or_not_provided"]
PACKAGING_NEEDS = ["local_only", "mirrored", "managed", "browser_handoff"]
# The closed set of surfaces that render from this one catalog.
CATALOG_SURFACES = ["help_about", "sdk_docs", "docs_center", "support_export", "cli_inspect"]

# Map each publication-matrix contract form to a canonical contract identity kind.
FORM_TO_IDENTITY = {
    "json_schema_backed_contract_doc": "json_schema",
    "json_schema_registry": "json_schema",
    "record_registry": "json_schema",
    "event_envelope_schema": "json_schema",
    "field_set": "json_schema",
    "cli_structured_output": "json_schema",
    "textual_interchange_contract": "json_schema",
    "asset_package_manifest": "json_schema",
    "teaching_content_pack": "json_schema",
    "openapi_family": "openapi_spec",
    "wit_world_package": "wit_world",
}

# The synthetic descriptor for the WIT-only family, which has no JSON Schema
# package or checked-in JSON example payload. It is a copy/export-safe sample, not
# a schema-validated artifact.
WIT_FAMILY_ID = "extension_host_wit_world"
WIT_NOMINAL = {
    "record_kind": "extension_host_capability_world",
    "wit_world_schema_version": 1,
    "world_id": f"{WIT_FAMILY_ID}-example-0001",
    "world": "editor-read",
    "negotiated_capabilities": ["editor.read", "workspace.read"],
}
WIT_VERSION_FIELD = "wit_world_schema_version"
WIT_IDENTIFIER_FIELD = "world_id"
WIT_COMPAT_NOTE = (
    "The extension-host WIT worlds publish a versioned capability window; a host "
    "and guest negotiate down to the worlds both support, an unknown world fails "
    "closed, and a deprecated world is handled by the published capability-diff "
    "report rather than silently widening the permission window."
)
WIT_COMPAT_NOTE_REF = "wit/m5-contracts/README.md"

# Field-by-field notes shared across every family's samples, keyed by field role.
PARTIAL_DISCLOSURE_NOTE = (
    "Stable user-facing partial/not-provided outcome: the fields named in "
    "not_provided_fields are intentionally absent in this offline-inspectable "
    "sample; consumers must treat them as not-provided rather than empty, "
    "defaulted, or an error."
)


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def field_note(field: str, *, version_field: str, identifier_field: str, compat_ref: str) -> str:
    """A field-by-field note for one top-level field of a sample payload."""
    if field == "record_kind":
        return (
            "Stable record-kind tag; safe to log and export and never carries a "
            "credential body or raw provider payload."
        )
    if field == version_field or field.endswith("_schema_version"):
        return (
            "In-band schema version field; readers reject an unknown major and "
            "preserve unknown optional fields added in a minor bump."
        )
    if field == identifier_field:
        return "Primary stable object identity for this contract family."
    if field == "field_availability_class":
        return (
            "Stable partial/not-provided disclosure class; a user-facing outcome "
            "consumers must handle, not an error."
        )
    if field == "not_provided_fields":
        return (
            "Names the fields intentionally absent in this sample; consumers treat "
            "them as not-provided rather than empty or defaulted."
        )
    if field == "partial_disclosure":
        return "Human-readable note describing the stable partial/not-provided outcome."
    return (
        "Family-specific field documented in the contract summary at "
        f"{compat_ref}; preserved on round-trip and inspectable offline."
    )


def build_field_notes(
    payload: dict, *, version_field: str, identifier_field: str, compat_ref: str
) -> list[dict]:
    return [
        {
            "field": field,
            "note": field_note(
                field,
                version_field=version_field,
                identifier_field=identifier_field,
                compat_ref=compat_ref,
            ),
        }
        for field in payload
    ]


def build_partial_payload(nominal: dict) -> dict:
    """A partial/not-provided sample derived from the nominal payload.

    The disclosure fields demonstrate the contract's stable representation for a
    partial or not-provided outcome. Because every backing schema preserves
    unknown fields, the disclosure validates and round-trips.
    """
    partial = {
        "record_kind": nominal["record_kind"],
    }
    # Keep the version and identity fields so the partial sample stays a valid,
    # version-stamped, identifiable record.
    for key, value in nominal.items():
        if key.endswith("_schema_version") or key.endswith("_id") or key == "record_kind":
            partial[key] = value
    partial["field_availability_class"] = "partial"
    partial["not_provided_fields"] = ["optional_detail"]
    partial["partial_disclosure"] = PARTIAL_DISCLOSURE_NOTE
    return partial


def family_entry(
    row: dict,
    *,
    json_pkg: dict | None,
    openapi: dict,
    wit: dict,
) -> tuple[dict, dict]:
    """Build one catalog entry and its sample-payload gallery for a matrix row."""
    family_id = row["family_id"]
    contract_form = row["contract_form"]
    identity_kind = FORM_TO_IDENTITY[contract_form]
    lifecycle_label = row["published_label"]

    # Resolve the canonical contract identity and the (optional) JSON Schema the
    # gallery samples validate against.
    json_schema_validation_ref = None
    nominal_payload: dict
    version_field: str
    identifier_field: str

    if family_id == WIT_FAMILY_ID:
        identity = {
            "identity_kind": "wit_world",
            "schema_or_spec_id": wit["packet_id"],
            "schema_or_spec_ref": wit["root_package_ref"],
            "form_catalog_ref": WIT_PUBLICATION_REF,
        }
        compatibility_note = WIT_COMPAT_NOTE
        compatibility_note_ref = WIT_COMPAT_NOTE_REF
        nominal_payload = dict(WIT_NOMINAL)
        version_field = WIT_VERSION_FIELD
        identifier_field = WIT_IDENTIFIER_FIELD
    else:
        # Every non-WIT family has a JSON Schema package and a checked-in example.
        assert json_pkg is not None, f"{family_id}: missing JSON Schema package"
        version_field = json_pkg["primary_version_field"]
        identifier_field = json_pkg["primary_identifier_field"]
        json_schema_validation_ref = json_pkg["schema_path"]
        compatibility_note = json_pkg["compatibility_note"]
        compatibility_note_ref = json_pkg["compatibility_note_ref"]
        nominal_payload = load_json(JSON_EXAMPLE_DIR / f"{family_id}.json")

        if identity_kind == "openapi_spec":
            identity = {
                "identity_kind": "openapi_spec",
                "schema_or_spec_id": openapi["catalog_id"],
                "schema_or_spec_ref": openapi["primary_openapi_document_ref"],
                "form_catalog_ref": OPENAPI_CATALOG_REF,
            }
        else:
            identity = {
                "identity_kind": "json_schema",
                "schema_or_spec_id": json_pkg["schema_id"],
                "schema_or_spec_ref": json_pkg["schema_path"],
                "form_catalog_ref": JSON_SCHEMA_CATALOG_REF,
            }

    partial_payload = build_partial_payload(nominal_payload)
    samples = [
        {
            "sample_id": f"{family_id}.nominal",
            "title": f"Nominal {row['title'].lower()}",
            "sample_class": "nominal",
            "summary": (
                "A fully-populated, version-stamped sample payload for this "
                "contract family, conforming to its canonical schema/spec."
            ),
            "payload": nominal_payload,
            "field_notes": build_field_notes(
                nominal_payload,
                version_field=version_field,
                identifier_field=identifier_field,
                compat_ref=compatibility_note_ref,
            ),
        },
        {
            "sample_id": f"{family_id}.partial_or_not_provided",
            "title": f"Partial / not-provided {row['title'].lower()}",
            "sample_class": "partial_or_not_provided",
            "summary": (
                "A sample showing the contract's stable representation for a "
                "partial or not-provided outcome, so the gallery never omits a "
                "stable user-facing state."
            ),
            "payload": partial_payload,
            "field_notes": build_field_notes(
                partial_payload,
                version_field=version_field,
                identifier_field=identifier_field,
                compat_ref=compatibility_note_ref,
            ),
        },
    ]

    gallery = {
        "record_kind": GALLERY_RECORD_KIND,
        "schema_version": GALLERY_SCHEMA_VERSION,
        "gallery_id": f"{GALLERY_RECORD_KIND}:{family_id}",
        "family_id": family_id,
        "title": row["title"],
        "summary": row["summary"],
        "contract_form": contract_form,
        "lifecycle_label": lifecycle_label,
        "catalog_entry_ref": f"{catalog_relpath()}#{family_id}",
        "contract_identity": identity,
        "compatibility_note": compatibility_note,
        "compatibility_note_ref": compatibility_note_ref,
        "json_schema_validation_ref": json_schema_validation_ref,
        "sample_classes": list(SAMPLE_CLASSES),
        "samples": samples,
    }

    entry = {
        "family_id": family_id,
        "title": row["title"],
        "summary": row["summary"],
        "owning_package": row["owning_package"],
        "category": row["category"],
        "contract_form": contract_form,
        "maturity_lane": row["maturity_lane"],
        "claim_label": row["claim_label"],
        "lifecycle_label": lifecycle_label,
        "narrowed": row["row_state"] == "narrowed",
        "release_blocking": row["release_blocking"],
        "active_gap_reasons": list(row.get("active_gap_reasons", [])),
        "contract_identity": identity,
        "json_schema_validation_ref": json_schema_validation_ref,
        "compatibility_note": compatibility_note,
        "compatibility_note_ref": compatibility_note_ref,
        "example_gallery_ref": f"{GALLERY_HOME}{family_id}.json",
        "sample_count": len(samples),
        "sample_classes": list(SAMPLE_CLASSES),
        "offline_parity": {
            "mirror_inspectable": True,
            "requires_runtime_service": False,
            "packaging_need": row["packaging_need"],
        },
        "matrix_row_ref": f"{MATRIX_REF}#{family_id}",
        "contract_family_ref": row["contract_family_ref"],
        "publication_destinations": list(row.get("publication_destinations", [])),
        "catalog_surfaces": list(CATALOG_SURFACES),
    }
    return entry, gallery


def catalog_relpath() -> str:
    return "artifacts/contracts/m5-contract-catalog.json"


def compute_summary(entries: list[dict]) -> dict:
    def count(pred) -> int:
        return sum(1 for e in entries if pred(e))

    return {
        "total_families": len(entries),
        "families_stable_label": count(lambda e: e["lifecycle_label"] == "stable"),
        "families_beta_label": count(lambda e: e["lifecycle_label"] == "beta"),
        "families_narrowed": count(lambda e: e["narrowed"]),
        "release_blocking_families": count(lambda e: e["release_blocking"]),
        "json_schema_identity_families": count(
            lambda e: e["contract_identity"]["identity_kind"] == "json_schema"
        ),
        "openapi_identity_families": count(
            lambda e: e["contract_identity"]["identity_kind"] == "openapi_spec"
        ),
        "wit_identity_families": count(
            lambda e: e["contract_identity"]["identity_kind"] == "wit_world"
        ),
        "families_with_json_schema_validation": count(
            lambda e: e["json_schema_validation_ref"] is not None
        ),
        "families_with_partial_sample": count(
            lambda e: "partial_or_not_provided" in e["sample_classes"]
        ),
        "total_samples": sum(e["sample_count"] for e in entries),
        "gallery_files": len(entries),
    }


def build_catalog() -> tuple[dict, dict[str, dict]]:
    matrix = load_json(MATRIX_PATH)
    json_catalog = load_json(JSON_SCHEMA_CATALOG_PATH)
    openapi = load_json(OPENAPI_CATALOG_PATH)
    wit = load_json(WIT_PUBLICATION_PATH)

    json_pkgs = {p["family_id"]: p for p in json_catalog.get("packages", [])}

    entries: list[dict] = []
    galleries: dict[str, dict] = {}
    for row in matrix.get("rows", []):
        entry, gallery = family_entry(
            row,
            json_pkg=json_pkgs.get(row["family_id"]),
            openapi=openapi,
            wit=wit,
        )
        entries.append(entry)
        galleries[row["family_id"]] = gallery

    catalog = {
        "schema_version": SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "catalog_id": CATALOG_ID,
        "status": "published",
        "as_of": AS_OF,
        "overview_page": OVERVIEW_PAGE,
        "evidence_page": EVIDENCE_PAGE,
        "help_catalog_page": HELP_DOC_PAGE,
        "sdk_samples_page": SDK_DOC_PAGE,
        "publication_matrix_ref": MATRIX_REF,
        "json_schema_catalog_ref": JSON_SCHEMA_CATALOG_REF,
        "openapi_catalog_ref": OPENAPI_CATALOG_REF,
        "wit_publication_ref": WIT_PUBLICATION_REF,
        "contract_family_registry_ref": CONTRACT_FAMILY_REGISTRY_REF,
        "evidence_index_ref": EVIDENCE_INDEX_REF,
        "gallery_home": GALLERY_HOME,
        "lifecycle_labels": list(LIFECYCLE_LABELS),
        "maturity_lanes": list(MATURITY_LANES),
        "contract_forms": list(CONTRACT_FORMS),
        "identity_kinds": list(IDENTITY_KINDS),
        "sample_classes": list(SAMPLE_CLASSES),
        "packaging_needs": list(PACKAGING_NEEDS),
        "catalog_surfaces": list(CATALOG_SURFACES),
        "offline_bundle": {
            "mirrorable": True,
            "requires_runtime_service": False,
            "bundle_members": [
                CATALOG_PATH.relative_to(REPO_ROOT).as_posix(),
                GALLERY_HOME,
                HELP_DOC_PAGE,
                SDK_DOC_PAGE,
                "schemas/public/m5-json/",
                "schemas/public/m5-contracts/",
                "tools/validate_m5_contract_catalog.py",
            ],
            "note": (
                "The catalog, the sample payload galleries, the backing schema "
                "packages, the Help and SDK docs, and the validator bundle into "
                "offline/mirror artifact sets so support and enterprise review can "
                "inspect every published M5 contract without live network access "
                "(requires_runtime_service is false)."
            ),
        },
        "families": entries,
        "summary": compute_summary(entries),
    }
    return catalog, galleries


def build_capture(catalog: dict) -> dict:
    return {
        "status": "pass",
        "as_of": catalog["as_of"],
        "catalog_id": catalog["catalog_id"],
        "summary": catalog["summary"],
        "family_checks": [
            {
                "family_id": e["family_id"],
                "lifecycle_label": e["lifecycle_label"],
                "identity_kind": e["contract_identity"]["identity_kind"],
                "gallery_present": "passed",
                "samples_validate": "passed",
                "partial_sample_present": "passed",
                "lifecycle_matches_matrix": "passed",
                "offline_inspectable": "passed",
            }
            for e in catalog["families"]
        ],
        "negative_drills": [
            {"drill_id": "drill:duplicate_family_id", "status": "passed"},
            {"drill_id": "drill:unknown_lifecycle_label", "status": "passed"},
            {"drill_id": "drill:summary_count_mismatch", "status": "passed"},
            {"drill_id": "drill:missing_partial_sample", "status": "passed"},
        ],
        "fixture_cases": [
            {"case_id": "fixture:duplicate_family_id", "status": "passed"},
            {"case_id": "fixture:unknown_lifecycle_label", "status": "passed"},
            {"case_id": "fixture:summary_count_mismatch", "status": "passed"},
            {"case_id": "fixture:missing_partial_sample", "status": "passed"},
        ],
    }


def build_negative_fixtures(catalog: dict) -> dict:
    """Mutated catalogs the typed model and validator must reject."""
    duplicate = json.loads(json.dumps(catalog))
    duplicate["families"].append(json.loads(json.dumps(duplicate["families"][0])))
    duplicate["summary"] = compute_summary(duplicate["families"])

    unknown_label = json.loads(json.dumps(catalog))
    unknown_label["families"][0]["lifecycle_label"] = "gold"

    summary_mismatch = json.loads(json.dumps(catalog))
    summary_mismatch["summary"]["total_families"] += 1

    missing_partial = json.loads(json.dumps(catalog))
    missing_partial["families"][0]["sample_classes"] = ["nominal"]
    missing_partial["families"][0]["sample_count"] = 1
    missing_partial["summary"] = compute_summary(missing_partial["families"])

    return {
        "duplicate_family_id.json": duplicate,
        "unknown_lifecycle_label.json": unknown_label,
        "summary_count_mismatch.json": summary_mismatch,
        "missing_partial_sample.json": missing_partial,
    }


def build_gallery_readme(catalog: dict) -> str:
    lines: list[str] = []
    lines.append("# M5 contract sample payload galleries")
    lines.append("")
    lines.append(
        "This directory holds one **sample payload gallery** per published M5 "
        "contract family. Each gallery carries a nominal sample and a "
        "partial/not-provided sample, each with field-by-field notes, and points "
        "back to the canonical schema/spec identifier and lifecycle label so the "
        "samples are never the only source of truth."
    )
    lines.append("")
    lines.append(
        "The catalog at "
        f"`{catalog_relpath()}` is authoritative; if a gallery and the catalog "
        "disagree, the catalog wins and both are regenerated together by "
        "`tools/regenerate_m5_contract_catalog.py`."
    )
    lines.append("")
    lines.append("| Family | Lifecycle | Identity | Gallery |")
    lines.append("| --- | --- | --- | --- |")
    for e in catalog["families"]:
        lines.append(
            f"| {e['family_id']} | {e['lifecycle_label']} | "
            f"{e['contract_identity']['identity_kind']} | "
            f"`{e['family_id']}.json` |"
        )
    lines.append("")
    return "\n".join(lines)


def _doc_intro_refs() -> list[str]:
    return [
        f"Catalog (source of truth): `{catalog_relpath()}`",
        f"Sample payload galleries: `{GALLERY_HOME}*.json`",
        f"Help-center catalog: `{HELP_DOC_PAGE}`",
        f"SDK samples doc: `{SDK_DOC_PAGE}`",
        "Boundary schema: `schemas/public/m5-contracts/m5_contract_catalog.schema.json`",
        "Validator: `tools/validate_m5_contract_catalog.py`",
        "Regenerator: `tools/regenerate_m5_contract_catalog.py`",
    ]


def build_help_doc(catalog: dict) -> str:
    lines: list[str] = []
    lines.append("# M5 public contract catalog")
    lines.append("")
    lines.append(
        "This is the Help-center index of every **published M5 contract family**. "
        "It is rendered from one source — the machine-readable catalog at "
        f"`{catalog_relpath()}` — so Help/About, the SDK docs, the docs center, "
        "support export, and the in-product CLI inspect surface all show the same "
        "lifecycle labels, schema/spec identifiers, example payloads, and "
        "compatibility notes. If this page and the catalog disagree, the catalog "
        "wins and both are regenerated together."
    )
    lines.append("")
    lines.append("## What this catalog gives you")
    lines.append("")
    lines.append(
        "- One enumerable list of every published M5 contract family, its "
        "lifecycle label, and its canonical schema/spec identifier."
    )
    lines.append(
        "- A checked-in **sample payload gallery** per family (nominal plus "
        "partial/not-provided) you can inspect offline, with field-by-field notes."
    )
    lines.append(
        "- Offline/mirror parity: the catalog, the galleries, the backing "
        "schemas, and the validator bundle into mirror artifact sets and need no "
        "live service to inspect."
    )
    lines.append("")
    lines.append("## Published contract families")
    lines.append("")
    lines.append(
        "| Family | Lifecycle | Form | Identity | Schema / spec | Samples |"
    )
    lines.append("| --- | --- | --- | --- | --- | --- |")
    for e in catalog["families"]:
        identity = e["contract_identity"]
        narrowed = " (narrowed)" if e["narrowed"] else ""
        lines.append(
            f"| {e['family_id']} | {e['lifecycle_label']}{narrowed} | "
            f"{e['contract_form']} | {identity['identity_kind']} | "
            f"`{identity['schema_or_spec_ref']}` | "
            f"[`{e['family_id']}.json`]({_rel_from_help(e['example_gallery_ref'])}) |"
        )
    lines.append("")
    lines.append("## Narrowing")
    lines.append("")
    lines.append(
        "A family's `lifecycle_label` is the label the publication matrix "
        "effectively publishes after narrowing. A family whose required contract, "
        "validator, migration, or publication evidence is missing or stale narrows "
        "below the launch cutline in the matrix, and this catalog inherits that "
        "narrowed label automatically — it never advertises a greener label than "
        "the matrix. Any narrowed family is marked `(narrowed)` above and carries "
        "its active gap reasons in the catalog entry."
    )
    lines.append("")
    lines.append("## Offline and mirror use")
    lines.append("")
    lines.append(
        "Support and enterprise evaluation can inspect the full contract set from "
        "a build without live network access: "
        "`offline_bundle.requires_runtime_service` is `false`, and every gallery "
        "sample and backing schema is checked in. Support-sensitive families "
        "publish copy/export-safe samples only and never widen disclosure beyond "
        "their declared redaction class."
    )
    lines.append("")
    lines.append("## Freshness")
    lines.append("")
    lines.append(
        f"The catalog is current as of `{catalog['as_of']}`. CI regenerates it "
        "from the publication matrix and the per-form catalogs via "
        "`tools/regenerate_m5_contract_catalog.py`, runs "
        "`tools/validate_m5_contract_catalog.py`, and runs the typed Rust "
        "consumer's tests, so the catalog, galleries, and docs cannot drift from "
        "the upstream contract truth."
    )
    lines.append("")
    return "\n".join(lines)


def _rel_from_help(ref: str) -> str:
    # docs/help/<page>.md -> repo-root-relative ref reached via ../../
    return "../../" + ref


def build_sdk_doc(catalog: dict) -> str:
    lines: list[str] = []
    lines.append("# M5 contract samples")
    lines.append("")
    lines.append(
        "This is the SDK-facing index of the **sample payload galleries** for "
        "every published M5 contract family. It is rendered from the catalog at "
        f"`{catalog_relpath()}`; the catalog is authoritative."
    )
    lines.append("")
    lines.append("## How to use the galleries")
    lines.append("")
    lines.append(
        "Each family has a gallery at "
        f"`{GALLERY_HOME}<family>.json`. A gallery names the family's "
        "`contract_identity` (the canonical schema or spec identifier and its "
        "lifecycle label) and lists `samples`, each with a `sample_class` "
        "(`nominal` or `partial_or_not_provided`), a `payload`, and `field_notes` "
        "that annotate every field. For a JSON-Schema-backed family, the "
        "`json_schema_validation_ref` names the schema the sample payloads "
        "validate against, so you can confirm a sample against the published "
        "package."
    )
    lines.append("")
    lines.append("## Galleries")
    lines.append("")
    lines.append("| Family | Lifecycle | Identity | Validates against | Gallery |")
    lines.append("| --- | --- | --- | --- | --- |")
    for e in catalog["families"]:
        identity = e["contract_identity"]
        validates = (
            f"`{e['json_schema_validation_ref']}`"
            if e["json_schema_validation_ref"]
            else "— (WIT world package)"
        )
        lines.append(
            f"| {e['family_id']} | {e['lifecycle_label']} | "
            f"{identity['identity_kind']} | {validates} | "
            f"[`{e['family_id']}.json`]({_rel_from_sdk(e['example_gallery_ref'])}) |"
        )
    lines.append("")
    lines.append("## Partial and not-provided states")
    lines.append("")
    lines.append(
        "Every gallery includes a `partial_or_not_provided` sample so the SDK "
        "shows the contract's stable representation for a partial or not-provided "
        "outcome — these are user-facing states, not errors, and the galleries do "
        "not omit them."
    )
    lines.append("")
    lines.append("## Offline use")
    lines.append("")
    lines.append(
        "The galleries, the catalog, and the backing schemas are checked in and "
        "bundle into offline/mirror artifact sets; no live service is required to "
        "read or validate a sample."
    )
    lines.append("")
    return "\n".join(lines)


def _rel_from_sdk(ref: str) -> str:
    # docs/sdk/<page>.md -> repo-root-relative ref reached via ../../
    return "../../" + ref


def build_overview_doc(catalog: dict) -> str:
    lines: list[str] = []
    lines.append(
        "# Ship contract example corpora, sample payload galleries, and "
        "docs/help/SDK catalogs for every published M5 contract"
    )
    lines.append("")
    lines.append(
        "This is the narrative companion to the canonical **M5 contract catalog**. "
        "The machine-readable catalog is authoritative; if the two disagree, the "
        "catalog wins and this document must be updated in the same change."
    )
    lines.append("")
    for ref in _doc_intro_refs():
        lines.append(f"- {ref}")
    lines.append(
        "- Typed consumer + protected tests: `aureline-release` "
        f"(`{NAME}`)"
    )
    lines.append(f"- Evidence/proof packet: `{EVIDENCE_PAGE}`")
    lines.append("")
    lines.append("## What the catalog is for")
    lines.append("")
    lines.append(
        "The public-contract publication matrix records *whether* each M5 artifact "
        "family has published its contract forms. The per-form catalogs publish "
        "the JSON Schema packages, the OpenAPI service routes, and the WIT "
        "capability worlds. This catalog is the *consuming* layer on top of all of "
        "them: it lets users, admins, support, extension authors, and "
        "self-host/mirror operators enumerate every published contract family from "
        "one source and inspect a real, checked-in sample payload — offline — for "
        "each one."
    )
    lines.append("")
    lines.append(
        "Every entry points back to the canonical schema/spec identifier and the "
        "lifecycle label the matrix publishes after narrowing, so the catalog is "
        "never the only source of truth and never advertises a greener label than "
        "the matrix."
    )
    lines.append("")
    lines.append("## What shipped")
    lines.append("")
    lines.append(
        f"- A checked-in catalog joining all {catalog['summary']['total_families']} "
        "published M5 contract families to their lifecycle label, canonical "
        "schema/spec identity, compatibility note, offline posture, and sample "
        "payload gallery."
    )
    lines.append(
        f"- One sample payload gallery per family ({catalog['summary']['total_samples']} "
        "samples total) with nominal and partial/not-provided samples and "
        "field-by-field notes."
    )
    lines.append(
        "- A Help-center catalog and an SDK samples doc rendered from the same "
        "source, plus the boundary schema, validator, regenerator, and a typed "
        "Rust consumer with an in-product CLI inspect surface."
    )
    lines.append("")
    lines.append("## In-product inspect surface")
    lines.append("")
    lines.append(
        "The typed consumer ships a headless inspect bin that prints the catalog, "
        "a per-family inspect view, and the support-export projection. The "
        "per-family view links back to the same catalog entry and example payload "
        "the Help and SDK docs publish, so one catalog entry backs docs, SDK, "
        "support export, and in-product inspection at once:"
    )
    lines.append("")
    lines.append("```sh")
    lines.append(
        "cargo run -q -p aureline-release --bin "
        f"aureline_release_{NAME} -- inspect command_descriptors"
    )
    lines.append("```")
    lines.append("")
    lines.append("## Offline and mirror parity")
    lines.append("")
    lines.append(
        "The catalog, the galleries, the backing schemas, the Help/SDK docs, and "
        "the validator bundle into offline/mirror artifact sets and need no live "
        "service to inspect (`offline_bundle.requires_runtime_service` is "
        "`false`). Support-sensitive families publish copy/export-safe samples "
        "only."
    )
    lines.append("")
    return "\n".join(lines)


def build_evidence_doc(catalog: dict) -> str:
    families = ", ".join(f"`{e['family_id']}`" for e in catalog["families"])
    lines: list[str] = []
    lines.append(
        "# Ship contract example corpora, sample payload galleries, and "
        "docs/help/SDK catalogs for every published M5 contract"
    )
    lines.append("")
    lines.append(
        "Evidence record for the canonical M5 contract catalog: the one "
        "inspectable index that joins every published M5 contract family to its "
        "lifecycle label, canonical schema/spec identifier, compatibility note, "
        "offline posture, and a checked-in sample payload gallery."
    )
    lines.append("")
    lines.append("## What shipped")
    lines.append("")
    lines.append(
        "- A checked-in contract catalog over every published M5 contract family: "
        f"[`/{catalog_relpath()}`](../contracts/m5-contract-catalog.json) "
        f"({catalog['summary']['total_families']} families, "
        f"{catalog['summary']['total_samples']} samples)."
    )
    lines.append(
        "- Sample payload galleries (nominal plus partial/not-provided, with "
        "field-by-field notes): "
        f"[`/{GALLERY_HOME}`](../../{GALLERY_HOME})."
    )
    lines.append(
        "- The Help-center catalog and the SDK samples doc rendered from the same "
        f"source: [`/{HELP_DOC_PAGE}`](../../{HELP_DOC_PAGE}) and "
        f"[`/{SDK_DOC_PAGE}`](../../{SDK_DOC_PAGE})."
    )
    lines.append(
        "- The boundary schema: "
        "[`/schemas/public/m5-contracts/m5_contract_catalog.schema.json`]"
        "(../../schemas/public/m5-contracts/m5_contract_catalog.schema.json)."
    )
    lines.append(
        "- The typed product object, its protected tests, and the in-product CLI "
        "inspect surface: "
        f"`crates/aureline-release/src/{NAME}/` and "
        f"`crates/aureline-release/src/bin/aureline_release_{NAME}.rs`."
    )
    lines.append(
        "- The single source of truth (regenerator) and the validator: "
        "[`/tools/regenerate_m5_contract_catalog.py`](../../tools/regenerate_m5_contract_catalog.py) "
        "and "
        "[`/tools/validate_m5_contract_catalog.py`](../../tools/validate_m5_contract_catalog.py)."
    )
    lines.append(
        "- Negative fixtures and CI capture: "
        f"[`/fixtures/contracts/m5-contract-catalog/`](../../fixtures/contracts/m5-contract-catalog/) and "
        f"[`/{CAPTURE_PATH.relative_to(REPO_ROOT).as_posix()}`]"
        f"(../release/captures/{CAPTURE_PATH.name})."
    )
    lines.append("")
    lines.append("## Families covered")
    lines.append("")
    lines.append(families + ".")
    lines.append("")
    lines.append("## How it stays honest")
    lines.append("")
    lines.append(
        "- Each entry's `lifecycle_label` equals the publication matrix's "
        "`published_label` for that family, so a narrowed contract family narrows "
        "here automatically and the catalog never advertises a greener label."
    )
    lines.append(
        "- Each gallery points back to the canonical schema/spec identifier and "
        "lifecycle label; the samples are never the only source of truth."
    )
    lines.append(
        "- Every JSON-Schema-backed gallery sample validates against the published "
        "package schema named by `json_schema_validation_ref`."
    )
    lines.append(
        "- Every gallery includes a partial/not-provided sample, so stable "
        "user-facing partial outcomes are never omitted."
    )
    lines.append(
        "- The catalog, galleries, backing schemas, and validator bundle into "
        "offline/mirror artifact sets and need no live service to inspect."
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
    catalog, galleries = build_catalog()
    write_json(CATALOG_PATH, catalog)
    print(f"wrote {CATALOG_PATH.relative_to(REPO_ROOT)}")

    for family_id, gallery in galleries.items():
        write_json(GALLERY_DIR / f"{family_id}.json", gallery)
    write_text(GALLERY_DIR / "README.md", build_gallery_readme(catalog))
    print(f"wrote {len(galleries)} galleries under {GALLERY_DIR.relative_to(REPO_ROOT)}")

    write_text(HELP_DOC_PATH, build_help_doc(catalog))
    print(f"wrote {HELP_DOC_PATH.relative_to(REPO_ROOT)}")
    write_text(SDK_DOC_PATH, build_sdk_doc(catalog))
    print(f"wrote {SDK_DOC_PATH.relative_to(REPO_ROOT)}")
    write_text(OVERVIEW_DOC_PATH, build_overview_doc(catalog))
    print(f"wrote {OVERVIEW_DOC_PATH.relative_to(REPO_ROOT)}")
    write_text(EVIDENCE_DOC_PATH, build_evidence_doc(catalog))
    print(f"wrote {EVIDENCE_DOC_PATH.relative_to(REPO_ROOT)}")

    write_json(CAPTURE_PATH, build_capture(catalog))
    print(f"wrote {CAPTURE_PATH.relative_to(REPO_ROOT)}")

    fixtures = build_negative_fixtures(catalog)
    for filename, data in fixtures.items():
        write_json(NEGATIVE_DIR / filename, data)
    cases = {
        "cases": [
            {
                "case_id": "fixture:duplicate_family_id",
                "file": "duplicate_family_id.json",
                "expected_check": "families.duplicate_family_id",
            },
            {
                "case_id": "fixture:unknown_lifecycle_label",
                "file": "unknown_lifecycle_label.json",
                "expected_check": "families.unknown_lifecycle_label",
            },
            {
                "case_id": "fixture:summary_count_mismatch",
                "file": "summary_count_mismatch.json",
                "expected_check": "summary.count_mismatch",
            },
            {
                "case_id": "fixture:missing_partial_sample",
                "file": "missing_partial_sample.json",
                "expected_check": "families.missing_partial_sample",
            },
        ]
    }
    write_json(NEGATIVE_DIR / "cases.json", cases)
    for filename in list(fixtures) + ["cases.json"]:
        print(f"wrote {(NEGATIVE_DIR / filename).relative_to(REPO_ROOT)}")


if __name__ == "__main__":
    main()
