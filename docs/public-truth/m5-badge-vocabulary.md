# Badge vocabulary and explanation drawers

The [descriptor / badge matrix](m5-descriptor-badge-matrix.md) freezes *which* descriptor
families exist, *which* consumers bind them, and *how* a weaker value narrows or blocks a
claim. It does not decide how an individual descriptor value reads on screen — it carries only
an explanation-drawer message *id*. Before this lane each surface resolved those ids locally:
the marketplace shipped its own support-class chips, the About/Help cards shipped their own
provenance states, and docs and support tooling hand-authored their own copy. Identical
descriptor states could look and read differently from one surface to the next.

This document is the contract for the one resolved **badge / explanation-drawer toolkit** every
public-truth surface renders. The `m5_badge_vocabulary` module in `aureline-release` owns the
toolkit and publishes one packet every consumer reads. The machine-readable inventory lives at
[`artifacts/public-truth/m5-badge-vocabulary.json`](../../artifacts/public-truth/m5-badge-vocabulary.json)
(human drawer catalog:
[`artifacts/public-truth/m5-badge-vocabulary-governance.md`](../../artifacts/public-truth/m5-badge-vocabulary-governance.md)),
the release parity proof at
[`artifacts/release/m5-descriptor-parity-proof/badge-vocabulary.json`](../../artifacts/release/m5-descriptor-parity-proof/badge-vocabulary.json),
and the packet schema is
[`schemas/provenance/m5-badge-vocabulary.schema.json`](../../schemas/provenance/m5-badge-vocabulary.schema.json).

## One badge, one label, one drawer

For every controlled-enum value behind a badge the packet publishes one
`BadgeVocabularyEntry`:

- an **export-safe badge id** (`<dimension>.<value_token>`) that survives copy, export, and
  CLI/help summaries unchanged;
- a **user-facing label** drawn from the controlled vocabulary — never surface-local copy;
- a one-line **summary** and the **explanation-drawer** body that read the same on every
  surface;
- a **tone** (`authoritative`, `informational`, `caution`, `blocking`) and its traffic-light
  **signal**;
- the **claim effect** (`none`, `narrows`, `blocks`) the value carries, generated from the same
  downgrade behavior the descriptor lane freezes, so a badge can never read calmer than the
  claim it narrows;
- the **descriptor identity** behind it — badge family, dimension, and value token — so an
  export or copy never loses the value the badge stands for.

The labels are *generated from* the controlled enums the descriptor lane already freezes, so
support and claim terminology cannot drift into local copy. The contract-named terms —
`Official`, `Mirrored`, `Side-loaded`, `Signature verified`, `Attestation available`,
`Not provided`, `Partial`, `Certified`, `Supported`, `Limited`, `Experimental`,
`Retest pending`, and `Evidence stale` — each render as exactly one badge, recorded in the
packet's `required_term_coverage`.

## Four families, eight dimensions

Entries are grouped under the four badge families, one group each. A family can render more
than one descriptor dimension, because a single descriptor carries more than one truth-bearing
facet:

| Badge family | Dimensions | Source enums |
|--------------|------------|--------------|
| `provenance_badge` | `source_origin`, `signature_state` | provenance source class, signature / attestation state |
| `freshness_badge` | `freshness_state`, `evidence_state` | evidence freshness window, evidence completeness |
| `qualification_badge` | `support_class` | support-class claim chips |
| `client_scope_badge` | `client_kind`, `authority_class`, `handoff_requirement` | client scope, authority, handoff |

Each group cites the same family explanation-drawer message id the descriptor matrix already
points at (`public_truth_descriptor.drawer.<descriptor_family>`), so the matrix's pointer now
resolves to *this* vocabulary rather than a parallel one.

The qualification badge renders the user-facing **support-class claim chips** (`Certified`,
`Supported`, `Limited`, `Community`, `Experimental`, `Unsupported`). The descriptor lane's
qualification ladder (`stable`/`beta`/…) is the *derived* narrowing rung the matrix computes,
a different concept; this vocabulary is the claim a user actually sees.

## Weaker states never disappear

The weaker provenance origins — `mirror`, `offline_bundle`, `side_loaded`, and `not_provided` —
are first-class badge ids, never collapsed into omission. Every `caution` badge carries a
`narrows` claim effect and every `blocking` badge carries a `blocks` claim effect, so a stale,
mirrored, side-loaded, scoped, or absent state always reads as the narrowing it is. A narrowed
client scope or weaker evidence can therefore never render as if it carried authority or
capability parity it lacks.

## One vocabulary, every surface

The release center, Help/About, marketplace, docs/help, support exports, and companion handoffs
all render *this* vocabulary rather than parallel badge or copy vocabularies — the `disclosure`
block records that every one of those surfaces reads the same machine-readable entries. Because
each entry carries its descriptor identity, an export or copy keeps the value behind the badge;
because the drawer text is defined once here, the expansion reads identically in the UI, in
CLI/help summaries, in docs, and in support tooling. The packet carries metadata and copy only:
no credential bodies and no raw provider payloads.

The headless emitter is the only mint-from-truth path:

```sh
cargo run -q -p aureline-release --bin aureline_release_m5_badge_vocabulary -- support-export
cargo run -q -p aureline-release --bin aureline_release_m5_badge_vocabulary -- markdown
cargo run -q -p aureline-release --bin aureline_release_m5_badge_vocabulary -- family provenance_badge
cargo run -q -p aureline-release --bin aureline_release_m5_badge_vocabulary -- badge source_origin.mirror
cargo run -q -p aureline-release --bin aureline_release_m5_badge_vocabulary -- term "Signature verified"
cargo run -q -p aureline-release --bin aureline_release_m5_badge_vocabulary -- validate
```
