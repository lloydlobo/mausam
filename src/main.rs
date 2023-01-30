// $ RUST_BACKTRACE=1 mausam
// $ CARGO_LOG=trace cargo run

use anyhow::{anyhow, Context};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    match mausam::run().await {
        Ok(v) => Ok(println!("{}", serde_json::to_string_pretty(&v)?)),
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
