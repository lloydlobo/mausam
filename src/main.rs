// $ RUST_BACKTRACE=1 mausam
// $ CARGO_LOG=trace cargo run

// FIXME: Docker container has .env file. Remove it before pushing ot docker hub

// TODO:
// ./mausam: error while loading shared libraries: libssl.so.1
// .1: cannot open shared object file: No such file or directory

mod app;
mod cli;
mod models;

use anyhow::{anyhow, Context};

// https://github.com/mitchmindtree/plutchik/blob/master/examples/test.rs
// use plutchik::{Emotion, EMOTIONS, Wheel};
// assert!(Wheel::mean(&[Serenity, Acceptance, Joy, Trust]).closest_emotion() == Love);
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    match app::run().await {
        Ok(v) => Ok(println!("{}", serde_json::to_string_pretty(&v)?)),
        // TODO: Parse result via clap in lib::run()
        Err(e) => {
            let context = anyhow::format_err!(
                "Failed to run at `{}`: `{:#?}`",
                std::env::current_dir()?
                    .to_str()
                    .context(anyhow!("Failed to find current_dir.\n>> Trace: {:#?}", e))
                    .unwrap_err(),
                e,
            );
            eprintln!("{:#?}", e.context(context));
            std::process::exit(1)
        }
    }
}
