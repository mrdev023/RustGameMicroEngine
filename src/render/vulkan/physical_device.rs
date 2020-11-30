use vulkano::instance::{Instance, PhysicalDevice, PhysicalDeviceType};
use std::sync::Arc;

pub fn get_physical_device(instance: &Arc<Instance>) -> Option<PhysicalDevice> {
    #[cfg(debug_assertions)]
    {
        println!("###################################### PRINT PHYSICAL DEVICES ######################################");
        for physical_device in PhysicalDevice::enumerate(instance) {
            println!(
                "Available device:   {} (type: {:?})\
                \n\t\t\t\t\tDriver version: {}\
                \n\t\t\t\t\tAPI Version: {:?}\
                \n\t\t\t\t\tVendor ID: {}\
                \n\t\t\t\t\tDevice ID: {}",
                physical_device.name(),
                physical_device.ty(),
                physical_device.driver_version(),
                physical_device.api_version(),
                physical_device.pci_vendor_id(),
                physical_device.pci_device_id()
            );
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