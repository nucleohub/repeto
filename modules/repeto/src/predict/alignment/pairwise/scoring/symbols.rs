pub use super::super::{Score, Symbol};

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum EquivType {
    Match,
    Mismatch,
    Equivalent,
}

pub trait EquivClassifier {
    fn classify(&self, s1: Symbol, s2: Symbol) -> EquivType;
}

pub trait Scorer: EquivClassifier {
    fn score(&self, seq1pos: usize, s1: Symbol, seq2pos: usize, s2: Symbol) -> Score;
}

pub trait PosInvariantScorer: EquivClassifier {
    fn score(&self, s1: Symbol, s2: Symbol) -> Score;
}

impl<T: PosInvariantScorer> Scorer for T {
    #[inline(always)]
    fn score(&self, _: usize, s1: Symbol, _: usize, s2: Symbol) -> Score {
        self.score(s1, s2)
    }
}

pub struct MatchMismatch {
    pub samesc: Score,
    pub diffsc: Score,
}

impl EquivClassifier for MatchMismatch {
    #[inline(always)]
    fn classify(&self, s1: Symbol, s2: Symbol) -> EquivType {
        match s1 == s2 {
            true => EquivType::Match,
            false => EquivType::Mismatch
        }
    }
}

impl PosInvariantScorer for MatchMismatch {
    #[inline(always)]
    fn score(&self, a: Symbol, b: Symbol) -> Score {
        if a == b { self.samesc } else { self.diffsc }
    }
}

pub struct Matrix<T> {
    alphsize: usize,
    scores: Vec<T>,
    subtypes: Vec<EquivType>,
}

impl<T: Into<Score> + Copy> Matrix<T> {
    pub fn new(alphsize: usize, matched: T, mismatched: T) -> Self {
        let mut scores = Vec::new();
        scores.resize(alphsize * alphsize, mismatched);
        let mut subtypes = Vec::new();
        subtypes.resize(alphsize * alphsize, EquivType::Mismatch);

        for i in 0..alphsize {
            scores[i * alphsize + i] = matched;
            subtypes[i * alphsize + i] = EquivType::Match;
        }

        Self {
            alphsize,
            scores,
            subtypes,
        }
    }

    pub fn set(&mut self, a: Symbol, b: Symbol, weight: T, subtype: EquivType) -> &mut Self {
        let ind = (a as usize) * self.alphsize + (b as usize);
        self.scores[ind] = weight;
        self.subtypes[ind] = subtype;
        self
    }
}

impl<T: Into<Score> + Copy> EquivClassifier for Matrix<T> {
    fn classify(&self, a: Symbol, b: Symbol) -> EquivType {
        let ind = (a as usize) * self.alphsize + (b as usize);
        self.subtypes[ind]
    }
}

impl<T: Into<Score> + Copy> PosInvariantScorer for Matrix<T> {
    #[inline(always)]
    fn score(&self, a: Symbol, b: Symbol) -> Score {
        let ind = (a as usize) * self.alphsize + (b as usize);
        self.scores[ind].into()
    }
}
