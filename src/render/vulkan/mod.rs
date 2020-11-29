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

#[cfg(debug_assertions)]
fn get_physical_device<'a>(instance: &'a Arc<Instance>) -> Option<PhysicalDevice<'a>> {
    println!("###################################### PRINT PHYSICAL DEVICES ######################################");
    for physical_device in PhysicalDevice::enumerate(instance) {
        println!("Available device: {} (type: {:?})", physical_device.name(), physical_device.ty());
    }
    let physical_device = match PhysicalDevice::enumerate(instance).find(|physical_device| physical_device.ty() == PhysicalDeviceType::DiscreteGpu) {
        Some(physical_device) => Some(physical_device),
        None => match PhysicalDevice::enumerate(instance).next() {
            Some(physical_device) => Some(physical_device),
            None => None
        }
    };
    match physical_device {
        Some(physical_device) => println!(
            "--- Using device: {} (type: {:?})",
            physical_device.name(),
            physical_device.ty()
        ),
        None => println!("--- Error: No device found")
    }
    println!("####################################### END PHYSICAL DEVICES #######################################");
    physical_device
}

#[cfg(not(debug_assertions))]
fn get_physical_device<'a>(instance: &'a Arc<Instance>) -> Option<PhysicalDevice<'a>> {
    match PhysicalDevice::enumerate(instance).find(|physical_device| physical_device.ty() == PhysicalDeviceType::DiscreteGpu) {
        Some(physical_device) => Some(physical_device),
        None => match PhysicalDevice::enumerate(instance).next() {
            Some(physical_device) => Some(physical_device),
            None => None
        }
    }
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

    let source_content = 0 .. 64;
    let source = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false,
                                            source_content).expect("failed to create buffer");

    let dest_content = (0 .. 64).map(|_| 0);
    let dest = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false,
                                            dest_content).expect("failed to create buffer");

    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
    builder.copy_buffer(source.clone(), dest.clone()).unwrap();
    let command_buffer = builder.build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();

    finished.then_signal_fence_and_flush().unwrap()
                        .wait(None).unwrap();

    let src_content = source.read().unwrap();
    let dest_content = dest.read().unwrap();
    assert_eq!(&*src_content, &*dest_content);
}