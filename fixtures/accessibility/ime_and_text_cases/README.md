# Accessibility IME and text cases

Reviewable fixture corpus for the accessibility and IME packet family.
These files seed the cases the platform-input matrix and shell
conformance checklist cite.

Shared shape:

- `fixture_id` — stable id for the case
- `title` — short human-readable label
- `scenario` — what the case is trying to prove
- `platform_profile_refs` or `platform_profile_scope` — named profile
  rows from `artifacts/platform/claimed_desktop_profiles.yaml`
- `surface_groups` — values from
  `artifacts/accessibility/platform_input_matrix.yaml`
- `assistive_technology_refs` — AT rows when the case is AT-sensitive
- `input_row_refs` / `locale_row_refs` — claimed input or locale rows
- `steps` — concrete actions and the expected focus/announcement or text
  result after each step
- `expected_results` — result state and the failure classes this case is
  meant to catch

Seeded cases:

- `editor_ime_dead_key_altgr_emoji_commit.yaml`
- `mixed_direction_technical_strings.yaml`
- `copy_representation_parity.yaml`
- `virtualized_selection_scope.yaml`
- `range_selection_anchor_stability.yaml`
- `multi_window_mixed_dpi_composition.yaml`
- `platform_input_feature_unavailable.yaml`
