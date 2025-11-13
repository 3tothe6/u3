use u3::cmd;

fn main() {
    cmd!("git.exe", "switch", "this-does-not-exist").unwrap();
    cmd!("git.exe", "--help").unwrap();
}
