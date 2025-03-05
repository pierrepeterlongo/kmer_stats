# kmer_stats

From a fasta/fastq[.gz] file to extremely basical statistics: 
- number of nucleotides
- number of sequences
- number of kmers (all, not distinct) = Sum(size of sequence - k + 1)

Parallelized, it should be faster than any tool including more features.

# Install
```bash
git clone https://github.com/pierrepeterlongo/kmer_stats.git
cd kmer_stats
RUSTFLAGS="-C target-cpu=native" cargo install --path .
```

Rust has to be installed locally. If needed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

# Usage example
```
kmer_stats --in-sequences tiny_test/in.fa                     
#nucleotides: 500
#sequences: 5
#31-mer:  350
```

```
 kmer_stats --in-sequences tiny_test/in.fa --only-numbers
500
5
350
```

```
kmer_stats --in-sequences tiny_test/in.fa -k 2
#nucleotides: 500
#sequences: 5
#2-mer:  495
```

