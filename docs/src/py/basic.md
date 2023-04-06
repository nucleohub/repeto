# Python tutorial

Before you start, make sure you've installed the repeto Python bindings. You can find installation
instructions [here](../install.md).

**repeto** is currently focused only on inverted DNA/RNA repeats (IRs), and there are two main functions you'll need to
know to get started: `repeto.predict` and `repeto.optimize`. `repeto.predict` is used to find IRs in a given sequence,
while `repeto.optimize` uses user-supplied scores to derive an optimal matching for the IRs (i.e., a valid RNA secondary
structure). You can find more details about the algorithm [here](../algorithm.md).

Here's a basic usage example:

```python
import repeto

# Predict inverted repeats in the given sequence
sequence = "AAAAATTTTTAAAAATTTT"
ir = repeto.predict(
    sequence.encode("ASCII"),  # Target DNA/RNA sequence 
    # IRs' thresholds
    min_score=5,  # Minimum alignment score
    min_matches_run=2,  # Minimum number of consecutive base pairs (e.g., min dsRNA/cruciform stem length)
)

# Optimize inverted repeats matching
scores = [1, 2, ...]
assert len(scores) == len(sequence)
ir = repeto.optimize(
    ir,  # Inverted repeats to optimize
    scores,  # User-supplied scores
)
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
import pybedtools
from pybedtools import BedTool, Interval

MAX_ARMS_DISTANCE = 10_000  # hard-max distance between two arms of a dsRNA
SITES = "binding_sites.bed"  # file with binding sites

# Group sites within MAX_ARMS_GAP proximity
chrominfo = pybedtools.chromsizes("hg38")  # Use your assembly, see pybedtools doc for details
groups = BedTool(SITES)
.set_chromsizes(chrominfo)
.slop(MAX_ARMS_DISTANCE)
.sort()
.merge(s=True, c=6, o="distinct")

# fix the strand column
groups = BedTool([Interval(x.chrom, x.start, x.end, strand=x.fields[3]) for x in groups]).sort()
```

In this code, MAX_DISTANCE is the maximum distance between two arms of a dsRNA. Feel free to adjust this value,
but keep in mind that very long IRs (>5 kbp) can fold back to form a dsRNA on a distance as long as 50 kbp.

### Fetch sequences

After grouping nearby binding sites, the next step is to fetch the genome sequences:

```python
from Bio import SeqIO

# Fetch fasta
assembly = "assembly.fa"  # path to an indexed genome assembly 
fasta = groups.sequence(fi=assembly, s=True).seqfn

# Parse sequences & preprocess
sequences = []
for group, seq in zip(groups, SeqIO.parse(fasta, format='fasta')):
    seq = str(seq.seq).upper()

    # Flip reverse complemented sequences for easier coordinates mapping later
    if group.strand == "-":
        seq = seq[::-1]

    sequences.append((group, seq))
```

### Predict & Optimize inverted repeats

Next, we'll search for inverted repeats that could have formed double-stranded RNAs(>=50bp) sensed by the protein of
interest:

```python
from collections import defaultdict
from typing import List, Tuple

import intervaltree
import repeto
from joblib import Parallel, delayed

MIN_DSRNA_SIZE = 50

# Build a simple genomic index of binding sites to filter predicted IRs
INDEX = defaultdict(lambda *args: intervaltree.IntervalTree())
for site in BedTool(SITES):
    INDEX[(site.chrom, site.strand)].addi(site.start, site.end, site)


# Function to predict IRs and "optimize" them
def job(interval: Interval, sequence: str) -> Tuple[Interval, List[repeto.InvertedRepeat]]:
    # Predict all IRs in the given sequence
    irs, alnscores = repeto.predict(sequence.encode("ASCII"), min_score=MIN_DSRNA_SIZE, min_matches_run=MIN_DSRNA_SIZE)

    # Remove IRs that don't overlap binding sites on both "sides" (dsRNA stem arms)
    # Calculate optimization score as the alignment score + (number of bases overlapping binding sites) * 2
    filtered, index = [], INDEX[(interval.chrom, interval.strand)]
    for ir, score in zip(irs, alnscores):
        # Skip IRs with too large gap
        maxgap = max(segment.right.start - segment.left.end for segment in ir.segments)
        if maxgap >= MAX_ARMS_DISTANCE:
            continue

        replicated = False
        for segment in ir.segments:
            # Calculate the number of base pairs that overlap with binding sites on each arm
            arm_overlaps = []
            for arm in segment.left, segment.right:
                overlap_size = [
                    x.overlap_size(arm.left.start, arm.left.end) for x in index.overlap(arm.left.start, arm.left.end)
                ]
                arm_overlaps.append([size for size in overlap_size if size >= MIN_DSRNA_SIZE])
            left, right = arm_overlaps

            # Keep the IR if at least some segment (dsRNA stem) overlaps with peaks on both sides (arms)
            replicated = replicated or len(left) > 0 and len(right) > 0

            # Update the score based on the number of overlapping base pairs
            score += sum(left) * 2 + sum(right) * 2
        if replicated:
            filtered.append((ir, score))

    # Find the optimal combination of inverted repeats (=data-supported dsRNAs)
    # Maximize the alignment score and the total length of binding sites explained by the IR * 2
    irs, scores = zip(*filtered)
    irs, _ = repeto.optimize(irs, scores)

    # Transform IR coordinates from sequence-based to genome-based
    for ir in irs:
        ir.shift(interval.start)

    return (interval.chrom, interval.strand, irs)


# Apply the function to all sequences in parallel
results = Parallel()(delayed(job)(i, s) for i, s in sequences)
```

The code example above is the longest because **repeto** makes **you** responsible for creating meaningful scores. This
makes it as flexible as possible and allows you to explore any exotic weighing options.

### Final results

Getting back to the original question, let's check the number of binding sites explained by IRs:

```python
# First let's save repeto results as BED12 file. 
# This file can be also used to visualize results in a genome browser later
saveto = "inverted-repeats.bed"
with open(saveto) as stream:
    for contig, strand, irs in results:
        for ir in irs:
            stream.write(f"{ir.to_bed12(contig=contig, strand=strand)}\n")

irs = BedTool(saveto).sort().saveas(saveto)

# How many binding sites were explained by our putative dsRNAs?
sites = BedTool(SITES)
total_sites = len(sites)
intersection = sites.sort().intersect(irs, wa=True, u=True, split=True, s=True)
intersection = len(intersection)

print(f"Total explained binding sites: {intersection} ({intersection / total_sites * 100: .2f}%)")
```

### Concluding remarks

Although the above example is a little bit synthetic, it should provide you with a solid ground on how to use repeto
in a real world project. Good luck!
