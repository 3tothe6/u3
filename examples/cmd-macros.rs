use u3::{cmd, cmd_o};

fn main() {
    cmd!("git.exe", "switch", "this-does-not-exist").unwrap();
    println!("{:?}", cmd_o!("git.exe", "--help").unwrap());
}
