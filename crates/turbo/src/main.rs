use app::App;
use tracing::info;

mod app;
mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    info!("Starting application");
    let mut app = App::new();
    info!("Parsing command line arguments done");
    app.run().await;
    info!("Application started");
    Ok(())
}
