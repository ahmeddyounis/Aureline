# Approval Ticket Alpha Fixtures

`baseline_packet.json` is the protected launch-wedge packet for provider and helper-backed authority. It proves that high-risk mutations cite a short-lived approval ticket or reviewed-scope object, preserve export-safe actor lineage, and fail closed on expiry, target drift, trust/profile drift, sandbox drift, policy-epoch drift, and actor-scope mismatch.

Run the provider alpha consumer with:

```sh
cargo run -p aureline-provider --bin aureline_provider_alpha -- --approval-ticket-alpha --validate-only
```
