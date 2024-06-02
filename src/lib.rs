use std::{
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

use git2::Repository;

pub fn install_repo(repo_url: &Option<String>, path: &Option<String>) {
    println!("Installing {repo_url:?} at {path:?}");
    let tmp_path = "/tmp/dotfiles";

    // @REF [Path vs PathBuf](https://nick.groenen.me/notes/rust-path-vs-pathbuf/)
    let mut dot_files_path = dirs::config_local_dir().unwrap();

    if let Some(path) = path {
        dot_files_path = PathBuf::from(path);
    } else if let Some(cfg) = dirs::config_local_dir() {
        dot_files_path = cfg;
    } else if let Some(cfg) = dirs::config_dir() {
        dot_files_path = cfg;
    }

    let _repo: Repository = if let Some(repo_url) = &repo_url {
        if repo_url.starts_with("https://")
        /* && repo_url.contains("/dotfiles.git")*/
        {
            println!("Custom repo");
            match Repository::clone(repo_url, tmp_path) {
                Ok(repo) => repo,
                Err(e) => panic!("Failed to clone: {}", e),
            }
        } else {
            println!("Default repo");
            let repo_url = format!("https://github.com/{}/dotfiles.git", repo_url);
            match Repository::clone(&repo_url, tmp_path) {
                Ok(repo) => repo,
                Err(e) => panic!("Failed to clone: {}", e),
            }
        }
    } else {
        println!("Creating new repo at {tmp_path}");
        Repository::init(tmp_path).expect("Could not create dotiles")
    };

    let _file_renaming = rename_files(dot_files_path.as_path(), Path::new(tmp_path));

    // TODO: Work with std::fs::{read_dir, rename} to rename all old config files that would be
    // overwritten.

    // let checkout = git2::build::CheckoutBuilder::new();
    //
    // let repo_head = repo.checkout_head(checkout);
    // println!("{:?}", repo_url.unwrap())
    // let head = repo.head().expect("So no head?");
    // repo.checkout_head(head.into());

    // Cleanup
    // let _remove_tmp_dir = fs::remove_dir_all(tmp_path);
}

// TODO: Look at changing &Path to AsRef<Path>
/*
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
*/

// TODO: Turn this spaghetti into something that actually works.
fn rename_files(dot_files_path: &Path, tmp_path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(tmp_path)? {
        let new_path = entry?.path();
        // What am I even doing with `new_entry`?
        if let new_entry = new_path.file_name() {
            // At this point, `new_entry` is an OsStr
        };
        // 
        let old_path = dot_files_path.join(new_path.as_path());
        match old_path.try_exists() {
            Ok(true) => fs::rename(old_path, new_path),
            Ok(false) => fs::rename(old_path, new_path),
            Err(e) => panic!("Failed to rename: {}", e),
        }?;
        println!("{entry:?}");
    }
    Ok(())
}
