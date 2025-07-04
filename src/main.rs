use clap::{Parser, ValueEnum};
use colored::Colorize;
use file_hasher::{hash_file, HashAlgorithm};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::Path;
use std::time::Duration;

#[derive(Parser)]
#[command(author, version, about = "A CLI tool to calculate multiple hash types for files", long_about = None)]
struct Args {
    /// Files to hash
    #[arg(required = true)]
    files: Vec<String>,

    /// Hash algorithm to use
    #[arg(short, long, value_enum, default_value = "sha256")]
    algorithm: Algorithm,

    /// Verify file against provided hash
    #[arg(short, long)]
    verify: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Algorithm {
    #[value(name = "sha256")]
    Sha256,
    #[value(name = "md5")]
    Md5,
    #[value(name = "sha1")]
    Sha1,
    #[value(name = "all")]
    All,
}

impl From<Algorithm> for HashAlgorithm {
    fn from(algo: Algorithm) -> Self {
        match algo {
            Algorithm::Sha256 => HashAlgorithm::SHA256,
            Algorithm::Md5 => HashAlgorithm::MD5,
            Algorithm::Sha1 => HashAlgorithm::SHA1,
            Algorithm::All => HashAlgorithm::SHA256, // Default for all
        }
    }
}

fn main() {
    let args = Args::parse();

    if let Some(expected_hash) = args.verify {
        if args.files.len() > 1 {
            eprintln!(
                "{}: Cannot verify multiple files against a single hash",
                "Error".red()
            );
            std::process::exit(1);
        }
        verify_file(&args.files[0], &expected_hash, args.algorithm);
    } else {
        for file in &args.files {
            hash_and_display(file, args.algorithm);
        }
    }
}

fn hash_and_display(file_path: &str, algorithm: Algorithm) {
    let path = Path::new(file_path);

    if !path.exists() {
        eprintln!("{}: File '{}' not found", "Error".red(), file_path);
        std::process::exit(1);
    }

    let file_size = match fs::metadata(path) {
        Ok(metadata) => metadata.len(),
        Err(e) => {
            eprintln!(
                "{}: Cannot read file metadata for '{}': {}",
                "Error".red(),
                file_path,
                e
            );
            return;
        }
    };

    let show_progress = file_size > 10 * 1024 * 1024; // Show progress for files > 10MB
    let pb = if show_progress {
        let pb = ProgressBar::new(file_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} {msg}")
                .unwrap()
                .progress_chars("##-"),
        );
        pb.set_message(format!(
            "Hashing {}",
            path.file_name().unwrap().to_string_lossy()
        ));
        pb.enable_steady_tick(Duration::from_millis(100));
        Some(pb)
    } else {
        None
    };

    println!("\n{} {}", "File:".bold(), file_path.cyan());

    if algorithm == Algorithm::All {
        let algorithms = vec![
            (Algorithm::Sha256, HashAlgorithm::SHA256),
            (Algorithm::Md5, HashAlgorithm::MD5),
            (Algorithm::Sha1, HashAlgorithm::SHA1),
        ];

        for (algo, hash_algo) in algorithms {
            match hash_file(path, hash_algo) {
                Ok(hash) => {
                    let algo_name = format!("{:?}", algo).to_uppercase();
                    println!("  {}: {}", algo_name.green().bold(), hash);
                }
                Err(e) => {
                    eprintln!("  {}: Failed to compute {:?} - {}", "Error".red(), algo, e);
                }
            }
        }
    } else {
        match hash_file(path, algorithm.into()) {
            Ok(hash) => {
                let algo_name = format!("{:?}", algorithm).to_uppercase();
                println!("  {}: {}", algo_name.green().bold(), hash);
            }
            Err(e) => {
                eprintln!("  {}: Failed to compute hash - {}", "Error".red(), e);
            }
        }
    }

    if let Some(pb) = pb {
        pb.finish_and_clear();
    }
}

fn verify_file(file_path: &str, expected_hash: &str, algorithm: Algorithm) {
    let path = Path::new(file_path);

    if !path.exists() {
        eprintln!("{}: File '{}' not found", "Error".red(), file_path);
        std::process::exit(1);
    }

    let hash_algo = if algorithm == Algorithm::All {
        // Try to detect hash type by length
        match expected_hash.len() {
            32 => HashAlgorithm::MD5,
            40 => HashAlgorithm::SHA1,
            64 => HashAlgorithm::SHA256,
            _ => {
                eprintln!(
                    "{}: Cannot determine hash algorithm from hash length",
                    "Error".red()
                );
                std::process::exit(1);
            }
        }
    } else {
        algorithm.into()
    };

    match hash_file(path, hash_algo) {
        Ok(computed_hash) => {
            if computed_hash.eq_ignore_ascii_case(expected_hash) {
                println!(
                    "{} Hash verification for '{}'",
                    "✓ MATCH:".green().bold(),
                    file_path
                );
                println!("  Expected: {}", expected_hash);
                println!("  Computed: {}", computed_hash);
            } else {
                println!(
                    "{} Hash verification for '{}'",
                    "✗ MISMATCH:".red().bold(),
                    file_path
                );
                println!("  Expected: {}", expected_hash);
                println!("  Computed: {}", computed_hash);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("{}: Failed to compute hash - {}", "Error".red(), e);
            std::process::exit(1);
        }
    }
}
