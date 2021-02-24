use std::time::{Duration, Instant};

use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

pub trait Ui {
    fn size(&self) -> (u32, u32);

    fn update(&mut self, frame: &mut [u8], input: &WinitInputHelper, dt: Duration) -> Result<()>;
}

pub trait Run {
    fn run(self) -> Result<()>;
}

impl<T: Ui + 'static> Run for T {
    fn run(mut self) -> Result<()> {
        log::info!("Starting UI");

        let event_loop = EventLoop::new();

        let (width, height) = self.size();
        let logical_size = LogicalSize::new(width, height);
        let window = WindowBuilder::new()
            .with_title("NES Emulator")
            .with_inner_size(logical_size)
            .with_min_inner_size(logical_size)
            .build(&event_loop)?;

        let phys_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(phys_size.width, phys_size.height, &window);
        let mut pixels = Pixels::new(width as u32, height as u32, surface_texture)?;

        let mut input = WinitInputHelper::new();

        let mut time = Instant::now();

        event_loop.run(move |event, _, control_flow| {
            log::trace!("UI event: {:?}", &event);

            *control_flow = ControlFlow::Poll;

            if let Event::RedrawRequested(_) = event {
                if let Err(e) = pixels.render() {
                    log::error!("Exiting due to render error: {}", e);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            if !input.update(&event) {
                return;
            }

            if input.quit() {
                log::info!("Exiting due to user request");
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            };

            let now = Instant::now();
            let dt = now.duration_since(time);
            time = now;

            log::trace!("Updating frame after: {:?}", &dt);
            if let Err(e) = self.update(pixels.get_frame(), &input, dt) {
                log::error!("Exiting due to emulation error: {}", e);
                *control_flow = ControlFlow::Exit;
                return;
            }

            window.request_redraw();
        });
    }
}
