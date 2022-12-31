use dotenv::dotenv;
use itertools::Itertools;
use reqwest::{header, Client};
use std::{env, fs::OpenOptions, io::Write, process};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("TOKEN").expect("$TOKEN is not set");
    let year = env::var("YEAR")
        .expect("$YEAR is not set")
        .parse::<u32>()
        .expect("$YEAR must be a number");

    let mut headers = header::HeaderMap::new();
    let mut session_header = header::HeaderValue::from_str(format!("session={token}").as_str())
        .expect("Error building cookie header");
    session_header.set_sensitive(true);
    headers.insert(header::COOKIE, session_header);

    let client = Client::builder().default_headers(headers).build().unwrap();
    let responses = (1..=25)
        .map(|d| {
            let endpoint = format!("https://adventofcode.com/{year}/day/{d}/input");
            println!("{endpoint}");
            client.get(endpoint).send()
        })
        .collect_vec()
        .into_iter()
        .map(|x| async { x.await.unwrap().text().await.unwrap() })
        .collect_vec();

    for (day, res) in (1..=25).zip(responses) {
        let input_path = format!("src/inputs/{day:02}.txt");
        let mut file = match OpenOptions::new()
            .write(true)
            .create(true)
            .open(&input_path)
        {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to create module file: {e}");
                process::exit(1);
            }
        };

        match file.write_all(res.await.as_bytes()) {
            Ok(_) => {
                println!("Downloaded input file \"{}\"", &input_path);
            }
            Err(e) => {
                eprintln!("Failed to write module contents: {e}");
                process::exit(1);
            }
        }
    }
}
