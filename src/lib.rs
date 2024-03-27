use std::{
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

use git2::Repository;

pub fn install_repo(repo_url: &Option<String>, path: &Option<String>) {
    println!("Installing {repo_url:?} at {path:?}");
    let dot_files_path = "/tmp/dotfiles";

    // @REF [Path vs PathBuf](https://nick.groenen.me/notes/rust-path-vs-pathbuf/)
    let mut home_path = dirs::config_local_dir().unwrap();

    if let Some(path) = path {
        home_path = PathBuf::from(path);
    } else if let Some(cfg) = dirs::config_local_dir() {
        home_path = cfg;
    } else if let Some(cfg) = dirs::config_dir() {
        home_path = cfg;
    }

    let _repo: Repository = if let Some(repo_url) = &repo_url {
        if repo_url.starts_with("https://")
        /* && repo_url.contains("/dotfiles.git")*/
        {
            println!("Custom repo");
            match Repository::clone(repo_url, dot_files_path) {
                Ok(repo) => repo,
                Err(e) => panic!("Failed to clone: {}", e),
            }
        } else {
            println!("Default repo");
            let repo_url = format!("https://github.com/{}/dotfiles.git", repo_url);
            match Repository::clone(&repo_url, dot_files_path) {
                Ok(repo) => repo,
                Err(e) => panic!("Failed to clone: {}", e),
            }
        }
    } else {
        println!("Creating new repo at {dot_files_path}");
        Repository::init(dot_files_path).expect("Could not create dotiles")
    };

    // let _file_renaming = visit_dirs(home_path.as_path(), &rename_files);

    // TODO: rename old config folder and move files in repo to be new config files

    // let checkout = git2::build::CheckoutBuilder::new();
    //
    // let repo_head = repo.checkout_head(checkout);
    // println!("{:?}", repo_url.unwrap())
    // let head = repo.head().expect("So no head?");
    // repo.checkout_head(head.into());

    // Cleanup
    // let _remove_tmp_dir = fs::remove_dir_all(dot_files_path);
}

// TODO: Look at changing &Path to AsRef<Path>
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry) -> io::Result<()>) -> io::Result<()> {
    if dir.is_dir() {
        println!("{dir:?}");
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry)?
            }
        }
    }
    Ok(())
}

fn rename_files(entry: &DirEntry) -> io::Result<()> {
    println!("{entry:?}");
    Ok(())
}
