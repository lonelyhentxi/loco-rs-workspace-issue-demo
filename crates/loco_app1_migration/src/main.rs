use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    cli::run_cli(loco_app1_migration::Migrator).await;
}
