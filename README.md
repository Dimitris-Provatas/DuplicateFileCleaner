# Rust Duplicate File Cleaner

![linux build workflow](https://github.com/Dimitris-Provatas/DuplicateFileCleaner/actions/workflows/Linux_Build.yml/badge.svg)

Simple project to clear duplicate files from a directory, written in Rust.

# IMPORTANT!
## This program uses hash sums of the files to find duplicates in the same directory!
## It also uses the [remove_file function](https://doc.rust-lang.org/std/fs/fn.remove_file.html) which is not reversable!
### Please use this with care.

This project was created because I wanted to clear around 11K of images that contained duplicates and I could not find a good solution online. All programs I tried either left duplicates or did not work.

This project now parses 10K images in less than a minute and I want to find a way to multithread it so it will be even faster.

## TODOs

- [X] Make it work
- [] Multithreading
- [] Document code
- [] Make the production executable less than 11MB!
