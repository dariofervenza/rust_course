mod find_and_replace;
// in that module, only run function is public, which is the one we need to call

fn main() {
    find_and_replace::run()

    //  run with   cargo run text penis cargo.toml copy.toml

    // trigger error with  cargo run [] penis cargo.toml copy.toml
}


