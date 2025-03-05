//! kmer stats

#![warn(missing_docs)]


/* mod declarations */
pub mod cli;
pub mod count;
pub mod file_parsing;


/// Extract sequences that contain some kmers
///
/// Also output the kmers that occur in the reads with their number of occurrences.
#[allow(clippy::too_many_arguments)]
pub fn kmer_stats(
    in_fasta_reads: String,
    kmer_size: usize,
) -> anyhow::Result<()> {
    // check that in_fasta_reads is a non empty file if it exists:
    if !in_fasta_reads.is_empty() {
        cli::validate_non_empty_file(in_fasta_reads.clone())?;
    }

    let (total_sequences, total_kmer, total_nucleotides) = count::stats_fastx_file_par(in_fasta_reads, kmer_size)?;
    println!("Number of nucleotides seen {}",
            total_nucleotides
            );
    println!("Number of sequences seen {}",
            total_sequences
            );
    println!("Number of kmer seen {}",
            total_kmer
            );
    Ok(())
}
