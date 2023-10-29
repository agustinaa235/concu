extern crate std_semaphore;


use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std_semaphore::Semaphore;



/**
Cinco filósofos se sientan alrededor de una mesa y pasan su vida cenando y pensando.
Cada filósofo tiene un plato de fideos y un palito chino a la izquierda de su plato.
Para comer los fideos son necesarios dos palitos y cada filósofo sólo puede tomar los que
están a su izquierda y derecha. Si cualquier filósofo toma un palito y el otro está ocupado,
se quedará esperando, con el tenedor en la mano, hasta que pueda tomar el otro tenedor,
para luego empezar a comer.
*/

const NUM_FILOSOFOS: usize = 5;

fn main() {
    
    let tenedores = (0..NUM_FILOSOFOS)
    .map(|_| Arc::new(Semaphore::new(1)))
    .collect::<Vec<_>>();

    let tomo_ambos = Arc::new(Semaphore::new(4));
    let mut filosofos = vec![];
    for fil in 0..NUM_FILOSOFOS{
        let tenedores_clone = tenedores.clone();
        let tomo_ambos_clone = tomo_ambos.clone();
        filosofos.push(thread::spawn(move || procesar_filosofo(fil as usize, tenedores_clone, tomo_ambos_clone)));
    }

    for fil in filosofos{
        fil.join().unwrap();
    }
}

fn como(id: usize){
    println!("[Filosofo {:?}] comiendo", id);
    thread::sleep(Duration::from_millis(1000));
}

fn procesar_filosofo(id: usize ,tenedores: Vec<Arc<Semaphore>>, tomo_ambos: Arc<Semaphore>){

    loop {

        tomo_ambos.acquire();

        println!("[Filosofo {:?}] va a tomar los palitos", id);
        tenedores[id ].acquire(); // Tomo un tenedor izquierdo
        tenedores[(id+ 1)% NUM_FILOSOFOS].acquire(); // Tomo el tenedor derecho 

        como(id);

        tenedores[id].release(); // Tomo un tenedor izquierdo
        tenedores[(id + 1)% NUM_FILOSOFOS].release(); // Tomo el tenedor derecho

        println!("[Filosofo {:?}] termino de comer", id);
        tomo_ambos.release();
    }
}