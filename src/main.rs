extern crate plotters;
use std::collections::HashMap;
use std::io::{self, Write};
use std::fs::File;
use serde::Deserialize;
//------regression use--------
use ndarray::{Array1,Array2};
use linfa::Dataset;
use linfa_linear::{LinearRegression,FittedLinearRegression};
use linfa::prelude::*;
mod clean;
#[derive(Debug, Deserialize)]
struct CountryData {
    #[serde(rename = "Region/Country/Area")]
    country_name: String,
    #[serde(rename = "Year")]
    year: usize,
    #[serde(rename = "Threatened species")]
    species: String,
    #[serde(rename = "Value")]
    value: usize,
    #[serde(rename = "Source")]
    source: String,
    #[serde(rename = "Footnotes")]
    footnote: Option<String>,
}

//create a hashmap of reading the values, with outer key country name, inner key species type, and value for (year, value)
type DataThreatenedNum = HashMap<String, HashMap<String, Vec<(usize, usize)>>>;


fn read_data(path: &str) -> Result<DataThreatenedNum, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut data: DataThreatenedNum = HashMap::new();
    for (index, result) in rdr.deserialize::<CountryData>().enumerate() {
        match result {
            Ok(line) => {
                let country = line.country_name.clone();
                let species_type = match line.species.as_str() {
                    "Threatened Species: Total (number)" => "Total",
                    "Threatened Species: Vertebrates (number)" => "Vertebrates",
                    "Threatened Species: Invertebrates (number)" => "Invertebrates",
                    "Threatened Species: Plants (number)" => "Plants",
                    _ => continue,
                };
                let value_num = line.value;

                data.entry(country)
                    .or_insert_with(HashMap::new)
                    .entry(species_type.to_string())
                    .or_insert_with(Vec::new)
                    .push((line.year, value_num));
            }
            Err(error) => {
                eprintln!("Error deserializing line {}: {:?}. Error: {}", index + 1, error, error);
            }
        }
    }
    Ok(data)
}
//-----------------------------linear regression-----------------------------
fn fit_model(country_data: &[(usize, usize)]) -> FittedLinearRegression<f64> {
    //prepare years and values as f64 vectors
    let years: Vec<f64> = country_data.iter().map(|&(year, _)| year as f64).collect();
    let values: Vec<f64> = country_data.iter().map(|&(_, value)| value as f64).collect();

    //convert to ndarray::Array2 for features and Array1 for targets
    let x = Array2::from_shape_vec((years.len(), 1), years)
        .expect("Failed to create feature matrix");
    let y = Array1::from_vec(values); 
    //create a dataset
    let dataset = Dataset::new(x, y);
    //fit the model
    let lin_reg = LinearRegression::new();
    let model = lin_reg.fit(&dataset).expect("Failed to fit linear regression model");
    //calculate mean absolute error 
    let predictions = model.predict(&dataset);
    let loss = (dataset.targets() - predictions)
        .mapv(f64::abs)
        .mean();
    if let Some(loss_value) = loss {
        println!("Mean Absolute Error: {}", loss_value);
    } 
    else {
        println!("Mean Absolute Error: No loss value calculated");
    }
    model
}

fn predict_next_years(model: &FittedLinearRegression<f64>, last_year: usize, n: usize) -> Vec<(usize, f64)> {
    let mut predictions = Vec::new();
    for i in 1..=n {
        let next_year = (last_year + i) as f64;
        let prediction = model.predict(&Array2::from_shape_vec((1, 1), vec![next_year]).unwrap());
        predictions.push((last_year + i, prediction[0]));
    }
    predictions
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    //create cleaned file
    let t = clean::read_clean_file("ThreatenedSpecies.csv");
    let data = read_data("ThreatenedSpecies_Cleaned.csv")?;

    println!("Enter country name: ");
    let mut country_input = String::new();
    io::stdin().read_line(&mut country_input).expect("Failed to read line");
    let country_input = country_input.trim();

    println!("Enter species type (Total, Plants, Vertebrates, Invertebrates): ");
    let mut species_input = String::new();
    io::stdin().read_line(&mut species_input).expect("Failed to read line");
    let species_input = species_input.trim();

    println!("Enter the number of years to predict: ");
    let mut years_input = String::new();
    io::stdin().read_line(&mut years_input).expect("Failed to read line");
    let n: usize = years_input.trim().parse().unwrap_or(10); //default to 10 if input is invalid

    if let Some(country_data) = data.get(country_input) {
        if let Some(species_data) = country_data.get(species_input) {
            println!("Data for {} ({}) species: {:?}", country_input, species_input, species_data);
            // Fit the model using the species data
            let model = fit_model(species_data);
            let last_year = species_data.iter().map(|&(year, _)| year).max().unwrap();

            let predictions = predict_next_years(&model, last_year, n);
            for (year, predicted) in predictions {
                println!(
                    "Predicted numbers of threatened species ({}) for {} in year {}: {:.2}",
                    species_input, country_input, year, predicted
                );
            }
        } 
        else {
            println!("No data available for {} ({}) species", country_input, species_input);
        }
    } 
    else {
        println!("No data available for {}", country_input);
    }
    Ok(())

}


//function used for test
fn create_mock_csv(path: &str) {
    let mut file = File::create(path).expect("Unable to create mock CSV file");
    writeln!(
        file,
        "Region/Country/Area,Year,Threatened species,Value,Source,Footnotes\n\
        Zambia,2004,Threatened Species: Total (number),34,SomeSource,\n\
        Zambia,2010,Threatened Species: Total (number),62,SomeSource,\n\
        Zambia,2015,Threatened Species: Total (number),85,SomeSource,\n\
        Zambia,2019,Threatened Species: Total (number),90,SomeSource,\n\
        Zambia,2020,Threatened Species: Total (number),102,SomeSource,\n\
        Zambia,2021,Threatened Species: Total (number),111,SomeSource,\n\
        Zambia,2022,Threatened Species: Total (number),139,SomeSource,"
    )
    .expect("Unable to write mock CSV data");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_data_total() {
        // Create a temporary mock CSV file
        let mock_csv_path = "test_mock_data.csv";
        create_mock_csv(mock_csv_path);
        let data_test = read_data(mock_csv_path).unwrap();
        assert!(
            data_test.contains_key("Zambia"),
            "Expected data to contain key 'Zambia'"
        );
        if let Some(zambia_data) = data_test.get("Zambia") {
            assert!(zambia_data.contains_key("Total"), "Expected data to contain 'Total' species for Zambia");
        
            // Get the vector of data for the "Total" species and check its length
            let total_data = zambia_data.get("Total").unwrap();
            assert_eq!(total_data.len(), 7, "Expected 7 entries for Zambia's 'Total' species");
        } 
        else {
            panic!("Data for Zambia was not found");
        }
        std::fs::remove_file(mock_csv_path).expect("Failed to remove mock CSV file");
    }

    #[test]
    fn test_fit_model() {
        let test_data = vec![(2000, 1200), (2001, 1220), (2002, 1250)];
        let model = fit_model(&test_data);
        let predictions = model.predict(&Array2::from_shape_vec((3, 1), vec![2003.0, 2004.0, 2005.0]).unwrap());
        assert_eq!(predictions.len(), 3);
        assert!(predictions[0] > 1200.0); // Predicts value greater than 1200
    }

    #[test]
    fn test_predict_next_years() {
        let test_data = vec![(2000, 1200), (2001, 1220), (2002, 1250)];
        let model = fit_model(&test_data);
        let predictions = predict_next_years(&model, 2002, 5);
        
        assert_eq!(predictions.len(), 5);
        assert_eq!(predictions[0].0, 2003);
        assert!(predictions[0].1 > 1250.0); 
    }
}
