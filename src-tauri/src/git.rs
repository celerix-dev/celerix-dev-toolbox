use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use tokio::fs;
use tokio::process::Command;
use base64::{Engine as _, engine::general_purpose};
use tauri::Manager;

#[derive(Serialize, Deserialize)]
pub struct GitBranch {
    pub name: String,
    pub is_current: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GitStash {
    pub index: usize,
    pub message: String,
    pub branch: String,
}

#[derive(Serialize, Deserialize)]
pub struct GitRemote {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct GitCommitFile {
    pub path: String,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct GitCommit {
    pub hash: String,
    pub author: String,
    pub author_email: String,
    pub message: String,
    pub body: String,
    pub date: String,
    pub parents: Vec<String>,
    pub branches: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct GitStatusFile {
    pub path: String,
    pub status: String,
    pub is_staged: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SshKeyInfo {
    pub public_key: String,
    pub has_key: bool,
    pub path: String,
}

async fn run_git_command(path: &str, args: &[&str]) -> Result<std::process::Output, String> {
    Command::new("git")
        .arg("-C").arg(path)
        .args(args)
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GIT_SSH_COMMAND", "ssh -o BatchMode=yes")
        .output()
        .await
        .map_err(|e| format!("Failed to execute git {}: {}", args.get(0).unwrap_or(&"command"), e))
}

#[tauri::command]
pub async fn clear_avatar_cache(app_handle: tauri::AppHandle) -> Result<(), String> {
    let cache_dir = app_handle.path().app_cache_dir().map_err(|e| e.to_string())?.join("avatars");
    if cache_dir.exists() {
        fs::remove_dir_all(&cache_dir).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn get_avatar(app_handle: tauri::AppHandle, email: String, name: String, repo_path: Option<String>) -> Result<String, String> {
    let email = email.trim().to_lowercase();
    let name = name.trim();
    let hash = format!("{:x}", md5::compute(&email));
    let cache_dir = app_handle.path().app_cache_dir().map_err(|e| e.to_string())?.join("avatars");
    if !cache_dir.exists() {
        fs::create_dir_all(&cache_dir).await.map_err(|e| e.to_string())?;
    }
    let file_path = cache_dir.join(format!("{}.png", hash));
    if file_path.exists() {
        let bytes = fs::read(&file_path).await.map_err(|e| e.to_string())?;
        return Ok(format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(bytes)));
    }

    let mut avatar_bytes = None;
    let client = reqwest::Client::new();

    // 1. Try Remote-specific avatar if repo_path is provided
    if let Some(path) = repo_path {
        // Simple check for GitHub remotes
        if let Ok(output) = run_git_command(&path, &["remote", "-v"]).await {
            let remote_info = String::from_utf8_lossy(&output.stdout);
            if remote_info.contains("github.com") {
                // Try to get GitHub username using `gh` CLI if available
                let gh_username = Command::new("gh")
                    .args(&["api", "user", "--jq", ".login"])
                    .output()
                    .await
                    .ok()
                    .filter(|o| o.status.success())
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());

                if let Some(username) = gh_username {
                    let unavatar_url = format!("https://unavatar.io/github/{}?fallback=false", username);
                    if let Ok(response) = client.get(unavatar_url).send().await {
                        if response.status().is_success() {
                            if let Ok(bytes) = response.bytes().await {
                                avatar_bytes = Some(bytes);
                            }
                        }
                    }
                }

                // If gh failed or didn't return a match, try name-based check (if it looks like a username)
                if avatar_bytes.is_none() && !name.contains(' ') && !name.is_empty() {
                    let unavatar_url = format!("https://unavatar.io/github/{}?fallback=false", name);
                    if let Ok(response) = client.get(unavatar_url).send().await {
                        if response.status().is_success() {
                            if let Ok(bytes) = response.bytes().await {
                                avatar_bytes = Some(bytes);
                            }
                        }
                    }
                }

                // Try email with unavatar (works better than unavatar.io/github/EMAIL)
                if avatar_bytes.is_none() {
                    let unavatar_url = format!("https://unavatar.io/{}?fallback=false", email);
                    if let Ok(response) = client.get(unavatar_url).send().await {
                        if response.status().is_success() {
                            if let Ok(bytes) = response.bytes().await {
                                avatar_bytes = Some(bytes);
                            }
                        }
                    }
                }
            }
        }
    }

    // 2. Fallback to Gravatar
    if avatar_bytes.is_none() {
        let url = format!("https://www.gravatar.com/avatar/{}?d=identicon&s=128", hash);
        let response = client.get(url).send().await.map_err(|e| e.to_string())?;
        if response.status().is_success() {
            avatar_bytes = Some(response.bytes().await.map_err(|e| e.to_string())?);
        }
    }

    if let Some(bytes) = avatar_bytes {
        fs::write(&file_path, &bytes).await.map_err(|e| e.to_string())?;
        return Ok(format!("data:image/png;base64,{}", general_purpose::STANDARD.encode(bytes)));
    }

    Err("Failed to fetch avatar".to_string())
}

#[tauri::command]
pub async fn get_git_status(path: String) -> Result<Vec<GitStatusFile>, String> {
    {
        let _repo = gix::open(&path).or_else(|_| gix::discover(&path)).map_err(|e| format!("Could not open git repo: {}", e))?;
    }
    let output = run_git_command(&path, &["status", "--porcelain"]).await?;
    if !output.status.success() {
        return Err(format!("Git status failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    let mut status_files = Vec::new();
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.len() < 4 { continue; }
        let x = line.chars().nth(0).unwrap_or(' ');
        let y = line.chars().nth(1).unwrap_or(' ');
        let file_path = line[3..].to_string();
        if x != ' ' && x != '?' {
            status_files.push(GitStatusFile { path: file_path.clone(), status: format!("{} ", x), is_staged: true });
        }
        if y != ' ' || (x == '?' && y == '?') {
            status_files.push(GitStatusFile { path: file_path, status: format!(" {}", y), is_staged: false });
        }
    }
    Ok(status_files)
}

#[tauri::command]
pub async fn git_commit(path: String, subject: String, body: String, amend: bool) -> Result<(), String> {
    let mut args = vec!["commit"];
    if amend { args.push("--amend"); }
    let message = if body.is_empty() { subject } else { format!("{}\n\n{}", subject, body) };
    args.push("-m");
    args.push(&message);
    let output = run_git_command(&path, &args).await?;
    if !output.status.success() {
        return Err(format!("Git commit failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    Ok(())
}

#[tauri::command]
pub async fn git_stage_file(path: String, file_path: String) -> Result<(), String> {
    let output = run_git_command(&path, &["add", &file_path]).await?;
    if !output.status.success() { return Err(format!("Git add failed: {}", String::from_utf8_lossy(&output.stderr))); }
    Ok(())
}

#[tauri::command]
pub async fn git_stage_all(path: String) -> Result<(), String> {
    let output = run_git_command(&path, &["add", "-A"]).await?;
    if !output.status.success() { return Err(format!("Git add -A failed: {}", String::from_utf8_lossy(&output.stderr))); }
    Ok(())
}

#[tauri::command]
pub async fn git_unstage_file(path: String, file_path: String) -> Result<(), String> {
    let output = run_git_command(&path, &["reset", "HEAD", "--", &file_path]).await?;
    if !output.status.success() { return Err(format!("Git reset failed: {}", String::from_utf8_lossy(&output.stderr))); }
    Ok(())
}

#[tauri::command]
pub async fn git_unstage_all(path: String) -> Result<(), String> {
    let output = run_git_command(&path, &["reset", "HEAD"]).await?;
    if !output.status.success() { return Err(format!("Git reset HEAD failed: {}", String::from_utf8_lossy(&output.stderr))); }
    Ok(())
}

#[tauri::command]
pub async fn git_discard_changes(path: String, files: Vec<String>) -> Result<(), String> {
    if files.is_empty() { return Ok(()); }
    
    let mut args = vec!["checkout".to_string(), "--".to_string()];
    for file in &files {
        args.push(file.clone());
    }
    
    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let output = run_git_command(&path, &arg_refs).await?;
    if !output.status.success() {
        return Err(format!("Git discard changes failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    
    let status = get_git_status(path.clone()).await?;
    let untracked: Vec<String> = files.into_iter()
        .filter(|f| status.iter().any(|s| &s.path == f && s.status.trim() == "??"))
        .collect();
    
    if !untracked.is_empty() {
        let mut clean_args = vec!["clean".to_string(), "-f".to_string(), "--".to_string()];
        for file in &untracked {
            clean_args.push(file.clone());
        }
        let clean_arg_refs: Vec<&str> = clean_args.iter().map(|s| s.as_str()).collect();
        let output = run_git_command(&path, &clean_arg_refs).await?;
        if !output.status.success() {
            return Err(format!("Git clean failed: {}", String::from_utf8_lossy(&output.stderr)));
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn git_stash_save(path: String, files: Vec<String>, message: Option<String>) -> Result<(), String> {
    if files.is_empty() { return Ok(()); }

    // Stashing specific files is a bit involved in git.
    // One common way:
    // 1. Stage the files we want to stash
    // 2. git stash push --staged -m "message"
    
    // First, stage the files
    for file in &files {
        git_stage_file(path.clone(), file.clone()).await?;
    }

    let mut args = vec!["stash".to_string(), "push".to_string(), "--staged".to_string()];
    if let Some(m) = message {
        if !m.trim().is_empty() {
            args.push("-m".to_string());
            args.push(m);
        }
    }

    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let output = run_git_command(&path, &arg_refs).await?;
    if !output.status.success() {
        return Err(format!("Git stash push --staged failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(())
}

#[tauri::command]
pub async fn git_stash_drop(path: String, index: usize) -> Result<(), String> {
    let stash_ref = format!("stash@{{{}}}", index);
    let output = run_git_command(&path, &["stash", "drop", &stash_ref]).await?;
    if !output.status.success() {
        return Err(format!("Git stash drop failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    Ok(())
}

#[tauri::command]
pub async fn git_stash_pop(path: String, index: usize) -> Result<(), String> {
    let stash_ref = format!("stash@{{{}}}", index);
    let output = run_git_command(&path, &["stash", "pop", &stash_ref]).await?;
    if !output.status.success() {
        return Err(format!("Git stash pop failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    Ok(())
}

#[tauri::command]
pub async fn get_git_diff(path: String, file_path: String) -> Result<String, String> {
    let output = run_git_command(&path, &["diff", "HEAD", "--", &file_path]).await?;
    if !output.status.success() {
        let output_fallback = run_git_command(&path, &["diff", "--", &file_path]).await?;
        if !output_fallback.status.success() {
            let status_output = run_git_command(&path, &["status", "--porcelain", &file_path]).await?;
            let status_str = String::from_utf8_lossy(&status_output.stdout);
            if status_str.starts_with("??") {
                let abs_path = PathBuf::from(&path).join(&file_path);
                let content = fs::read_to_string(abs_path).await.map_err(|e| e.to_string())?;
                let mut diff = format!("--- /dev/null\n+++ b/{}\n", file_path);
                for line in content.lines() { diff.push_str("+"); diff.push_str(line); diff.push_str("\n"); }
                return Ok(diff);
            }
            return Err(format!("Git diff failed: {}", String::from_utf8_lossy(&output_fallback.stderr)));
        }
        return Ok(String::from_utf8_lossy(&output_fallback.stdout).to_string());
    }
    let diff = String::from_utf8_lossy(&output.stdout).to_string();
    if diff.is_empty() {
        let output_cached = run_git_command(&path, &["diff", "--cached", "--", &file_path]).await?;
        let diff_cached = String::from_utf8_lossy(&output_cached.stdout).to_string();
        if !diff_cached.is_empty() { return Ok(diff_cached); }
        let abs_path = PathBuf::from(&path).join(&file_path);
        if let Ok(content) = fs::read_to_string(abs_path).await {
            if !content.is_empty() {
                let mut diff = format!("--- /dev/null\n+++ b/{}\n", file_path);
                for line in content.lines() { diff.push_str("+"); diff.push_str(line); diff.push_str("\n"); }
                return Ok(diff);
            }
        }
    }
    Ok(diff)
}

#[tauri::command]
pub fn get_git_branches(path: String) -> Result<Vec<GitBranch>, String> {
    let repo = gix::open(&path).or_else(|_| gix::discover(&path)).map_err(|e| e.to_string())?;
    let references = repo.references().map_err(|e| e.to_string())?;
    let head_ref = repo.head().map_err(|e| e.to_string())?;
    let head_name = head_ref.referent_name().map(|n| n.as_bstr().to_string());
    let mut branches = Vec::new();
    let local_branches = references.local_branches().map_err(|e| e.to_string())?;
    for res in local_branches {
        let reference = res.map_err(|e| e.to_string())?;
        let full_name = reference.name().as_bstr().to_string();
        let is_current = Some(&full_name) == head_name.as_ref();
        let name = if full_name.starts_with("refs/heads/") { full_name["refs/heads/".len()..].to_string() } else { full_name };
        branches.push(GitBranch { name, is_current });
    }
    Ok(branches)
}

#[tauri::command]
pub async fn get_git_commits(path: String) -> Result<Vec<GitCommit>, String> {
    // Use a custom format with a unique delimiter to handle multi-line bodies and special characters
    // %H: full commit hash
    // %an: author name
    // %ae: author email
    // %at: author date, UNIX timestamp
    // %s: subject
    // %b: body
    // %P: parent hashes
    // %D: ref names
    let delimiter = "----------COMMIT-PART----------";
    let end_delimiter = "----------COMMIT-END----------";
    let format = format!("%H{0}%an{0}%ae{0}%at{0}%s{0}%b{0}%P{0}%D{1}", delimiter, end_delimiter);
    
    let output = run_git_command(&path, &["log", "--all", "-n", "100", &format!("--pretty=format:{}", format)]).await?;
    if !output.status.success() { return Err(format!("Git log failed: {}", String::from_utf8_lossy(&output.stderr))); }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut commits = Vec::new();
    
    for commit_str in stdout.split(end_delimiter) {
        let commit_str = commit_str.trim();
        if commit_str.is_empty() { continue; }
        
        let parts: Vec<&str> = commit_str.split(delimiter).collect();
        if parts.len() >= 8 {
            let hash = parts[0].to_string();
            let author = parts[1].to_string();
            let author_email = parts[2].to_string();
            let date = parts[3].to_string();
            let message = parts[4].to_string();
            let body = parts[5].trim().to_string();
            let parents = parts[6].split_whitespace().map(|s| s.to_string()).collect();
            
            let mut branches = Vec::new();
            let mut tags = Vec::new();
            if !parts[7].is_empty() {
                for r in parts[7].split(',') {
                    let r = r.trim();
                    if r.starts_with("HEAD -> ") { 
                        branches.push(r["HEAD -> ".len()..].to_string()); 
                    } else if r.starts_with("tag: ") { 
                        tags.push(r["tag: ".len()..].to_string()); 
                    } else {
                        branches.push(r.to_string());
                    }
                }
            }
            
            commits.push(GitCommit { 
                hash, 
                author, 
                author_email, 
                message, 
                body, 
                date, 
                parents, 
                branches,
                tags
            });
        }
    }
    Ok(commits)
}

#[tauri::command]
pub async fn get_commit_files(path: String, hash: String) -> Result<Vec<GitCommitFile>, String> {
    let output = run_git_command(&path, &["show", "--name-status", "--format=", &hash]).await?;
    if !output.status.success() {
        return Err(format!("Git show failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut files = Vec::new();
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            files.push(GitCommitFile {
                status: parts[0].to_string(),
                path: parts[1].to_string(),
            });
        }
    }
    Ok(files)
}

#[tauri::command]
pub async fn get_commit_file_diff(path: String, hash: String, file_path: String) -> Result<String, String> {
    // Actually we want the diff. 'git show hash -- file_path' shows the diff.
    let output = run_git_command(&path, &["show", "--format=", &hash, "--", &file_path]).await?;
    if !output.status.success() {
        return Err(format!("Git show diff failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tauri::command]
pub fn get_git_remotes(path: String) -> Result<Vec<GitRemote>, String> {
    let repo = gix::open(&path).or_else(|_| gix::discover(&path)).map_err(|e| e.to_string())?;
    let config = repo.config_snapshot();
    let mut remotes = Vec::new();
    if let Some(remote_names) = config.sections_by_name("remote") {
        for section in remote_names {
            let name = section.header().subsection_name().map(|n| n.to_string()).unwrap_or_default();
            if name.is_empty() { continue; }
            let url = section.value("url").map(|v| v.to_string()).unwrap_or_default();
            remotes.push(GitRemote { name, url });
        }
    }
    Ok(remotes)
}

#[tauri::command]
pub async fn switch_branch(path: String, branch_name: String) -> Result<(), String> {
    let output = run_git_command(&path, &["checkout", &branch_name]).await?;
    if !output.status.success() { return Err(format!("Git checkout failed: {}", String::from_utf8_lossy(&output.stderr))); }
    Ok(())
}

#[tauri::command]
pub fn get_git_remote_branches(path: String) -> Result<Vec<String>, String> {
    let repo = gix::open(&path).or_else(|_| gix::discover(&path)).map_err(|e| e.to_string())?;
    let references = repo.references().map_err(|e| e.to_string())?;
    let mut branches = Vec::new();
    let remote_branches = references.remote_branches().map_err(|e| e.to_string())?;
    for res in remote_branches {
        let reference = res.map_err(|e| e.to_string())?;
        let full_name = reference.name().as_bstr().to_string();
        let name = if full_name.starts_with("refs/remotes/") { full_name["refs/remotes/".len()..].to_string() } else { full_name };
        if !name.ends_with("/HEAD") { branches.push(name); }
    }
    Ok(branches)
}

#[tauri::command]
pub fn get_git_tags(path: String) -> Result<Vec<String>, String> {
    let repo = gix::open(&path).or_else(|_| gix::discover(&path)).map_err(|e| e.to_string())?;
    let references = repo.references().map_err(|e| e.to_string())?;
    let mut tags = Vec::new();
    let tags_refs = references.tags().map_err(|e| e.to_string())?;
    for res in tags_refs {
        let reference = res.map_err(|e| e.to_string())?;
        let full_name = reference.name().as_bstr().to_string();
        let name = if full_name.starts_with("refs/tags/") { full_name["refs/tags/".len()..].to_string() } else { full_name };
        tags.push(name);
    }
    Ok(tags)
}

#[tauri::command]
pub async fn get_git_stashes(path: String) -> Result<Vec<GitStash>, String> {
    let output = run_git_command(&path, &["stash", "list", "--format=%gd|%s|%gD"]).await?;
    if !output.status.success() {
        return Err(format!("Git stash list failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut stashes = Vec::new();
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 3 {
            // %gd is stash@{0}, %s is subject, %gD is reflog selector
            let index_str = parts[0].trim_start_matches("stash@{").trim_end_matches("}");
            let index = index_str.parse::<usize>().unwrap_or(0);
            let message = parts[1].to_string();
            let branch = parts[2].to_string();
            stashes.push(GitStash { index, message, branch });
        }
    }
    Ok(stashes)
}

#[tauri::command]
pub async fn git_checkout_remote_branch(path: String, remote_branch: String, new_branch_name: Option<String>) -> Result<(), String> {
    let exists = {
        let repo = gix::open(&path).or_else(|_| gix::discover(&path)).map_err(|e| e.to_string())?;
        let default_local_name = if let Some(pos) = remote_branch.find('/') { &remote_branch[pos + 1..] } else { &remote_branch };
        let target_local_name = new_branch_name.as_deref().unwrap_or(default_local_name);
        let references = repo.references().map_err(|e| e.to_string())?;
        let local_branches = references.local_branches().map_err(|e| e.to_string())?;
        let mut exists = false;
        for res in local_branches { if let Ok(reference) = res { if reference.name().as_bstr().to_string() == target_local_name { exists = true; break; } } }
        exists
    };
    let default_local_name = if let Some(pos) = remote_branch.find('/') { &remote_branch[pos + 1..] } else { &remote_branch };
    let target_local_name = new_branch_name.as_deref().unwrap_or(default_local_name);
    let mut args = vec!["checkout"];
    if exists {
        if new_branch_name.is_some() { return Err(format!("Branch '{}' exists.", target_local_name)); }
        args.push(target_local_name);
    } else { args.extend_from_slice(&["-b", target_local_name, "--track", &remote_branch]); }
    let output = run_git_command(&path, &args).await?;
    if !output.status.success() { return Err(format!("Git checkout failed: {}", String::from_utf8_lossy(&output.stderr))); }
    Ok(())
}

#[tauri::command]
pub async fn get_ssh_key_info() -> Result<SshKeyInfo, String> {
    let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).map_err(|_| "No home dir")?;
    let ssh_path = PathBuf::from(home).join(".ssh");
    let (key_path, pub_key_path) = (ssh_path.join("id_ed25519"), ssh_path.join("id_ed25519.pub"));
    if key_path.exists() && pub_key_path.exists() {
        let public_key = fs::read_to_string(pub_key_path).await.map_err(|e| e.to_string())?;
        Ok(SshKeyInfo { public_key, has_key: true, path: key_path.to_string_lossy().to_string() })
    } else {
        Ok(SshKeyInfo { public_key: "".to_string(), has_key: false, path: key_path.to_string_lossy().to_string() })
    }
}

#[tauri::command]
pub async fn generate_ssh_key() -> Result<SshKeyInfo, String> {
    let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")).map_err(|_| "No home dir")?;
    let ssh_path = PathBuf::from(home).join(".ssh");
    if !ssh_path.exists() { fs::create_dir_all(&ssh_path).await.map_err(|e| e.to_string())?; }
    let key_path = ssh_path.join("id_ed25519");
    if key_path.exists() { return Err("SSH key exists".to_string()); }
    let output = Command::new("ssh-keygen").args(&["-t", "ed25519", "-f", &key_path.to_string_lossy(), "-N", "", "-C", "celerix-app"]).output().await.map_err(|e| e.to_string())?;
    if !output.status.success() { return Err(format!("ssh-keygen failed: {}", String::from_utf8_lossy(&output.stderr))); }
    get_ssh_key_info().await
}

#[tauri::command]
pub async fn git_fetch(path: String) -> Result<(), String> {
    let output = run_git_command(&path, &["fetch", "--all"]).await?;
    if !output.status.success() { return Err(format!("Git fetch failed: {}", String::from_utf8_lossy(&output.stderr))); }
    Ok(())
}

#[tauri::command]
pub async fn git_pull(path: String) -> Result<(), String> {
    let output = run_git_command(&path, &["pull"]).await?;
    if !output.status.success() { return Err(format!("Git pull failed: {}", String::from_utf8_lossy(&output.stderr))); }
    Ok(())
}

#[tauri::command]
pub async fn git_push(path: String) -> Result<(), String> {
    let output = run_git_command(&path, &["push"]).await?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("no upstream branch") {
            // Try to find the current branch name
            let branch_name = {
                let repo = gix::open(&path).or_else(|_| gix::discover(&path)).map_err(|e| e.to_string())?;
                let head_ref = repo.head().map_err(|e| e.to_string())?;
                if let Some(head_name) = head_ref.referent_name() {
                    let full_name = head_name.as_bstr().to_string();
                    if full_name.starts_with("refs/heads/") {
                        full_name["refs/heads/".len()..].to_string()
                    } else {
                        full_name
                    }
                } else {
                    return Err("Could not determine current branch name".to_string());
                }
            };
            
            // Get remotes
            let remotes = get_git_remotes(path.clone())?;
            if remotes.len() == 1 {
                let remote_name = &remotes[0].name;
                let output = run_git_command(&path, &["push", "--set-upstream", remote_name, &branch_name]).await?;
                if output.status.success() {
                    return Ok(());
                } else {
                    return Err(format!("Git push --set-upstream failed: {}", String::from_utf8_lossy(&output.stderr)));
                }
            } else if remotes.is_empty() {
                return Err("No remotes configured to push to.".to_string());
            } else {
                return Err(format!("Branch '{}' has no upstream. Please set it manually or choose a remote.", branch_name));
            }
        }
        return Err(format!("Git push failed: {}", stderr));
    }
    Ok(())
}

#[tauri::command]
pub async fn git_create_tag(path: String, tag_name: String, commit_hash: String, message: Option<String>, push_all: bool) -> Result<(), String> {
    let mut args = vec!["tag".to_string()];
    if let Some(msg) = message {
        if !msg.trim().is_empty() {
            args.push("-a".to_string());
            args.push(tag_name.clone());
            args.push("-m".to_string());
            args.push(msg);
        } else {
            args.push(tag_name.clone());
        }
    } else {
        args.push(tag_name.clone());
    }
    args.push(commit_hash);

    let arg_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    let output = run_git_command(&path, &arg_refs).await?;
    if !output.status.success() {
        return Err(format!("Git tag failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    if push_all {
        let output = run_git_command(&path, &["push", "--tags"]).await?;
        if !output.status.success() {
            return Err(format!("Git push --tags failed: {}", String::from_utf8_lossy(&output.stderr)));
        }
    } else {
        // Push only the new tag to current remote
        // We can find the current remote by looking at the branch's upstream
        let remotes = get_git_remotes(path.clone())?;
        if !remotes.is_empty() {
            // Usually we want to push to origin if it exists, or the first remote
            let remote_name = remotes.iter().find(|r| r.name == "origin").map(|r| r.name.as_str()).unwrap_or(remotes[0].name.as_str());
            let output = run_git_command(&path, &["push", remote_name, &tag_name]).await?;
            if !output.status.success() {
                return Err(format!("Git push tag failed: {}", String::from_utf8_lossy(&output.stderr)));
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn git_create_branch(path: String, branch_name: String, start_point: String, checkout: bool) -> Result<(), String> {
    if checkout {
        let output = run_git_command(&path, &["checkout", "-b", &branch_name, &start_point]).await?;
        if !output.status.success() { return Err(format!("Git checkout -b failed: {}", String::from_utf8_lossy(&output.stderr))); }
    } else {
        let output = run_git_command(&path, &["branch", &branch_name, &start_point]).await?;
        if !output.status.success() { return Err(format!("Git branch failed: {}", String::from_utf8_lossy(&output.stderr))); }
    }
    Ok(())
}

#[tauri::command]
pub async fn git_delete_branch(path: String, branch_name: String, delete_remote: bool) -> Result<(), String> {
    // Check if it's the current branch
    {
        let repo = gix::open(&path).or_else(|_| gix::discover(&path)).map_err(|e| e.to_string())?;
        let head_ref = repo.head().map_err(|e| e.to_string())?;
        if let Some(head_name) = head_ref.referent_name() {
            let full_name = head_name.as_bstr().to_string();
            let current_name = if full_name.starts_with("refs/heads/") { &full_name["refs/heads/".len()..] } else { &full_name };
            if current_name == branch_name {
                return Err("Cannot delete the currently active branch.".to_string());
            }
        }
    }

    // Delete local branch
    let output = run_git_command(&path, &["branch", "-D", &branch_name]).await?;
    if !output.status.success() {
        return Err(format!("Failed to delete local branch '{}': {}", branch_name, String::from_utf8_lossy(&output.stderr)));
    }

    if delete_remote {
        let remotes_output = run_git_command(&path, &["remote"]).await?;
        if remotes_output.status.success() {
            let remotes_str = String::from_utf8_lossy(&remotes_output.stdout);
            for remote in remotes_str.lines() {
                let remote = remote.trim();
                if remote.is_empty() { continue; }
                let _ = run_git_command(&path, &["push", remote, "--delete", &branch_name]).await;
            }
        }
    }

    Ok(())
}
