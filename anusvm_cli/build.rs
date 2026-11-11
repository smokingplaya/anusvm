use anyhow::bail;
use git2::Repository;

fn main() {
  let (git_commithash_short, git_branch) = match fetch_git_info() {
    Ok((hash, branch)) => (hash, branch),
    Err(err) => panic!("anusVM™ unable to build due to: {err}")
  };

  println!("cargo:rustc-env=GIT_COMMIT_SHORT={}", git_commithash_short);
  println!("cargo:rustc-env=GIT_BRANCH={}", git_branch);

  println!("cargo:rerun-if-changed=../.git/HEAD");
}

fn fetch_git_info() -> anyhow::Result<(String, String)> {
  let repo = Repository::init("../")?;
  let head = repo.head()?;

  let git_commithash = head.peel_to_commit()?
    .id().to_string();

  let git_branch = match head.is_branch() {
    true => head.shorthand()?.to_string(),
    false => bail!("failed to fetch git branch")
  };

  let git_commithash_short = git_commithash[0..7].to_owned();

  Ok((git_commithash_short, git_branch))
}