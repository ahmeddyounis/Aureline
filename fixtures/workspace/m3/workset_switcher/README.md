# M3 workset switcher beta fixtures

Canonical fixtures for the M3 workset switcher beta lane. Each JSON file
describes a workspace, one or more workset artifacts, and the expected
projections: switcher rows (with portability labels and root taxonomy),
optional activation previews, and the reopen-parity packet across the
default consumer classes.

The fixtures back the integration test
`crates/aureline-workspace/tests/workset_switcher_beta.rs`. Add a new
fixture by appending a `.json` entry whose root document conforms to the
schema described in [`workset_artifact_beta.md`](../../../../docs/workspace/m3/workset_artifact_beta.md).

| Fixture                                  | Scope class             | Portability label             | Notes                                                                 |
| ---------------------------------------- | ----------------------- | ----------------------------- | --------------------------------------------------------------------- |
| `portable_named_workset.json`            | `selected_workset`      | `portable_with_rebinding`     | Two local repo roots, fully indexed; remote reopen exact.             |
| `sparse_slice_widening_preview.json`     | `sparse_slice`          | `local_only`                  | Activation preview widens to a portable parent; remote reopen degraded by rebinding. |
| `policy_limited_admin_hidden.json`       | `policy_limited_view`   | `managed_provider_locked`     | Admin policy hides members; the parity packet omits support_export.   |
| `managed_provider_locked.json`           | `selected_workset`      | `managed_provider_locked`     | Managed cloud root; export is forbidden; headless reopen degraded by managed_provider_unavailable. |
