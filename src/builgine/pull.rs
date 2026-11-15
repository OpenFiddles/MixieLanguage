use crate::ast::PullSpec;
use sha2::{Digest, Sha256};
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
            let file_path=destination.join(&plan.resource);
            let response=reqwest::blocking::Client::builder().https_only(true).build()
                .map_err(|e|format!("HTTP client error: {e}"))?.get(&plan.url).send()
                .map_err(|e|format!("download failed: {e}"))?.error_for_status()
                .map_err(|e|format!("download returned an error: {e}"))?;
            let mut response=response;
            let mut file=OpenOptions::new().write(true).create_new(true).open(&file_path)
                .map_err(|e|format!("cannot create downloaded file: {e}"))?;
            let mut hasher=Sha256::new(); let mut buffer=[0u8;65536];
            loop {
                let n=response.read(&mut buffer).map_err(|e|format!("download read failed: {e}"))?;
                if n==0 {break;}
                hasher.update(&buffer[..n]);
                file.write_all(&buffer[..n]).map_err(|e|format!("file write failed: {e}"))?;
            }
            file.flush().map_err(|e|format!("file flush failed: {e}"))?;
            let actual=format!("{:x}",hasher.finalize());
            if actual!=expected {
                drop(file); let _=fs::remove_file(&file_path);
                return Err(format!("SHA256 mismatch: expected {expected}, received {actual}"));
            }
            return Ok(file_path);
        }
        _=>return Err("unknown pull kind".into()),
    }
    Ok(destination.to_path_buf())
}

/// Runs only the fixed Make backend after explicit confirmation that the source is trusted.
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
