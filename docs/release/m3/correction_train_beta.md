# Beta correction train, hotfix, and backport workflow

This packet makes post-beta correction work inspectable before a real
shiproom needs it. The canonical machine-readable source is
[`artifacts/release/m3/correction_train/packet.json`](../../../artifacts/release/m3/correction_train/packet.json)
with packet id `correction.train.beta.release_control`. The generated
support projection is
[`artifacts/release/m3/correction_train/support_export_projection.json`](../../../artifacts/release/m3/correction_train/support_export_projection.json),
and the headless gate is
[`tools/ci/m3/correction_train/`](../../../tools/ci/m3/correction_train).

The packet shares one standard form across release, support, and docs:
`correction_scope`, `correction_risk`, `correction_evidence`,
`target_channels`, `triage_lane`, `backport_decision`,
`rollback_target`, and `known_issue_update`.

## Source Artifacts

- Correction train packet:
  [`artifacts/release/m3/correction_train/packet.json`](../../../artifacts/release/m3/correction_train/packet.json)
- Support-export projection:
  [`artifacts/release/m3/correction_train/support_export_projection.json`](../../../artifacts/release/m3/correction_train/support_export_projection.json)
- Boundary schema:
  [`schemas/release/correction_train_packet.schema.json`](../../../schemas/release/correction_train_packet.schema.json)
- Correction train template:
  [`artifacts/release/m3/correction_train_template.md`](../../../artifacts/release/m3/correction_train_template.md)
- Hotfix packet template:
  [`artifacts/release/m3/hotfix_packet_template.md`](../../../artifacts/release/m3/hotfix_packet_template.md)
- Backport packet template:
  [`artifacts/release/m3/backport_packet_template.md`](../../../artifacts/release/m3/backport_packet_template.md)
- Fixture manifest:
  [`fixtures/release/correction_train_cases/manifest.yaml`](../../../fixtures/release/correction_train_cases/manifest.yaml)

## Triage Lanes

Every correction row resolves to exactly one `triage_lane`:

| Lane | When it is used | Release rule |
|---|---|---|
| `hotfix` | Security, signing, policy-escape, remote-code-exec, trust-boundary, or unmitigated data-loss risk on a claimed supported surface. | Smallest viable patch, named `rollback_target`, and same-lane `known_issue_update`. |
| `backport` | The source fix can ride a correction train, but a supported stable or long-support line carries the broken contract. | Each affected supported line has a `backport_decision` of `yes`, `no`, or `defer`. |
| `correction_train_only` | Beta-only or preview-only protected-path work where claims are already narrowed and no supported stable line is affected. | Ships on the correction train; support export still records scope, risk, evidence, and target channels. |
| `next_cycle` | Non-protected polish or breadth work with no support, security, data, rollback, compatibility, or support-window impact. | Must not piggyback on hotfix or backport lanes. |

## Hotfix Lane

The seeded hotfix row is
`correction:item.policy_bundle_signature_bypass`. It targets
`channel:beta`, `channel:stable`, and `channel:lts`, and every affected
supported line records `backport_decision = yes`.

The current beta build remains
`build-id:aureline:beta:2.1.0-beta.1:aarch64-apple-darwin:release:b7ee32adb5eb`.
The beta and stable rollback target is
`release_candidate:aureline.2_0_4_stable`; the long-support rollback
target is `release_candidate:aureline.1_8_8_lts`.

Hotfix rows are non-conforming if they omit `correction_scope`,
`correction_risk`, `correction_evidence`, `target_channels`,
`rollback_target`, or `known_issue_update`.

## Backport Matrix

The seeded backport row is
`correction:item.schema_rollback_hook_regression`. It rides
`correction_train:beta.2_1` while assigning an explicit stable-line
`backport_decision = yes` because the rollback contract is claimed on the
stable line.

Backport rows must record:

- release line and support-line class;
- whether the line is affected;
- `backport_decision` as `yes`, `no`, `defer`, or `not_applicable`;
- rationale, owner, due date, target release when applicable, and
  `rollback_target`;
- `known_issue_update`, docs/help update, and support-note refs.

## Train-Only Lane

The seeded train-only row is
`correction:item.search_startup_regression`. It stays
`correction_train_only` because the affected benchmark wording is beta
only, the public claim is narrowed, and no supported stable or
long-support line carries the claim.

## Next-Cycle Lane

The seeded next-cycle row is
`correction:item.release_notes_sort_order`. It stays `next_cycle` because
it is non-protected copy polish with no user-data, security,
compatibility, support-window, or rollback impact.

## Support And Docs Projection

Support export reads
`artifacts/release/m3/correction_train/support_export_projection.json`
and quotes the same `triage_lane`, `backport_decision`,
`correction_scope`, `correction_risk`, `correction_evidence`,
`target_channels`, `rollback_target`, and `known_issue_update` fields as
this release page and Help / About truth surfaces. The support projection
is metadata-only and uses
`metadata_only_no_raw_logs_paths_or_secrets`.

## Verification

```bash
python3 -m tools.ci.m3.correction_train --repo-root . --check
```

The gate validates the packet, schema, support projection, failure-drill
mutations, consuming docs/help surfaces, and validation capture together.
