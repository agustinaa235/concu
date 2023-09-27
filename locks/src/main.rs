extern crate rand;

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;
use rand::{Rng, thread_rng};
use std::collections::HashMap;

/* Realizar un pequeño idle game
    ● Algunas entidades "extraerán" (generarán aleatoriamente) oro
    ● Algunas entidades podrán convertir oro en recursos (a gusto)
    ● Otras entidades podrán convertir combinaciones de recursos en + oro
    ● Otras entidades podrán solamente consumir recursos
    ● Periódicamente se reporta por pantalla el nivel de recursos y oro
*/


const ENTIDADES: u32 = 4;



const MONTO_INICIAL : f64 = 0.0;
const RECURSO_INICIAL: i32 = 0;

fn main() {
   
    let oro = Arc::new(RwLock::new(MONTO_INICIAL));
    let hamburguesa = Arc::new(RwLock::new(RECURSO_INICIAL));
    let papas = Arc::new(RwLock::new(RECURSO_INICIAL));

   
    let mut entidades: Vec<JoinHandle<()>> =Vec::new();
    
    let oro_actual = oro.clone();
    let oro_consumir_hamburguesa = oro.clone();
    let hamburgesa_actual = hamburguesa.clone();
    let entidad1 =  thread::spawn(move || producir_oro(1, oro_actual));
    entidades.push(entidad1);

    let entidad2 =  thread::spawn(move || compro_hamburguesa(2, oro_consumir_hamburguesa, hamburgesa_actual));
    entidades.push(entidad2);
    

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("Estado del juego: Oro {:?} | hambuguesas {:?} ", oro.read(), hamburguesa.read());
    }
     
    
    
}

fn producir_oro(id:u32, oro_a_producir:Arc<RwLock<f64>>){
    loop {
        println!("Thread [{}] que produce oro", id);
        if let Ok(mut oro) = oro_a_producir.write(){
            let produzco_oro = 20.0;
            *oro += produzco_oro;
            thread::sleep(Duration::from_millis(1000));
        }
    }
}

fn compro_hamburguesa(id:u32, oro_a_consumir:Arc<RwLock<f64>>, hamburguesa_a_producir:Arc<RwLock<i32>>){
    loop {
        println!("Thread [{}] que produce hamburgesas", id);
    
        if let Ok(mut oro_actual) = oro_a_consumir.write(){
            if let Ok(mut hamburgesa) = hamburguesa_a_producir.write(){
                if *oro_actual >= 10.0 {
                    *oro_actual -= 10.0;
                    *hamburgesa +=1;
                }
            }
            thread::sleep(Duration::from_millis(1000));
        }
    }
}
