# Preview-row packet inputs

These fixtures seed the runtime-heavy and ecosystem-heavy preview-row
packet validator. They are metadata-only examples used by AI, review,
support, and release checks to make sure notebook trust, repair posture,
install-review truth, compatibility labels, and activation budgets stay
explicit on every claimed beta row.

| Fixture | Covers |
|---|---|
| `current_runtime_preview_row_input.json` | Runtime row with current compatibility and activation-budget truth. |
| `current_ecosystem_install_review_input.json` | Ecosystem row with install-review, compatibility-label, and activation-budget truth. |
| `notebook_first_held_out_of_claim_input.json` | Notebook-first row downgraded out of the claimed beta set. |

