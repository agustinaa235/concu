use std::sync::{Arc, Mutex};
use tokio::task;

struct MyStruct {
    connections: Mutex<Vec<u32>>,
}

impl MyStruct {
    async fn my_method(&self) {
        let mut data = self.connections.lock().unwrap();
        // Realizar operaciones seguras con data aquí
        data.push(42);
    }
}

#[tokio::main]
async fn main() {
    let my_struct = Arc::new(MyStruct {
        connections: Mutex::new(Vec::new()),
    });

    let cloned_struct = Arc::clone(&my_struct);

    let handle = task::spawn(async move {
        process_connection(cloned_struct).await;
    });

    handle.await.unwrap();
}

async fn process_connection(my_struct: Arc<MyStruct>) {
    my_struct.my_method().await;
}
