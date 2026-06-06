# Session Role Admission And Retention Qualification

Source packet: `artifacts/collab/m4/session-role-admission-and-retention-qualification.json`

A collaboration session surface is Stable only when the session envelope answers these questions before acceptance:

- whose workspace the participant is entering
- who invited or approved entry
- which role is requested or admitted
- when the invite, admission, or rejoin path expires
- which client boundary applies
- what retention mode applies
- whether guests are present or blocked
- which export/delete rights apply to local and managed copies
- how authority narrows if relay, policy, guest, client, or hold state changes

Stable rows are observer-first unless a separate authority review grants more. Browser observer join and mobile rejoin are Preview in this packet, so product copy, Help/About, support exports, and release notes must not describe them as stable collaboration.

Mid-session retention broadening, guest admission, role widening, route visibility widening, or support-evidence enablement requires a visible review event. Silent scope drift is not a valid state.

Presenter/follow surfaces must show presenter identity, viewer/co-presenter state, breakaway state, and a return-to-presenter affordance. Follow state is metadata only; it is not edit authority or source history.

Export/delete surfaces must keep local-only copies separate from managed copies. Legal hold blocks content deletion where policy requires it, but participants can still review metadata and export the hold summary.
