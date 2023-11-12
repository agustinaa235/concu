use rand::Rng;
extern crate std_semaphore;
use std::sync::{Arc, RwLock};
use std_semaphore::Semaphore;
use std::thread;

const CANTIDAD :usize  = 10;


fn main(){

    let sem = Arc::new(Semaphore::new(3));
    let contador = Arc::new(RwLock::new(0));

    let mut process = vec![];
    for i in 0..=CANTIDAD {
        let sem_clone = sem.clone();
        let contador_clone = contador.clone();
        process.push(thread::spawn(move || contador_process(i, sem_clone, contador_clone)));
    }

    
    for cliente in process {
        cliente.join().unwrap();
    }


}

fn contador_process(id: usize, sem: Arc<Semaphore>, contador: Arc<RwLock<u32>>){
    loop{
        println!("Proceso {:?} esperando para sumar ", id);
        sem.acquire();
        if let Ok(mut cantidad) = contador.write() {
            let amount: u32 =  rand::thread_rng().gen() ;
            *cantidad += amount / 10000000;
            println!("[Proceso {:?}]: agrega  {:?} y quedan {:?}", id, amount, *cantidad);
        }
        sem.release();
        println!("Proceso {:?} termina de sumar ", id);
    }
}