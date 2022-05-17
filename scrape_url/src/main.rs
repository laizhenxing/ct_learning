use std::fs;

fn main() {
    let is_pi = pi();
    let not_pi1 = not_pi();
    let not_pi2 = {
        not_pi();
    };

    println!("is_pi: {:?}", is_pi);
    println!("not_pi1: {:?}", not_pi1);
    println!("not_pi2: {:?}", not_pi2)
}

fn pi() -> f64 {
    3.1415926
}

fn not_pi() {
    3.1415926;
}

fn html2Md() {
    let url = "https://www.rust-lang.org/";
    let output = "output.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("markdown content has been writed in {}", output);
}
