use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Row {
    left: i64,
    right: i64,
}

fn main() -> Result<()> {
    let file_content = std::fs::read_to_string("examples/2024_1/input.txt")
        .context("Failed to read input file")?;

    // Replace triple spaces with a colon
    let cleaned_content = file_content.replace("   ", ",");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .trim(csv::Trim::All) // Trim
        .from_reader(cleaned_content.as_bytes());

    let mut left = Vec::new();
    let mut right = Vec::new();

    for result in rdr.deserialize() {
        let row: Row = result.context("Failed to deserialize row")?;
        left.push(row.left);
        right.push(row.right);
    }

    left.sort();
    right.sort();

    let mut sum: i64 = 0;
    left.into_iter().zip(right.iter()).for_each(|(l, r)| {
        let distance = r - l;
        sum += distance.abs();
    });

    println!("Sum: {}", sum);

    Ok(())
}
