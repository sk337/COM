use obfstr::obfstr as s;
use octocrab::Octocrab;
use semver::Version;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let octocrab = Octocrab::builder()
        .personal_token(s!("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"))
        .build()?;

    let repo = octocrab.repos(s!("sk337"), s!("COM"));

    let latest_release = repo.releases().get_latest().await?;

    let tag = latest_release.tag_name.strip_prefix("v").unwrap();

    let latest_version = Version::from_str(tag);

    let latest_version = latest_version.unwrap_or_else(|_| {
        panic!(
            "Failed to parse version from tag name: {}",
            latest_release.tag_name
        )
    });

    let current_version = Version::from_str(env!("CARGO_PKG_VERSION")).unwrap_or_else(|_| {
        panic!(
            "Failed to parse current version: {}",
            env!("CARGO_PKG_VERSION")
        )
    });

    let arch = std::env::consts::ARCH;

    println!("Arch: {}", arch);

    if latest_version <= current_version {
        println!("Already up to date");
        return Ok(());
    }
    println!("Updating to {}", latest_version);

    Ok(())
}
