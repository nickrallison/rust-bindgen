#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct C {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(
        ::std::mem::size_of::<C>(),
        40usize,
        concat!("Size of: ", stringify!(C))
    );
    assert_eq!(
        ::std::mem::align_of::<C>(),
        4usize,
        concat!("Alignment of ", stringify!(C))
    );
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<C>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(C), "::", stringify!(a))
        );
    }
    test_field_a();
    fn test_field_big_array() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<C>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).big_array) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(C),
                "::",
                stringify!(big_array)
            )
        );
    }
    test_field_big_array();
}
impl Default for C {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
pub struct C_with_zero_length_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_C_with_zero_length_array() {
    assert_eq!(
        ::std::mem::size_of::<C_with_zero_length_array>(),
        40usize,
        concat!("Size of: ", stringify!(C_with_zero_length_array))
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_zero_length_array>(),
        4usize,
        concat!("Alignment of ", stringify!(C_with_zero_length_array))
    );
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<C_with_zero_length_array>::uninit(
                    );
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(C_with_zero_length_array),
                "::",
                stringify!(a)
            )
        );
    }
    test_field_a();
    fn test_field_big_array() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<C_with_zero_length_array>::uninit(
                    );
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).big_array) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(C_with_zero_length_array),
                "::",
                stringify!(big_array)
            )
        );
    }
    test_field_big_array();
    fn test_field_zero_length_array() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<C_with_zero_length_array>::uninit(
                    );
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).zero_length_array) as usize -
                    ptr as usize
            },
            37usize,
            concat!(
                "Offset of field: ",
                stringify!(C_with_zero_length_array),
                "::",
                stringify!(zero_length_array)
            )
        );
    }
    test_field_zero_length_array();
}
impl Default for C_with_zero_length_array {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct C_with_zero_length_array_2 {
    pub a: ::std::os::raw::c_int,
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_C_with_zero_length_array_2() {
    assert_eq!(
        ::std::mem::size_of::<C_with_zero_length_array_2>(),
        4usize,
        concat!("Size of: ", stringify!(C_with_zero_length_array_2))
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_zero_length_array_2>(),
        4usize,
        concat!("Alignment of ", stringify!(C_with_zero_length_array_2))
    );
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    C_with_zero_length_array_2,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(C_with_zero_length_array_2),
                "::",
                stringify!(a)
            )
        );
    }
    test_field_a();
    fn test_field_zero_length_array() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    C_with_zero_length_array_2,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).zero_length_array) as usize -
                    ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(C_with_zero_length_array_2),
                "::",
                stringify!(zero_length_array)
            )
        );
    }
    test_field_zero_length_array();
}
#[repr(C)]
pub struct C_with_incomplete_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_C_with_incomplete_array() {
    assert_eq!(
        ::std::mem::size_of::<C_with_incomplete_array>(),
        40usize,
        concat!("Size of: ", stringify!(C_with_incomplete_array))
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_incomplete_array>(),
        4usize,
        concat!("Alignment of ", stringify!(C_with_incomplete_array))
    );
}
impl Default for C_with_incomplete_array {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct C_with_incomplete_array_2 {
    pub a: ::std::os::raw::c_int,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_C_with_incomplete_array_2() {
    assert_eq!(
        ::std::mem::size_of::<C_with_incomplete_array_2>(),
        4usize,
        concat!("Size of: ", stringify!(C_with_incomplete_array_2))
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_incomplete_array_2>(),
        4usize,
        concat!("Alignment of ", stringify!(C_with_incomplete_array_2))
    );
}
#[repr(C)]
pub struct C_with_zero_length_array_and_incomplete_array {
    pub a: ::std::os::raw::c_int,
    pub big_array: [::std::os::raw::c_char; 33usize],
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_C_with_zero_length_array_and_incomplete_array() {
    assert_eq!(
        ::std::mem::size_of::<C_with_zero_length_array_and_incomplete_array>(),
        40usize,
        concat!(
            "Size of: ",
            stringify!(C_with_zero_length_array_and_incomplete_array)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_zero_length_array_and_incomplete_array>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(C_with_zero_length_array_and_incomplete_array)
        )
    );
}
impl Default for C_with_zero_length_array_and_incomplete_array {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct C_with_zero_length_array_and_incomplete_array_2 {
    pub a: ::std::os::raw::c_int,
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_C_with_zero_length_array_and_incomplete_array_2() {
    assert_eq!(
        ::std::mem::size_of::<C_with_zero_length_array_and_incomplete_array_2>(
        ),
        4usize,
        concat!(
            "Size of: ",
            stringify!(C_with_zero_length_array_and_incomplete_array_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<C_with_zero_length_array_and_incomplete_array_2>(
        ),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(C_with_zero_length_array_and_incomplete_array_2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct WithDtor {
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_WithDtor() {
    assert_eq!(
        ::std::mem::size_of::<WithDtor>(),
        4usize,
        concat!("Size of: ", stringify!(WithDtor))
    );
    assert_eq!(
        ::std::mem::align_of::<WithDtor>(),
        4usize,
        concat!("Alignment of ", stringify!(WithDtor))
    );
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WithDtor>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(WithDtor),
                "::",
                stringify!(b)
            )
        );
    }
    test_field_b();
}
#[repr(C)]
pub struct IncompleteArrayNonCopiable {
    pub whatever: *mut ::std::os::raw::c_void,
    pub incomplete_array: __IncompleteArrayField<C>,
}
#[test]
fn bindgen_test_layout_IncompleteArrayNonCopiable() {
    assert_eq!(
        ::std::mem::size_of::<IncompleteArrayNonCopiable>(),
        8usize,
        concat!("Size of: ", stringify!(IncompleteArrayNonCopiable))
    );
    assert_eq!(
        ::std::mem::align_of::<IncompleteArrayNonCopiable>(),
        8usize,
        concat!("Alignment of ", stringify!(IncompleteArrayNonCopiable))
    );
}
impl Default for IncompleteArrayNonCopiable {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Union {
    pub d: f32,
    pub i: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Union() {
    assert_eq!(
        ::std::mem::size_of::<Union>(),
        4usize,
        concat!("Size of: ", stringify!(Union))
    );
    assert_eq!(
        ::std::mem::align_of::<Union>(),
        4usize,
        concat!("Alignment of ", stringify!(Union))
    );
    fn test_field_d() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Union>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).d) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Union),
                "::",
                stringify!(d)
            )
        );
    }
    test_field_d();
    fn test_field_i() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Union>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).i) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Union),
                "::",
                stringify!(i)
            )
        );
    }
    test_field_i();
}
impl Default for Union {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WithUnion {
    pub data: Union,
}
#[test]
fn bindgen_test_layout_WithUnion() {
    assert_eq!(
        ::std::mem::size_of::<WithUnion>(),
        4usize,
        concat!("Size of: ", stringify!(WithUnion))
    );
    assert_eq!(
        ::std::mem::align_of::<WithUnion>(),
        4usize,
        concat!("Alignment of ", stringify!(WithUnion))
    );
    fn test_field_data() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<WithUnion>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(WithUnion),
                "::",
                stringify!(data)
            )
        );
    }
    test_field_data();
}
impl Default for WithUnion {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct RealAbstractionWithTonsOfMethods {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_RealAbstractionWithTonsOfMethods() {
    assert_eq!(
        ::std::mem::size_of::<RealAbstractionWithTonsOfMethods>(),
        1usize,
        concat!("Size of: ", stringify!(RealAbstractionWithTonsOfMethods))
    );
    assert_eq!(
        ::std::mem::align_of::<RealAbstractionWithTonsOfMethods>(),
        1usize,
        concat!(
            "Alignment of ",
            stringify!(RealAbstractionWithTonsOfMethods)
        )
    );
}
extern "C" {
    #[link_name = "\u{1}_ZNK32RealAbstractionWithTonsOfMethods3barEv"]
    pub fn RealAbstractionWithTonsOfMethods_bar(
        this: *const RealAbstractionWithTonsOfMethods,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN32RealAbstractionWithTonsOfMethods3barEv"]
    pub fn RealAbstractionWithTonsOfMethods_bar1(
        this: *mut RealAbstractionWithTonsOfMethods,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN32RealAbstractionWithTonsOfMethods3barEi"]
    pub fn RealAbstractionWithTonsOfMethods_bar2(
        this: *mut RealAbstractionWithTonsOfMethods,
        foo: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}_ZN32RealAbstractionWithTonsOfMethods3staEv"]
    pub fn RealAbstractionWithTonsOfMethods_sta();
}
impl RealAbstractionWithTonsOfMethods {
    #[inline]
    pub unsafe fn bar(&self) {
        RealAbstractionWithTonsOfMethods_bar(self)
    }
    #[inline]
    pub unsafe fn bar1(&mut self) {
        RealAbstractionWithTonsOfMethods_bar1(self)
    }
    #[inline]
    pub unsafe fn bar2(&mut self, foo: ::std::os::raw::c_int) {
        RealAbstractionWithTonsOfMethods_bar2(self, foo)
    }
    #[inline]
    pub unsafe fn sta() {
        RealAbstractionWithTonsOfMethods_sta()
    }
}
