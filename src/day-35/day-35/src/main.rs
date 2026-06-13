fn main() {
    let mut addr = String::new();
    addr.push_str("example.com");
    addr.push(':');
    addr.push_str("8080");
    println!("{addr}");

    match addr.find(':') {
        Some(i) => {
            let host = &addr[..i];
            let port = &addr[i + 1..];
            println!("host = {host}, port = {port}");

            let normalized = format!("{}:{}", host.to_lowercase(), port);
            println!("normalized = {normalized}");
        }
        None => println!("no port given"),
    }

    let addresses = vec!["example.com:8080", "LOCALHOST:443", "no-port-here"];
    for addr in &addresses {
        match addr.find(':') {
            Some(i) => {
                let host = &addr[..i];
                let port = &addr[i + 1..];
                println!("host = {host}, port = {port}");

                let normalized = format!("{}:{}", host.to_lowercase(), port);
                println!("normalized = {normalized}");
            }
            None => println!("no-port-here"),
        }
    }
}
