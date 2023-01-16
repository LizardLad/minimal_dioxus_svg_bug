use dioxus::prelude::*;
use dioxus_elements::input_data::MouseButton as DioxusMouseButton;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(app);
}

struct Point {
	x: i32,
	y: i32,
}

struct Circle<'a> {
	x: i32,
	y: i32,
	r: u16,
	key: &'a str,
}

#[derive(Clone, Copy, PartialEq)]
enum MouseButton {
	None,
	Left,
	Right,
	Middle,
}

#[derive(Clone)]
struct Click {
	x: i32,
	y: i32,
	button: MouseButton,
	current: bool,
}

fn app(cx: Scope) -> Element {
    let window_offset = use_state(cx, || Point{x: 0, y: 0});
	let click_position = use_state(cx, || Click{x: 0, y: 0, button: MouseButton::None, current: false});

	let middle_mouse_down_handler = move |evt: Event<MouseData>| {
		let position = evt.coordinates().element().to_i32();
		click_position.modify(|click| {let new_click = Click {x: position.x, y: position.y, button: MouseButton::Middle, current: true, ..*click}; new_click})
	};

	let mouse_move_handler = move |evt: Event<MouseData>| {
		if !click_position.current || click_position.button != MouseButton::Middle {
			return;
		}
		let position = evt.coordinates().element().to_i32();
		if position.x != click_position.x && position.y != click_position.y {
			let x = position.x - click_position.x;
			let y = position.y - click_position.y;
			window_offset.set(Point {x, y});
			click_position.set(Click {x, y,  ..(*(click_position.current()))});
		}
	};

	let middle_mouse_up_handler = move |_evt: Event<MouseData>| {
		if click_position.current {
			click_position.set(Click {current: false, ..(*(click_position.current()))});
		}
	};

	let mouse_down_handler = move |evt: Event<MouseData>| {if evt.data.trigger_button() == Some(DioxusMouseButton::Auxiliary) {middle_mouse_down_handler(evt);}};
	let mouse_up_handler = move |evt: Event<MouseData>| {if evt.data.trigger_button() == Some(DioxusMouseButton::Auxiliary) {middle_mouse_up_handler(evt);}};

	let circles = [Circle {x: 50, y: 50, r: 50, key: "cirle_1"}, Circle {x: 150, y: 150, r: 50, key: "cirle_2"}, Circle {x: 150, y: 50, r: 50, key: "cirle_3"}];
    let circle_nodes = circles.iter().map(|circle_data: &Circle| {
		let cx = (circle_data.x - window_offset.x).to_string();
		let cy = (circle_data.y - window_offset.y).to_string();
		let r = circle_data.r.to_string();
		let key = circle_data.key;
		rsx!(
			circle {
				key: "{key}",
				"cx": "{cx}",
				"cy": "{cy}",
				"r": "{r}",
				"fill": "red",
				prevent_default: "oncontextmenu",
				oncontextmenu: move |evt| {evt.stop_propagation();},
			}
		)
	});

	cx.render(rsx!{
		svg {
			style: "height: 100vw; width: 100vw;",
			prevent_default: "oncontextmenu",
			onmousemove: move |evt| {evt.stop_propagation(); mouse_move_handler(evt.clone()); log::debug!("Mouse Move: {evt:?}");},
			onmousedown: move |evt| {evt.stop_propagation(); mouse_down_handler(evt.clone()); log::debug!("Mouse Down: {evt:?}");},
			onmouseup: move |evt| {evt.stop_propagation(); mouse_up_handler(evt.clone()); log::debug!("Mouse Up: {evt:?}");},
			oncontextmenu: move |evt| {evt.stop_propagation();},
			circle_nodes
		}
	})
}
