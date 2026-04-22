# Reviewed-pack and late-copy change-control policy

This document freezes the reviewed-pack model and the controlled
late-copy workflow Aureline uses for release-bearing trust, legal,
policy, recovery, support, and compatibility copy. The machine-readable
boundary is
[`/schemas/docs/late_copy_change_packet.schema.json`](../../schemas/docs/late_copy_change_packet.schema.json);
worked examples live in
[`/fixtures/docs/late_copy_examples/`](../../fixtures/docs/late_copy_examples/).

Related contracts:

- [`/docs/docs/docs_pack_manifest_contract.md`](./docs_pack_manifest_contract.md)
  — docs-pack manifest family that already owns the docs/help source,
  version, freshness, and publishable-state vocabulary.
- [`/docs/docs/help_about_service_health_routes.md`](./help_about_service_health_routes.md)
  — destination-descriptor contract for Help, About, service-health,
  migration, and support/export routes.
- [`/docs/governance/claim_manifest_contract.md`](../governance/claim_manifest_contract.md)
  — claim-row publication contract for canonical public copy and
  downgrade routing.
- [`/schemas/release/compatibility_row.schema.json`](../../schemas/release/compatibility_row.schema.json)
  — compatibility-row contract migration/support/release copy already
  cites by stable row id.
- [`/docs/release/release_artifact_graph.md`](../release/release_artifact_graph.md)
  — release-artifact graph that treats docs/help, migration, support,
  and public-proof copy as release-bearing truth.
- [`/docs/governance/change_budget_workflow.md`](../governance/change_budget_workflow.md)
  — freeze-era exception path this policy layers on top of instead of
  replacing.
- [`/docs/governance/evidence_freshness_policy.md`](../governance/evidence_freshness_policy.md)
  — freshness rules that still govern the evidence and owner rows this
  policy references.

## Why freeze this now

The docs-pack manifest contract already freezes the pack-level source,
version, freshness, locale, and publishable-state vocabulary. The
claim-manifest contract already freezes canonical public copy,
downgrade posture, and channel bindings. The compatibility-row contract
already freezes the migration/support/release wording that hangs off a
named boundary row. What remained implicit was the release-bearing copy
discipline that sits between them:

- which combination of those source rows counts as the reviewed source
  for a given train or build;
- how Help, About, service-health, migration, support-export, release
  notes, and public-proof surfaces bind to that reviewed source without
  free-form restatement; and
- what happens when a trust, legal, policy, recovery, support, or
  compatibility sentence must change after string freeze.

Without that discipline, late copy changes become silent edits on
release-bearing surfaces: Help or service-health text can drift from the
claim row, recovery copy can drift from the compatibility row, and
support or migration wording can carry a newer disclosure than the
reviewed docs pack with no explicit artifact explaining why.

This policy closes that gap. It gives downstream work one reviewed-pack
version model and one late-copy packet family rather than a mix of docs
PRs, release-note edits, support macros, and shiproom folklore.

## Scope

Frozen at this revision:

- one `reviewed_pack_version_record` shape that binds a running
  build to the reviewed source rows release-bearing surfaces are
  allowed to quote;
- one `late_copy_change_packet` shape for any post-string-freeze delta
  that temporarily overrides or narrows that reviewed source;
- one binding-state label set that distinguishes reviewed-current,
  stale-reviewed, active-override, reversed-override, and blocked
  unreviewed copy; and
- one reviewer / verification / rollback rule set shared across docs,
  migration, support, release notes, CLI/help, evaluation, and
  public-proof lanes.

Out of scope until a superseding decision row opens:

- localization workflow automation, translation-memory tooling, or CMS
  integration;
- auto-generated packet emission from release tooling;
- editing UI for late-copy packets; and
- replacing the existing docs-pack, claim-row, compatibility-row, or
  destination-descriptor contracts.

## What counts as reviewed source

Release-bearing copy counts as reviewed source only when it resolves to
at least one versioned artifact family that already owns the truth:

- a `docs_pack_manifest_record` revision or support-runbook pack
  revision that resolves a
  `help_status_badge_record.source_revision_ref`;
- a `claim_row` canonical-copy field, known-limit note, or exclusion
  note cited through the claim-manifest contract;
- a `compatibility_row` field such as support-window language,
  migration guidance, skew-window summary, or known deviation text;
- a `destination_descriptor_record` field that owns a route, boundary,
  handoff, or availability disclosure; or
- a versioned migration/support packet family that is itself already
  tied back to one of the rows above by stable ref.

The following do **not** count as reviewed source for release-bearing
surfaces:

- ad hoc UI strings that are not bound to a reviewed packet or row;
- release-note drafts, field macros, or incident scratch text with no
  stable ref;
- screenshots or marketing captions;
- localized or forked prose that lost the source-language ref,
  claim-row ref, compatibility-row ref, or docs-pack revision; and
- one-off shiproom edits applied directly to a surface without a new
  pack version or a late-copy packet.

## Reviewed-pack model

A `reviewed_pack_version_record` is the immutable tuple a release-bearing
surface binds to when it quotes governed copy. It does not replace the
upstream source rows; it pins which reviewed rows are in force together.

Every reviewed-pack version carries:

- one `pack_version_id` and one `running_build_identity_ref`;
- one or more reviewed source bindings, each naming the source artifact
  kind, stable ref, optional revision ref, and the surfaces that bind to
  it;
- the binding state for each source (`reviewed_current`,
  `stale_reviewed_source`, `late_copy_override_active`,
  `late_copy_override_reversed`, `blocked_unreviewed`);
- linked `claim_row_id` and `compat_row` refs where the source row
  narrows or widens public/support truth; and
- the reviewers who approved this reviewed-pack version as the current
  release-bearing source set.

Rules:

1. A release-bearing surface MUST bind to a reviewed-pack version or to
   an active late-copy packet that explicitly names the reviewed-pack
   version it overrides.
2. Updating any bound source revision, claim row, compatibility row, or
   destination descriptor creates a new reviewed-pack version. Silent
   in-place mutation is forbidden.
3. A surface may render many rows from one reviewed-pack version, but
   those rows MUST remain traceable to the same `pack_version_id`.
4. Derived or live rows may still render their own source/freshness
   axes, but if they are quoted on release-bearing help/support/public
   surfaces they MUST resolve back to the reviewed pack or to the active
   late-copy packet that narrowed it.

## Surface binding rules

The reviewed-pack model applies when any of these surfaces carry trust,
legal, policy, recovery, support, or compatibility copy:

- docs pane
- docs browser
- Help / About
- service health
- migration notes
- support export
- release notes
- CLI/help
- evaluation artifacts
- public-proof packets

Existing source, version-match, freshness, client-scope, and
service-contract axes remain separately addressable. This policy does
not replace ADR-0013 or the claim-manifest downgrade rules; it adds the
copy-binding layer that says whether a surface is reading the reviewed
source directly, reading a stale reviewed source, or reading an active
override packet.

## Binding-state labels

| Binding state | Meaning | Required surface behavior |
|---|---|---|
| `reviewed_current` | Surface text still matches the reviewed-pack source binding. | No late-copy disclosure is needed. |
| `stale_reviewed_source` | The reviewed source row is still the last approved source, but a newer row, refresh, or superseding pack exists and has not been reviewed onto this train. | Surface must label the copy as stale and may not present it as current truth. |
| `late_copy_override_active` | Surface is temporarily quoting a late-copy packet instead of the reviewed-pack source. | Packet id and override note must remain inspectable on the primary review/export surface. |
| `late_copy_override_reversed` | A prior override existed, but the surface has been rebound to reviewed source or to a superseding packet. | Reversal remains inspectable for audit/support, but the surface no longer claims the override as active. |
| `blocked_unreviewed` | No reviewed source or approved late-copy override exists for the required sentence. | Surface must narrow, hide, or block the claim instead of inventing copy. |

## Normal copy evolution versus late copy

Normal copy evolution is the ordinary path:

- the surface is not release-bearing; or
- the change lands before string freeze; or
- the change is carried by a normal docs-pack, claim-row, compatibility,
  or route update that becomes the new reviewed-pack version before the
  affected train freezes.

Controlled late copy is required when all of the following are true:

1. the change lands after string freeze for the affected train or
   release-bearing packet;
2. the text is trust, legal, policy, recovery, support, or
   compatibility copy; and
3. the text appears on a release-bearing surface listed above.

Late copy may correct, narrow, or restate a disclosure. It may **not**
silently widen a claim beyond the current `effective_claim_posture`, the
current compatibility row, or the current reviewed-pack source binding.
If the change widens truth, the owning claim row or compatibility row
must widen in the same change and the late-copy packet must say so.

## Reason classes and reviewer requirements

Each late-copy packet names one primary reason class:

| Reason class | Minimum required reviewers |
|---|---|
| `legal_obligation_correction` | docs/public-truth owner, legal/policy owner, release owner |
| `policy_requirement_correction` | docs/public-truth owner, legal/policy owner, release owner |
| `trust_boundary_disclosure_correction` | docs/public-truth owner, security/trust owner, release owner |
| `service_health_state_correction` | docs/public-truth owner, security/trust owner, release owner |
| `recovery_instruction_correction` | docs/public-truth owner, recovery owner, support owner, release owner |
| `support_path_correction` | docs/public-truth owner, support owner, release owner |
| `compatibility_scope_correction` | docs/public-truth owner, compatibility owner, release owner |
| `claim_evidence_alignment` | docs/public-truth owner, release owner, and the owner of the linked claim or evidence row |

The packet may name extra reviewers such as a domain owner or incident
commander, but it may not omit the minimum reviewer set for the reason
class it declares.

## Packet requirements

Every `late_copy_change_packet` carries:

- the `reviewed_pack_version_ref` it overrides or narrows;
- the timing class (`after_string_freeze_before_release`,
  `post_release_hotfix`, or `out_of_band_service_notice`);
- the owner and required-reviewer set;
- the affected surfaces;
- a `prior_text_ref` and `new_text_ref` for every affected surface;
- verification notes explaining what was re-checked;
- linked claim-row refs and linked compatibility-row refs; and
- rollback or reversal notes.

When the new text lives only in the packet until a new reviewed-pack
version is cut, the affected surface binds to
`late_copy_override_active`. Once the reviewed source is updated and the
surface rebinds, the packet moves to `superseded` or `reversed` and the
binding label changes to `late_copy_override_reversed` or
`reviewed_current`.

## Rollback and reversal

Every late-copy packet must explain how the temporary divergence ends:

- restore the prior reviewed text;
- supersede the override with a new reviewed-pack version;
- withdraw the affected claim and route to a known-limit or support
  path; or
- rebind the surface to another already-reviewed source row.

If an override remains active long enough that it becomes normal truth,
the reviewed-pack source must be updated and a new reviewed-pack version
must replace it. Permanent "temporary" overrides are non-conforming.

Any change that removes, supersedes, or reverses late-copy text without
closing the packet or updating the reviewed-pack binding is likewise
non-conforming.
