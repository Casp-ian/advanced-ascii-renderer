use image::Rgba;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

pub struct WgpuContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline_edges: wgpu::ComputePipeline,
    bind_group_edges: wgpu::BindGroup,
    pipeline_scale: wgpu::ComputePipeline,
    bind_group_scale: wgpu::BindGroup,
    input_buffer: wgpu::Buffer,
    intermediate_storage_buffer: wgpu::Buffer,
    output_storage_buffer: wgpu::Buffer,
    output_staging_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    input_width: u32,
    input_height: u32,
    output_width: u32,
    output_height: u32,
    input_buffer_size: wgpu::BufferAddress,
    intermediate_buffer_size: wgpu::BufferAddress,
    output_buffer_size: wgpu::BufferAddress,
}

impl WgpuContext {
    // TODO get rid of unwraps for better error messages
    pub async fn setup(
        input_width: u32,
        input_height: u32,
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

        // let input_texture_size = wgpu::Extent3d {
        //     width: input_width,
        //     height: input_height,
        //     depth_or_array_layers: 1,
        // };

        // 4 u8 for every pixel 4 * 1
        let input_buffer_size = (input_width * input_height * 4) as wgpu::BufferAddress;
        let input_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("staging buffer"),
            size: input_buffer_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // let input_texture = device.create_texture(&wgpu::TextureDescriptor {
        //     label: Some("input texture"),
        //     size: input_texture_size,
        //     mip_level_count: 1,
        //     sample_count: 1,
        //     dimension: wgpu::TextureDimension::D2,
        //     format: wgpu::TextureFormat::Rgba8Unorm,
        //     usage: wgpu::TextureUsages::TEXTURE_BINDING
        //         | wgpu::TextureUsages::COPY_DST
        //         | wgpu::TextureUsages::RENDER_ATTACHMENT,
        //     view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
        // });

        // For portability reasons, WebGPU draws a distinction between memory that is
        // accessible by the CPU and memory that is accessible by the GPU. Only
        // buffers accessible by the CPU can be mapped and accessed by the CPU and
        // only buffers visible to the GPU can be used in shaders. In order to get
        // data from the GPU, we need to use CommandEncoder::copy_buffer_to_buffer
        // (which we will later) to copy the buffer modified by the GPU into a
        // mappable, CPU-accessible buffer which we'll create here.

        // 1 u32 enum for every pixel 1 * 4 = 4
        let intermediate_buffer_size = (input_width * input_height * 4) as wgpu::BufferAddress;
        // this one lives on GPU
        let intermediate_storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("staging buffer"),
            size: intermediate_buffer_size,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // 3 f32 for every pixel, 4 bytes for every f32, 3 * 4 = 12
        let output_buffer_size = (output_width * output_height * 12) as wgpu::BufferAddress;
        // this one lives on the CPU i think
        let output_staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("staging buffer"),
            size: output_buffer_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // this one lives on GPU
        let output_storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("staging buffer"),
            size: output_buffer_size,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_SRC
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("uniform buffer"),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            contents: bytemuck::cast_slice::<u32, u8>(&[
                input_width,
                input_height,
                output_width,
                output_height,
            ]),
        });

        let pipeline_edges = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &shader,
            entry_point: Some("do_edges"),
            compilation_options: Default::default(),
            cache: None,
        });

        let bind_group_edges = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            // this layout using an empty layout from the pipeline looks weird but works
            // from documentation: "If this pipeline was created with a default layout, then bind groups created with the returned BindGroupLayout can only be used with this pipeline."
            layout: &pipeline_edges.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &uniform_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    // resource: wgpu::BindingResource::TextureView(
                    //     &input_texture.create_view(&wgpu::TextureViewDescriptor::default()),
                    // ),
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &input_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &intermediate_storage_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });

        let pipeline_scale = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &shader,
            entry_point: Some("do_scale"),
            compilation_options: Default::default(),
            cache: None,
        });

        let bind_group_scale = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline_scale.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &uniform_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    // resource: wgpu::BindingResource::TextureView(
                    //     &input_texture.create_view(&wgpu::TextureViewDescriptor::default()),
                    // ),
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &input_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &intermediate_storage_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &output_storage_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
            ],
        });

        return Ok(WgpuContext {
            device,
            queue,
            pipeline_edges,
            bind_group_edges,
            pipeline_scale,
            bind_group_scale,
            input_buffer,
            intermediate_storage_buffer,
            output_storage_buffer,
            output_staging_buffer,
            uniform_buffer,
            input_width,
            input_height,
            output_width,
            output_height,
            input_buffer_size,
            intermediate_buffer_size,
            output_buffer_size,
        });
    }

    // NOTE this functions return type isnt really representative, most of the values will be reinterpreted into something other than f32
    pub async fn process(
        &self,
        input_image: image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<Vec<f32>, &str> {
        // will get the image to process to the gpu
        self.queue
            .write_buffer(&self.input_buffer, 0, input_image.as_raw());

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
            compute_pass.set_pipeline(&self.pipeline_edges);
            compute_pass.set_bind_group(0, Some(&self.bind_group_edges), &[]);
            compute_pass.insert_debug_marker("edges");

            compute_pass.dispatch_workgroups(self.input_width, self.input_height, 1);
        }

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.pipeline_scale);
            compute_pass.set_bind_group(0, Some(&self.bind_group_scale), &[]);
            compute_pass.insert_debug_marker("scale");

            compute_pass.dispatch_workgroups(self.output_width, self.output_height, 1);
        }

        // will get the output back to the cpu
        encoder.copy_buffer_to_buffer(
            &self.output_storage_buffer,
            0,
            &self.output_staging_buffer,
            0,
            self.output_buffer_size,
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
            // let result = data.to_vec();

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
