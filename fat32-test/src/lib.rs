#![feature(associated_type_bounds)]
#![allow(unused)]
use crate::device::FakeDevice;
use mfat32::Fat32;
mod device;
mod logging;
mod other_fat32;
mod test1_create_list_cd;
mod test2_read_write;
mod test3_delete;
mod test4_rename;





#[test]
fn intergenerational_test() {
    // create your fat32
    logging::init_logger();
    let device = FakeDevice::new("./test.img");
    let fat32 = Fat32::new(device).unwrap();
    let root = fat32.root_dir();
    // get a directory
    // begin test
    test1_create_list_cd::test1_create_list_cd(root.clone());
    test2_read_write::test2_read_write(root.clone());
    test3_delete::test_delete_file_and_dir(root.clone());
    test4_rename::test_rename(root.clone());
}
