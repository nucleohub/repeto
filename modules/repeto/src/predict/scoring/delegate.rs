use super::super::{Score, Symbol};

use super::{gaps, symbols};

pub struct Delegate<G: gaps::ScoringFunc, S: symbols::ScoringFunc> {
    pub gap: G,
    pub symbols: S,
}

impl<G: gaps::ScoringFunc, S: symbols::ScoringFunc> gaps::ScoringFunc for Delegate<G, S> {
    #[inline(always)]
    fn seq1_gap_open(&self, pos: usize) -> Score {
        self.gap.seq1_gap_open(pos)
    }

    #[inline(always)]
    fn seq1_gap_extend(&self, pos: usize) -> Score {
        self.gap.seq1_gap_extend(pos)
    }

    #[inline(always)]
    fn seq2_gap_open(&self, pos: usize) -> Score {
        self.gap.seq2_gap_open(pos)
    }

    #[inline(always)]
    fn seq2_gap_extend(&self, pos: usize) -> Score {
        self.gap.seq2_gap_extend(pos)
    }
}

impl<G: gaps::ScoringFunc, S: symbols::ScoringFunc> symbols::ScoringFunc for Delegate<G, S> {
    #[inline(always)]
    fn score(&self, posa: usize, a: Symbol, posb: usize, b: Symbol) -> (Score, symbols::SubType) {
        self.symbols.score(posa, a, posb, b)
    }
}

impl<G: gaps::ScoringFunc, S: symbols::ScoringFunc> super::ScoringFunc for Delegate<G, S> {}
