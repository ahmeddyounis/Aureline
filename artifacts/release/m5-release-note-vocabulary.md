# M5 release-note vocabulary

The frozen controlled vocabulary every what's-new card, release note, and exported summary draws from.
It is generated from the `m5_release_note_evidence` enums in `crates/aureline-release`; the
[`ReleaseNoteVocabulary::canonical`] block embedded in each packet must match these tokens exactly, and
[`ReleaseNoteEvidenceSet::validate`] rejects any drift. Keep this file, the schema, and the enums in
lockstep.

- Schema: [`schemas/release/m5-release-note-evidence-row.schema.json`](../../schemas/release/m5-release-note-evidence-row.schema.json)
- Contract: [`docs/release/m5-release-note-evidence-contract.md`](../../docs/release/m5-release-note-evidence-contract.md)

[`ReleaseNoteVocabulary::canonical`]: ../../crates/aureline-release/src/m5_release_note_evidence/mod.rs
[`ReleaseNoteEvidenceSet::validate`]: ../../crates/aureline-release/src/m5_release_note_evidence/mod.rs

## Change classes

Declaration order is least→most action-demanding. The vocabulary deliberately separates a routine
docs-only note from a behavior-changing or security-sensitive one.

| Token | Behavior-changing / security-sensitive | Requires direct action link | Readiness |
|---|---|---|---|
| `docs_only` | no | no | `informational` |
| `compatibility` | yes | no | `informational` |
| `behavioral` | yes | no | `action_recommended` |
| `policy` | yes | no | `action_recommended` |
| `deprecated` | yes | no | `action_recommended` |
| `migration_required` | yes | yes | `action_required` |
| `admin_action_required` | yes | yes | `action_required` |
| `security` | yes | yes (advisory + direct link) | `action_required` |
| `breaking` | yes | yes | `action_required` |

A *behavior-changing or security-sensitive* note (every class except `docs_only`) must carry at least
one **substantive** evidence link. A note that *requires a direct action link* must additionally link
directly to a setting / import / rollback surface. A `security` note must also link to an advisory.

## Evidence link kinds

| Token | Direct action | Substantive evidence |
|---|---|---|
| `evidence_packet` | no | yes |
| `security_advisory` | no | yes |
| `migration_doc` | no | yes |
| `certification_delta` | no | yes |
| `rollback_control` | yes | yes |
| `setting_surface` | yes | no |
| `import_surface` | yes | no |
| `docs_page` | no | no |

*Direct action* links are the in-app surfaces a user acts on (rollback control, setting, import).
*Substantive evidence* links back the claim (packet, advisory, migration doc, certification delta,
rollback control) as opposed to a bare docs pointer.

## Note readiness

| Token | Gate | Meaning |
|---|---|---|
| `informational` | `governed` | No action; read when convenient. |
| `action_recommended` | `narrowed` | Review or migrate when ready. |
| `action_required` | `blocked` | A setting / import / rollback action is called for. |

Readiness never blocks a workflow; it only classifies how much action a note asks for.

## What's-new card

| Field | Frozen value | Meaning |
|---|---|---|
| `dismissible` | `true` | The card can be dismissed. |
| `reopenable` | `true` | The card can be reopened after dismissal. |
| `blocks_typing` / `blocks_save` / `blocks_restore` / `blocks_recovery` | `false` | The card never blocks a workflow. |
| `dismiss_state` | `active` \| `dismissed` | Whether the card is currently shown. |
| `reopen_surfaces` | `update_center`, `help_center` | Where a dismissed card can be reopened. |

## Reopen surfaces

| Token | Surface |
|---|---|
| `update_center` | The update center. |
| `help_center` | The Help center / About surface. |

## Consumers

| Token | Surface |
|---|---|
| `update_center` | The in-product update center. |
| `whats_new_panel` | The in-product what's-new panel. |
| `help_center` | The Help center / About surface. |
| `docs_help` | The published docs/Help release notes. |
| `release_center` | The release center / public-truth automation. |
| `support_export` | The support export. |

## Gap kinds

| Token | Gate | Cause |
|---|---|---|
| `action_recommended` | `narrowed` | A read note recommends action. |
| `action_required` | `blocked` | A read note requires action. |
| `note_not_published` | `blocked` | A note the consumer reads is not published in the set. |

## Reused vocabularies

Artifact classes, deployment profiles, and channels are reused verbatim from the update /
support-lifecycle vocabulary, and gate / status / signal from the descriptor-badge runtime, so this
communication layer can never drift from the layers above it.

- Artifact classes: `core_runtime`, `extension_packs`, `schema_contracts`, `workspace_state`,
  `configuration`, `language_runtimes`, `docs_help_content`
- Profiles: `managed`, `self_hosted`
- Channels: `stable`, `beta`, `preview`, `nightly`, `lts` (`stable` / `lts` are support-sensitive)
- Gate decisions: `governed`, `narrowed`, `blocked`
