# Stabilize Hybrid Retrieval Epochs And Locality

Stable evidence packet:
[`hybrid_retrieval_inspector_packet.json`](hybrid_retrieval_inspector_packet.json).

The checked-in packet proves:

- query classification is present (`mixed`);
- lexical, structural, graph, embedding, and fusion lanes are declared;
- embedding recall is keyed by workspace, snapshot, retrieval epoch, embedder,
  tokenizer, chunker, trust boundary, policy scope, and retention policy;
- signed mirrored-pack embeddings disclose signature and compatibility refs;
- policy-hidden managed recall omissions are export-safe and disclosed;
- search results, AI context, review workspace, docs/help, and support export
  all preserve the same packet id; and
- retrieved candidates cannot authorize mutation without live target
  re-resolution.
