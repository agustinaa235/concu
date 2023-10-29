extern crate rand;
extern crate std_semaphore;


use std::sync::{Arc, RwLock};
use std::time::Duration;
use std_semaphore::Semaphore;
use std::thread;
/* 
Consumidor-productor donde tengo hilos que agregan canicas al bowl y otros que sacan canicas del bowl
*/

const PRODUCTORES : i32 = 3;

const CONSUMIDORES : i32 = 10;

fn main() {
    let bowl_no_vacio = Arc::new(Semaphore::new(0));
    let bowl_no_lleno = Arc::new(Semaphore::new(15));
    let bowl_con_canicas = Arc::new(RwLock::new(0));

    let mut productores = vec![];
    let mut consumidores = vec![];

    for i in 0..PRODUCTORES {
        let bowl_no_vacio_productores = bowl_no_vacio.clone();
        let bowl_no_lleno_productores = bowl_no_lleno.clone();
        let bowl_con_canicas_productores = bowl_con_canicas.clone();
        productores.push(thread::spawn(move || procesar_productores(i, bowl_no_lleno_productores, bowl_no_vacio_productores, bowl_con_canicas_productores)));
    }

    for i in 0..CONSUMIDORES {
        let bowl_no_vacio_consumidores = bowl_no_vacio.clone();
        let bowl_no_lleno_consumidores = bowl_no_lleno.clone();
        let bowl_con_canicas_consumidores = bowl_con_canicas.clone();
        consumidores.push(thread::spawn(move || procesar_consumidores(i, bowl_no_lleno_consumidores, bowl_no_vacio_consumidores, bowl_con_canicas_consumidores)));
    }

    for consumidor in consumidores {
        consumidor.join().unwrap();
    }

    for productor in productores {
        productor.join().unwrap();
    }

}

fn procesar_productores(prod_id: i32, bowl_no_lleno: Arc<Semaphore>, bowl_no_vacio: Arc<Semaphore>, bowl_con_canicas: Arc<RwLock<i32>>){

    loop {
        
        bowl_no_lleno.acquire();
        if let Ok(mut canicas) = bowl_con_canicas.write() {
            *canicas += 1;
            println!("[PRODUCTOR {:?}]: agrega una canica y quedan {:?}", prod_id, *canicas);
        }
       
        bowl_no_vacio.release();
    }
}

fn procesar_consumidores(cons_id: i32,  bowl_no_lleno: Arc<Semaphore>, bowl_no_vacio: Arc<Semaphore>, bowl_con_canicas: Arc<RwLock<i32>>){
    loop {
        bowl_no_vacio.acquire();
        if let Ok(mut canicas) = bowl_con_canicas.write(){
            *canicas -= 1;
            println!("[CONSUMIDOR {:?}]: saca una canica y quedan {:?}", cons_id, *canicas);
        }
       
        bowl_no_lleno.release();
        thread::sleep(Duration::from_secs(2));
        println!("[CONSUMIDOR {:?}]: consume canica", cons_id);
        
    }
}
