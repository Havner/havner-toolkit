mod device;
mod types;

use openaction::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value::Null, json};
use std::sync::LazyLock;
use tokio::sync::Mutex;

use device::Uinput;
use types::Token;

static UINPUT: LazyLock<Mutex<Option<Uinput>>> = LazyLock::new(|| Mutex::new(Option::None));

async fn execute_input(value: Option<String>) -> Result<(), anyhow::Error> {
    let Some(value) = value else {
        return Ok(());
    };
    if value.trim().is_empty() {
        return Ok(());
    }

    let mut uinput_guard = UINPUT.lock().await;
    std::thread::spawn(move || -> Result<(), anyhow::Error> {
        if uinput_guard.is_none() {
            uinput_guard.replace(Uinput::new()?);
            // give a little time for initialization so the first event works
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
        let uinput = uinput_guard.as_mut().ok_or(anyhow::anyhow!("lock failed"))?;
        let tokens: Vec<Token> = ron::from_str(&value)?;
        for token in tokens {
            uinput.execute(token)?;
        }
        Ok(())
    })
    .join()
    .unwrap_or(Ok(()))?;

    Ok(())
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct SimulationSettings {
    down: Option<String>,
    up: Option<String>,
    anticlockwise: Option<String>,
    clockwise: Option<String>,
}

pub struct SimulationAction;

#[async_trait]
impl Action for SimulationAction {
    const UUID: &'static str = "com.havner.toolbox.uinputsimulation";
    type Settings = SimulationSettings;

    async fn key_down(
        &self,
        instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        if let Err(error) = execute_input(settings.down.clone()).await {
            log::warn!("Failed to simulate input: {error}");
            instance.send_to_property_inspector(json!({ "error": error.to_string() })).await?;
        } else if settings.down.as_ref().is_some_and(|x| !x.trim().is_empty()) {
            instance.send_to_property_inspector(json!({ "error": Null })).await?;
        }
        Ok(())
    }

    async fn key_up(&self, instance: &Instance, settings: &Self::Settings) -> OpenActionResult<()> {
        if let Err(error) = execute_input(settings.up.clone()).await {
            log::warn!("Failed to simulate input: {error}");
            instance.send_to_property_inspector(json!({ "error": error.to_string() })).await?;
        } else if settings.up.as_ref().is_some_and(|x| !x.trim().is_empty()) {
            instance.send_to_property_inspector(json!({ "error": Null })).await?;
        }
        Ok(())
    }

    async fn dial_down(
        &self,
        instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        self.key_down(instance, settings).await
    }

    async fn dial_up(
        &self,
        instance: &Instance,
        settings: &Self::Settings,
    ) -> OpenActionResult<()> {
        self.key_up(instance, settings).await
    }

    async fn dial_rotate(
        &self,
        instance: &Instance,
        settings: &Self::Settings,
        ticks: i16,
        _pressed: bool,
    ) -> OpenActionResult<()> {
        let input = if ticks < 0 { &settings.anticlockwise } else { &settings.clockwise };
        for _ in 0..ticks.abs() {
            if let Err(error) = execute_input(input.clone()).await {
                log::warn!("Failed to simulate input: {error}");
                instance.send_to_property_inspector(json!({ "error": error.to_string() })).await?;
            } else if input.as_ref().is_some_and(|x| !x.trim().is_empty()) {
                instance.send_to_property_inspector(json!({ "error": Null })).await?;
            }
        }
        Ok(())
    }
}
