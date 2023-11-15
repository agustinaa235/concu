
use std::sync::{Mutex,Condvar};

struct Semaforo {
    cvar: Condvar,
    mutex :  Mutex<usize>
}

impl Semaforo {


    pub fn new () -> Semaforo{
        Semaforo{
            cvar:  Condvar::new(),
            mutex: Mutex::new(1)
        }
    }
    pub fn adquire(&self){
        let mut aux = self.cvar.wait_while(self.mutex.lock().unwrap(), |contador| {
            println!("Espero hasta q el semaforo se libere");
            *contador <= 0
        }).unwrap();
        println!("Puedo tomar semaforo");
        *aux -= 1;
        self.cvar.notify_one();
    }

    pub fn release(&self){
        let mut contador = self.mutex.lock().unwrap();
        println!("Libero semaforo");
        *contador += 1;

        self.cvar.notify_one();
    }
}

fn main() {
    let sem = Semaforo::new();
    sem.adquire();
    sem.release();
}
