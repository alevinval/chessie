use std::sync::{LazyLock, Mutex};

use crate::search::SearchResult;

const TT_SIZE: usize = 1 << 20;

pub(crate) static TT: LazyLock<TranspositionTable> = LazyLock::new(TranspositionTable::new);

pub(crate) struct TranspositionTable {
    entries: Mutex<Vec<Option<TTEntry>>>,
}

impl TranspositionTable {
    fn new() -> Self {
        Self { entries: Mutex::new(vec![None; TT_SIZE]) }
    }

    /// Returns the entry if it matches `hash`, was stored at a shallower or
    /// equal ply, and is either a mate score (exact at any depth) or was
    /// stored with at least `remaining` depth left.
    pub fn probe(&self, hash: u64, ply: usize, remaining: usize) -> Option<TTEntry> {
        let entries = self.entries.lock().unwrap();
        let entry = entries[tt_idx(hash)]?;
        let deep_enough = entry.depth >= remaining || entry.result.mate_dist.is_some();
        (entry.hash == hash && entry.ply <= ply && deep_enough).then_some(entry)
    }

    /// Replaces the slot only if the new entry is at least as deep as the
    /// one stored.
    pub fn store(&self, entry: TTEntry) {
        let mut entries = self.entries.lock().unwrap();
        let i = tt_idx(entry.hash);
        if !entries[i].is_some_and(|old| entry.ply < old.ply) {
            entries[i] = Some(entry);
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum Bound {
    Exact,
    Lower,
    Upper,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct TTEntry {
    pub hash: u64,
    pub bound: Bound,
    pub ply: usize,
    pub depth: usize,
    pub result: SearchResult,
}

const fn tt_idx(hash: u64) -> usize {
    hash as usize & (TT_SIZE - 1)
}

#[cfg(test)]
mod test {
    use super::*;

    fn entry(hash: u64, ply: usize, depth: usize) -> TTEntry {
        TTEntry {
            hash,
            bound: Bound::Exact,
            ply,
            depth,
            result: SearchResult { eval: 0, movement: None, mate_dist: None },
        }
    }

    #[test]
    fn stored_entry_is_probed_back() {
        let tt = TranspositionTable::new();
        tt.store(entry(0xdead_beef, 3, 1));
        assert_eq!(tt.probe(0xdead_beef, 5, 1), Some(entry(0xdead_beef, 3, 1)));
        assert_eq!(tt.probe(0xdead_beef, 2, 1), None);
    }

    #[test]
    fn shallower_stored_entry_is_rejected() {
        let tt = TranspositionTable::new();
        tt.store(entry(0xdead_beef, 3, 1));
        assert_eq!(tt.probe(0xdead_beef, 5, 2), None);
    }

    #[test]
    fn mate_score_is_probed_at_any_remaining_depth() {
        let tt = TranspositionTable::new();
        let mut e = entry(0xdead_beef, 3, 0);
        e.result.mate_dist = Some(0);
        tt.store(e);
        assert_eq!(tt.probe(0xdead_beef, 5, 2), Some(e));
    }

    #[test]
    fn shallower_entry_does_not_replace_deeper() {
        let tt = TranspositionTable::new();
        tt.store(entry(0xdead_beef, 5, 0));
        tt.store(entry(0xdead_beef, 2, 2));
        assert_eq!(tt.probe(0xdead_beef, 5, 0), Some(entry(0xdead_beef, 5, 0)));
    }

    #[test]
    fn deeper_entry_replaces_shallower() {
        let tt = TranspositionTable::new();
        tt.store(entry(0xdead_beef, 2, 2));
        tt.store(entry(0xdead_beef, 5, 0));
        assert_eq!(tt.probe(0xdead_beef, 5, 0), Some(entry(0xdead_beef, 5, 0)));
    }
}
