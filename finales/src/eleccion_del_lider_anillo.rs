
/*
    Escribir un programa de rust que implemente el 
    algoritmo del anillo para elección de líder con actores.
*/

extern crate actix;
use actix::{Actor, Context, Handler, System, Message, Addr};

#[derive(Message)]
#[rtype(result = "Result<(),()>")]
struct StartElection {}

#[derive(Message)]
#[rtype(result = "Result<(),()>")]
struct Election {
    nodes: Vec<u32>
}

#[derive(Message)]
#[rtype(result = "Result<(),()>")]
struct Coordinator {
    node_coordinator_id: u32
}

#[derive(Message)]
#[rtype(result = "Result<(),()>")]
struct NodeConnected {
    next_node: Addr<Node>
}

struct Node{
    id: u32,
    neighboar_addr : Option<Addr<Node>>,
    coordinator_id: u32
}

impl Actor for Node{
    type Context = Context<Self>;
}

impl Handler<StartElection> for Node {

    type Result = Result<(),()>;
    
    fn handle(&mut self, _msg: StartElection, _ctx: &mut Context<Self>) -> Self::Result {
        match &self.neighboar_addr {
            Some(addr) => {
                println!("[Node {}] comenzo la eleccion", self.id);
                let neighbors_vectors = vec![self.id];
                addr.try_send(Election { nodes: neighbors_vectors }).unwrap();
            }
            None => {
                println!("[Node {}] No neighbor to send Election message.", self.id);
            }
        }
        Ok(())
    }
    
}

impl Handler<Election> for Node {
    type Result = Result<(),()>;
    fn handle(&mut self, _msg: Election, _ctx: &mut Context<Self>) -> Self::Result {
        let mut neighboars_vectors = _msg.nodes;
        if neighboars_vectors.contains(&self.id) {
            if let Some(coordinator_id) = neighboars_vectors.iter().max() {
                println!("[Node {:?}], Ya se recorrio todo el anillo y el coordinador es: {}", self.id, coordinator_id);
                self.coordinator_id = *coordinator_id;
                if let Some(addr) = &self.neighboar_addr {
                    addr.try_send(Coordinator {
                        node_coordinator_id: *coordinator_id,
                    })
                    .unwrap();
                }
            } else {
                println!("El vector está vacío");
            }
        } else {
            println!("[Node {:?}],Todavia no recorrio el anillo", self.id);
            neighboars_vectors.push(self.id);
            if let Some(addr) = &self.neighboar_addr {
                addr.try_send(Election {
                    nodes: neighboars_vectors,
                })
                .unwrap();
            }
        }
        Ok(())
    }
}

impl Handler<Coordinator> for Node {
    type Result = Result<(),()>;
    fn handle(&mut self, _msg: Coordinator, _ctx: &mut Context<Self>) -> Self::Result {
        if self.coordinator_id == _msg.node_coordinator_id {
            println!("[Node {:?}] ya todos saben quien es el coordiantor", self.id);
        } else {
            self.coordinator_id = _msg.node_coordinator_id;
            println!("[Node {:?}] todavia no todos saben quien es el coordinador", self.id);
            if let Some(addr) = &self.neighboar_addr {
                addr.try_send(Coordinator {
                    node_coordinator_id: _msg.node_coordinator_id,
                })
                .unwrap();
            }
        }
        Ok(())
    }
}

impl Handler<NodeConnected> for Node {
    type Result = Result<(),()>;
    fn handle(&mut self, _msg: NodeConnected, _ctx: &mut Context<Self>)-> Self::Result {
        self.neighboar_addr = Some(_msg.next_node);
        Ok(())
    }
}

fn main(){
    let system = System::new();
    system.block_on(async {
        let node1 = Node { id: 1, neighboar_addr: None , coordinator_id: 0}.start();
        let node2 = Node { id: 2, neighboar_addr: None , coordinator_id: 0}.start();
        let node3 = Node { id: 3, neighboar_addr: None , coordinator_id: 0}.start();

        // Configurar los nodos en el anillo
        node1.do_send(NodeConnected { next_node: node2.clone() });
        node2.do_send(NodeConnected { next_node: node3.clone() });
        node3.do_send(NodeConnected { next_node: node1.clone() });

        node1.do_send(StartElection{});
    });
    // Ejecutar el sistema
    system.run().unwrap();
}