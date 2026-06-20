#!/usr/bin/env python3
"""Regenerate the efficiency publication-ingestion register.

The register binds every stable-facing surface that describes low-power or
thermal behavior -- docs, in-product help, About, service-health, the
policy-or-admin surface, and support exports -- to the one canonical
efficiency-state claim entry it renders. The canonical entries are the rows of
the M5 efficiency-state governance matrix, so the register can never disagree
with the matrix. The register's derived fields (each binding's rendered
projection, claim-support level, and the narrowed/quarantined propagation
projection) are recomputed with the *same* engine the CI gate
(`ci/check_efficiency_publication_ingestion.py`) uses. Run after the matrix
changes:

    python3 tools/regenerate_efficiency_publication_ingestion.py

then re-run the gate:

    python3 ci/check_efficiency_publication_ingestion.py --repo-root .
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(REPO_ROOT / "ci"))

import check_efficiency_publication_ingestion as gate  # noqa: E402

AS_OF = "2026-06-20"
GENERATED_AT = "2026-06-20T14:00:00Z"

REGISTER_REL = gate.REGISTER_REL
SCHEMA_REL = gate.SCHEMA_REL
MATRIX_REL = gate.MATRIX_REL
ADMIN_FIELDS_REL = gate.ADMIN_FIELDS_REL

SURFACE_LABELS = {
    "docs": "Documentation",
    "help": "In-product help",
    "about": "About surface",
    "service_health": "Service-health surface",
    "admin": "Policy-or-admin surface",
    "support_export": "Support export",
}

SURFACE_DESCRIPTIONS = {
    "docs": "Published documentation pages describing low-power and thermal behavior.",
    "help": "In-product help topics describing low-power and thermal behavior.",
    "about": "The in-product About surface, which advertises only claim-bearing low-power postures.",
    "service_health": "The service-health surface that reports the active low-power posture.",
    "admin": "The policy-or-admin surface that shows the posture and its override rules.",
    "support_export": "Support and diagnostics exports that explain the low-power posture without raw logs.",
}

# The export-safe fields each surface discloses. Every name is in the
# export-safe allowlist; raw telemetry, payloads, and content never appear.
SURFACE_DISCLOSED_FIELDS = {
    "docs": [
        "entry_id",
        "title",
        "m5_surface",
        "efficiency_state",
        "source_of_change",
        "posture",
        "effective_posture",
        "certification_state",
        "claim_support",
        "override_posture",
        "recovery_state",
    ],
    "help": [
        "entry_id",
        "title",
        "m5_surface",
        "efficiency_state",
        "posture",
        "effective_posture",
        "certification_state",
        "claim_support",
        "override_posture",
    ],
    "about": [
        "entry_id",
        "title",
        "posture",
        "effective_posture",
        "certification_state",
        "claim_support",
    ],
    "service_health": [
        "entry_id",
        "m5_surface",
        "efficiency_state",
        "source_of_change",
        "effective_posture",
        "certification_state",
        "claim_support",
        "recovery_state",
    ],
    "admin": [
        "entry_id",
        "m5_surface",
        "posture",
        "effective_posture",
        "certification_state",
        "claim_support",
        "override_posture",
        "recovery_state",
        "fired_narrowing_reasons",
    ],
    "support_export": [
        "entry_id",
        "m5_surface",
        "efficiency_state",
        "source_of_change",
        "posture",
        "effective_posture",
        "certification_state",
        "claim_support",
        "override_posture",
        "recovery_state",
        "fired_narrowing_reasons",
    ],
}

EXPORT_SAFE_FIELD_DESCRIPTIONS = {
    "entry_id": "Stable governance-matrix row id the surface points at.",
    "title": "Reviewable title of the claim entry.",
    "m5_surface": "The M5 surface the entry governs.",
    "efficiency_state": "Active efficiency-state token.",
    "source_of_change": "Source-of-change tokens that drove the state.",
    "posture": "Claimed low-power posture token.",
    "published_claim_ceiling": "Highest posture the entry may publish.",
    "effective_posture": "Narrowed effective posture after governance.",
    "certification_state": "Certification outcome token.",
    "claim_support": "Whether the claim is supported, narrowed, or unsupported.",
    "override_posture": "Whether and how the adaptation may be overridden.",
    "recovery_state": "Staged-recovery state token.",
    "fired_narrowing_reasons": "Reasons that narrowed the claim, for diagnosis.",
}

FORBIDDEN_FIELD_REASONS = {
    "raw_energy_trace": "Raw energy traces are telemetry, not export-safe claim vocabulary.",
    "raw_power_samples": "Raw power samples are telemetry, not export-safe claim vocabulary.",
    "raw_thermal_samples": "Raw thermal samples are telemetry, not export-safe claim vocabulary.",
    "raw_battery_telemetry": "Raw battery telemetry is not export-safe claim vocabulary.",
    "raw_log": "Raw logs may carry content and are never published to a surface.",
    "provider_payload": "Raw provider payloads never cross the publication boundary.",
    "secret_material": "Secret material never crosses the publication boundary.",
    "user_content": "User content never crosses the publication boundary.",
    "file_path": "File paths can leak user content and are never published.",
    "machine_label": "Raw machine labels are not export-safe claim vocabulary.",
}


def surface_locator(surface: str, entry: dict) -> str:
    m5 = entry["m5_surface"]
    if surface == "docs":
        return f"docs/efficiency/publication-ingestion.md#{m5.replace('_', '-')}"
    return f"{surface}.efficiency.low_power.{m5}"


def rendered_projection(entry: dict) -> dict:
    return {
        "efficiency_state": entry["efficiency_state"],
        "source_of_change": list(entry["source_of_change"]),
        "posture": entry["posture"],
        "published_claim_ceiling": entry["published_claim_ceiling"],
        "effective_posture": entry["effective_posture"],
        "certification_state": entry["certification_state"],
        "claim_support": entry["claim_support"],
        "override_posture": entry["override_posture"],
        "recovery_state": entry["recovery_state"],
        "fired_narrowing_reasons": list(entry["fired_narrowing_reasons"]),
    }


def binding_note(surface: str, entry: dict) -> str:
    return (
        f"{SURFACE_LABELS[surface]} renders the {entry['m5_surface']} entry's "
        f"{entry['claim_support']} low-power posture verbatim from the governance matrix."
    )


def build_register(canonical: gate.Canonical) -> dict:
    bindings = []
    for entry_id, entry in canonical.entries.items():
        for surface in gate.required_surfaces_for_entry(entry):
            bindings.append(
                {
                    "record_kind": "efficiency_publication_ingestion_binding",
                    "schema_version": 1,
                    "binding_id": f"ingest.{surface}.{entry_id}",
                    "surface": surface,
                    "surface_locator": surface_locator(surface, entry),
                    "consumes_entry_id": entry_id,
                    "renders_verbatim_from_entry": True,
                    "rendered_projection": rendered_projection(entry),
                    "disclosed_fields": list(SURFACE_DISCLOSED_FIELDS[surface]),
                    "notes": binding_note(surface, entry),
                }
            )

    def rank(level):
        return canonical.rank.get(level, 0)

    narrowed = sorted(
        eid
        for eid, e in canonical.entries.items()
        if rank(e["effective_posture"]) < rank(e["published_claim_ceiling"])
    )
    quarantined = sorted(
        eid
        for eid, e in canonical.entries.items()
        if e["certification_state"] == gate.QUARANTINE_CERTIFICATION
    )

    surface_kinds = [
        {
            "surface": surface,
            "requires_claim_bearing": kind["requires_claim_bearing"],
            "always_required": kind["always_required"],
            "description": SURFACE_DESCRIPTIONS[surface],
        }
        for surface, kind in gate.SURFACE_KINDS.items()
    ]

    claim_support_levels = [
        {
            "level": "supported",
            "from_certification_state": "certified",
            "description": "The published claim is fully supported: the effective posture equals the published ceiling.",
        },
        {
            "level": "narrowed",
            "from_certification_state": "narrowed",
            "description": "A narrowing reason fired; the effective posture is below the published ceiling and every surface renders the narrowed claim.",
        },
        {
            "level": "unsupported",
            "from_certification_state": "quarantined",
            "description": "The claim is quarantined to the undeclared-badge floor; every surface renders it as unsupported, not a stale ceiling.",
        },
    ]

    export_safe_fields = [
        {"field": name, "description": EXPORT_SAFE_FIELD_DESCRIPTIONS[name]}
        for name in gate.EXPORT_SAFE_FIELDS
    ]
    forbidden_fields = [
        {"field": name, "why": FORBIDDEN_FIELD_REASONS[name]}
        for name in gate.FORBIDDEN_FIELDS
    ]

    return {
        "record_kind": "efficiency_publication_ingestion_register",
        "schema_version": 1,
        "register_id": "aureline.efficiency_publication_ingestion",
        "generated_at": GENERATED_AT,
        "register_revision": 1,
        "as_of": AS_OF,
        "title": "Efficiency publication-ingestion register binding every stable-facing low-power surface to one canonical claim entry.",
        "summary": (
            "Documentation, in-product help, the About surface, service-health, the policy-or-admin surface, and support exports "
            "each render one efficiency-state governance-matrix row verbatim instead of cloning low-power prose. A narrowed or "
            "unsupported claim, its override rules, and its recovery state propagate to every surface in the same train, and only "
            "export-safe claim vocabulary crosses the boundary."
        ),
        "governance_matrix_ref": MATRIX_REL,
        "admin_surface_fields_ref": ADMIN_FIELDS_REL,
        "source_refs": [
            SCHEMA_REL,
            MATRIX_REL,
            ADMIN_FIELDS_REL,
            "crates/aureline-shell/src/efficiency/mod.rs",
            "crates/aureline-shell/src/efficiency/governance/mod.rs",
        ],
        "surface_kinds": surface_kinds,
        "claim_support_levels": claim_support_levels,
        "export_safe_fields": export_safe_fields,
        "forbidden_fields": forbidden_fields,
        "bindings": bindings,
        "propagation_projection": {
            "narrowed_entry_ids": narrowed,
            "quarantined_entry_ids": quarantined,
            "surfaces_aligned": True,
            "rationale": (
                "Every consuming surface renders the entry's effective posture and claim-support level, so a narrowed or "
                "unsupported claim cannot survive on one surface while another still advertises the ceiling."
            ),
        },
        "inspection": {
            "how_to_recompute": "Re-derive every binding's projection and the propagation projection from the governance matrix rows and compare to the stored register.",
            "surface_authorization": "Every surface is bound for every entry, except About, which is bound only for entries whose effective posture is claim-bearing.",
            "propagation_rule": "A surface must never publish a stronger claim than the entry's narrowed effective posture.",
            "governance_matrix_ref": MATRIX_REL,
        },
    }


def main() -> int:
    canonical = gate.load_canonical(REPO_ROOT)
    register = build_register(canonical)
    out_path = REPO_ROOT / REGISTER_REL
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(
        json.dumps(register, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"
    )
    print(
        f"wrote {REGISTER_REL}: {len(register['bindings'])} bindings, "
        f"{len(canonical.entries)} entries"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
