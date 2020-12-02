use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::io::{Seek, SeekFrom};

fn read_string() {
    let curs = Cursor::new(
        b"\
diff --git a/example.rs b/example.rs
index f38589a..0f1bb83 100644
--- a/example.rs
+++ b/example.rs
@@ -1,5 +1,5 @@
-// Output the square of a number.
-fn print_square(num: f64) {
-    let result = f64::powf(num, 2.0);
-    println!(\"The square of {:.2} is {:.2}.\", num, result);
+// Output the cube of a number.
+fn print_cube(num: f64) {
+    let result = f64::powf(num, 3.0);
+    println!(\"The cube of {:.2} is {:.2}.\", num, result);
 }"
        .to_vec(),
    );
    println!("{:?}", curs);
}

fn read_to_string() {
    let mut curs = Cursor::new(vec![0; 1]);
    write!(curs, "a").unwrap();
    dbg!(curs.get_ref());
    let mut s = String::new();
    curs.seek(SeekFrom::Start(0)).unwrap();
    curs.read_to_string(&mut s).unwrap();
    dbg!(s);
}

pub fn main() {
    read_string();
    read_to_string()
}
