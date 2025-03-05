//! Define Command Line Interface

/* std use */

/* crates use */
use clap::Parser;

/* project use */

/// Basical stats from fasta/fastq[.gz] file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Input fasta or fastq [.gz|zst] file
    #[arg(long, verbatim_doc_comment)]
    pub in_sequences: String,

    /// Size of the kmers to index and search
    #[arg(short, long, default_value_t = 31)]
    pub kmer_size: usize,

    /// Number of threads
    ///    Note: if not provided, the number of threads is set to the number of logical cores
    #[arg(short, long, default_value_t = 0, verbatim_doc_comment)]
    pub threads: usize,
}

/// check that a file name corresponds to a non empty file:
pub fn validate_non_empty_file(in_file: String) -> anyhow::Result<()> {
    if let Ok(metadata) = std::fs::metadata(in_file.clone()) {
        // Check if the file exists
        if !metadata.is_file() {
            anyhow::bail!("{:#} exists, but it's not a file.", in_file)
        }
    } else {
        anyhow::bail!("{:#} is not a file.", in_file)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_empty_file_test() -> anyhow::Result<()> {
        let temp_dir = tempfile::tempdir()?;
        let directory = temp_dir.into_path();
        let file = directory.join("empty.fasta");

        // test directory
        assert!(
            validate_non_empty_file(directory.clone().into_os_string().into_string().unwrap())
                .is_err()
        );

        // test not exist
        assert!(
            validate_non_empty_file(file.clone().into_os_string().into_string().unwrap()).is_err()
        );

        // test work
        std::fs::File::create(&file)?;
        assert!(
            validate_non_empty_file(file.clone().into_os_string().into_string().unwrap()).is_ok()
        );

        Ok(())
    }
}
