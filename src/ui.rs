use anyhow::{Context, Error, Result};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const LOGICAL_WIDTH: u32 = 800;
const LOGICAL_HEIGHT: u32 = 600;

pub fn run() -> Result<()> {
    log::info!("Starting UI");

    let event_loop = EventLoop::new();

    let logical_size = LogicalSize::new(LOGICAL_WIDTH, LOGICAL_HEIGHT);
    let window = WindowBuilder::new()
        .with_title("NES Emulator")
        .with_inner_size(logical_size)
        .with_min_inner_size(logical_size)
        .build(&event_loop)?;

    let PhysicalSize { width, height } = window.inner_size();
    let surface_texture = SurfaceTexture::new(width, height, &window);
    let mut pixels = Pixels::new(LOGICAL_WIDTH, LOGICAL_HEIGHT, surface_texture)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                log::info!("Closing window");
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => {
                pixels.render().unwrap();
            }
            Event::RedrawRequested(_) => {
                pixels.render().unwrap();
            }
            _ => {}
        }
    });

    Ok(())
}
