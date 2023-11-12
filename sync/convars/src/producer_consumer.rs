use std::sync::{Arc, Mutex,Condvar};
use std::thread;



const LIMIT: usize = 10;


fn main(){

    let pair = Arc::new((Mutex::new(0), Condvar::new()));
    let pair_agent_producer = pair.clone();

    let pair_agent_consumer = pair.clone();

    let producer = thread::spawn(move || producer_process(pair_agent_producer));

    let consumer = thread::spawn(move || consumer_process(pair_agent_consumer));

    let _ = producer.join();
    let _ = consumer.join();
    

}


fn producer_process(pair_agent_producer: Arc<(Mutex<usize>, Condvar)>){
    loop{
        let (mutex, cvar) = &*pair_agent_producer;
        // produzco solo cuando hay espacio, sino devo esperar 
        let mut guard = cvar.wait_while(mutex.lock().unwrap(), |productos| {
        println!("Espero hasta que la cantidad de productos sea menor al limite");
        *productos == LIMIT
        }).unwrap();
        *guard += 1;
        println!("Produzco {:?}", *guard);
        cvar.notify_all();
    }
}

fn consumer_process(pair_agent_consumer:  Arc<(Mutex<usize>, Condvar)>){
    loop{
        let (mutex, cvar) = &*pair_agent_consumer;
        // Consumo solo cuando hay productos para consumir sino espero
        let mut guard = cvar.wait_while(mutex.lock().unwrap(), |productos| {
        println!("Espero hasta que haya productos para consumir");
        *productos == 0
    }).unwrap();
        *guard -=1;
        println!("Consumo {:?}", *guard);
        cvar.notify_all();
    }
}