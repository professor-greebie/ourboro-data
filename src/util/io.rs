use crate::census::census_model::CensusFilter;
use crate::pccf::PostalCode;
use crate::util::csv_util;
use crate::util::cli::Cli;
use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::{self, prelude::*, BufRead};
use std::path::Path;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;


const TEMP_DIRECTORY: &str = "data/resources/tmp/";
//const OUTPUT_DIRECTORY: &str = "data/resources/output/";
const PCCF_TEMP_FILE: &str = "./data/resources/quick/PCCF_FCCP_V2312_2021.txt";

pub fn read_and_clean_csv(path: &str) -> Vec<Vec<String>> {
    let mut result = VecDeque::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten() {
            let cleaned: Vec<String> = csv_util::split_csv_line(&line).into_iter().map(|s| s.to_string().replace(" ", "")).collect();
            result.push_back(cleaned);
        }
    }
    result.pop_front();
    Vec::from(result)
}


pub fn read_pccf(postal_filter : Option<String>, province_filter : Option<String>) -> Vec<PostalCode> {
    let mut temp_vec: Vec<PostalCode> = Vec::new();
    if Path::new(PCCF_TEMP_FILE).exists() {
        if let Ok(lines) = read_lines(PCCF_TEMP_FILE) {
            temp_vec = lines
                .flatten()
                .par_bridge()
                .filter(|line| {
                    PostalCode::is_filtered_postal_code(&line, postal_filter.clone()) && PostalCode::is_filtered_province(&line, province_filter.clone())
                })
                .map(|line| {
                    PostalCode::from(&line)
                })
                .collect();
        }
    }
    temp_vec

}

pub fn read_pccf_filtered_postal_code_list(postal_filters: Vec<String>) -> Vec<PostalCode> {
    let mut temp_vec: Vec<PostalCode> = Vec::new();
    if Path::new(PCCF_TEMP_FILE).exists() {
        if let Ok(lines) = read_lines(PCCF_TEMP_FILE) {
            temp_vec = lines
                .flatten()
                .par_bridge()
                .filter(|line| {
                    PostalCode::is_filtered_postal_code_list(&line, &postal_filters)
                })
                .map(|line| {
                    PostalCode::from(&line)
                })
                .collect();
        }
    }
    temp_vec

}

/// Get the user-defined sources from the command line arguments
/// # Arguments
/// * `args` - The command line arguments
/// # Example
/// ```rust
/// use ourboro_data::util::cli::Cli;
/// use ourboro_data::util::io::get_user_defined_sources;
/// let args = Cli {
///    pccf: Some("test".to_string()),
///   income: Some(true),
///  sample: Some(true),
/// population: Some(true),
/// land_area: Some(true),
/// total_occupied_dwellings: Some(true),
/// total_single_detached_houses: Some(true),
/// take: Some(2),
/// skip: Some(0),
/// verbose: Some(true),
/// output: Some("test".to_string()),
/// xlsx: Some("test".to_string()),
/// };
/// let result = get_user_defined_sources(&args);
/// assert_eq!(result.len(), 4); 
/// 

pub fn get_user_defined_sources(args : &Cli) -> Vec<CensusFilter> {
    let mut census_filters: Vec<CensusFilter> = Vec::new();
    if args.all.unwrap_or(false) {
        census_filters.push(CensusFilter::Population2021);
        census_filters.push(CensusFilter::Population2016);
        census_filters.push(CensusFilter::LandArea);
        census_filters.push(CensusFilter::TotalOccupiedDwellings);
        census_filters.push(CensusFilter::TotalSingleDetachedHouses);
        census_filters.push(CensusFilter::MedianAfterTaxIncome);
        census_filters.push(CensusFilter::Population2016);
        census_filters.push(CensusFilter::TotalSemiDetachedHouses);
        census_filters.push(CensusFilter::TotalRowHouses);
        census_filters.push(CensusFilter::TotalApartmentDuplex);
        census_filters.push(CensusFilter::TotalApartmentBuildingLessThan5Stories);
        census_filters.push(CensusFilter::TotalApartmentBuilding5StoriesOrMore);
        census_filters.push(CensusFilter::TotalOtherDwellings);
        census_filters.push(CensusFilter::TotalMovableDwellings);
        census_filters.push(CensusFilter::AverageAfterTaxIncome);
        census_filters.push(CensusFilter::MedianTotalIncomeEconomicFamily);
        census_filters.push(CensusFilter::AverageTotalIncomeEconomicFamily);
        census_filters.push(CensusFilter::AverageAfterTaxIncomeOneParent);
        census_filters.push(CensusFilter::MedianTotalIncomeOneParent);
        census_filters.push(CensusFilter::TotalCensusFamiliesPrivateHouseHolds);
        census_filters.push(CensusFilter::MarriedWithChildren);
        census_filters.push(CensusFilter::CommonLawWithChildren);
        census_filters.push(CensusFilter::OneParentFamilies);
        census_filters.push(CensusFilter::AverageTotalChildrenPerFamily);
        census_filters.push(CensusFilter::LowIncomeMeasureAfterTax);
        census_filters.push(CensusFilter::GiniIndexOnAfterTaxIncome);
        census_filters.push(CensusFilter::HouseholdsSpending30PercentOrMoreOfIncomeOnShelter);
        census_filters.push(CensusFilter::ImmigrantStatusImmigrant);
        census_filters.push(CensusFilter::ImmigrantStatusNonImmigrant);

    }
    if args.population.unwrap_or(false) {
        census_filters.push(CensusFilter::Population2021);
    }
    if args.land_area.unwrap_or(false) {
        census_filters.push(CensusFilter::LandArea);
    }
    if args.total_occupied_dwellings.unwrap_or(false) {
        census_filters.push(CensusFilter::TotalOccupiedDwellings);
    }
    if args.total_single_detached_houses.unwrap_or(false) {
        census_filters.push(CensusFilter::TotalSingleDetachedHouses);
    }
    if args.income.unwrap_or(false) {
        census_filters.push(CensusFilter::MedianAfterTaxIncome);
    }
    if args.population.unwrap_or(false) {
        census_filters.push(CensusFilter::Population2021);
    }


    census_filters
}


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
        let tmp = format!("{}{}", TEMP_DIRECTORY, output);
        let vec = lines.flatten().skip(skip).take(take).collect();
        let _ = write_lines_temp(vec, tmp);
    }
    Ok(())
}

pub fn get_census_data(input: &String, output: &String, filter: usize) -> Option<Vec<Vec<String>>> {
    let tmp_vec = format!("{}{}", TEMP_DIRECTORY, output);
    //let output_vec = format!("{}/{}", OUTPUT_DIRECTORY, output);
    if !Path::new(&tmp_vec).exists() {
        let _ = get_and_cache_filtered_census(input, output, filter);
    }
    let result = collect_from_cache(&output);
    result
}


fn get_and_cache_filtered_census(input: &String, output: &String, filter: usize) {
    let tmp_vec = format!("{}{}", TEMP_DIRECTORY, output);
    if let Ok(lines) = read_lines(input) {
        let vec = lines
            .flatten()
            .par_bridge()
            .filter(|line| { 
                let splitted = csv_util::split_csv_line(&line);
                let is_census_subdivision = splitted[3] == "Dissemination area";
                is_census_subdivision && splitted[8] == filter.to_string()
            })
            .map(|ln: String| {
                let splitted = csv_util::split_csv_line(&ln);
                let mut location = format!("{}", splitted[4]).to_string();
                location.retain(|c| ![','].contains(&c));
                let vector =  vec![splitted[1].to_string(), splitted[3].to_string(), splitted[8].to_string(), location, splitted[11].to_string()].join(",");
                vector
            })
            .collect();
        let _ = cache_data_set(tmp_vec, vec);
    }
}

fn cache_data_set(cache_name: String, vector_string: Vec<String>) -> io::Result<()> {
    let _ = write_lines_temp(vector_string, cache_name);
    Ok(())
}

fn collect_from_cache(filename: &str) -> Option<Vec<Vec<String>>> {
    if let Ok(lines) = read_lines(format!("{}/{}", TEMP_DIRECTORY, filename)) {
        return Some(
            lines
                .flatten()
                .par_bridge()
                .map(|line: String| {
                    let splitted = csv_util::split_csv_line(&line);
                    splitted
                })
                .collect(),
        );
    } 
    None
}

fn read_lines<P>(filename: P) -> std::io::Result<std::io::Lines<std::io::BufReader<fs::File>>>
where
    P: AsRef<Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


pub fn write_lines_temp(temp_vec: Vec<String>, filename: String) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(temp_vec.join("\n").as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_readlines() {
        let test_directory = "data/resources/test/";
        let result = read_lines(format!("{}test_readlines.txt", test_directory ));
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
