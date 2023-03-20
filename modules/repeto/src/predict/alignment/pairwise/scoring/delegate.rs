use super::{gaps, symbols};
use super::super::{Score, Symbol};

pub struct Delegate<G: gaps::Scorer, S: symbols::Scorer> {
    pub gap: G,
    pub symbols: S,
}

impl<G: gaps::Scorer, S: symbols::Scorer> gaps::Scorer for Delegate<G, S> {
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

impl<G: gaps::Scorer, S: symbols::Scorer> symbols::EquivClassifier for Delegate<G, S> {
    #[inline(always)]
    fn classify(&self, s1: Symbol, s2: Symbol) -> symbols::EquivType {
        self.symbols.classify(s1, s2)
    }
}

impl<G: gaps::Scorer, S: symbols::Scorer> symbols::Scorer for Delegate<G, S> {
    #[inline(always)]
    fn score(&self, posa: usize, a: Symbol, posb: usize, b: Symbol) -> Score {
        self.symbols.score(posa, a, posb, b)
    }
}

impl<G: gaps::Scorer, S: symbols::Scorer> super::ScoringScheme for Delegate<G, S> {}
