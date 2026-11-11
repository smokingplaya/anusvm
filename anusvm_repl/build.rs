use anyhow::{anyhow, bail};
use chrono::Utc;
use git2::Repository;

fn main() {
  let (git_commithash_short, git_branch) = match fetch_git_info() {
    Ok((hash, branch)) => (hash, branch),
    Err(err) => panic!("anusVM™ REPL unable to build due to: {err}")
  };

  println!("cargo:rustc-env=GIT_COMMIT_SHORT={}", git_commithash_short);
  println!("cargo:rustc-env=GIT_BRANCH={}", git_branch);

  let build_data = match collect_build_data() {
    Ok(build_data) => build_data,
    Err(err) => panic!("anusVM™ REPL unable to collect build data due to: {err}")
  };

  let profile_formatted = match build_data.profile {
    BuildDataProfile::Release => "release",
    BuildDataProfile::Debug => "debug",
    BuildDataProfile::Custom(profile) => &profile.trim().to_lowercase()
  };

  println!("cargo:rustc-env=BUILDDATA_PROFILE={profile_formatted}");
  println!("cargo:rustc-env=BUILDDATA_USER={}", build_data.user);
  println!("cargo:rustc-env=BUILDDATA_TIME={}", build_data.build_time);

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

fn collect_build_data() -> anyhow::Result<BuildData> {
  let build_time = Utc::now();

  let user = match option_env!("USERNAME") {
    Some(user) => user,
    None => option_env!("USER").unwrap_or("unknown"),
  };

  let user = user.to_string();

  let profile_name = std::env::var("PROFILE")
    .map_err(|_| anyhow!("'PROFILE' environment variable is not provided"))?;

  let profile = match profile_name.as_str() {
    "release" => BuildDataProfile::Release,
    "debug" => BuildDataProfile::Debug,
    _ => BuildDataProfile::Custom(profile_name)
  };

  Ok(BuildData {
    profile,
    user,
    build_time
  })
}

struct BuildData {
  pub profile: BuildDataProfile,
  pub user: String,
  pub build_time: chrono::DateTime<Utc>
}

#[derive(Debug, Clone)]
enum BuildDataProfile {
  Release,
  Debug,
  Custom(String)
}