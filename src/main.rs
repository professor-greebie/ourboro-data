
mod cli;
mod pccf;
mod util;
mod census;
use std::fs::{self, File};
use std::io::{self, BufRead, prelude::*};
use std::path::Path;
use rayon::prelude::*;
use crate::cli::Cli;
use crate::util::csv_util::head;

const PCCF_TEMP_FILE: &str = "temp_output_file";

use clap::Parser;
fn main() {
    let mut temp_vec: Vec<(String, String)> = Vec::new();
    let args = Cli::parse();
    let sample = args.sample.unwrap_or(false);
    let population = args.population.unwrap_or(false);
    let income = args.income.unwrap_or(false);
    let pccf_line_file = args.pccf.unwrap_or("./PCCF_FCCP_V2312_2021.txt".to_string());
    let take = args.take.unwrap_or(100);
    let skip: usize = args.skip.unwrap_or(0);
    let housing_data_path = "98-401-X2021006_English_CSV_data_Ontario.csv";
    let filter_postal = args.postal.unwrap_or("NONE".to_string()).chars().filter(|c| !c.is_whitespace()).collect::<String>().to_uppercase();
    let filter_province = args.province.unwrap_or("NONE".to_string()).chars().filter(|c| !c.is_whitespace()).collect::<String>().to_uppercase();
    
    if let Ok(lines) = read_lines(housing_data_path) {
        let vec = lines.flatten().skip(skip).take(take).collect();
        let _ = write_lines_temp(vec, "sample_file".to_string());
    }

    if Path::new(PCCF_TEMP_FILE).exists() {
        println!("Reading from temp file");
        if let Ok(lines) = read_lines(PCCF_TEMP_FILE) {
            temp_vec = lines.flatten().par_bridge()
              .map(|line| {
                let splitted = util::csv_util::split_csv_line(&line);
                return (splitted[0].clone(), splitted[1].clone())

            } ).collect();
        }
    } else if let Ok(lines) = read_lines(housing_data_path) {
        temp_vec = lines.flatten().par_bridge()
          .filter(|line| {
            return util::csv_util::split_csv_line(&line)[9] == "Population, 2021"; 
          })
        .map(|ln: String| {
            let splitted = util::csv_util::split_csv_line(&ln);
            return (splitted[1].clone(), splitted[11].clone())

        } ).collect();
        let _ = write_lines_temp(temp_vec.clone().into_iter()
          .map(|a| a.0 + ", " + &a.1).collect(), PCCF_TEMP_FILE.to_string());

        if sample {
            println!("sample: {:?}", head(temp_vec.clone(), take));
        }
}

    if let Ok(lines_pccf) = read_lines(pccf_line_file) {
        println!("Reading PCCF File");
        let population_vector = lines_pccf.skip(skip).flatten().par_bridge()
          .filter(|line| {
            return pccf::postal_code::is_filtered_postal_code(&line, &filter_postal) && pccf::postal_code::is_filtered_province(&line, &filter_province); 
          }).map(|line_pccf|
            pccf::postal_code::create_postal_code(&line_pccf))
              .filter(|pc| {
                return temp_vec.clone().into_iter().filter(|(dguid, _)| {
                    dguid[9..].contains(&pc.dguid)}
                ).collect::<Vec<(String, String)>>().len() > 0;
            }).map(|pc| {
                let population_vec = temp_vec.clone().into_iter().filter(|(dguid, _)| {
                    dguid[9..].contains(&pc.dguid)}
                ).map(|(_, population)| {
                    population.trim().to_string()
                }).collect::<Vec<String>>().to_vec();
                let census_item = census::census_model::CensusPopulationStruct {
                    postal_code: pc.postal_code.clone(),
                    dguid: pc.dguid.clone(),
                    population: population_vec,
                };
                return census_item;
            }).collect::<Vec<census::census_model::CensusPopulationStruct>>();
        let _ = write_lines_temp(population_vector.into_iter()
          .map(|a| a.postal_code + ", " + &a.dguid + ", " + &a.population.join(", ").to_string()).collect(), "population_file".to_string());
        // Consumes the iterator, returns an (Optional) String
        
    }

    
    
    //let verbose = args.verbose.unwrap_or(false);
    //let output = args.output.unwrap_or("./output.csv".to_string());
    //let contents = fs::read_to_string(pccf_line).expect("Something went wrong reading the file");
    //println!("With text:\n{}", contents);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn write_lines_temp(temp_vec: Vec<String>, filename: String) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(temp_vec.join("\n").as_bytes())?;
    Ok(())
}