use astrobox_ng_wit::astrobox::psys_host::ui_v3::{self, Element};
use std::sync::{Mutex, OnceLock};

pub const EVENT_MAC_INPUT: &str = "mac_input";
pub const EVENT_SN_INPUT: &str = "sn_input";
pub const EVENT_CALC_CLICK: &str = "calc_click";

#[derive(Default)]
pub struct UiState {
    pub mac: String,
    pub sn: String,
    pub code: Option<String>,
    pub error: Option<String>,
    pub root_element_id: Option<String>,
}

static UI_STATE: OnceLock<Mutex<UiState>> = OnceLock::new();

pub fn ui_state() -> &'static Mutex<UiState> {
    UI_STATE.get_or_init(Mutex::default)
}

pub fn update_mac(value: String) {
    let mut state = ui_state().lock().unwrap_or_else(|p| p.into_inner());
    state.mac = value;
    state.code = None;
    state.error = None;
}

pub fn update_sn(value: String) {
    let mut state = ui_state().lock().unwrap_or_else(|p| p.into_inner());
    state.sn = value;
    state.code = None;
    state.error = None;
}

pub fn set_code(code: String) {
    let mut state = ui_state().lock().unwrap_or_else(|p| p.into_inner());
    state.code = Some(code);
    state.error = None;
}

pub fn set_error(error: String) {
    let mut state = ui_state().lock().unwrap_or_else(|p| p.into_inner());
    state.error = Some(error);
    state.code = None;
}

pub fn set_root_id(id: String) {
    let mut state = ui_state().lock().unwrap_or_else(|p| p.into_inner());
    state.root_element_id = Some(id);
}

fn input_row(
    label: &str,
    placeholder: &str,
    default_value: &str,
    event_id: &str,
) -> Element {
    let label_el = Element::new(ui_v3::ElementType::P, Some(label))
        .size(13)
        .text_color("#888888")
        .margin_bottom(3);

    let input = Element::new(ui_v3::ElementType::Input, None)
        .prop("type", "text")
        .prop("placeholder", placeholder)
        .prop("default-value", default_value)
        .width_full()
        .padding_left(10)
        .padding_right(10)
        .padding_top(5)
        .padding_bottom(5)
        .size(14)
        .bg("#1e1e1e")
        .text_color("#ffffff")
        .border(1, "#333333")
        .radius(6)
        .on(ui_v3::Event::Input, event_id);

    Element::new(ui_v3::ElementType::Div, None)
        .width_full()
        .margin_bottom(12)
        .child(label_el)
        .child(input)
}

pub fn build_ui(state: &UiState) -> Element {
    let title = Element::new(ui_v3::ElementType::P, Some("AB Unlock Code"))
        .size(20)
        .text_color("#ffffff")
        .margin_bottom(4);

    let subtitle = Element::new(
        ui_v3::ElementType::P,
        Some("输入 MAC 与 SN 计算小米设备解锁码"),
    )
    .size(12)
    .text_color("#888888")
    .margin_bottom(18);

    let mac_row = input_row(
        "MAC 地址",
        "例如 00:11:22:33:44:55",
        &state.mac,
        EVENT_MAC_INPUT,
    );

    let sn_row = input_row("序列号 (SN)", "例如 SN123456789", &state.sn, EVENT_SN_INPUT);

    let calc_button = Element::new(ui_v3::ElementType::Button, Some("计算解锁码"))
        .width_full()
        .padding(10)
        .bg("#ff6b00")
        .text_color("#ffffff")
        .radius(8)
        .size(15)
        .on(ui_v3::Event::Click, EVENT_CALC_CLICK);

    let mut children: Vec<Element> = vec![
        title,
        subtitle,
        mac_row,
        sn_row,
        calc_button,
    ];

    if let Some(code) = &state.code {
        let result_label = Element::new(ui_v3::ElementType::P, Some("解锁码"))
            .size(13)
            .text_color("#888888")
            .margin_top(16)
            .margin_bottom(6);

        let code_text = Element::new(ui_v3::ElementType::P, Some(code.as_str()))
            .size(30)
            .text_color("#ff6b00");

        children.push(result_label);
        children.push(code_text);
    }

    if let Some(error) = &state.error {
        let error_text = Element::new(ui_v3::ElementType::P, Some(error.as_str()))
            .size(13)
            .text_color("#ff4444")
            .margin_top(10);
        children.push(error_text);
    }

    let mut content = Element::new(ui_v3::ElementType::Div, None)
        .flex()
        .flex_direction(ui_v3::FlexDirection::Column)
        .width_full();

    for child in children {
        content = content.child(child);
    }

    Element::new(ui_v3::ElementType::Card, None)
        .width_full()
        .padding(16)
        .bg("#181818")
        .border(1, "#2a2a2a")
        .radius(12)
        .child(content)
}

pub fn render_main_ui(element_id: &str) {
    let state = ui_state().lock().unwrap_or_else(|p| p.into_inner());
    ui_v3::render(element_id, build_ui(&state));
}
