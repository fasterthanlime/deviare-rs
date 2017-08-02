
fn main() {
  println!("cargo:rustc-link-lib=static=NktHookLib64");
  println!(
    "cargo:rustc-link-search=native=C:\\msys64\\home\\amwenger\\Dev\\capsule\\build64\\deps\\lib"
  );
}
