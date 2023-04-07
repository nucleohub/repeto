# Python tutorial

Before you start, make sure you've installed the repeto Python bindings. You can find installation
instructions [here](../install.md).

**repeto** is currently focused only on inverted DNA/RNA repeats (IRs), and there are two main functions you'll need to
know to get started: `repeto.predict` and `repeto.optimize`. `repeto.predict` is used to find IRs in a given sequence,
while `repeto.optimize` uses user-supplied scores to derive an optimal matching for the IRs (i.e., a valid RNA secondary
structure). You can find more details about the algorithm [here](../algorithm.md).

Here's a basic usage example:

```python
{{#include basic-usage.py}}
```

In this example, `sequence` is the target DNA/RNA sequence, and `min_score` and `min_matches_run` are thresholds for IRs
to report. The `scores` list is a user-supplied list of scores to use for optimizing the IRs.

## Realistic Use Case

Let's say we're studying an RNA-binding protein and want to check if its binding is co-localized with putative long
double-stranded RNAs (dsRNAs) formed by inverted repeats. Our colleagues have already performed an experiment, mapped
all
binding sites to the human genome, and provided us with a BED file containing their (stranded) coordinates. Our goal is
to
check for co-localization.

To do this, we will use several additional Python libraries that can be installed via pip:

```shell
python -m pip install pybedtools, intervaltree, joblib
```

### Group binding sites

First, we need to group nearby binding sites together. This is done to speed up the downstream analysis
and avoid double processing of sequences derived from nearby binding sites:

```python
{{#include real-use-case.py:group-binding-sites}}
```

In this code, MAX_DISTANCE is the maximum distance between two arms of a dsRNA. Feel free to adjust this value,
but keep in mind that very long IRs (>5 kbp) can fold back to form a dsRNA on a distance as long as 50 kbp.

### Fetch sequences

After grouping nearby binding sites, the next step is to fetch the genome sequences:

```python
{{#include real-use-case.py:fetch-sequences}}
```

### Predict & Optimize inverted repeats

Next, we'll search for inverted repeats that could have formed double-stranded RNAs(>=50bp) sensed by the protein of
interest:

```python
{{#include real-use-case.py:inverted-repeats}}
```

The code example above is the longest because **repeto** makes **you** responsible for creating meaningful scores. This
makes it as flexible as possible and allows you to explore any exotic weighing options.

### Final results

Getting back to the original question, let's check the number of binding sites explained by IRs:

```python
{{#include real-use-case.py:final-results}}
```

### Concluding remarks

Although the above example is a little bit synthetic, it should provide you with a solid ground on how to use repeto
in a real world project. Good luck!
