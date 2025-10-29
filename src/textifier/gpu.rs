use image::{Luma, Rgb, Rgba};
use wgpu::{
    ExperimentalFeatures,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::textifier::types::{Direction, PixelData};

pub struct WgpuContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline_edges: wgpu::ComputePipeline,
    bind_group_edges: wgpu::BindGroup,
    pipeline_scale: wgpu::ComputePipeline,
    bind_group_scale: wgpu::BindGroup,
    input_buffer: wgpu::Buffer,
    // intermediate_storage_buffer: wgpu::Buffer,
    output_storage_buffer: wgpu::Buffer,
    output_staging_buffer: wgpu::Buffer,
    // uniform_buffer: wgpu::Buffer,
    input_width: u32,
    input_height: u32,
    output_width: u32,
    output_height: u32,
    // input_buffer_size: wgpu::BufferAddress,
    // intermediate_buffer_size: wgpu::BufferAddress,
    output_buffer_size: wgpu::BufferAddress,
}

impl WgpuContext {
    // TODO get rid of unwraps for better error messages
    pub async fn setup(
        input_width: u32,
        input_height: u32,
        output_width: u32,
        output_height: u32,
        threshold: f32,
        lines: image::ImageBuffer<Luma<u8>, Vec<u8>>,
    ) -> Result<WgpuContext, String> {
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
                memory_hints: wgpu::MemoryHints::Performance,
                experimental_features: ExperimentalFeatures::disabled(),
                trace: wgpu::Trace::Off,
            })
            .await
            .unwrap();

        // Our shader, kindly compiled with Naga.
        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        // 4 u8 for every pixel 4 * 1
        let input_buffer_size = (input_width * input_height * 4) as wgpu::BufferAddress;
        let input_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("input buffer"),
            size: input_buffer_size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

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

        let config_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("uniform buffer"),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            contents: bytemuck::cast_slice::<u32, u8>(&[
                input_width,
                input_height,
                output_width,
                output_height,
                bytemuck::cast::<f32, u32>(threshold),
            ]),
        });

        // TODO unhardcode this shit
        // for now 5 chars high, every char is 8x8
        let line_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("line buffer"),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
            contents: lines.as_raw(),
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
                        buffer: &config_buffer,
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
                // settings
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &config_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                // input
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
                // intermediate
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &intermediate_storage_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                // output
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &output_storage_buffer,
                        offset: 0,
                        size: None,
                    }),
                },
                // lines
                // wgpu::BindGroupEntry {
                //     binding: 4,
                //     resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                //         buffer: &line_buffer,
                //         offset: 0,
                //         size: None,
                //     }),
                // },
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
            // intermediate_storage_buffer,
            output_storage_buffer,
            output_staging_buffer,
            // uniform_buffer,
            input_width,
            input_height,
            output_width,
            output_height,
            // input_buffer_size,
            // intermediate_buffer_size,
            output_buffer_size,
        });
    }

    pub async fn process(
        &self,
        input_image: image::ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<Vec<Vec<PixelData>>, String> {
        // will get the image to the gpu
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

        // We now map the download buffer so we can read it. Mapping tells wgpu that we want to read/write
        // to the buffer directly by the CPU and it should not permit any more GPU operations on the buffer.
        //
        // Mapping requires that the GPU be finished using the buffer before it resolves, so mapping has a callback
        // to tell you when the mapping is complete.
        let buffer_slice = self.output_staging_buffer.slice(..);
        buffer_slice.map_async(wgpu::MapMode::Read, |_| {
            // In this case we know exactly when the mapping will be finished,
            // so we don't need to do anything in the callback.
        });

        // Wait for the GPU to finish working on the submitted work. This doesn't work on WebGPU, so we would need
        // to rely on the callback to know when the buffer is mapped.
        self.device
            .poll(wgpu::PollType::wait_indefinitely())
            .unwrap();

        // We can now read the data from the buffer.
        let data = buffer_slice.get_mapped_range();

        // Convert the data back to a slice of f32.
        let raw_result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();

        // Unmaps buffer from memory
        drop(data);
        self.output_staging_buffer.unmap();

        // Cast bytes to correct type
        let single_vec_data: Vec<PixelData> = raw_result
            .chunks_exact(3)
            .map(|x| PixelData {
                direction: Direction::from_int(bytemuck::cast(x[0])),
                color: Rgb([
                    bytemuck::cast_slice::<f32, u8>(&[x[1]])[0],
                    bytemuck::cast_slice::<f32, u8>(&[x[1]])[1],
                    bytemuck::cast_slice::<f32, u8>(&[x[1]])[2],
                ]),
                brightness: x[2],
            })
            .collect();

        let result = single_vec_data
            .chunks(self.output_width as usize)
            .map(|x| x.to_vec())
            .collect::<Vec<Vec<PixelData>>>();

        return Ok(result);
    }
}
