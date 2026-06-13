# M5 AI/Docs/Recall Row Certification Fixtures

## support_export_row_narrows_on_stale_spend_receipt.json

An auto-narrowing drill fixture for the recall-row certification capstone. Every
claimed M5 AI/docs/recall surface — composer inline assist, patch review,
side-branch agents, docs/browser recall, code understanding, semantic/hybrid
search, managed/offline reporting, and support/export — carries its four proof
pillars: a memory-class proof, a prompt-result-cache and session-artifact
governance proof, a hybrid-retrieval or embedding-locality proof, and a
spend-receipt proof.

The support/export row claims `stable` but its spend-receipt pillar has aged to
`stale`. Because a claimed row may not outrun current proof, the row auto-narrows
to `effective` `beta`, records a `proof_stale` narrowing trigger, and carries a
precise degraded label rather than a generic provider error. Every other row keeps
all four pillars current, managed and mixed locality stays disclosed, the
semantic/hybrid lane labels its mixed retrieval generation, and every durable
pillar declares its retention/delete/export posture.

The fixture validates against
`schemas/ai/certify-ai-memory-classes-prompt-result-cache-and-session-artifact-governance-hybrid-retrieval-or-embedding-locality-and.schema.json`.
