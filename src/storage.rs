use std::{
    collections::HashMap,
    env, fs,
    io::{self, Read, Write},
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

const TOKEN_FILENAME: &str = "token";
const USERDATA_FILENAME: &str = "user_data";
const CLI_NAME: &str = "handson";

fn cli_config_path() -> io::Result<PathBuf> {
    let base = match env::var_os("XDG_CONFIG_HOME") {
        Some(path) if !path.is_empty() => PathBuf::from(path),
        _ => {
            let home = env::var_os("HOME").ok_or_else(|| io::Error::other("HOME is not set"))?;
            PathBuf::from(home).join(".config")
        }
    };

    let dir = base.join(CLI_NAME);
    fs::create_dir_all(&dir)?;
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o700))?;

    Ok(dir)
}

fn token_path() -> io::Result<PathBuf> {
    cli_config_path().map(|dir| dir.join(TOKEN_FILENAME))
}

fn userdata_path() -> io::Result<PathBuf> {
    cli_config_path().map(|dir| dir.join(USERDATA_FILENAME))
}

pub fn save_token(token: &str) -> io::Result<()> {
    let path = token_path()?;

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(path)?;

    file.set_permissions(fs::Permissions::from_mode(0o600))?;
    file.write_all(token.as_bytes())?;
    file.sync_all()
}

pub fn load_token() -> io::Result<String> {
    let token = fs::read_to_string(token_path()?)?;
    let token = token.trim_end().to_owned();

    if token.is_empty() {
        return Err(io::Error::other("Token is empty"));
    }

    Ok(token)
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct RepositoryInfo {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct Repositories(HashMap<String, Vec<RepositoryInfo>>);

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub struct UserData {
    repositories: Repositories,
    current_lab: String,
}

pub fn load_userdata() -> io::Result<UserData> {
    let path = userdata_path()?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .mode(0o600)
        .open(path)?;
    file.set_permissions(fs::Permissions::from_mode(0o600))?;

    let mut json = String::new();
    let _ = file.read_to_string(&mut json)?;
    if json.trim().is_empty() {
        return Ok(UserData::default());
    }
    let user_data: UserData = serde_json::from_str(&json)?;

    Ok(user_data)
}

pub fn save_userdata(userdata: &UserData) -> io::Result<()> {
    let path = userdata_path()?;

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(0o600)
        .open(path)?;

    file.set_permissions(fs::Permissions::from_mode(0o600))?;

    let json = serde_json::to_string(userdata)?;
    file.write_all(json.as_bytes())?;

    file.sync_all()
}

pub fn load_repositories() -> io::Result<Repositories> {
    Ok(load_userdata()?.repositories)
}

pub fn save_repositories(repositories: &Repositories) -> io::Result<()> {
    let mut userdata = load_userdata()?;
    userdata.repositories = repositories.clone();
    save_userdata(&userdata)?;

    Ok(())
}

pub fn add_repository(url: &str, lab: &str) -> io::Result<()> {
    let mut repositories = load_repositories()?;
    let repo_info = RepositoryInfo {
        url: url.to_string(),
    };
    repositories
        .0
        .entry(lab.to_string())
        .or_default()
        .push(repo_info);
    save_repositories(&repositories)?;

    Ok(())
}

pub fn load_current_lab() -> io::Result<String> {
    Ok(load_userdata()?.current_lab)
}

pub fn save_current_lab(lab: &str) -> io::Result<()> {
    let mut userdata = load_userdata()?;
    userdata.current_lab = lab.to_string();
    save_userdata(&userdata)?;

    Ok(())
}
