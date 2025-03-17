use std::process:{Command, ExitStatus};

fn main() {
    println!("Spawning a child process to run 'ls -l'");

    let ouptut = Command::new("ls")
        .arg("-l")
        .spawn() // spawn the child process
        .expect("Failed to start process");

    let status: ExitStatus = output.wait().expect("Failed to wait on child");

    println!("Child process exited with status: {}", status);
}