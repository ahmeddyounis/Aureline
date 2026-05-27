# publish_launch_language_conformance_packs_truth_packet fixture corpus

This directory pins the baseline stable posture for the launch-language
conformance pack publication truth packet plus five narrowed-below-stable
postures. Each JSON file declares a `case_name`, `scenario`, packet
`input`, and an `expect` block carrying the expected promotion state,
finding count, row count, closed-vocabulary token sets, and expected
finding kinds. The integration tests at
`crates/aureline-language/tests/publish_launch_language_conformance_packs_truth_packet.rs`
load every fixture and assert the validator agrees with the recorded
posture.
