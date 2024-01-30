
mod cli;
mod pccf;
mod util;
mod census;
use std::{fmt::format, path::PathBuf};

const PCCF_TEMP_FILE: &str = "./data/scratch/pccf_temp.txt";
const CENSUS_DATA_FILE: &str = "98-401-X2021006_English_CSV_data_Ontario.csv";
const INPUT_DIRECTORY: &str = "./data/resources/quick";
const OUTPUT_DIRECTORY: &str = "./data/resources/output";

use clap::Parser;
fn main() {
    //let temp_vec: Vec<(String, String)> = Vec::new();
    let args = cli::Cli::parse();
    let sample = args.sample.unwrap_or(false);
    let xlsx_to_csv = args.xlsx.unwrap_or(PathBuf::from("NONE"));
    let _population = args.population.unwrap_or(false);
    let _income = args.income.unwrap_or(false);
    //let pccf_line_file = args.pccf.unwrap_or(pccf_path!().to_string());
    let take = args.take.unwrap_or(2);
    let skip: usize = args.skip.unwrap_or(0);
    let census_data_path = "/98-401-X2021006_English_CSV_data_Ontario.csv";
    //let filter_postal = args.postal.unwrap_or("NONE".to_string()).chars().filter(|c| !c.is_whitespace()).collect::<String>().to_uppercase();
    //let filter_province = args.province.unwrap_or("NONE".to_string()).chars().filter(|c| !c.is_whitespace()).collect::<String>().to_uppercase();
    

    // Output some census data to review information
    if sample {
        print!("Outputting sample data");
        let output_file = "sample_file.csv";
        let input = format!("{}/{}", INPUT_DIRECTORY, CENSUS_DATA_FILE);
        let output = format!("{}/{}", OUTPUT_DIRECTORY, output_file);
        let _ = util::io::output_sample(input, output,take, skip);
    }

    if _population {
        let population_census_filter: census::census_model::CensusFilter = census::census_model::CensusFilter::Population2021; 
        let input = format!("{}/{}", INPUT_DIRECTORY, CENSUS_DATA_FILE);
        let output = format!("{}/{}", OUTPUT_DIRECTORY, population_census_filter.cache_name());
        let _ = util::io::get_and_cache_filtered_census(&input, &output, population_census_filter.clone().filterColumn());
    }

    if xlsx_to_csv.exists() {
        util::csv_util::xlsx_to_csv(&xlsx_to_csv.as_path().to_str().unwrap());
    }

}

//    if Path::new(PCCF_TEMP_FILE).exists() {
//        println!("Reading from temp file");
//        if let Ok(lines) = util::io::read_lines(PCCF_TEMP_FILE) {
//            temp_vec = lines.flatten().par_bridge()
//              .map(|line| {
//                let splitted = util::csv_util::split_csv_line(&line);
//                return (splitted[0].clone(), splitted[1].clone())
//
//            } ).collect();
//        }
//    } else if let Ok(lines) =util::io::read_lines(census_data_path) {
//        temp_vec = lines.flatten().par_bridge()
//          .filter(|line| {
//            return util::csv_util::split_csv_line(&line)[9] == "Population, 2021"; 
//          })
//        .map(|ln: String| {
//            let splitted = util::csv_util::split_csv_line(&ln);
//          return (splitted[1].clone(), splitted[11].clone())
//
//        } ).collect();
//        let _ = write_lines_temp(temp_vec.clone().into_iter()
//          .map(|a| a.0 + ", " + &a.1).collect(), PCCF_TEMP_FILE.to_string());
//
//}

//    if let Ok(lines_pccf) = read_lines(pccf_line_file) {
//        println!("Reading PCCF File");
//        let population_vector = lines_pccf.skip(skip).flatten().par_bridge()
//          .filter(|line| {
//            return pccf::postal_code::is_filtered_postal_code(&line, &filter_postal) && pccf::postal_code::is_filtered_province(&line, &filter_province); 
//          }).map(|line_pccf|
//            pccf::postal_code::create_postal_code(&line_pccf))
//              .filter(|pc| {
//                return temp_vec.clone().into_iter().filter(|(dguid, _)| {
//                    dguid[9..].contains(&pc.dguid)}
//                ).collect::<Vec<(String, String)>>().len() > 0;
//            }).map(|pc| {
//                let population_vec = temp_vec.clone().into_iter().filter(|(dguid, _)| {
//                    dguid[9..].contains(&pc.dguid)}
//                ).map(|(_, population)| {
//                    population.trim().to_string()
//                }).collect::<Vec<String>>().to_vec();
//                let census_item = census::census_model::CensusPopulationStruct {
//                    postal_code: pc.postal_code.clone(),
//                   dguid: pc.dguid.clone(),
//                    population: population_vec,
//                };
//                return census_item;
//            }).collect::<Vec<census::census_model::CensusPopulationStruct>>();
//      let _ = write_lines_temp(population_vector.into_iter()
//          .map(|a| a.postal_code + ", " + &a.dguid + ", " + &a.population.join(", ").to_string()).collect(), "population_file".to_string());
//        // Consumes the iterator, returns an (Optional) String
        
//    }

    
    
    //let verbose = args.verbose.unwrap_or(false);
    //let output = args.output.unwrap_or("./output.csv".to_string());
    //let contents = fs::read_to_string(pccf_line).expect("Something went wrong reading the file");
    //println!("With text:\n{}", contents);
