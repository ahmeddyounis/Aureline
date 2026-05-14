# Interaction Integrity Alpha

This contract note binds the first cross-surface transfer packet to the product surfaces that already own copy, paste, drag/drop, reopen, recovery, undo, and support-export truth. The canonical machine-readable shape is [`transfer_action.schema.json`](../../schemas/events/transfer_action.schema.json), and the protected packet fixture is [`m2_interaction_integrity_packet.json`](../../artifacts/ux/m2_interaction_integrity_packet.json).

## Contract Surface

Every covered transfer action declares:

- a stable action id, source surface, and action kind;
- the representation class crossing the boundary, such as `plain_text`, `raw_safe`, `rendered`, `with_context`, or `metadata_only`;
- the target boundary and safe target label;
- the recovery class, named undo group, or checkpoint when durable state can change;
- any high-friction review required before bytes leave the product or mutate a workspace.

The support export projection includes action ids and schema refs only. Raw clipboard bodies, raw terminal paste bodies, raw file bodies, private paths, raw URLs, and credentials stay out of the packet by default.

## Protected Lanes

| Lane | Required truth | Fixture coverage |
|---|---|---|
| Default editor copy | Plain text remains the default transfer. Richer variants are explicit actions. | `transfer:copy:editor-default` |
| Sensitive copy | Label-first preview is required before clipboard write for risky classes such as support-bundle links or private paths. | `transfer:copy:support-link-sensitive` |
| Durable editor paste | Paste into a buffer creates a named undo group and mutation-journal ref. | `transfer:paste:editor-buffer` |
| Terminal paste | Remote, multiline, or production-labeled paste shows host label, clipboard route, line count, policy/trust result, cancel, continue, and no auto-submit posture. | `transfer:paste:terminal-prod` |
| Project-entry drop | The drop advertises the visible verb and destination scope before commit; broad import creates checkpoint truth and durable progress with cancel. | `transfer:drop:project-entry-import` |
| Diff reopen | Intentional close reopens with compare target, selection, and scroll identity. | `transfer:reopen:diff` |
| Terminal recovery | Disconnect recovery restores transcript evidence only and forbids command rerun. | `transfer:recover:terminal` |

## Validation Rules

The shell validator rejects packets that omit the default plain-text copy path, skip label-first sensitive-copy review, allow terminal paste bypass, expose drag/drop without verb truth, omit cancel for durable large transfer progress, collapse intentional close and crash/disconnect recovery, miss the two required named group scopes, or include raw payload bodies in support export.

The checked-in fixtures cover the minimum external-alpha slice:

- [`interaction_integrity_alpha/manifest.json`](../../fixtures/ux/interaction_integrity_alpha/manifest.json)
- [`high_risk_remote_multiline_review.json`](../../fixtures/terminal/paste_boundary_alpha/high_risk_remote_multiline_review.json)
