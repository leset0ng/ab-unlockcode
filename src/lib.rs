use astrobox_ng_wit::exports::astrobox::psys_plugin::{
    event_v3::{self, EventType},
    lifecycle,
};
use astrobox_ng_wit::FutureReader;

pub mod calc;
pub mod logger;
pub mod ui;

struct MyPlugin;

impl event_v3::Guest for MyPlugin {
    #[allow(async_fn_in_trait)]
    fn on_event(event_type: EventType, event_payload: String) -> FutureReader<String> {
        let (writer, reader) = astrobox_ng_wit::wit_future::new::<String>(|| "".to_string());

        match event_type {
            EventType::ProviderAction => {}
            _ => {}
        };

        tracing::info!("on_event type={:?} payload={}", event_type, event_payload);

        astrobox_ng_wit::spawn(async move {
            let _ = writer.write("".to_string()).await;
        });

        reader
    }

    fn on_ui_event_v3(
        event_id: String,
        event: event_v3::Event,
        event_payload: String,
    ) -> FutureReader<String> {
        let (writer, reader) = astrobox_ng_wit::wit_future::new::<String>(|| "".to_string());

        match event {
            event_v3::Event::Input => handle_input(&event_id, &event_payload),
            event_v3::Event::Change => handle_input(&event_id, &event_payload),
            event_v3::Event::Click => handle_click(&event_id),
            _ => {}
        }

        astrobox_ng_wit::spawn(async move {
            let _ = writer.write("".to_string()).await;
        });

        reader
    }

    fn on_ui_render(element_id: String) -> FutureReader<()> {
        let (writer, reader) = astrobox_ng_wit::wit_future::new::<()>(|| ());

        ui::set_root_id(element_id.clone());
        ui::render_main_ui(&element_id);

        astrobox_ng_wit::spawn(async move {
            let _ = writer.write(()).await;
        });

        reader
    }

    fn on_card_render(_card_id: String) -> FutureReader<()> {
        let (writer, reader) = astrobox_ng_wit::wit_future::new::<()>(|| ());

        astrobox_ng_wit::spawn(async move {
            let _ = writer.write(()).await;
        });

        reader
    }
}

impl lifecycle::Guest for MyPlugin {
    #[allow(async_fn_in_trait)]
    fn on_load() -> () {
        logger::init();
        tracing::info!("AB Unlock Code plugin loaded");
    }
}

astrobox_ng_wit::export!(MyPlugin);

fn handle_input(event_id: &str, payload: &str) {
    let value = parse_input_value(payload);

    match event_id {
        ui::EVENT_MAC_INPUT => ui::update_mac(value),
        ui::EVENT_SN_INPUT => ui::update_sn(value),
        _ => {}
    }
}

fn handle_click(event_id: &str) {
    match event_id {
        ui::EVENT_CALC_CLICK => {
            let (mac, sn, root_id) = {
                let state = ui::ui_state().lock().unwrap_or_else(|p| p.into_inner());
                (state.mac.clone(), state.sn.clone(), state.root_element_id.clone())
            };

            let mac_clean = mac.trim();
            let sn_clean = sn.trim();

            if mac_clean.is_empty() || sn_clean.is_empty() {
                ui::set_error("MAC 和 SN 都不能为空".to_string());
            } else {
                let code = calc::calc_unlock_code(mac_clean, sn_clean);
                ui::set_code(code);
            }

            if let Some(root_id) = root_id {
                ui::render_main_ui(&root_id);
            }
        }
        ui::EVENT_COPY_CLICK => {
            let code = {
                let state = ui::ui_state().lock().unwrap_or_else(|p| p.into_inner());
                state.code.clone()
            };

            if let Some(code) = code {
                astrobox_ng_wit::spawn(async move {
                    let _ = astrobox_ng_wit::astrobox::psys_host::clipboard::write_text(&code)
                        .await;
                });
            }
        }
        _ => {}
    }
}

fn parse_input_value(payload: &str) -> String {
    serde_json::from_str::<serde_json::Value>(payload)
        .ok()
        .and_then(|v| v.get("value").and_then(|v| v.as_str()).map(String::from))
        .unwrap_or_default()
}
