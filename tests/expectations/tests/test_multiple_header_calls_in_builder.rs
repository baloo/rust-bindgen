/* automatically generated by rust-bindgen */

extern "C" {
    #[link_name = "\u{1}foo"]
    pub static mut foo:
        ::std::option::Option<
        unsafe extern "C" fn(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int)
            -> ::std::os::raw::c_int,
    >;
}
pub type Char = ::std::os::raw::c_char;
pub type SChar = ::std::os::raw::c_schar;
pub type UChar = ::std::os::raw::c_uchar;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Test {
    pub ch: ::std::os::raw::c_char,
    pub u: ::std::os::raw::c_uchar,
    pub d: ::std::os::raw::c_schar,
    pub cch: ::std::os::raw::c_char,
    pub cu: ::std::os::raw::c_uchar,
    pub cd: ::std::os::raw::c_schar,
    pub Cch: Char,
    pub Cu: UChar,
    pub Cd: SChar,
    pub Ccch: Char,
    pub Ccu: UChar,
    pub Ccd: SChar,
}
#[test]
fn bindgen_test_layout_Test() {
    assert_eq!(
        ::std::mem::size_of::<Test>(),
        12usize,
        concat!("Size of: ", stringify!(Test))
    );
    assert_eq!(
        ::std::mem::align_of::<Test>(),
        1usize,
        concat!("Alignment of ", stringify!(Test))
    );
    assert_eq!(
        unsafe { &(*(0 as *const Test)).ch as *const _ as usize },
        0usize,
        concat!(
            "Alignment of field: ",
            stringify!(Test),
            "::",
            stringify!(ch)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const Test)).u as *const _ as usize },
        1usize,
        concat!(
            "Alignment of field: ",
            stringify!(Test),
            "::",
            stringify!(u)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const Test)).d as *const _ as usize },
        2usize,
        concat!(
            "Alignment of field: ",
            stringify!(Test),
            "::",
            stringify!(d)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const Test)).cch as *const _ as usize },
        3usize,
        concat!(
            "Alignment of field: ",
            stringify!(Test),
            "::",
            stringify!(cch)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const Test)).cu as *const _ as usize },
        4usize,
        concat!(
            "Alignment of field: ",
            stringify!(Test),
            "::",
            stringify!(cu)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const Test)).cd as *const _ as usize },
        5usize,
        concat!(
            "Alignment of field: ",
            stringify!(Test),
            "::",
            stringify!(cd)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const Test)).Cch as *const _ as usize },
        6usize,
        concat!(
            "Alignment of field: ",
            stringify!(Test),
            "::",
            stringify!(Cch)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const Test)).Cu as *const _ as usize },
        7usize,
        concat!(
            "Alignment of field: ",
            stringify!(Test),
            "::",
            stringify!(Cu)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const Test)).Cd as *const _ as usize },
        8usize,
        concat!(
            "Alignment of field: ",
            stringify!(Test),
            "::",
            stringify!(Cd)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const Test)).Ccch as *const _ as usize },
        9usize,
        concat!(
            "Alignment of field: ",
            stringify!(Test),
            "::",
            stringify!(Ccch)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const Test)).Ccu as *const _ as usize },
        10usize,
        concat!(
            "Alignment of field: ",
            stringify!(Test),
            "::",
            stringify!(Ccu)
        )
    );
    assert_eq!(
        unsafe { &(*(0 as *const Test)).Ccd as *const _ as usize },
        11usize,
        concat!(
            "Alignment of field: ",
            stringify!(Test),
            "::",
            stringify!(Ccd)
        )
    );
}
