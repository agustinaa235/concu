
use rand::prelude::*;
/*
productor/consumidor acotado
El productor generaba números aleatorios cada un cierto intervalo, y el consumidor los iba
acumulando (sumando) en su estado interno. También el consumidor debe tener la
capacidad de poder consultarle el acumulado desde un sistema externo.

*/
extern crate rand;
extern crate std_semaphore;


use std::sync::{Arc, RwLock};
use std::time::Duration;
use std_semaphore::Semaphore;
use std::thread;



fn main(){

    let is_not_full = Arc::new(Semaphore::new(20));
    let is_not_empty = Arc::new(Semaphore::new(0));
    let numbers = Arc::new(RwLock::new(Vec::new()));

    let mut producers = vec![];
    let mut consumers = vec![];

    for producer_id in 0..3{
        let is_not_full_producer = is_not_full.clone();
        let is_not_empty_producer = is_not_empty.clone();
        let number_producer = numbers.clone();
        producers.push(thread::spawn(move || process_producer(producer_id, is_not_empty_producer, is_not_full_producer, number_producer)));
    }
    for consumer_id in 0..10{
        let is_not_full_consumer = is_not_full.clone();
        let is_not_empty_consumer = is_not_empty.clone();
        let number_consumer = numbers.clone();
        consumers.push(thread::spawn(move || process_consumer(consumer_id, is_not_empty_consumer, is_not_full_consumer, number_consumer)));
    }

    for producer in producers{
        producer.join();
    }

    for consumer in consumers{
        consumer.join();
    }
}

fn process_producer(producer_id: usize, is_not_empty: Arc<Semaphore>, is_not_full: Arc<Semaphore>, number: Arc<RwLock<Vec<f64>>>){
    loop{
        println!("[Producer {:?}]: esperando para producer", producer_id);
        is_not_full.acquire(); // espero hasta q me avisen que la cola esta vacia
        let  random_number: f64  = rand::thread_rng().gen();

        if let Ok(mut buf) = number.write() {
            &buf.push(random_number);
            println!("[PRODUCTOR {:?}] produce este numero {:?}", producer_id, random_number);
        }

        is_not_empty.release(); // aviso que la cola ya no esta vacia
    }

}

fn process_consumer(consumer_id: usize, is_not_empty: Arc<Semaphore>, is_not_full: Arc<Semaphore>, number: Arc<RwLock<Vec<f64>>>){
    loop{
        println!("[Consumer {:?}]: esperando para consumir", consumer_id);
        is_not_empty.acquire(); // espero hasta que la cola no esta vacia 

        if let Ok(mut buf) = number.write() {
            let value = &buf.pop();
            println!("[Consumidor {:?}] consume este numero {:?}", consumer_id, value);
        }
        thread::sleep(Duration::from_secs(5));

        is_not_full.release(); // aviso que ya no esta lleno
    }

}

