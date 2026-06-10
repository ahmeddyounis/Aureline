# Signed and shared recipe packs, safe automation graduation, and preview-first replay

This contract ships user-authored automation into one export-safe truth packet
whose unit of truth is a recipe-pack row. Shell, docs, support export, and
release tooling consume the packet directly instead of re-describing a pack's
signature, automation authority, or replay posture by hand.

- Packet type: `aureline_ai::implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay::RecipePackGraduationPacket`
- Schema: [`schemas/ai/implement-signed-and-shared-recipe-packs-safe-automation-graduation-and-preview-first-replay.schema.json`](../../../schemas/ai/implement-signed-and-shared-recipe-packs-safe-automation-graduation-and-preview-first-replay.schema.json)
- Support export: [`artifacts/ai/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay/support_export.json`](../../../artifacts/ai/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay/support_export.json)
- Fixtures: [`fixtures/ai/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay/`](../../../fixtures/ai/m5/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay/)

This lane projects from the recorded-macro / declarative-recipe contract in
[`docs/automation/recipe_and_macro_contract.md`](../recipe_and_macro_contract.md):
recipe packs are declarative, content-addressed, and gated, and they reuse the
frozen command preview, approval, and audit rules rather than inventing a parallel
scripting surface. It reuses the tool-gateway side-effect and approval
vocabularies, the routing-policy provider/locality mode vocabulary, and the frozen
M5 qualification, downgrade, and rollback-posture vocabularies — it does not fork a
parallel set of terms.

## The recipe-pack row

Each `RecipePackRow` binds, for one signed and shareable recipe pack:

| Field | Meaning |
| --- | --- |
| `pack_id`, `pack_label`, `pack_family_label`, `pack_version` | Identity, label, family, and version. |
| `manifest_content_address` | Content address of the manifest bytes, proving the exact pack a replay rode. |
| `descriptor_pack_ref` | Opaque link to the matching automation descriptor pack, when one exists. |
| `publisher_source_class`, `publisher_identity_ref` | Who published the pack and the signed identity record. |
| `signature_class` | Author, organization, managed-channel, author-and-organization, or unsigned-local-only. |
| `share_scope_class` | User-local, workspace-local, organization-managed channel, portable profile export, or support bundle export. |
| `resolved_mode` | Local, BYOK, managed, or enterprise-gateway mode the pack resolves to. |
| `automation_authority_class` | The automation authority the pack has graduated to. |
| `state` | Admitted, pending first-use review, policy-blocked, trust-blocked, quarantined, or withdrawn. |
| `claimed_qualification` | Stable, Beta, Preview, Experimental, Held, or Unavailable. |
| `step_disclosures` | One disclosure per effect the pack's steps can produce: side-effect class, replay preview, approval gate, audit, reversibility, and a review-safe disclosure label. |
| `downgrade_rules` | Closed set of triggers that narrow the claim. |
| `rollback_posture`, `rollback_verified` | Reversal posture for a pack-policy change and whether it was drilled. |
| `evidence_packet_refs` | Evidence backing a claimed pack. |

## Automation authority and safe graduation

The `automation_authority_class` is the safe-automation-graduation axis. It bounds
the strongest effect a pack may produce:

- `inspect_only_no_authority` — the pack may produce no mutating effect at all.
- `local_reversible_only` / `local_with_approval` — local edits, optionally gated.
- `external_reversible_with_approval` — external reversible effects behind a gate.
- `external_irreversible_admin_gated` — an irreversible external publish behind an
  admin-approval gate.
- `managed_only_template_authority` — a managed-only template distributed on the
  managed channel.

A pack that discloses an irreversible publish must have graduated to an admin-gated
or managed-template authority; a managed-only template authority must resolve to a
managed or enterprise-gateway mode.

## Preview-first replay

Every disclosure carries a `replay_preview`. A mutating step previews before its
replay applies — a full preview, a diff, or a dry run — so a replay never surprises
the user with an effect they did not see first. An inspect-only step needs no
preview; a step with no available preview must block. `RecipePackRow::is_preview_first`
projects the guarantee that every mutating step previews before replay.

## Invariants enforced by validation

`RecipePackGraduationPacket::validate` returns a closed set of typed violations.
Signed and shared recipe packs follow the same preview, policy, and audit rules as
first-party commands:

- Every pack carries a content-addressed manifest, advertises at least one step
  disclosure, gives each disclosure a label, and discloses no side-effect class
  twice.
- A signed pack carries a publisher identity; a pack shared beyond a local
  workspace must be signed; a managed-channel pack must resolve to a managed or
  enterprise-gateway mode and carry organization authority.
- A pack with no authority discloses no mutating effect; an irreversible publish
  requires an admin-gated or managed-template authority; a managed-only template
  authority runs on a managed mode.
- A mutating step previews before replay, carries a real approval gate, and is
  audited.
- An irreversible external publish must be externally auditable (run-record
  timeline or support export), and a declared reversibility must agree with the
  effect class.
- A blocked pack — policy-blocked, trust-blocked, quarantined, or withdrawn — may
  not keep a Stable, Beta, or Preview claim; a pack pending first-use review may not
  claim Stable.
- A claimed pack carries evidence refs, has a verified rollback path when its
  posture can be reversed, and carries a closed downgrade rule set that includes the
  proof-stale and provider-unavailable triggers and only narrows below the claimed
  qualification.
- The packet carries a proof-freshness block so stale proof automatically narrows
  claimed packs.

## Boundary

The packet carries content addresses, classes, and review-safe labels only. Raw
shell fragments, raw filesystem paths, raw endpoint URLs, credential bodies, raw
API keys, and OAuth tokens never cross this boundary; `validate` rejects
export-safe JSON that embeds raw automation or credential material.

## Regenerating the artifacts

The checked-in support export and fixtures are produced by the in-crate builder
and can be regenerated deterministically:

```bash
cargo run -p aureline-ai --example dump_recipe_pack_graduation_packet -- support
cargo run -p aureline-ai --example dump_recipe_pack_graduation_packet -- fixture
```
