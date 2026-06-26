# Content-Ops Metadata for Docs/Help Snippets, Export/Report Headings, Captions, and Translator Notes

This document is the contract for the content-ops metadata catalog. The catalog is
the single source of truth that gives Aureline's *non-runtime* wording — docs/help
snippets, export/report headings, screenshot/demo captions, and translator notes —
the same source/command/version/build context the product already expects from
runtime surfaces. Docs/help, release notes, support exports, CLI/help, and the
screenshot/demo pipeline resolve content-ops provenance through this catalog rather
than maintaining parallel, uncited, versionless captions and headings.

It is the focused content-ops projection of the product-wide
[translation-safe content-ops contract](../../copy/translation_safe_content_ops_contract.md):
the four artifact kinds carried here are exactly the easy-to-corrupt surfaces that
move through translation, screenshot capture, docs/help packaging, and support
workflows. Where that contract owns a canonical id, command id, glossary term,
placeholder kind, or fallback class, it wins; this catalog materializes the metadata
that proves those identities survive into the four claimed M5 artifact kinds.

- Record kind: `m5_content_ops_metadata_catalog`
- Schema: [`schemas/content/m5-content-ops-metadata.schema.json`](../../../schemas/content/m5-content-ops-metadata.schema.json)
- Canonical support export: [`artifacts/content/m5-content-ops-proof/support_export.json`](../../../artifacts/content/m5-content-ops-proof/support_export.json)
- Summary artifact: [`artifacts/content/m5-content-ops-proof/m5_content_ops_metadata.md`](../../../artifacts/content/m5-content-ops-proof/m5_content_ops_metadata.md)
- Fixtures: [`fixtures/content/m5-content-ops-metadata/`](../../../fixtures/content/m5-content-ops-metadata/)
- Producer: `aureline_shell::content::content_ops_metadata::current_content_ops_metadata_catalog_export`
- Headless emitter: `aureline_shell_m5_content_ops_metadata`

## Content-ops metadata entries

A `ContentOpsEntry` is a typed provenance packet for one artifact kind. Each entry
carries a stable, locale-neutral `entry_id`, the `source_ref` its wording came from,
the `command_ref` (command/source/route) it reflects, a `version_context` (product
version + build), translation-safe `placeholder_notes`, a `locale_fallback` posture,
and the reuse `consumers` that must reconstruct it.

The four `ContentArtifactKind`s are exactly the artifacts the lane is required to
carry metadata for:

- `docs_help_snippet` — a docs browser, help, service-health, support, or learning
  prose excerpt carried as a snippet.
- `export_report_heading` — a report heading, support-bundle heading, evidence
  export label, release row, or CSV/JSON companion label.
- `screenshot_demo_caption` — a caption, subtitle, voice-over line, alt text, or
  presentation copy paired with captured product media.
- `translator_note` — a translator-facing review note carrying placeholder
  semantics, glossary refs, or caption-governance guidance for a target string.

`render_provenance` reconstructs a deterministic provenance line — text, kind,
source, command, version, build, capture posture/sync, machine field, and target —
so docs/help, support, release, and screenshot/demo surfaces can explain where their
wording came from and which command/source/version/build it reflects.

## No versionless release/support truth

Rendered artifacts (`docs_help_snippet`, `export_report_heading`,
`screenshot_demo_caption`) declare a product version and build ref, and any entry
marked `release_support_path` is denied if it is versionless. A
`screenshot_demo_caption` additionally declares its `capture_posture` (`live`,
`mocked`, `synthetic`), its `caption_sync_state`, and a `mocked_versus_live_disclosed`
flag — so a caption can never imply live/stable/current product truth while lacking
the metadata that would prove it. Translator notes are review guidance, not rendered
product truth, so they are exempt from the version requirement unless they are
themselves placed on a release/support path.

## Headings pair a human label with a machine code

An `export_report_heading` carries a locale-neutral `machine_field_name` (export
field id / report column id) beside its localizable `canonical_text`. The human label
may localize freely; the machine code never moves. This keeps an exported heading
both translatable and machine-stable, so a downstream report column or CSV/JSON
companion label binds to the code, not the translated prose.

## Translation-safe placeholders

Every placeholder in a rendered string resolves to a `PlaceholderNote` by id, not by
position. A note carries the literal `placeholder` token (e.g. `{count}`), its
locale-neutral `token_id`, a typed `PlaceholderKind`, a `TokenFidelityClass`, a human
`semantic`, and a `fallback`. `count` placeholders declare a plural-rule ref;
`glossary_term_token` and `enumerated_state_token` placeholders resolve through a
controlled glossary ref rather than a translator-local synonym. A `translator_note`
attaches these notes to a variable-rich safety-critical string or heading through its
`target_string_ref`, so the placeholder semantics and fallback posture travel with
the string into translation. The placeholder kinds, token-fidelity classes, and
translator-note classes mirror the product-wide content-ops contract's closed sets.

## Locale fallback posture

Each entry declares a `LocaleFallback`: the authoritative `default_locale`, a
`strategy` (`source_language_route`, `nearest_locale`, `machine_token`, or
`policy_blocked`), the `fallback_chain`, and a disclosure flag. A non-policy-blocked
chain terminates at the source language; a `policy_blocked` fallback names the policy
ref and discloses the block. Non-authoritative fallback is always disclosed.

## Locale neutrality

Machine-facing identity stays locale-neutral while human prose localizes around it.
Entry ids, machine field names, command refs, and placeholder token ids are lowercase
ascii (`[a-z0-9_.]`); placeholder literals are brace-wrapped locale-neutral tokens.
Only `canonical_text`, placeholder `semantic`/`fallback` prose localize. The localized
overlay fixture rewrites every prose field into a pseudo-localized form while keeping
every id, code, token, command ref, locale tag, and posture byte-for-byte identical —
proving a translation can never fork a command id, an export field id, or a
placeholder token into machine identity.

## Cross-consumer reuse

The same entry objects are reconstructed across docs/help, release notes, support
exports, the screenshot/demo pipeline, and CLI/help. The `shared_reuse_entry_ids`
must each span at least `SHARED_ENTRY_MIN_REUSE_CONSUMERS` (3) distinct consumers.
`cross_consumer_reuse` maps each entry to the consumers that reconstruct it, and
validation fails if a shared entry collapses to fewer consumers.

## Validation invariants

`ContentOpsMetadataCatalog::validate` enforces, among others:

- record kind, schema version, and identity are present;
- the eight closed inventories match the canonical token lists;
- entry ids, machine field names, command refs, and placeholder token ids are unique
  where required and locale-neutral;
- every artifact kind, consumer, capture posture, and fallback strategy is
  represented;
- export/report headings carry a machine field name; translator notes carry a class,
  a target ref, and placeholder notes;
- rendered artifacts and release/support-path entries declare version and build;
- screenshot/demo captions declare capture posture, caption-sync state, and the
  mocked-versus-live disclosure;
- variable-rich rendered strings carry a placeholder note per token; count
  placeholders declare plural rules; glossary/enumerated tokens carry glossary refs;
- locale fallback posture is complete and disclosed;
- each shared reuse entry spans at least three consumers;
- the trust-review and consumer-projection invariants all hold;
- the export carries no raw boundary material.

## Acceptance mapping

| Acceptance clause | Resolved by |
|---|---|
| Docs/help, support/report, and screenshot/demo artifacts can explain where their wording came from and which command/source/version/build they reflect. | `source_ref`, `command_ref`, `version_context`, `render_provenance`, and the support export. |
| Translation-safe placeholder notes and fallback posture are available for variable-rich safety-critical strings and headings. | `PlaceholderNote`, the per-token coverage invariant, plural/glossary refs, `LocaleFallback`, and translator notes bound through `target_string_ref`. |
| Release/help/support materials no longer rely on uncited or versionless captions/headings on claimed M5 surfaces. | The `EntryIncomplete`, `MissingVersionContext`, and `CaptionPostureUndeclared` invariants and the `release_help_support_never_versionless` trust-review flag. |
| A screenshot/demo caption never implies live/stable/current truth without build/version/source metadata. | `CapturePosture`, `CaptionSyncState`, `mocked_versus_live_disclosed`, and the `CaptionPostureUndeclared` invariant. |

## Fixtures

The fixtures are valid, export-safe catalog packets minted from the same seed builder
as the canonical export by `aureline_shell_m5_content_ops_metadata`. See
[the fixtures README](../../../fixtures/content/m5-content-ops-metadata/README.md).
