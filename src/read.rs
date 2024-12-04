use std::fs::File;
use std::io::BufRead;
pub fn read_file(path:&str) -> (usize, Vec<(usize, usize)>) {
    //reads a file containing graph data and returns the number of edges and a list of edges
    let mut first_line = true;
    let mut result:Vec<(usize, usize)> = Vec::new();
    let mut num_edges: usize = 0;
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        //each line is read as a Result<String, Error>, it’s unwrapped with expect, causing the program to panic if an error occurs
        let line_string = line.expect("Error reading");
        if first_line {
            //the first line is expected to contain the number of edges, parsed into num_edges. This is assumed to be a valid usize integer.
            num_edges = line_string.parse::<usize>().unwrap();
            first_line = false;
        }
        else {
            //each subsequent line is split into whitespace-separated components, where the first component is converted to x and the second to y
            let v: Vec<&str> = line_string.trim().split(' ').collect();
            let x:usize = v[0].parse::<usize>().unwrap();
            let y:usize = v[1].parse::<usize>().unwrap();
            result.push((x,y));
        }
    }
    //returns a tuple containing num_edges and the list of edges result
    return (num_edges, result);
} 
