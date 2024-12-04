use anyhow::Context;
use anyhow::Result;
use ndarray::Array2;
use std::fs;
use std::io::Read;

fn main() -> Result<()> {
    let mut file = fs::File::open("examples/2024_4/input.txt").unwrap();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)
        .context("Failed to read file")?;

    // Convert buffer to string
    let file_content = String::from_utf8(buffer).context("Failed to convert buffer to string")?;

    // remove line breaks
    let file_content = file_content.replace("\n", "");

    // split string at each 140th character and create an array
    let binding = file_content.chars().collect::<Vec<_>>();
    let chunks = binding.chunks(140);

    // create a 2D array
    let mut array = Array2::<char>::from_elem((140, 140), ' ');

    // fill the 2D array
    for (i, chunk) in chunks.enumerate() {
        for (j, c) in chunk.iter().enumerate() {
            array[[i, j]] = *c;
        }
    }

    let shifts = [-1isize, 0, 1];

    // Iterate for X
    let mut sum = 0;
    // First look through the array for X
    for col in 0..140 {
        for row in 0..140 {
            if array[[col, row]] == 'X' {
                // Then we have to look around for all M
                for colstep in shifts {
                    for rowstep in shifts {
                        let mut testcol = col as isize + colstep;
                        let mut testrow = row as isize + rowstep;

                        if testcol < 0 || testcol >= 140 || testrow < 0 || testrow >= 140 {
                            continue;
                        }

                        if array[[testcol as usize, testrow as usize]] == 'M' {
                            testcol += colstep;
                            testrow += rowstep;
                            if testcol < 0 || testcol >= 140 || testrow < 0 || testrow >= 140 {
                                continue;
                            }
                            if array[[testcol as usize, testrow as usize]] == 'A' {
                                testcol += colstep;
                                testrow += rowstep;
                                if testcol < 0 || testcol >= 140 || testrow < 0 || testrow >= 140 {
                                    continue;
                                }
                                if array[[testcol as usize, testrow as usize]] == 'S' {
                                    sum += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    print!("Sum: {}", sum);

    Ok(())
}
