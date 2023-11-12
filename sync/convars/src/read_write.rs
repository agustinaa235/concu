
/*
Un estado se comparte entre varios procesos.
Algunos procesos necesitan actualizar dicho estado,
mientras que otros solo necesitan leerlo.
Mientras que un proceso está leyendo el estado, otros
pueden leerlo, pero ninguno modificarlo

Mientras que un proceso está modificando el estado,
ningun otro puede leerlo ni modificarlo.

*/
use std::sync::{Arc, Mutex,Condvar};
use std::thread;
use std::time::Duration;

const LECTORES : usize = 10;
const ESCRITORES : usize = 3;

#[derive(Debug)]
struct ReadWrite {
    readers: i32,
    writing: bool
}


fn main(){

    let read_write = Arc::new((Mutex::new(ReadWrite{ readers: 0, writing: false }), Condvar::new()));

    let mut lectores = vec![];
    let mut escritores = vec![];
    for lector in 0..=LECTORES{
        let read_write_clone = read_write.clone();
        lectores.push(move || thread::spawn(process_readers(lector, read_write_clone)));
    }

    for escritor in 0..ESCRITORES{
        let read_write_clone = read_write.clone();
        escritores.push(move || thread::spawn(process_writers(escritor, read_write_clone)));
    }

    for lector in lectores{
        lector.join();
    }
    for escritor in escritores{
        escritor.join();
    }

}

fn process_readers(id: usize, read_write: Arc<(Mutex<ReadWrite>, Condvar)>){
    let (mutex, cvar) = &*read_write;
    let mut _guard = cvar.wait_while(mutex.lock().unwrap(), |rw|{
        rw.writing // esperan solo si hay escritores escribiendo
    }).unwrap();
    _guard.readers += 1;
    println!("Proceso {:?} empieza  a leer", id);
    thread::sleep(Duration::from_millis(1000));
    println!("proceso {:?} termina de leer", id);
    _guard.readers -= 1;
    cvar.notify_all();
}

fn process_writers(id: usize, read_write: Arc<(Mutex<ReadWrite>, Condvar)>){
    let (mutex, cvar) = &*read_write;
    let mut _guard = cvar.wait_while(mutex.lock().unwrap(), |rw|{
        rw.writing & rw.readers > 0 // esperan si hay escritores escribiendo y si hay lectores
    }).unwrap();
    _guard.writing = true;
    println!("Proceso {:?} empieza  a escribir", id);
    thread::sleep(Duration::from_millis(3000));
    println!("proceso {:?} termina de escribir", id);
    _guard.writing = false;
    cvar.notify_all();
}





