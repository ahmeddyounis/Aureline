# Provider Route Disclosure Fixtures

## managed_held_pending_graduation.json

A disclosure fixture where the managed lane is held pending upstream provider
graduation. The local and BYOK routes stay claimed (Stable and Beta) with fully
disclosed region, retention, and cost. The two managed routes are held: because
a held route is not a claimed lane, it carries no evidence refs and may leave
its region, retention, and cost `unknown_unverified` rather than fabricating a
disclosure it cannot back. Every route — claimed or held — still declares a mode
that matches its locality and narrows to a strictly lower qualification on stale
proof. This demonstrates that an un-graduated managed lane is visibly held
instead of shown with optimistic placeholder disclosure.
