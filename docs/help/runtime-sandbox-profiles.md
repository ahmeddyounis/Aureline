# Runtime Sandbox Profile Help

A runtime action shows a sandbox profile ID, backend class, and approval lineage
when it can execute. If the platform cannot enforce the requested profile,
Aureline shows a stricter downgrade, unsupported state, or fail-closed denial.

Browser companion surfaces cannot launch local shells, kernels, or device
execution. They can request approval or route to an approved remote/managed
backend, but they do not grant themselves runtime authority.

Approval history and expiry banners show who authorized the action, the granted
scope, the target, the sandbox profile, when the ticket expires, and which drift
or policy change requires reapproval. Support bundles include the same metadata
without raw secrets or raw command bodies.
