use std::net::TcpStream;
use std::io::{Write, Read};

pub struct RedisClient {
    s: TcpStream
}

impl RedisClient {
    pub fn new(addr: &str) -> RedisClient {
        let s = TcpStream::connect(addr).unwrap();
        RedisClient {
            s
        }
    }

    pub fn ping(&mut self) -> String {
        let mut cmd = Command::new();
        cmd.add_array(1)
            .add_bulk_string("PING");
        self.s.write(cmd.content.as_bytes()).unwrap();
        self.s.flush().unwrap();
        let mut res = [0; 512];
        let n = self.s.read(&mut res).unwrap();
        return String::from_utf8_lossy(&res[0..n]).to_string();
    }

    pub fn set(&mut self, key: &str, value: &str) {
        let mut cmd = Command::new();
        cmd.add_array(3)
            .add_bulk_string("SET")
            .add_bulk_string(key)
            .add_bulk_string(value);
        self.s.write(cmd.content.as_bytes()).unwrap();
        self.s.flush().unwrap();
    }

    pub fn get(&mut self, key: &str) -> String {
        let mut cmd = Command::new();
        cmd.add_array(2)
            .add_bulk_string("GET")
            .add_bulk_string(key);
        self.s.write(cmd.content.as_bytes()).unwrap();
        self.s.flush().unwrap();
        let mut res = [0; 512];
        let n = self.s.read(&mut res).unwrap();
        let ret = parse_result(String::from_utf8_lossy(&res[0..n]).to_string().as_str()).unwrap();
        return ret.to_string();
    }
}

fn parse_result(res: &str) -> Option<String> {
    let arr: Vec<&str> = res.split("\r\n").collect();
    return match &arr[0][0..1] {
        "$" => Some(arr[1].to_string()),
        "+" => Some(arr[1].to_string()),
        "-" => Some(arr[0].to_string()),
        "*" => {
            let len = arr[0][1..].parse::<usize>().unwrap();
            let mut v: Vec<String> = Vec::new();
            for i in 0..len {
                v.push(arr[i + 1].to_string());
            }
            let res = v.join(",");
            Some(res)
        }
        _ => None,
    };
}

struct Command {
    content: String
}

impl Command {
    pub fn new() -> Command {
        Command {
            content: "".to_string()
        }
    }

    fn add_array(&mut self, n: usize) -> &mut Self {
        self.content.push('*');
        self.content.push_str(n.to_string().as_str());
        self.content.push_str("\r\n");
        self
    }

    fn add_bulk_string(&mut self, s: &str) -> &mut Self {
        if s == "" {
            self.content.push_str("$-1\r\n");
            self
        } else {
            self.content.push_str("$");
            self.content.push_str(s.len().to_string().as_str());
            self.content.push_str("\r\n");
            self.content.push_str(s);
            self.content.push_str("\r\n");
            self
        }
    }
}