-- Add migration script here
CREATE TABLE IF NOT EXISTS census (year INTEGER PRIMARY KEY, cType TEXT CHECK(cType IN ('decade', 'five_year')) DEFAULT 'decade', cOf TEXT CHECK(cOf IN ('population', 'agriculture')) DEFAULT 'population', censusTitle TEXT);
CREATE TABLE IF NOT EXISTS characteristic (charId INTEGER PRIMARY KEY,charName TEXT,charNote TEXT);
CREATE TABLE IF NOT EXISTS location (dguid TEXT PRIMARY KEY,alt_geo_code TEXT,geo_level TEXT,geo_name TEXT);
CREATE TABLE IF NOT EXISTS count_result (dguid TEXT,charId INTEGER,total_result NUMERIC,male_plus_result NUMERIC,female_plus_result NUMERIC,total_rate NUMERIC,male_plus_rate NUMERIC,female_plus_rate NUMERIC,PRIMARY KEY (dguid, charId));
CREATE TABLE IF NOT EXISTS postal_code_data (postal_code TEXT PRIMARY KEY,dguid TEXT,location_name TEXT,province TEXT,lat NUMERIC,lon NUMERIC,community_name TEXT);