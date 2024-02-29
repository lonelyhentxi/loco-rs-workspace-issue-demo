use loco_rs::cli;
use loco_app1::app::App;
use loco_app1_migration::Migrator;
use share_lib::A;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    println!("share-lib A={}", A);
    cli::main::<App, Migrator>().await
}
