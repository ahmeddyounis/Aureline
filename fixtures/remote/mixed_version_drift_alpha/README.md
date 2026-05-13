# Helper Capability and Mixed-Version Drift Fixtures

This fixture lane protects the helper/agent capability negotiation alpha
contract consumed by the drift harness. The cases are data-only and can be
validated without a live remote agent, helper binary, tunnel broker, or managed
workspace runtime.

The cases cover:

- a supported adjacent-window capability exchange;
- a mixed-version attach that narrows to review-only;
- an untested same-major pairing that must stay `retry_required` until a probe
  runs;
- an unsupported required-feature skew that must stay `unsupported_skew`.

Run:

```sh
python3 ci/check_helper_capabilities_alpha.py --repo-root . --render-negotiation-projection
```

To prove the failure drill catches label drift:

```sh
python3 ci/check_helper_capabilities_alpha.py --repo-root . \
  --force-drill helper_capability_case:remote_agent.unsupported_required_feature:rewrite_unsupported_skew_to_supported
```
