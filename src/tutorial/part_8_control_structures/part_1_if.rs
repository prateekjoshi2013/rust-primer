pub fn main() {
    let grade;
    let mut marks = 95;
    if marks >= 90 {
        grade = 'A';
    } else if marks > 80 {
        grade = 'B';
    } else if marks > 70 {
        grade = 'C';
    } else if marks > 60 {
        grade = 'D';
    } else {
        grade = 'f';
    }
    println!("grade: {} marks: {}", grade, marks);

    marks = if grade == 'A' {
        95
    } else if grade == 'B' {
        85
    } else if grade == 'C' {
        75
    } else if grade == 'D' {
        65
    } else {
        0
    };
    println!("grade: {} marks: {}", grade, marks);
}
