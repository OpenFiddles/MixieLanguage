use crate::ast::PullSpec;
#[cfg(not(target_arch = "wasm32"))]
use sha2::{Digest, Sha256};
#[cfg(not(target_arch = "wasm32"))]
use std::{fs::{self, OpenOptions}, io::{Read, Write}, path::{Path, PathBuf}, process::Command};

#[derive(Debug, Clone, PartialEq)]
pub struct PullPlan {
    pub resource: String, pub url: String, pub kind: &'static str,
    pub revision: Option<String>, pub checksum: Option<String>, pub build_system: Option<String>,
}

pub fn validate_pull(s: &PullSpec) -> Result<PullPlan, String> {
    let (url, kind) = if let Some(v) = &s.source { (v, "source") }
        else if let Some(v) = &s.binary { (v, "binary") }
        else { return Err("pull needs a source or binary URL".into()); };
    if !(url.starts_with("https://") || (kind == "source" && url.starts_with("git@"))) {
        return Err("pull URLs must use HTTPS (SSH Git URLs are source-only)".into());
    }
    if kind == "binary" {
        let checksum = s.checksum.as_deref().ok_or("binary pulls require SHA256")?;
        let digest = checksum.strip_prefix("SHA256:").unwrap_or(checksum);
        if digest.len() != 64 || !digest.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("checksum must be a 64-character SHA256 hex digest".into());
        }
        if s.revision.is_some() { return Err("revision is only valid for source pulls".into()); }
    }
    if kind == "source" && s.checksum.is_some() { return Err("checksum is only valid for binary pulls".into()); }
    if s.revision.as_deref().is_some_and(|r| r.is_empty() || r.starts_with('-')) {
        return Err("revision must be non-empty and cannot start with '-'".into());
    }
    if s.build.is_some() && kind != "source" { return Err("build is only valid for source pulls".into()); }
    if s.build.as_deref().is_some_and(|v| v != "make") {
        return Err("initial source builder recognizes build = \"make\" only".into());
    }
    Ok(PullPlan { resource:s.name.clone(), url:url.clone(), kind, revision:s.revision.clone(),
        checksum:s.checksum.clone(), build_system:s.build.clone() })
}

pub fn describe_pull(s: &PullSpec) -> Result<String, String> {
    let p=validate_pull(s)?;
    let mut out=format!("Builgine pull plan: {}\n  kind: {}\n  url: {}\n",p.resource,p.kind,p.url);
    if let Some(v)=p.revision { out.push_str(&format!("  revision: {v}\n")); }
    if let Some(v)=p.checksum { out.push_str(&format!("  checksum: {v}\n")); }
    if let Some(v)=p.build_system { out.push_str(&format!("  build system: {v}\n")); }
    out.push_str("  status: validated; use Builgine's pull API to retrieve this resource\n");
    Ok(out)
}

/// Clones source with Git, or streams an HTTPS binary and verifies SHA-256 before accepting it.
#[cfg(not(target_arch = "wasm32"))]
pub fn execute_pull(plan: &PullPlan, destination: impl AsRef<Path>) -> Result<PathBuf,String> {
    let destination=destination.as_ref();
    if destination.exists() {
        if !destination.is_dir() || fs::read_dir(destination).map_err(|e|e.to_string())?.next().is_some() {
            return Err(format!("destination must be a new or empty directory: {}",destination.display()));
        }
    } else { fs::create_dir_all(destination).map_err(|e|format!("cannot create destination: {e}"))?; }
    if plan.kind == "binary" {
        let name = Path::new(&plan.resource);
        if name.components().count() != 1 || !matches!(name.components().next(), Some(std::path::Component::Normal(_))) {
            return Err("binary resource name must be a single safe filename".into());
        }
    }
    match plan.kind {
        "source" => {
            let status=Command::new("git").arg("clone").arg("--filter=blob:none").arg("--no-checkout")
                .arg("--").arg(&plan.url).arg(destination).status()
                .map_err(|e|format!("could not start git (install Git): {e}"))?;
            if !status.success(){return Err(format!("git clone failed with status {status}"));}
            let revision=plan.revision.as_deref().unwrap_or("HEAD");
            let status=Command::new("git").arg("-C").arg(destination).arg("checkout")
                .arg("--detach").arg(revision).status()
                .map_err(|e|format!("could not run git checkout: {e}"))?;
            if !status.success(){return Err(format!("git checkout failed with status {status}"));}
        }
        "binary" => {
            let expected=plan.checksum.as_deref().ok_or("binary pull has no checksum")?;
            let expected=expected.strip_prefix("SHA256:").unwrap_or(expected).to_ascii_lowercase();
            const MAX_DOWNLOAD_BYTES: u64 = 2 * 1024 * 1024 * 1024;
            let file_path=destination.join(&plan.resource);
            if file_path.exists() { return Err(format!("artifact already exists: {}",file_path.display())); }
            let temp_path=destination.join(format!(".{}.part",plan.resource));
            let result=(|| -> Result<(),String> {
                let client=reqwest::blocking::Client::builder().https_only(true)
                    .connect_timeout(std::time::Duration::from_secs(30))
                    .timeout(std::time::Duration::from_secs(300)).build()
                    .map_err(|e|format!("HTTP client error: {e}"))?;
                let mut response=client.get(&plan.url).send()
                    .map_err(|e|format!("download failed: {e}"))?.error_for_status()
                    .map_err(|e|format!("download returned an error: {e}"))?;
                if response.content_length().is_some_and(|n|n>MAX_DOWNLOAD_BYTES) {
                    return Err("download exceeds the 2 GiB safety limit".into());
                }
                let mut file=OpenOptions::new().write(true).create_new(true).open(&temp_path)
                    .map_err(|e|format!("cannot create temporary download: {e}"))?;
                let mut hasher=Sha256::new(); let mut buffer=[0u8;65536]; let mut total=0u64;
                loop {
                    let n=response.read(&mut buffer).map_err(|e|format!("download read failed: {e}"))?;
                    if n==0 {break;}
                    total=total.saturating_add(n as u64);
                    if total>MAX_DOWNLOAD_BYTES { return Err("download exceeds the 2 GiB safety limit".into()); }
                    hasher.update(&buffer[..n]);
                    file.write_all(&buffer[..n]).map_err(|e|format!("file write failed: {e}"))?;
                }
                file.flush().map_err(|e|format!("file flush failed: {e}"))?;
                file.sync_all().map_err(|e|format!("file sync failed: {e}"))?;
                let actual=format!("{:x}",hasher.finalize());
                if actual!=expected { return Err(format!("SHA256 mismatch: expected {expected}, received {actual}")); }
                drop(file);
                fs::hard_link(&temp_path,&file_path)
                    .map_err(|e|format!("cannot publish verified artifact: {e}"))?;
                Ok(())
            })();
            let _=fs::remove_file(&temp_path);
            result?;
            return Ok(file_path);
        }
        _=>return Err("unknown pull kind".into()),
    }
    Ok(destination.to_path_buf())
}

/// Runs only the fixed Make backend after explicit confirmation that the source is trusted.
#[cfg(not(target_arch = "wasm32"))]
pub fn build_source(plan:&PullPlan,source_dir:impl AsRef<Path>,confirm_trusted_source:bool)->Result<(),String>{
    if !confirm_trusted_source{return Err("build cancelled: explicitly confirm that the source tree is trusted".into());}
    if plan.kind!="source"||plan.build_system.as_deref()!=Some("make"){
        return Err("this build plan does not select the make source builder".into());
    }
    let source_dir=source_dir.as_ref();
    if !source_dir.join("Makefile").is_file()&&!source_dir.join("makefile").is_file(){
        return Err("source directory has no Makefile".into());
    }
    let status=Command::new("make").arg("-C").arg(source_dir).status()
        .map_err(|e|format!("could not start make (install build tools): {e}"))?;
    if !status.success(){return Err(format!("make failed with status {status}"));}
    Ok(())
}
