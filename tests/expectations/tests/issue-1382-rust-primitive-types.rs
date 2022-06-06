#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type i8_ = i8;
pub type u8_ = u8;
pub type i16_ = i16;
pub type u16_ = u16;
pub type i32_ = i32;
pub type u32_ = u32;
pub type i64_ = i64;
pub type u64_ = u64;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Foo {
    pub i8_: ::std::os::raw::c_int,
    pub u8_: ::std::os::raw::c_int,
    pub i16_: ::std::os::raw::c_int,
    pub u16_: ::std::os::raw::c_int,
    pub i32_: ::std::os::raw::c_int,
    pub u32_: ::std::os::raw::c_int,
    pub i64_: ::std::os::raw::c_int,
    pub u64_: ::std::os::raw::c_int,
    pub i128_: ::std::os::raw::c_int,
    pub u128_: ::std::os::raw::c_int,
    pub isize_: ::std::os::raw::c_int,
    pub usize_: ::std::os::raw::c_int,
    pub f32_: ::std::os::raw::c_int,
    pub f64_: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(
        ::std::mem::size_of::<Foo>(),
        56usize,
        concat!("Size of: ", stringify!(Foo))
    );
    assert_eq!(
        ::std::mem::align_of::<Foo>(),
        4usize,
        concat!("Alignment of ", stringify!(Foo))
    );
    fn test_field_i8() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).i8_) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Foo),
                "::",
                stringify!(i8_)
            )
        );
    }
    test_field_i8();
    fn test_field_u8() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).u8_) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(Foo),
                "::",
                stringify!(u8_)
            )
        );
    }
    test_field_u8();
    fn test_field_i16() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).i16_) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(Foo),
                "::",
                stringify!(i16_)
            )
        );
    }
    test_field_i16();
    fn test_field_u16() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).u16_) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(Foo),
                "::",
                stringify!(u16_)
            )
        );
    }
    test_field_u16();
    fn test_field_i32() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).i32_) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(Foo),
                "::",
                stringify!(i32_)
            )
        );
    }
    test_field_i32();
    fn test_field_u32() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).u32_) as usize - ptr as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(Foo),
                "::",
                stringify!(u32_)
            )
        );
    }
    test_field_u32();
    fn test_field_i64() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).i64_) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(Foo),
                "::",
                stringify!(i64_)
            )
        );
    }
    test_field_i64();
    fn test_field_u64() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).u64_) as usize - ptr as usize
            },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(Foo),
                "::",
                stringify!(u64_)
            )
        );
    }
    test_field_u64();
    fn test_field_i128() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).i128_) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(Foo),
                "::",
                stringify!(i128_)
            )
        );
    }
    test_field_i128();
    fn test_field_u128() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).u128_) as usize - ptr as usize
            },
            36usize,
            concat!(
                "Offset of field: ",
                stringify!(Foo),
                "::",
                stringify!(u128_)
            )
        );
    }
    test_field_u128();
    fn test_field_isize() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).isize_) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(Foo),
                "::",
                stringify!(isize_)
            )
        );
    }
    test_field_isize();
    fn test_field_usize() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).usize_) as usize - ptr as usize
            },
            44usize,
            concat!(
                "Offset of field: ",
                stringify!(Foo),
                "::",
                stringify!(usize_)
            )
        );
    }
    test_field_usize();
    fn test_field_f32() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).f32_) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(Foo),
                "::",
                stringify!(f32_)
            )
        );
    }
    test_field_f32();
    fn test_field_f64() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).f64_) as usize - ptr as usize
            },
            52usize,
            concat!(
                "Offset of field: ",
                stringify!(Foo),
                "::",
                stringify!(f64_)
            )
        );
    }
    test_field_f64();
}
