
/*
    escribir en rust productor-consumidor con canales
*/
extern crate rand;
use std::thread;
use std::time::Duration;
use rand::{Rng};
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};

fn process_producer(producer_id: i32, sender: Sender<u32>){
    loop {
        let product : u32 = rand::thread_rng().gen_range(0..=100);
        println!("[PRODUCER {:?} ]: produzco el numero {:?}", producer_id, product);
        let _ = sender.send(product);
        thread::sleep(Duration::from_secs(1));
    }
}
fn process_consumer(consumer_id: i32, receiver: Arc<Mutex<Receiver<u32>>>){
    loop{

        let product = receiver.lock().unwrap().recv().unwrap();
        println!("[CONSUMER {:?} ]: consumo el numero {:?}", consumer_id, product);
        thread::sleep(Duration::from_secs(2));
    }
}
fn main(){

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut producers = vec![];
    let mut consumers = vec![];
    for i in 1..5{
        let sender_clone = sender.clone();
        producers.push(thread::spawn(move || process_producer(i, sender_clone)));
    }
   for i in 1..7{
    let receiver_clone = receiver.clone();
    consumers.push(thread::spawn(move || process_consumer(i, receiver_clone)));
   }

   for producer in producers{
    let _ = producer.join();
   }

   for consumer in consumers{
    let _ = consumer.join();
   }
}

