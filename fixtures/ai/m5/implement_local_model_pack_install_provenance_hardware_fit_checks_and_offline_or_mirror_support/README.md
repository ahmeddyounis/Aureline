# Local Model Pack Fixtures

## offline_mirror_with_held_quarantine.json

A pack catalogue sourced entirely through offline and mirror channels. The
offline-bundle small pack and the configured-mirror mid pack stay claimed
(Stable and Beta) with verified, signed provenance — offline and mirror delivery
does not waive the signing bar. The air-gapped sideload pack failed its
signature, so it is held (not a claimed lane), carries no evidence refs, may
leave its hardware fit `unknown_unverified`, and narrows to `unavailable` on
stale proof rather than fabricating a disclosure it cannot back. This
demonstrates that an offline or mirror install path still proves provenance, and
that a tampered sideload is visibly quarantined instead of shown as installed.
