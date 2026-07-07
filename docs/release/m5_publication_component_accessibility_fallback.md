# M5 publication-component accessibility & auto-narrowing (M05-866)

This lane is the accessibility-and-auto-narrowing capstone over the frozen
[M5 release-center component matrix](../../schemas/ui/m5-release-center-components.schema.json).
Where the freeze matrix defines the reusable release candidate card, version-bump row,
publish-target row / review sheet, artifact provenance bundle card, promotion timeline
step, and rollback / revocation row primitives, and the 861–864 implementation lanes
resolve their per-surface truth, this lane certifies — **per component family** — that
publication claims stay keyboard-complete, screen-reader-reachable, CLI/export-safe,
and honestly self-narrowing.

- Boundary schema: [`schemas/ui/m5-publication-component-accessibility-fallback.schema.json`](../../schemas/ui/m5-publication-component-accessibility-fallback.schema.json)
- Release proof: [`artifacts/release/m5-publication-component-accessibility-fallback-proof/`](../../artifacts/release/m5-publication-component-accessibility-fallback-proof/)
- Fixtures: [`fixtures/ui/m5-publication-component-accessibility-fallback/`](../../fixtures/ui/m5-publication-component-accessibility-fallback/)
- Implementation: `crates/aureline-release/src/implement_keyboard_screen_reader_cli_export_parity_and_publication_component_claim_auto_narrowing/`

## What it certifies

Each row keys on one frozen `M5ReleaseCenterComponentFamily` and reuses that frozen
family vocabulary plus the frozen `M5ReleaseCenterRequiredLabel` and
`M5ReleaseCenterDowngradeTrigger` and the shared `M5ReleaseCenterConsumerSurface`
consumer surfaces, so the certified labels stay byte-identical to the matrix and the
sibling primitive packets.

### Keyboard / screen-reader / CLI reach

Every family exposes a keyboard-complete, screen-reader-reachable, and
CLI/headless-reachable path into the same candidate scope / blocker freshness,
public-surface impact, target visibility / mutability / auth source, signature /
attestation / SBOM / digest lineage, rollout ring, and rollback blast radius the rich
surface shows — never a view-only card that strands assistive-tech or headless users.
The hierarchy-heavy family (the artifact provenance bundle card's digest-lineage tree
with its attestation / SBOM sub-rows) additionally binds its tree to a flat list /
textual path.

### Export parity

The support / release / evaluation export reconstructs each component's meaning from
typed tokens and opaque refs **without a screenshot**, preserving the same auth
sources, provenance states, rollout rings, and rollback scopes shown in-product.
Copy / export is offered as text, JSON, and Markdown.

### Honest auto-narrowing

When a publication dimension — evidence freshness, signature / attestation state,
target auth posture, or mirror verification (plus public-surface impact and rollback
blast radius) — is partial, stale, unverified, or policy-blocked, the component's
**publication-support claim** auto-narrows from `certified` / `supported` to the
permitted ceiling and names the binding dimension and frozen trigger while preserving
canonical identity. A component with every dimension verified carries **no** spurious
narrowing, so an old `Certified` or `Supported` label never lingers on degraded
evidence, a masked target-auth posture, or an unverified mirror.

| Condition state | Permitted support ceiling |
| --- | --- |
| `verified` | `certified` |
| `partial` | `degraded` |
| `stale` | `provisional` |
| `unverified` | `unverified` |
| `policy_blocked` | `policy_blocked` |

The binding dimension names the on-topic frozen trigger it governs:

| Binding dimension | Frozen trigger |
| --- | --- |
| `evidence_freshness` | `blocker_freshness_hidden` |
| `public_surface_impact` | `version_bump_impact_unstated` |
| `target_auth_posture` | `target_auth_source_masked` |
| `signature_attestation_state` | `signature_or_attestation_overclaimed` |
| `mirror_verification` | `proof_stale` |
| `rollback_blast_radius` | `rollback_blast_radius_understated` |

The effective support claim is the weakest ceiling across all modeled dimensions,
capped at the family's full claim. A stale, partial, unverified, or policy-blocked
publication can therefore no longer keep an old `Certified` / `Supported` label, and a
public-facing claim can never outrun the proof it is being viewed away from.

### Cross-surface disclosure

The same narrowed state surfaces in the release center, update center / service
health, docs/help, evaluation packs, headless CLI, mirror console, release proof, and
support/admin exports (`M5ReleaseCenterConsumerSurface`), so claim publication and
field triage stay aligned on publication-component downgrade behavior. Every narrower
rendering surface discloses its reduced interactivity and preserves its labels;
nothing is silently dropped.

## Certified rows

Six families, one row each: **2 green / 4 yellow / 0 red**.

| Row | Family | Effective claim | Binding dimension |
| --- | --- | --- | --- |
| `a11y:release-candidate-card` | release candidate card | `certified` (green) | — |
| `a11y:version-bump-row` | version-bump row | `supported` (green) | — |
| `a11y:publish-target-row` | publish-target row | `degraded` | target auth posture (partial) |
| `a11y:artifact-provenance-bundle-card` | artifact provenance bundle card | `unverified` | signature / attestation state (unverified) |
| `a11y:promotion-timeline-step` | promotion timeline step | `provisional` | mirror verification (stale) |
| `a11y:rollback-revocation-row` | rollback / revocation row | `policy_blocked` | rollback blast radius (policy-blocked) |

## Metadata-only boundary

Raw artifacts, signing keys, publish credentials, mirror URLs, and provider cursors
never cross this boundary. The packet carries only typed class tokens, opaque
summary / evidence refs, booleans, and redacted labels.

## Regenerating the proof

The checked-in `support_export.json`, `matrix.csv`, and `report.md` are the one source
of truth, byte-aligned with the `seeded_m5_publication_a11y_fallback_packet()`
builder. Regenerate with:

```
GEN_PUBLICATION_A11Y_ARTIFACTS=1 cargo test -p aureline-release generate_artifacts
```
