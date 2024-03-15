use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::process::exit;
use std::thread::spawn;
use std::sync::{Arc, Mutex, LockResult};
use std::time::SystemTime;

struct Game {
    spoints: u32,
    cpoints: u32,
    winpoints: u32,
}

struct Account {
    username: String,
    password: String,
    logged: bool,
    game: Game,
}

fn handle_client(mut stream: TcpStream, accounts: Arc<Mutex<Vec<Account>>>) {
    let mut buffer = [0; 1024];
    let mut id = 0;
    let mut id_opt = None;
    let mut res;

    println!("new client connected");

    // ricezione comando
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                // nel caso di connessione chiusa
                println!("id: {}", id);
                if n == 0 {
                    println!("client disconnected");

                    // se loggato logout
                    // check che id sia valido
                    if id_opt.is_none() {
                        break;
                    }

                    let lock_result = accounts.lock();
                    match lock_result {
                        LockResult::Ok(mut vector) => {
                            if vector[id].logged {
                                vector[id].logged = false;
                                println!("{} logged out", vector[id].username);
                            }
                        }
                        LockResult::Err(_) => {
                            println!("error: failed to lock accounts");
                        }
                    }
                    break;
                }

                let msg = String::from_utf8_lossy(&buffer[..n]);
                println!("received: {}", msg);
                let mut msg = msg.split_whitespace();
                match msg.next() {
                    Some("login") => {  
                        let username;
                        match msg.next() {
                            Some(value) => username = value,
                            None => {
                                res = stream.write(b"error: username not provided\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                }
                                continue;
                            }
                        }               
                        
                        let password;
                        match msg.next() {
                            Some(value) => password = value,
                            None => {
                                res = stream.write(b"error: password not provided\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                }
                                continue;
                            }
                        }

                        let mut wrong_password = false;
                        let lock_result = accounts.lock();
                        match lock_result {
                            LockResult::Ok(mut vector) => {
                                for (i, account) in vector.iter().enumerate() {
                                    if account.username == username {
                                        if account.password == password {
                                            id_opt = Some(i);
                                        } else {
                                            wrong_password = true;
                                            res = stream.write(b"error: wrong password\n");
                                            if let Err(e) = res {
                                                println!("failed: {}", e);
                                            }
                                        }
                                        break;
                                    }
                                }

                                if wrong_password {
                                    continue;
                                }

                                match id_opt {
                                    Some(value) => {
                                        id = value;
                                        // se online errore
                                        if vector[id].logged {
                                            res = stream.write(b"error: account already logged in\n");
                                            if let Err(e) = res {
                                                println!("failed: {}", e);
                                            }
                                            continue;
                                        }
        
                                        vector[id].logged = true;
                                        res = stream.write(b"logged in\n");
                                        if let Err(e) = res {
                                            println!("failed: {}", e);
                                            continue;
                                        }
                                        println!("{} logged in", vector[id].username);
                                    },
                                    None => {
                                        let new_account = Account {
                                            username: username.to_string(),
                                            password: password.to_string(),
                                            logged: true,
                                            game: Game {
                                                spoints: 0,
                                                cpoints: 0,
                                                winpoints: 3,
                                            },
                                        };
                                        vector.push(new_account);
                                        id = vector.len() - 1;
                                        id_opt = Some(id);
                                        println!("{} created", username);
                                        res = stream.write(b"account created and logged in\n");
                                        if let Err(e) = res {
                                            println!("failed: {}", e);
                                            continue;
                                        }                           
                                    },
                                }
                            }
                            LockResult::Err(_) => {
                                let _ = stream.write(b"error: failed to lock accounts\n");
                                panic!("failed to lock accounts");
                            }
                        }

                        
                    }
                    Some("logout") => {
                        let lock_result = accounts.lock();
                        match lock_result {
                            LockResult::Ok(mut vector) => {
                                vector[id].logged = false;
                                println!("{} logged out", vector[id].username);
                                res = stream.write(b"logged out\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                    continue;
                                }
                            }
                            LockResult::Err(_) => {
                                let _ = stream.write(b"error: failed to lock accounts\n");
                                panic!("failed to lock accounts");
                            }
                        }
                        break;
                    }
                    Some("play") => {
                        let n_tcp;
                        match msg.next() {
                            Some(value) => n_tcp = value,
                            None => {
                                res = stream.write(b"error: number of points not provided\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                }
                                continue;
                            }
                        };
                        let n;
                        match n_tcp.parse::<u32>() {
                            Ok(value) => n = value,
                            Err(_) => {
                                res = stream.write(b"error: invalid number of points\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                }
                                continue;
                            }
                        };

                        let lock_result = accounts.lock();
                        match lock_result {
                            LockResult::Ok(mut vector) => {
                                vector[id].game.winpoints = n;
                                vector[id].game.spoints = 0;
                                vector[id].game.cpoints = 0;
                                println!("{} started a game with {} points", vector[id].username, n);
                                res = stream.write(b"game started\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                    continue;
                                }
                            }
                            LockResult::Err(_) => {
                                let _ = stream.write(b"error: failed to lock accounts\n");
                                panic!("failed to lock accounts");
                            }
                        }
                    }
                    Some("choice") => {
                        let choice;
                        match msg.next() {
                            Some(value) => choice = value,
                            None => {
                                res = stream.write(b"error: choice not provided\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                }
                                continue;
                            }
                        };

                        let numchoice;
                        match choice {
                            "r" => numchoice = 0,
                            "p" => numchoice = 1,
                            "s" => numchoice = 2,
                            _ => {
                                res = stream.write(b"error: invalid choice\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                }
                                continue;
                            }
                        };
                        let server_choice = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() % 3;
                        let schoice_str = match server_choice {
                            0 => "rock",
                            1 => "paper",
                            2 => "scissors",
                            _ => "error: invalid server choice",
                        };
                        let result = (numchoice + 3 - server_choice) % 3;

                        let lock_result = accounts.lock();
                        match lock_result {
                            LockResult::Ok(mut vector) => {
                                let msg;
                                let account = &mut vector[id];
                                match result {
                                    0 => {
                                        msg = "tie\n";
                                    }
                                    1 => {
                                        account.game.cpoints += 1;
                                        msg = "you win\n";
                                    }
                                    2 => {
                                        account.game.spoints += 1;
                                        msg = "you lose\n";
                                    }
                                    _ => {
                                        msg = "error: invalid result\n";
                                    }
                                }

                                let msg_state = 
                                    if account.game.spoints == account.game.winpoints 
                                        {format!("{} - {} => you lose the game\n", account.game.cpoints, account.game.spoints)}
                                    else if account.game.cpoints == account.game.winpoints
                                        {format!("{} - {} => you win the game\n", account.game.cpoints, account.game.spoints)}
                                    else
                                        {format!("{} - {} ({} to win)\n", account.game.cpoints, account.game.spoints, account.game.winpoints)};

                                let res = stream.write(format!("{}: {}{}", schoice_str, msg, msg_state).as_bytes());
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                    continue;
                                }
                            }
                            LockResult::Err(_) => {
                                let _ = stream.write(b"error: failed to lock accounts\n");
                                panic!("failed to lock accounts");
                            }
                        }
                    }
                    Some("quit") => {
                        println!("{} quit the game", id);
                        res = stream.write(b"end of game\n");
                        if let Err(e) = res {
                            println!("failed: {}", e);
                            continue;
                        }
                    }
                    _ => {
                        res = stream.write(b"error: invalid command\n");
                        if let Err(e) = res {
                            println!("failed: {}", e);
                            continue;
                        }
                    }
                }
            },
            Err(e) => {
                println!("failed: {}", e);
                continue;
            }
        }
    }
}

fn main() {
    let listener;
    let listener_opt = TcpListener::bind("127.0.0.1:8080");
    match listener_opt {
        Ok(value) => listener = value,
        Err(e) => {
            println!("failed to bind: {}", e);
            exit(1);
        }
    }

    println!("server started");

    let accounts = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let accounts_clone = Arc::clone(&accounts);
                spawn(move || {
                    handle_client(stream, accounts_clone);
                });
            }
            Err(e) => {
                println!("failed: {}", e);
            }
        }
    }
}