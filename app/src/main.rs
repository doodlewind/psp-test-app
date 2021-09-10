#![no_std]
#![no_main]

use demolib_sys::test;
psp::module!("test_app_module", 1, 1);

#[no_mangle]
pub extern "C" fn my_print(a: i32) {
  psp::dprintln!("print {}", a);
}

fn psp_main() {
  unsafe {
    psp::enable_home_button();
    psp::dprintln!("Test app started!");
    test();
    psp::dprintln!("C function called!");
  }
}
