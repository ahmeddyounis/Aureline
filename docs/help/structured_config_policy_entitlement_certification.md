# Structured config certification help truth

This help page is the downstream consumer-facing explanation for the structured
config, policy, and entitlement certification packet at:

- [`/artifacts/config/structured_config_policy_entitlement_certification.json`](../../artifacts/config/structured_config_policy_entitlement_certification.json)

Help/About, support export, release-center, and shiproom surfaces should reuse
that packet's state, evidence age, local-safe floor, supported profile rows,
and narrowing reasons instead of paraphrasing them.

## Terms

- `certified` — the row currently holds its full claim on the named artifact
  family or deployment profile.
- `limited` — the row is still claimable, but only with explicitly narrower
  language.
- `offline_only` — the row is currently limited to mirrored, cached, or
  offline-transferred continuity.
- `retest_pending` — the row is not currently claimable until the named drill
  is rerun or the blocked trust condition is repaired.

## Local-safe continuity

When policy freshness, managed auth, or signer continuity degrades, the packet
preserves the same local-safe floor across Help/About and support surfaces:

- local editing, inspect, and export remain available where the profile claims
  them;
- managed widening pauses with explicit reasons; and
- stale or last-known-good state never presents as fresh or live.

## Required downstream disclosures

Any help-facing consumer of the packet must preserve:

- the current certification state,
- exact evidence age,
- the local-safe floor,
- supported deployment profiles, and
- narrowing reasons such as stale policy, reauth required, signer rotation, or
  mirror/offline fallback.
