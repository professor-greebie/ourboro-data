
use crate::census::census_model::CensusFilter;
use crate::util::io;

use super::PostalCode;

pub fn process_pccf(codes: &Vec<PostalCode>, filters: &Vec<CensusFilter>, input: &String) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    let headers: String = filters.iter().map(|filter| filter.cache_name()).collect::<Vec<String>>().join(", ");
    let header = format!("postal_code, dguid, {}", headers).split(",").map(|x| x.to_string()).collect();
    result.push(header);
    for postal_code in codes {
        let mut line = postal_code._postal_code.clone() + ", " + &postal_code._dguid;
        for filter in filters.clone() {
            let output = filter.cache_name();
            let dataset: Option<Vec<Vec<String>>> = io::get_census_data(&input, &output, filter.filter_column())
              .map(|option| option.into_iter().filter(|filter| filter[3] == postal_code._dguid).collect());
            let value = dataset.unwrap().iter().map(|filter| filter[4].clone()).collect::<Vec<String>>().join(", ");
            line = line + ", " + &value;
        }
        result.push(line.split(",").map(|x| x.to_string()).collect());
    }
    result

}