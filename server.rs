/*
Code a client-server application in Rust using the TCP protocol defined in std::net.
The server listens for incoming connections on 127.0.0.1. The server should be able to handle multiple clients at the same time using threads.

The server should be able to handle the following commands for accounts handling:
login <username> <password> - logs in the user with the given username and password
    if account with same username does not exists, signup the user
logout - logs out the user, end of game and disconnects the client
play <n>
    the game is a simple rock-paper-scissors game where the server generates a random choice and the client sends his choice. 
    The first with n points wins.
choice <r|p|s>
    the server receives r, p or s and compares it with the server's choice. The server sends the result and points to the client.
quit 
    end of game

Game is a struct with points of server, points of client, points to win, choice of server
Account is a struct with username, password, logged, next (for the linked list of accounts)

Avoid the use of unwrap. Use instead match, if let or ?.
*/

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::process::exit;
use std::thread::spawn;
use std::sync::{Arc, Mutex, LockResult};
use std::rc::Rc;
use std::cell::RefCell;
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
    next: Option<Rc<Refcell<Account>>>,
}

struct LinkedList {
    head: Option<Rc<Refcell<Account>>>,
}

impl LinkedList {
    fn new() -> LinkedList {
        LinkedList {
            head: None,
        }
    }

    fn insert(&mut self, account: Account) {
        let mut new_node = Rc::new(RefCell::new(account));
        if let Some(&mut head) = self.head {
            new_node.next = Some(Rc::clone(head));
        }
        self.head = Some(new_node);
    }

    /* return a mutable pointer to account with that username */
    fn fin

    fn find(&self, username: &str) -> Option<&Account> {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.username == username {
                return Some(node);
            }
            current = &node.next;
        }
        None
    }
}


fn handle_client(mut stream: TcpStream, accounts: Arc<Mutex<LinkedList>>) {
    let mut buffer = [0; 1024];
    let mut account: &mut Account;
    let mut account_opt = None;
    let mut res;

    // ricezione comando
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                let msg = String::from_utf8_lossy(&buffer[..n]);
                let mut msg = msg.split_whitespace();
                match msg.next() {
                    Some("login") => {  
                        let mut username = "";
                        match msg.next() {
                            Some(value) => username = value,
                            None => {
                                res = stream.write(b"error: username not provided\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                    continue;
                                }
                            }
                        }               
                        
                        let mut password = "";
                        match msg.next() {
                            Some(value) => password = value,
                            None => {
                                res = stream.write(b"error: password not provided\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                    continue;
                                }
                            }
                        }

                        
                        let lock_result = accounts.lock();
                        match lock_result {
                            LockResult::Ok(value) => {
                                account_opt = value.find(username);
                            }
                            LockResult::Err(_) => {
                                res = stream.write(b"error: failed to lock accounts\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                    continue;
                                }
                            }
                        }

                        match account_opt {
                            Some(value) => {
                                account = value;
                                if account.password == password {
                                    // se online errore
                                    if account.logged {
                                        res = stream.write(b"error: account already logged in\n");
                                        if let Err(e) = res {
                                            println!("failed: {}", e);
                                            continue;
                                        }
                                    }
    
                                    account.logged = true;
                                    res = stream.write(b"logged in\n");
                                    if let Err(e) = res {
                                        println!("failed: {}", e);
                                        continue;
                                    }
                                } else {
                                    res = stream.write(b"error: wrong password\n");
                                    if let Err(e) = res {
                                        println!("failed: {}", e);
                                        continue;
                                    }
                                }
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
                                    next: None,
                                };
    
                                let lock_result = accounts.lock();
                                match lock_result {
                                    LockResult::Ok(value) => {
                                        value.insert(new_account);
                                    }
                                    LockResult::Err(_) => {
                                        res = stream.write(b"error: failed to lock accounts\n");
                                        if let Err(e) = res {
                                            println!("failed: {}", e);
                                            continue;
                                        }
                                    }
                                }
                                
                                res = stream.write(b"account created and logged in\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                    continue;
                                }                           
                            },
                        }
                    }
                    Some("logout") => {
                        if account_opt.is_none() || account.logged == false {
                            res = stream.write(b"error: account not logged in\n");
                            if let Err(e) = res {
                                println!("failed: {}", e);
                                continue;
                            }
                        }

                        // closes connection and thread
                        account.logged = false;
                        res = stream.write(b"logged out\n");
                        if let Err(e) = res {
                            println!("failed: {}", e);
                            continue;
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
                                    continue;
                                }
                            }
                        };
                        let n: u32;
                        match n_tcp.parse::<u32>() {
                            Ok(value) => n = value,
                            Err(_) => {
                                res = stream.write(b"error: invalid number of points\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                    continue;
                                }
                            }
                        };
                        account.game.winpoints = n;
                        account.game.spoints = 0;
                        account.game.cpoints = 0;
                        res = stream.write(b"game started\n");
                        if let Err(e) = res {
                            println!("failed: {}", e);
                            continue;
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
                                    continue;
                                }
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
                                    continue;
                                }
                            }
                        };
                        let server_choice = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() % 3;
                        let result = (numchoice - server_choice + 3) % 3;
                        match result {
                            0 => {
                                res = stream.write(b"tie\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                    continue;
                                }
                            }
                            1 => {
                                account.game.cpoints += 1;
                                res = stream.write(b"you win\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                    continue;
                                }
                            }
                            2 => {
                                account.game.spoints += 1;
                                res = stream.write(b"you lose\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                    continue;
                                }
                            }
                            _ => {
                                res = stream.write(b"error: invalid result\n");
                                if let Err(e) = res {
                                    println!("failed: {}", e);
                                    continue;
                                }
                            }
                        }
                        if account.game.spoints == account.game.winpoints {
                            let msg = format!("{} - {} => you lose the game\n", account.game.spoints, account.game.cpoints);
                            res = stream.write(msg.as_bytes());
                            if let Err(e) = res {
                                println!("failed: {}", e);
                                continue;
                            }
                        } else if account.game.cpoints == account.game.winpoints {
                            let msg = format!("{} - {} => you win the game\n", account.game.spoints, account.game.cpoints);
                            res = stream.write(msg.as_bytes());
                            if let Err(e) = res {
                                println!("failed: {}", e);
                                continue;
                            }
                        } else {
                            let msg = format!("{} - {} ({} to win)\n", account.game.spoints, account.game.cpoints, account.game.winpoints);
                            res = stream.write(msg.as_bytes());
                            if let Err(e) = res {
                                println!("failed: {}", e);
                                continue;
                            }
                        }
                    }
                    Some("quit") => {
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

    let accounts = Arc::new(Mutex::new(LinkedList::new()));

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