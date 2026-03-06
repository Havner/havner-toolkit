mod uinput;

use openaction::*;

struct GlobalEventHandler;
#[async_trait]
impl global_events::GlobalEventHandler for GlobalEventHandler {}

#[tokio::main]
async fn main() -> OpenActionResult<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    global_events::set_global_event_handler(&GlobalEventHandler);
    register_action(uinput::SimulationAction).await;

    run(std::env::args().collect()).await
}
