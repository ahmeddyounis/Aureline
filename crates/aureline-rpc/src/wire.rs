//! Outer framing identifiers.
//!
//! Frozen at `wire/1` + `aureline/bin/1` at the initial decision.
//! A new wire version or a second encoding id requires a new decision
//! row and cannot be added by editing this module in isolation.

/// Outer framing protocol identifier.
pub const WIRE_PROTOCOL_VERSION: WireProtocolVersion = WireProtocolVersion("wire/1");

/// Connection-level wire encoding id negotiated in the handshake.
pub const ENCODING_ID: EncodingId = EncodingId("aureline/bin/1");

/// Newtype for the wire protocol identifier so callers cannot pass an
/// arbitrary string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WireProtocolVersion(pub &'static str);

impl WireProtocolVersion {
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}

/// Newtype for the wire encoding id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EncodingId(pub &'static str);

impl EncodingId {
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}
