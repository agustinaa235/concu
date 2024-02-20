

use std::sync::{Arc, Mutex,Condvar};
use std::thread;
extern crate rand;
use crate::rand::Rng;
use std::time::Duration;


fn main(){

    let pair_monitor = Arc::new((Mutex::new(Vec::new()), Condvar::new()));
    let mut consumers = vec![];
    for id in 1..4 {
        let pair_consumer = pair_monitor.clone();
        consumers.push(thread::spawn(move || process_consumer(id, pair_consumer)));
    }

    let pair_producer = pair_monitor.clone();
    let producer = thread::spawn(move || process_producer(pair_producer));

    let _ = producer.join();

    for consumer in consumers{
        let _ = consumer.join();
    }
}

fn  process_consumer(id: u32, pair_consumer: Arc<(Mutex<Vec<u32>>, Condvar)>){

    loop {
        let (mutex, cvar) = &*pair_consumer;
        let mut guard = cvar.wait_while(mutex.lock().unwrap(), |numbers|{
            println!("[Consumidro {:?}] Espero hasta que haya valores para consumir", id);
            numbers.len() == 0  
        }).unwrap();
            let value = guard.pop().unwrap();
            println!("[Consumidor {:?}] Consumi el valor {:?}", id, value);
            thread::sleep(Duration::from_secs(5));
            cvar.notify_one();
    }

}

fn  process_producer(pair_producer: Arc<(Mutex<Vec<u32>>, Condvar)>){

    loop {
        let (mutex, cvar) = &*pair_producer;
        let mut guard = cvar.wait_while(mutex.lock().unwrap(), |numbers|{
            println!("[Productor] Espero hasta que pueda agregar valores");
            numbers.len() >= 10  
        }).unwrap();
            let random_number : u32 = rand::thread_rng().gen_range(0..=100);
            guard.push(random_number);
            println!("[Productor] produci el valor {:?}", random_number);
            cvar.notify_one();
    }

}