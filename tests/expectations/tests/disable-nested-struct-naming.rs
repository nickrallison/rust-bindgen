#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct foo {
    pub b1: bar1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar1 {
    pub x1: ::std::os::raw::c_int,
    pub b2: bar1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar1__bindgen_ty_1 {
    pub x2: ::std::os::raw::c_int,
    pub b3: bar1__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar1__bindgen_ty_1__bindgen_ty_1 {
    pub x3: ::std::os::raw::c_int,
    pub b4: bar4,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct bar4 {
    pub x4: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_bar4() {
    assert_eq!(
        ::std::mem::size_of::<bar4>(),
        4usize,
        concat!("Size of: ", stringify!(bar4))
    );
    assert_eq!(
        ::std::mem::align_of::<bar4>(),
        4usize,
        concat!("Alignment of ", stringify!(bar4))
    );
    fn test_field_x4() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<bar4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).x4) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(bar4),
                "::",
                stringify!(x4)
            )
        );
    }
    test_field_x4();
}
#[test]
fn bindgen_test_layout_bar1__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<bar1__bindgen_ty_1__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(bar1__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<bar1__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(bar1__bindgen_ty_1__bindgen_ty_1)
        )
    );
    fn test_field_x3() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    bar1__bindgen_ty_1__bindgen_ty_1,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).x3) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(bar1__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(x3)
            )
        );
    }
    test_field_x3();
    fn test_field_b4() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    bar1__bindgen_ty_1__bindgen_ty_1,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b4) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(bar1__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(b4)
            )
        );
    }
    test_field_b4();
}
#[test]
fn bindgen_test_layout_bar1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<bar1__bindgen_ty_1>(),
        12usize,
        concat!("Size of: ", stringify!(bar1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<bar1__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(bar1__bindgen_ty_1))
    );
    fn test_field_x2() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<bar1__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).x2) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(bar1__bindgen_ty_1),
                "::",
                stringify!(x2)
            )
        );
    }
    test_field_x2();
    fn test_field_b3() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<bar1__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b3) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(bar1__bindgen_ty_1),
                "::",
                stringify!(b3)
            )
        );
    }
    test_field_b3();
}
#[test]
fn bindgen_test_layout_bar1() {
    assert_eq!(
        ::std::mem::size_of::<bar1>(),
        16usize,
        concat!("Size of: ", stringify!(bar1))
    );
    assert_eq!(
        ::std::mem::align_of::<bar1>(),
        4usize,
        concat!("Alignment of ", stringify!(bar1))
    );
    fn test_field_x1() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<bar1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).x1) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(bar1),
                "::",
                stringify!(x1)
            )
        );
    }
    test_field_x1();
    fn test_field_b2() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<bar1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b2) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(bar1),
                "::",
                stringify!(b2)
            )
        );
    }
    test_field_b2();
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        16usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    fn test_field_b1() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b1) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(foo), "::", stringify!(b1))
        );
    }
    test_field_b1();
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _bindgen_ty_1 {
    pub anon2: _bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct _bindgen_ty_1__bindgen_ty_1 {
    pub b: baz,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct baz {
    pub x: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_baz() {
    assert_eq!(
        ::std::mem::size_of::<baz>(),
        4usize,
        concat!("Size of: ", stringify!(baz))
    );
    assert_eq!(
        ::std::mem::align_of::<baz>(),
        4usize,
        concat!("Alignment of ", stringify!(baz))
    );
    fn test_field_x() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<baz>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(baz), "::", stringify!(x))
        );
    }
    test_field_x();
}
#[test]
fn bindgen_test_layout__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(_bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(_bindgen_ty_1__bindgen_ty_1))
    );
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    _bindgen_ty_1__bindgen_ty_1,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(b)
            )
        );
    }
    test_field_b();
}
#[test]
fn bindgen_test_layout__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(_bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(_bindgen_ty_1))
    );
    fn test_field_anon2() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).anon2) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_bindgen_ty_1),
                "::",
                stringify!(anon2)
            )
        );
    }
    test_field_anon2();
}
extern "C" {
    pub static mut anon1: _bindgen_ty_1;
}
