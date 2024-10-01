use csv::ReaderBuilder;
use std::{error::Error, fs::File, io::BufReader, path::Path};

pub fn read_csv(
    path: impl AsRef<Path>,
    delimiter: u8,
    header: bool,
) -> Result<Vec<Vec<f64>>, Box<dyn Error>> {
    let reader = BufReader::new(File::open(path)?);
    let mut reader = ReaderBuilder::new()
        .has_headers(header)
        .delimiter(delimiter)
        .from_reader(reader);

    let mut samples = Vec::new();

    for result in reader.deserialize() {
        let record: Vec<f64> = result.unwrap();
        // Replace NaNs with 0s
        let record = record
            .to_vec()
            .iter()
            .map(|v| if v.is_nan() { 0.0 } else { *v })
            .collect::<Vec<f64>>();
        samples.push(record);
    }
    Ok(samples)
}
