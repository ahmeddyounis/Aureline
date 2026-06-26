# Browser / Provider-Console Handoff-Objects Fixtures

Each fixture is a case file with a `record_kind` of
`browser_provider_console_handoff_objects_case`, a `scenario` describing what the
case proves, a packet `input`, and an `expect` block naming the derived promotion
state and the validation finding kinds the validator must raise. The integration
test materializes each `input` and asserts the promotion state and expected
findings, so these fixtures pin the guardrails the canonical support export keeps
green.

Regenerate any case with the headless emitter, e.g.:

```sh
cargo run -q -p aureline-docs --bin aureline_docs_browser_provider_console_handoff_objects -- fixture baseline_stable
```

## baseline_stable.json

The baseline packet certifies `stable`: every docs/help/AI/provider-console exit
routes through one handoff object naming its destination class, the reason
in-product viewing was insufficient, the privacy consequence, the trust/policy
posture, and a return anchor. No handoff leaks raw code, README, ADR, or prompt
context, and the support-export and docs-history surfaces reconstruct every
handoff.

## hidden_context_share_blocks_stable.json

A docs-browser handoff would carry a raw code selection across the boundary. The
validator raises `hidden_context_share_detected` and blocks stable because a
handoff never silently exfiltrates raw code, README, ADR, or prompt context.

## ordinary_navigation_shares_context_blocks_stable.json

A docs-browser handoff that is part of ordinary navigation shares the user's
query terms. The validator raises `ordinary_navigation_shares_context` and blocks
stable because ordinary docs navigation must not share workspace or query
context.

## raw_browser_open_bypass_blocks_stable.json

A provider-console pivot opens without going through explicit handoff review. The
validator raises `raw_browser_open_bypass` and blocks stable because raw browser
opens, provider-console pivots, and docs fallbacks may not bypass handoff review.

## return_anchor_missing_blocks_stable.json

A handoff drops its return anchor. The validator raises `return_anchor_missing`
and blocks stable because every handoff must keep a return anchor so the reader
can get back to the governed surface.

## privacy_consequence_inconsistent_blocks_stable.json

A handoff declares `no_context_shared` while its shared-context object still
carries the resolved destination ref. The validator raises
`privacy_consequence_inconsistent` and blocks stable because the declared privacy
consequence must match what actually crosses the boundary.

## exit_coverage_missing_blocks_stable.json

The help/about exit loses its handoff object. The validator raises
`exit_coverage_missing` and blocks stable because every
docs/help/AI/provider-console exit must route through a handoff object.

## history_drops_handoff_blocks_stable.json

The reopened docs-history projection stops reconstructing one handoff. The
validator raises `history_reconstruction_drops_handoff` and blocks stable because
support-export and history surfaces must reconstruct every handoff rather than
flattening one into ordinary navigation.

## blocked_handoff_presented_available_blocks_stable.json

A provider-console pivot is blocked by policy yet still offered as an actionable
open. The validator raises `blocked_handoff_presented_available` and blocks stable
because a policy-blocked or unavailable destination may not be presented as
available.

## blocked_handoff_narrows_below_stable.json

A provider-console pivot is blocked by policy and honestly disclosed as blocked,
not offered as actionable. The validator raises `handoff_unavailable_narrowed`
and narrows below stable rather than blocking, because the handoff stays valid and
attributable but cannot claim an available action.

## shared_context_blocked_narrows_below_stable.json

An AI-answer handoff honestly blocks a context share that would have exceeded its
qualified scope. The validator raises `shared_context_blocked_narrowed` and
narrows below stable rather than blocking, because the share was prevented and
disclosed.
