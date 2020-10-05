pub use winit::window::Fullscreen;
pub use winit::window::WindowId as Id;

use std::default::Default;

use crate::gfx::wgpu;
use crate::gfx::{frame::Frame};

mod proxy;
pub use proxy::Proxy;

pub type LogicalSize = winit::dpi::LogicalSize<f32>;
pub type PhysicalSize = winit::dpi::PhysicalSize<u32>;

pub const DEFAULT_DIMENSIONS: LogicalSize = LogicalSize {
    width: 1024.0,
    height: 768.0,
};

pub struct WindowBuilder {
    window: winit::window::WindowBuilder,
    title_was_set: bool,
    swap_chain_builder: SwapChainBuilder,
    power_preference: wgpu::PowerPreference,
    device_desc: Option<wgpu::DeviceDescriptor>,
    // user_functions: UserFunctions,
    msaa_samples: Option<u32>,
}

impl WindowBuilder {
    /// Begin building a new window.
    pub fn new() -> Self {
        WindowBuilder {
            window: winit::window::WindowBuilder::new(),
            title_was_set: false,
            swap_chain_builder: Default::default(),
            power_preference: wgpu::defaults::power_preference(),
            device_desc: None,
            //user_functions: Default::default(),
            msaa_samples: None,
        }
    }

    /// Build the window with some custom window parameters.
    pub fn window(mut self, window: winit::window::WindowBuilder) -> Self {
        self.window = window;
        self
    }

    /// Specify a set of parameters for building the window surface swap chain.
    pub fn swap_chain_builder(mut self, swap_chain_builder: SwapChainBuilder) -> Self {
        self.swap_chain_builder = swap_chain_builder;
        self
    }

    /// Specify the power preference desired for the WGPU adapter.
    ///
    /// By default, this is `wgpu::PowerPreference::HighPerformance`.
    pub fn power_preference(mut self, pref: wgpu::PowerPreference) -> Self {
        self.power_preference = pref;
        self
    }

    /// Specify a device descriptor to use when requesting the logical device from the adapter.
    /// This allows for specifying custom wgpu device extensions.
    pub fn device_descriptor(mut self, device_desc: wgpu::DeviceDescriptor) -> Self {
        self.device_desc = Some(device_desc);
        self
    }

    /// Specify the number of samples per pixel for the multisample anti-aliasing render pass.
    ///
    /// If `msaa_samples` is unspecified, the first default value that nannou will attempt to use
    /// can be found via the `Frame::DEFAULT_MSAA_SAMPLES` constant.
    ///
    /// **Note:** This parameter has no meaning if the window uses a **raw_view** function for
    /// rendering graphics to the window rather than a **view** function. This is because the
    /// **raw_view** function provides a **RawFrame** with direct access to the swap chain image
    /// itself and thus must manage their own MSAA pass.
    ///
    /// On the other hand, the `view` function provides the `Frame` type which allows the user to
    /// render to a multisampled intermediary image allowing Nannou to take care of resolving the
    /// multisampled image to the swap chain image. In order to avoid confusion, The `Window::build`
    /// method will `panic!` if the user tries to specify `msaa_samples` as well as a `raw_view`
    /// method.
    ///
    /// *TODO: Perhaps it would be worth adding two separate methods for specifying msaa samples.
    /// One for forcing a certain number of samples and returning an error otherwise, and another
    /// for attempting to use the given number of samples but falling back to a supported value in
    /// the case that the specified number is not supported.*
    pub fn msaa_samples(mut self, msaa_samples: u32) -> Self {
        self.msaa_samples = Some(msaa_samples);
        self
    }

    // /// A function for processing key press events associated with this window.
    // pub fn key_pressed<M>(mut self, f: KeyPressedFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.key_pressed = Some(KeyPressedFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing key release events associated with this window.
    // pub fn key_released<M>(mut self, f: KeyReleasedFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.key_released = Some(KeyReleasedFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing mouse moved events associated with this window.
    // pub fn mouse_moved<M>(mut self, f: MouseMovedFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.mouse_moved = Some(MouseMovedFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing mouse pressed events associated with this window.
    // pub fn mouse_pressed<M>(mut self, f: MousePressedFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.mouse_pressed = Some(MousePressedFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing mouse released events associated with this window.
    // pub fn mouse_released<M>(mut self, f: MouseReleasedFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.mouse_released = Some(MouseReleasedFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing mouse wheel events associated with this window.
    // pub fn mouse_wheel<M>(mut self, f: MouseWheelFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.mouse_wheel = Some(MouseWheelFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing mouse entered events associated with this window.
    // pub fn mouse_entered<M>(mut self, f: MouseEnteredFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.mouse_entered = Some(MouseEnteredFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing mouse exited events associated with this window.
    // pub fn mouse_exited<M>(mut self, f: MouseExitedFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.mouse_exited = Some(MouseExitedFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing touch events associated with this window.
    // pub fn touch<M>(mut self, f: TouchFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.touch = Some(TouchFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing touchpad pressure events associated with this window.
    // pub fn touchpad_pressure<M>(mut self, f: TouchpadPressureFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.touchpad_pressure = Some(TouchpadPressureFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing window moved events associated with this window.
    // pub fn moved<M>(mut self, f: MovedFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.moved = Some(MovedFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing window resized events associated with this window.
    // pub fn resized<M>(mut self, f: ResizedFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.resized = Some(ResizedFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing hovered file events associated with this window.
    // pub fn hovered_file<M>(mut self, f: HoveredFileFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.hovered_file = Some(HoveredFileFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing hovered file cancelled events associated with this window.
    // pub fn hovered_file_cancelled<M>(mut self, f: HoveredFileCancelledFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.hovered_file_cancelled =
    //         Some(HoveredFileCancelledFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing dropped file events associated with this window.
    // pub fn dropped_file<M>(mut self, f: DroppedFileFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.dropped_file = Some(DroppedFileFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing the focused event associated with this window.
    // pub fn focused<M>(mut self, f: FocusedFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.focused = Some(FocusedFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing the unfocused event associated with this window.
    // pub fn unfocused<M>(mut self, f: UnfocusedFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.unfocused = Some(UnfocusedFnAny::from_fn_ptr(f));
    //     self
    // }

    // /// A function for processing the window closed event associated with this window.
    // pub fn closed<M>(mut self, f: ClosedFn<M>) -> Self
    // where
    //     M: 'static,
    // {
    //     self.user_functions.closed = Some(ClosedFnAny::from_fn_ptr(f));
    //     self
    // }

    /// Builds the window, inserts it into the `App`'s display map and returns the unique ID.
    pub async fn build(self, event_loop: &winit::event_loop::EventLoop<()>, instance: &wgpu::Instance) -> (Window, wgpu::Adapter, wgpu::Device, wgpu::Queue) {
        let WindowBuilder {
            mut window,
            title_was_set,
            swap_chain_builder,
            power_preference,
            device_desc,
            //user_functions,
            msaa_samples,
        } = self;

        // If the title was not set, default to the "PHANTOMa - <exe_name>".
        if !title_was_set {
            let exe = std::env::current_exe()
                .unwrap()
                .file_stem()
                .expect("exe path contained no file stem")
                .to_string_lossy()
                .to_string();

            let title = format!("PHANTOMa - {}", exe);
            window = window.with_title(title);
        }

        // Set default dimensions in the case that none were given.
        let initial_window_size = window
            .window
            .inner_size
            .or_else(|| {
                window
                    .window
                    .fullscreen
                    .as_ref()
                    .map(|fullscreen| match fullscreen {
                        Fullscreen::Exclusive(video_mode) => {
                            let monitor = video_mode.monitor();
                            video_mode
                                .size()
                                .to_logical::<f32>(monitor.scale_factor())
                                .into()
                        }
                        Fullscreen::Borderless(monitor) => monitor.as_ref().unwrap()
                            .size()
                            .to_logical::<f32>(monitor.as_ref().unwrap().scale_factor())
                            .into(),
                    })
            })
            .unwrap_or_else(|| {
                let mut dim = DEFAULT_DIMENSIONS;
                if let Some(min) = window.window.min_inner_size {
                    match min {
                        winit::dpi::Size::Logical(min) => {
                            dim.width = dim.width.max(min.width as _);
                            dim.height = dim.height.max(min.height as _);
                        }
                        winit::dpi::Size::Physical(min) => {
                            dim.width = dim.width.max(min.width as _);
                            dim.height = dim.height.max(min.height as _);
                            unimplemented!("consider scale factor");
                        }
                    }
                }
                if let Some(max) = window.window.max_inner_size {
                    match max {
                        winit::dpi::Size::Logical(max) => {
                            dim.width = dim.width.min(max.width as _);
                            dim.height = dim.height.min(max.height as _);
                        }
                        winit::dpi::Size::Physical(max) => {
                            dim.width = dim.width.min(max.width as _);
                            dim.height = dim.height.min(max.height as _);
                            unimplemented!("consider scale factor");
                        }
                    }
                }
                dim.into()
            });

        // Use the `initial_swapchain_dimensions` as the default dimensions for the window if none
        // were specified.
        if window.window.inner_size.is_none() && window.window.fullscreen.is_none() {
            window.window.inner_size = Some(initial_window_size);
        }

        // Build the window.
        let window = window.build(event_loop).unwrap();
        let size = window.inner_size();
        let scale_factor = window.scale_factor();

        // Build the wgpu surface.
        let surface = unsafe { instance.create_surface(&window) };

        // Request the adapter.
        let request_adapter_opts = wgpu::RequestAdapterOptions {
            power_preference,
            compatible_surface: Some(&surface),
        };
        let adapter = instance
            .request_adapter(&request_adapter_opts)
            .await
            .unwrap();

        // Instantiate the logical device.
        let device_desc = device_desc.unwrap_or_else(|| crate::gfx::wgpu::defaults::device_descriptor());
        let (device, queue) = adapter.request_device(&device_desc, None).await.unwrap();

        // Build the swapchain.
        let swap_chain = swap_chain_builder.build(&device, &surface, size);

        let window = Window {
            id: window.id(),
            window,
            surface,
            swap_chain,
            // frame_data,
            // frame_count,
            // user_functions,

            // TODO: default to 4 or something
            msaa_samples: msaa_samples.unwrap_or(1),
            size,
            scale_factor,
        };

        (window, adapter, device, queue)
    }

    fn map_window<F>(mut self, map: F) -> Self
    where
        F: FnOnce(winit::window::WindowBuilder) -> winit::window::WindowBuilder,
    {
        self.window = map(self.window);
        self
    }

    // Window builder methods.
    //
    // NOTE: On new versions of winit, we should check whether or not new `WindowBuilder` methods
    // have been added that we should expose.

    /// Requests the window to be a specific size in points.
    ///
    /// This describes to the "inner" part of the window, not including desktop decorations like the
    /// title bar.
    pub fn size(self, width: u32, height: u32) -> Self {
        self.map_window(|w| w.with_inner_size(winit::dpi::LogicalSize { width, height }))
    }

    /// Set the minimum size in points for the window.
    pub fn min_size(self, width: u32, height: u32) -> Self {
        self.map_window(|w| w.with_min_inner_size(winit::dpi::LogicalSize { width, height }))
    }

    /// Set the maximum size in points for the window.
    pub fn max_size(self, width: u32, height: u32) -> Self {
        self.map_window(|w| w.with_max_inner_size(winit::dpi::LogicalSize { width, height }))
    }

    /// Requests the window to be a specific size in points.
    ///
    /// This describes to the "inner" part of the window, not including desktop decorations like the
    /// title bar.
    pub fn size_pixels(self, width: u32, height: u32) -> Self {
        self.map_window(|w| w.with_inner_size(winit::dpi::PhysicalSize { width, height }))
    }

    /// Whether or not the window should be resizable after creation.
    pub fn resizable(self, resizable: bool) -> Self {
        self.map_window(|w| w.with_resizable(resizable))
    }

    /// Requests a specific title for the window.
    pub fn title<T>(mut self, title: T) -> Self
    where
        T: Into<String>,
    {
        self.title_was_set = true;
        self.map_window(|w| w.with_title(title))
    }

    /// Create the window fullscreened on the current monitor.
    // pub fn fullscreen(self) -> Self {
    //     let fullscreen = Fullscreen::Borderless(self.app.primary_monitor());
    //     self.fullscreen_with(Some(fullscreen))
    // }

    /// Set the window fullscreen state with the given settings.
    ///
    /// - `None` indicates a normal window. This is the default case.
    /// - `Some(Fullscreen)` means fullscreen with the desired settings.
    pub fn fullscreen_with(self, fullscreen: Option<Fullscreen>) -> Self {
        self.map_window(|w| w.with_fullscreen(fullscreen))
    }

    /// Requests maximized mode.
    pub fn maximized(self, maximized: bool) -> Self {
        self.map_window(|w| w.with_maximized(maximized))
    }

    /// Sets whether the window will be initially hidden or visible.
    pub fn visible(self, visible: bool) -> Self {
        self.map_window(|w| w.with_visible(visible))
    }

    /// Sets whether the background of the window should be transparent.
    pub fn transparent(self, transparent: bool) -> Self {
        self.map_window(|w| w.with_transparent(transparent))
    }

    /// Sets whether the window should have a border, a title bar, etc.
    pub fn decorations(self, decorations: bool) -> Self {
        self.map_window(|w| w.with_decorations(decorations))
    }

    /// Sets whether or not the window will always be on top of other windows.
    pub fn always_on_top(self, always_on_top: bool) -> Self {
        self.map_window(|w| w.with_always_on_top(always_on_top))
    }

    /// Sets the window icon.
    pub fn window_icon(self, window_icon: Option<winit::window::Icon>) -> Self {
        self.map_window(|w| w.with_window_icon(window_icon))
    }
}

// #[derive(Debug)]
pub struct Window {
    pub id: Id,
    pub(crate) window: winit::window::Window,

    pub(crate) surface: wgpu::Surface,
    pub(crate) swap_chain: SwapChain,

    // TODO: not pub
    pub msaa_samples: u32,
    // pub(crate) user_functions: UserFunctions,

    pub size: PhysicalSize,
    pub scale_factor: f64,
}

impl Window {
    pub(crate) fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub(crate) fn rebuild_swap_chain(&mut self, device: &wgpu::Device, size: PhysicalSize) {
        self.swap_chain.rebuild(&device, &self.surface, size);
    }
}

/// A swap_chain and its images associated with a single window.
pub(crate) struct SwapChain {
    // The descriptor used to create the original swap chain. Useful for recreation.
    pub(crate) descriptor: wgpu::SwapChainDescriptor,
    // TODO: during frame request, not redraw
    // This is an `Option` in order to allow for separating ownership of the swapchain from the
    // window during a `RedrawRequest`. Other than during `RedrawRequest`, this should always be
    // `Some`.
    pub(crate) swap_chain: Option<wgpu::SwapChain>,
}

impl SwapChain {
    pub(crate) fn build(device: &wgpu::Device, surface: &wgpu::Surface, descriptor: wgpu::SwapChainDescriptor) -> Self {
        let swap_chain = device.create_swap_chain(surface, &descriptor);
        Self {
            swap_chain: Some(swap_chain),
            descriptor,
        }
    }

    pub(crate) fn rebuild(&mut self, device: &wgpu::Device, surface: &wgpu::Surface, size: PhysicalSize) {
        std::mem::drop(self.swap_chain.take());

        self.descriptor.width = size.width;
        self.descriptor.height = size.height;

        self.swap_chain = Some(device.create_swap_chain(surface, &self.descriptor));
    }

    pub fn next_frame<'a>(&mut self, device: &'a wgpu::Device, staging: &'a mut wgpu::util::StagingBelt) -> Option<Frame<'a>> {
        let swap_chain = self.swap_chain.as_mut().unwrap();

        if let Ok(frame) = swap_chain.get_current_frame() {
            let view = wgpu::SwapChainTextureView::new(self, frame);

            Some(Frame::new(device, staging, view))
        } else {
            None
        }
    }
}


/// SwapChain building parameters for which Nannou will provide a default if unspecified.
///
/// See the builder methods for more details on each parameter.
#[derive(Clone, Debug, Default)]
pub struct SwapChainBuilder {
    pub usage: Option<wgpu::TextureUsage>,
    pub format: Option<wgpu::TextureFormat>,
    pub present_mode: Option<wgpu::PresentMode>,
}

impl SwapChainBuilder {
    pub const DEFAULT_USAGE: wgpu::TextureUsage = wgpu::TextureUsage::OUTPUT_ATTACHMENT;
    pub const DEFAULT_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;
    pub const DEFAULT_PRESENT_MODE: wgpu::PresentMode = wgpu::PresentMode::Fifo;

    /// A new empty **SwapChainBuilder** with all parameters set to `None`.
    pub fn new() -> Self {
        Default::default()
    }

    /// Specify the texture format for the swap chain.
    pub fn format(mut self, format: wgpu::TextureFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// The way in which swap chain images are presented to the display.
    ///
    /// By default, nannou will attempt to select the ideal present mode depending on the current
    /// app `LoopMode`.
    pub fn present_mode(mut self, present_mode: wgpu::PresentMode) -> Self {
        self.present_mode = Some(present_mode);
        self
    }

    /// Build the swap chain.
    pub(crate) fn build(
        self,
        device: &wgpu::Device,
        surface: &wgpu::Surface,
        size: PhysicalSize,
    ) -> SwapChain {
        let usage = self.usage.unwrap_or(Self::DEFAULT_USAGE);
        let format = self.format.unwrap_or(Self::DEFAULT_FORMAT);
        let present_mode = self.present_mode.unwrap_or(Self::DEFAULT_PRESENT_MODE);

        let desc = wgpu::SwapChainDescriptor {
            usage,
            format,
            width: size.width,
            height: size.height,
            present_mode,
        };

        SwapChain::build(device, surface, desc)
    }
}