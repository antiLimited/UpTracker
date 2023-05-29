extern crate uptime_lib;

fn main() {
    // Constants
    let minute: f64 = 60.0;
    let day: f64 = 24.0;

    // Raw uptime
    let utime = uptime_lib::get().expect("Failed to aquire uptime");

    // 64 bit float version uptime
    let uptime = utime.as_secs_f64();

    // Time values
    let hours =
        ((uptime / minute / minute) - ((uptime / minute / minute / day).floor() * day)).floor();
    let mins =
        ((uptime / minute) - ((uptime / minute / minute).floor() * minute)).floor();
    let seconds =
        ((uptime) - ((uptime / minute).floor() * minute)).floor();
    let days =
        (uptime / minute / minute / day).floor();

    // Output time values
    print!("Uptime: {} days,", days);
    print!(" {} hours,", hours);
    print!(" {} minutes,", mins);
    println!(" {} seconds", seconds);
}
