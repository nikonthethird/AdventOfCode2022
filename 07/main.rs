#![feature(let_chains)]
use std::{error::Error, fs::read_to_string, collections::{BTreeMap, VecDeque}};
use lazy_static::lazy_static;
use regex::Regex;

struct Directory {
    directories: BTreeMap<String, Directory>,
    files: BTreeMap<String, usize>,
}

impl Directory {
    fn new() -> Self {
        Self {
            directories: Default::default(),
            files: Default::default(),
        }
    }

    fn size(&self) -> usize {
        self.files.values().sum::<usize>() +
        self.directories.values().map(|directory| directory.size()).sum::<usize>()
    }

    fn sum_of_sizes(&self) -> usize {
        let size = self.size();
        (if size <= 100_000 { size } else { 0 }) +
        self.directories.values().map(|directory| directory.sum_of_sizes()).sum::<usize>()
    }

    fn size_of_dir_to_delete(&self, unused_space: usize) -> Option<usize> {
        let size = self.size();
        let child_size = self.directories.values().fold(None, |child_size, child_dir|
            match (child_size, child_dir.size_of_dir_to_delete(unused_space)) {
                (None, c) => c,
                (c, None) => c,
                (Some(c1), Some(c2)) => Some(c1.min(c2)),
            }
        );
        if unused_space + size >= 30_000_000 {
            if let Some(c) = child_size && c < size { child_size } else { Some(size) }
        } else {
            child_size
        }
        
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input_lines = read_to_string("input.txt")?.split('\n').skip(1).map(ToOwned::to_owned).collect::<VecDeque<_>>();
    let mut root_directory = Directory::new();

    handle_cd(&mut root_directory, &mut input_lines)?;

    let sum_of_sizes = root_directory.sum_of_sizes();
    println!("2022-12-07 Part 1: {sum_of_sizes}");

    let unused_space = 70_000_000 - root_directory.size();
    let directory_to_delete = root_directory.size_of_dir_to_delete(unused_space).ok_or("no directory found")?;
    Ok(println!("2022-12-07 Part 2: {directory_to_delete}"))
}

fn handle_cd(directory: &mut Directory, input_lines: &mut VecDeque<String>) -> Result<(), Box<dyn Error>> {
    lazy_static! {
        static ref CMD_REGEX: Regex = Regex::new(r"^\$ (?P<name>\w+)(?: (?P<param>.+))?$").unwrap();
    }
    Ok(while !input_lines.is_empty() {
        let input_line = input_lines.pop_front().ok_or("no input line")?;
        let command_captures = CMD_REGEX.captures(&input_line).ok_or("no command in input line")?;
        let command_name = command_captures.name("name").ok_or("no command name in input line")?;
        let parameter_opt = command_captures.name("param").map(|param| param.as_str().to_owned());
        match command_name.as_str() {
            "ls" =>
                handle_ls(directory, input_lines)?,
            "cd" if parameter_opt.as_ref().map(String::as_str) != Some("..") => {
                let parameter = parameter_opt.ok_or("no command parameter")?;
                let child_directory = Directory::new();
                directory.directories.entry(parameter.clone()).or_insert(child_directory);
                handle_cd(directory.directories.get_mut(&parameter).ok_or("no child directory")?, input_lines)?;
            }
            _ => break,
        }
    })
}

fn handle_ls(directory: &mut Directory, input_lines: &mut VecDeque<String>) -> Result<(), Box<dyn Error>> {
    lazy_static! {
        static ref DIR_REGEX: Regex = Regex::new(r"^dir (?P<name>.+)$").unwrap();
        static ref FILE_REGEX: Regex = Regex::new(r"^(?P<len>\d+) (?P<name>.+)$").unwrap();
    }
    Ok(while let Some(front) = input_lines.front() && !front.starts_with("$") {
        let input_line = input_lines.pop_front().ok_or("no input line")?;
        if let Some(file_captures) = FILE_REGEX.captures(&input_line) {
            let file_name = file_captures.name("name").ok_or("no file name")?.as_str().to_owned();
            let file_length = file_captures.name("len").ok_or("no file length")?.as_str().parse()?;
            directory.files.entry(file_name).or_insert(file_length);
        }
    })
}
