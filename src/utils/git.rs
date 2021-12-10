use std::io::{Cursor, Read, Write};

//use duct::cmd;
use serde::{Deserialize, Serialize};

use crate::data::config::get_keyring;

pub fn get_release(url: String, out: &std::path::Path) -> bool {
    if let Ok(token_unwrapped) = get_keyring().get_password() {
        get_release_with_token(url, out, &token_unwrapped)
    } else {
        get_release_without_token(url, out)
    }
}

pub fn get_release_without_token(url: String, out: &std::path::Path) -> bool {
    let mut buffer = Cursor::new(Vec::new());
    ureq::get(&url)
        .call()
        .unwrap()
        .into_reader()
        .read_to_end(buffer.get_mut())
        .unwrap();
    let mut file = std::fs::File::create(out).expect("create so file failed");
    file.write_all(&*buffer.into_inner())
        .expect("Failed to write out downloaded bytes");

    out.exists()
}

pub fn get_release_with_token(url: String, out: &std::path::Path, token: &str) -> bool {
    // had token, use it!
    // download url for a private thing: still need to get asset id!
    // from this: "https://github.com/$USER/$REPO/releases/download/$TAG/$FILENAME"
    // to this: "https://$TOKEN@api.github.com/repos/$USER/$REPO/releases/assets/$ASSET_ID"
    let split: Vec<String> = url.split('/').map(|el| el.to_string()).collect();

    // Obviously this is a bad way of parsing the GH url but like I see no better way, people better not use direct lib uploads lol
    // (I know mentioning it here will make people do that, so fuck y'all actually thinking of doing that)
    let user = split.get(3).unwrap();
    let repo = split.get(4).unwrap();
    let tag = split.get(7).unwrap();
    let filename = split.get(8).unwrap();

    let asset_data_link = format!(
        "https://{}@api.github.com/repos/{}/{}/releases/tags/{}",
        &token, &user, &repo, &tag
    );

    let data;
    match ureq::get(&asset_data_link).call() {
        Ok(o) => data = o.into_json::<GithubReleaseData>().unwrap(),
        Err(e) => {
            let error_string = e.to_string().replace(&token, "***");
            panic!("{}", error_string);
        }
    }

    for asset in data.assets.iter() {
        if asset.name.eq(filename) {
            // this is the correct asset!
            let mut buffer = Cursor::new(Vec::new());
            let download = asset
                .url
                .replace("api.github.com", &format!("{}@api.github.com", token));

            ureq::get(&download)
                .call()
                .unwrap()
                .into_reader()
                .read_to_end(buffer.get_mut())
                .unwrap();
            let mut file = std::fs::File::create(out).expect("create so file failed");
            file.write_all(&*buffer.into_inner())
                .expect("Failed to write out downloaded bytes");
            break;
        }
    }

    out.exists()
}

pub fn clone(mut url: String, branch: Option<String>, out: &std::path::Path) -> bool {
    if let Ok(token_unwrapped) = get_keyring().get_password() {
        if let Some(gitidx) = url.find("github.com") {
            url.insert_str(gitidx, &format!("{}@", token_unwrapped));
        }
    }

    let mut git = std::process::Command::new("git");
    git.arg("clone")
        .arg(format!("{}.git", url))
        .arg(&out)
        .arg("--depth")
        .arg("1")
        .arg("--recurse-submodules")
        .arg("--shallow-submodules")
        .arg("--quiet");

    if let Some(branch_unwrapped) = branch {
        git.arg("--branch").arg(branch_unwrapped);
    } else {
        println!("No branch name found, cloning default branch");
    }

    match git.output() {
        Ok(_o) => {
            println!("status: {}", _o.status);
            println!(
                "stdout: {}",
                std::str::from_utf8(_o.stdout.as_slice()).unwrap()
            );
            println!(
                "stderr: {}",
                std::str::from_utf8(_o.stderr.as_slice()).unwrap()
            );
        }
        Err(e) => {
            let mut error_string = e.to_string();

            if let Ok(token_unwrapped) = get_keyring().get_password() {
                error_string = error_string.replace(&token_unwrapped, "***");
            }

            println!("{}", error_string);
        }
    }

    out.exists()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubReleaseAsset {
    pub url: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GithubReleaseData {
    pub assets: Vec<GithubReleaseAsset>,
}
