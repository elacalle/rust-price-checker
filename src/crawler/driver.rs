use fantoccini;

use super::options::Capabilities;

pub struct Driver {
    pub client: fantoccini::Client,
}

impl Driver {
    pub fn new(url: &str, capabilities: Capabilities) -> Driver {
        let rt = tokio::runtime::Handle::current();

        let (tx, rx) = tokio::sync::oneshot::channel(); // Canal para comunicar el resultado

        // Ejecuta la operación en segundo plano
        tokio::spawn(async move {
            let mut binding = fantoccini::ClientBuilder::native();
            let future = binding
                .capabilities(capabilities.generate())
                .connect("http://127.0.0.1:4444");
            let result = future.await;
            tx.send(result).expect("Failed to send result");
        });

        // Espera a que la operación en segundo plano termine y obtiene el resultado
        let client = rt.block_on(rx).expect("Failed to receive result");

        return Driver {
            client: client.unwrap(),
        };
    }
}
