# aureline-telemetry

## Purpose
Hot-path instrumentation, tracing, and metrics primitives. Hosts the recording
surface used by every other crate to emit spans, counters, and latency samples
that feed protected-path traces and support-bundle export.

The crate also owns the onboarding task-success telemetry recorder for
privacy-safe first-useful-work timing and migration funnel event captures.

## Protected-path status
Protected. Latency-trace fidelity, redaction discipline, and trace-context
correctness depend on this crate.

## Allowed dependencies
- No internal dependencies. This crate is a leaf foundation usable by any
  other internal crate.

## Canonical owner path
`crates/aureline-telemetry/`

## Work packages
- WP-13 (Release engineering and governance)
- WP-14 (Quality engineering, certification, and claim publication)
