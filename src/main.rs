extern crate redis_cli;

use redis_cli::RedisClient;

fn main() {
    let mut cli = RedisClient::new("127.0.0.1:6379");
    let res = cli.ping();
    println!("{:?}", res);
    cli.set("xcc", "100000000000");
    let d = cli.get("xcc");
    println!("{:?}", d);
    // let h = thread::spawn(move || {
    //     let mut data = [0 as u8; 6]; // using 6 byte buffer
    //     s.read(&mut data).unwrap();
    //     println!("{:?}", data)
    // });
    // h.join().unwrap();

    // let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    // for s in listener.incoming() {
    //     let mut s = s.expect("failed");
    //     let mut buf = [0; 20];
    //     s.read(&mut buf).unwrap();
    //     println!("{:?}", String::from_utf8_lossy(&mut buf))
    // }
}