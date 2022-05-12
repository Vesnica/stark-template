// Copyright Vesnica
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use std::io::Write;
use std::time::Instant;

use log::debug;
use winter_prover::StarkProof;
use winter_verifier::verify;

pub mod air;
use air::{from_data, Data, FreshAir};

use clap::Parser;

#[derive(Parser)]
#[clap(name = "verifier", author, version, about, long_about = None)]
struct Cli {
    #[clap(long, short, display_order = 1, default_value_t = String::from("./stark.toml"))]
    proof_file_path: String,
}

fn main() {
    env_logger::Builder::new()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter_level(log::LevelFilter::Debug)
        .init();

    let cli = Cli::parse();

    let data: Data = confy::load_path(cli.proof_file_path).unwrap();
    let (pub_inputs, proof_bytes) = from_data(data);
    let proof = StarkProof::from_bytes(&proof_bytes).unwrap();
    let now = Instant::now();
    match verify::<FreshAir>(proof, pub_inputs) {
        Ok(_) => debug!(
            "Proof verified in {:.1} ms",
            now.elapsed().as_micros() as f64 / 1000f64
        ),
        Err(msg) => debug!("Failed to verify proof: {}", msg),
    }
}
