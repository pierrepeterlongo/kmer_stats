//! Count declarations

// use std::sync::Mutex;

/* crates use */
use anyhow::Context as _;
use fxread::initialize_reader;
use rayon::prelude::*;



/// for a given fasta file, count 
/// the number of sequences, the number of kmers, the number of nucleotides
#[allow(clippy::too_many_arguments)]
pub fn stats_fastx_file_par(
    file_name: String,
    kmer_size: usize,
) -> anyhow::Result<(usize, usize, usize)>
{
    const CHUNK_SIZE: usize = 32; // number of records
    const INPUT_CHANNEL_SIZE: usize = 8; // in units of CHUNK_SIZE records

    struct Chunk {
        records: Vec<usize>, // leb seq
        // records: Vec<(usize, fxread::Record, Box<dyn MatchedSequence>)>,
    }

    let (input_tx, input_rx) = std::sync::mpsc::sync_channel::<Chunk>(INPUT_CHANNEL_SIZE);
    

   
    let reader_thread = std::thread::spawn(move || -> anyhow::Result<()> {
        let mut reader = initialize_reader(&file_name).unwrap();
        for _id in 0.. {
            let mut vec = Vec::with_capacity(CHUNK_SIZE);
            for _ in 0..CHUNK_SIZE {
                match reader.next_record()? {
                    None => break,
                    Some(record) => vec.push(record.seq().len()),
                }
            }
            if vec.is_empty() || input_tx.send(Chunk {records: vec }).is_err() {
                return Ok(());
            }
        }
        unreachable!()
    });

    

    let (total_sequences, total_kmer, total_nucleotides) = input_rx
        .into_iter()
        .par_bridge()
        .map(move |mut chunk| {
            let mut tt_kmer = 0; // total number of kmers
            let mut tt_sequences = 0;  // total number of sequences
            let mut tt_nt = 0; // total number of nucleotides

            for seq_len in &mut chunk.records {
                tt_nt += *seq_len;
                tt_kmer += *seq_len - kmer_size + 1;
                tt_sequences += 1;
            }
            (tt_sequences, tt_kmer, tt_nt)
        })
        .reduce(|| (0, 0, 0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.2));

    reader_thread
        .join()
        .unwrap()
        .context("Error reading the sequences")?;
    

    Ok((total_sequences, total_kmer, total_nucleotides))
}


#[cfg(test)]
mod tests {
    /* crate use */

}
