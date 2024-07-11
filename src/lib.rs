use git2::Repository;
use std::{
    ffi::{OsStr, OsString},
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn install_repo(repo_url: &Option<String>, path: &Option<String>) {
    println!("Installing {repo_url:?} at {path:?}");
    let tmp_path = "/tmp/dotfiles";

    // @REF [Path vs PathBuf](https://nick.groenen.me/notes/rust-path-vs-pathbuf/)
    // Assuming here that users want to just use default config directory
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

    // let _file_renaming = rename_files(&dot_files_path, &PathBuf::from(tmp_path));
    let _file_renaming = rename_files_recursively(&dot_files_path, &PathBuf::from(tmp_path));

    // TODO: Work with std::fs::{read_dir, rename} to rename all old config files that would be
    // overwritten.

    // let checkout = git2::build::CheckoutBuilder::new();
    //
    // let repo_head = repo.checkout_head(checkout);
    // println!("{:?}", repo_url.unwrap())
    // let head = repo.head().expect("So no head?");
    // repo.checkout_head(head.into());

    // Cleanup
    let _remove_tmp_dir = fs::remove_dir_all(tmp_path);
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
// dot_files_path: path to user's existing dotfiles directory
// tmp_path: path to temporary directory where the repo is cloned

// 7/10/24: Converting params to be PathBuf. Haven't converted the rest of
//          the code to reflect this yet
fn rename_files(dot_files_path: &PathBuf, tmp_path: &PathBuf) -> io::Result<()> {
    // Iterate over each item in tmp_path. Checking tmp_path first before
    // doing any renaming, as I don't want to rename/move existing files
    // that would not be affected by dotcomfy installation.
    for entry in fs::read_dir(tmp_path.as_path())? {
        let new_path = entry?.path();
        println!("{new_path:?}");
        //let new_entry = new_path.file_name()?.to_os_string();
        if let Some(new_entry) = &new_path.file_name() {
            let old_path = append_to_path(&dot_files_path, &new_entry);
            println!("{old_path:?}");
            // At this point, `new_entry` is an OsStr
            // Want to check to see if new_entry has a corresponding entry
            // in dot_files_path. If so, rename corresponding entry to
            // {corresponding_entry}.pre-dotcomfy, put new_entry in its place.
            // I think I need to move the following logic into the new_entry block.
            // match old_path.try_exists() {
            //     Ok(true) => fs::rename(old_path, new_path),
            //     Ok(false) => fs::rename(old_path, new_path),
            //     Err(e) => panic!("Failed to rename: {}", e),
            // }?;
        }
    }
    Ok(())
}

fn rename_files_recursively(dot_files_path: &PathBuf, tmp_path: &PathBuf) -> io::Result<()> {
    // Skip empty directories
    for entry in WalkDir::new(tmp_path).min_depth(1) {
        let new_path = entry?.path();
        println!("{new_path:?}");
        //let new_entry = new_path.file_name()?.to_os_string();
        if let Some(new_entry) = &new_path.file_name() {
            let old_path = append_to_path(&dot_files_path, &new_entry);
            println!("{old_path:?}");
            // At this point, `new_entry` is an OsStr
            // Want to check to see if new_entry has a corresponding entry
            // in dot_files_path. If so, rename corresponding entry to
            // {corresponding_entry}.pre-dotcomfy, put new_entry in its place.
            // I think I need to move the following logic into the new_entry block.
            // match old_path.try_exists() {
            //     Ok(true) => fs::rename(old_path, new_path),
            //     Ok(false) => fs::rename(old_path, new_path),
            //     Err(e) => panic!("Failed to rename: {}", e),
            // }?;
        }
    }
    Ok(())
}

fn append_to_path(p: impl Into<OsString>, s: impl AsRef<OsStr>) -> PathBuf {
    let mut p = p.into();
    p.push(s);
    p.into()
}
