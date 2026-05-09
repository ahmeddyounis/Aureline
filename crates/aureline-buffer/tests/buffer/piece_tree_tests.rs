use aureline_buffer::{Buffer, BufferConfig, BufferError, RevisionId, TransactionSpec, UndoClass};

use crate::rng::{random_range, XorShift64};

#[test]
fn line_index_splits_lf_crlf_and_cr() {
    let mut buffer = Buffer::from_str("a\r\nb\nc\rd");
    let snapshot = buffer.snapshot();
    assert_eq!(snapshot.line_count(), 4);
    assert_eq!(snapshot.line_str(0), Some("a"));
    assert_eq!(snapshot.line_str(1), Some("b"));
    assert_eq!(snapshot.line_str(2), Some("c"));
    assert_eq!(snapshot.line_str(3), Some("d"));
}

#[test]
fn line_index_includes_final_empty_line_after_trailing_newline() {
    let mut buffer = Buffer::from_str("a\n");
    let snapshot = buffer.snapshot();
    assert_eq!(snapshot.line_count(), 2);
    assert_eq!(snapshot.line_str(0), Some("a"));
    assert_eq!(snapshot.line_str(1), Some(""));
}

#[test]
fn coordinate_translation_is_grapheme_aware() {
    let text = "a\u{0301}b\n"; // "áb\n"
    let mut buffer = Buffer::from_str(text);
    let snapshot = buffer.snapshot();
    assert_eq!(snapshot.grapheme_count_in_line(0), Some(2));

    let start = snapshot.byte_offset_for_line_grapheme(0, 0).unwrap();
    assert_eq!(start, 0);

    let second = snapshot.byte_offset_for_line_grapheme(0, 1).unwrap();
    let expected_second = "a\u{0301}".as_bytes().len();
    assert_eq!(second, expected_second);

    assert_eq!(snapshot.line_grapheme_for_byte_offset(second), Some((0, 1)));
}

#[test]
fn revision_ids_track_buffer_version() {
    let mut buffer = Buffer::from_str("hello");
    assert_eq!(buffer.revision_id(), RevisionId(buffer.version()));
    let before = buffer.revision_id();
    buffer.insert(5, "!", "user_keystroke").unwrap();
    assert_eq!(buffer.revision_id(), RevisionId(before.0 + 1));
}

#[test]
fn inverse_cap_rejection_is_a_deterministic_error_case() {
    let mut buffer = Buffer::from_bytes_with_config(
        b"abc",
        BufferConfig {
            inverse_cap_bytes: 4,
        },
    );
    let before = buffer.contents();
    let before_version = buffer.version();
    let err = buffer
        .insert(3, "0123456789", "user_keystroke")
        .unwrap_err();
    assert!(matches!(err, BufferError::InverseTooLarge { .. }));
    assert_eq!(buffer.contents(), before);
    assert_eq!(buffer.version(), before_version);
}

#[test]
fn randomized_edit_and_undo_restores_original_bytes() {
    let corpus = include_str!("../../../../fixtures/text/large/clean_small_text.txt");
    let mut buffer = Buffer::from_str(corpus);
    let original = buffer.contents();

    let mut rng = XorShift64::new(0xA5A5_F00D_D15E_A5A5);
    let mut commits = 0usize;
    for i in 0..128usize {
        let len = buffer.len();
        match i % 3 {
            0 => {
                let offset = rng.next_usize(len.saturating_add(1));
                buffer.insert(offset, "x", "user_keystroke").unwrap();
            }
            1 => {
                let range = random_range(&mut rng, len);
                buffer.delete(range, "user_keystroke").unwrap();
            }
            _ => {
                let range = random_range(&mut rng, len);
                buffer.replace(range, "y", "user_keystroke").unwrap();
            }
        }
        commits += 1;
    }

    for _ in 0..commits {
        buffer.undo().expect("undo must succeed");
    }

    assert_eq!(buffer.contents(), original);
}

#[test]
fn only_revertible_groups_restore_parent_state_exactly() {
    let mut buffer = Buffer::from_str("one\ntwo\nthree\n");
    let before = buffer.contents();

    let mut tx = buffer
        .begin(
            TransactionSpec::new(UndoClass::RefactorMultiFile, "command:apply_change")
                .with_label("Apply workspace change"),
        )
        .unwrap();
    tx.replace(0..3, "ONE").unwrap();
    tx.replace(4..7, "TWO").unwrap();
    tx.commit().unwrap();

    buffer.undo().expect("undo must restore parent snapshot");
    assert_eq!(buffer.contents(), before);
}
