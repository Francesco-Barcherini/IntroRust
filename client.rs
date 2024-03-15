use std::io::prelude::*;
use std::net::TcpStream;
use std::io;

fn main() {
    let stream_opt = TcpStream::connect("localhost:8080");
    let mut stream;

    match stream_opt {
        Ok(value) => stream = value,
        Err(e) => {
            println!("failed to connect: {}", e);
            return;
        }
    }

    let mut buffer;
    
    println!("Welcome to rock, paper, scissors game!");

    // loop login
    loop {
        println!("login or create an account: login <username> <password>");
        let mut input = String::new();
        let res = io::stdin().read_line(&mut input);
        if let Err(e) = res {
            println!("failed: {}", e);
            continue;
        }
        
        let input = input.trim();
        let mut input = input.split_whitespace();
        let command = input.next().unwrap();
        match command {
            "login" => {
                let username;
                let password;
                let res = input.next();
                match res {
                    Some(value) => username = value,
                    None => {
                        println!("error: username not provided");
                        continue;
                    }
                }
                
                let res = input.next();
                match res {
                    Some(value) => password = value,
                    None => {
                        println!("error: password not provided");
                        continue;
                    }
                }

                let msg = format!("login {} {}\n", username, password);
                let res = stream.write(msg.as_bytes());
                if let Err(e) = res {
                    println!("failed: {}", e);
                    continue;
                }
                let mut bytes = [0; 1024];
                match stream.read(&mut bytes) {
                    Ok(n) => {
                        buffer = String::from_utf8_lossy(&bytes[..n]).to_string();
                    }
                    Err(e) => {
                        println!("failed: {}", e);
                        continue;
                    }
                }

                // if buffer starts with error, print it and continue, else break
                if buffer.starts_with("error") {
                    println!("{}", buffer);
                    buffer.clear();
                    continue;
                } else {
                    println!("{}", buffer);
                    buffer.clear();
                    break;
                }
            }
            _ => {
                println!("invalid command");
            }
        }
    }

    /*
    commands:
    - play <number of points>
    - <rock, paper, scissors>
    - logout
    - quit
    */
    println!("**** commands ****");
    println!("play <number of points> // start a new game against the server");
    println!("<r|p|s> // make a choice (rock, paper, scissors)");
    println!("logout // logout from the server");
    println!("quit // quit the game");
    println!("******************");

    let mut playing = false;
    loop {

        let mut input = String::new();
        let res = io::stdin().read_line(&mut input);
        if let Err(e) = res {
            println!("failed: {}", e);
            continue;
        }
        
        let input = input.trim();
        let mut input = input.split_whitespace();
        let command = input.next().unwrap();
        match command {
            "play" => {
                let n;
                let res = input.next();
                match res {
                    Some(value) => n = value,
                    None => {
                        println!("error: number of points not provided");
                        continue;
                    }
                }
                let msg = format!("play {}\n", n);
                let res = stream.write(msg.as_bytes());
                if let Err(e) = res {
                    println!("failed: {}", e);
                    continue;
                }

                let mut bytes = [0; 1024];
                match stream.read(&mut bytes) {
                    Ok(n) => {
                        buffer = String::from_utf8_lossy(&bytes[..n]).to_string();
                    }
                    Err(e) => {
                        println!("failed: {}", e);
                        continue;
                    }
                }

                // if buffer does not start with error, playing = true
                if !buffer.starts_with("error") {
                    playing = true;
                }

                println!("{}", buffer);
                buffer.clear();
            }
            "rock" | "paper" | "scissors" | "r" | "p" | "s" => {
                if !playing {
                    println!("error: not playing");
                    continue;
                }
                let msg = format!("choice {}\n", command[0..1].to_string());
                let res = stream.write(msg.as_bytes());
                if let Err(e) = res {
                    println!("failed: {}", e);
                    continue;
                }

                let mut bytes = [0; 1024];
                match stream.read(&mut bytes) {
                    Ok(n) => {
                        buffer = String::from_utf8_lossy(&bytes[..n]).to_string();
                    }
                    Err(e) => {
                        println!("failed: {}", e);
                        continue;
                    }
                }

                println!("{}", buffer);

                // se buffer contiene "=>", allora il gioco è finito
                if buffer.contains("=>") {
                    playing = false;
                    println!("**** commands ****");
                    println!("play <number of points> // start a new game against the server");
                    println!("<r|p|s> // make a choice (rock, paper, scissors)");
                    println!("logout // logout from the server");
                    println!("quit // quit the game");
                    println!("******************");
                }

                buffer.clear();
            }
            "logout" => {
                let res = stream.write(b"logout\n");
                if let Err(e) = res {
                    println!("failed: {}", e);
                    continue;
                }

                let mut bytes = [0; 1024];
                match stream.read(&mut bytes) {
                    Ok(n) => {
                        buffer = String::from_utf8_lossy(&bytes[..n]).to_string();
                    }
                    Err(e) => {
                        println!("failed: {}", e);
                        continue;
                    }
                }

                println!("{}", buffer);
                buffer.clear();
                break;
            }
            "quit" => {
                if !playing {
                    println!("error: not playing");
                    continue;
                }

                let res = stream.write(b"quit\n");
                if let Err(e) = res {
                    println!("failed: {}", e);
                    continue;
                }

                let mut bytes = [0; 1024];
                match stream.read(&mut bytes) {
                    Ok(n) => {
                        buffer = String::from_utf8_lossy(&bytes[..n]).to_string();
                    }
                    Err(e) => {
                        println!("failed: {}", e);
                        continue;
                    }
                }

                println!("{}", buffer);
                buffer.clear();
            }
            _ => {
                println!("invalid command");
            }
        }
    }
}