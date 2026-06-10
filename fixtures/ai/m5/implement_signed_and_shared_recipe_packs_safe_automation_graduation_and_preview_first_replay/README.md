# Signed And Shared Recipe Pack Fixtures

## blocked_pack_narrows.json

A recipe-pack catalogue captured after a pack signature failed verification.

The organization review-comment template stays at Stable: it is admitted, signed by
author and organization, distributed on the managed channel under a managed-only
template authority, its reversible external comment previews in full before replay
and is per-invocation approved and audited to the run-record timeline, and its
checkpoint-reversible rollback is verified.

The quarantined deploy template demonstrates narrowing: its managed-channel
signature could not be verified, so its state is `quarantined_signature` and it
claims `held` rather than any public lane. Its irreversible external publish shows a
diff before replay, is denied by policy while quarantined, and is audited to the
run-record timeline; every downgrade rule narrows to `unavailable`, so a provider
outage keeps it out of every claimed lane.

This demonstrates that a blocked pack drops its public claim instead of hiding
behind a Stable, Beta, or Preview label, that a mutating step still previews,
gates, and audits even while blocked, and that the downgrade rules narrow rather
than hide the pack.
