extern crate uptime_lib;

fn main() {
    let minute: f64 = 60.0;
    let day: f64 = 24.0;
    match uptime_lib::get() {
        Ok(uptime) => {
            print!("Uptime: {} days,", (uptime.as_secs_f64() / minute / minute / day).floor());
            print!(" {} hours,", ((uptime.as_secs_f64() / minute / minute) - ((uptime.as_secs_f64() / minute / minute / day).floor() * day)).floor());
            print!(" {} minutes,", ((uptime.as_secs_f64() / minute) - ((uptime.as_secs_f64() / minute / minute).floor() * minute)).floor());
            println!(" {} seconds", ((uptime.as_secs_f64()) - ((uptime.as_secs_f64() / minute).floor() * minute)).floor());
        }
        Err(err) => {
            eprintln!("uptime: {}", err);
            std::process::exit(1);
        }
    }
}
