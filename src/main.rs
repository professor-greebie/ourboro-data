extern crate ourboro;
use clap::Parser;
use ourboro::util;
use itertools::izip;

//static PCCF_TEMP_FILE: &str = "./data/scratch/pccf_temp.txt";
static CENSUS_DATA_FILE: &str = "98-401-X2021006_English_CSV_data_Ontario.csv";
static OURBORO_DATA_FILE: &str = "homeowner_postal_code_stats.csv";
static INPUT_DIRECTORY: &str = "./data/resources/quick";
static OUTPUT_DIRECTORY: &str = "./data/resources/output";

fn main() {
    //let temp_vec: Vec<(String, String)> = Vec::new();
    let args: ourboro::util::cli::Cli = ourboro::util::cli::Cli::parse();
    let _ourboro = args.ourboro.unwrap_or(false);
    let _vector_of_filters = ourboro::util::io::get_user_defined_sources(&args);
    let _sample = args.sample.unwrap_or(false);
    let _xlsx_to_csv = args.xlsx.unwrap_or("NONE".to_string());
    let _population = args.population.unwrap_or(false);
    let _income = args.income.unwrap_or(false);
    //let pccf_line_file = args.pccf.unwrap_or(pccf_path!().to_string());
    let take = args.take.unwrap_or(2);
    let skip: usize = args.skip.unwrap_or(0);
    let filter_postal = args.postal.map(|filter : String| filter.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_uppercase());
    let filter_province = args.province.map(|filter| filter.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_uppercase());

    
    //pccf_data.iter().for_each(|pc| println!("{:?}", pc.dguid));
    
    // Specific to the ourboro project
    if _ourboro {

        // Create headers for the output file

        let headers = _vector_of_filters.clone().iter().map(|filter| filter.cache_name()).collect::<Vec<String>>();
        let finishing_headers = headers.clone().iter().map(|x| "finishing_".to_string() + x).collect::<Vec<String>>();

        // Create headers for GroupA
        let group_headers = format!("user_id, starting_postal_code, starting_dguid, starting_lat, starting_lon, {}, finishing_postal_code, finishing_dguid, finishing_lat, finishing_starting_lon, {}", headers.join(", "), finishing_headers.join(", ")).split(",").map(|x| x.to_string()).collect::<Vec<String>>();
        //println!("{:?}", group_headers);
        let mut final_res = Vec::new();
        final_res.push(group_headers);
        
        // Create headers for GroupB

        println!("Running ourboro specific code run");
        let input = format!("{}/{}", INPUT_DIRECTORY, OURBORO_DATA_FILE);
        let output = format!("{}/{}", OUTPUT_DIRECTORY, "ourboro_output.csv");
        //println!("{:?}", input);
        let comparator_postal_codes: Vec<Vec<String>> = ourboro::util::io::read_and_clean_csv(&input);
        let ids = comparator_postal_codes.clone().into_iter().map(|x| x[0].clone()).collect::<Vec<String>>();
        let group_a = comparator_postal_codes.clone().into_iter().map(|x| x[1].clone()).collect::<Vec<String>>();
        let group_b = comparator_postal_codes.clone().into_iter().map(|x| x[2].clone()).collect::<Vec<String>>();
        let filtered_pccf_group_a = ourboro::util::io::read_pccf_filtered_postal_code_list(group_a);
        let filtered_pccf_group_b = ourboro::util::io::read_pccf_filtered_postal_code_list(group_b);


        let result_a = ourboro::pccf::processing::process_pccf(&filtered_pccf_group_a, &_vector_of_filters, &input);
        let result_b = ourboro::pccf::processing::process_pccf(&filtered_pccf_group_b, &_vector_of_filters, &input);
        //resultA.iter().for_each(|line| println!("{:?}", line));
        //resultB.iter().for_each(|line| println!("{:?}", line));

        let final_result = izip!(ids.iter(), result_a.iter().skip(1), result_b.iter().skip(1)).map(|(a, b, c)| { 
            let combined = format!("{}, {}, {}", a, b.join(", "), c.join(", ")); 
            combined.split(",").map(|x| x.to_string()).collect::<Vec<String>>()
        }).collect::<Vec<Vec<String>>>();
        for line in final_res.iter() {
            println!("{:?}", line);
        }
        final_res.extend(final_result);
        for line in final_res.iter() {
            println!("{:?}", line);
        }

        let _ = util::csv_util::write_csv(&output, final_res);
    }

    // Output some census data to review information
    if _sample {
        print!("Outputting sample data");
        let output_file = "sample_file.csv";
        let input = format!("{}/{}", INPUT_DIRECTORY, CENSUS_DATA_FILE);
        let output = format!("{}/{}", OUTPUT_DIRECTORY, output_file);
        let _ = util::io::output_sample(input, output_file.to_string(), take, skip);
    }



    if !_vector_of_filters.is_empty() && !_ourboro {
        let pccf_data = ourboro::util::io::read_pccf(filter_postal, filter_province);

        println!("Attempting to create filter");

        let mut result = Vec::new(); 
        let headers = _vector_of_filters.clone().iter().map(|filter| filter.cache_name()).collect::<Vec<String>>().join(", ");
        let header = format!("postal_code, dguid, {}", headers).split(",").map(|x| x.to_string()).collect();
        result.push(header);
        
        for postal_code in pccf_data {
            let mut line = postal_code._postal_code.clone() + ", " + &postal_code._dguid;
            for filter in _vector_of_filters.clone() {
                let input = format!("{}/{}", INPUT_DIRECTORY, CENSUS_DATA_FILE);
                let output = filter.cache_name();
                let dataset: Option<Vec<Vec<String>>> = util::io::get_census_data(&input, &output, filter.filter_column())
                  .map(|option| option.into_iter().filter(|filter| filter[3] == postal_code._dguid).collect());
                let value = dataset.unwrap().iter().map(|filter| filter[4].clone()).collect::<Vec<String>>().join(", ");
                line = line + ", " + &value;
            }
            result.push(line.split(",").map(|x| x.to_string()).collect());
        }

        //result.iter().for_each(|line| println!("{}", line));
        let output_file = format!("{}/{}", OUTPUT_DIRECTORY, "output.csv");
        let _ = util::csv_util::write_csv(&output_file, result);

        
    }}

    //    if xlsx_to_csv.to_path().exists() {
    //      util::csv_util::xlsx_to_csv(&xlsx_to_csv);
    // }

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
