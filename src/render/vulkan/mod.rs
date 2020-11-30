use std::sync::Arc;
use vulkano::sync::GpuFuture;
use vulkano::command_buffer::CommandBuffer;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::device::Device;
use vulkano::instance::PhysicalDevice;
use vulkano::instance::PhysicalDeviceType;
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::image::{StorageImage, Dimensions};
use vulkano::format::{Format, ClearValue};
use image::{ImageBuffer, Rgba};

fn get_physical_device(instance: &Arc<Instance>) -> Option<PhysicalDevice> {
    #[cfg(debug_assertions)]
    {
        println!("###################################### PRINT PHYSICAL DEVICES ######################################");
        for physical_device in PhysicalDevice::enumerate(instance) {
            println!("Available device: {} (type: {:?})", physical_device.name(), physical_device.ty());
        }
    }
    let physical_device = PhysicalDevice::enumerate(instance)
        .find(|physical_device| physical_device.ty() == PhysicalDeviceType::DiscreteGpu)
        .or_else(|| PhysicalDevice::enumerate(instance).next());
    #[cfg(debug_assertions)]
    {
        match physical_device {
            Some(physical_device) => println!(
                "--- Using device: {} (type: {:?})",
                physical_device.name(),
                physical_device.ty()
            ),
            None => println!("--- Error: No device found")
        }
        println!("####################################### END PHYSICAL DEVICES #######################################");
    }
    Some(physical_device?)
}

pub fn test() {
    let instance = Instance::new(None, &InstanceExtensions::none(), None)
                        .expect("Failed to create instance");

    let physical = get_physical_device(&instance).unwrap();

    let queue_family = physical.queue_families()
                            .find(|&q| q.supports_graphics())
                            .expect("couldn't find a graphical queue family");

    let (device, mut queues) = {
        Device::new(physical, &Features::none(), &DeviceExtensions::none(),
                    [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
    };

    let queue = queues.next().unwrap();


    let image = StorageImage::new(device.clone(), Dimensions::Dim2d { width: 1024, height: 1024 },
                                  Format::R8G8B8A8Unorm, Some(queue.family())).unwrap();

    let buf = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false,
                                             (0 .. 1024 * 1024 * 4).map(|_| 0u8))
                                                        .expect("failed to create buffer");

    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
    builder
        .clear_color_image(image.clone(), ClearValue::Float([0.0, 0.0, 1.0, 1.0])).unwrap()
        .copy_image_to_buffer(image.clone(), buf.clone()).unwrap();
    let command_buffer = builder.build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
    image.save("image.png").unwrap();
}