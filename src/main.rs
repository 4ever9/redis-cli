extern crate redis_cli;

use redis_cli::RedisClient;
use shrust::{Shell, ShellIO};

fn main() {
    let cli = RedisClient::new("127.0.0.1:6379");

    let mut shell = Shell::new(cli);
    shell.new_command_noargs("ping", "Redis ping command", |_, cli| {
        let res = cli.ping();
        println!("{:?}", res);
        Ok(())
    });

    shell.new_command("set", "Redis set command", 2, |_, cli, s| {
        cli.set(s[0], s[1]);
        Ok(())
    });

    shell.new_command("get", "Redis get command", 1, |_, cli, s| {
        let res = cli.get(s[0]);
        println!("{:?}", res);
        Ok(())
    });

    shell.run_loop(&mut ShellIO::default());
}