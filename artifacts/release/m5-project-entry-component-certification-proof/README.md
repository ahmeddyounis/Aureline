# M5 Project-Entry Component Surface Certification Proof

Release evidence for M05-843, the closing certification capstone over the frozen
M5 project-entry component matrix (M05-836 / 839 / 840 / 841) and its
first-consumer adoption lane (M05-842).

The packet certifies that the ten reusable project-entry component families
behave consistently on every claimed M5 project-entry surface — Start Center,
command palette, system-open, deep-link, CLI/headless, template/prebuild, clone,
import, and restore. Each surface is scored across five truth axes
(profile/remote badge, restore class, trust posture, first-useful-work routing,
and always-on export parity) and either passes (green), auto-narrows its
interactive claim to the weakest supported ceiling (yellow), or blocks (red)
when a degraded axis is hidden behind a full-truth claim.

The governing invariant is: a degraded axis must produce a visible tier
narrowing, so unsupported or degraded entry paths narrow visibly instead of
inheriting full-truth labels from healthier lanes. Every row cites exactly one
canonical release-proof bundle (`m5-project-entry-component-proof/packet.json`).

## Files

- `support_export.json` — canonical metadata-only packet (`include_str!`-embedded
  by the Rust module and asserted byte-aligned with the seeded builder).
- `matrix.csv` — one certified surface per line for release / support handoff.
- `report.md` — deterministic Markdown summary.

## Regenerate

```
cargo run -p aureline-shell --example dump_project_entry_component_certification
```

- **Rust module:** `crates/aureline-shell/src/m5_project_entry_component_certification/`
- **Boundary schema:** `schemas/ui/m5-project-entry-component-certification.schema.json`
- **Contract doc:** `docs/opening-projects/m5_project_entry_component_certification_contract.md`
