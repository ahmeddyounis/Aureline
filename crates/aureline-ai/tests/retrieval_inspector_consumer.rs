//! AI consumer coverage for retrieval-inspector packet parity.
//!
//! AI context export wraps the search-owned retrieval packet instead of
//! re-deriving lane, locality, or embedding truth from context rows.

use std::path::{Path, PathBuf};

use aureline_ai::AiContextRetrievalExport;
use aureline_search::{
    RetrievalConsumerSurface, RetrievalInspectorPacket, HYBRID_RETRIEVAL_BETA_PACKET_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root canonicalizes")
}

#[test]
fn ai_context_export_preserves_search_owned_retrieval_packet() {
    let path = repo_root().join(HYBRID_RETRIEVAL_BETA_PACKET_REF);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("artifact {path:?} must read: {err}"));
    let packet: RetrievalInspectorPacket = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("artifact {path:?} must parse: {err}"));

    assert!(packet.has_projection_for(RetrievalConsumerSurface::AiContext));
    assert!(packet.validate().is_empty());

    let export = AiContextRetrievalExport::from_packet(
        "ai-context-export:retrieval-inspector:m3:0001",
        "context-snapshot:ai:retrieval-inspector:m3:0001",
        "request-workspace:retrieval-inspector:m3:0001",
        "2026-05-17T12:22:00Z",
        packet.clone(),
    );

    assert!(export.validate().is_empty());
    assert_eq!(export.retrieval_findings(), Vec::new());
    assert_eq!(export.retrieval_packet, packet);
}
