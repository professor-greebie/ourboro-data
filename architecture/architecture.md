# PCCF Reader Architecture


```mermaid

sequenceDiagram
  User ->> Main : select sources
  Main ->> IO : get_and_cache resources
  IO ->> Main : return_cached_data
  Main ->> IO : get_pccf_data
  IO ->> PCCF : create_postal_code_entities
  PCCF ->> Main : PostalCode
  Main ->> Process : match_postal_dguid_to_census_data
  Process ->> Main : return_processed_data
  Main ->> User : output to user


  

```