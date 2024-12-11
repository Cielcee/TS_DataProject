use csv::{ReaderBuilder, WriterBuilder};
use std::fs::File;
//read in the data and outputs the cleaned data with rearranged header,

//returns error is teh file operations or csv parsing fails
pub fn read_clean_file(input:&str, output:&str) -> Result<(), Box<dyn std::error::Error>>{
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
        //let header = result?;
        let mut col_fields: Vec<String> = header.iter().map(String::from).collect();
        col_fields.retain(|fields| fields != "Series");
        col_fields.retain(|fields| !fields.trim().is_empty()); //remove extra commas
        let required_order = vec!["T25","Region/Country/Area","Year","Threatened species","Value","Source","Footnotes"];
        let reordered_header: Vec<&str> = required_order.into_iter().filter(|col| col_fields.contains(&col.to_string())).collect();
        wtr.write_record(&reordered_header)?;
    }
    for result in rdr.records(){
        let record = result?;

        let mut col_fields: Vec<String> = record.iter().map(String::from).collect();
        col_fields.retain(|fields| !fields.trim().is_empty());
        if col_fields.len() == 6 {
            col_fields.push(String::new());
            col_fields.truncate(7);
            wtr.write_record(&col_fields)?;
        }
        else if col_fields.len() == 7{
            let swap = col_fields[5].clone();
            col_fields[5] = col_fields[6].clone();
            col_fields[6] = swap;
            wtr.write_record(&col_fields)?;
        }
        else{
            println!("{:?}, Error in col fields", col_fields)
        }
    }
    // Ensure all data is written to the output file
    wtr.flush()?;
    Ok(())
}