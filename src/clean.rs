use csv::{ReaderBuilder, WriterBuilder};
use std::fs::File;
use std::error::Error;

pub fn read_clean_file(input:&str) -> Result<(), Box<dyn Error>>{
    //reads the input and the supposed output
    let input_file = File::open(input)?;
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .flexible(true)//process rows with various field counts, avoiding UnequalLengths error
        .trim(csv::Trim::All) //trim the white space between extra commas
        .from_reader(input_file);
    let output_file = File::create("ThreatenedSpecies_Cleaned.csv")?;
    let mut wtr = WriterBuilder::new().from_writer(output_file);
    if let Ok(header) = rdr.headers(){
        let mut col_fields: Vec<String> = header.iter().map(String::from).collect();
        col_fields.retain(|fields| fields != "T25" && fields != "Series");
        col_fields.retain(|fields| !fields.trim().is_empty()); //remove extra commas
        let required_order = vec!["Region/Country/Area","Year","Threatened species","Value","Source","Footnotes"];
        let reordered_header: Vec<&str> = required_order.into_iter().filter(|col| col_fields.contains(&col.to_string())).collect();
        wtr.write_record(&reordered_header)?;
    }
    
    for result in rdr.records(){
        let record = result?;
        let mut col_fields: Vec<String> = record.iter().map(String::from).collect();
        col_fields.remove(0); //remove T25 col
        let value_str = col_fields[3].replace("\"", "").replace(",", "");

        //check if the value is numeric
        if value_str.chars().all(|c| c.is_digit(10)) {
            //if it's a valid numeric string, parse the number as usize
            match value_str.parse::<usize>() {
                Ok(value) => {
                    col_fields[3] = value.to_string();
                    }
                Err(e) => {
                    println!("Error parsing value: {} - {:?}", value_str, e);
                    }
                }
            } 
            else {
             //if it's not numeric, leave the value unchanged
            col_fields[3] = col_fields[3].replace("\"", "").replace(",", ""); 
        }

        col_fields.retain(|fields| !fields.trim().is_empty());
        if col_fields.len() == 5 {
            col_fields.push(String::new());
            wtr.write_record(&col_fields)?;
        }
        else if col_fields.len() == 6{
            col_fields.swap(4, 5);
            wtr.write_record(&col_fields)?;
        }
        else{
            println!("{:?}, Error in col fields", col_fields)
        }
        
    }
    wtr.flush()?;
    Ok(())
}