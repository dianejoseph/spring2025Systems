fn track changes(){
    let mut tracker = 0;
    let mut update = || {
        tracker += 1;
        println!("Tracker: {}", tracker);
    };

    update(); // tracker: 1
    update(); // tracker: 2
}

fn main(){
    track_changes();
}