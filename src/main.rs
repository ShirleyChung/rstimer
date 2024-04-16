use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Instant, Duration};

fn main() {
    // 宣告一個"通道", 以便在不同緒之間傳遞資料
    let (tx, rx) = mpsc::channel();
    // 流逝的時間
    let mut elapsed_time = Duration::new(0, 0);
    // 起始的時間
    let mut start_time = Instant::now();
    // 執行中
    let mut is_running = false;

    println!("輸入s起動碼錶; 輸入p暫停; 輸入q結束");
    // 取得輸入值的緒
    thread::spawn(move || {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            tx.send(input.trim().to_string()).expect("Failed to send message");
        }
    });

    // 迴圈，持續地顯示時間
    loop {
        let interval = Duration::from_millis(100);
        thread::sleep(interval);

        // 判斷從輸入緒取得的值
        match rx.try_recv() {
            Ok(command) => match command.as_str() {
                "s" => {
                    if !is_running {
                        is_running = true;
                        start_time = Instant::now();
                        println!("Timer started.");
                    } else {
                        println!("Timer is already running.");
                    }
                }
                "p" => {
                    if is_running {
                        is_running = false;
                        elapsed_time += start_time.elapsed();
                        println!("Timer paused.");
                    } else {
                        println!("Timer is not running.");
                    }
                }
                "q" => {
                    if is_running {
                        elapsed_time += start_time.elapsed();
                    }
                    println!("Timer stopped. Elapsed time: {:?}", elapsed_time);
                    break;
                }
                _ => println!("Invalid input. Please enter 's' to start, 'p' to pause, or 'q' to quit."),
            },
            Err(_) => {}
        }

        if is_running {
            let elapsed = elapsed_time + start_time.elapsed();
            println!("Elapsed time: {:?}", elapsed);
        }
    }
}
