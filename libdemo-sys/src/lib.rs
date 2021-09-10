#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern "C" {
  fn test_alloc();
}

pub unsafe fn test() {
  test_alloc();
}
