use anyhow::Context;
use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone)]
struct Report {
    levels: Vec<isize>,
}

impl Report {
    fn valid_diffs(&self) -> bool {
        // calculate difference of i and i + 1
        let mut diffs = Vec::new();
        for i in 0..self.levels.len() - 1 {
            diffs.push(self.levels[i + 1] - self.levels[i]);
        }

        // Check that the diffs are within 1 to 3
        diffs.iter().all(|&x| x >= 1 && x <= 3)
    }

    fn same_sign(&self) -> bool {
        // Check if the sign is the same for all diffs, i.e. abs(sum) == len
        let signs = self.levels.iter().map(|&x| x.signum()).collect::<Vec<_>>();
        signs.iter().sum::<isize>().abs() == signs.len() as isize
    }
}

impl<'de> Deserialize<'de> for Report {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize the row as a vector of strings
        let row: Vec<String> = Deserialize::deserialize(deserializer)?;
        // Convert each string to an `isize`
        let levels = row
            .into_iter()
            .map(|field| field.parse::<isize>().map_err(serde::de::Error::custom))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Report { levels })
    }
}

fn main() -> Result<()> {
    let path = "examples/2024_2/input.txt";
    let file = fs::File::open(path).context("Failed to open file")?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .flexible(true)
        .trim(csv::Trim::All) // Trim
        .from_reader(file);

    let mut reports: Vec<Report> = Vec::new();

    for result in rdr.deserialize() {
        let report: Report = result.context("Failed to deserialize row")?;
        reports.push(report);
    }

    dbg!(&reports.len());

    let mut safe_reports: Vec<Report> = Vec::new();

    // iterate over the reports
    for report in reports {
        // calculate difference of i and i + 1

        if report.valid_diffs() && report.same_sign() {
            safe_reports.push(report);
        }
    }

    // 118 is too low
    println!("Safe reports: {}", safe_reports.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe() {
        let report = Report {
            levels: vec![7, 6, 4, 2, 1],
        };

        println!("{:?}", report.valid_diffs());

        assert!(report.valid_diffs());
        assert!(report.same_sign());
    }
    #[test]
    fn test_unsafe() {
        let report = Report {
            levels: vec![1, 2, 7, 8, 9],
        };

        assert!(!report.valid_diffs());
        assert!(!report.same_sign());
    }
}
