use std::env;
use std::fs::File;
use std::io::Write;
extern crate time;

fn writecalc(cnt: i32, wcnt: i32){
    println!("start: {}KB * {} = {}MB", wcnt*320/1000, cnt, wcnt*320*cnt/1000000);
    let start_at = time::get_time();
    // open
    let mut file = File::create("/tmp/input.txt").unwrap();
    println!(" open done: {}", time::get_time() - start_at);

    for _ in 0..cnt {
        let nstart_at = time::get_time();

        for _ in 0..wcnt {
            file.write_all(b"aaaaaaaaaabbbbbbbbbbccccccccccdddddddddd").unwrap();
        }
        println!("    write done: {}", time::get_time() - nstart_at);

        // fsync
        file.sync_all().unwrap();
        println!("     sync done: {}", time::get_time() - nstart_at);
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let cnt: i32 = args[1].parse().unwrap();
    let wcnt:i32 = args[2].parse().unwrap();

    let start_at = time::get_time();
    writecalc(cnt, wcnt);
    println!("done: {}", time::get_time() - start_at);
}
