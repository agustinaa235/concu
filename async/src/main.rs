use reqwest;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let mut items: Vec<String> = vec![];
    for n in 1..5{
        let fact = client
            .get("https://uselessfacts.jsph.pl/api/v2/facts/random?language=en")
            .header("Accept", "text/plain")
            .send()
            .await?
            .text()
            .await?;
        //println!("{:?}", fact);
        items.push(fact)
    }
    println!("{:?}", items);
    Ok(())
}
