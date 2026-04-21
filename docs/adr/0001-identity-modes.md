# ADR 0001 — Identity modes and workspace-trust posture

- **Decision id:** D-0009 (see `artifacts/governance/decision_index.yaml#D-0009`)
- **Status:** Accepted
- **Decision date:** 2026-04-19
- **Freeze deadline:** 2026-08-01
- **Owner:** `@ahmeddyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** security_trust_review
- **Related requirement ids:** none

## Context

The product must serve hobbyists, OSS contributors, startups, and regulated
enterprises from a single binary without forcing any of them onto the
wrong identity model. The source documents explicitly require three
separable identity modes and a workspace-trust posture layered
underneath them; without a frozen shape, the local-core lane risks
acquiring a hidden sign-in wall as managed services accrete, and the
boundary-manifest rows that cite this decision stay ambiguous.

The freeze matters because the desktop core is the product's floor
commitment: local editing, save, undo, search, local Git, tasks, and
local / BYOK AI must keep working with no account, no hosted control
plane, and no network. Anything that conflates "identity" with "able to
edit a file" is a regression of that floor. Equally, enterprise
deployments must be able to layer standards-based identity and policy
on top without reimplementing the desktop.

This ADR closes `D-0009` (identity modes and workspace-trust posture)
and ratifies the boundary-manifest rows that cite it.

## Decision

Aureline freezes **three identity modes** and one **workspace-trust
posture** that applies inside every mode.

### Identity modes

1. **`account_free_local`** — the default. No account, no hosted
   dependency, no embedded-vendor auth flow. The desktop runs with
   zero identity state. Editing, save, undo, search, workspace VFS,
   local Git, tasks (subject to trust), local model execution, and
   BYOK AI providers all reach a usable state without any sign-in.
   A local-only user profile MAY be created for personalisation; it
   is a local file and never contacts a remote service.
2. **`self_hosted_org`** — optional, standards-based. The deployment
   may integrate with a customer- or vendor-operated identity
   provider over OpenID Connect (system browser, not embedded
   webview) and SCIM for provisioning. Policy is distributed as
   signed bundles with local caches and inspectable diffs. The
   reference implementation MUST be operable by the customer; a
   vendor-managed form MAY exist alongside but is not a prerequisite
   for any self-hosted deployment.
3. **`managed_convenience`** — optional, hosted-only. Vendor-hosted
   services (managed settings sync, hosted marketplace UI, managed
   AI quota and audit UI, fleet admin UI / SCIM dashboards) layer
   over the `self_hosted_org` protocols. The managed form adds
   convenience; it never gates the protocols below it. Disabling or
   losing a managed service narrows the affected claim without
   reclassifying the underlying capability.

Modes are **not exclusive deployment postures**: a single install MAY
speak to an organisation's IdP while still being used offline by the
same operator. The mode determines which identity surface is reachable
at a given moment, not what the user may install.

### Workspace-trust posture

A binary **trusted / restricted** state applies inside every identity
mode and is authoritative for code-executing actions:

- **Trusted workspace** — repo-defined tasks, debuggers, notebook
  kernels, extension activation hooks, and environment managers may
  run. Identity is not required to grant trust; the anonymous local
  user may grant trust for a workspace.
- **Restricted workspace** — open, read, search, and navigate are
  permitted; editing is permitted; saving to disk is permitted.
  Repo-defined command execution, auto-activated extensions, and
  injected launchers are gated until trust is granted. Restricted
  mode is the default for a workspace the current user has not
  previously trusted.

Workspace trust is a local decision bound to the workspace path and
the current user profile. `self_hosted_org` and `managed_convenience`
policy bundles MAY narrow what "trusted" allows; they MAY NOT widen
it, and they MAY NOT remove restricted mode's read / search / edit /
save guarantees.

### What works offline, what may require sign-in, what must never gate local editing

| Capability                                  | Offline | May require sign-in | Must never gate local editing |
|---------------------------------------------|:-------:|:-------------------:|:-----------------------------:|
| Editor, buffer, save, undo                  |   yes   |         no          |             yes               |
| Workspace VFS, local Git (local ops)        |   yes   |         no          |             yes               |
| Search over the local workspace             |   yes   |         no          |             yes               |
| Tasks and debugger (in trusted workspace)   |   yes   |         no          |             yes               |
| Local model execution / BYOK AI             |   yes   |         no          |             yes               |
| Extension activation (already installed)    |   yes   |         no          |             yes               |
| New extension discovery / install           | cached  |     mode-dependent  |             n/a               |
| Managed settings sync push                  |   no    |         yes         |             n/a               |
| Managed AI quota / audit UI                 |   no    |         yes         |             n/a               |
| Fleet admin UI, SCIM provisioning           |   no    |         yes         |             n/a               |
| OIDC policy-bundle refresh                  | last-known-good |  yes        |             n/a               |

The final column is the invariant: **no row in the top half of the
table may ever acquire a sign-in prerequisite without a superseding
ADR.** Adding such a prerequisite would supersede this ADR, not edit
it; it would also supersede the boundary-manifest
`mandatory_vendor_hosted_auth` row.

### Degraded-mode and downgrade behaviour

When an identity or control-plane dependency is unavailable:

- **Sign-in unreachable.** Cached session and last-known-good policy
  continue to apply; the local product remains fully usable. Only
  *new* protected operations (those the last-known policy marks as
  requiring a fresh token) pause with a visible, recoverable
  `sign-in required` state. Local editing, save, search, local Git,
  tasks in a trusted workspace, and local / BYOK AI never pause.
- **Policy-bundle refresh fails.** The last signed bundle stays
  active. Expired bundles degrade to the safe defaults documented
  by the bundle itself rather than a hard lockout, unless the
  bundle was explicitly configured to fail closed.
- **Managed control plane unreachable.** `managed_convenience`
  surfaces (dashboards, fleet UI, hosted marketplace browse, managed
  sync) show a narrowed-claim banner. `self_hosted_org` protocols
  remain reachable if the customer-operated control plane is up. If
  neither is up, the install runs as `account_free_local` until a
  reachable control plane returns; nothing about local editing
  changes.
- **Downgrade path.** An install MAY move from `managed_convenience`
  to `self_hosted_org` by re-pointing the IdP and policy sources,
  and from either to `account_free_local` by disabling the IdP and
  policy sources. The desktop does not re-mint identity or destroy
  local state during a downgrade. Upgrades follow the reverse path
  and never rewrite local history.

### Actor classes

The modes and trust posture combine with a closed set of actor
classes. Each class has a distinct evidence and audit shape; none may
be silently elevated.

- **Anonymous local user.** Default in `account_free_local`. No
  remote identity. Workspace-trust grants apply locally.
- **Authenticated user.** OIDC subject in `self_hosted_org` or
  `managed_convenience`. Carries a stable subject id, optional group
  memberships, and a session with an explicit expiry.
- **Admin.** Authenticated user with policy-author or fleet-operator
  scope. Admin-only actions are distinguishable in the audit stream
  and require an active session, not merely a cached one.
- **Service account.** Automation identity (CI, bots, MDM hooks).
  Distinguishable from human identities in evidence and audit
  streams; may hold delegated permissions narrower than any human
  role.

Workspace-trust decisions bind to the acting local user + workspace
pair, not to an OIDC subject. This keeps trust decisions portable
across identity modes and prevents "I signed in, therefore this repo
is trusted" confusion.

### Future escalation points (explicitly out of scope for this ADR)

- **Workspace-trust state machine and UI.** Exact prompts, revocation
  surfaces, and per-folder vs per-workspace scoping land in a
  separate ADR. This ADR freezes the posture's guarantees, not the
  interaction model.
- **OIDC / SCIM contract freeze.** Protocol profile, claim mapping,
  and deprovisioning semantics require their own RFC / ADR before
  any non-spike self-hosted deployment lands.
- **Policy-bundle schema.** Signing, diff, and inspection rules
  require a schema change tracked against the governance schemas
  registry; this ADR commits only to the properties the bundle
  MUST carry (signed, cached, diffable, human-inspectable,
  last-known-good safe).
- **Phishing-resistant sign-in (WebAuthn / passkeys).** Required in
  `managed_convenience` and recommended in `self_hosted_org` where
  the IdP supports it; specific flow freezing is a follow-on ADR.
- **Break-glass / delegated admin roles.** Enterprise escalation
  paths (temporary elevation, audit-reviewer role, tenant export)
  are reserved under `fleet_admin_ui_scim` and will land as their
  own decisions.
- **Credential storage boundaries.** OS keychain / Credential
  Manager / Secret Service integration rules and the degraded path
  when the platform store is unavailable land with the native-OS
  integration ADR.

## Consequences

- **Frozen:** the local-core capabilities listed in the
  "what works offline" table above cannot acquire a sign-in
  prerequisite without a superseding ADR.
- **Frozen:** the boundary-manifest
  `mandatory_vendor_hosted_auth` row stays `out_of_scope`;
  reversing it requires a superseding row and ADR, not an edit.
- **Frozen:** workspace-trust restricted mode's read / search /
  edit / save guarantees are invariant across identity modes and
  policy bundles.
- **Permitted:** `self_hosted_org` and `managed_convenience` layers
  may add OIDC, SCIM, signed policy bundles, and hosted convenience
  UIs on top of the self-hostable reference implementation.
- **Permitted:** downgrading an install from managed to self-hosted
  to account-free is a reversible operator action; no desktop state
  is re-minted or destroyed.
- **Follow-up:** workspace-trust state-machine ADR, OIDC / SCIM
  profile RFC, policy-bundle schema change, and native-OS
  credential-storage ADR all land as their own decisions.
- **Ratifies:** boundary-manifest rows `identity_policy_service`
  (`self_host_friendly`), `fleet_admin_ui_scim`
  (`managed_convenience`), and `mandatory_vendor_hosted_auth`
  (`out_of_scope`) per the manifest's ratification rule.

## Alternatives considered

- **Single trusted mode (the D-0009 default-if-unresolved posture).**
  Narrow to one trusted-workspace mode for the first beta, with
  untrusted workspaces read-only and no managed or self-hosted
  identity surfaces. Rejected: the single-mode default collapses
  hobbyist and enterprise requirements, leaves no articulated home
  for OIDC / SCIM / policy bundles, and offers no narrowed-claim
  behaviour under managed outages. It would still satisfy the
  freeze-by date, but would push every enterprise integration into
  a later emergency ADR and risk implicit drift in the meantime.
- **Vendor-hosted mandatory sign-in.** Require a vendor account for
  the desktop to start. Rejected: directly contradicts the open-core
  floor commitment and would supersede the
  `mandatory_vendor_hosted_auth` out-of-scope row. No user benefit
  outweighs the loss of local-core trust.
- **Flat "account vs no-account" model.** One boolean: signed in or
  not. Rejected: collapses `self_hosted_org` and
  `managed_convenience`, which have materially different operator
  expectations (control-plane ownership, data boundary, offboarding
  story). Encodes the drift this ADR exists to prevent.
- **Defer to a later milestone.** Leave the identity posture
  unfrozen and let the narrowed default apply automatically.
  Rejected: the default is the single-trusted-mode posture above,
  whose downsides are listed. Deferral without a concrete artifact
  is exactly what M0 foundations governance forbids for
  protected-lane-visible behaviour.

The `D-0009` default-if-unresolved narrowing would have frozen the
product into the single-trusted-mode shape. Accepting this ADR
replaces that narrowing with a frozen three-mode posture plus a
preserved restricted / trusted workspace split, so the narrowing
does not apply.

## Source anchors

- `.t2/docs/Aureline_PRD.md:1524` — "three clearly separated modes".
- `.t2/docs/Aureline_PRD.md:1526` — "Account-free local mode — no
  account, no hosted dependency, no hidden sign-in wall".
- `.t2/docs/Aureline_PRD.md:1530` — Local / managed / required-
  behavior matrix for authentication, provisioning, authorization,
  policy distribution, audit identity, and tenant separation.
- `.t2/docs/Aureline_PRD.md:1542` — "expiry or outage of managed
  identity services should degrade only the managed operations that
  require fresh authorization, not the ability to keep working
  locally".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:278` —
  "workspace trust, least privilege, network/egress governance,
  signed artifacts".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:703` —
  "workspace trust, restricted mode, policy evaluation,
  identity/session state".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:712` —
  "local editing must survive control-plane outages".

## Linked artifacts

- Decision register row: `artifacts/governance/decision_index.yaml#D-0009`
- RFC: none.
- Affected lanes: `crates/aureline-vfs`, `crates/aureline-rpc`,
  `artifacts/governance/ownership_matrix.yaml#scorecard_lane_index:release_evidence`,
  `artifacts/governance/ownership_matrix.yaml#decision_forums:security_trust_review`.
- Boundary-manifest rows ratified:
  `docs/product/boundary_manifest_strawman.md#identity_policy_service`,
  `docs/product/boundary_manifest_strawman.md#fleet_admin_ui_scim`,
  `docs/product/boundary_manifest_strawman.md#mandatory_vendor_hosted_auth`.

## Supersession history

First acceptance. No supersession.
