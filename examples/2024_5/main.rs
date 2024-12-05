use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Rule {
    first: isize,
    second: isize,
}

#[derive(Debug, Clone)]
struct Update {
    update: Vec<isize>,
}

// Implement custom deserialization for `Update`
impl<'de> Deserialize<'de> for Update {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize the row as a vector of strings
        let row: Vec<String> = Deserialize::deserialize(deserializer)?;
        // Convert each string to an `isize`
        let update = row
            .into_iter()
            .map(|field| field.parse::<isize>().map_err(serde::de::Error::custom))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Update { update })
    }
}

fn main() -> Result<()> {
    // READ RULES
    let path = "examples/2024_5/input_rules.txt";
    let file = fs::File::open(path).context("Failed to open file")?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .trim(csv::Trim::All) // Trim
        .from_reader(file);

    let mut rules: Vec<Rule> = Vec::new();

    for result in rdr.deserialize() {
        let rule: Rule = result.context("Failed to deserialize row")?;
        rules.push(rule);
    }

    //dbg!(rules);

    // READ UPDATES
    let path = "examples/2024_5/input_updates.txt";
    let file = fs::File::open(path).context("Failed to open file")?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .trim(csv::Trim::All) // Trim
        .flexible(true)
        .from_reader(file);

    let mut updates: Vec<Update> = Vec::new();

    for result in rdr.deserialize() {
        let update: Update = result.context("Failed to deserialize row")?;
        updates.push(update);
    }

    dbg!(&updates);

    // Iterate over each update, and check for the rules, retaining those who match
    let mut valid_updates: Vec<Update> = Vec::new();

    for update in &updates {
        // get the pages vec
        let pages = &update.update;

        let mut valid = true;

        for rule in &rules {
            // Find the location of the first page, if any
            let first_page = pages.iter().position(|&x| x == rule.first);
            // Find the location of the second page, if any
            let second_page = pages.iter().position(|&x| x == rule.second);

            if first_page.is_some() && second_page.is_some() {
                if first_page.unwrap() > second_page.unwrap() {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            valid_updates.push(update.clone());
        }
    }

    // for each of the valid updates, get the middle page
    let mut middle_pages: Vec<isize> = Vec::new();
    for update in &valid_updates {
        let pages = &update.update;
        let middle_page_index = pages[pages.len() / 2];
        middle_pages.push(middle_page_index);
    }

    // Finally, sum the middle pages
    let sum: isize = middle_pages.iter().sum();
    println!("Sum: {}", sum);

    Ok(())
}
