use wgpu::{Backends, Instance, InstanceDescriptor};

fn main() {
    let descriptor = InstanceDescriptor {
        backends: Backends::all(),
        ..Default::default()
    };
    let instances = Instance::new(&descriptor);
    println!("Available adapters:");
    for (i, adapter) in instances
        .enumerate_adapters(Backends::all())
        .iter()
        .enumerate()
    {
        println!("{i}: {:?}", adapter.get_info());
    }
}
