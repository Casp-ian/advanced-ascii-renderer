use image::{DynamicImage, GenericImageView, Rgba};
use pollster::block_on;

use crate::{Args, PixelData};

struct WgpuContext {
    failed: bool,
    ready: bool,
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    pipeline: Option<wgpu::ComputePipeline>,
    bind_group: Option<wgpu::BindGroup>,
    input_texture: Option<wgpu::Texture>,
    output_storage_texture: Option<wgpu::Texture>,
    output_staging_buffer: Option<wgpu::Buffer>,
    input_texture_width: Option<u32>,
    input_texture_height: Option<u32>,
    output_width: Option<u32>,
    output_height: Option<u32>,
    input_texture_size: Option<wgpu::Extent3d>,
    output_buffer_size: Option<wgpu::BufferAddress>,
}
impl WgpuContext {
    const fn empty() -> WgpuContext {
        WgpuContext {
            failed: false,
            ready: false,
            device: None,
            queue: None,
            pipeline: None,
            bind_group: None,
            input_texture: None,
            output_storage_texture: None,
            output_staging_buffer: None,
            input_texture_width: None,
            input_texture_height: None,
            output_width: None,
            output_height: None,
            input_texture_size: None,
            output_buffer_size: None,
        }
    }

    async fn setup(
        &mut self,
        image_width: u32,
        image_height: u32,
        output_width: u32, // TODO remove?? or keep these for when we rework scaling to be in gpu
        output_height: u32,
    ) {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_defaults(),
                    memory_hints: wgpu::MemoryHints::Performance,
                },
                None,
            )
            .await
            .unwrap();

        // Our shader, kindly compiled with Naga.
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let input_texture_size = wgpu::Extent3d {
            width: image_width,
            height: image_height,
            depth_or_array_layers: 1,
        };

        let input_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("input texture"),
            size: input_texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm, // TODO might need to be rgba8Unorm according to examples
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
        });

        // For portability reasons, WebGPU draws a distinction between memory that is
        // accessible by the CPU and memory that is accessible by the GPU. Only
        // buffers accessible by the CPU can be mapped and accessed by the CPU and
        // only buffers visible to the GPU can be used in shaders. In order to get
        // data from the GPU, we need to use CommandEncoder::copy_buffer_to_buffer
        // (which we will later) to copy the buffer modified by the GPU into a
        // mappable, CPU-accessible buffer which we'll create here.

        // this one lives on the CPU i think
        let output_buffer_size = (image_width * image_height * 4) as wgpu::BufferAddress;
        let output_staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("staging buffer"),
            size: output_buffer_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // this one lives on GPU
        let output_storage_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("storage texture"),
            size: input_texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm, // TODO might need to be rgba8Unorm according to examples
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
        });

        // This can be though of as the function signature for our CPU-GPU function.
        // let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //     label: None,
        //     entries: &[
        //         // INPUT TEXTURE
        //         wgpu::BindGroupLayoutEntry {
        //             binding: 0,
        //             visibility: wgpu::ShaderStages::COMPUTE,
        //             ty: wgpu::BindingType::Texture {
        //                 sample_type: wgpu::TextureSampleType::Float { filterable: false },
        //                 view_dimension: wgpu::TextureViewDimension::D2,
        //                 multisampled: false,
        //             },
        //             count: None,
        //         },
        //         // OUTPUT TEXTURE
        //         wgpu::BindGroupLayoutEntry {
        //             binding: 1,
        //             visibility: wgpu::ShaderStages::COMPUTE,
        //             ty: wgpu::BindingType::Texture {
        //                 sample_type: wgpu::TextureSampleType::Float { filterable: false },
        //                 view_dimension: wgpu::TextureViewDimension::D2,
        //                 multisampled: false,
        //             },
        //             count: None,
        //         },
        //     ],
        // });

        // let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        //     label: None,
        //     bind_group_layouts: &[&bind_group_layout],
        //     push_constant_ranges: &[],
        // });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });

        // this is needed for the bind group to not break, and i think this will have a nicer way to do in a next version of wgpu
        let bind_group_layout = pipeline.get_bind_group_layout(0);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        &input_texture.create_view(&wgpu::TextureViewDescriptor::default()),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(
                        &output_storage_texture
                            .create_view(&wgpu::TextureViewDescriptor::default()),
                    ),
                },
            ],
        });

        self.ready = true;
        self.device = Some(device);
        self.queue = Some(queue);
        self.pipeline = Some(pipeline);
        self.bind_group = Some(bind_group);

        self.input_texture = Some(input_texture);
        self.output_storage_texture = Some(output_storage_texture);
        self.output_staging_buffer = Some(output_staging_buffer);

        self.input_texture_width = Some(image_width);
        self.input_texture_height = Some(image_height);
        self.output_width = Some(output_width);
        self.output_height = Some(output_height);

        self.input_texture_size = Some(input_texture_size);
        self.output_buffer_size = Some(output_buffer_size);
    }

    async fn process(
        &mut self,
        input_image: image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<Vec<u32>, &str> {
        let device = self
            .device
            .as_ref()
            .expect("This should have been set in setup()");
        let queue = self
            .queue
            .as_ref()
            .expect("This should have been set in setup()");
        let pipeline = self
            .pipeline
            .as_ref()
            .expect("This should have been set in setup()");
        let bind_group = self
            .bind_group
            .as_ref()
            .expect("This should have been set in setup()");

        let input_texture = self
            .input_texture
            .as_ref()
            .expect("This should have been set in setup()");
        let output_storage_texture = self
            .output_storage_texture
            .as_ref()
            .expect("This should have been set in setup()");
        let output_staging_buffer = self
            .output_staging_buffer
            .as_ref()
            .expect("This should have been set in setup()");

        let input_texture_width = self
            .input_texture_width
            .as_ref()
            .expect("This should have been set in setup()");
        let input_texture_height = self
            .input_texture_height
            .as_ref()
            .expect("This should have been set in setup()");
        let output_width = self
            .output_width
            .as_ref()
            .expect("This should have been set in setup()");
        let output_height = self
            .output_height
            .as_ref()
            .expect("This should have been set in setup()");

        let input_texture_size = self
            .input_texture_size
            .as_ref()
            .expect("This should have been set in setup()");
        let output_buffer_size = self
            .output_buffer_size
            .as_ref()
            .expect("This should have been set in setup()");

        // if &size_of_val(input_image) != size {
        //     self.failed = true;
        //     return Err("input size changed");
        // }

        // Local buffer contents -> GPU storage buffer
        // Adds a write buffer command to the queue. This command is more complicated
        // than it appears.
        // queue.write_buffer(&input_texture, 0, bytemuck::cast_slice(input));

        eprintln!("{}", input_image.len());
        eprintln!("{}", input_image.width());
        eprintln!("{}", input_image.height());
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: input_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(input_image.as_raw()),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * input_texture_width),
                rows_per_image: Some(*input_texture_height),
            },
            wgpu::Extent3d {
                width: *input_texture_width,
                height: *input_texture_height,
                depth_or_array_layers: 1,
            },
            // *input_texture_size,
        );

        // A command encoder executes one or many pipelines.
        // It is to WebGPU what a command buffer is to Vulkan.
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&pipeline);
            compute_pass.set_bind_group(0, Some(bind_group), &[]);
            compute_pass.insert_debug_marker("compute collatz iterations"); // TODO lol correct marker

            // TODO workgroup count https://blog.redwarp.app/image-filters/
            // Number of cells to run, the (x,y,z) size of item being processed
            // TODO this should be output, not input... i think
            compute_pass.dispatch_workgroups(*input_texture_width, *input_texture_height, 1);
            // TODO fucked according to tutorial
        }

        // Sets adds copy operation to command encoder.
        // Will copy data from storage buffer on GPU to staging buffer on CPU.
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTextureBase {
                texture: &output_storage_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBufferBase {
                buffer: &output_staging_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(input_texture_width * 4),
                    rows_per_image: Some(*input_texture_height),
                },
            },
            wgpu::Extent3d {
                width: *input_texture_width,
                height: *input_texture_height,
                depth_or_array_layers: 1,
            },
        );

        // Submits command encoder for processing
        queue.submit(Some(encoder.finish()));

        // Note that we're not calling `.await` here.
        let buffer_slice = output_staging_buffer.slice(..);
        // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
        let (sender, receiver) = flume::bounded(1);
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        // Poll the device in a blocking manner so that our future resolves.
        // In an actual application, `device.poll(...)` should
        // be called in an event loop or on another thread.
        // TODO the comment above is from the example, i do want this to be an actual application
        device.poll(wgpu::Maintain::wait()).panic_on_timeout();

        // Awaits until `buffer_future` can be read from
        if let Ok(Ok(())) = receiver.recv_async().await {
            // Gets contents of buffer
            let data = buffer_slice.get_mapped_range();
            // Since contents are got in bytes, this converts these bytes back to u32
            let result = bytemuck::cast_slice(&data).to_vec();

            // With the current interface, we have to make sure all mapped views are
            // dropped before we unmap the buffer.
            drop(data);
            output_staging_buffer.unmap(); // Unmaps buffer from memory
                                           // If you are familiar with C++ these 2 lines can be thought of similarly to:
                                           //   delete myPointer;
                                           //   myPointer = NULL;
                                           // It effectively frees the memory

            // Returns data from buffer
            return Ok(result);
        } else {
            self.failed = true;
            return Err("cant run on gpu");
        }
    }
}

// maybe std::cell can save us from unsafe
static mut CONTEXT: WgpuContext = WgpuContext::empty();

pub fn try_process_on_gpu(
    image: DynamicImage,
    width: u32,
    height: u32,
    args: &Args,
) -> Result<Vec<Vec<PixelData>>, &str> {
    // TODO do i have to be unsafe? i think its fine tho
    if unsafe { CONTEXT.failed } {
        return Err("failed before, not retrying");
    }

    // TODO maybe move this check into context
    if unsafe { !CONTEXT.ready } {
        // let (image_width, image_height) = image.dimensions();
        block_on(unsafe { CONTEXT.setup(64, 64, width, height) });
    }

    let compute_result = block_on(unsafe {
        CONTEXT.process(
            image
                .resize_exact(64, 64, image::imageops::FilterType::Triangle)
                .to_rgba8(),
        )
    });
    match compute_result {
        Ok(data) => {
            println!("gpu returned {:?}", data);
            println!("{} to {}", data.len(), width);
            // TODO actually return
            return Ok(vec![]);
        }
        Err(message) => return Err(message),
    }
}
