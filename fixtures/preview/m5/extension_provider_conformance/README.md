# M5 Extension-Provider Conformance Fixtures

## conformance_declare_repair_and_bounded_profiles.json

A declare-before-you-back, stale-or-weaker-provider repair, and bounded-profile
drill fixture for the extension-provider conformance packet. The five rows cover
both provider origins — first-party and contributed — all four provider statuses,
and all four operating profiles through one shared vocabulary.

The packet demonstrates the conformance rules:

- A **live, conformant first-party** embedded-preview provider whose declaration
  (exact mapping, full attach, hot reload) satisfies the claimed row; it is the
  only row that offers a write-capable designer flow.
- A **conformant first-party** external-browser provider whose host is offline, so
  the row degrades to a `mirror_offline` snapshot with an `offline_mirror_only`
  trigger, a precise degraded label, and `use_mirror_offline` repair guidance —
  bounded truth rather than a blank surface.
- A **stale contributed** device-bridge provider whose declaration went stale; the
  prior stronger declaration is preserved, the row is held `inspect_only`, and
  `reverify_declaration` repair guidance is offered.
- A **weaker contributed** lite-bridge provider that would replace a stronger one;
  the current declaration is strictly weaker than the preserved prior (it drops a
  target kind, network depth, and hot reload), the swap is refused, the row is
  `policy_limited`, and `restore_stronger_provider` repair guidance is offered.
- An **unavailable contributed** remote-runtime provider whose last declaration is
  preserved as unresolved state on a `mirror_offline` profile with
  `reinstall_provider` repair guidance.

Every non-conformant or bounded row carries an explicit downgrade trigger, a
precise non-generic degraded label, and repair guidance; the clean live row carries
none. No row offers a write-capable flow except the live, conformant one.

The fixture validates against
`schemas/preview/extension_provider_conformance.schema.json` and is byte-aligned
with the in-crate builder via
`cargo run -p aureline-preview --example dump_m5_extension_provider_conformance`.
