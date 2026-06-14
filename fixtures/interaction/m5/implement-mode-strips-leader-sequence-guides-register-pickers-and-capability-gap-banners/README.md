# M5 mode-strip surface fixtures

## notebook_downgrades_on_stale_verification.json

An auto-downgrade drill fixture for the mode-strip surface packet. Every claimed
M5 surface — editor, notebook, data/API, preview, docs, and a provider-linked
companion — binds the frozen keyboard-mode taxonomy to its live posture: an
imported keymap source preset (Vim, Neovim, Emacs, Helix, or a non-modal
default), a visible current mode, the frozen mode-strip class it realizes, a
pending-sequence state (operator and sequence tokens, count prefix, timeout
posture, resolution), a register/clipboard picker, an accessibility block, and
any capability-gap banner. Each strip carries a reopenable verification proof
keyed by a non-display fingerprint distinct from its id.

The notebook strip claims `parity_complete`, but its verification proof has aged
outside its freshness window (`proof_currency` is `stale_expired`). Because a
claimed surface may not outrun current proof, the strip auto-downgrades to an
effective grade of `parity_unverified`, records a `stale_verification_proof`
downgrade trigger, and carries a precise downgraded label rather than a generic
provider error. A verification-freshness downgrade does not require an affordance
capability-gap banner: every interaction axis the notebook advertises is still
identified, so nothing was narrowed or rejected.

The data/API strip that imported a leader-key workflow it cannot host keeps its
honest downgrade: its sequence axis is `unsupported_downgraded`, its effective
grade ranks below its claim, and a `modal_sequence_unsupported` capability-gap
banner explains the rejection — reachable by keyboard and announced to assistive
technology, never hover-only. The docs strip demonstrates a live pending
sequence: an `operator:change` operator with a count prefix of `2` is in flight
while the surface awaits the completing motion.

The companion strip is provider-linked: its `provider_linked_surface` origin
pairs with an `imported_current` proof currency, which backs the provider strip's
posture but never a local one — a provider-backed surface never reads as a
locally verified rerun. It also carries a non-downgrading `named_register_unsupported`
banner that honestly explains that copy narrows to a single clipboard with a
plain-text fallback.

The fixture validates against
`schemas/interaction/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners.schema.json`
and shares the seeded surface set with the checked support export at
`artifacts/interaction/m5/implement-mode-strips-leader-sequence-guides-register-pickers-and-capability-gap-banners/support_export.json`.
