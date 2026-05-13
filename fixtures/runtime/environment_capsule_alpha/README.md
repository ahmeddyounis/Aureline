# Environment-capsule alpha fixtures

Protected fixture expectations for the environment-capsule alpha seed
contract. The validator checks these ids against the checked-in workspace
template seed so capsule, template, launch-bundle, and Start Center truth
cannot drift silently.

The fixture intentionally asserts ids and acceptance states rather than raw
environment values. Secret-bearing variables must stay represented as
credential aliases and secret classes.
