use std::time::Duration;

use gloo_worker::{HandlerId, Worker, WorkerScope};
use log::info;
use serde::{Serialize, Deserialize};

use crate::{game::node_uttt::UtttState, gui::controller::run};

#[derive(Serialize, Deserialize)]
pub struct AIRequest {
    pub state: UtttState,
    pub seconds: u64,
}

#[derive(Serialize, Deserialize)]
pub struct AIResponse {
    pub best_move: Option<(usize, usize)>,
}

pub struct AIWorker;

impl Worker for AIWorker {
    type Input = AIRequest;
    type Message = ();
    type Output = AIResponse;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self
    }

    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}

    fn received(
        &mut self,
        scope: &WorkerScope<Self>,
        input: Self::Input,
        handler_id: HandlerId,
    ) {
        info!("Calculating best move...");
        let best_move = run(&input.state, Duration::from_secs(input.seconds));

        scope.respond(
            handler_id,
            AIResponse {
                best_move: best_move.map(|(b, c)| (b.0 as usize, c.0 as usize)),
            },
        );
    }
}
