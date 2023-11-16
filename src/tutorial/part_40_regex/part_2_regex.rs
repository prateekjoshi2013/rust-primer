use regex::Regex;

pub fn main() {
    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "2012-03-14, 2013-01-01, 2014-07-05";
    for cap in re.captures_iter(text) {
        println!("Match: {}, {} ,{} , {}", &cap[0], &cap[1], &cap[2], &cap[3]);
    }
}
