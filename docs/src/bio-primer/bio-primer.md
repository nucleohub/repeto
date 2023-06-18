# Biological primer

DNA, the essential carrier of genetic information in living cells, exhibits a surprisingly high degree of repetition,
especially in eukaryotic genomes. In model organisms such as humans and mice, repeats of various origins constitute
approximately 50% of the genome. The fraction of repeats is even higher in plants, particularly maize. However, the
function, if any, of these repetitive sequences remains elusive and is likely to be context-specific.

![Repeat types](../intro/rep-types.svg)

Genomic repeats are mathematically simple entities: sequences that occur two or more times, possibly with inaccuracies,
across a set of chromosomes. These can be further classified based on their localization and ordering. Tandem repeats,
such as dinucleotide repeats, are positioned directly adjacent to each other in the genome, while interspersed repeats,
such as retrotransposons, SINEs, and LINEs, occupy multiple locations in the genome. Direct repeats occur in the same
direction, while inverted repeats (IRs) have their sequence followed by its reverse complement.

Although the above definitions are suitable for algorithmic purposes, they contradict the established understanding of
repetitive sequences. Sequences such as pseudogenes, TATA boxes, and transcription factor (TF) sequence-specific binding
sites, although they meet the definition of repeats, are not typically referred to as such. Conversely, sequences such
as dinucleotide repeats (e.g. (CA)n or (AT)n) and transposable elements (e.g. Alu or LINE) are often regarded as the
most repetitive sequences. Thus, the demarcation line of what constitutes a "biological repeat" is flexible and
context-dependent.

This primer focuses on inverted repeats and adheres to the mathematical definition of inverted repeats while 
highlighting additional constraints and details of sequence origins where appropriate.

## DNA: cruciforms

Cruciforms are formed when the double-stranded DNA (dsDNA) containing an IR partially unwinds, causing the complementary
repeats to anneal to each other, forming a four-stranded structure. The resulting cruciform is stabilized by both base
pairing between IR segments and by the formation of loops that might be further stabilized by protein binding. These
structures can have various biological consequences, such as the modulation of gene expression or creation of genomic
instability.

## RNA: hairpins

