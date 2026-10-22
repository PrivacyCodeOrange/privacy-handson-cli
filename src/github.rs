use serde_json::Value;

pub struct DeviceCodeInfo {
    pub device_code: String,
    // Interval in seconds at which to poll requests for the access_token
    pub interval: u64,
    pub user_code: String,
    pub verification_uri: String,
}

pub fn request_device_code(client_id: &str) -> DeviceCodeInfo {
    // Sending a POST to the url, as documented here: https://docs.github.com/en/enterprise-cloud@latest/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app#using-the-device-flow-to-generate-a-user-access-token
    let url = "https://github.com/login/device/code";

    let response = minreq::post(url)
        .with_body(format!("client_id={}", client_id))
        .with_header("Accept", "application/json")
        .with_header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .expect(&format!(
            "Can't connect to {}. Are you connected to the internet?",
            url
        ));

    let data = response
        .json::<Value>()
        .expect("Can't parse github's device response. Is it JSON?");

    let device_code = data["device_code"]
        .as_str()
        .expect("Can't find `device_code` in JSON response.")
        .to_string();

    let interval = data["interval"]
        .as_u64()
        .expect("Can't find `interval` in JSON response.");

    let user_code = data["user_code"]
        .as_str()
        .expect("Can't find `user_code` in JSON response.")
        .to_string();

    let verification_uri = data["verification_uri"]
        .as_str()
        .expect("Can't find `verification_uri` in JSON response.")
        .to_string();

    DeviceCodeInfo {
        device_code,
        interval,
        user_code,
        verification_uri,
    }
}

pub fn request_token(client_id: &str, device_code: &str) -> Value {
    let url = "https://github.com/login/oauth/access_token";

    let body = format!(
        "client_id={}&device_code={}&grant_type={}",
        client_id,
        urlencoding::encode(device_code),
        "urn:ietf:params:oauth:grant-type:device_code",
    );

    let response = minreq::post(url)
        .with_body(body)
        .with_header("Accept", "application/json")
        .with_header("Content-Type", "application/x-www-form-urlencoded")
        .send()
        .expect(&format!(
            "Can't connect to {}. Are you connected to the internet?",
            url
        ));

    let data = response
        .json::<Value>()
        .expect("Can't parse github's oauth response. Is it JSON?");

    data
}

pub fn get_repo_is_template(owner: &str, repo_name: &str) -> bool {
    // Reference: https://docs.github.com/en/rest/repos/repos?apiVersion=2026-03-10#get-a-repository
    let url = format!("https://api.github.com/repos/{}/{}", owner, repo_name);

    let response = minreq::get(&url)
        .with_header("User-Agent", "rust-cli")
        .with_header("Accept", "application/vnd.github+json")
        .with_header("X-GitHub-Api-Version", "2026-03-10")
        .send()
        .expect(&format!(
            "Can't connect to {}. Are you connected to the internet?",
            url
        ));

    let data = response
        .json::<Value>()
        .expect("Can't parse github's get repo response. Is it JSON?");

    let Some(is_template) = data["is_template"].as_bool() else {
        return false;
    };

    return is_template;
}

pub enum GenerateRepoFromTemplateError {
    RepoAlreadyExistsError,
}
use GenerateRepoFromTemplateError::*;

pub fn generate_repo_from_template(
    access_token: &str,
    template_onwer: &str,
    template_repo: &str,
    new_repo_owner: &str,
    new_repo_name: &str,
    private: bool,
) -> Result<(String, String), GenerateRepoFromTemplateError> {
    // Sending a POST to the url, as documented here: https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28&ref=hackernoon.com&utm_source=chatgpt.com#create-a-repository-using-a-template
    let url = format!(
        "https://api.github.com/repos/{}/{}/generate",
        template_onwer, template_repo
    );

    let body = format!(
        r#"{{
            "owner": "{}",
            "name": "{}",
            "private": {}
        }}"#,
        new_repo_owner, new_repo_name, private,
    );

    let response = minreq::post(&url)
        .with_body(body)
        .with_header("User-Agent", "rust-cli")
        .with_header("Accept", "application/vnd.github+json")
        .with_header("X-GitHub-Api-Version", "2026-03-10")
        .with_header("Content-Type", "application/json")
        .with_header("Authorization", format!("Bearer {}", access_token))
        .send()
        .expect(&format!(
            "Can't connect to {}. Are you connected to the internet?",
            url
        ));

    let data = response
        .json::<Value>()
        .expect("Can't parse github's device response. Is it JSON?");

    // Case: repository already exists
    if let Some(message) = data["message"].as_str() {
        if message == "Could not clone: Name already exists on this account" {
            return Err(RepoAlreadyExistsError);
        }
    }

    let full_name = data["full_name"]
        .as_str()
        .expect("Can't find `full_name` in JSON response.")
        .to_string();

    let html_url = data["html_url"]
        .as_str()
        .expect("Can't find `html_url` in JSON response.")
        .to_string();

    Ok((full_name, html_url))
}

pub fn get_user_installations_client_ids(access_token: &str) -> Vec<String> {
    // Sending a GET to the url, as documented here: https://docs.github.com/en/rest/apps/installations?utm_source=chatgpt.com&apiVersion=2026-03-10#list-app-installations-accessible-to-the-user-access-token
    let url = "https://api.github.com/user/installations";

    let response = minreq::get(url)
        .with_param("per_page", "100")
        .with_header("User-Agent", "rust-cli")
        .with_header("Accept", "application/vnd.github+json")
        .with_header("X-GitHub-Api-Version", "2026-03-10")
        .with_header("Authorization", format!("Bearer {}", access_token))
        .send()
        .expect(&format!(
            "Can't connect to {}. Are you connected to the internet?",
            url
        ));

    let data = response
        .json::<Value>()
        .expect("Can't parse github's device response. Is it JSON?");

    let installations = data["installations"]
        .as_array()
        .expect("Can't find `installations` in JSON response.");

    let client_ids: Option<Vec<String>> = installations
        .iter()
        .map(|inst| inst["client_id"].as_str().map(|s| s.to_string()))
        .collect();
    let Some(client_ids) = client_ids else {
        panic!("Can't find `client_id` in installation object");
    };

    return client_ids;
}

pub fn get_login_id(access_token: &str) -> Option<String> {
    let url = "https://api.github.com/user";

    let response = minreq::get(url)
        .with_body(format!("{{ \"access_token\": {} }}", access_token))
        .with_header("User-Agent", "rust-cli")
        .with_header("Accept", "application/vnd.github+json")
        .with_header("Authorization", format!("Bearer {}", access_token))
        .send()
        .expect(&format!(
            "Can't connect to {}. Are you connected to the internet?",
            url
        ));

    let data = response
        .json::<Value>()
        .expect("Can't parse github's user response. Is it JSON?");

    let Some(login_id) = data["login"].as_str() else {
        return None;
    };

    Some(login_id.to_string())
}
