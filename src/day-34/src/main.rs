fn main() {
    let mut packets: Vec<u32> = Vec::new();
    packets.push(64);
    packets.push(64);
    packets.push(64);
    println!("{:?}", packets);

    match packets.get(1) {
        Some(size) => println!("Packet 2 is {size} bytes"),
        None => println!("No packet there"),
    }

    let mut total: u32 = 0;
    for size in &packets {
        total += size;
    }
    println!("Total received: {total} bytes");

    for size in &mut packets {
        *size = (*size).min(1500);
    }
}
