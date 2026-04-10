use tokio::sync::mpsc;
use crate::core::types::{Job, Response};
use crate::brain::intent::{classify, Intent};
use crate::brain::llm::ask_llm;
use crate::system::metrics::collect_system_state;

pub struct Engine {
    pub rx: mpsc::Receiver<Job>,
}

impl Engine {
    pub async fn run(mut self) {
        println!("Draco Engine running...");

        while let Some(job) = self.rx.recv().await {
            match job {
                Job::UserRequest(req) => {
                    let result = self.handle_request(req.input).await;
                    println!("Response: {}", result.output);
                }

                Job::BackgroundTask(task) => {
                    println!("Background task: {}", task);
                }
            }
        }
    }

    async fn handle_request(&self, input: String) -> Response {
        let intent = classify(&input);

        let system_state = collect_system_state();

        match intent {
            Intent::Chat => {
                let prompt = format!(
                    "System: {}\nUser: {}",
                    system_state, input
                );

                let reply = ask_llm(&prompt)
                    .await
                    .unwrap_or("LLM error".into());

                Response { output: reply }
            }

            Intent::Command(cmd) => {
                // placeholder (will connect sandbox later)
                Response {
                    output: format!("Command detected: {}", cmd),
                }
            }
        }
    }
}
