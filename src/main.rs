use std::process::exit;
use clap::Parser;
use colored::Colorize;
use csv2json::cli::parser::Args;
use csv2json::parser::csv_reader::CsvObject;
use csv2json::parser::parser::parse;

fn main() {
    let cli = Args::parse();

    if cli.filename.extension().unwrap().to_str().unwrap() != "csv" {
        error("This file has not a correct extension name. Please check your file.");
        return;
    }

    let save_path = match cli.save {
        Some(path) => path,
        None => cli.filename.clone().with_extension("json"),
    };

    // Start parse process.
    let csv_object = match CsvObject::read_from_file(cli.filename.to_str().unwrap_or_else(|| {
        error("Failed to resolve filename.");
        exit(0);
    })) {
        Ok(object) => object,
        Err(err) => {
            error(err.as_str());
            return;
        }
    };

    let json_object = match parse(csv_object) {
        Ok(object) => object,
        Err(err) => {
            error(err.as_str());
            return;
        }
    };

    match json_object.write_to_file(save_path.as_os_str().to_str().unwrap_or_else(|| {
        warn("Save path invalid. Changed to 'result.json'.");
        "result.json"
    })) {
        Ok(_) => (),
        Err(err) => {
            error(format!("Cannot write into file. Error: {}", err).as_str());
        }
    }

    info("Parse done.")
}

fn info(message: &str) {
    println!("{}{}", "[ INFO] --- ".blue(), message);
}

fn warn(message: &str) {
    println!("{}{}", "[ WARN] --- ".yellow(), message);
}

fn error(message: &str) {
    println!("{}{}", "[ERROR] --- ".red(), message);
}
