use dictionary2::DICTIONARY;
use rand::seq::SliceRandom;

fn main() {
    webbrowser::open(&format!(
        "https://web.roblox.com/discover/?Keyword={}",
        DICTIONARY.choose(&mut rand::thread_rng()).unwrap()
    ))
    .unwrap();
}
