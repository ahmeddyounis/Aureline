//! WGPU surface backend that blits a CPU-rasterized buffer.
//!
//! This backend is a minimal bridge: higher layers can keep using the existing
//! placeholder rasterizer (0RGB buffer) while the window swaps through a real
//! GPU-backed surface.

use std::borrow::Cow;
use std::sync::Arc;

use winit::window::Window;

use crate::PixelRect;

/// GPU surface backend that presents a CPU-rasterized `0RGB` buffer.
#[derive(Debug)]
pub struct WgpuBlitRenderer {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    upload_bytes: Vec<u8>,
    upload_region_bytes: Vec<u8>,
}

impl WgpuBlitRenderer {
    /// Creates a new backend bound to the given window.
    ///
    /// # Errors
    ///
    /// Returns an error if the system cannot provide a compatible GPU adapter
    /// or device for the given surface.
    pub fn new(window: Arc<Window>) -> Result<Self, Box<dyn std::error::Error>> {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone())?;
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))?;

        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
                label: Some("aureline-render.device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                ..Default::default()
            }))?;

        let (surface_config, texture, texture_view, sampler, upload_bytes) =
            create_surface_resources(&window, &surface, &adapter, &device)?;

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("aureline-render.blit.bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("aureline-render.blit.bind_group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let pipeline = create_pipeline(&device, &bind_group_layout, surface_config.format);

        Ok(Self {
            window,
            surface,
            adapter,
            device,
            queue,
            surface_config,
            texture,
            texture_view,
            sampler,
            bind_group_layout,
            bind_group,
            pipeline,
            upload_bytes,
            upload_region_bytes: Vec::new(),
        })
    }

    /// Reconfigures the swapchain to match the current window size.
    pub fn resize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (config, texture, view, sampler, upload_bytes) =
            create_surface_resources(&self.window, &self.surface, &self.adapter, &self.device)?;
        self.surface_config = config;
        self.texture = texture;
        self.texture_view = view;
        self.sampler = sampler;
        self.upload_bytes = upload_bytes;
        self.bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("aureline-render.blit.bind_group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });
        Ok(())
    }

    /// Uploads a `0RGB` buffer and presents it via the GPU surface.
    ///
    /// The pixel format matches the shell's software rasterizer: each `u32` is
    /// `0x00RRGGBB`.
    ///
    /// # Errors
    ///
    /// Returns an error if the swapchain cannot provide a surface texture or if
    /// the caller supplies an undersized pixel buffer.
    pub fn render_0rgb(&mut self, pixels_0rgb: &[u32]) -> Result<(), Box<dyn std::error::Error>> {
        let width = self.surface_config.width;
        let height = self.surface_config.height;
        if width == 0 || height == 0 {
            return Ok(());
        }
        let required = width.saturating_mul(height) as usize;
        if pixels_0rgb.len() < required {
            return Err("pixel buffer smaller than surface dimensions".into());
        }

        let surface_texture = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(wgpu::SurfaceError::Timeout) => return Ok(()),
            Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                self.resize()?;
                match self.surface.get_current_texture() {
                    Ok(texture) => texture,
                    Err(wgpu::SurfaceError::Timeout) => return Ok(()),
                    Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => return Ok(()),
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        return Err("wgpu surface out of memory".into())
                    }
                    Err(wgpu::SurfaceError::Other) => return Err("wgpu surface error".into()),
                }
            }
            Err(wgpu::SurfaceError::OutOfMemory) => return Err("wgpu surface out of memory".into()),
            Err(wgpu::SurfaceError::Other) => return Err("wgpu surface error".into()),
        };

        self.write_pixels(width, height, &pixels_0rgb[..required]);
        self.blit_to_surface(surface_texture)?;
        Ok(())
    }

    /// Uploads dirty rectangles from a `0RGB` buffer and presents the existing texture.
    ///
    /// The pixel format matches the shell's software rasterizer: each `u32` is
    /// `0x00RRGGBB`.
    ///
    /// Passing an empty `dirty_rects` slice keeps the previously uploaded texture
    /// unchanged and simply re-presents it on the GPU surface.
    ///
    /// # Errors
    ///
    /// Returns an error if the swapchain cannot provide a surface texture or if
    /// the caller supplies an undersized pixel buffer.
    pub fn render_0rgb_dirty(
        &mut self,
        pixels_0rgb: &[u32],
        dirty_rects: &[PixelRect],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let width = self.surface_config.width;
        let height = self.surface_config.height;
        if width == 0 || height == 0 {
            return Ok(());
        }

        let required = width.saturating_mul(height) as usize;
        if pixels_0rgb.len() < required {
            return Err("pixel buffer smaller than surface dimensions".into());
        }

        let surface_texture = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(wgpu::SurfaceError::Timeout) => return Ok(()),
            Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                self.resize()?;
                match self.surface.get_current_texture() {
                    Ok(texture) => texture,
                    Err(wgpu::SurfaceError::Timeout) => return Ok(()),
                    Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => return Ok(()),
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        return Err("wgpu surface out of memory".into())
                    }
                    Err(wgpu::SurfaceError::Other) => return Err("wgpu surface error".into()),
                }
            }
            Err(wgpu::SurfaceError::OutOfMemory) => return Err("wgpu surface out of memory".into()),
            Err(wgpu::SurfaceError::Other) => return Err("wgpu surface error".into()),
        };

        let full_bounds = PixelRect::new(0, 0, width, height);
        if dirty_rects.len() == 1 && dirty_rects[0] == full_bounds {
            self.write_pixels(width, height, &pixels_0rgb[..required]);
        } else {
            for rect in dirty_rects {
                self.write_pixels_region(width, height, &pixels_0rgb[..required], *rect);
            }
        }

        self.blit_to_surface(surface_texture)?;
        Ok(())
    }

    fn write_pixels(&mut self, width: u32, height: u32, pixels_0rgb: &[u32]) {
        let required_bytes = (width as usize)
            .saturating_mul(height as usize)
            .saturating_mul(4);
        if self.upload_bytes.len() != required_bytes {
            self.upload_bytes.resize(required_bytes, 0);
        }
        for (idx, rgb) in pixels_0rgb.iter().enumerate() {
            let r = ((rgb >> 16) & 0xff) as u8;
            let g = ((rgb >> 8) & 0xff) as u8;
            let b = (rgb & 0xff) as u8;
            let out = idx.saturating_mul(4);
            if let Some(px) = self.upload_bytes.get_mut(out..out.saturating_add(4)) {
                px[0] = r;
                px[1] = g;
                px[2] = b;
                px[3] = 255;
            }
        }

        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &self.upload_bytes,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width.saturating_mul(4)),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
    }

    fn write_pixels_region(
        &mut self,
        width: u32,
        height: u32,
        pixels_0rgb: &[u32],
        rect: PixelRect,
    ) {
        if width == 0 || height == 0 || rect.is_empty() {
            return;
        }

        let window_bounds = PixelRect::new(0, 0, width, height);
        let Some(rect) = rect.intersection(window_bounds) else {
            return;
        };
        if rect.is_empty() {
            return;
        }

        let region_width = rect.width as usize;
        let region_height = rect.height as usize;
        let required_bytes = region_width.saturating_mul(region_height).saturating_mul(4);
        if self.upload_region_bytes.len() != required_bytes {
            self.upload_region_bytes.resize(required_bytes, 0);
        }

        let src_width = width as usize;
        for row in 0..region_height {
            let src_y = rect.y as usize + row;
            let src_row_start = src_y
                .saturating_mul(src_width)
                .saturating_add(rect.x as usize);
            let Some(src_row) =
                pixels_0rgb.get(src_row_start..src_row_start.saturating_add(region_width))
            else {
                continue;
            };

            let dst_row_start = row.saturating_mul(region_width).saturating_mul(4);
            for (col, rgb) in src_row.iter().enumerate() {
                let r = ((rgb >> 16) & 0xff) as u8;
                let g = ((rgb >> 8) & 0xff) as u8;
                let b = (rgb & 0xff) as u8;
                let out = dst_row_start.saturating_add(col.saturating_mul(4));
                if let Some(px) = self.upload_region_bytes.get_mut(out..out.saturating_add(4)) {
                    px[0] = r;
                    px[1] = g;
                    px[2] = b;
                    px[3] = 255;
                }
            }
        }

        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d {
                    x: rect.x,
                    y: rect.y,
                    z: 0,
                },
                aspect: wgpu::TextureAspect::All,
            },
            &self.upload_region_bytes,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(rect.width.saturating_mul(4)),
                rows_per_image: Some(rect.height),
            },
            wgpu::Extent3d {
                width: rect.width,
                height: rect.height,
                depth_or_array_layers: 1,
            },
        );
    }

    fn blit_to_surface(
        &mut self,
        surface_texture: wgpu::SurfaceTexture,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("aureline-render.blit.encoder"),
            });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("aureline-render.blit.pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                ..Default::default()
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.draw(0..6, 0..1);
        }

        let command_buffer = encoder.finish();
        self.queue.submit([command_buffer]);
        surface_texture.present();
        Ok(())
    }
}

fn create_surface_resources(
    window: &Window,
    surface: &wgpu::Surface<'static>,
    adapter: &wgpu::Adapter,
    device: &wgpu::Device,
) -> Result<
    (
        wgpu::SurfaceConfiguration,
        wgpu::Texture,
        wgpu::TextureView,
        wgpu::Sampler,
        Vec<u8>,
    ),
    Box<dyn std::error::Error>,
> {
    let size = window.inner_size();
    let width = size.width.max(1);
    let height = size.height.max(1);
    let caps = surface.get_capabilities(adapter);
    let format = caps
        .formats
        .iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or_else(|| caps.formats[0]);
    let mut config = surface
        .get_default_config(adapter, width, height)
        .ok_or("wgpu surface unsupported for adapter")?;
    config.format = format;
    config.present_mode = wgpu::PresentMode::Fifo;
    surface.configure(device, &config);

    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("aureline-render.blit.texture"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("aureline-render.blit.sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });
    let upload_bytes = vec![0; width as usize * height as usize * 4];
    Ok((config, texture, texture_view, sampler, upload_bytes))
}

fn create_pipeline(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
    surface_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader_src = r#"
struct VertexOut {
  @builtin(position) pos: vec4<f32>,
  @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) idx: u32) -> VertexOut {
  var positions = array<vec2<f32>, 6>(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>( 1.0, -1.0),
    vec2<f32>(-1.0,  1.0),
    vec2<f32>(-1.0,  1.0),
    vec2<f32>( 1.0, -1.0),
    vec2<f32>( 1.0,  1.0),
  );
  let p = positions[idx];
  var out: VertexOut;
  out.pos = vec4<f32>(p, 0.0, 1.0);
  out.uv = vec2<f32>((p.x + 1.0) * 0.5, (1.0 - p.y) * 0.5);
  return out;
}

@group(0) @binding(0) var frame_tex: texture_2d<f32>;
@group(0) @binding(1) var frame_sampler: sampler;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
  return textureSample(frame_tex, frame_sampler, in.uv);
}
"#;
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("aureline-render.blit.shader"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_src)),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("aureline-render.blit.pipeline_layout"),
        bind_group_layouts: &[bind_group_layout],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("aureline-render.blit.pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    })
}
