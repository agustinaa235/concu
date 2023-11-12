
use async_std::task;
use futures::future::join_all;
use std::thread;
use std::time::Duration;
use rand::{thread_rng, Rng};

async fn request_function(url: &str) -> f64{
    println!("Proceso url {:?}", url);
    let random_result: f64 = rand::thread_rng().gen();
    thread::sleep(Duration::from_millis((random_result * 1000.0) as u64));
    println!("Tiempo de procesamiento de la url {:?} es {:?}", url, random_result * 1000.0);
    random_result*1000.0
}



async fn async_main(){
    let n = 10;
    println!("Proceso main");

    let urls = vec!["url"; n];

    let apis_result: Vec<_> = urls
    .iter()
    .map( |url| request_function(&url))
    .collect();

    let aux = join_all(apis_result).await;

    let mut total_time : f64 = 0.00;
    for i in &aux {
        total_time += i;
    }

    println!("Promedio de tiempo de ejecucion es {:?}", total_time/aux.len() as f64);

}



fn main(){
    let _response = task::block_on(async_main());
}