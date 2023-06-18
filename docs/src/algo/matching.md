# Matching optimization

After prediction, it is common to end up with many formally correct inverted repeats that may not be biologically
relevant. Typically, the original predictions need to be filtered based on prior knowledge or experimental data to
select only "interesting" inverted repeats for downstream analysis. However, it can be challenging to choose the most
appropriate set of inverted repeats when a sequence can be matched to many other inverted reverse complements
downstream, and there are many overlapping combinations. That's where matching optimization comes in, to help derive a
non-ambiguous combination of inverted repeats with an "optimal score."

The interesting part is that you define meaningful scores for each predicted inverted repeat. These scores can be based
on biological data or a combination of prior knowledge and the length of IRs. Repeto doesn't care about the scores and
tries to derive the best match based on what you consider important.

## Problem formulation

The optimization procedure starts with a set of IRs and associated scores. The goal is to find a 'valid' combination of
inverted repeats that has a maximum score. By 'valid,' we mean that the following combinations are prohibited:

* Overlapping IR segments: Each nucleotide must belong to one and only one inverted repeat.
* Crossing IR segments: If we imagine IR matches as arcs, then there must be no 'crosses.'

However, 'nesting' IRs into gaps of other IRs (including those between segments) is allowed.

For simplicity, we omit strict limits and mathematical formulation here as they have little value for the discussion,
but they are fairly easy to derive if needed.

### Optimal IRs Matching = Valid RNA Secondary Structure

Given the above formulation, we can highlight one important property of the optimal IRs matching - it can be converted
into a valid RNA secondary structure. Of course, such a structure is very coarse-grained and is only useful to find
double-stranded RNA segments (= inverted repeats) that are truly supported by the user-defined scores.


## Solution pseudocode
TODO





