
/*
    1era fecha diciembre 2022
    Ejercicio de codeo (en papel) de un problema de productor-consumidor 
    con actores. El productor generaba números aleatorios cada un cierto 
    intervalo, y el consumidor los iba acumulando (sumando) en su estado 
    interno. También el consumidor debe tener la capacidad de poder 
    consultarle el acumulado desde un sistema externo.
*/
extern crate actix;
use actix::{Actor, Context, Handler, System, Message, Addr, AsyncContext};
use std::time::Duration;
use rand::Rng;

#[derive(Message)]
#[rtype(result = "()")]
struct Producir ();

#[derive(Message)]
#[rtype(result = "()")]
struct Consumir (u32);

#[derive(Message)]
#[rtype(result = "Result<u32,()>")]
struct Acumulado;


struct Producer {
    interval: Duration,
    consumer: Addr<Consumer>,
    }

struct Consumer {
    acumulated: u32
}

impl Actor for Consumer{
    type Context = Context<Self>;
}

impl Actor for Producer{
    type Context = Context<Self>;
}

impl Handler<Producir> for Producer {
    type Result = ();

    fn handle(&mut self, _: Producir, ctx: &mut Self::Context) {
        // Enviar mensajes al consumidor cada 2 segundos
        ctx.run_interval(self.interval, |act, _| {
            let numero : u32 = rand::thread_rng().gen_range(0..=100);
            println!("[Productor] : producto numero {:?}", numero);
            act.consumer.do_send(Consumir(numero));
        });
    }
}


impl Handler<Consumir> for Consumer {
    type Result = ();
    fn handle(&mut self, _msg:Consumir, _ctx: &mut Context<Self>)-> Self::Result {
        let value = _msg.0; // valor a consumir 
        self.acumulated += value;
        println!("[Consumidor]: consumo número {}. Acumulado: {}", _msg.0, self.acumulated);
    }
} 

impl Handler<Acumulado> for Consumer {
    type Result = Result<u32, ()>;
    fn handle(&mut self, _msg:Acumulado, _ctx: &mut Context<Self>) -> Self::Result {
        Ok(self.acumulated)
    }
}

fn main(){

    let system = System::new();
    system.block_on(async {
        let consumer = Consumer {acumulated: 0}.start();
        let consumer_clone = consumer.clone();
        Producer { interval: Duration::from_secs(1), consumer: consumer_clone}.start().do_send(Producir());

        let acumulado = consumer.send(Acumulado).await.unwrap();
        println!("Acumulado desde sistema externo: {:?}", acumulado);

    });
    system.run().unwrap();

}