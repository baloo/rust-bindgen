/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Copy, Clone)]
pub struct Base {
    pub large: [::std::os::raw::c_int; 33usize],
}
#[test]
fn bindgen_test_layout_Base() {
    assert_eq!(
        ::std::mem::size_of::<Base>(),
        132usize,
        concat!("Size of: ", stringify!(Base))
    );
    assert_eq!(
        ::std::mem::align_of::<Base>(),
        4usize,
        concat!("Alignment of ", stringify!(Base))
    );
    assert_eq!(
        unsafe { &(*(0 as *const Base)).large as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(Base),
            "::",
            stringify!(large)
        )
    );
}
impl Default for Base {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for Base {
    fn eq(&self, other: &Base) -> bool {
        &self.large[..] == &other.large[..]
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShouldDerivePartialEq {
    pub _base: Base,
}
#[test]
fn bindgen_test_layout_ShouldDerivePartialEq() {
    assert_eq!(
        ::std::mem::size_of::<ShouldDerivePartialEq>(),
        132usize,
        concat!("Size of: ", stringify!(ShouldDerivePartialEq))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldDerivePartialEq>(),
        4usize,
        concat!("Alignment of ", stringify!(ShouldDerivePartialEq))
    );
}
impl Default for ShouldDerivePartialEq {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for ShouldDerivePartialEq {
    fn eq(&self, other: &ShouldDerivePartialEq) -> bool {
        self._base == other._base
    }
}
