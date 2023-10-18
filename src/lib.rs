use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window}, dpi::PhysicalSize,
};

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch="wasm32")]
use log::info;

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    // The window must be declared after the surface so
    // it gets droppe dafter it as the surface contains
    // unsafe references to the window's resources
    window: Window,
}

impl State {
    async fn new(window: Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // # Safety
        // The surface needs to live as long as the window that created it
        // State owns the window so this should be safe
        let surface = unsafe {instance.create_surface(&window)}.unwrap();

        /*
        // NOTE: NAVAIL on WASM
        let _ = instance.enumerate_adapters(wgpu::Backends::all()).find(|adapter|{
            println!("{:?}", adapter);
            adapter.is_surface_supported(&surface)
        }).unwrap(); */

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None).await.unwrap();

            let surface_caps = surface.get_capabilities(&adapter);

            let surface_format = surface_caps.formats.iter()
                .copied()
                .find(|f| f.is_srgb())
                .unwrap_or(surface_caps.formats[0]);

            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface_format,
                width: size.width,
                height: size.height,
                present_mode: surface_caps.present_modes[0],
                alpha_mode: surface_caps.alpha_modes[0],
                view_formats: vec![],
            };

            surface.configure(&device, &config);

            Self {
                window,
                surface,
                device,
                queue,
                config,
                size
            }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        todo!()
    }
}

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub async fn run() {
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

    let mut state = State::new(window).await;
    event_loop.run(
        //Start of closure
        move |event, _, control_flow|
        // Closure has one match statement
        match event {
            // First outer match case
            Event::WindowEvent {ref event, window_id} if window_id == state.window().id() =>
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
                    },
                    WindowEvent::Resized(physical_size) => {
                        println!("physical_size: {:?}", physical_size);
                        #[cfg(target_arch = "wasm32")]
                        info!("physical_size: {:?}", physical_size);
                        state.resize(*physical_size);
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, ..} => {
                        state.resize(**new_inner_size);
                    },
                    _ => {}
                },
            // Second outer match case
            _ => {},
        }
    );
}