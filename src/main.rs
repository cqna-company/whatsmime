/* 파일 확장자 별 시그니처(mime-type) 추출 프로그램 */
/* Create by CQNA 2024 */

use std::env;
use std::path::Path;
use std::process;

use mime_guess::from_path;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 3 || args[1] != "--mime-type" {
        eprintln!("Usage: {} --mime-type [file_name]", args[0]);
        process::exit(1);
    }

    let file_name = &args[2];

    if !Path::new(file_name).exists() {
        eprintln!("Error: File '{}' does not exist.", file_name);
        process::exit(1);
    }

    match detect_mime_type(file_name) {
        Ok(mime_type) => println!("MIME type of '{}': {}", file_name, mime_type),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
    
}

fn detect_mime_type(file_name: &str) -> Result<String, String> {
    let mime_type = from_path(file_name)
        .first()
        .ok_or_else(|| format!("Failed to detect MIME type for file '{}'", file_name))?;

    Ok(mime_type.to_string())
}
