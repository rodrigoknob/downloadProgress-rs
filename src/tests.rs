#[cfg(test)]

mod tests {
    use std::process::Command;

    #[test]
    fn it_works() {

        let output = Command::new("ls")
            // .arg("-C")
            // .arg("tasklist")
            // .arg("/FI")
            // .arg(pid_query)
            .output()
            .expect("Failed to execute command");

        println!("checked success");
        println!("output : {}", String::from_utf8(output.stdout).unwrap());

    }
}