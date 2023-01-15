use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct ClipboardHistory<const N: usize> {
    deque: VecDeque<String>,
    cursor: usize,
}

impl<const N: usize> ClipboardHistory<N> {
    const MAX_SIZE: usize = N;

    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, value: String) {
        if self.is_full() {
            self.deque.pop_front();
        } else {
            self.cursor += 1;
        }

        self.deque.push_back(value);
    }

    pub fn is_full(&self) -> bool {
        self.cursor == Self::MAX_SIZE
    }

    pub fn len(&self) -> usize {
        Self::MAX_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::ClipboardHistory;

    #[test]
    fn test_clipboard_history_size_is_fixed() {
        let mut history: ClipboardHistory<5> = ClipboardHistory::new();
        assert!(!history.is_full());
        assert_eq!(history.len(), 5);

        history.push("a".to_string());
        history.push("b".to_string());
        history.push("c".to_string());
        history.push("d".to_string());
        history.push("e".to_string());

        assert!(history.is_full());
        assert_eq!(history.len(), 5);

        history.push("f".to_string());

        assert!(history.is_full());
        assert_eq!(history.len(), 5);
    }
}
