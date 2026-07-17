# Shared Review-Pack Consumers: One Vocabulary Across Surfaces

- Packet: `m5-review-pack-shared-consumers:stable:0001`
- Surface: `M5 review-pack shared consumers (one vocabulary across surfaces)`
- Consumer bindings: 18 (8 narrowed)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-16T00:00:00Z)

## Consumer bindings

- **Review-pack record (one repo-defined pack: version / digest, scope selectors, evaluator identity)** [`rpsc-record-review-detail`]: object `review_pack_record` on `review_detail`, representation `desktop_full`, role `pack_version_and_digest_disclosure`
- **Review-pack record (one repo-defined pack: version / digest, scope selectors, evaluator identity)** [`rpsc-record-summary`]: object `review_pack_record` on `review_pack_summary`, representation `desktop_full`, role `pack_version_and_digest_disclosure`
- **Review-pack record (one repo-defined pack: version / digest, scope selectors, evaluator identity)** [`rpsc-record-support`]: object `review_pack_record` on `support_export_packet`, representation `exported_redacted`, role `pack_version_and_digest_disclosure`
- **Ownership signal (advisory-owner-versus-enforced-owner provenance for a scope slice)** [`rpsc-ownership-overlay`]: object `ownership_signal` on `ownership_overlay`, representation `desktop_full`, role `owner_provenance_disclosure`
- **Ownership signal (advisory-owner-versus-enforced-owner provenance for a scope slice)** [`rpsc-ownership-review-detail`]: object `ownership_signal` on `review_detail`, representation `compact_narrowed`, role `owner_provenance_disclosure`
- **Ownership signal (advisory-owner-versus-enforced-owner provenance for a scope slice)** [`rpsc-ownership-support`]: object `ownership_signal` on `support_export_packet`, representation `exported_redacted`, role `owner_provenance_disclosure`
- **Required-evidence / required-check row (one required evidence or check plus its evaluator result class)** [`rpsc-evidence-merge-readiness`]: object `required_evidence_check_row` on `merge_readiness`, representation `desktop_full`, role `required_evidence_and_check_disclosure`
- **Required-evidence / required-check row (one required evidence or check plus its evaluator result class)** [`rpsc-evidence-review-detail`]: object `required_evidence_check_row` on `review_detail`, representation `desktop_full`, role `required_evidence_and_check_disclosure`
- **Required-evidence / required-check row (one required evidence or check plus its evaluator result class)** [`rpsc-evidence-support`]: object `required_evidence_check_row` on `support_export_packet`, representation `exported_redacted`, role `required_evidence_and_check_disclosure`
- **Local-CI parity strip (local-parity-estimate-versus-provider-authoritative state per check)** [`rpsc-parity-strip`]: object `local_ci_parity_strip` on `local_ci_parity_strip`, representation `desktop_full`, role `local_versus_provider_parity_disclosure`
- **Local-CI parity strip (local-parity-estimate-versus-provider-authoritative state per check)** [`rpsc-parity-merge-readiness`]: object `local_ci_parity_strip` on `merge_readiness`, representation `remote_projected`, role `local_versus_provider_parity_disclosure`
- **Local-CI parity strip (local-parity-estimate-versus-provider-authoritative state per check)** [`rpsc-parity-provider-handoff`]: object `local_ci_parity_strip` on `provider_handoff`, representation `desktop_full`, role `local_versus_provider_parity_disclosure`
- **AI review policy hook (an AI review run under a disclosed review-pack version / digest and policy)** [`rpsc-aihook-ai-panel`]: object `ai_policy_hook` on `ai_review_panel`, representation `desktop_full`, role `evaluator_result_class_disclosure`
- **AI review policy hook (an AI review run under a disclosed review-pack version / digest and policy)** [`rpsc-aihook-review-detail`]: object `ai_policy_hook` on `review_detail`, representation `desktop_full`, role `evaluator_result_class_disclosure`
- **AI review policy hook (an AI review run under a disclosed review-pack version / digest and policy)** [`rpsc-aihook-provider-handoff`]: object `ai_policy_hook` on `provider_handoff`, representation `remote_projected`, role `evaluator_result_class_disclosure`
- **Review-template packet (comment / summary template and attribution bound to the pack it came from)** [`rpsc-template-summary`]: object `review_template_packet` on `review_pack_summary`, representation `desktop_full`, role `template_attribution_disclosure`
- **Review-template packet (comment / summary template and attribution bound to the pack it came from)** [`rpsc-template-help-docs`]: object `review_template_packet` on `help_docs`, representation `compact_narrowed`, role `template_attribution_disclosure`
- **Review-template packet (comment / summary template and attribution bound to the pack it came from)** [`rpsc-template-support`]: object `review_template_packet` on `support_export_packet`, representation `exported_redacted`, role `template_attribution_disclosure`
