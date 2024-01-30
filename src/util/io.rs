use crate::util::csv_util;
use std::fs::{self, File};
use std::io::{self, prelude::*, BufRead};
use std::path::Path;

use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;
use rayon::prelude::*;

const TEST_DIRECTORY: &str = "./data/resources/test/";


/// Outputs a sample of the file to a new file with the same name as the original file with the prefix "sample_file"
/// # Arguments
/// * `filename` - The name of the file to sample
/// * `take` - The number of lines to take from the file
/// * `skip` - The number of lines to skip from the file
/// # Example
/// ```
/// use ourboro_data::util::io::output_sample;
/// let _ = output_sample(test_case("test_readlines.txt").to_string(), 2, 0);
/// ```
pub fn output_sample(input: String, output: String, take: usize, skip: usize) -> io::Result<()> {
    if let Ok(lines) = read_lines(input.clone()) {
        let vec = lines.flatten().skip(skip).take(take).collect();
        let _ = write_lines_temp(vec, output);
    }
    Ok(())
}

pub fn get_and_cache_filtered_census(input: &String, output: &String, filter: usize) {
    if let Ok(lines) = read_lines(input) {
        let vec = lines
            .flatten()
            .par_bridge()
            .filter(|line| {
                
                let splitted = csv_util::split_csv_line(&line);
                let is_census_subdivision = splitted[3] == "Census division" || splitted[3] == "Census subdivision" || splitted[3] == "Dissemination area";
                return is_census_subdivision && splitted[8] == filter.to_string();
                todo!("Build user-assigned filtration features.")
            })
            .map(|ln: String| {
                let splitted = csv_util::split_csv_line(&ln);
                let mut location = format!("Location: {}", splitted[4]).to_string();
                location.retain(|c| ![','].contains(&c));
                let vector =  vec![splitted[1].to_string(), splitted[3].to_string(), splitted[8].to_string(), location, splitted[11].to_string()].join(",");
                return vector;
            })
            .collect();
        let _ = cache_data_set(output.to_string(), vec);
    }
}

fn cache_data_set(cache_name: String, vector_string: Vec<String>) -> io::Result<()> {
    let _ = write_lines_temp(vector_string, cache_name);
    Ok(())
}

fn collect_from_cache(filename: &str) -> Option<Vec<Vec<String>>> {
    if let Ok(lines) = read_lines(filename) {
        return Some(
            lines
                .flatten()
                .par_bridge()
                .map(|line: String| {
                    let splitted = csv_util::split_csv_line(&line);
                    return splitted;
                })
                .collect(),
        );
    } else {
        return None;
    }
}

fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn write_lines_temp(temp_vec: Vec<String>, filename: String) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(temp_vec.join("\n").as_bytes())?;
    Ok(())
}

mod test {
    use super::*;
    #[test]
    fn test_readlines() {
        let result = read_lines(format!("{}test_readlines.txt", TEST_DIRECTORY ));
        assert!(result.is_ok());
        let lines = result.unwrap();
        let mut count = 0;
        for line in lines {
            assert!(line.is_ok());
            count += 1;
        }
        assert_eq!(count, 3);
    }
}
