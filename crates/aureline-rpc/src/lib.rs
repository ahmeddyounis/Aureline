//! RPC transport for the supervisor / service fabric.
//!
//! Provides the framing, request/response surface, deadline and cancellation
//! plumbing, and trace-context propagation used by every cross-process call.

#![doc(html_root_url = "https://docs.rs/aureline-rpc/0.0.0")]
