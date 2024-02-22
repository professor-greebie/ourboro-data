pub struct CensusPopulationStruct {
    pub postal_code: String,
    pub dguid: String,
    pub population: Vec<String>,
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
    LowIncomeMeasureAfterTax,
    GiniIndexOnAfterTaxIncome,
    HouseholdsSpending30PercentOrMoreOfIncomeOnShelter,
    ImmigrantStatusImmigrant,
    ImmigrantStatusNonImmigrant,
    UnknownFilter
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
            Self::AverageAfterTaxIncomeOneParent => "average_after_tax_income_one_parent".to_string(),
            Self::MedianTotalIncomeOneParent => "median_total_income_one_parent".to_string(),
            Self::TotalCensusFamiliesPrivateHouseHolds => {
                "total_census_families_private_house_holds".to_string()
            }
            Self::MarriedWithChildren => "married_with_children".to_string(),
            Self::CommonLawWithChildren => "common_law_with_children".to_string(),
            Self::OneParentFamilies => "one_parent_families".to_string(),
            Self::AverageTotalChildrenPerFamily => "average_total_children_per_family".to_string(),
            Self::LowIncomeMeasureAfterTax => "low_income_measure_after_tax".to_string(),
            Self::GiniIndexOnAfterTaxIncome => "gini_index_on_after_tax_income".to_string(),
            Self::HouseholdsSpending30PercentOrMoreOfIncomeOnShelter => {
                "households_spending_30_percent_or_more_of_income_on_shelter".to_string()
            }
            Self::ImmigrantStatusImmigrant => "immigrant_status_immigrant".to_string(),
            Self::ImmigrantStatusNonImmigrant => "immigrant_status_non_immigrant".to_string(),
            Self::UnknownFilter => "unknown_filter".to_string(),


        }
    }

    pub fn filter_string(self) -> String {
        match self {
            Self::Population2021 => "Population, 2021".to_string(),
            Self::Population2016 => "Population, 2016".to_string(),
            Self::LandArea => "Land area in square kilometres".to_string(),
            Self::TotalOccupiedDwellings => {
                "Total - Occupied private dwellings by structural type of dwelling - 100% data"
                    .to_string()
            }
            Self::TotalSingleDetachedHouses => "Single-detached house".to_string(),
            Self::TotalSemiDetachedHouses => "Semi-detached house".to_string(),
            Self::TotalRowHouses => "Row house".to_string(),
            Self::TotalApartmentDuplex => "Apartment or flat in a duplex".to_string(),
            Self::TotalApartmentBuildingLessThan5Stories => {
                "Apartment in a building that has five or more storeys".to_string()
            }
            Self::TotalApartmentBuilding5StoriesOrMore => {
                "Apartment in a building that has fewer than five storeys".to_string()
            }
            Self::TotalOtherDwellings => "Other dwelling".to_string(),
            Self::TotalMovableDwellings => "Movable dwelling".to_string(),
            Self::MedianAfterTaxIncome => {
                "Median after-tax income of households in 2020 ($)".to_string()
            }
            Self::AverageAfterTaxIncome => {
                "Average after-tax income of households in 2020 ($)".to_string()
            }
            Self::MedianTotalIncomeEconomicFamily => {
                "Median total income of economic families in 2020 ($)".to_string()
            }
            Self::AverageTotalIncomeEconomicFamily => {
                "Average total income of economic families in 2020 ($)".to_string()
            }
            Self::AverageAfterTaxIncomeOneParent => {
                "Average after-tax income of one-parent families in 2015 ($)".to_string()
            }
            Self::MedianTotalIncomeOneParent => {
                "Median total income of one-parent families in 2020 ($)".to_string()
            }
            Self::TotalCensusFamiliesPrivateHouseHolds => {
                "Total - Census families in private households by family size - 100% data"
                    .to_string()
            }
            Self::MarriedWithChildren => "With children".to_string(),
            Self::CommonLawWithChildren => "With children".to_string(),
            Self::OneParentFamilies => "One-parent census families".to_string(),
            Self::AverageTotalChildrenPerFamily => {
                "Average number of children at home per census family".to_string()
            }
            Self::LowIncomeMeasureAfterTax => {
                "In low income based on the Low-income measure, after tax (LIM-AT)".to_string()
            }
            Self::GiniIndexOnAfterTaxIncome => {
                "Gini coefficient of the after-tax income distribution of households in 2020"
                    .to_string()
            }
            Self::HouseholdsSpending30PercentOrMoreOfIncomeOnShelter => {
                "Households spending 30% or more of its income on shelter costs".to_string()
            }
            Self::ImmigrantStatusImmigrant => "Immigrants".to_string(),
            Self::ImmigrantStatusNonImmigrant => "Non-immigrants".to_string(),
            Self::UnknownFilter => "Unknown Filter".to_string(),
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
            Self::MedianAfterTaxIncome => 115,
            Self::AverageAfterTaxIncome => 128,
            Self::MedianTotalIncomeEconomicFamily => 243,
            Self::AverageTotalIncomeEconomicFamily => 252,
            Self::AverageAfterTaxIncomeOneParent => 256,
            Self::MedianTotalIncomeOneParent => 246,
            Self::TotalCensusFamiliesPrivateHouseHolds => 71,
            Self::MarriedWithChildren => 81,
            Self::CommonLawWithChildren => 84,
            Self::OneParentFamilies => 86,
            Self::AverageTotalChildrenPerFamily => 77,
            Self::LowIncomeMeasureAfterTax => 340,
            Self::GiniIndexOnAfterTaxIncome => 381,
            Self::HouseholdsSpending30PercentOrMoreOfIncomeOnShelter => 1467,
            Self::ImmigrantStatusImmigrant => 1529,
            Self::ImmigrantStatusNonImmigrant => 1528,
            Self::UnknownFilter => 0,
        }
    }
}
