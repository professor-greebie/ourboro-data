const POSTAL_CODE_END: usize = 6;
const FORWARD_SORTATION_AREA_END: usize = 9;
const PROVINCE_END: usize = 11;
const CENSUS_DIVISION_ID_END: usize = 15;
const CENSUS_SUBDIVISION_ID_END: usize = 22;
const CENSUS_SUBDIVISION_NAME_END: usize = 92;
const CENSUS_SUBDIVISION_TYPE_END: usize = 95;
const CENSUS_SUBDIVISION_CODE_END: usize = 98;
const STATISTICAL_AREA_CLASS_END: usize = 101;
const STATISTICAL_AREA_TYPE_END: usize = 102;
const CENSUS_TRACT_NAME_END: usize = 109;
const ECONOMIC_REGION_CODE_END: usize = 111;
const DESIGNATED_PLACE_CODE_END: usize = 115;
const FEDERAL_ELECTORAL_DISTRICT_CODE_END: usize = 120;
const POPULATION_CENTER_RA_END: usize = 124;
const POPULATION_CENTER_RA_TYPE_END: usize = 125;
const DISSEMINATION_AREA_ID_END: usize = 133;
const DISSEMINATION_BLOCK_CODE_END: usize = 136;
const REPRESENTATIVE_POINT_TYPE_END: usize = 137;
const POINT_LATITUDE_END: usize = 148;
const POINT_LONGITUDE_END: usize = 161;
const SINGLE_LINK_INDICATOR_END: usize = 162;
const PC_TYPE_END: usize = 163;
const COMMUNITY_NAME_END: usize = 193;
const DELIVERY_MODE_END: usize = 194;
const HISTORIC_DELIVERY_MODE_END: usize = 195;
const BIRTH_DATE_END: usize = 203;
const RETIRED_DATE_END: usize = 211;
const DELIVERY_INSTALLATION_END: usize = 212;
const QUALITY_INDICATOR_END: usize = 215;
const SOURCE_GEO_END: usize = 216;
const POPULATION_CENTRE_AND_RURAL_AREA_INDICATOR_END: usize = 217;

#[derive(Clone, Debug)]
pub struct PostalCode {
    pub _postal_code: String,
    _forward_sortation_area: String,
    _province: String,
    _province_iso3166_2: String,
    _census_division_id: String,
    _census_subdivision_id: String,
    _census_subdivision_name: String,
    _census_subdivision_type: String,
    _census_subdivision_code: String,
    _statistical_area_class: String,
    _statistical_area_type: String,
    _census_tract_name: String,
    _economic_region_code: String,
    _designated_place_code: String,
    _federal_electoral_district_code: String,
    _population_center_ra: String,
    _population_center_ra_type: String,
    _dissemation_area_id: String,
    _dissemation_block_code: String,
    _representative_point_type: String,
    pub _point_latitude: String,
    pub _point_longitude: String,
    _single_link_indicator: String,
    _pc_type: String,
    _community_name: String,
    _delivery_mode: String,
    _historic_delivery_mode: String,
    _birth_date: String,
    _retired_date: String,
    _delivery_installation: String,
    _quality_indicator: String,
    _source_geo: String,
    _population_centre_and_rural_area_indicator: String,
    pub _dguid: String,
}

impl PostalCode {
    pub fn empty() -> Self {
        Self {
            _postal_code: "NA".to_string(),
            _forward_sortation_area: "".to_string(),
            _province: "".to_string(),
            _province_iso3166_2: "".to_string(),
            _census_division_id: "".to_string(),
            _census_subdivision_id: "".to_string(),
            _census_subdivision_name: "".to_string(),
            _census_subdivision_type: "".to_string(),
            _census_subdivision_code: "".to_string(),
            _statistical_area_class: "".to_string(),
            _statistical_area_type: "".to_string(),
            _census_tract_name: "".to_string(),
            _economic_region_code: "".to_string(),
            _designated_place_code: "".to_string(),
            _federal_electoral_district_code: "".to_string(),
            _population_center_ra: "".to_string(),
            _population_center_ra_type: "".to_string(),
            _dissemation_area_id: "".to_string(),
            _dissemation_block_code: "".to_string(),
            _representative_point_type: "".to_string(),
            _point_latitude: "".to_string(),
            _point_longitude: "".to_string(),
            _single_link_indicator: "".to_string(),
            _pc_type: "".to_string(),
            _community_name: "".to_string(),
            _delivery_mode: "".to_string(),
            _historic_delivery_mode: "".to_string(),
            _birth_date: "".to_string(),
            _retired_date: "".to_string(),
            _delivery_installation: "".to_string(),
            _quality_indicator: "".to_string(),
            _source_geo: "".to_string(),
            _population_centre_and_rural_area_indicator: "".to_string(),
            _dguid: "".to_string(),
        }
    }
    pub fn from(line: &String) -> Self {
        let province = line[FORWARD_SORTATION_AREA_END..PROVINCE_END].to_string();
        let census_subdivision_id = line[CENSUS_DIVISION_ID_END..CENSUS_SUBDIVISION_ID_END].to_string();
        let dissemation_area_id = line[POPULATION_CENTER_RA_TYPE_END..DISSEMINATION_AREA_ID_END].to_string();
        Self {
          _postal_code : line[..POSTAL_CODE_END].to_string(),
          _forward_sortation_area : line[POSTAL_CODE_END..FORWARD_SORTATION_AREA_END].to_string(),
          _province : province.clone().to_string(),
          _province_iso3166_2 : fssa_province_to_iso3166_2(&province),
          _census_division_id  : line[PROVINCE_END..CENSUS_DIVISION_ID_END].to_string(),
          _census_subdivision_id  :
            census_subdivision_id.clone().to_string(),
          _census_subdivision_name  :
            line[CENSUS_SUBDIVISION_ID_END..CENSUS_SUBDIVISION_NAME_END].to_string() ,
          _census_subdivision_type  :
            line[CENSUS_SUBDIVISION_NAME_END..CENSUS_SUBDIVISION_TYPE_END].to_string() ,
          _census_subdivision_code  :
            line[CENSUS_SUBDIVISION_TYPE_END..CENSUS_SUBDIVISION_CODE_END].to_string() ,
          _statistical_area_class  :
            line[CENSUS_SUBDIVISION_CODE_END..STATISTICAL_AREA_CLASS_END].to_string() ,
          _statistical_area_type  :
            line[STATISTICAL_AREA_CLASS_END..STATISTICAL_AREA_TYPE_END].to_string() ,
          _census_tract_name  : line[STATISTICAL_AREA_TYPE_END..CENSUS_TRACT_NAME_END].to_string() ,
          _economic_region_code  :
            line[CENSUS_TRACT_NAME_END..ECONOMIC_REGION_CODE_END].to_string() ,
          _designated_place_code  :
            line[ECONOMIC_REGION_CODE_END..DESIGNATED_PLACE_CODE_END].to_string() ,
          _federal_electoral_district_code  :
            line[DESIGNATED_PLACE_CODE_END..FEDERAL_ELECTORAL_DISTRICT_CODE_END].to_string() ,
          _population_center_ra  :
            line[FEDERAL_ELECTORAL_DISTRICT_CODE_END..POPULATION_CENTER_RA_END].to_string() ,
          _population_center_ra_type  :
            line[POPULATION_CENTER_RA_END..POPULATION_CENTER_RA_TYPE_END].to_string() ,
          _dissemation_area_id  :
            dissemation_area_id.clone().to_string() ,
          _dissemation_block_code  :
            line[DISSEMINATION_AREA_ID_END..DISSEMINATION_BLOCK_CODE_END].to_string() ,
          _representative_point_type  :
            line[DISSEMINATION_BLOCK_CODE_END..REPRESENTATIVE_POINT_TYPE_END].to_string() ,
          _point_latitude  : line[REPRESENTATIVE_POINT_TYPE_END..POINT_LATITUDE_END].to_string() ,
          _point_longitude  : line[POINT_LATITUDE_END..POINT_LONGITUDE_END].to_string() ,
          _single_link_indicator  :
            line[POINT_LONGITUDE_END..SINGLE_LINK_INDICATOR_END].to_string() ,
          _pc_type  : line[SINGLE_LINK_INDICATOR_END..PC_TYPE_END].to_string() ,
          _community_name  : line[PC_TYPE_END..COMMUNITY_NAME_END].to_string() ,
          _delivery_mode  : line[COMMUNITY_NAME_END..DELIVERY_MODE_END].to_string() ,
          _historic_delivery_mode  :
            line[DELIVERY_MODE_END..HISTORIC_DELIVERY_MODE_END].to_string() ,
          _birth_date  : line[HISTORIC_DELIVERY_MODE_END..BIRTH_DATE_END].to_string() ,
          _retired_date  : line[BIRTH_DATE_END..RETIRED_DATE_END].to_string() ,
          _delivery_installation  : line[RETIRED_DATE_END..DELIVERY_INSTALLATION_END].to_string() ,
          _quality_indicator  : line[DELIVERY_INSTALLATION_END..QUALITY_INDICATOR_END].to_string() ,
          _source_geo  : line[QUALITY_INDICATOR_END..SOURCE_GEO_END].to_string() ,
          _population_centre_and_rural_area_indicator  :
            line[SOURCE_GEO_END..POPULATION_CENTRE_AND_RURAL_AREA_INDICATOR_END].to_string() ,
          _dguid  : dissemation_area_id.clone().to_string(),
    }
}
pub fn is_filtered_postal_code(line: &String, filter: Option<String>) -> bool {
    let postal_code: String = line[..POSTAL_CODE_END].to_string();
    if filter.is_none() {
        return true;
    }
    let filter_real = filter.unwrap(); 
    postal_code[..filter_real.len()].contains(&filter_real)
}

pub fn is_filtered_postal_code_list(line: &String, filter: &Vec<String>) -> bool {
    let postal_code: String = line[..POSTAL_CODE_END].to_string();
    if filter.is_empty() {
        return true;
    }
    filter.iter().any(|f| postal_code[..f.len()].contains(f))
}

pub fn is_filtered_province(line: &String, filter: Option<String>) -> bool {
    let province: String = line[FORWARD_SORTATION_AREA_END..PROVINCE_END].to_string();
    if filter.is_none() {
        return true;
    }
    let filter_real = filter.unwrap();
    province == filter_real ||  fssa_province_to_iso3166_2(&province) == filter_real
}
}

pub fn fssa_province_to_iso3166_2(fsa: &String) -> String {
    match fsa.as_str() {
        "10" => "NL".to_string(),
        "11" => "PE".to_string(),
        "12" => "NS".to_string(),
        "13" => "NB".to_string(),
        "24" => "QC".to_string(),
        "35" => "ON".to_string(),
        "46" => "MB".to_string(),
        "47" => "SK".to_string(),
        "48" => "AB".to_string(),
        "59" => "BC".to_string(),
        "60" => "YK".to_string(),
        "61" => "NT".to_string(),
        "62" => "NU".to_string(),
        _ => "XX".to_string(),
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_filters_a_line_by_postal_code() {
        let test_line: String =
            "A0A1A0FSA24CDUI_CSDUI__seventy_characters_reserved_for_the_census_subdivision_"
                .to_string();
        let filter_true: Option<String> = Some("A0A".to_string());
        let filter_false: Option<String> = Some("A0B".to_string());
        let filter_true_short = Some("A0".to_string());
        let filter_false_short = Some("A1".to_string());
        let actual_true = PostalCode::is_filtered_postal_code(&test_line, filter_true);
        let actual_false = PostalCode::is_filtered_postal_code(&test_line, filter_false);
        let actual_true_short = PostalCode::is_filtered_postal_code(&test_line, filter_true_short);
        let actual_false_short = PostalCode::is_filtered_postal_code(&test_line.clone(), filter_false_short);
        assert!(actual_true);
        assert!(!actual_false);
        assert!(actual_true_short);
        assert!(!actual_false_short);
    }

    #[test]
    fn it_filters_a_line_by_province() {
        let test_line: String =
            "A0A1A0FSA24CDUI_CSDUI__seventy_characters_reserved_for_the_census_subdivision_"
                .to_string();
        let filter_true: Option<String> = Some("QC".to_string());
        let filter_false: Option<String> = Some("PE".to_string());
        let actual_true = PostalCode::is_filtered_province(&test_line, filter_true);
        let actual_false = PostalCode::is_filtered_province(&test_line, filter_false);
        assert!(actual_true);
        assert!(!actual_false);
    }

    #[test]
    fn it_parses_a_line() {
        let test_line: String =
            "A0A1A0FSA24CDUI_CSDUI__seventy_characters_reserved_for_the_census_subdivision_\
    that_include__CDTCCSSACXCTNAME_ERDPL_FED13POPCP_DAuid__DB_\
    ._LATITUDE___LONGITUDE___.+_COMMUNITY_NAME______________/|H_B_DATE_RETIREDD$QI_@#"
                .to_string();
        dbg!(test_line.len());
        let postal_code_struct = PostalCode::from(&test_line);
        let expected = "A0A1A0";
        let actual = postal_code_struct._postal_code;
        assert_eq!(expected, actual);
        let fsa_expected = "FSA";
        let fsa_actual = postal_code_struct._forward_sortation_area;
        assert_eq!(fsa_expected, fsa_actual);
        let province_expected = "24";
        let province_actual = postal_code_struct._province;
        assert_eq!(province_expected, province_actual);
        let province_iso3166_2_expected = "QC";
        let province_iso3166_2_actual = postal_code_struct._province_iso3166_2;
        assert_eq!(province_iso3166_2_expected, province_iso3166_2_actual);
        let census_division_id_expected = "CDUI";
        let census_division_id_actual = postal_code_struct._census_division_id;
        assert_eq!(census_division_id_expected, census_division_id_actual);
        let census_subdivision_id_expected = "_CSDUI_";
        let census_subdivision_id_actual = postal_code_struct._census_subdivision_id;
        assert_eq!(census_subdivision_id_expected, census_subdivision_id_actual);
        let census_subdivision_name_expected =
            "_seventy_characters_reserved_for_the_census_subdivision_\
        that_include__";
        let census_subdivision_name_actual = postal_code_struct._census_subdivision_name;
        assert_eq!(
            census_subdivision_name_expected,
            census_subdivision_name_actual
        );
        let census_subdivision_type_expected = "CDT";
        let census_subdivision_type_actual = postal_code_struct._census_subdivision_type;
        assert_eq!(
            census_subdivision_type_expected,
            census_subdivision_type_actual
        );
        let census_subdivision_code_expected = "CCS";
        let census_subdivision_code_actual = postal_code_struct._census_subdivision_code;
        assert_eq!(
            census_subdivision_code_expected,
            census_subdivision_code_actual
        );
        let statistical_area_class_expected = "SAC";
        let statistical_area_class_actual = postal_code_struct._statistical_area_class;
        assert_eq!(
            statistical_area_class_expected,
            statistical_area_class_actual
        );
        let statistical_area_type_expected = "X";
        let statistical_area_type_actual = postal_code_struct._statistical_area_type;
        assert_eq!(statistical_area_type_expected, statistical_area_type_actual);
        let census_tract_name_expected = "CTNAME_";
        let census_tract_name_actual = postal_code_struct._census_tract_name;
        assert_eq!(census_tract_name_expected, census_tract_name_actual);
        let economic_region_code_expected = "ER";
        let economic_region_code_actual = postal_code_struct._economic_region_code;
        assert_eq!(economic_region_code_expected, economic_region_code_actual);
        let designated_place_code_expected = "DPL_";
        let designated_place_code_actual = postal_code_struct._designated_place_code;
        assert_eq!(designated_place_code_expected, designated_place_code_actual);
        let federal_electoral_district_code_expected = "FED13";
        let federal_electoral_district_code_actual =
            postal_code_struct._federal_electoral_district_code;
        assert_eq!(
            federal_electoral_district_code_expected,
            federal_electoral_district_code_actual
        );
        let population_center_ra_expected = "POPC";
        let population_center_ra_actual = postal_code_struct._population_center_ra;
        assert_eq!(population_center_ra_expected, population_center_ra_actual);
        let population_center_ra_type_expected = "P";
        let population_center_ra_type_actual = postal_code_struct._population_center_ra_type;
        assert_eq!(
            population_center_ra_type_expected,
            population_center_ra_type_actual
        );
        let dissemation_area_id_expected = "_DAuid__";
        let dissemation_area_id_actual = postal_code_struct._dissemation_area_id;
        assert_eq!(dissemation_area_id_expected, dissemation_area_id_actual);
        let dissemation_block_code_expected = "DB_";
        let dissemation_block_code_actual = postal_code_struct._dissemation_block_code;
        assert_eq!(
            dissemation_block_code_expected,
            dissemation_block_code_actual
        );
        let representative_point_type_expected = ".";
        let representative_point_type_actual = postal_code_struct._representative_point_type;
        assert_eq!(
            representative_point_type_expected,
            representative_point_type_actual
        );
        let point_latitude_expected = "_LATITUDE__";
        let point_latitude_actual = postal_code_struct._point_latitude;
        assert_eq!(point_latitude_expected, point_latitude_actual);
        let point_longitude_expected = "_LONGITUDE___";
        let point_longitude_actual = postal_code_struct._point_longitude;
        assert_eq!(point_longitude_expected, point_longitude_actual);
        let single_link_indicator_expected = ".";
        let single_link_indicator_actual = postal_code_struct._single_link_indicator;
        assert_eq!(single_link_indicator_expected, single_link_indicator_actual);
        let pc_type_expected = "+";
        let pc_type_actual = postal_code_struct._pc_type;
        assert_eq!(pc_type_expected, pc_type_actual);
        let community_name_expected = "_COMMUNITY_NAME______________/";
        let community_name_actual = postal_code_struct._community_name;
        assert_eq!(community_name_expected, community_name_actual);
        let delivery_mode_expected = "|";
        let delivery_mode_actual = postal_code_struct._delivery_mode;
        assert_eq!(delivery_mode_expected, delivery_mode_actual);
        let historic_delivery_mode_expected = "H";
        let historic_delivery_mode_actual = postal_code_struct._historic_delivery_mode;
        assert_eq!(
            historic_delivery_mode_expected,
            historic_delivery_mode_actual
        );
        let birth_date_expected = "_B_DATE_";
        let birth_date_actual = postal_code_struct._birth_date;
        assert_eq!(birth_date_expected, birth_date_actual);
        let retired_date_expected = "RETIREDD";
        let retired_date_actual = postal_code_struct._retired_date;
        assert_eq!(retired_date_expected, retired_date_actual);
        let delivery_installation_expected = "$";
        let delivery_installation_actual = postal_code_struct._delivery_installation;
        assert_eq!(delivery_installation_expected, delivery_installation_actual);
        let quality_indicator_expected = "QI_";
        let quality_indicator_actual = postal_code_struct._quality_indicator;
        assert_eq!(quality_indicator_expected, quality_indicator_actual);
        let source_geo_expected = "@";
        let source_geo_actual = postal_code_struct._source_geo;
        assert_eq!(source_geo_expected, source_geo_actual);
        let population_centre_and_rural_area_indicator_expected = "#";
        let population_centre_and_rural_area_indicator_actual =
            postal_code_struct._population_centre_and_rural_area_indicator;
        assert_eq!(
            population_centre_and_rural_area_indicator_expected,
            population_centre_and_rural_area_indicator_actual
        );
        let dguid_expected = "_DAuid__";
        let dguid_actual = postal_code_struct._dguid;
        assert_eq!(dguid_expected, dguid_actual);
    }
}
