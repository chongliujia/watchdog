use sysinfo::{System, SystemExt, ProcessExt, DiskExt, NetworksExt, NetworksExt};


fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    /*

    println!("Total memory: {} KB", system.total_memory());
    println!("Used memory: {} KB", system.used_memory());

    */

    println!("=> disk: ");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }

    println!("=> networks: ");
    for (interface_name, data) in sys.networks() {
        println!("{}: {}/{} B", interface_name, data.received(), data.transmitted());
    }

    println!("=> components: ");
    for component in sys.components() {
        println!("{:?}", component);
    }

    println!("=> system: ");

    println!("total memory: {} bytes", sys.total_memory());
    println!("used memory:  {} bytes", sys.used_memory());
    println!("total swap:   {} bytes", sys.total_swap())



}
