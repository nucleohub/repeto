# Introduction

**Repeto** is a bioinformatics library designed to assist researchers in studying inverted repeats (IRs) in DNA/RNA
sequences. Its main features include:

- *Flexible IR prediction*.
- *Optimization of IR matching*. For RNA, the optimized set of IRs can be converted into a correct secondary
  structure.
- *High-performance & ease of use*. Library core is implemented in Rust and provides typed and documented
  Python bindings.

### IR notation

<img src="introduction/IR-notation.svg" width=540 alt="IR notation">


Throughout the library, we use the following notation to describe IRs: 
- Each IR consists of one or more segments, numbered from outer to inner. 
- Each segment represents a contiguous sequence and its inverse complement downstream.  
- Segments may or may not be separated by gaps, depending on the parameters used to find IRs in the sequence.  

Note that there is always at least one gap of length >= 0 that separates upstream sequences (left IR blocks) from their
inverted reverse complements downstream (right IR blocks).

### Repeto workflow

<img src="introduction/workflow.svg" style="float: right" width=280 alt="Repeto workflow">

The Repeto workflow consists of three main steps:

1. Predicting IRs from the input sequence using user-defined thresholds.
2. Scoring predicted IRs based on, for example, experimental data or biological significance.
3. Optimizing IR matching to derive a non-ambiguous set of IRs with a maximum cumulative score.

See installation instructions and Python tutorial to get started.

### Citations

If you use Repeto in your research, please cite the following paper: TODO.
