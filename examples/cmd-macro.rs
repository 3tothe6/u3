use u3::cmd;

fn main() {
    cmd!("git.exe", "--help").unwrap();
}
