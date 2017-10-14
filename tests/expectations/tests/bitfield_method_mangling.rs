/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct mach_msg_type_descriptor_t {
    pub _bitfield_1: u32,
    pub __bindgen_align: [u32; 0usize],
}
#[test]
fn bindgen_test_layout_mach_msg_type_descriptor_t() {
    assert_eq!(
        ::std::mem::size_of::<mach_msg_type_descriptor_t>(),
        4usize,
        concat!("Size of: ", stringify!(mach_msg_type_descriptor_t))
    );
    assert_eq!(
        ::std::mem::align_of::<mach_msg_type_descriptor_t>(),
        4usize,
        concat!("Alignment of ", stringify!(mach_msg_type_descriptor_t))
    );
}
impl mach_msg_type_descriptor_t {
    #[inline]
    pub fn pad3(&self) -> ::std::os::raw::c_uint {
        let mut unit_field_val: u32 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u32 as *mut u8,
                ::std::mem::size_of::<u32>(),
            )
        };
        let mask = 0xffffff as u32;
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_pad3(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 0xffffff as u32;
        let val = val as u32 as u32;
        let mut unit_field_val: u32 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u32 as *mut u8,
                ::std::mem::size_of::<u32>(),
            )
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &unit_field_val as *const _ as *const u8,
                &mut self._bitfield_1 as *mut _ as *mut u8,
                ::std::mem::size_of::<u32>(),
            );
        }
    }
    #[inline]
    pub fn type_(&self) -> ::std::os::raw::c_uint {
        let mut unit_field_val: u32 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u32 as *mut u8,
                ::std::mem::size_of::<u32>(),
            )
        };
        let mask = 0xff000000 as u32;
        let val = (unit_field_val & mask) >> 24usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_type(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 0xff000000 as u32;
        let val = val as u32 as u32;
        let mut unit_field_val: u32 = unsafe { ::std::mem::uninitialized() };
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &self._bitfield_1 as *const _ as *const u8,
                &mut unit_field_val as *mut u32 as *mut u8,
                ::std::mem::size_of::<u32>(),
            )
        };
        unit_field_val &= !mask;
        unit_field_val |= (val << 24usize) & mask;
        unsafe {
            ::std::ptr::copy_nonoverlapping(
                &unit_field_val as *const _ as *const u8,
                &mut self._bitfield_1 as *mut _ as *mut u8,
                ::std::mem::size_of::<u32>(),
            );
        }
    }
    #[inline]
    pub fn new_bitfield_1(pad3: ::std::os::raw::c_uint, type_: ::std::os::raw::c_uint) -> u32 {
        ((0 | ((pad3 as u32 as u32) << 0usize) & (0xffffff as u32))
            | ((type_ as u32 as u32) << 24usize) & (0xff000000 as u32))
    }
}
