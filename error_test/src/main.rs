use std::fs;

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }
    return results;
}

fn main() {
    let mut errors_text: Vec<String> = vec![];
    match fs::read_to_string("logs.txt"){
        Ok(content) => {
            let errors_found: Vec<String> = extract_errors(content.as_str());
            println!("{:?}", errors_found);
            errors_text = erro  rs_found;
        },
        Err(error) => println!("Error: {}", error),
    }
    if !&errors_text.is_empty() {
         match fs::write("errors.txt", errors_text.join("\n")){
            Ok(..) => println!("Errors written to file"),
            Err(error) => println!("Error: {}", error),
         }
    } 
} 