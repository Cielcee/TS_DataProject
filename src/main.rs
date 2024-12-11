//150 lines not including test, multiple files (seperated read files)
//initialize on github
extern crate plotters;
use plotters::prelude::*;
use std::collections::HashMap;
use linfa::prelude::*;
use std::process::Command;
//use linfa_linear::LinearRegression;
mod clean;
struct CountryData{
    country_code: u32,
    country_name: String,
    year: u32,
    series: String,
    value: u32,
}

type DataSet = Vec<CountryData>;

//-----------------------------data cleaning-----------------------------
fn data_header_rearranged() -> CountryData{
    //let mut rdr = csv::Reader::from_path("ThreatenedSpecies.csv").unwrap();
    unimplemented!()

}


//-----------------------------linear regression-----------------------------

fn main() {
    let t = clean::read_clean_file("ThreatenedSpecies.csv", "mod.csv");
    println!("{:?}", t);
}
