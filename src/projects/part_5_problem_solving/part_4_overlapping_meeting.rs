pub fn main() {
    let mut meeting_sec_b = vec![vec![14, 15], vec![5, 10]];
    meeting_sec_b.sort_by_key(|p| p[1]);
    let mut meeting_sec_a = vec![vec![13, 15], vec![15, 16], vec![7, 9]];
    meeting_sec_a.sort_by_key(|p| p[1]);
    let (mut i, mut j) = (0, 0);
    let mut res: Vec<Vec<i32>> = Vec::new();
    while i < meeting_sec_a.len() && j < meeting_sec_b.len() {
        if meeting_sec_a[i][1] < meeting_sec_b[j][1] {
            let s = if meeting_sec_a[i][0] < meeting_sec_b[j][0] {
                meeting_sec_b[j][0]
            } else {
                meeting_sec_a[i][0]
            };
            let e = if meeting_sec_a[i][1] < meeting_sec_b[j][1] {
                meeting_sec_a[i][1]
            } else {
                meeting_sec_b[j][1]
            };
            if s < e {
                res.push(vec![s, e]);
            }
            i += 1;
        } else {
            let s = if meeting_sec_a[i][0] < meeting_sec_b[j][0] {
                meeting_sec_b[j][0]
            } else {
                meeting_sec_a[i][0]
            };
            let e = if meeting_sec_a[i][1] < meeting_sec_b[j][1] {
                meeting_sec_a[i][1]
            } else {
                meeting_sec_b[j][1]
            };
            if s < e {
                res.push(vec![s, e]);
            }
            j += 1;
        }

        println!("{:?}", res);
    }
}
