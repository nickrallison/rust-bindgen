// bindgen-flags: \-\-rust-target=1.33

union Bar {
  unsigned char foo;
} __attribute__ ((__aligned__ (16)));


union Baz {
  union Bar bar;
};
