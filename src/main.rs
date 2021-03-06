extern crate tinyfiledialogs as tfd;

use ash::vk;
use std::ffi::c_void;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn main() {
    //Initialize the Vulkan API
    let vk_entry = ash::Entry::linked();
    let vk_instance = {
        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };

        let layer_names = [];
        let extension_names = [];
        let vk_create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            enabled_extension_count: extension_names.len() as u32,
            pp_enabled_extension_names: &extension_names as *const *const _,
            enabled_layer_count: layer_names.len() as u32,
            pp_enabled_layer_names: &layer_names as *const *const _,
            ..Default::default()
        };

        unsafe { vk_entry.create_instance(&vk_create_info, None).unwrap() }
    };

    //Write out device info for each physical device found in the system
    unsafe {
        match vk_instance.enumerate_physical_devices() {
            Ok(phys_devices) => {
                let path = "./vulkan_physical_device_info.txt";
                let mut out_file = OpenOptions::new().write(true).create(true).open(path).unwrap();
                for device in phys_devices {
                    let mut indexing_features = vk::PhysicalDeviceDescriptorIndexingFeatures::default();
                    let properties = vk_instance.get_physical_device_properties(device);
                    let mut physical_device_features = vk::PhysicalDeviceFeatures2 {
                        p_next: &mut indexing_features as *mut _ as *mut c_void,
                        ..Default::default()
                    };
                    vk_instance.get_physical_device_features2(device, &mut physical_device_features);
                    writeln!(out_file, "Device {:?}", device).unwrap();
                    writeln!(out_file, "{:#?}", properties).unwrap();
                    writeln!(out_file, "{:#?}", physical_device_features).unwrap();
                    writeln!(out_file, "{:#?}", indexing_features).unwrap();

                    let full_path = Path::new(path).canonicalize().unwrap();
                    let message = format!("Graphics device info written to {}", full_path.to_str().unwrap());
                    tfd::message_box_ok("Success!", &message, tfd::MessageBoxIcon::Info);
                }
            }
            Err(e) => {
                let message = format!("Unable to enumerate physical devices: {}", e);
                tfd::message_box_ok("Error enumerating physical devices", &message, tfd::MessageBoxIcon::Error);
                panic!("{}", message);
            }
        }
    }
}
