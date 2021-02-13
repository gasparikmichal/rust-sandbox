pub fn run() {
    // Print to console
    println!("Hello from the print.rs");

    // Basic Formatting
    println!("{} is from {}", "Michal", "Bratislava");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}",
             "Michal", "Bratislava", "code");

    // Named Arguments
    println!("{name} likes to play {activity}",
             name = "Michal", activity = "AirSoft");
}
