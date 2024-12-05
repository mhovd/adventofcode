use anyhow::{Context, Result};
use serde::Deserialize;

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

    let mut distances = Vec::new();
    left.clone()
        .into_iter()
        .zip(right.clone().iter())
        .for_each(|(l, r)| {
            let distance = r - l;
            distances.push(distance);
        });

    let sum = distances.iter().sum::<i64>();
    println!("Sum: {}", sum);

    let mut counts = Vec::new();
    for l in left.clone() {
        // count how many times l is in right
        let count = right.iter().filter(|&r| *r == l).count() as i64;
        counts.push(count);
    }

    let mut countsum = 0;
    for i in 0..counts.len() {
        countsum += counts[i] * left[i];
    }
    println!("Adjusted sum: {}", countsum);

    Ok(())
}
