# High-authority entry interstitials (beta)

This document freezes the workspace-facing beta contract for **high-authority
and cross-boundary entry interstitials** — the typed review that stands between
an OS-level or browser-level entry path and its execution. Plain local open
stays fast and low-friction; this contract only governs entry that is *broader*
than a plain local open.

Machine-readable boundary:

- [`/schemas/workspace/entry_interstitial.schema.json`](../../../schemas/workspace/entry_interstitial.schema.json)

Runtime model:

- [`/crates/aureline-shell/src/entry_interstitials/`](../../../crates/aureline-shell/src/entry_interstitials/)

Fixtures:

- [`/fixtures/workspace/m3/high_authority_entry/`](../../../fixtures/workspace/m3/high_authority_entry/)

UX review artifact:

- [`/artifacts/ux/m3/high_authority_entry_review.md`](../../../artifacts/ux/m3/high_authority_entry_review.md)

Related contracts this composes with, rather than re-deriving:

- Deep-link admission: [`/crates/aureline-shell/src/deeplink/mod.rs`](../../../crates/aureline-shell/src/deeplink/mod.rs)
- Notification exact-reopen / placeholder vocabulary: [`/crates/aureline-shell/src/notifications/`](../../../crates/aureline-shell/src/notifications/)
- System-browser auth return paths: [`/crates/aureline-shell/src/system_browser_return_paths/`](../../../crates/aureline-shell/src/system_browser_return_paths/)
- Protocol-handler ownership audit: [`/artifacts/ux/m3/protocol_handler_audit.md`](../../ux/m3/protocol_handler_audit.md)

## What is in scope

Six entry kinds flow through the interstitial gate, and only when the requested
action widens authority or crosses a boundary:

| Kind | Token | Typical origin |
| --- | --- | --- |
| Protocol / deep-link activation | `protocol_deep_link` | System default browser, OS shell, external provider |
| Auth callback return | `auth_callback_return` | Identity-provider system-browser callback |
| Collaboration join | `collaboration_join` | Collaboration service invite |
| Remote target open | `remote_target_open` | Deep link or recent item resolving to SSH / container / cloud |
| Managed resume | `managed_resume` | Managed admin surface / MDM-pushed action |
| Notification reopen | `notification_reopen` | OS notification or system surface |

**Out of scope.** An ordinary quick-open, drag-and-drop, or file-association
open that resolves to an exact, local, already-trusted target crosses no
boundary. It takes the fast path and is admitted as an
`entry_plain_local_open_record` with no prompt. The interstitial never replaces
those flows.

## The decision: plain local open vs. reviewed interstitial

`evaluate_entry_interstitial` computes the set of boundaries an entry crosses.
If the set is empty *and* the kind is not inherently cross-boundary, the entry
is a plain local open and runs directly. Otherwise a reviewed
`entry_interstitial_record` is materialized.

A boundary is recorded when any of the following hold:

- the requested action raises authority beyond a plain read/open
  (`authority_widening`),
- the resolved target identity must be reviewed, or its truth state is not
  `exact_available` (`target_boundary`),
- the target belongs to a different tenant/organization (`tenant_boundary`),
- trust must be reviewed or narrowed (`trust_boundary`),
- the target is remote and binds remote authority (`remote_boundary`),
- org policy must be reviewed (`policy_boundary`).

Auth callback returns, collaboration joins, remote opens, and managed resumes
are inherently cross-boundary and always materialize a record. Deep links and
notification reopens are evaluated against the rules above, so a deep link or
notification that resolves to a plain local object stays fast.

## What the interstitial discloses

Every `entry_interstitial_record` shows, in redaction-safe terms:

- **Origin / source** — `source_class` plus a `source_label` (e.g. "Default
  browser deep link"). An origin outside the shell is disclosed, never hidden.
- **Requested action class** — what the entry wants to do, through an authority
  lens (`resume_session`, `auth_return`, `join_presence`, …).
- **Target identity and scope** — `target_kind`, an opaque `object_identity_ref`,
  the workspace scope, and where relevant the tenant scope and the
  channel/build owner.
- **Trust / policy effect** — a one-line `trust_policy_effect_label` and the
  typed `authority_effect`.
- **Why we are asking** — `confirm_explanation` names the boundaries crossed so
  the user understands the prompt rather than guessing.
- **Safe paths** — `confirm`, `reject`, and `defer` actions, each with the
  exact outcome of choosing it.

## Four honesty invariants

1. **No silent execution.** `silent_execution_forbidden` is always `true`. A
   high-authority entry that arrives from an OS surface or a browser callback
   cannot run before confirm/reject is shown.
2. **Target truth is preserved.** When the target is moved, missing,
   policy-blocked, downgraded, expired, unreachable, or ambiguous, the record
   carries a truthful `target_placeholder` with bounded `fallback_actions`
   (locate, reconnect, re-auth, inspect-only, open-without-restore, retry,
   return to Start Center, export evidence). The shell never opens an empty
   shell in place of the intended object. When the exact action cannot run, the
   confirm action is disabled with a typed reason while the fallbacks stay live.
3. **Never a generic home surface.** `reopens_generic_home` is always `false`.
   Notification and system-surface reopen resolve the exact object or an
   announced placeholder — they never silently land on a generic home screen.
4. **Canonical command, no widening.** The confirm action's `command_id` is
   exactly the `canonical_command_ref` the in-product path uses for the same
   action. `authority_not_widened` is always `true`, so an OS-origin path can
   never grant more authority than the in-product path. Reject and defer bind to
   the canonical no-change and return-to-Start-Center commands.

## Support / export parity

The same packet is projected into support export as an
`entry_interstitial_support_packet_record`
([`support_export.rs`](../../../crates/aureline-shell/src/entry_interstitials/support_export.rs)).
It carries the typed classes, the opaque object-identity ref, the truth state,
the canonical command ref, and the invariant echoes — but no raw URLs, paths,
callback bodies, or credentials. A route/origin incident can therefore be
reconstructed from logs and exports without scraping transient UI text.

## Known limits (beta)

- The module projects the typed record and decision; wiring it into the live
  deep-link, auth-return, notification, and managed-resume dispatchers is staged
  separately. Each dispatcher feeds an `EntryInterstitialRequest` carrying the
  canonical command id for the action it would otherwise run inline.
- Boundary detection is intentionally conservative: any non-exact target or any
  identity-review requirement raises a `target_boundary`, so a borderline entry
  errs toward review rather than toward a silent open.
