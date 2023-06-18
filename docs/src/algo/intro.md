# Algorithmic details

This section provides details about the algorithms that form the core of **Repeto**. In addition to providing ideas and
pseudocode, we also include complexity estimations and references where applicable.

In short, repeat prediction is based on all-local alignment of sequences to themselves, while matching optimization is
treated as a dynamic programming problem that aims to optimize the total user-supplied score.

GLOSSARY