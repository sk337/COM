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
    let repo = octocrab.repos("sk337", "COM");
    let releases = repo.releases().list().send().await.unwrap();
    let tag_names = releases
        .into_iter()
        .filter_map(|release| Some(release.tag_name.clone()))
        .collect::<Vec<_>>();
    let latest_release = tag_names
        .iter()
        .filter_map(|tag| {
            if tag.starts_with('v') {
                // Remove the 'v' and parse the version
                semver::Version::parse(&tag[1..])
                    .ok()
                    .map(|version| (version, tag))
            } else {
                None
            }
        })
        .max_by(|(version_a, _), (version_b, _)| version_a.cmp(version_b))
        .map(|(_, tag)| tag.clone())
        .unwrap_or_else(|| "v0.0.0".to_string());
    println!("Latest release: {}", latest_release);

    let release = repo
        .releases()
        .get_by_tag(&latest_release)
        .await
        .expect("Failed to get release");
    let assets = release.assets;
    for asset in &assets {
        println!("Asset: {}", asset.name);
    }
}
