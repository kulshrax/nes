use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

use crate::ppu::{FRAME_HEIGHT, FRAME_WIDTH};

pub struct Ui<'p, 'i> {
    pub frame: &'p mut Pixels<Window>,
    pub input: &'i WinitInputHelper,
}

impl<'p, 'i> Ui<'p, 'i> {
    fn new(frame: &'p mut Pixels<Window>, input: &'i WinitInputHelper) -> Self {
        Self { frame, input }
    }
}

pub fn run<F>(mut callback: F) -> Result<()>
where
    F: FnMut(Ui) -> Result<()> + 'static,
{
    log::info!("Starting UI");

    let event_loop = EventLoop::new();

    let logical_size = LogicalSize::new(FRAME_WIDTH as f64, FRAME_HEIGHT as f64);
    let window = WindowBuilder::new()
        .with_title("NES Emulator")
        .with_inner_size(logical_size)
        .with_min_inner_size(logical_size)
        .build(&event_loop)?;

    let PhysicalSize { width, height } = window.inner_size();
    let surface_texture = SurfaceTexture::new(width, height, &window);
    let mut pixels = Pixels::new(FRAME_WIDTH as u32, FRAME_HEIGHT as u32, surface_texture)?;

    let mut input = WinitInputHelper::new();

    event_loop.run(move |event, _, control_flow| {
        log::trace!("Event: {:?}", &event);

        *control_flow = ControlFlow::Wait;

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

        log::trace!("Calling user callback");
        if let Err(e) = callback(Ui::new(&mut pixels, &input)) {
            log::error!("Exiting due to emulation error: {}", e);
            *control_flow = ControlFlow::Exit;
            return;
        }

        //window.request_redraw();
    });
}
