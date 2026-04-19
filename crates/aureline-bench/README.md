# aureline-bench

## Purpose
Benchmark harness and trace-fixture host for protected-path workflows. Drives
latency, throughput, and trace-fidelity measurements that feed governance
artifacts and certification packets.

## Protected-path status
Not protected itself. Governs evidence for protected paths but is not on a
hot path; production crates must never depend on it.

## Allowed dependencies
- May depend on any other internal crate to exercise it from a benchmark.
- Must not be depended on by any other internal crate.

## Canonical owner path
`crates/aureline-bench/`

## Work packages
- WP-14 (Quality engineering, certification, and claim publication)
