#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo {
    pub bar: [foo__bindgen_ty_1; 2usize],
    pub baz: [[[foo__bindgen_ty_2; 4usize]; 3usize]; 2usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1 {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1))
    );
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<foo__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(foo__bindgen_ty_1),
                "::",
                stringify!(a)
            )
        );
    }
    test_field_a();
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<foo__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(foo__bindgen_ty_1),
                "::",
                stringify!(b)
            )
        );
    }
    test_field_b();
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_2 {
    pub a: ::std::os::raw::c_int,
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_2>(),
        8usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_2))
    );
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<foo__bindgen_ty_2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(foo__bindgen_ty_2),
                "::",
                stringify!(a)
            )
        );
    }
    test_field_a();
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<foo__bindgen_ty_2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(foo__bindgen_ty_2),
                "::",
                stringify!(b)
            )
        );
    }
    test_field_b();
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        208usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    fn test_field_bar() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).bar) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(foo),
                "::",
                stringify!(bar)
            )
        );
    }
    test_field_bar();
    fn test_field_baz() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).baz) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(foo),
                "::",
                stringify!(baz)
            )
        );
    }
    test_field_baz();
}
