use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::time::Instant;

use anyhow::{anyhow, Result};

const INPUT_FILE_PATH: &str = "input.txt";

fn main() -> Result<()> {
    let start = Instant::now();
    let input_file: File = File::open(INPUT_FILE_PATH)?;
    let calibration_document = CalibrationDocument::read(&mut BufReader::new(input_file))?;
    let calibration_values = CalibrationValues::try_from(&calibration_document)?;
    let sum = calibration_values.sum();
    let end = Instant::now();
    println!("{sum}");
    println!("Calculated sum in {} µs", (end - start).as_micros());
    Ok(())
}

struct CalibrationValues(Vec<u8>);

impl CalibrationValues {
    pub fn new(values: Vec<u8>) -> Self {
        Self(values)
    }

    pub fn sum(&self) -> u32 {
        self.0.iter().copied().map(|b| b as u32).sum()
    }
}

impl TryFrom<&CalibrationDocument> for CalibrationValues {
    type Error = anyhow::Error;

    fn try_from(document: &CalibrationDocument) -> Result<Self, Self::Error> {
        let mut values = Vec::with_capacity(document.lines.len());
        for line in &document.lines {
            let mut digits = (None, None);
            for c in line.iter().copied() {
                if c.is_ascii_digit() {
                    match digits {
                        (None, None) => {
                            digits.0 = Some(c - b'0');
                        }
                        (Some(_), None) | (Some(_), Some(_)) => {
                            digits.1 = Some(c - b'0');
                        }
                        _ => unreachable!()
                    }
                }
            }
            let digits = match digits {
                (Some(d1), Some(d2)) => (d1, d2),
                (Some(d), None) => (d, d),
                _ => return Err(anyhow!("Could not find any digit in line {:?}", line))
            };
            let value = digits.0 * 10 + digits.1;
            values.push(value);
        }
        Ok(Self::new(values))
    }
}

struct CalibrationDocument {
    lines: Vec<Vec<u8>>,
}

impl CalibrationDocument {
    pub fn new(lines: Vec<Vec<u8>>) -> Self {
        Self {
            lines
        }
    }

    pub fn read(reader: &mut impl BufRead) -> Result<Self, std::io::Error> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        loop {
            let read = reader.read_line(&mut current_line)?;
            if read == 0 {
                break;
            }
            lines.push(current_line.clone().into_bytes());
            current_line.clear();
        }
        Ok(Self::new(lines))
    }
}
