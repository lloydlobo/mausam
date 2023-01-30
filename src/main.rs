#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    match mausam::run().await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{:#?}", e);
            std::process::exit(1)
        }
    };
    Ok(())
}
