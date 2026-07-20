// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Process-local, one-shot authority for preview/apply Git flows.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::status::RepositoryIdentity;

static NEXT_SERVICE_ID: AtomicU64 = AtomicU64::new(1);

const MAX_AUTHORITY_ENTRIES: usize = 64;
const MAX_AUTHORITY_ENTRY_BYTES: usize = 8 * 1024 * 1024;
const MAX_AUTHORITY_TOTAL_BYTES: usize = 32 * 1024 * 1024;
const MAX_AUTHORITY_AGE: Duration = Duration::from_secs(10 * 60);

#[derive(Debug)]
struct AuthorityEntry<A> {
    token: u128,
    sequence: u64,
    issued_at: Instant,
    retained_bytes: usize,
    projection: Vec<u8>,
    authority: A,
}

#[derive(Debug)]
struct AuthorityState<A> {
    retained_bytes: usize,
    entries: HashMap<String, AuthorityEntry<A>>,
}

/// Binds a serializable inspection record to non-serializable apply evidence.
///
/// Entries are deliberately process-local and one-shot. A cloned live preview
/// can be applied through a clone of its issuing service, while a deserialized
/// record, a preview from another service, or a tampered public projection
/// cannot become mutation authority.
pub(crate) struct PreviewAuthorityStore<A> {
    service_id: u64,
    next_token: AtomicU64,
    state: Mutex<AuthorityState<A>>,
}

impl<A> std::fmt::Debug for PreviewAuthorityStore<A> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (entry_count, retained_bytes) = self
            .state
            .lock()
            .map(|state| (state.entries.len(), state.retained_bytes))
            .unwrap_or_default();
        formatter
            .debug_struct("PreviewAuthorityStore")
            .field("service_id", &self.service_id)
            .field("entry_count", &entry_count)
            .field("retained_bytes", &retained_bytes)
            .finish_non_exhaustive()
    }
}

impl<A> Default for PreviewAuthorityStore<A> {
    fn default() -> Self {
        Self {
            service_id: NEXT_SERVICE_ID.fetch_add(1, Ordering::Relaxed),
            next_token: AtomicU64::new(1),
            state: Mutex::new(AuthorityState {
                retained_bytes: 0,
                entries: HashMap::new(),
            }),
        }
    }
}

impl<A> PreviewAuthorityStore<A> {
    /// Issues fresh authority and invalidates an older preview with the same
    /// stable record ref.
    pub(crate) fn issue(
        &self,
        preview_ref: &str,
        projection: Vec<u8>,
        authority: A,
        authority_bytes: usize,
    ) -> Option<u128> {
        self.issue_at(
            preview_ref,
            projection,
            authority,
            authority_bytes,
            Instant::now(),
        )
    }

    fn issue_at(
        &self,
        preview_ref: &str,
        projection: Vec<u8>,
        authority: A,
        authority_bytes: usize,
        issued_at: Instant,
    ) -> Option<u128> {
        let retained_bytes = projection
            .capacity()
            .checked_add(authority_bytes)?
            .checked_add(preview_ref.len())?
            .checked_add(std::mem::size_of::<AuthorityEntry<A>>())?;
        let mut state = self.state.lock().ok()?;
        purge_expired(&mut state, issued_at);
        remove_entry(&mut state, preview_ref);
        if retained_bytes > MAX_AUTHORITY_ENTRY_BYTES {
            return None;
        }
        let sequence = self.next_token.fetch_add(1, Ordering::Relaxed);
        if sequence == 0 || self.service_id == 0 {
            return None;
        }
        let token = (u128::from(self.service_id) << 64) | u128::from(sequence);
        while state.entries.len() >= MAX_AUTHORITY_ENTRIES
            || state.retained_bytes.saturating_add(retained_bytes) > MAX_AUTHORITY_TOTAL_BYTES
        {
            evict_oldest(&mut state)?;
        }
        state.retained_bytes = state.retained_bytes.checked_add(retained_bytes)?;
        state.entries.insert(
            preview_ref.to_string(),
            AuthorityEntry {
                token,
                sequence,
                issued_at,
                retained_bytes,
                projection,
                authority,
            },
        );
        Some(token)
    }

    /// Invalidates any older authority using the same stable preview ref.
    pub(crate) fn revoke(&self, preview_ref: &str) {
        if let Ok(mut state) = self.state.lock() {
            purge_expired(&mut state, Instant::now());
            remove_entry(&mut state, preview_ref);
        }
    }

    /// Consumes authority only when both the private token and the complete
    /// serialized inspection projection still match the issued preview.
    pub(crate) fn consume(
        &self,
        preview_ref: &str,
        token: Option<u128>,
        projection: &[u8],
    ) -> Option<A> {
        let token = token?;
        let mut state = self.state.lock().ok()?;
        purge_expired(&mut state, Instant::now());
        let entry = state.entries.get(preview_ref)?;
        if entry.token != token || entry.projection != projection {
            return None;
        }
        let entry = state.entries.remove(preview_ref)?;
        state.retained_bytes = state.retained_bytes.saturating_sub(entry.retained_bytes);
        Some(entry.authority)
    }
}

fn purge_expired<A>(state: &mut AuthorityState<A>, now: Instant) {
    let expired = state
        .entries
        .iter()
        .filter(|(_, entry)| {
            now.checked_duration_since(entry.issued_at)
                .is_some_and(|age| age >= MAX_AUTHORITY_AGE)
        })
        .map(|(preview_ref, _)| preview_ref.clone())
        .collect::<Vec<_>>();
    for preview_ref in expired {
        if let Some(entry) = state.entries.remove(&preview_ref) {
            state.retained_bytes = state.retained_bytes.saturating_sub(entry.retained_bytes);
        }
    }
}

fn evict_oldest<A>(state: &mut AuthorityState<A>) -> Option<()> {
    let oldest = state
        .entries
        .iter()
        .min_by(|(left_ref, left), (right_ref, right)| {
            left.sequence
                .cmp(&right.sequence)
                .then_with(|| left_ref.cmp(right_ref))
        })
        .map(|(preview_ref, _)| preview_ref.clone())?;
    let entry = state.entries.remove(&oldest)?;
    state.retained_bytes = state.retained_bytes.saturating_sub(entry.retained_bytes);
    Some(())
}

fn remove_entry<A>(state: &mut AuthorityState<A>, preview_ref: &str) {
    if let Some(entry) = state.entries.remove(preview_ref) {
        state.retained_bytes = state.retained_bytes.saturating_sub(entry.retained_bytes);
    }
}

/// Compares the canonical repository/worktree and Git storage boundaries while
/// tolerating presentation aliases such as macOS `/var` versus `/private/var`.
pub(crate) fn same_repository_identity(
    expected: &RepositoryIdentity,
    observed: &RepositoryIdentity,
) -> bool {
    expected.repo_ref == observed.repo_ref
        && expected.worktree_ref == observed.worktree_ref
        && same_canonical_path(&expected.repo_root, &observed.repo_root)
        && same_canonical_path(&expected.git_dir, &observed.git_dir)
        && same_canonical_path(&expected.common_dir, &observed.common_dir)
}

fn same_canonical_path(left: &std::path::Path, right: &std::path::Path) -> bool {
    match (std::fs::canonicalize(left), std::fs::canonicalize(right)) {
        (Ok(left), Ok(right)) => left == right,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authority_is_projection_bound_and_one_shot() {
        let store = PreviewAuthorityStore::default();
        let token = store
            .issue("preview", b"projection".to_vec(), "authority", 9)
            .expect("authority issued");

        assert_eq!(
            store.consume("preview", Some(token), b"tampered"),
            None,
            "tampering does not yield authority"
        );
        assert_eq!(
            store.consume("preview", Some(token), b"projection"),
            Some("authority")
        );
        assert_eq!(
            store.consume("preview", Some(token), b"projection"),
            None,
            "authority cannot be replayed"
        );
    }

    #[test]
    fn debug_output_never_exposes_retained_projection_or_authority() {
        let store = PreviewAuthorityStore::default();
        let _ = store.issue(
            "private-preview-ref",
            b"private-projection".to_vec(),
            "private-authority",
            17,
        );

        let debug = format!("{store:?}");
        assert!(!debug.contains("private-preview-ref"));
        assert!(!debug.contains("private-projection"));
        assert!(!debug.contains("private-authority"));
        assert!(debug.contains("entry_count"));
    }

    #[test]
    fn authority_does_not_cross_service_instances() {
        let first = PreviewAuthorityStore::default();
        let second = PreviewAuthorityStore::default();
        let first_token = first
            .issue("preview", b"projection".to_vec(), "first", 5)
            .expect("first authority issued");
        let _second_token = second
            .issue("preview", b"projection".to_vec(), "second", 6)
            .expect("second authority issued");

        assert_eq!(
            second.consume("preview", Some(first_token), b"projection"),
            None
        );
    }

    #[test]
    fn revoke_and_failed_replacement_invalidate_older_authority() {
        let store = PreviewAuthorityStore::default();
        let token = store
            .issue("preview", b"projection".to_vec(), "old", 3)
            .expect("old authority issued");
        store.revoke("preview");
        assert_eq!(store.consume("preview", Some(token), b"projection"), None);

        let token = store
            .issue("preview", b"projection".to_vec(), "old", 3)
            .expect("replacement basis issued");
        assert_eq!(
            store.issue(
                "preview",
                vec![0; MAX_AUTHORITY_ENTRY_BYTES],
                "oversized",
                1,
            ),
            None
        );
        assert_eq!(
            store.consume("preview", Some(token), b"projection"),
            None,
            "a failed newer issue must not leave the older token live"
        );
    }

    #[test]
    fn authority_is_bounded_and_evicts_oldest_deterministically() {
        let store = PreviewAuthorityStore::default();
        let mut tokens = Vec::new();
        for index in 0..=MAX_AUTHORITY_ENTRIES {
            let preview_ref = format!("preview-{index:03}");
            let token = store
                .issue(&preview_ref, vec![index as u8], index, 8)
                .expect("bounded authority issued");
            tokens.push((preview_ref, token, index));
        }
        let (oldest_ref, oldest_token, _) = &tokens[0];
        assert_eq!(
            store.consume(oldest_ref, Some(*oldest_token), &[0]),
            None,
            "oldest authority is evicted first"
        );
        let (newest_ref, newest_token, newest_value) = tokens.last().expect("newest token");
        assert_eq!(
            store.consume(newest_ref, Some(*newest_token), &[*newest_value as u8]),
            Some(*newest_value)
        );
    }

    #[test]
    fn oversized_and_expired_authority_is_rejected() {
        let store = PreviewAuthorityStore::default();
        assert_eq!(
            store.issue(
                "oversized",
                vec![0; MAX_AUTHORITY_ENTRY_BYTES],
                "authority",
                1,
            ),
            None
        );

        let issued_at = Instant::now()
            .checked_sub(MAX_AUTHORITY_AGE + Duration::from_secs(1))
            .expect("old instant");
        let token = store
            .issue_at("expired", b"projection".to_vec(), "expired", 7, issued_at)
            .expect("old authority can be inserted for expiry test");
        assert_eq!(store.consume("expired", Some(token), b"projection"), None);
    }

    #[test]
    fn total_retained_byte_limit_evicts_oldest_authority() {
        let store = PreviewAuthorityStore::default();
        let projection_bytes = 7 * 1024 * 1024;
        let mut issued = Vec::new();
        for index in 0..5 {
            let preview_ref = format!("large-preview-{index}");
            let projection = vec![index as u8; projection_bytes];
            let token = store
                .issue(&preview_ref, projection, index, 1)
                .expect("individual authority stays under the per-entry limit");
            issued.push((preview_ref, token, index));
        }
        let (oldest_ref, oldest_token, oldest_value) = &issued[0];
        assert_eq!(
            store.consume(
                oldest_ref,
                Some(*oldest_token),
                &vec![*oldest_value as u8; projection_bytes]
            ),
            None,
            "the fifth seven-megabyte entry deterministically evicts the oldest"
        );
        let (newest_ref, newest_token, newest_value) = issued.last().expect("newest entry");
        assert_eq!(
            store.consume(
                newest_ref,
                Some(*newest_token),
                &vec![*newest_value as u8; projection_bytes]
            ),
            Some(*newest_value)
        );
    }
}
