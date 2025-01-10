use std::fs::File;
use std::io::{self, BufRead};

/// Simple wrapper function that read two lines from `auth.txt` and return.
/// You need to fill `auth.txt` with your own AUT and SES values from browser cookies
/// in order to run some examples.
pub fn get_aut_ses_from_env() -> (String, String) {
    let file = File::open("auth.txt").unwrap();
    let reader = io::BufReader::new(file);

    // Collect the first two lines.
    let mut lines = reader.lines();
    let aut = lines.next().unwrap().unwrap();
    let ses = lines.next().unwrap().unwrap();

    (aut, ses)
}
