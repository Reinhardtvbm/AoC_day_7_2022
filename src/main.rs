use std::{fs::read_to_string, rc::Rc};

mod tree;

use tree::{Tree, TreeError, Type};

use crate::tree::{get_directories, part_1};

enum Command {
    List,
    None,
}

fn main() -> Result<(), TreeError> {
    let file_contents = read_to_string("data.txt").unwrap();
    let lines = file_contents.lines();

    /*
        - if the is a '$' then it is a command
            - "ls": child directories will be listed
            - "cd /": go to outermost directory
            - "cd ..": go back one directory
    */

    let mut file_system: Tree = Tree::new((0, String::from("head")));
    let mut command = Command::None;

    for line in lines {
        if let Some(first_char) = line.chars().nth(0) {
            match first_char {
                '$' => {
                    if let Some(third_char) = line.chars().nth(2) {
                        match third_char {
                            'c' => {
                                let directory_name =
                                    line.trim().split(' ').collect::<Vec<&str>>()[2];

                                if directory_name == "/" {
                                    file_system.return_to_head()?;
                                } else if directory_name == ".." {
                                    file_system.move_out()?;
                                } else {
                                    file_system.move_into_directory(directory_name.to_string())?;
                                }
                            }
                            _ => command = Command::List,
                        }
                    }
                }
                _ => match command {
                    Command::List => {
                        if let Some(first_char) = line.chars().nth(0) {
                            let split_line = line.trim().split(' ').collect::<Vec<&str>>();

                            if first_char != 'd' {
                                let new_value = (
                                    split_line[0].parse::<usize>().unwrap(),
                                    split_line[1].to_string(),
                                );

                                file_system.add_child(new_value, Type::File)?;
                            } else {
                                file_system
                                    .add_child((0, split_line[1].to_string()), Type::Directory)?;
                            }
                        }
                    }
                    _ => (),
                },
            }
        } else {
            return Err(TreeError::NoHeadDirectory);
        }
    }

    println!(
        "Part 1: {}",
        part_1(Some(Rc::clone(&file_system.head.clone().unwrap())))
    );

    // ======================================================================================================================================
    // ======================================================================================================================================
    // ======================================================================================================================================

    let mut directories = get_directories(Some(Rc::clone(&file_system.head.unwrap())));

    directories.sort();

    println!("directories: {:#?}", directories);

    let required_space = directories.last().unwrap() - 40_000_000;

    let mut part_2 = 0;

    for directory_size in directories {
        if directory_size > required_space {
            part_2 = directory_size;
            break;
        }
    }

    println!("Part 2: {}", part_2);

    Ok(())
}
