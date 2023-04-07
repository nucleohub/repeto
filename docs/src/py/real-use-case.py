# ANCHOR: group-binding-sites
import pybedtools
import os
from pybedtools import BedTool, Interval

MAX_ARMS_DISTANCE = 5_000  # hard-max distance between two arms of a dsRNA
SITES = "data/binding-sites.bed"  # file with binding sites
ASSEMBLY = "data/assembly.fa"  # indexed genome fasta

# Use pybedtools builtin chromsizes
# chrominfo = pybedtools.chromsizes("hg38")

# Or use fasta index to calculate them on the fly
assert os.path.exists(ASSEMBLY + ".fai"), "Use samtools faidx to index genome fasta"
with open(ASSEMBLY + ".fai") as stream:
    lines = [x.split('\t') for x in stream.readlines()]
    chrominfo = {contig: (0, length) for contig, length, *_ in lines}

# Group sites within MAX_ARMS_GAP proximity
groups = BedTool(SITES) \
    .set_chromsizes(chrominfo) \
    .slop(b=MAX_ARMS_DISTANCE) \
    .sort() \
    .merge(s=True, c=6, o="distinct")

# fix the strand column
groups = BedTool([Interval(x.chrom, x.start, x.end, strand=x.fields[3]) for x in groups]).sort()
# ANCHOR_END: group-binding-sites

# ANCHOR: fetch-sequences
from Bio import SeqIO

# Fetch fasta
fasta = groups.sequence(fi=ASSEMBLY, s=True).seqfn

# Parse sequences & preprocess
sequences = []
for group, seq in zip(groups, SeqIO.parse(fasta, format='fasta')):
    seq = str(seq.seq).upper()

    # Flip reverse complemented sequences for easier coordinates mapping later
    if group.strand == "-":
        seq = seq[::-1]

    sequences.append((group, seq))
# ANCHOR_END: fetch-sequences

# ANCHOR: inverted-repeats
from collections import defaultdict
from typing import List, Tuple

import intervaltree
import repeto
from joblib import Parallel, delayed

MIN_DSRNA_SIZE = 30

# Build a simple genomic index of binding sites to filter predicted IRs
INDEX = defaultdict(lambda *args: intervaltree.IntervalTree())
for site in BedTool(SITES):
    INDEX[(site.chrom, site.strand)].addi(site.start, site.end, site)


# Function to predict IRs and "optimize" them
def job(interval: Interval, sequence: str) -> Tuple[Interval, str, List[repeto.InvertedRepeat]]:
    # Predict all IRs in the given sequence
    irs, alnscores = repeto.predict(sequence.encode("ASCII"), min_score=MIN_DSRNA_SIZE, min_matches_run=MIN_DSRNA_SIZE)

    # Transform IR coordinates from sequence-based to genome-based
    for ir in irs:
        ir.shift(interval.start)

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
            # Calculate the number of base pairs in runs with length >= MIN_DSRNA_SIZE
            # that overlap with binding sites for each stem arm
            arm_overlaps = []
            for arm in segment.left, segment.right:
                overlap_size = [x.overlap_size(arm.start, arm.end) for x in index.overlap(arm.start, arm.end)]
                arm_overlaps.append([size for size in overlap_size if size >= MIN_DSRNA_SIZE])
            left, right = arm_overlaps

            # Keep the IR if at least some segment (dsRNA stem) overlaps with peaks on both sides (arms)
            replicated = replicated or len(left) > 0 and len(right) > 0

            # Update the score based on the number of overlapping base pairs
            score += sum(left) * 2 + sum(right) * 2
        if replicated:
            filtered.append((ir, score))

    if len(filtered) == 0:
        return interval, sequence, []

    # Find the optimal combination of inverted repeats (=data-supported dsRNAs)
    # Maximize the alignment score and the total length of binding sites explained by the IR * 2
    irs, scores = zip(*filtered)
    irs, _ = repeto.optimize(irs, scores)
    return interval, sequence, irs


# Apply the function to all sequences in parallel
results = Parallel(n_jobs=-1)(delayed(job)(i, s) for i, s in sequences)
results = [x for x in results if len(x[2]) > 0]  # drop sequences without matched IRs
# ANCHOR_END: inverted-repeats


# ANCHOR: final-results
# First let's save repeto results as BED12 file.
# This file can be also used to visualize results in a genome browser later
saveto = "inverted-repeats.bed"
with open(saveto, 'w') as stream:
    for window, _, irs in results:
        for ir in irs:
            stream.write(f"{ir.to_bed12(contig=window.chrom, strand=window.strand)}\n")

irs = BedTool(saveto).sort().saveas(saveto)

# How many binding sites were explained by our putative dsRNAs?
sites = BedTool(SITES)
total_sites = len(sites)
intersection = sites.sort().intersect(irs, wa=True, u=True, split=True, s=True)
intersection = len(intersection)

print(f"Total explained binding sites: {intersection} ({intersection / total_sites * 100: .2f}%)")
# ANCHOR_END: final-results
