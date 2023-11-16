use regex::Regex;

pub fn main() {
    let re = Regex::new(r"[prt]ain").unwrap();
    let text = "rrrain spain none";
    println!("The text has a match {:?}", re.is_match(text));
    println!("The text has a match {:?}", re.find(text)); // first match
    for cap in re.captures_iter(text) {
        println!("match: {:?}", &cap[0]); // returns matched strings
    }
}
