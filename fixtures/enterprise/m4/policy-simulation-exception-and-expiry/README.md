# Policy Simulation, Exception Preview, Approval History, and Expiry Fixtures

These fixtures are generated from
`aureline_policy::policy_simulation_and_expiry`. They are the canonical stable
projection for policy-simulation views, exception-preview sheets, approval
history, diff summaries, expiry banners, support export, and managed-admin
handoff packets.

## Regenerate

```sh
cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- page > page.json
cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- simulation-views > simulation_views.json
cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- exception-sheets > exception_sheets.json
cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- approval-history > approval_history.json
cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- diff-summaries > diff_summaries.json
cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- expiry-banners > expiry_banners.json
cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- review-packet > review_packet.json
cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- summary > summary.json
cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- defects > defects.json
cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- support-export > support_export.json
cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- drill-raw-private-material-withdrawn > drill_raw_private_material_withdrawn.json
cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- drill-indefinite-approval-needs-review > drill_indefinite_approval_needs_review.json
cargo run -q -p aureline-policy --example dump_policy_simulation_and_expiry_fixtures -- drill-missing-projection-needs-review > drill_missing_projection_needs_review.json
```

## Files

| File | Purpose |
| --- | --- |
| `page.json` | Full stable packet with source beta page and all stable projections. |
| `simulation_views.json` | Pre-apply simulation views with changed areas, consequences, degraded modes, and expiry links. |
| `exception_sheets.json` | Exception and waiver preview sheets. |
| `approval_history.json` | Remembered-decision approval history rows. |
| `diff_summaries.json` | Policy diff and impact summaries. |
| `expiry_banners.json` | Expiry banners for exceptions, waivers, and remembered decisions. |
| `review_packet.json` | Cross-surface packet for desktop, CLI/headless, and admin/support handoff. |
| `summary.json` | Stable object counts and qualification token. |
| `defects.json` | Defect list for the seeded packet, empty when stable. |
| `support_export.json` | Support-export wrapper. |
| `drill_raw_private_material_withdrawn.json` | Failure drill: raw material withdraws the packet. |
| `drill_indefinite_approval_needs_review.json` | Failure drill: high-risk remembered decision without expiry narrows to review. |
| `drill_missing_projection_needs_review.json` | Failure drill: missing cross-surface projection narrows to review. |
