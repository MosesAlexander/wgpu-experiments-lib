use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window}, dpi::PhysicalSize,
};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch="wasm32")]
use log::info;

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch="wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Info).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450,400));

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-example")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            }).expect("Couldn't append canvas to document body");
    }

    #[cfg(target_arch = "wasm32")]
    info!("BANANA POWER MOTHERFUCKER");

    println!("BANANA POWER!!!");
    event_loop.run(
        //Start of closure
        move |event, _, control_flow|
        // Closure has one match statement
        match event {
            // First outer match case
            Event::WindowEvent {ref event, window_id} if window_id == window.id() =>
                // Inner match statement
                match event {
                    // Sometimes I wish match expressions were fall-through
                    WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                                    input: KeyboardInput { // This KeyboardInput struct is different from the one above
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                        ..
                    } => {
                        *control_flow = ControlFlow::Exit;
                        #[cfg(target_arch = "wasm32")]
                        info!("PRESSED ESC OH MY GOD I SAW THAT");
                    }
                    _ => {}
                },
            // Second outer match case
            _ => {},
        }
    );
}