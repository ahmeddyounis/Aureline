use aureline_buffer::Buffer;

use crate::rng::XorShift64;

#[test]
fn edit_sequences_match_naive_model_and_roundtrip_undo_redo() {
    let mut seeds = vec![
        0xA5A5_F00D_D15E_A5A5,
        0x0123_4567_89AB_CDEF,
        0xDEAD_BEEF_CAFE_BABE,
        0x0000_0000_0000_0001,
    ];

    let corpus = include_str!("../../../../fixtures/text/large/clean_small_text.txt");
    let unicode = "a\u{0301} 👍🏽\n👩\u{200d}💻\n";
    let combined = format!("{corpus}\n{unicode}");
    let inputs = ["", "hello", "hello\nworld\n", unicode, combined.as_str()];

    for (seed_idx, seed) in seeds.drain(..).enumerate() {
        for (input_idx, input) in inputs.iter().enumerate() {
            let mut rng = XorShift64::new(seed ^ ((seed_idx as u64) << 32) ^ input_idx as u64);
            run_case(&mut rng, input);
        }
    }
}

fn run_case(rng: &mut XorShift64, initial: &str) {
    let mut buffer = Buffer::from_str(initial);
    let mut model = NaiveModel::new(initial);

    let steps = 512usize;
    for step in 0..steps {
        let choice = rng.next_usize(100);
        match choice {
            0..=9 => {
                if !model.can_undo() {
                    continue;
                }
                assert!(
                    buffer.undo().is_some(),
                    "undo should be available (step={step})"
                );
                model.undo();
            }
            10..=14 => {
                if !model.can_redo() {
                    continue;
                }
                assert!(
                    buffer.redo().is_some(),
                    "redo should be available (step={step})"
                );
                model.redo();
            }
            15..=59 => apply_insert(rng, &mut buffer, &mut model, step),
            60..=79 => apply_delete(rng, &mut buffer, &mut model, step),
            _ => apply_replace(rng, &mut buffer, &mut model, step),
        }

        assert_eq!(
            buffer.len(),
            buffer.contents().len(),
            "buffer length should match materialised contents (step={step})"
        );
        assert_eq!(
            buffer.contents(),
            model.text.as_bytes(),
            "buffer/model mismatch (step={step})"
        );
    }

    while model.can_undo() {
        assert!(buffer.undo().is_some(), "undo should be available");
        model.undo();
    }

    assert_eq!(
        buffer.contents(),
        initial.as_bytes(),
        "undo should restore original bytes"
    );

    while model.can_redo() {
        assert!(buffer.redo().is_some(), "redo should be available");
        model.redo();
    }

    assert_eq!(
        buffer.contents(),
        model.text.as_bytes(),
        "redo should restore final bytes"
    );
}

fn apply_insert(rng: &mut XorShift64, buffer: &mut Buffer, model: &mut NaiveModel, step: usize) {
    let boundaries = char_boundaries(&model.text);
    let offset = boundaries[rng.next_usize(boundaries.len())];
    let token = pick_insert_token(rng);
    buffer
        .insert(offset, token, "fixture_user_keystroke")
        .unwrap_or_else(|err| panic!("insert failed at step={step}: {err:?}"));
    model.insert(offset, token);
}

fn apply_delete(rng: &mut XorShift64, buffer: &mut Buffer, model: &mut NaiveModel, step: usize) {
    let boundaries = char_boundaries(&model.text);
    let a = boundaries[rng.next_usize(boundaries.len())];
    let b = boundaries[rng.next_usize(boundaries.len())];
    let (start, end) = if a <= b { (a, b) } else { (b, a) };
    buffer
        .delete(start..end, "fixture_user_keystroke")
        .unwrap_or_else(|err| panic!("delete failed at step={step}: {err:?}"));
    model.delete(start..end);
}

fn apply_replace(rng: &mut XorShift64, buffer: &mut Buffer, model: &mut NaiveModel, step: usize) {
    let boundaries = char_boundaries(&model.text);
    let a = boundaries[rng.next_usize(boundaries.len())];
    let b = boundaries[rng.next_usize(boundaries.len())];
    let (start, end) = if a <= b { (a, b) } else { (b, a) };
    let token = pick_insert_token(rng);
    buffer
        .replace(start..end, token, "fixture_user_keystroke")
        .unwrap_or_else(|err| panic!("replace failed at step={step}: {err:?}"));
    model.replace(start..end, token);
}

fn char_boundaries(text: &str) -> Vec<usize> {
    let mut out: Vec<usize> = text.char_indices().map(|(idx, _)| idx).collect();
    out.push(text.len());
    out.sort_unstable();
    out.dedup();
    if out.is_empty() {
        out.push(0);
    }
    out
}

fn pick_insert_token(rng: &mut XorShift64) -> &'static str {
    const TOKENS: [&str; 12] = [
        "x",
        "y",
        " ",
        "\n",
        "\r\n",
        "🙂",
        "👍🏽",
        "👩\u{200d}💻",
        "a\u{0301}",
        "Ω",
        "中",
        "λ",
    ];
    TOKENS[rng.next_usize(TOKENS.len())]
}

struct NaiveModel {
    text: String,
    undo_stack: Vec<String>,
    redo_stack: Vec<String>,
}

impl NaiveModel {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    fn checkpoint(&mut self) {
        self.undo_stack.push(self.text.clone());
        self.redo_stack.clear();
    }

    fn insert(&mut self, offset: usize, text: &str) {
        self.checkpoint();
        self.text.insert_str(offset, text);
    }

    fn delete(&mut self, range: std::ops::Range<usize>) {
        self.checkpoint();
        self.text.replace_range(range, "");
    }

    fn replace(&mut self, range: std::ops::Range<usize>, text: &str) {
        self.checkpoint();
        self.text.replace_range(range, text);
    }

    fn undo(&mut self) {
        let prev = self.undo_stack.pop().expect("undo must exist");
        let current = std::mem::replace(&mut self.text, prev);
        self.redo_stack.push(current);
    }

    fn redo(&mut self) {
        let next = self.redo_stack.pop().expect("redo must exist");
        let current = std::mem::replace(&mut self.text, next);
        self.undo_stack.push(current);
    }
}
