pub use super::super::{Score, Symbol};

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SubType {
    Match,
    Mismatch,
}

pub trait ScoringFunc {
    fn score(&self, seq1pos: usize, s1: Symbol, seq2pos: usize, s2: Symbol) -> (Score, SubType);
}

pub trait PosInvariantScoringFunc {
    fn score(&self, s1: Symbol, s2: Symbol) -> (Score, SubType);
}

impl<T: PosInvariantScoringFunc> ScoringFunc for T {
    #[inline(always)]
    fn score(&self, _: usize, s1: Symbol, _: usize, s2: Symbol) -> (Score, SubType) {
        self.score(s1, s2)
    }
}

pub struct MatchMismatch {
    pub samesc: Score,
    pub diffsc: Score,
}

impl PosInvariantScoringFunc for MatchMismatch {
    #[inline(always)]
    fn score(&self, a: Symbol, b: Symbol) -> (Score, SubType) {
        if a == b {
            (self.samesc, SubType::Match)
        } else {
            (self.diffsc, SubType::Mismatch)
        }
    }
}

// impl<F: Fn(Symbol, Symbol) -> Score> PosInvariantScoringFunc for F {
//     #[inline(always)]
//     fn score(&self, a: Symbol, b: Symbol) -> Score {
//         (self)(a, b)
//     }
// }

pub struct Matrix<T> {
    alphsize: usize,
    scores: Vec<T>,
    subtypes: Vec<SubType>,
}

impl<T: Into<Score> + Copy> Matrix<T> {
    pub fn new(alphsize: usize, matched: T, mismatched: T) -> Self {
        let mut scores = Vec::new();
        scores.resize(alphsize * alphsize, mismatched);
        let mut subtypes = Vec::new();
        subtypes.resize(alphsize * alphsize, SubType::Mismatch);

        for i in 0..alphsize {
            scores[i * alphsize + i] = matched;
            subtypes[i * alphsize + i] = SubType::Match;
        }

        Self {
            alphsize,
            scores,
            subtypes,
        }
    }

    pub fn set(&mut self, a: Symbol, b: Symbol, weight: T, subtype: SubType) -> &mut Self {
        let ind = (a as usize) * self.alphsize + (b as usize);
        self.scores[ind] = weight;
        self.subtypes[ind] = subtype;
        self
    }
}

impl<T: Into<Score> + Copy> PosInvariantScoringFunc for Matrix<T> {
    #[inline(always)]
    fn score(&self, a: Symbol, b: Symbol) -> (Score, SubType) {
        let ind = (a as usize) * self.alphsize + (b as usize);
        (self.scores[ind].into(), self.subtypes[ind])
    }
}
