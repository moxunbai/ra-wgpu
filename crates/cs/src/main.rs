use app_surface::{AppSurface, SurfaceFrame};
// use wgpu::Buffer;
// use wgpu::BufferUsages;
// use wgpu::ComputePassDescriptor;
// use wgpu::Error;
// use wgpu::ShaderStages;
use wgpu::{
    ComputePassDescriptor,   BufferUsages, util::DeviceExt};
// use wgpu::TextureUsages;
// use wgpu::TextureViewDimension;
use anyhow::{   Result};
// use wgpu::Device;
// use std::iter;
use utils::framework::{run, Action};
use winit::{dpi::PhysicalSize, window::WindowId}; 
// use wgpu::util::BufferInitDescriptor;
// use wgpu::util::TextureDescriptor;
// use std::future::Future;
// use byteorder::{BigEndian, ReadBytesExt};

// mod image_node;
// use image_node::ImageNode;


// const SWAP_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
// const BUF_SIZE:u64 =20;

struct NullWake;

impl std::task::Wake for NullWake {
    fn wake(self: std::sync::Arc<Self>) {}
}

struct State {
    app: AppSurface,
    // NEW!
    // render_pipeline: wgpu::RenderPipeline,
    workgroup_count:(u32, u32), 
    compute_pipeline: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    // demo_unifom:DemoUnifom,
    // buf: wgpu::Buffer,
    storage_buffer: wgpu::Buffer,
    staging_buffer: wgpu::Buffer, 
    size:u64,
}
// #[repr(C)]
// #[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
// struct DemoUnifom {
//     value:[f32 ; 20],
// }

// impl DemoUnifom {
//     fn new() -> Self {
//         let mut a:[f32 ; 20]=[0.0;20];
//         for i  in 0..20 {
//             a[i] = ((1+i) as f32 )*1.1;
//         };
//         Self {
//             value:a
//         }
//     }
// }

// struct BufferProperties {
//     size: u64,
//     usages: BufferUsages,
//     #[cfg(feature = "buffer_labels")]
//     name: &'static str,
// }
// fn get_buf( 
//         size: u64,
//         #[allow(unused)] name: &'static str,
//         usage: BufferUsages,
//         app: &AppSurface, 
//     ) -> Buffer {
//         let rounded_size = size;
//         let props = BufferProperties {
//             size: rounded_size,
//             usages: usage,
//             #[cfg(feature = "buffer_labels")]
//             name,
//         };
         
//         app.device.create_buffer(&wgpu::BufferDescriptor {
//             #[cfg(feature = "buffer_labels")]
//             label: Some(name),
//             #[cfg(not(feature = "buffer_labels"))]
//             label: None,
//             size: rounded_size,
//             usage,
//             mapped_at_creation: false,
//         })
//     }

//     /// This will deadlock if the future is awaiting anything other than GPU progress.
// pub fn block_on_wgpu<F: Future>(device: &Device, mut fut: F) -> F::Output {
//     let waker = std::task::Waker::from(std::sync::Arc::new(NullWake));
//     let mut context = std::task::Context::from_waker(&waker);
//     // Same logic as `pin_mut!` macro from `pin_utils`.
//     let mut fut = unsafe { std::pin::Pin::new_unchecked(&mut fut) };
//     loop {
//         match fut.as_mut().poll(&mut context) {
//             std::task::Poll::Pending => {
//                 device.poll(wgpu::Maintain::Wait);
//             }
//             std::task::Poll::Ready(item) => break item,
//         }
//     }
// }

//  async fn genBuf(toBuf:&Buffer ,buf:&Buffer,app: &AppSurface, )->Result<()>  {
//         // let usage = BufferUsages::MAP_READ | BufferUsages::COPY_DST;
//         //     let _buf =  get_buf(80, "download", usage, app);
//         //     encoder.copy_buffer_to_buffer( buf, 0, &_buf, 0, 80);
        
//         let buf_slice = toBuf.slice(..);
//         let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
//         buf_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
//         if let Some(recv_result) = block_on_wgpu(&app.device, receiver.receive()) {
//             recv_result?;
//         } else {
//             // println!("hahahah  ERROR");
//             bail!("channel was closed");
//         }
            
//             let mapped = buf_slice.get_mapped_range(); 
             
//             for row in 0..20 {
//                 let start = row * 4;
//                 let bs=[mapped[start],mapped[start+1],mapped[start+2],mapped[start+3]];
                
//                 let aaaaa=f32::from_be_bytes(bs);
//                 println!("qqqqqqqqqq=={:?}======{:?}",row, aaaaa);
//             }
//             // f32::from_be_bytes(mapped);

//             // println!("qqqqqqqqqq========{:?}", f);
//             Ok(())
//     }

impl Action for State {
    fn new(app: AppSurface) -> Self {

         
         
        let numbers = vec![1, 2, 3, 4];

        let slice_size = numbers.len() * std::mem::size_of::<u32>();
        let size = slice_size as wgpu::BufferAddress; 
        let staging_buffer = app.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Instantiates buffer with data (`numbers`).
        // Usage allowing the buffer to be:
        //   A storage buffer (can be bound within a bind group and thus available to a shader).
        //   The destination of a copy.
        //   The source of a copy.
        let storage_buffer = app.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Storage Buffer"),
            contents: bytemuck::cast_slice(&numbers),
            usage: BufferUsages::STORAGE
                | BufferUsages::COPY_DST
                | BufferUsages::COPY_SRC,
        }); 
 
        let cs_shader= {
            let create_shader = |wgsl: &'static str| -> wgpu::ShaderModule {
                app.device
                    .create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: None,
                        source: wgpu::ShaderSource::Wgsl(wgsl.into()),
                    })
            };
            create_shader(include_str!("../assets/cs2.wgsl"))
            
        };

        
        let compute_pipeline = app
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            layout: None,
            module: &cs_shader,
            entry_point: "cs_main",
            label: None,
        });

        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);   
        let bind_group = app.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: storage_buffer.as_entire_binding(),
            }],
        }); 

        let workgroup_count = ((1000 + (32 -1)) / 32, (768 + (16 -1)) / 16);
 

        Self {
            app,
            compute_pipeline,
            workgroup_count,
            bind_group,
            // demo_unifom,
            // buf,
            storage_buffer,
            staging_buffer, 
            size
        }
    }

    fn get_adapter_info(&self) -> wgpu::AdapterInfo {
        self.app.adapter.get_info()
    }
    fn current_window_id(&self) -> WindowId {
        self.app.view.id()
    }
    fn resize(&mut self, size: &PhysicalSize<u32>) {
        if self.app.config.width == size.width && self.app.config.height == size.height {
            return;
        }
        self.app.resize_surface();
    }
    fn request_redraw(&mut self) {
        self.app.view.request_redraw();
    }
   

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // let (output, view) = self.app.get_current_frame_view(Some(self.app.config.format.remove_srgb_suffix()));
        let mut encoder = self
            .app
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut cpass = encoder.begin_compute_pass(&ComputePassDescriptor::default());
            cpass.set_pipeline(&self.compute_pipeline);
            cpass.set_bind_group(0, &self.bind_group, &[]);
            cpass.dispatch_workgroups(4 as u32, 1, 1); 
            // cpass.dispatch_workgroups(self.workgroup_count.0, self.workgroup_count.1, 1);
        }

        encoder.copy_buffer_to_buffer(&self.storage_buffer, 0, &self.staging_buffer, 0, self.size);

         
        // let mut bump: Option<BumpAllocators> = None;
        // let usage = BufferUsages::MAP_READ | BufferUsages::COPY_DST;
        // let _buf =  get_buf(80, "download", usage, &self.app);
        // encoder.copy_buffer_to_buffer( &self.buf, 0, &_buf, 0, 80);
        // let ft= genBuf(&_buf,&self.buf,&self.app);
       
        

        self.app.queue.submit(Some(encoder.finish()));

        // Note that we're not calling `.await` here.
        let buffer_slice = self.staging_buffer.slice(..);
        // Gets the future representing when `staging_buffer` can be read from
        let (tx, rx) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read,move |result| {
                tx.send(result).unwrap();
            });

           
            // Poll the device in a blocking manner so that our future resolves.
            // In an actual application, `device.poll(...)` should
            // be called in an event loop or on another thread.
        {
            self.app.device.poll(wgpu::Maintain::Wait);
            rx.recv().unwrap().unwrap();
            let data = buffer_slice.get_mapped_range();
            let result:Vec<u32> = data
            .chunks_exact(4)
            .map(|b| u32::from_ne_bytes(b.try_into().unwrap()))
            .collect();
    
            // With the current interface, we have to make sure all mapped views are
            // dropped before we unmap the buffer.
            drop(data);
            self.staging_buffer.unmap();  
             
            // output.present();
            println!("RRRRRRRRR====={:?}", result);

        }    
        //  pollster::block_on(ft);
        //  pollster::block_on(buffer_future);
        //   println!("qqqqqqqqqq========{:?}", self.demo_unifom.value);
        Ok(())
    }
}

pub fn main() {
    run::<State>(None, None);
}