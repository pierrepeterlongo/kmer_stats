//! kmer stats

/* std use */

use std::env;

/* crates use */
use clap::Parser as _;

/* project use */
use kmer_stats::cli::Args;
use kmer_stats::count::stats_fastx_file_par;

///////////////////////// MAIN /////////////////////////

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Set the number of threads for rayon
    // If the number of threads is not set, rayon will use the number of logical cores
    env::set_var("RAYON_NUM_THREADS", args.threads.to_string());

    let (total_sequences, total_kmer, total_nucleotides) = stats_fastx_file_par(args.in_sequences, args.kmer_size)?;
    println!("#nucleotides: {}",
            total_nucleotides
            );
    println!("#sequences: {}",
            total_sequences
            );
    println!("#{}-mer:  {}",  args.kmer_size, total_kmer);
    Ok(())
}
