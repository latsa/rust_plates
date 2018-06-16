
extern "C" {
    fn square(val: i32) -> i32;
}

pub fn square2(val: i32)  -> i32 {
   unsafe {
      square(val)
   }
}


