//! Self-update against GitHub Releases (same assets as `install.sh`).

use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Context, Result};
use sha2::{Digest, Sha256};

const BINARY: &str = "f00";
const RELEASES: &str = "https://github.com/theesfeld/f00/releases";
const API_LATEST: &str = "https://api.github.com/repos/theesfeld/f00/releases/latest";

/// Current package version from Cargo.
pub fn current_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Target triple for the running binary.
pub fn host_target() -> Result<&'static str> {
    // Keep in sync with install.sh `rust_target`.
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        return Ok("x86_64-unknown-linux-gnu");
    }
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        return Ok("aarch64-unknown-linux-gnu");
    }
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    {
        return Ok("x86_64-apple-darwin");
    }
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        return Ok("aarch64-apple-darwin");
    }
    #[cfg(all(target_os = "freebsd", target_arch = "x86_64"))]
    {
        return Ok("x86_64-unknown-freebsd");
    }
    #[cfg(all(target_os = "freebsd", target_arch = "aarch64"))]
    {
        return Ok("aarch64-unknown-freebsd");
    }
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        return Ok("x86_64-pc-windows-msvc");
    }
    #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
    {
        return Ok("aarch64-pc-windows-msvc");
    }
    #[allow(unreachable_code)]
    Err(anyhow!(
        "unsupported platform for self-update; install manually from {}",
        RELEASES
    ))
}

fn agent() -> ureq::Agent {
    ureq::AgentBuilder::new()
        .user_agent(concat!("f00/", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(60))
        .build()
}

#[derive(Debug, Clone)]
pub struct ReleaseInfo {
    pub tag: String,
    pub version: String,
}

/// Fetch latest stable release tag from GitHub Releases API.
pub fn latest_release() -> Result<ReleaseInfo> {
    let agent = agent();
    let resp = agent
        .get(API_LATEST)
        .set("Accept", "application/vnd.github+json")
        .call()
        .context("fetch latest release from GitHub")?;
    let body: serde_json::Value = resp.into_json().context("parse releases JSON")?;
    let tag = body
        .get("tag_name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("releases API missing tag_name"))?
        .to_string();
    let version = tag.trim_start_matches('v').to_string();
    Ok(ReleaseInfo { tag, version })
}

/// Compare dotted numeric versions (`1.2.3`). Returns `Ordering`.
pub fn cmp_version(a: &str, b: &str) -> std::cmp::Ordering {
    let parse = |s: &str| -> Vec<u64> {
        s.trim_start_matches('v')
            .split(|c: char| !c.is_ascii_digit())
            .filter(|p| !p.is_empty())
            .filter_map(|p| p.parse().ok())
            .collect()
    };
    let mut aa = parse(a);
    let mut bb = parse(b);
    let n = aa.len().max(bb.len());
    aa.resize(n, 0);
    bb.resize(n, 0);
    aa.cmp(&bb)
}

/// Result of a check-update invocation.
#[derive(Debug)]
pub enum CheckResult {
    UpToDate {
        current: String,
        latest: String,
    },
    UpdateAvailable {
        current: String,
        latest: String,
        tag: String,
    },
}

pub fn check_update() -> Result<CheckResult> {
    let current = current_version().to_string();
    let rel = latest_release()?;
    if cmp_version(&current, &rel.version) == std::cmp::Ordering::Less {
        Ok(CheckResult::UpdateAvailable {
            current,
            latest: rel.version,
            tag: rel.tag,
        })
    } else {
        Ok(CheckResult::UpToDate {
            current,
            latest: rel.version,
        })
    }
}

fn asset_name(target: &str) -> String {
    if cfg!(windows) {
        format!("{BINARY}-{target}.zip")
    } else {
        format!("{BINARY}-{target}.tar.gz")
    }
}

fn download_bytes(url: &str) -> Result<Vec<u8>> {
    let agent = agent();
    let resp = agent
        .get(url)
        .call()
        .with_context(|| format!("GET {url}"))?;
    let mut data = Vec::new();
    resp.into_reader()
        .read_to_end(&mut data)
        .context("read download body")?;
    Ok(data)
}

fn verify_sha256(data: &[u8], sums: &str, asset: &str) -> Result<()> {
    let mut expected = None;
    for line in sums.lines() {
        let line = line.trim();
        if line.ends_with(asset) || line.split_whitespace().nth(1) == Some(asset) {
            expected = line.split_whitespace().next().map(|s| s.to_string());
            break;
        }
    }
    let Some(expected) = expected else {
        eprintln!("f00: warning: no SHA256SUMS entry for {asset}; skipping verify");
        return Ok(());
    };
    let mut hasher = Sha256::new();
    hasher.update(data);
    let actual = format!("{:x}", hasher.finalize());
    if actual != expected {
        bail!("checksum mismatch for {asset}\n  expected: {expected}\n  actual:   {actual}");
    }
    Ok(())
}

fn extract_binary(archive: &[u8], dest_dir: &Path) -> Result<PathBuf> {
    let out_bin = dest_dir.join(if cfg!(windows) {
        format!("{BINARY}.exe")
    } else {
        BINARY.to_string()
    });

    #[cfg(not(windows))]
    {
        use flate2::read::GzDecoder;
        use tar::Archive;
        let dec = GzDecoder::new(archive);
        let mut ar = Archive::new(dec);
        let mut found = false;
        for ent in ar.entries().context("read tar")? {
            let mut ent = ent.context("tar entry")?;
            let path = ent.path().context("tar path")?.into_owned();
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == BINARY {
                let mut f = File::create(&out_bin).context("create temp binary")?;
                io::copy(&mut ent, &mut f).context("extract binary")?;
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = fs::metadata(&out_bin)?.permissions();
                    perms.set_mode(0o755);
                    fs::set_permissions(&out_bin, perms)?;
                }
                found = true;
                break;
            }
        }
        if !found {
            bail!("binary {BINARY} not found in archive");
        }
    }

    #[cfg(windows)]
    {
        use std::io::Cursor;
        let reader = Cursor::new(archive);
        let mut zip = zip::ZipArchive::new(reader).context("open zip")?;
        let mut found = false;
        for i in 0..zip.len() {
            let mut file = zip.by_index(i).context("zip entry")?;
            let name = file.name().to_string();
            if name.ends_with("f00.exe") || name.ends_with("/f00.exe") || name == "f00.exe" {
                let mut f = File::create(&out_bin).context("create temp binary")?;
                io::copy(&mut file, &mut f).context("extract binary")?;
                found = true;
                break;
            }
        }
        if !found {
            bail!("binary f00.exe not found in zip");
        }
    }

    Ok(out_bin)
}

fn replace_current_exe(new_bin: &Path) -> Result<()> {
    let current = env::current_exe().context("current_exe")?;
    let current = fs::canonicalize(&current).unwrap_or(current);
    let dir = current
        .parent()
        .ok_or_else(|| anyhow!("cannot determine install directory"))?;

    let staged = dir.join(format!(
        ".f00-update-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));

    fs::copy(new_bin, &staged).context("stage new binary")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&staged)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&staged, perms)?;
    }

    let backup = dir.join(format!("{BINARY}.old"));
    let _ = fs::remove_file(&backup);

    // Atomic-ish: move current aside, move new into place.
    #[cfg(unix)]
    {
        fs::rename(&current, &backup).context("backup current binary")?;
        if let Err(e) = fs::rename(&staged, &current) {
            // try restore
            let _ = fs::rename(&backup, &current);
            return Err(e).context("install new binary");
        }
        let _ = fs::remove_file(&backup);
    }

    #[cfg(windows)]
    {
        // On Windows, running image may lock the file; write next to it and instruct.
        let final_path = current.clone();
        if let Err(e) = fs::rename(&staged, &final_path) {
            // try replace via remove
            let _ = fs::remove_file(&final_path);
            fs::rename(&staged, &final_path)
                .with_context(|| format!("replace binary failed: {e}"))?;
        }
    }

    Ok(())
}

/// Download latest release and replace the running binary.
pub fn perform_update() -> Result<(String, String)> {
    let current = current_version().to_string();
    let rel = latest_release()?;
    if cmp_version(&current, &rel.version) != std::cmp::Ordering::Less {
        println!(
            "f00 {current} is already up to date (latest {})",
            rel.version
        );
        return Ok((current.clone(), rel.version));
    }

    let target = host_target()?;
    let asset = asset_name(target);
    let url = format!("{RELEASES}/download/{}/{asset}", rel.tag);
    eprintln!("f00: downloading {} …", rel.tag);

    let data = download_bytes(&url)?;
    let sums_url = format!("{RELEASES}/download/{}/SHA256SUMS", rel.tag);
    if let Ok(sums) = download_bytes(&sums_url) {
        let sums = String::from_utf8_lossy(&sums);
        verify_sha256(&data, &sums, &asset)?;
        eprintln!("f00: checksum verified");
    } else {
        eprintln!("f00: warning: SHA256SUMS not available; skipping verify");
    }

    let tmp = env::temp_dir().join(format!("f00-upd-{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp)?;
    let new_bin = extract_binary(&data, &tmp)?;
    replace_current_exe(&new_bin)?;
    let _ = fs::remove_dir_all(&tmp);

    println!("f00: {current} → {}", rel.version);
    Ok((current, rel.version))
}

/// Print check-update result; exit code 0 if up to date, 1 if behind, 2 on error.
pub fn print_check_update() -> i32 {
    match check_update() {
        Ok(CheckResult::UpToDate { current, latest }) => {
            println!("f00 {current} (latest {latest}) — up to date");
            0
        }
        Ok(CheckResult::UpdateAvailable {
            current,
            latest,
            tag,
        }) => {
            println!("f00 {current} → {latest} available ({tag})");
            println!("Run: f00 --update");
            1
        }
        Err(e) => {
            eprintln!("f00: check-update failed: {e:#}");
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_cmp_basic() {
        assert_eq!(cmp_version("0.3.0", "0.4.0"), std::cmp::Ordering::Less);
        assert_eq!(cmp_version("0.4.0", "0.4.0"), std::cmp::Ordering::Equal);
        assert_eq!(cmp_version("0.4.1", "0.4.0"), std::cmp::Ordering::Greater);
        assert_eq!(cmp_version("v0.4.0", "0.3.9"), std::cmp::Ordering::Greater);
    }

    #[test]
    fn current_version_nonzero() {
        assert!(!current_version().is_empty());
    }
}
