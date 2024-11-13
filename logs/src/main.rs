use std::{fs, io::Error};

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");
    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }
    results
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n"))?;
    Ok(())
    // let text = fs::read_to_string("logs.txt").expect("failed to read logs.txt");
    // let error_logs = extract_errors(text.as_str());
    // fs::write("errors.txt", error_logs.join("\n")).expect("failed to write errors.txt");
    // match fs::read_to_string("logs.txt") {
    //     Ok(read_text) => {
    //        let error_logs = extract_errors(read_text.as_str());
    //         match fs::write("./asff/errors.txt", error_logs.join("\n")) {
    //             Ok(..) => {
    //                 println!("Wrote errors.txt")
    //             },
    //             Err(error_while_writing_file) => {
    //                 println!("Writing of errors.txt got failed: {}", error_while_writing_file);
    //             }
    //         }
    //     },
    //     Err(error_occurred) => {
    //         println!("{}", error_occurred);
    //     }
    // }

}