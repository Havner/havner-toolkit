pub mod types;

use openaction::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value::Null, json};
use std::sync::mpsc::Sender;
use std::sync::{OnceLock};

use types::Token;

static WORKER_TX: OnceLock<Sender<Vec<Token>>> = OnceLock::new();

pub fn set_worker_tx(tx: Sender<Vec<Token>>) -> Result<(), Sender<Vec<Token>>> {
    WORKER_TX.set(tx)
}

async fn execute_input(value: Option<String>) -> Result<(), anyhow::Error> {
    let Some(value) = value else {
        return Ok(());
    };
    if value.trim().is_empty() {
        return Ok(());
    }

    if let Some(tx) = WORKER_TX.get() {
        let tokens: Vec<Token> = ron::from_str(&value)?;
        tx.send(tokens)?;
    }

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
    const UUID: &'static str = "com.havner.uinput.simulation";
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
}
