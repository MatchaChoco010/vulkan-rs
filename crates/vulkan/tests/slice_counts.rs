use vulkan::vk;

#[test]
fn a_slice_setter_stores_every_length_that_its_u8_count_holds() {
    let offsets = [0i32; 256];
    for len in 0..=255 {
        let sps =
            vk::StdVideoH264SequenceParameterSet::default().offset_for_ref_frame(&offsets[..len]);
        assert_eq!(
            usize::from(sps.num_ref_frames_in_pic_order_cnt_cycle),
            len,
            "the count is the length of the slice"
        );
        if len == 0 {
            assert!(sps.p_offset_for_ref_frame.is_null());
        } else {
            assert_eq!(sps.p_offset_for_ref_frame, offsets.as_ptr());
        }
    }
}

#[test]
#[should_panic(
    expected = "StdVideoH264SequenceParameterSet::pOffsetForRefFrame: the slice has 256 elements, more than `num_ref_frames_in_pic_order_cnt_cycle`, a u8, can count"
)]
fn a_slice_setter_panics_for_a_slice_longer_than_its_u8_count_holds() {
    let offsets = [0i32; 256];
    let sps = vk::StdVideoH264SequenceParameterSet::default().offset_for_ref_frame(&offsets);
    assert_eq!(
        usize::from(sps.num_ref_frames_in_pic_order_cnt_cycle),
        offsets.len(),
        "the count does not hold the length of the slice"
    );
}
