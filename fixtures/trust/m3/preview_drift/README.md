# Preview Drift Fixtures

These fixtures replay the beta preview-commit guard across high-risk apply
classes. Each case proves that stale preview or approval state blocks apply,
emits stable reason tokens, and sends the caller back through review rather
than auto-refreshing the proof basis.
