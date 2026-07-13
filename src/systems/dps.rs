use tokio::{
    sync::mpsc::{Receiver, Sender},
    task::JoinSet,
};

use crate::query_state::QueryState;

pub struct DecisionPipelineSystem {
    ies_to_dps_rx: Receiver<QueryState>,
    dps_to_srs_tx: Sender<QueryState>,
}

impl DecisionPipelineSystem {
    pub async fn run(mut self) -> JoinSet<()> {
        println!("Starting DPS...");

        let mut join_set = JoinSet::<()>::new();

        // Receive query states from the IES
        join_set.spawn(async move {
            loop {
                let message = match self.ies_to_dps_rx.recv().await {
                    None => {
                        panic!("IES-DPS channel closed unexpectedly.");
                    }
                    Some(message) => message,
                };

                println!("[DPS] RECEIVED QUERY FROM IES");

                // 1. Interrogate cache

                // 2. Interrogate LZA

                // 3. Interrogate SHS

                // 4. Forward query to SRS
                println!("[DPS] SENDING QUERY TO SRS"); 
                if let Err(e) = self.dps_to_srs_tx.try_send(message) {
                    match e {
                        tokio::sync::mpsc::error::TrySendError::Closed(_) => {
                            panic!("DPS-SRS channel closed unexpectedly.");
                        }
                        tokio::sync::mpsc::error::TrySendError::Full(_) => {
                            println!("DPS-SRS channel overloaded. Dropping query.");
                        }
                    }
                }
            }
        });

        join_set
    }

    pub fn new(ies_to_dps_rx: Receiver<QueryState>, dps_to_srs_tx: Sender<QueryState>) -> Self {
        Self {
            ies_to_dps_rx,
            dps_to_srs_tx,
        }
    }
}
