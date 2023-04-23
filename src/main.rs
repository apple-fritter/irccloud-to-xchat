use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use zip::read::ZipArchive;
use std::path::Path;

fn convert_irccloud_to_xchat(input_zip: &str, output_dir: &str) -> std::io::Result<()> {
    let zip_file = File::open(input_zip)?;
    let mut zip = ZipArchive::new(zip_file)?;

    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let file_name = file.name();
        let channel = file_name.split("/").last().unwrap();
        let output_file = format!("{}/{}.log", output_dir, channel);

        let mut output = File::create(output_file)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let line_parts: Vec<&str> = line.split(" ").collect();
            let timestamp = line_parts[0];
            let nickname = line_parts[1];
            let message = line_parts[2..].join(" ");

            writeln!(
                output,
                "{} <{}> {}",
                timestamp, nickname, message
            )?;
        }
    }

    Ok(())
}
