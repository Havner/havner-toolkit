mod uinput;
mod worker;

use crate::uinput::set_worker_tx;
use crate::worker::worker::spawn_worker;
use openaction::*;

struct GlobalEventHandler;
#[async_trait]
impl global_events::GlobalEventHandler for GlobalEventHandler {}

#[tokio::main]
async fn main() -> OpenActionResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    let tx = spawn_worker();

    set_worker_tx(tx).expect("Couldn't set worker tx");

    global_events::set_global_event_handler(&GlobalEventHandler);

    register_action(uinput::SimulationAction).await;

    run(std::env::args().collect()).await
}
