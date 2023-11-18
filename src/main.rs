use std::fs::File;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");
    let mut port = serialport::new("/dev/ttyUSB0", 115200).open().unwrap();

    // std::thread::sleep(Duration::from_secs(5));

    // let mut bigbuf = Vec::with_capacity(512 * 16);

    let mut bigbuf = Vec::with_capacity(192);

    for i in 43..=6562 {
        let path = format!("assets/48x32/opt-bad_apple_{:03}.bin", i);
        println!("{}", path);
        let r = read_file(&path);
        if r.is_some() {
            bigbuf.extend(r.unwrap());
        }
        if bigbuf.len() == 192 {
            println!("{}", bigbuf.len());
            let mut waiter = [0; 1];
            while waiter[0] != 0b11001100 {
                port.read_exact(&mut waiter).ok();
            }

            port.write_all(&bigbuf).unwrap();
            bigbuf.clear();
        }
    }
}

fn read_file(filename: &str) -> Option<Vec<u8>> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => return None,
    };

    use std::io::Read;
    let mut br = BufReader::new(file);
    // let mut buffer = Vec::with_capacity(192);
    let mut buffer = Vec::with_capacity(512);
    br.read_to_end(&mut buffer).unwrap();

    Some(buffer)
}
