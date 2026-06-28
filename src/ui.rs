use astrobox_ng_wit::astrobox::psys_host::{self, ui};
use std::sync::{Mutex, OnceLock};

pub const EXAMPLE_BUTTON_CLICK_EVENT: &str = "example_button_click";

struct UiState {
    click_count: usize,
    root_element_id: Option<String>,
}

static UI_STATE: OnceLock<Mutex<UiState>> = OnceLock::new();

fn ui_state() -> &'static Mutex<UiState> {
    UI_STATE.get_or_init(|| {
        Mutex::new(UiState {
            click_count: 0,
            root_element_id: None,
        })
    })
}

pub fn ui_event_processor(evtype: ui::Event, event: &str) {
    match evtype {
        ui::Event::Click => match event {
            EXAMPLE_BUTTON_CLICK_EVENT => {
                let (root_element_id, click_count) = {
                    let mut state = ui_state()
                        .lock()
                        .unwrap_or_else(|poisoned| poisoned.into_inner());
                    state.click_count = state.click_count.saturating_add(1);
                    (state.root_element_id.clone(), state.click_count)
                };

                if let Some(root_element_id) = root_element_id {
                    psys_host::ui::render(&root_element_id, build_main_ui(click_count));
                }
            }
            _ => {}
        },
        _ => {}
    }
}

pub fn build_main_ui(click_count: usize) -> ui::Element {
    let example_button = ui::Element::new(ui::ElementType::Button, Some("点我试试？"))
        .bg("#00FF00")
        .on(ui::Event::Click, EXAMPLE_BUTTON_CLICK_EVENT);

    let example_img = ui::Element::new(
        ui::ElementType::Image,
        Some(crate::resources::UI_EXAMPLE_IMAGE_B64),
    )
    .width(200)
    .height(200);

    let example_text_content = if click_count == 0 {
        "何意味。这只是一个示例插件。".to_string()
    } else if click_count < 101 {
        format!("已被点击{}次", click_count)
    } else {
        format!("干嘛...还点... ({})", click_count)
    };
    let example_text =
        ui::Element::new(ui::ElementType::P, Some(example_text_content.as_str())).size(26);

    let example_select = ui::Element::new(ui::ElementType::Select, None)
        .child(ui::Element::new(ui::ElementType::Option, Some("电棍")))
        .child(ui::Element::new(ui::ElementType::Option, Some("炫狗")))
        .child(ui::Element::new(ui::ElementType::Option, Some("叮咚鸡")));

    ui::Element::new(ui::ElementType::Div, None)
        .flex()
        .flex_direction(ui::FlexDirection::Column)
        .width_full()
        .justify_center()
        .align_center()
        .child(example_img)
        .child(example_text)
        .child(example_button)
        .child(example_select)
}

pub fn render_main_ui(element_id: &str) {
    let click_count = {
        let mut state = ui_state()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        state.root_element_id = Some(element_id.to_string());
        state.click_count
    };

    psys_host::ui::render(element_id, build_main_ui(click_count));
}
