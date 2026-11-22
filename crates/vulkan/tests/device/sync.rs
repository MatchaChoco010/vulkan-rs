use crate::gpu::Gpu;
use vulkan::vk;

#[test]
fn timeline_semaphore_waits_report_timeout_as_success_code() {
    let Some(gpu) = Gpu::new(&[]) else { return };
    unsafe {
        let device = &gpu.device;
        let mut timeline = vk::SemaphoreTypeCreateInfo::default()
            .semaphore_type(vk::SemaphoreType::TIMELINE)
            .initial_value(1);
        let semaphore = device
            .create_semaphore(
                &vk::SemaphoreCreateInfo::default().push_next(&mut timeline),
                None,
            )
            .unwrap();
        assert_eq!(device.get_semaphore_counter_value(semaphore), Ok(1));

        device
            .signal_semaphore(
                &vk::SemaphoreSignalInfo::default()
                    .semaphore(semaphore)
                    .value(5),
            )
            .unwrap();
        assert_eq!(device.get_semaphore_counter_value(semaphore), Ok(5));

        let semaphores = [semaphore];
        let reached = [5];
        let wait = vk::SemaphoreWaitInfo::default()
            .semaphores(&semaphores)
            .values(&reached);
        assert_eq!(device.wait_semaphores(&wait, 0), Ok(vk::VkResult::SUCCESS));

        let not_reached = [6];
        let wait = vk::SemaphoreWaitInfo::default()
            .semaphores(&semaphores)
            .values(&not_reached);
        assert_eq!(device.wait_semaphores(&wait, 0), Ok(vk::VkResult::TIMEOUT));

        device.destroy_semaphore(semaphore, None);
    }
}

#[test]
fn fence_round_trips_through_a_sync_file() {
    let Some(gpu) = Gpu::new(&[vk::KHR_EXTERNAL_FENCE_FD_NAME]) else {
        return;
    };
    if !gpu.has(vk::KHR_EXTERNAL_FENCE_FD_NAME) {
        eprintln!("skipping: VK_KHR_external_fence_fd is not supported");
        return;
    }
    unsafe {
        let device = &gpu.device;
        let handle_type = vk::ExternalFenceHandleTypeFlagBits::SYNC_FD;

        let mut properties = vk::ExternalFenceProperties::default();
        gpu.instance.get_physical_device_external_fence_properties(
            gpu.physical_device,
            &vk::PhysicalDeviceExternalFenceInfo::default().handle_type(handle_type),
            &mut properties,
        );
        if !properties.external_fence_features.contains(
            vk::ExternalFenceFeatureFlags::EXPORTABLE | vk::ExternalFenceFeatureFlags::IMPORTABLE,
        ) {
            eprintln!("skipping: sync fd fences are not exportable and importable");
            return;
        }

        let mut export = vk::ExportFenceCreateInfo::default().handle_types(handle_type.into());
        let source = device
            .create_fence(&vk::FenceCreateInfo::default().push_next(&mut export), None)
            .unwrap();

        device.queue_submit2(gpu.queue, &[], source).unwrap();
        let fd = device
            .get_fence_fd_khr(
                &vk::FenceGetFdInfoKHR::default()
                    .fence(source)
                    .handle_type(handle_type),
            )
            .unwrap();
        assert!(fd >= -1);

        let target = device
            .create_fence(&vk::FenceCreateInfo::default(), None)
            .unwrap();
        device
            .import_fence_fd_khr(
                &vk::ImportFenceFdInfoKHR::default()
                    .fence(target)
                    .flags(vk::FenceImportFlags::TEMPORARY)
                    .handle_type(handle_type)
                    .fd(fd),
            )
            .unwrap();
        assert_eq!(
            device.wait_for_fences(&[target], true, u64::MAX),
            Ok(vk::VkResult::SUCCESS)
        );

        device.destroy_fence(target, None);
        device.destroy_fence(source, None);
    }
}

#[test]
fn semaphore_round_trips_through_a_sync_file() {
    let Some(gpu) = Gpu::new(&[vk::KHR_EXTERNAL_SEMAPHORE_FD_NAME]) else {
        return;
    };
    if !gpu.has(vk::KHR_EXTERNAL_SEMAPHORE_FD_NAME) {
        eprintln!("skipping: VK_KHR_external_semaphore_fd is not supported");
        return;
    }
    unsafe {
        let device = &gpu.device;
        let handle_type = vk::ExternalSemaphoreHandleTypeFlagBits::SYNC_FD;

        let mut properties = vk::ExternalSemaphoreProperties::default();
        gpu.instance
            .get_physical_device_external_semaphore_properties(
                gpu.physical_device,
                &vk::PhysicalDeviceExternalSemaphoreInfo::default().handle_type(handle_type),
                &mut properties,
            );
        if !properties.external_semaphore_features.contains(
            vk::ExternalSemaphoreFeatureFlags::EXPORTABLE
                | vk::ExternalSemaphoreFeatureFlags::IMPORTABLE,
        ) {
            eprintln!("skipping: sync fd semaphores are not exportable and importable");
            return;
        }

        let mut export = vk::ExportSemaphoreCreateInfo::default().handle_types(handle_type.into());
        let source = device
            .create_semaphore(
                &vk::SemaphoreCreateInfo::default().push_next(&mut export),
                None,
            )
            .unwrap();

        let signals = [vk::SemaphoreSubmitInfo::default().semaphore(source)];
        let submits = [vk::SubmitInfo2::default().signal_semaphore_infos(&signals)];
        device
            .queue_submit2(gpu.queue, &submits, vk::Fence::null())
            .unwrap();
        let fd = device
            .get_semaphore_fd_khr(
                &vk::SemaphoreGetFdInfoKHR::default()
                    .semaphore(source)
                    .handle_type(handle_type),
            )
            .unwrap();
        assert!(fd >= -1);

        let target = device
            .create_semaphore(&vk::SemaphoreCreateInfo::default(), None)
            .unwrap();
        device
            .import_semaphore_fd_khr(
                &vk::ImportSemaphoreFdInfoKHR::default()
                    .semaphore(target)
                    .flags(vk::SemaphoreImportFlags::TEMPORARY)
                    .handle_type(handle_type)
                    .fd(fd),
            )
            .unwrap();

        let fence = device
            .create_fence(&vk::FenceCreateInfo::default(), None)
            .unwrap();
        let waits = [vk::SemaphoreSubmitInfo::default()
            .semaphore(target)
            .stage_mask(vk::PipelineStageFlags2::ALL_COMMANDS)];
        let submits = [vk::SubmitInfo2::default().wait_semaphore_infos(&waits)];
        device.queue_submit2(gpu.queue, &submits, fence).unwrap();
        assert_eq!(
            device.wait_for_fences(&[fence], true, u64::MAX),
            Ok(vk::VkResult::SUCCESS)
        );

        device.destroy_fence(fence, None);
        device.destroy_semaphore(target, None);
        device.destroy_semaphore(source, None);
    }
}
