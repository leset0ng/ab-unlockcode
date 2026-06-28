use astrobox_ng_wit::FutureReader;

use astrobox_ng_wit::exports::astrobox::psys_plugin::{
    event::{self, EventType},
    lifecycle,
};

pub mod logger;
pub mod ui;
pub mod resources;

struct MyPlugin;

impl event::Guest for MyPlugin {
    #[allow(async_fn_in_trait)]
    fn on_event(event_type: EventType, event_payload: String) -> FutureReader<String> {
        let (writer, reader) = astrobox_ng_wit::wit_future::new::<String>(|| "".to_string());

        match event_type {
            EventType::PluginMessage => {}
            EventType::InterconnectMessage => {}
            EventType::DeviceAction => {}
            EventType::ProviderAction => {}
            EventType::DeeplinkAction => {}
            EventType::TransportPacket => {}
            EventType::Timer => {}
        };

        tracing::info!("event_payload: {}", event_payload);

        astrobox_ng_wit::spawn(async move {
            let _ = writer.write("".to_string()).await;
        });

        reader
    }

    fn on_ui_event(
        event_id: String,
        event: event::Event,
        _event_payload: String,
    ) -> astrobox_ng_wit::FutureReader<String> {
        let (writer, reader) = astrobox_ng_wit::wit_future::new::<String>(|| "".to_string());

        ui::ui_event_processor(event, &event_id);

        astrobox_ng_wit::spawn(async move {
            let _ = writer.write("".to_string()).await;
        });

        reader
    }

    fn on_ui_render(element_id: String) -> astrobox_ng_wit::FutureReader<()> {
        let (writer, reader) = astrobox_ng_wit::wit_future::new::<()>(|| ());

        ui::render_main_ui(&element_id);

        astrobox_ng_wit::spawn(async move {
            let _ = writer.write(()).await;
        });

        reader
    }

    fn on_card_render(_card_id: String) -> astrobox_ng_wit::FutureReader<()> {
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
        tracing::info!("Hello AstroBox V2 Plugin!");
    }
}

astrobox_ng_wit::export!(MyPlugin);
