use std::{
  fs,
  collections::HashMap,
  u32
};

const ROOT: &'static str = "/";
const DUMMY_ID: u32 = 0;
const ROOT_ID: u32 = 1;

#[derive(Debug)]
struct File {
  size: u32
}

#[derive(Debug)]
struct Folder {
  children: HashMap<String, u32>,
}

#[derive(Debug)]
enum FileType {
  File(File),
  Folder(Folder)
}

fn build_filesystem(lines: &Vec<Vec<&str>>) -> HashMap<u32, FileType> {
  let mut filesystem = HashMap::new();
  filesystem.insert(ROOT_ID, FileType::Folder(Folder{children: HashMap::new()}));
  filesystem.insert(DUMMY_ID, FileType::Folder(Folder{
    children: HashMap::from([(ROOT.to_string(), ROOT_ID)])
  }));
  
  let mut dir_stack = Vec::new();
  
  let mut id_counter = ROOT_ID + 1;
  let mut curr_dir = 0;

  let mut index = 0;
  while index < lines.len() {
    let curr_line = &lines[index];
    if curr_line[1] == "cd" {
      if curr_line[2] == ".." {
        curr_dir = dir_stack.pop().unwrap();
      } else {
        dir_stack.push(curr_dir.clone());
        let FileType::Folder(curr_folder) = filesystem.get(&curr_dir).unwrap() else {
          panic!("Not a directory: {}", curr_line[2]);
        };
        curr_dir = *curr_folder.children.get(&curr_line[2].to_string()).unwrap();
      }
      index += 1;
    } else {
      index += 1;
      while index < lines.len() && lines[index][0] != "$" {
        if lines[index][0] == "dir" {
          let FileType::Folder(curr_folder) = filesystem.get_mut(&curr_dir).unwrap() else {
            panic!("Not a directory: {}", curr_line[2]);
          };
          curr_folder.children.insert(lines[index][1].to_string(), id_counter);
          filesystem.insert(id_counter, FileType::Folder(Folder{children: HashMap::new()}));
        } else {
          let FileType::Folder(curr_folder) = filesystem.get_mut(&curr_dir).unwrap() else {
            panic!("Not a directory: {}", curr_line[2]);
          };
          curr_folder.children.insert(lines[index][1].to_string(), id_counter);
          filesystem.insert(id_counter, FileType::File(File{size: lines[index][0].parse::<u32>().unwrap()}));
        }
        id_counter += 1;
        index += 1;
      }
    }
  }
  return filesystem;
}

fn get_all_dir_sizes(filesystem: &HashMap<u32, FileType>, curr_id: u32) -> HashMap<u32, u32> {
  let FileType::Folder(curr_folder) = filesystem.get(&curr_id).unwrap() else {
    panic!("Not a directory: {}", curr_id);
  };

  let mut total_size = 0;
  let mut sizes = HashMap::new();

  for (_, child_id) in curr_folder.children.iter() {
    let child_file = filesystem.get(child_id).unwrap();
    match child_file {
      FileType::File(file) => total_size += file.size,
      FileType::Folder(_) => {
        let child_size = get_all_dir_sizes(filesystem, *child_id);
        sizes.extend(child_size);
        total_size += sizes.get(child_id).unwrap();
      }
    }
  }

  sizes.insert(curr_id, total_size);
  return sizes;
}

fn sum_all_dir_sizes_below_limit(filesystem: &HashMap<u32, FileType>) -> u32 {
  let dir_sizes = get_all_dir_sizes(filesystem, ROOT_ID);
  let limit = 100000;
  return dir_sizes.values().filter(|size| **size <= limit).sum();
}

fn get_smallest_dir_size_to_delete(filesystem: &HashMap<u32, FileType>) -> u32 {
  let dir_sizes = get_all_dir_sizes(filesystem, ROOT_ID);
  let total_disk_space = 70000000;
  let used_space = dir_sizes.get(&ROOT_ID).unwrap();
  let open_space = total_disk_space - used_space;
  let space_needed = 30000000 - open_space;

  let mut space_deleted = u32::MAX;
  for (_, size) in dir_sizes.iter() {
    if *size >= space_needed && *size < space_deleted {
      space_deleted = *size;
    }
  }
  return space_deleted;
}

fn main() {
  let contents = fs::read_to_string("./data/q7.txt")
    .expect("Should have been able to read file");
  let lines = contents.split('\n').map(|line| line.split(' ').collect())
    .collect::<Vec<Vec<&str>>>();
  let filesystem = build_filesystem(&lines);
  println!("Part 1: {}", sum_all_dir_sizes_below_limit(&filesystem));
  println!("Part 2: {}", get_smallest_dir_size_to_delete(&filesystem));
}