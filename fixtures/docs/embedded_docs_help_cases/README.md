# Embedded docs / help parity cases

Worked YAML fixtures for the cross-surface parity packet frozen in
[`/artifacts/docs/embedded_docs_help_parity_packet.md`](../../../artifacts/docs/embedded_docs_help_parity_packet.md).
Each fixture binds one logical scenario across four surface lanes
(embedded docs / help pane, Help / About pane, onboarding guided
surfaces, exported support / help links) so the parity audit can
compare panes field-for-field without inventing surface-local
vocabulary.

The single-surface help-pane state fixtures live under
[`/fixtures/docs/help_pane_cases/`](../help_pane_cases/) and validate
against
[`/schemas/docs/help_pane_state.schema.json`](../../../schemas/docs/help_pane_state.schema.json);
the fixtures here compose those single-surface payloads with the
About, onboarding, and support-export rows so a reviewer can read
one parity row per case rather than chasing four separate fixtures.

## Intended usage

- **Parity audit corpus.** The cross-surface parity audit emits one
  parity row per case and compares the four lane rows
  field-for-field. A lane that disagrees on any axis listed in the
  parity packet routes a `public_drift_item_record` against the
  affected pack / route / example.
- **Verification lanes.** The docs-public-truth and embedded-
  boundary verification lanes read these fixtures so cached /
  mirrored / stale / not-installed / locale-blocked / policy-
  narrowed states stay first-class on every consuming surface.
- **Reviewer worked corpus.** Reviewers walk one case at a time
  rather than re-deriving cross-surface rules per scenario; the
  packet's normative rules pin the answer for every lane.

## Required state coverage

The fixture set covers every state class the parity packet names.
Adding a new case requires a new fixture rather than overloading an
existing one.

- [`cached_only_owner_unreachable_parity.yaml`](./cached_only_owner_unreachable_parity.yaml)
  — cached-only because the canonical owner is unreachable; all
  four lanes renderable with the typed cached-only disclosure.
- [`mirror_only_offline_parity.yaml`](./mirror_only_offline_parity.yaml)
  — air-gapped signed-mirror copy with no live owner contact;
  external-open forbidden by deployment policy.
- [`stale_example_disclosed_inline_parity.yaml`](./stale_example_disclosed_inline_parity.yaml)
  — partially-stale curated pack rendering a stale example with
  the typed inline disclosure on every consuming lane.
- [`locale_unavailable_not_installed_parity.yaml`](./locale_unavailable_not_installed_parity.yaml)
  — requested locale missing and not installed; pane / onboarding
  suppressed; About / support-export degrade to primary locale
  with the typed disclosure.
- [`not_installed_pack_parity.yaml`](./not_installed_pack_parity.yaml)
  — referenced pack with no copy resident; pane / onboarding
  suppressed; About lists pack as not-installed; support-export
  omits the pack body.
- [`external_open_allowed_optional_same_object_parity.yaml`](./external_open_allowed_optional_same_object_parity.yaml)
  — in-product render plus optional system-browser route to the
  canonical owner of the same logical object.
- [`external_open_blocked_by_policy_parity.yaml`](./external_open_blocked_by_policy_parity.yaml)
  — admin policy forbids browser handoff; every lane discloses
  the policy narrowing on the primary surface.
- [`policy_limited_pane_suppressed_parity.yaml`](./policy_limited_pane_suppressed_parity.yaml)
  — pane suppressed entirely by admin policy; About / support-
  export carry the policy narrowing in the known-limits row.

## Related artifacts and schemas

- [`/artifacts/docs/embedded_docs_help_parity_packet.md`](../../../artifacts/docs/embedded_docs_help_parity_packet.md)
  — parity packet that pins the lanes, axes, and parity rules.
- [`/artifacts/docs/stale_example_audit_rows.yaml`](../../../artifacts/docs/stale_example_audit_rows.yaml)
  — audit-row companion for stale-example parity failures.
- [`/schemas/docs/help_pane_state.schema.json`](../../../schemas/docs/help_pane_state.schema.json)
  — embedded docs / help pane state-record schema.
- [`/schemas/docs/destination_descriptor.schema.json`](../../../schemas/docs/destination_descriptor.schema.json)
  — destination descriptor (route / trust / boundary / external-
  open) re-exported by every lane.
- [`/schemas/docs/docs_pack_manifest.schema.json`](../../../schemas/docs/docs_pack_manifest.schema.json)
  — docs-pack source / version / signing / mirror lineage / locale
  truth.
- [`/schemas/about/about_card.schema.json`](../../../schemas/about/about_card.schema.json)
  — About card the Help / About lane projects from.
- [`/schemas/docs/onboarding_pack_state.schema.json`](../../../schemas/docs/onboarding_pack_state.schema.json)
  — onboarding pack state the onboarding-guided lane projects from.
- [`/schemas/public_truth/public_drift_item.schema.json`](../../../schemas/public_truth/public_drift_item.schema.json)
  — drift-item shape filed when parity fails.
