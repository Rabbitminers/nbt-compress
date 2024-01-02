use std::num::NonZeroU64;
use std::time::Instant;

use tracing::{warn, info};
use zopfli::Format::Gzip;
use clap::Parser;

#[derive(Parser, Debug)]
struct CompressionArgs {
    #[arg(help="The initial number of iterations to perform when compressing the data")]
    #[arg(long="min_iterations", default_value="100")]
    pub minimum_iterations: u64,

    #[arg(help="The final number of iterations to perform when compressing the data")]
    #[arg(long="max_iterations", default_value="500")]
    pub maximum_iterations: u64,

    #[arg(help="How many iterations to increase by each time until the maximum is reached")]
    #[arg(short, default_value="25")]
    pub step: u64,

    #[arg(help="The file to compress")]
    #[arg(short, long)]
    pub file: String,

    #[arg(help="The number of times to repeat each number of iterations")]
    #[arg(short, default_value="1")]
    pub repetitions: u32
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    #[cfg(debug_assertions)]
    warn!("running in debug mode, this may not be representive of realworld performance");

    let args = CompressionArgs::parse();

    let file = std::fs::read(&args.file)?;
    let mut iterations: u64 = args.minimum_iterations; 
    
    while iterations <= args.maximum_iterations {
        for _ in 0..args.repetitions {
            compress_data(&file, iterations)?;
        }
        iterations += args.step;
    }

    Ok(())
}

fn compress_data(file: &Vec<u8>, iterations: u64) -> anyhow::Result<()> {
    let options = zopfli::Options {
        iteration_count: NonZeroU64::new(iterations).unwrap(),
        ..Default::default()
    };

    let mut compressed = Vec::with_capacity(file.len());
    let start_time = Instant::now();

    zopfli::compress(options, Gzip, &file[..], &mut compressed)?;

    let elapsed = start_time.elapsed();
    compressed.shrink_to_fit();

    let original_size = file.len();
    let compressed_size = compressed.len();
    let saved_space = original_size - compressed_size;


    info!(%iterations, ?elapsed, %original_size, %compressed_size, %saved_space);

    Ok(())
}

