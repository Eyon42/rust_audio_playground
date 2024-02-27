use autopilot::mouse::location;
use autopilot::screen::size;

fn main() {
    let screen_size = size();
    for i in 0..10000 {
        let mouse_loc = location();
        let nx = mouse_loc.x / screen_size.width;
        let ny = mouse_loc.y / screen_size.height;
        print!("x:{nx:.2} y:{ny:.2}    \r");
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    println!("")
}
