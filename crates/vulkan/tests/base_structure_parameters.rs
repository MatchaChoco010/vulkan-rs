use std::cell::Cell;
use std::ffi::{CStr, c_char};
use vulkan::vk::{self, VkResult};
use vulkan::{Device, Instance};

#[derive(Default)]
struct Driver {
    passed: Cell<usize>,
    passed_type: Cell<vk::StructureType>,
}

type GetPipelineProperties = unsafe extern "system" fn(
    vk::Device,
    *const vk::PipelineInfoKHR<'_>,
    *mut vk::BaseOutStructure<'_>,
) -> VkResult;

fn identifier_byte(index: usize) -> u8 {
    0xa0 + index as u8
}

unsafe extern "system" fn get_pipeline_properties(
    device: vk::Device,
    _pipeline_info: *const vk::PipelineInfoKHR<'_>,
    pipeline_properties: *mut vk::BaseOutStructure<'_>,
) -> VkResult {
    let driver = unsafe { &*device.0.cast::<Driver>() };
    driver.passed.set(pipeline_properties as usize);
    let s_type = unsafe { (*pipeline_properties).s_type };
    driver.passed_type.set(s_type);
    if s_type != vk::StructureType::PIPELINE_PROPERTIES_IDENTIFIER_EXT {
        return VkResult::ERROR_UNKNOWN;
    }
    let identifier = pipeline_properties.cast::<vk::PipelinePropertiesIdentifierEXT<'_>>();
    for index in 0..vk::UUID_SIZE {
        unsafe { (*identifier).pipeline_identifier[index] = identifier_byte(index) };
    }
    VkResult::SUCCESS
}

unsafe extern "system" fn get_device_proc_addr(
    _device: vk::Device,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    match unsafe { CStr::from_ptr(name) }.to_bytes() {
        b"vkGetPipelinePropertiesEXT" => Some(unsafe {
            std::mem::transmute::<GetPipelineProperties, unsafe extern "system" fn()>(
                get_pipeline_properties,
            )
        }),
        _ => None,
    }
}

#[test]
fn the_driver_writes_the_whole_structure_that_get_pipeline_properties_is_given() {
    let driver = Driver::default();
    let handle = vk::Device(std::ptr::from_ref(&driver).cast_mut().cast());
    let fp = unsafe { vk::DeviceFn::load(Some(get_device_proc_addr), handle) };
    let device = Device::from_raw(handle, fp);

    assert!(
        size_of::<vk::PipelinePropertiesIdentifierEXT<'_>>()
            > size_of::<vk::BaseOutStructure<'_>>(),
        "the structure that the command writes is larger than VkBaseOutStructure"
    );
    let mut identifier = vk::PipelinePropertiesIdentifierEXT::default();
    let address = std::ptr::from_ref(&identifier) as usize;
    let result = unsafe {
        device.get_pipeline_properties_ext(&vk::PipelineInfoKHR::default(), &mut identifier)
    };
    assert_eq!(result, Ok(()));
    assert_eq!(
        driver.passed.get(),
        address,
        "the driver is given the address of the structure"
    );
    assert!(
        driver.passed_type.get() == vk::StructureType::PIPELINE_PROPERTIES_IDENTIFIER_EXT,
        "the driver reads the sType of the structure"
    );
    let expected: [u8; vk::UUID_SIZE] = std::array::from_fn(identifier_byte);
    assert_eq!(
        identifier.pipeline_identifier, expected,
        "every byte that the driver wrote after sType and pNext is in the structure"
    );
}

type GetEngineOperationProperties = unsafe extern "system" fn(
    vk::PhysicalDevice,
    u32,
    *const vk::QueueFamilyDataGraphPropertiesARM<'_>,
    *mut vk::BaseOutStructure<'_>,
) -> VkResult;

const LEVEL: vk::DataGraphTOSALevelARM = vk::DataGraphTOSALevelARM::_8K;

unsafe extern "system" fn get_engine_operation_properties(
    physical_device: vk::PhysicalDevice,
    _queue_family_index: u32,
    _queue_family_data_graph_properties: *const vk::QueueFamilyDataGraphPropertiesARM<'_>,
    properties: *mut vk::BaseOutStructure<'_>,
) -> VkResult {
    let driver = unsafe { &*physical_device.0.cast::<Driver>() };
    driver.passed.set(properties as usize);
    let s_type = unsafe { (*properties).s_type };
    driver.passed_type.set(s_type);
    if s_type != vk::StructureType::QUEUE_FAMILY_DATA_GRAPH_TOSA_PROPERTIES_ARM {
        return VkResult::ERROR_UNKNOWN;
    }
    let tosa = properties.cast::<vk::QueueFamilyDataGraphTOSAPropertiesARM<'_>>();
    unsafe { (*tosa).level = LEVEL };
    VkResult::SUCCESS
}

unsafe extern "system" fn get_instance_proc_addr(
    _instance: vk::Instance,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    match unsafe { CStr::from_ptr(name) }.to_bytes() {
        b"vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM" => Some(unsafe {
            std::mem::transmute::<GetEngineOperationProperties, unsafe extern "system" fn()>(
                get_engine_operation_properties,
            )
        }),
        _ => None,
    }
}

#[test]
fn the_driver_writes_the_whole_structure_that_the_engine_operation_query_is_given() {
    let driver = Driver::default();
    let handle = vk::Instance(std::ptr::null_mut());
    let fp = unsafe { vk::InstanceFn::load(Some(get_instance_proc_addr), handle) };
    let instance = Instance::from_raw(handle, fp);
    let physical_device = vk::PhysicalDevice(std::ptr::from_ref(&driver).cast_mut().cast());

    assert!(
        size_of::<vk::QueueFamilyDataGraphTOSAPropertiesARM<'_>>()
            > size_of::<vk::BaseOutStructure<'_>>(),
        "the structure that the command writes is larger than VkBaseOutStructure"
    );
    let mut tosa = vk::QueueFamilyDataGraphTOSAPropertiesARM::default();
    let address = std::ptr::from_ref(&tosa) as usize;
    let result = unsafe {
        instance.get_physical_device_queue_family_data_graph_engine_operation_properties_arm(
            physical_device,
            0,
            &vk::QueueFamilyDataGraphPropertiesARM::default(),
            &mut tosa,
        )
    };
    assert_eq!(result, Ok(()));
    assert_eq!(
        driver.passed.get(),
        address,
        "the driver is given the address of the structure"
    );
    assert!(
        driver.passed_type.get() == vk::StructureType::QUEUE_FAMILY_DATA_GRAPH_TOSA_PROPERTIES_ARM,
        "the driver reads the sType of the structure"
    );
    assert!(
        tosa.level == LEVEL,
        "what the driver wrote after sType and pNext is in the structure"
    );
}
