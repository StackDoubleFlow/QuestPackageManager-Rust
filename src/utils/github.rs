use crate::data::config::get_keyring;

pub fn get_release(url: String, out: &std::path::Path) -> bool {
    if let Ok(token_unwrapped) = get_keyring().get_password() {
        get_release_with_token(url, out, &token_unwrapped)
    } else {
        get_release_without_token(url, out)
    }
}

pub fn get_release_without_token(url: String, out: &std::path::Path) -> bool {
    // TODO:
    out.exists()
}

pub fn get_release_with_token(mut url: String, out: &std::path::Path, token: &str) -> bool {
    // TODO:
    if let Some(gitidx) = url.find("github.com") {
        url.insert_str(gitidx, token);
    }

    out.exists()
}

pub fn clone(url: String, branch: Option<String>, out: &std::path::Path) -> bool {
    if let Ok(token_unwrapped) = get_keyring().get_password() {
        clone_with_token(url, branch, out, &token_unwrapped)
    } else {
        clone_without_token(url, branch, out)
    }
}

pub fn clone_without_token(url: String, branch: Option<String>, out: &std::path::Path) -> bool {
    // TODO:
    out.exists()
}

pub fn clone_with_token(
    mut url: String,
    branch: Option<String>,
    out: &std::path::Path,
    token: &str,
) -> bool {
    // TODO:
    if let Some(gitidx) = url.find("github.com") {
        url.insert_str(gitidx, token);
    }

    out.exists()
}
