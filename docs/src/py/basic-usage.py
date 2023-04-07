import repeto

# Predict inverted repeats in the given sequence
sequence = "AAAAATTTTTAAAAATTTT"
irs, alignment_scores = repeto.predict(
    sequence.encode("ASCII"),  # Target DNA/RNA sequence
    # IRs' thresholds
    min_score=5,  # Minimum alignment score
    min_matches_run=2,  # Minimum number of consecutive base pairs (e.g., min dsRNA/cruciform stem length)
)

# Customize repeats scores
scores = [x ** 2 for x in alignment_scores]

# Optimize inverted repeats matching
irs, total_score = repeto.optimize(irs, scores)
