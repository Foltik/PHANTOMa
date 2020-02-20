#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;

mod component;
mod error;

#[allow(unused_imports)]
use rendy::{
    command::Families,
    factory::{Config, Factory},
    graph::{present::PresentNode, render::*, Graph, GraphBuilder, ImageId},
    hal::{
        self,
        command::{ClearColor, ClearValue},
        Backend,
    },
    init::winit::{
        dpi::PhysicalSize,
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        monitor::{MonitorHandle, VideoMode},
        window::{Fullscreen, Window, WindowBuilder},
    },
    init::AnyWindowedRendy,
    wsi::Surface,
};

use std::io::{stdin, stdout, Write};

use component::{cube, filter, ComponentState};

#[allow(dead_code)]
fn prompt_for_monitor(event_loop: &EventLoop<()>) -> MonitorHandle {
    for (num, monitor) in event_loop.available_monitors().enumerate() {
        println!("Monitor #{}: {:?}", num, monitor.name());
    }

    print!("Please write the number of the monitor to use: ");
    stdout().flush().unwrap();

    let mut num = String::new();
    stdin().read_line(&mut num).unwrap();
    let num = num.trim().parse().ok().expect("Please enter a number");
    let monitor = event_loop
        .available_monitors()
        .nth(num)
        .expect("Please enter a valid ID");

    println!("Using {:?}", monitor.name());

    monitor
}

#[allow(dead_code)]
fn prompt_for_video_mode(monitor: &MonitorHandle) -> VideoMode {
    for (i, video_mode) in monitor.video_modes().enumerate() {
        println!("Video mode #{}: {}", i, video_mode);
    }

    print!("Please write the number of the video mode to use: ");
    stdout().flush().unwrap();

    let mut num = String::new();
    stdin().read_line(&mut num).unwrap();
    let num = num.trim().parse().ok().expect("Please enter a number");
    let video_mode = monitor
        .video_modes()
        .nth(num)
        .expect("Please enter a valid ID");

    println!("Using {}", video_mode);

    video_mode
}

fn create_image<B: Backend>(
    factory: &Factory<B>,
    builder: &mut GraphBuilder<B, ComponentState>,
    surface: &Surface<B>,
    size: &PhysicalSize<u32>,
    clear: Option<ClearValue>,
) -> ImageId {
    builder.create_image(
        hal::image::Kind::D2(size.width, size.height, 1, 1),
        1,
        factory.get_surface_format(surface),
        clear,
    )
}

fn build_graph<B: Backend>(
    factory: &mut Factory<B>,
    families: &mut Families<B>,
    window: &Window,
    state: &ComponentState,
) -> Graph<B, ComponentState> {
    let mut graph_builder = GraphBuilder::new();

    let surface = factory.create_surface(window).unwrap();
    let size = window.inner_size();

    let white = ClearValue {
        color: ClearColor {
            float32: [1.0, 1.0, 1.0, 1.0],
        },
    };

    let color = create_image(factory, &mut graph_builder, &surface, &size, None);
    let mesh = create_image(factory, &mut graph_builder, &surface, &size, Some(white));

    log::debug!("Creating surface with size {}x{}", size.width, size.height);

    let cube = graph_builder.add_node(
        cube::TriangleDesc::default()
            .builder()
            .into_subpass()
            .with_color(mesh)
            .into_pass(),
    );

    let post = graph_builder.add_node(
        filter::FilterDesc::default()
            .builder()
            .with_image(mesh)
            .with_dependency(cube)
            .into_subpass()
            .with_color(color)
            .into_pass(),
    );

    graph_builder.add_node(PresentNode::builder(&factory, surface, color).with_dependency(post));

    graph_builder.build(factory, families, state).unwrap()
}

fn run<B: Backend>(
    event_loop: EventLoop<()>,
    mut factory: Factory<B>,
    mut families: Families<B>,
    window: Window,
) {
    let started = std::time::Instant::now();

    let size = window.inner_size();
    let mut state = ComponentState {
        frame: 0,
        t: 0.0,
        w: size.width,
        h: size.height,
        aspect: size.width as f32 / size.height as f32,
    };

    let mut graph = Some(build_graph(&mut factory, &mut families, &window, &state));

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::Resized(new) => {
                    graph.take().unwrap().dispose(&mut factory, &state);

                    state.w = new.width;
                    state.h = new.height;
                    state.aspect = state.w as f32 / state.h as f32;

                    graph = Some(build_graph(&mut factory, &mut families, &window, &state));
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                state.frame += 1;

                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                factory.maintain(&mut families);
                if let Some(ref mut graph) = graph {
                    graph.run(&mut factory, &mut families, &state);
                }
            }
            _ => {}
        }

        if *control_flow == ControlFlow::Exit {
            let elapsed = started.elapsed();
            let elapsed_ns = elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64;

            graph.take().unwrap().dispose(&mut factory, &state);

            log::info!(
                "Elapsed: {:?}. Frames: {}. FPS: {}",
                elapsed,
                state.frame,
                state.frame as u64 * 1_000_000_000 / elapsed_ns
            );
        }
    });
}

fn main() {
    env_logger::Builder::from_default_env()
        .filter_module("phantoma", log::LevelFilter::Trace)
        .init();

    let config: Config = Default::default();
    let event_loop = EventLoop::new();

    /*
    let mon = prompt_for_monitor(&event_loop);
    let mode = prompt_for_video_mode(&mon);
    */

    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize {
            width: 960,
            height: 640,
        })
        //.with_fullscreen(Some(Fullscreen::Exclusive(mode)))
        .with_title("PHANTOMa");

    let rendy = AnyWindowedRendy::init_auto(&config, window, &event_loop).unwrap();
    rendy::with_any_windowed_rendy!((rendy)
        (factory, families, _surface, window) => {
            run(event_loop, factory, families, window);
        }
    );
}
