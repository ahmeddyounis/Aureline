# Install Profile Beta Contract

This contract makes install topology inspectable for users, administrators,
support, and release reviewers. It binds install-profile cards, side-by-side
import sheets, rollout rows, repair/verify diagnostics, and uninstall summaries
to the same checked-in packet shape.

## Source Packets

- Profile cards, import sheets, and rollout rows:
  [`fixtures/install/m3/profile_cards_and_repair/profile_cards_packet.json`](../../../fixtures/install/m3/profile_cards_and_repair/profile_cards_packet.json)
- Repair, verify, failure, and uninstall diagnostics:
  [`fixtures/install/m3/profile_cards_and_repair/repair_verify_uninstall_packet.json`](../../../fixtures/install/m3/profile_cards_and_repair/repair_verify_uninstall_packet.json)
- Exact-build install diagnostics:
  [`artifacts/release/m3/install_diagnostics/install_diagnostics_packet.json`](../../../artifacts/release/m3/install_diagnostics/install_diagnostics_packet.json)
- Ring rollout evidence:
  [`artifacts/release/m3/ring_rollout/packet.json`](../../../artifacts/release/m3/ring_rollout/packet.json)

## Required User/Admin Truth

Every claimed beta profile card must expose:

- platform and architecture;
- install mode and channel;
- updater owner;
- binary root class and durable state-root refs;
- side-by-side or portable relation;
- rollback target and uninstall/deprovision path;
- diagnostics or support-export action;
- file-association and protocol-handler ownership where relevant.

Portable rows must keep durable state inside the declared portable root and
must suppress machine-global integrations such as services, shell hooks,
credential-store registration, file associations, and protocol handlers.

Side-by-side rows must use compare-before-apply import sheets. Import sheets
must include import, keep-separate or skip choices, a checkpoint created before
apply, and disclosures for state roots, handlers, and hidden shared-state
assumptions.

## Repair, Verify, And Uninstall

Enterprise and silent deployment operations must emit structured diagnostics
with:

- copyable install id;
- started and finished timestamps;
- install-profile card ref and diagnostics row ref;
- state-root refs;
- human-readable summary;
- failure summary, reason, and remediation when the operation fails;
- preserved user-state refs and removed install-state refs for uninstall.

The uninstall contract preserves user configuration, recovery roots, and
user-authored workspace files outside the install tree while removing declared
package markers and update state.

## Rollout Rings

Fleet-facing rows use the controlled deployment rings:

- Canary
- Pilot
- Broad
- LTS

Each ring row must name an owner, current promotion state, required evidence,
preserved evidence, rollback target, rollback stop conditions, and install
profile card refs. A ring row without bounded rollback posture is not
claimable.

## Verification

Run:

```bash
cargo test -p aureline-install --test profile_cards_and_repair_beta
```

The test validates the profile packet, repair packet, support projections,
portable integration suppression, side-by-side import choices, rollout ring
coverage, uninstall preservation, and failure-summary rejection behavior.
