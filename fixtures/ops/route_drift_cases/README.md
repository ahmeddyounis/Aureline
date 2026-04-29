# Route and provenance drift worked cases

These fixtures are short, reviewable scenarios that anchor the contract
frozen in:

- [`/docs/ops/event_provenance_and_route_inspector_contract.md`](../../../docs/ops/event_provenance_and_route_inspector_contract.md)

The packets are pre-implementation examples. They demonstrate the
minimum acceptance set for the inspector contract:

- local-only command with inspectable `No external route used` truth;
- remote-agent action with a hop timeline and stable event/run/session
  lineage;
- external-provider request with approval/decision linkage;
- proxy/certificate/mirror drift that forces renewed review rather than
  silent replay;
- imported/offline evidence reopening labeled non-live; and
- queued replay after reconnect that revalidates drift before replay.

