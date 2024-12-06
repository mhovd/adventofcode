use std::io::{Read, Write};

use anyhow::{Context, Result};
use ndarray::Array2;

#[derive(Clone, Copy, Debug, PartialEq)]
enum TileType {
    Visited,
    Unvisited,
    Blocked,
    Guard,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() -> Result<()> {
    let mut file = std::fs::File::open("examples/2024_6/input.txt").unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)
        .context("Failed to read file")?;

    // Convert buffer to string
    let file_content = String::from_utf8(buffer).context("Failed to convert buffer to string")?;

    // remove line breaks
    let file_content = file_content.replace("\n", "");

    // split string at each 140th character and create an array
    let char_vec = file_content.chars().collect::<Vec<_>>();
    let binding_enums: Vec<TileType> = char_vec
        .iter()
        .map(|c| match c {
            '.' => TileType::Unvisited,
            '#' => TileType::Blocked,
            '^' => TileType::Guard,
            _ => panic!("Invalid character"),
        })
        .collect();

    // create a 2D array
    let mut maze = Array2::<TileType>::from_shape_vec((130, 130), binding_enums).unwrap();

    // Initialize the guard position
    let mut position = maze
        .indexed_iter()
        .find(|(_, &tile)| tile == TileType::Guard)
        .map(|((i, j), _)| (i, j))
        .unwrap();

    // Intialize the direction
    let mut direction = Direction::Up;

    // Loop
    loop {
        // Mark the position as visited
        maze[position] = TileType::Visited;

        // Determine the next position
        let next_position = match direction {
            Direction::Up => (position.0 as isize - 1, position.1 as isize),
            Direction::Down => (position.0 as isize + 1, position.1 as isize),
            Direction::Left => (position.0 as isize, position.1 as isize - 1),
            Direction::Right => (position.0 as isize, position.1 as isize + 1),
        };

        // Check that the next position is within bounds
        if next_position.0 < 0
            || next_position.0 >= 130
            || next_position.1 < 0
            || next_position.1 >= 130
        {
            break;
        }

        let next_position = (next_position.0 as usize, next_position.1 as usize);

        // If the next position is blocked, change direction by going right
        if maze[next_position] == TileType::Blocked {
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
        }

        // Move to the next position
        position = match direction {
            Direction::Up => (position.0 - 1, position.1),
            Direction::Down => (position.0 + 1, position.1),
            Direction::Left => (position.0, position.1 - 1),
            Direction::Right => (position.0, position.1 + 1),
        };
    }

    // Count the number of visited tiles
    let visited_count = maze
        .iter()
        .filter(|&&tile| tile == TileType::Visited)
        .count();

    println!("Visited tiles: {}", visited_count);

    // Convert the enums to the characters to display it nicely
    let maze_chars: Vec<char> = maze
        .iter()
        .map(|&tile| match tile {
            TileType::Visited => 'X',
            TileType::Unvisited => '.',
            TileType::Blocked => '#',
            TileType::Guard => '^',
        })
        .collect();

    // Write the maze to a text file
    let mut output_file = std::fs::File::create("examples/2024_6/output.txt")
        .context("Failed to create output file")?;

    for chunk in maze_chars.chunks(130) {
        writeln!(output_file, "{}", chunk.iter().collect::<String>())
            .context("Failed to write to output file")?;
    }

    Ok(())
}
