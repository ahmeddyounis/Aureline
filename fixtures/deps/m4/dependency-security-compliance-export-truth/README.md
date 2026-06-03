# Fixtures: Dependency-security-compliance export truth

This directory contains example and test fixtures for the
`dependency_security_compliance_export_truth` packet.

## Files

- `valid_packet.json` — A complete, valid packet that exercises every
  vocabulary token and row kind.
- `no_findings_packet.json` — A minimal packet representing a workspace with
  no active findings and current feed data.
- `feed_outage_packet.json` — A minimal packet representing a feed outage
  where no advisory claim can be made.

All fixtures are intended for unit tests, integration tests, and schema
validation in CI.
