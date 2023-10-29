extern crate std_semaphore;


extern crate rand;
use std::thread;
use std::sync::Arc;
use std::time::Duration;
use std_semaphore::Semaphore;
use rand::{thread_rng, Rng};

/*
Una barberia tiene una sala de espera con sillas.
Si la barberia esta vacia, el barbero se pone a dormir.
Si un cliente entra y el barbero esta durmiendo, lo
despierta.
Y si el barbero esta atendiendo, se sienta en una de las
sillas y espera su turno.
El cliente espera a que le corten el pelo
*/

const CLIENTES: i32 = 5;
fn main(){

    let barbero_atiende = Arc::new(Semaphore::new(0));
    let clientes_esperando = Arc::new(Semaphore::new(0));
    let barbero_esta_libre = Arc::new(Semaphore::new(0));

    let barbero_atiende_clone = barbero_atiende.clone();
    let clientes_esperando_clone = clientes_esperando.clone();
    let corte_terminado_clone = barbero_esta_libre.clone();
    let barbero = thread::spawn(move || procesar_barbero(corte_terminado_clone, barbero_atiende_clone, clientes_esperando_clone));

    let mut clientes = vec![];
    for id in 0..=CLIENTES{
        let barbero_atiende_cliente = barbero_atiende.clone();
        let clientes_espero_cliente = clientes_esperando.clone();
        let corte_terminado_cliente = barbero_esta_libre.clone();
        clientes.push(thread::spawn(move || procesar_cliente(id, corte_terminado_cliente, barbero_atiende_cliente, clientes_espero_cliente)))
    }

    for cliente in clientes {
        cliente.join().unwrap();
    }
    let _ = barbero.join();
}


fn procesar_barbero(corte_terminado_clone: Arc<Semaphore>, barbero_atiende: Arc<Semaphore>, clientes_esperado: Arc<Semaphore>){
    loop {
        println!("[Barbero]: esperando a clientes");
        clientes_esperado.acquire(); // barbero espera a que llegue un cliente con un wait, cuando llega un cliente avanza 

        println!("[Barbero]: barbero cortanto el pelo");
        barbero_atiende.release(); // el barbero lo atiende

        thread::sleep(Duration::from_secs(2));

        println!("[Barbero]: barbero termine");
        corte_terminado_clone.release(); // barbero libera el semaforo de que termino de cortar el pelo
    }
}



fn procesar_cliente(id: i32, corte_terminado_clone:  Arc<Semaphore>, barbero_atiende: Arc<Semaphore>, clientes_esperado: Arc<Semaphore>){
    loop{
        println!("[Cliente {}]: recien entra el cliente", id);
        thread::sleep(Duration::from_secs(thread_rng().gen_range(2, 10)));
        clientes_esperado.release(); // libero el semaror del cliente esperando
        println!("[Cliente {}]: espero al que el barbero este listo", id);
        barbero_atiende.acquire(); // todo el semaphoro y espero a que el barbero este listo para cortarme el pelo

        println!("[Cliente {}] Esperando a que me termine de cortar", id);
        corte_terminado_clone.acquire();
        println!("[Cliente {}]: me corto el pelo", id);
    }
}