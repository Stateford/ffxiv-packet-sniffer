extern crate vcpkg;

fn main() {
    vcpkg::find_package("winpcap").unwrap();
}
