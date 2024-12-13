use csv::{ReaderBuilder, WriterBuilder};
use std::fs::File;

pub fn read_clean_file(input:&str) -> Result<(), Box<dyn std::error::Error>>{
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
            match value_str.parse::<usize>() {
                Ok(value) => {
                    // Update the value in the column fields
                    col_fields[3] = value.to_string();
                    wtr.write_record(&col_fields)?;
                }
                Err(e) => {
                    println!("Error parsing value: {} - {:?}", value_str, e);
                }
            }
        col_fields.retain(|fields| !fields.trim().is_empty());
        if col_fields.len() == 5 {
            col_fields.push(String::new());
            col_fields.truncate(6);
            wtr.write_record(&col_fields)?;
        }
        else if col_fields.len() == 6{
            let swap = col_fields[4].clone();
            col_fields[4] = col_fields[5].clone();
            col_fields[5] = swap;
            wtr.write_record(&col_fields)?;
        }
        else{
            println!("{:?}, Error in col fields", col_fields)
        }
    }
    wtr.flush()?;
    Ok(())
}