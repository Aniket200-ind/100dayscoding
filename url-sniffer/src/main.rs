use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;
use std::fs::File;
use std::io::prelude::*;

error_chain! {
    foreign_links{
        Reqerror(reqwest::Error);
        IoError(std::io::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    loop {
        let mut user_input = String::new();
        println!("\n");
        println!("\n");
        println!("Welcome to the Url-sniffer!");
        println!("Please select an option number from the following: ");
        println!("1. Enter a URL to scrape for links and print them to the console.");
        println!("2. Enter a URL to scrape for links and save them to a file.");
        println!("3. Exit");

        // Todo: Optimize the code if possible

        std::io::stdin().read_line(&mut user_input).unwrap();
        let user_input: u32 = user_input.trim().parse().unwrap();

        match user_input {
            1 => {
                println!("Enter a URL to scrape: ");
                let mut url = String::new();
                std::io::stdin().read_line(&mut url).expect("Failed to read the URL");
                let url = url.trim();
                let res = reqwest::get(url).await?.text().await?;
                Document::from(res.as_str())
                .find(Name("a"))
                .filter_map(|n| n.attr("href"))
                .for_each(|x| println!("\n{}", x));
            }
            2 => {
                println!("Enter a URL to scrape: ");
                let mut url = String::new();
                std::io::stdin().read_line(&mut url).expect("Failed to read the URL");
                let url = url.trim();
                let res = reqwest::get(url).await?.text().await?;
                let mut file = File::create("links.txt")?;
                Document::from(res.as_str())
                .find(Name("a"))
                .filter_map(|n| n.attr("href"))
                    .for_each(|x| file.write_all(format!("{}\n", x).as_bytes()).expect("Failed to write to file"));
                println!("\n");
                println!("Links have been saved to `links.txt` file in the current directory.");
            }
            3 => {
                println!("Thank you for using the Url-Sniffer! Goodbye!");
                break;
            }
            _ => {
                println!("Invalid input. Please try again.");
            }
        }
    }
    Ok(())
}