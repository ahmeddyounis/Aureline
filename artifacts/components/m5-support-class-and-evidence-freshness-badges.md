# M5 Support-Class and Evidence-Freshness Badge Primitive

- Packet: `m5-support-class-and-evidence-freshness-badge-primitive:stable:0001`
- Label: `M5 support-class and evidence-freshness badge primitive: certified/supported/limited/community/experimental support class and fresh/retest-pending/evidence-stale/imported-evidence freshness as two distinct, composable cues`
- Badge consumers: 6 (6 stable)
- Support-class values: certified, supported, limited, community, experimental
- Freshness values: fresh, retest_pending, evidence_stale, imported_evidence
- Effective-claim postures: claim_current, claim_retest_pending, claim_narrowed_evidence_stale, claim_narrowed_imported_evidence
- Proof freshness SLO: 720 hours (last refresh: 2026-07-08T00:00:00Z)

## Badge consumers

- **Onboarding Checklist**: `stable`
  - Owner: Onboarding badge owner
  - Scope: The onboarding checklist renders the shared support-class and evidence-freshness badges as two distinct cues so a certified capability with fresh evidence reads as a current claim, while a supported capability whose evidence was imported and not re-verified narrows the claim with a note that preserves the Supported context and offers a reverify next action
  - Worked resolutions: 2
    - support `certified` + freshness `fresh` → `claim_current` (note `current`)
    - support `supported` + freshness `imported_evidence` → `claim_narrowed_imported_evidence` (note `imported_evidence`)
- **Help Capability Card**: `stable`
  - Owner: Help badge owner
  - Scope: The Help capability card renders the shared badges so a supported capability with fresh evidence reads as a current claim, while a limited-scope capability whose retest is pending reads as retest-pending with an await-retest note rather than as stale or as an implied lower support class
  - Worked resolutions: 2
    - support `supported` + freshness `fresh` → `claim_current` (note `current`)
    - support `limited` + freshness `retest_pending` → `claim_retest_pending` (note `retest_pending`)
- **Marketplace Listing**: `stable`
  - Owner: Marketplace badge owner
  - Scope: The marketplace listing renders the shared badges so a certified capability whose evidence has gone stale narrows the claim while still showing the Certified support class as context — proving Certified never implies Fresh — and a community-supported capability with fresh evidence reads as a current claim
  - Worked resolutions: 2
    - support `certified` + freshness `evidence_stale` → `claim_narrowed_evidence_stale` (note `evidence_stale`)
    - support `community` + freshness `fresh` → `claim_current` (note `current`)
- **Diagnostics Report**: `stable`
  - Owner: Diagnostics badge owner
  - Scope: The diagnostics report renders the shared badges so a limited-scope capability with stale evidence reads as narrowed-evidence-stale with a refresh next action, while an experimental capability with imported evidence reads as narrowed-imported-evidence with a reverify next action — the same two-cue vocabulary a diagnostics reviewer reads elsewhere
  - Worked resolutions: 2
    - support `limited` + freshness `evidence_stale` → `claim_narrowed_evidence_stale` (note `evidence_stale`)
    - support `experimental` + freshness `imported_evidence` → `claim_narrowed_imported_evidence` (note `imported_evidence`)
- **Certification Record**: `stable`
  - Owner: Certification badge owner
  - Scope: The certification record renders the shared badges so a certified capability whose retest is pending reads as retest-pending while keeping the Certified support class visible, and a supported capability whose evidence is stale narrows the claim while preserving the Supported context — support class and evidence age stay separate facts a certifier reads together
  - Worked resolutions: 2
    - support `certified` + freshness `retest_pending` → `claim_retest_pending` (note `retest_pending`)
    - support `supported` + freshness `evidence_stale` → `claim_narrowed_evidence_stale` (note `evidence_stale`)
- **Evaluation Pack**: `stable`
  - Owner: Evaluation badge owner
  - Scope: The evaluation pack renders the shared badges so a community-supported capability with imported evidence narrows the claim while keeping the Community context, and an experimental capability with fresh evidence reads as a current claim rather than being penalised on freshness for its lower support class — the same support-class / freshness vocabulary an evaluation reviewer reads elsewhere
  - Worked resolutions: 2
    - support `community` + freshness `imported_evidence` → `claim_narrowed_imported_evidence` (note `imported_evidence`)
    - support `experimental` + freshness `fresh` → `claim_current` (note `current`)
