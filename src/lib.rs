use git2::Repository;
use std::{
    ffi::{OsStr, OsString},
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
    str::FromStr,
};
use walkdir::WalkDir;

pub fn install_repo(repo_url: &Option<String>, path: &Option<String>) {
    println!("Installing {repo_url:?} at {path:?}");
    let tmp_path = "/tmp/dotfiles";

    // @REF [Path vs PathBuf](https://nick.groenen.me/notes/rust-path-vs-pathbuf/)
    // Use home directory by default
    let mut old_dotfiles_path = dirs::home_dir().unwrap();

    if let Some(path) = path {
        old_dotfiles_path = PathBuf::from(path);
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

fn rename_symlink_unix(old_dotfiles_path: &PathBuf, dotcomfy_path: &PathBuf) -> io::Result<()> {
    // Skip empty directories
    for entry in WalkDir::new(dotcomfy_path).min_depth(1) {
        let entry = entry?;
        let new_path = entry.path();
        let dotcomfy_path_str = dotcomfy_path.to_str().unwrap();
        println!("New path: {new_path:?}");
        // We don't care about git files
        if new_path.to_str().unwrap().contains(".git") {
            println!("Skipping git stuff");
        } else if new_path.to_str() == Some(&(dotcomfy_path_str.to_owned() + "README.md")) {
            // In this condition, I'm trying to see if the entry is the Git
            // repo's surface level README.md. Right now, it's not being
            // caught for some reason.
            println!("Skipping repo's README");
        } else {
            if let Some(new_entry) = &new_path.file_name() {
                if new_entry.to_str().unwrap().contains(".") {
                    // center_path represents the path of the directory entry
                    // with the dotcomfy_path prefix removed.
                    let center_path = PathBuf::from_str(
                        &new_path
                            .to_str()
                            .unwrap()
                            .strip_prefix(dotcomfy_path.to_str().unwrap())
                            .unwrap(),
                    );
                    let old_path = append_to_path(&old_dotfiles_path, &center_path.unwrap());
                    println!("Old path: {old_path:?}");
                    // Want to check to see if new_entry has a corresponding entry
                    // in old_dotfiles_path. If so, rename corresponding entry to
                    // {corresponding_entry}.pre-dotcomfy, put new_entry symlink in its place.
                    match old_path.try_exists() {
                        Ok(true) => {
                            println!("Old path exists, renaming to {old_path:?}.pre-dotcomfy")
                        }
                        Ok(false) => println!("Old path DOES NOT exist, just creating symlink"),
                        Err(e) => {
                            panic!(
                                "Something went wrong when checking if {old_path:?} exists: {}",
                                e
                            )
                        } //     Ok(true) => fs::rename(old_path, new_path),
                          //     Ok(false) => fs::rename(old_path, new_path),
                          //     Err(e) => panic!("Failed to rename: {}", e),
                    };
                } else {
                    println!("Ignoring directories");
                }
            }
        }
        println!();
    }
    Ok(())
}

fn append_to_path(p: impl Into<OsString>, s: impl AsRef<OsStr>) -> PathBuf {
    let mut p = p.into();
    p.push(s);
    p.into()
}
