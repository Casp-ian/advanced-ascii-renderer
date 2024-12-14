use image::{DynamicImage, Rgba};

pub struct WgpuContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    input_texture: wgpu::Texture,
    output_storage_texture: wgpu::Texture,
    output_staging_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    input_width: u32,
    input_height: u32,
    output_width: u32,
    output_height: u32,
    input_texture_size: wgpu::Extent3d,
    output_buffer_size: wgpu::BufferAddress,
}

impl WgpuContext {
    // TODO get rid of unwraps for better error messages
    pub async fn setup(
        image_width: u32,
        image_height: u32,
        output_width: u32,
        output_height: u32,
    ) -> Result<WgpuContext, String> {
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
            format: wgpu::TextureFormat::Rgba8Unorm,
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
            format: wgpu::TextureFormat::Rgba8Snorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::COPY_DST,
            view_formats: &[wgpu::TextureFormat::Rgba8Snorm],
        });

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniform buffer"),
            size: 16 as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            mapped_at_creation: false,
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

        // NOTE this is needed for the bind group to not break, and i hope this will have a nicer way to do in a next version of wgpu
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
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &uniform_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });

        return Ok(WgpuContext {
            device,
            queue,
            pipeline,
            bind_group,
            input_texture,
            output_storage_texture,
            output_staging_buffer,
            uniform_buffer,
            input_width: image_width,
            input_height: image_height,
            output_width,
            output_height,
            input_texture_size,
            output_buffer_size,
        });
    }

    pub async fn process(
        &self,
        input_image: image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<Vec<i8>, &str> {
        // TODO
        // if &size_of_val(input_image) != size {
        //     return Err("input size changed");
        // }

        // Local buffer contents -> GPU storage buffer
        // Adds a write buffer command to the queue. This command is more complicated
        // than it appears.
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice::<u32, u8>(&[
                self.input_width,
                self.input_height,
                self.output_width,
                self.output_height,
            ]),
        );

        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.input_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(input_image.as_raw()),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(self.input_width * 4),
                rows_per_image: Some(self.input_height),
            },
            self.input_texture_size,
        );

        // A command encoder executes one or many pipelines.
        // It is to WebGPU what a command buffer is to Vulkan.
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipeline);
            compute_pass.set_bind_group(0, Some(&self.bind_group), &[]);
            compute_pass.insert_debug_marker("compute shader");

            compute_pass.dispatch_workgroups(self.input_width, self.input_height, 1);
        }

        // Sets adds copy operation to command encoder.
        // Will copy data from storage buffer on GPU to staging buffer on CPU.
        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTextureBase {
                texture: &self.output_storage_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::ImageCopyBufferBase {
                buffer: &self.output_staging_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(self.input_width * 4),
                    rows_per_image: Some(self.input_height),
                },
            },
            self.input_texture_size,
        );

        // Submits command encoder for processing
        self.queue.submit(Some(encoder.finish()));

        // Note that we're not calling `.await` here.
        let buffer_slice = self.output_staging_buffer.slice(..);
        // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
        let (sender, receiver) = flume::bounded(1);
        buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

        // Poll the device in a blocking manner so that our future resolves.
        // In an actual application, `device.poll(...)` should
        // be called in an event loop or on another thread.
        self.device.poll(wgpu::Maintain::wait()).panic_on_timeout();

        // Awaits until `buffer_future` can be read from
        if let Ok(Ok(())) = receiver.recv_async().await {
            // Gets contents of buffer
            let data = buffer_slice.get_mapped_range();
            // Since contents are got in bytes, this converts these bytes back to u32
            let result = bytemuck::cast_slice(&data).to_vec();

            // With the current interface, we have to make sure all mapped views are
            // dropped before we unmap the buffer.
            drop(data);
            self.output_staging_buffer.unmap(); // Unmaps buffer from memory
                                                // If you are familiar with C++ these 2 lines can be thought of similarly to:
                                                //   delete myPointer;
                                                //   myPointer = NULL;
                                                // It effectively frees the memory

            // Returns data from buffer
            return Ok(result);
        } else {
            return Err("cant run on gpu"); // TODO stupid message
        }
    }
}
