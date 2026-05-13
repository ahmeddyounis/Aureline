# Restricted-Mode Alpha Fixtures

These fixtures drive `crates/aureline-auth/tests/restricted_mode_alpha.rs`.

The protected path proves that a launch-wedge workspace can open in restricted
mode, keep local read/edit/search/save capability available, and disclose every
blocked or review-needed execution, mutation, install, provider, and AI apply
capability with source, scope, and recovery action. The shell wedge inspector
consumes the same packet projection so trust gating remains visible after the
workspace opens.
