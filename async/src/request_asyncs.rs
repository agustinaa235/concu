use async_std::task;
use futures::future::join_all;

async fn async_main() {
 
    let urls = vec!["https://uselessfacts.jsph.pl/api/v2/facts/random?language=en"; 100];
    let client = reqwest::Client::new();
    let apis_result: Vec<_> = urls
    .iter()
    .map(|url| {
        let client = &client;
        async move {
            let resp = client.get("https://uselessfacts.jsph.pl/api/v2/facts/random?language=en").send().await?;
            resp.bytes().await

        }
    }).collect();

    let bodies = join_all(apis_result).await;

    for bodie in bodies {
        println!("{:#?}", bodie);
    }

}

pub fn main() {
    let _response = task::block_on(async_main());
}