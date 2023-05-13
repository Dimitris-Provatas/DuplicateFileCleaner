use std::collections::HashMap;
use std::env;
use std::fs::{read_dir, remove_file};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::thread;

use checksums::{hash_file, Algorithm::SHA2256};
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    // 0: program name
    // 1: path for images
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("No path provided!");
        exit(1);
    }
    if args.len() > 2 {
        println!("Too many arguments! Please provide only a path.");
        exit(1);
    }

    // read the directory
    let dir_path = PathBuf::from(&args[1]);

    if !&dir_path.is_dir() {
        println!("Path was not a directory!");
        exit(1);
    }

    let dir_entries: Vec<_> = read_dir(&dir_path).unwrap().collect();

    let mut files_map: HashMap<String, Vec<String>> = HashMap::new();
    let thread_count = 8;

    println!("Processing images");

    let progress_bar = ProgressBar::new(dir_entries.len() as u64);
    progress_bar.set_style(
        ProgressStyle::with_template(
            "{spinner} [{elapsed_precise}] [{wide_bar:.purple}] {human_pos} processed",
        )
        .unwrap(),
    );
    progress_bar.tick();

    for entry in dir_entries.iter() {
        let path = entry.as_ref().unwrap().path();
        if path.is_dir() {
            continue;
        }
        let path_string = String::from(path.to_str().unwrap());
        let file_hash = hash_file(&path, SHA2256);

        match files_map.get_mut(&file_hash) {
            Some(res) => res.push(path_string),
            None => {
                files_map.insert(file_hash, [path_string].to_vec());
            }
        }

        progress_bar.inc(1);
    }
    progress_bar.finish_with_message("Processed all images in directory.");

    println!("Removing Duplicates");

    let mut total_deleted: u64 = 0;
    let mut total_with_dups: u64 = 0;
    let delete_progress = ProgressBar::new(files_map.len() as u64);
    delete_progress.set_style(
        ProgressStyle::with_template(
            "{spinner} [{elapsed_precise}] {human_pos}/{human_len} unique images processed",
        )
        .unwrap(),
    );
    delete_progress.tick();

    for (_, paths) in files_map.iter() {
        if paths.len() < 2 {
            continue;
        }

        total_with_dups += 1;
        for i in 1..paths.len() {
            total_deleted += 1;
            let _ = remove_file(Path::new(&paths[i]));

            delete_progress.inc(1);
        }
    }
    delete_progress.finish_with_message("Finished deleting duplicates.");

    println!("Started with {} files", dir_entries.len());
    println!(
        "Ended with {} files remaining",
        dir_entries.len() - total_deleted as usize
    );

    println!(
        "Deleted {} duplicates. {} files had duplicate entries.",
        total_deleted, total_with_dups,
    );
}

// TODO: Add multithreading
