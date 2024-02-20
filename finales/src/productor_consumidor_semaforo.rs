


extern crate rand;
extern crate std_semaphore;

use crate::rand::Rng;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std_semaphore::Semaphore;
use std::thread;


fn main(){

    let mut productores = vec![];
    let mut consumidores = vec![];

    let is_empty = Arc::new(Semaphore::new(0));
    let is_full = Arc::new(Semaphore::new(10));
    let numbers = Arc::new(RwLock::new(Vec::new()));

    for id in 0..5{
        let is_empty_clone_p = is_empty.clone();
        let is_full_clone_p =  is_full.clone();
        let numbers_clone_p = numbers.clone();
        productores.push(thread::spawn(move || procesar_productores(id, numbers_clone_p, is_empty_clone_p, is_full_clone_p)));
    }

    for id in 0..3{
        let is_empty_clone_c = is_empty.clone();
        let is_full_clone_c =  is_full.clone();
        let numbers_clone_c = numbers.clone();
        consumidores.push(thread::spawn(move || procesar_consumidores(id, numbers_clone_c, is_empty_clone_c, is_full_clone_c)));
    }

    for producer in productores{
        let _ = producer.join();
    }
    for consumer in consumidores{
        let _ = consumer.join();
    }  
}

fn procesar_productores(producer_id: u32, numbers: Arc<RwLock<Vec<u32>>>, is_empty : Arc<Semaphore>, is_full: Arc<Semaphore>){
    loop {
        println!("[PRODUTOR {:?}]: esperando para producer", producer_id);
        is_full.acquire(); // si esta llena la queue espero
        let random_number : u32 = rand::thread_rng().gen_range(0..=100);
        
        if let Ok(mut buffer) = numbers.write(){
            let _ = &buffer.push(random_number);
            println!("[PRODUCTOR {:?}] va a producir numero {:?}", producer_id, random_number);
        }
        thread::sleep(Duration::from_secs(1));
        is_empty.release(); // aviso que ya no esta vacio
    }
}


fn procesar_consumidores(consumidor_id: u32, numbers: Arc<RwLock<Vec<u32>>>, is_empty : Arc<Semaphore>, is_full: Arc<Semaphore>){
    loop {
        println!("[CONSUMIDOR {:?}]: esperando para producer", consumidor_id);
        is_empty.acquire(); //espero a que me digan si puedo consumir 
        if let Ok(mut buffer) = numbers.write(){
            let number = &buffer.pop();
            println!("[CONSUMIDOR {:?}] va a consumir el numero {:?}", consumidor_id, number);
        }
        thread::sleep(Duration::from_secs(3));
        is_full.release(); // aviso que ya no esta lleno 
    }
}

