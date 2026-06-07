# Database Safety And Result-Grid Labels

Database rows show stable labels only for the guarded contract they have
proved. The label does not mean every database engine, write workflow, or row
editing path is mature.

Stable database guard rows show:

- the connection class and execution origin;
- the auth-source mode without raw secrets;
- the current database/schema or imported scope;
- whether the target is read-only, write-capable, or policy blocked;
- statement-safety class and transaction posture before execution;
- result-grid scope, truncation, type headers, and export redaction;
- query-history retention/redaction posture;
- explain-plan mode, engine/version, and capture freshness;
- handoff destination, row/column scope, type-fidelity notes, freshness, and
  share/local restrictions.

Rows below stable are intentional. Preview, inspect-only, import-only, and labs
labels mean the surface lacks enough current evidence for stable database
execution or export claims.
