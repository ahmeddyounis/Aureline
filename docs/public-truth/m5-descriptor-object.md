# M5 public-truth descriptor objects

The artifact-bound public-truth **descriptor object** is the machine-readable record a claimed
M5 artifact carries to say where it came from, how its evidence is signed and how fresh it is,
what support class it qualifies for, and which client scope and authority it runs under. It is
the layer beneath the
[descriptor / badge governance matrix](m5-descriptor-badge-matrix.md): the matrix freezes which
descriptor *families* exist and which consumers render them; a descriptor object is the typed,
versioned *instance* an artifact attaches.

- Object / registry schema: `schemas/provenance/m5-descriptor-object.schema.json`
- Published registry: `artifacts/public-truth/descriptors/m5-descriptor-object-registry.json`
- Release parity proof: `artifacts/release/m5-descriptor-parity-proof/descriptor-objects.json`
- Runtime: `crates/aureline-release/src/m5_descriptor_object/`
- Emitter: `cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_object -- registry`

## Controlled enums

Every descriptor object is built only from frozen controlled vocabularies, so a nearby surface
can never invent a quasi-equivalent state:

| Facet | Vocabulary |
|-------|------------|
| `source_class` | `first_party_signed`, `vendor`, `community`, `mirror`, `offline_bundle`, `side_loaded`, `not_provided` |
| `signature_state` | `signed_attested`, `signed_unverified`, `attestation_only`, `unsigned`, `signature_invalid`, `not_provided` |
| `freshness_state` | `current`, `stale`, `expired`, `missing` |
| `freshness_evidence` / `qualification_evidence` | `complete`, `limited`, `partial`, `retest_pending`, `evidence_stale`, `not_provided` |
| `support_class` | `stable`, `beta`, `preview`, `experimental`, `deprecated`, `unavailable` |
| `client_kind` | `desktop_full`, `companion_scoped`, `mobile_companion`, `embedded_panel`, `browser_reference`, `handoff_only` |
| `authority_class` | `full_authority`, `scoped_authority`, `reference_only`, `handoff_only`, `not_provided` |
| `handoff_requirement` | `not_required`, `desktop_handoff_required`, `console_handoff_required`, `not_provided` |

Missing or partial evidence is first-class, never an omission: `not_provided`, `partial`,
`retest_pending`, `evidence_stale`, and `limited` all survive serialization as explicit tokens.

## Effective qualification is derived, never asserted

A descriptor object does not take its effective claim on faith. Every weaker value it carries
produces a named `narrowing` (`facet`, `token`, `effect`, `effective_floor`,
`reason_message_id`), and the effective qualification is the claimed support class floored at
every narrowing:

- **Clean** — first-party-signed, attested, current, complete, full desktop authority, no
  handoff — stands at its claimed class (e.g. `stable`).
- **Narrow** — any weaker-but-present value (a mirror, an unverified signature, stale or partial
  evidence, a scoped companion, a required handoff) floors the claim at `beta`.
- **Block** — absent provenance (`source_class: not_provided`), an invalid signature, expired or
  missing freshness, or absent evidence floors the claim at `unavailable`.

A narrowed client or weaker evidence can therefore never imply authority or capability parity it
does not have, and the reason is always inspectable rather than hidden.

## Identity and artifact binding survive export/import

Each object preserves its `descriptor_id` and a structured `artifact_ref` (`artifact_id`,
`artifact_family`, `artifact_kind`, `schema_ref`, `content_digest_ref`) as typed fields — never
flattened to a single string — so a consumer can rejoin a descriptor to its artifact after an
export/import round-trip. The runtime proves this with a serialize → deserialize check in the
registry conformance review.

## One runtime, every consumer

The `M5DescriptorObjectRegistry` is the single inspectable, serde-serializable truth packet the
public-truth consumers read: the release center, Help/About, marketplace, docs/help,
certification, evaluation packs, support exports, and companion handoffs. The packet carries
metadata and refs only — no credential bodies or raw provider payloads — and a redaction scan
runs as part of validation.
