extern crate git2;

use std::io::{Error, ErrorKind};
use self::git2::Repository;

// PLACEHOLDER
pub fn clone_remote_repo(remote_repo: &str) -> Result<(String), Error>{
  let url = "https://alanwilhelm/hydra_agent";

  // download to cache in ~/.hydra
  let repo = match Repository::clone(url, ".hydra") {
      Ok(repo) => repo,
      Err(e) => panic!("failed to clone: {}", e),
    };
    let s: String = "result".to_string();
    Ok(s)
}
