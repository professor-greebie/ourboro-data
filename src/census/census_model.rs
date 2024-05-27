pub struct CensusPopulationStruct {
    pub postal_code: String,
    pub dguid: String,
    pub population: Vec<String>,
}

pub struct CensusLineStruct {
    pub year: i64,
    pub dguid: String,
    pub alt_geo_code: String,
    pub geo_level: String,
    pub geo_name: String,
    pub total_no_response_short_form: f64,
    pub total_no_response_long_form: f64,
    pub data_quality_flag: String,
    pub characteristic_id: i64,
    pub characteristic_name: String,
    pub characteristic_note: String,
    pub c1_count_total: Option<i64>,
    pub c1_symbol: String,
    pub c2_count_men_plus: Option<i64>,
    pub c2_symbol: String,
    pub c3_count_women_plus: Option<i64>,
    pub c3_symbol: String,
    pub c10_rate_total: Option<f64>,
    pub c10_symbol: String,
    pub c11_rate_men_plus: Option<f64>,
    pub c11_symbol: String,
    pub c12_rate_women_plus: Option<f64>,
    pub c12_symbol: String,
}
impl CensusLineStruct {
    pub fn from_line(line: Vec<String>) -> Self {
        if line.len() != 23 {
            panic!("Invalid line length for census collection : {}", line.len());
        }
        Self {
            year: line[0].parse::<i64>().unwrap(),
            dguid: line[1].clone(),
            alt_geo_code: line[2].clone(),
            geo_level: line[3].clone(),
            geo_name: line[4].clone(),
            total_no_response_short_form: line[5].parse::<f64>().unwrap(),
            total_no_response_long_form: line[6].parse::<f64>().unwrap(),
            data_quality_flag: line[7].clone(),
            characteristic_id: line[8].parse::<i64>().unwrap(),
            characteristic_name: line[9].clone(),
            characteristic_note: line[10].clone(),
            c1_count_total: line[11].parse::<i64>().ok(),
            c1_symbol: line[12].clone(),
            c2_count_men_plus: line[13].parse::<i64>().ok(),
            c2_symbol: line[14].clone(),
            c3_count_women_plus: line[15].parse::<i64>().ok(),
            c3_symbol: line[16].clone(),
            c10_rate_total: line[17].parse::<f64>().ok(),
            c10_symbol: line[18].clone(),
            c11_rate_men_plus: line[19].parse::<f64>().ok(),
            c11_symbol: line[20].clone(),
            c12_rate_women_plus: line[21].parse::<f64>().ok(),
            c12_symbol: line[22].clone(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CensusFilter {
    Population2021,
    Population2016,
    LandArea,
    TotalOccupiedDwellings,
    TotalSingleDetachedHouses,
    TotalSemiDetachedHouses,
    TotalRowHouses,
    TotalApartmentDuplex,
    TotalApartmentBuildingLessThan5Stories,
    TotalApartmentBuilding5StoriesOrMore,
    TotalOtherDwellings,
    TotalMovableDwellings,
    MedianAfterTaxIncome,
    AverageAfterTaxIncome,
    MedianTotalIncomeEconomicFamily,
    AverageTotalIncomeEconomicFamily,
    AverageAfterTaxIncomeOneParent,
    MedianTotalIncomeOneParent,
    TotalCensusFamiliesPrivateHouseHolds,
    MarriedWithChildren,
    CommonLawWithChildren,
    OneParentFamilies,
    AverageTotalChildrenPerFamily,
    LowIncomeMeasureAfterTaxLIMAT,
    PrevalanceOfLowIncomeLIMAT,
    PrevalanceOfLowIncomeLICOAT,
    GiniIndexOnTotalIncome,
    GiniIndexOnMarketIncome,
    GiniIndexOnAfterTaxIncome,
    P90P10Ratio,
    HouseholdsSpending30PercentOrMoreOfIncomeOnShelter,
    ImmigrantStatusTotal,
    ImmigrantStatusImmigrant,
    ImmigrantStatusNonImmigrant,
    ImmigrantStatusNonPermanentResident,
    IndigenousIdentityIndigenous,
    IndigenousIdentityNonIndigenous,
    PrivateHouseholdsByTenureTotal,
    PrivateHouseholdsByTenureOwner,
    PrivateHouseholdsByTenureRenter,
    PrivateHouseholdsByTenureBandH,
    DwellingConditionTotal,
    DwellingConditionRegularMaintenance,
    DwellingConditionMajorRepairs,
    HousingSuitabilityTotal,
    HousingSuitabilitySuitable,
    HousingSuitabilityNotSuitable,
    MobilityStatusTotal,
    MobilityStatusMovers,
    MobilityStatusNonMovers,
    HighestCertificateNone,
    HighestCertificateHighSchool,
    HighestCertificateCollege,
    HighestCertificateUniversityBachelorOrHigher,
    HighestCertificateUniversityBachelor,
    HighestCertificateUniversityAboveBachelor,
    ParticipationRate,
    EmploymentRate,
    UnemploymentRate,
    CommutingEmployedLaborForceTotal,
    CommutingEmployedLaborForceLessThan15,
    CommutingEmployedLaborForce15To29,
    CommutingEmployedLaborForce30To44,
    CommutingEmployedLaborForce45To59,
    CommutingEmployedLaborForceMoreThan60,
    UnknownFilter,
}

impl CensusFilter {
    pub fn cache_name(self) -> String {
        match self {
            Self::Population2021 => "population_2021".to_string(),
            Self::Population2016 => "population_2016".to_string(),
            Self::LandArea => "land_area".to_string(),
            Self::TotalOccupiedDwellings => "total_occupied_dwellings".to_string(),
            Self::TotalSingleDetachedHouses => "total_single_detached_houses".to_string(),
            Self::TotalSemiDetachedHouses => "total_semi_detached_houses".to_string(),
            Self::TotalRowHouses => "total_row_houses".to_string(),
            Self::TotalApartmentDuplex => "total_apartment_duplex".to_string(),
            Self::TotalApartmentBuildingLessThan5Stories => {
                "total_apartment_building_less_than_5_stories".to_string()
            }
            Self::TotalApartmentBuilding5StoriesOrMore => {
                "total_apartment_building_5_stories_or_more".to_string()
            }
            Self::TotalOtherDwellings => "total_other_dwellings".to_string(),
            Self::TotalMovableDwellings => "total_movable_dwellings".to_string(),
            Self::MedianAfterTaxIncome => "median_after_tax_income".to_string(),
            Self::AverageAfterTaxIncome => "average_after_tax_income".to_string(),
            Self::MedianTotalIncomeEconomicFamily => {
                "median_total_income_economic_family".to_string()
            }
            Self::AverageTotalIncomeEconomicFamily => {
                "average_total_income_economic_family".to_string()
            }
            Self::AverageAfterTaxIncomeOneParent => {
                "average_after_tax_income_one_parent".to_string()
            }
            Self::MedianTotalIncomeOneParent => "median_total_income_one_parent".to_string(),
            Self::TotalCensusFamiliesPrivateHouseHolds => {
                "total_census_families_private_house_holds".to_string()
            }
            Self::MarriedWithChildren => "married_with_children".to_string(),

            Self::CommonLawWithChildren => "common_law_with_children".to_string(),
            Self::OneParentFamilies => "one_parent_families".to_string(),
            Self::AverageTotalChildrenPerFamily => "average_total_children_per_family".to_string(),
            Self::LowIncomeMeasureAfterTaxLIMAT => "low_income_measure_after_tax_LIMAT".to_string(),
            Self::PrevalanceOfLowIncomeLIMAT => "prevalance_of_low_income_LIMAT".to_string(),
            Self::PrevalanceOfLowIncomeLICOAT => "prevalance_of_low_income_LICOAT".to_string(),
            Self::GiniIndexOnTotalIncome => "gini_index_on_total_income".to_string(),
            Self::GiniIndexOnMarketIncome => "gini_index_on_market_income".to_string(),
            Self::GiniIndexOnAfterTaxIncome => "gini_index_on_after_tax_income".to_string(),
            Self::P90P10Ratio => "P90P10_ratio".to_string(),
            Self::HouseholdsSpending30PercentOrMoreOfIncomeOnShelter => {
                "households_spending_30_percent_or_more_of_income_on_shelter".to_string()
            }
            Self::ImmigrantStatusTotal => "immigrant_status_total".to_string(),
            Self::ImmigrantStatusImmigrant => "immigrant_status_immigrant".to_string(),
            Self::ImmigrantStatusNonImmigrant => "immigrant_status_non_immigrant".to_string(),
            Self::ImmigrantStatusNonPermanentResident => {
                "immigrant_status_non_permanent_resident".to_string()
            }
            Self::IndigenousIdentityIndigenous => "identity_indigenous".to_string(),
            Self::IndigenousIdentityNonIndigenous => "identity_non_indigenous".to_string(),
            Self::PrivateHouseholdsByTenureTotal => {
                "private_households_by_tenure_total".to_string()
            }
            Self::PrivateHouseholdsByTenureOwner => {
                "private_households_by_tenure_owner".to_string()
            }
            Self::PrivateHouseholdsByTenureRenter => {
                "private_households_by_tenure_renter".to_string()
            }
            Self::PrivateHouseholdsByTenureBandH => {
                "private_households_by_tenure_band_h".to_string()
            }
            Self::DwellingConditionTotal => "dwelling_condition_total".to_string(),
            Self::DwellingConditionRegularMaintenance => {
                "dwelling_condition_regular_maintenance".to_string()
            }
            Self::DwellingConditionMajorRepairs => "dwelling_condition_major_repairs".to_string(),
            Self::HousingSuitabilityTotal => "housing_suitability_total".to_string(),
            Self::HousingSuitabilitySuitable => "housing_suitability_suitable".to_string(),
            Self::HousingSuitabilityNotSuitable => "housing_suitability_not_suitable".to_string(),
            Self::MobilityStatusTotal => "mobility_status_total".to_string(),
            Self::MobilityStatusMovers => "mobility_status_movers".to_string(),
            Self::MobilityStatusNonMovers => "mobility_status_non_movers".to_string(),
            Self::HighestCertificateNone => "highest_certificate_none".to_string(),
            Self::HighestCertificateHighSchool => "high_school".to_string(),
            Self::HighestCertificateCollege => "college".to_string(),
            Self::HighestCertificateUniversityBachelorOrHigher => {
                "university_bachelor_or_higher".to_string()
            }
            Self::HighestCertificateUniversityBachelor => "university_bachelor".to_string(),
            Self::HighestCertificateUniversityAboveBachelor => {
                "university_above_bachelor".to_string()
            }
            Self::ParticipationRate => "participation_rate".to_string(),
            Self::EmploymentRate => "employment_rate".to_string(),
            Self::UnemploymentRate => "unemployment_rate".to_string(),
            Self::CommutingEmployedLaborForceTotal => {
                "commuting_employed_labor_force_total".to_string()
            }
            Self::CommutingEmployedLaborForceLessThan15 => {
                "commuting_employed_labor_force_less_than_15".to_string()
            }
            Self::CommutingEmployedLaborForce15To29 => {
                "commuting_employed_labor_force_15_to_29".to_string()
            }
            Self::CommutingEmployedLaborForce30To44 => {
                "commuting_employed_labor_force_30_to_44".to_string()
            }
            Self::CommutingEmployedLaborForce45To59 => {
                "commuting_employed_labor_force_45_to_59".to_string()
            }
            Self::CommutingEmployedLaborForceMoreThan60 => {
                "commuting_employed_labor_force_more_than_60".to_string()
            }
            Self::UnknownFilter => "unknown_filter".to_string(),
        }
    }

    pub fn filter_column(self) -> usize {
        match self {
            Self::Population2021 => 1,
            Self::Population2016 => 2,
            Self::LandArea => 7,
            Self::TotalOccupiedDwellings => 41,
            Self::TotalSingleDetachedHouses => 42,
            Self::TotalSemiDetachedHouses => 43,
            Self::TotalRowHouses => 44,
            Self::TotalApartmentDuplex => 45,
            Self::TotalApartmentBuildingLessThan5Stories => 46,
            Self::TotalApartmentBuilding5StoriesOrMore => 47,
            Self::TotalOtherDwellings => 48,
            Self::TotalMovableDwellings => 49,
            Self::TotalCensusFamiliesPrivateHouseHolds => 71,
            Self::AverageTotalChildrenPerFamily => 77,
            Self::MarriedWithChildren => 81,
            Self::CommonLawWithChildren => 84,
            Self::OneParentFamilies => 86,
            Self::MedianAfterTaxIncome => 115,
            Self::AverageAfterTaxIncome => 128,
            Self::MedianTotalIncomeEconomicFamily => 243,
            Self::MedianTotalIncomeOneParent => 246,
            Self::AverageTotalIncomeEconomicFamily => 252,
            Self::AverageAfterTaxIncomeOneParent => 256,

            Self::LowIncomeMeasureAfterTaxLIMAT => 340,
            Self::PrevalanceOfLowIncomeLIMAT => 345,
            Self::PrevalanceOfLowIncomeLICOAT => 360,
            Self::GiniIndexOnTotalIncome => 379,
            Self::GiniIndexOnMarketIncome => 380,
            Self::GiniIndexOnAfterTaxIncome => 381,
            Self::P90P10Ratio => 382,
            Self::IndigenousIdentityIndigenous => 1403,
            Self::IndigenousIdentityNonIndigenous => 1410,
            Self::PrivateHouseholdsByTenureTotal => 1414,
            Self::PrivateHouseholdsByTenureOwner => 1415,
            Self::PrivateHouseholdsByTenureRenter => 1416,
            Self::PrivateHouseholdsByTenureBandH => 1417,
            Self::HousingSuitabilityTotal => 1437,
            Self::HousingSuitabilitySuitable => 1438,
            Self::HousingSuitabilityNotSuitable => 1439,
            Self::DwellingConditionTotal => 1449,
            Self::DwellingConditionRegularMaintenance => 1450,
            Self::DwellingConditionMajorRepairs => 1451,
            Self::HouseholdsSpending30PercentOrMoreOfIncomeOnShelter => 1467,
            Self::ImmigrantStatusTotal => 1528,
            Self::ImmigrantStatusImmigrant => 1529,
            Self::ImmigrantStatusNonImmigrant => 1528,
            Self::ImmigrantStatusNonPermanentResident => 1530,       
            Self::MobilityStatusTotal => 1974,
            Self::MobilityStatusMovers => 1976,
            Self::MobilityStatusNonMovers => 1975,
            Self::HighestCertificateNone => 1999,
            Self::HighestCertificateHighSchool => 2000,
            Self::HighestCertificateCollege => 2002,
            Self::HighestCertificateUniversityBachelorOrHigher => 2008,
            Self::HighestCertificateUniversityBachelor => 2009,
            Self::HighestCertificateUniversityAboveBachelor => 2010,
            Self::ParticipationRate => 2228,
            Self::EmploymentRate => 2229,
            Self::UnemploymentRate => 2230,
            Self::CommutingEmployedLaborForceTotal => 2611,
            Self::CommutingEmployedLaborForceLessThan15 => 2612,
            Self::CommutingEmployedLaborForce15To29 => 2613,
            Self::CommutingEmployedLaborForce30To44 => 2614,
            Self::CommutingEmployedLaborForce45To59 => 2615,
            Self::CommutingEmployedLaborForceMoreThan60 => 2616,
            Self::UnknownFilter => 0,
        }
    }
}
