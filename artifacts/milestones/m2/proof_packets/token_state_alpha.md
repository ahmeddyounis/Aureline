# Proof Packet: Token/State Alpha Registry

```yaml
as_of: 2026-05-15
freshness_date: 2026-05-15
captured_at: 2026-05-15T17:24:31Z
stale_after: P14D
source_revision: git:7ef49d38b543d94113d56e1b3aa289eea9e62c2e
trigger_revision: alpha_token_state_registry_contract_set@2026-05-15
validator: ci/check_m2_state_semantics.py
validation_capture: artifacts/milestones/m2/captures/token_state_alpha_validation_capture.json
claim_change_state: no_claim_widening
```

## Scope

This packet covers the alpha token/state registry, component-state vocabulary,
badge/notice families, protected fixtures, and the first `aureline-ui`
consumer.

## Canonical Artifacts

- Registry: `artifacts/design/state_badge_families_alpha.yaml`
- Fixtures: `fixtures/design/m2_state_semantics/`
- Runtime consumer: `crates/aureline-ui/src/tokens/state_semantics.rs`
- Component-state consumer: `crates/aureline-ui/src/components/state_registry.rs`
- Reviewer page: `docs/design/m2_token_state_alpha.md`
- Validator: `ci/check_m2_state_semantics.py`

## Acceptance Evidence

The validator checks that:

- loading and pending are distinct;
- degraded, blocked, trust-restricted, and policy-locked states carry
  non-color cues;
- lifecycle, route, support-class, readiness, policy, trust, docs/help,
  package/marketplace, support-export, and theme-package badges are all
  registered;
- notice families cover info, warning, degraded, blocked, restricted, and
  success postures;
- shell, editor, search, docs/help, package/marketplace, trust, support-export,
  and embedded-extension surfaces consume the same vocabulary;
- embedded UI-bearing surfaces either inherit token families or declare an
  inheritance gap.

## Verification

```sh
python3 ci/check_m2_state_semantics.py --repo-root .
cargo test -p aureline-ui
git diff --check
```

## Residual Risk

The current proof is registry and API level. Later visual-diff runners still
need to capture rendered screenshots once the concrete shell renderer surfaces
exist for every listed consumer.
