use super::{Score, Storage, Trace};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Path {
    // Start and end positions (row & column) are inclusive
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub score: Score,
    pub matches: usize,
    pub potential_matches: usize,
}

impl Path {
    fn new(start: (usize, usize), score: Score) -> Self {
        Self {
            start,
            end: start,
            score,
            matches: 1,
            potential_matches: 0,
        }
    }

    fn extend(&mut self, row: usize, col: usize, newscore: Score, op: Trace) {
        if op == Trace::Match {
            self.potential_matches += 1;
        }

        if newscore > self.score {
            // Real extend
            self.end = (row, col);
            self.score = newscore;
            self.matches += self.potential_matches;
            self.potential_matches = 0;
            debug_assert!(self.start.0 <= self.end.0);
            debug_assert!(self.start.1 <= self.end.1);
        }
    }
}

pub struct AllOptimal {
    pub minmatches: usize,
    pub minscore: Score,

    diagonal: Option<Path>,
    savediag: bool,
    // Cache for current per-row maximums
    cache: Vec<Option<Path>>,
    // Cache for finished paths in each row
    results: Vec<Vec<Path>>,
}

impl AllOptimal {
    pub fn new(rows: usize, minlen: usize, minscore: Score) -> Self {
        Self {
            minmatches: minlen,
            minscore,
            diagonal: None,
            savediag: false,
            cache: vec![None; rows],
            results: vec![Vec::new(); rows],
        }
    }

    fn save(&mut self, p: Path) {
        if p.score < self.minscore || p.matches < self.minmatches {
            return;
        }

        let row = p.start.0;
        for r in &mut self.results[row] {
            if r.start == p.start {
                // If match & better score -> update the hit
                if r.score < p.score {
                    *r = p;
                }
                return;
            }
        }
        // New match -> store the new path
        self.results[row].push(p)
    }
}

impl Storage for AllOptimal {
    type Output = Vec<Vec<Path>>;

    fn step(&mut self, row: usize, col: usize, newscore: Score, op: Trace) {
        // diagonal       | cache[row - 1]
        // cache[row]     | result
        let result = match op {
            Trace::None => {
                // Stop the current track
                (self.cache[row].take(), true)
            }
            Trace::GapRow => {
                let result = (self.cache[row].take(), true);
                // Copy data from the top column
                debug_assert!(self.cache[row - 1].is_some());
                self.cache[row] = self.cache[row - 1];
                result
            }
            Trace::GapCol => {
                // Same path, nothing to do
                debug_assert!(
                    self.cache[row].is_some() && self.cache[row].unwrap().score >= newscore
                );
                (self.cache[row], false)
            }
            Trace::Match | Trace::Mismatch => {
                let result = (self.cache[row].take(), true);
                match self.diagonal.take() {
                    None => {
                        // Start the new path
                        self.cache[row] = Some(Path::new((row, col), newscore));
                    }
                    Some(mut diagonal) => {
                        // Extend & consume the diagonal
                        diagonal.extend(row, col, newscore, op);
                        self.cache[row] = Some(diagonal);
                    }
                }
                result
            }
        };
        // Try to save the diagonal if it wasn't consumed before
        if let Some(diagonal) = self.diagonal.take() {
            if self.savediag {
                self.save(diagonal);
            }
        };

        // Update the diagonal
        (self.diagonal, self.savediag) = result;
    }

    fn on_column_end(&mut self) {
        if let Some(diagonal) = self.diagonal.take() {
            if self.savediag {
                self.save(diagonal);
            }
        };
        (self.diagonal, self.savediag) = (None, false);

        // Drop the row
        if let Some(Some(hit)) = self.cache.pop() {
            self.save(hit);
        }
    }

    fn prepare(&mut self, newrows: usize, _: usize) {
        self.cache.clear();
        self.diagonal = None;
        self.savediag = false;

        self.results.clear();
        self.results.resize(newrows, Vec::new());

        self.cache.clear();
        self.cache.resize(newrows, None);
    }

    fn finalize(&mut self) -> Self::Output {
        debug_assert!(self.cache.is_empty() && !self.results.is_empty());
        let results = std::mem::take(&mut self.results);
        self.prepare(0, 0);
        results
    }
}
