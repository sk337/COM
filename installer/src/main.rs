use clap::Parser;

mod args;
mod utils;

#[tokio::main]
/// The main function for the installer
async fn main() {
    let args = args::Args::parse();
    let install_path = args
        .install_path
        .unwrap_or_else(|| utils::get_default_installation_path());

    println!("Installing to: {:?}", install_path);

    let octocrab = octocrab::Octocrab::builder()
        .build()
        .expect("Failed to create Octocrab client");
    let repo = octocrab.repos("sk337", "DOSDisassm");
    let releases = repo
        .releases()
        .list()
        .await
        .expect("Failed to get releases");
    let latest_release = releases
        .into_iter()
        .find(|release| release.tag_name == "latest")
        .expect("Failed to find latest release");
    let assets = latest_release.assets;
}
