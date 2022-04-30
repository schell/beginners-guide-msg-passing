use std::{
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    thread,
};

/// All the messages that our worker thread receives.
enum WorkerMsg {
    DoWork(String),
    Quit,
}

/// All the messages that our main thread receives.
enum MainMsg {
    WorkResult(String),
    UserInput(String),
    Quit,
}

/// Work payloads
enum Work {
    Add(u32, u32),
    Ping,
    Help,
}

impl TryFrom<String> for Work {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let words: Vec<&str> = value.split_ascii_whitespace().collect::<Vec<_>>();
        match words.as_slice() {
            ["add", a_str, b_str] => {
                let a = a_str.parse::<u32>()?;
                let b = b_str.parse::<u32>()?;
                Ok(Work::Add(a, b))
            }
            ["ping"] => Ok(Work::Ping),
            ["help"] => Ok(Work::Help),
            ["?"] => Ok(Work::Help),
            _ => anyhow::bail!("unsupported or malformed command string '{}'", value),
        }
    }
}

/// A worker thread that does some computation.
fn worker(main_tx: SyncSender<MainMsg>, worker_rx: Receiver<WorkerMsg>) {
    println!("worker thread starting up");
    let mut pings = 0u32;
    loop {
        match worker_rx.recv() {
            Ok(msg) => {
                let send_result = match msg {
                    WorkerMsg::DoWork(s) => match Work::try_from(s) {
                        Ok(Work::Add(a, b)) => {
                            main_tx.send(MainMsg::WorkResult(format!("{}", a + b)))
                        }
                        Ok(Work::Ping) => {
                            pings += 1;
                            main_tx.send(MainMsg::WorkResult(format!(
                                "ping'd worker thread {} {}",
                                pings,
                                match pings {
                                    1 => "time",
                                    _ => "times",
                                }
                            )))
                        }
                        Ok(Work::Help) => main_tx.send(MainMsg::WorkResult(
                            "available commands: add, ping, help (or ?), quit".to_string(),
                        )),
                        Err(err) => main_tx.send(MainMsg::WorkResult(format!(
                            "worker thread could not parse work: {}",
                            err
                        ))),
                    },
                    WorkerMsg::Quit => {
                        println!("worker thread got quit request");
                        break;
                    }
                };

                if let Err(err) = send_result {
                    eprintln!("worker thread encountered a send error on main_tx: {}", err);
                    break;
                }
            }
            Err(err) => {
                eprintln!(
                    "worker thread encountered a recv error on worker_rx: {}",
                    err
                );
                break;
            }
        }
    }

    println!("worker thread is exiting");
}

/// An input thread that receives input from the user.
fn input(main_tx: SyncSender<MainMsg>) {
    println!("input thread starting up");

    thread::sleep(std::time::Duration::from_secs_f32(0.5));
    println!("\nwelcome to the repl");

    let stdin = std::io::stdin();
    loop {
        let mut user_input = String::new();
        if let Err(err) = stdin.read_line(&mut user_input) {
            eprintln!("input thread encountered a send error on main_tx: {}", err);
            break;
        }

        let trimmed_input = user_input.trim().to_string();
        let send_result = match trimmed_input.as_str() {
            "quit" => {
                println!("input thread got quit request");
                let _ = main_tx.send(MainMsg::Quit);
                break;
            }

            _ => main_tx.send(MainMsg::UserInput(trimmed_input)),
        };

        if let Err(err) = send_result {
            eprintln!("input thread encountered a send error on main_tx: {}", err);
            break;
        }
    }

    println!("input thread is exiting");
}

fn main() {
    println!("main thread starting up");

    let bound = 1;
    let (worker_tx, worker_rx) = sync_channel::<WorkerMsg>(bound);
    let (main_tx, main_rx) = sync_channel::<MainMsg>(bound);

    let workers_main_tx = main_tx.clone();

    let worker_handle = thread::spawn(move || worker(workers_main_tx, worker_rx));
    let _input_handle = thread::spawn(move || input(main_tx));

    loop {
        match main_rx.recv() {
            Ok(msg) => match msg {
                MainMsg::WorkResult(s) => println!("> {}\n", s),
                MainMsg::UserInput(user_input) => {
                    if let Err(err) = worker_tx.send(WorkerMsg::DoWork(user_input)) {
                        eprintln!("main thread encountered a send error on worker_tx: {}", err);
                        break;
                    }
                }
                MainMsg::Quit => {
                    println!("main thread got quit request");
                    if let Err(err) = worker_tx.send(WorkerMsg::Quit) {
                        eprintln!("main thread encountered a send error on worker_tx: {}", err);
                    }
                    if let Err(_err) = worker_handle.join() {
                        eprintln!("worker thread erred during exit");
                    }
                    break;
                }
            },

            Err(err) => {
                eprintln!("main thread encountered a recv error on main_rx: {}", err);
                break;
            }
        }
    }

    println!("main thread is exiting");
    println!("goodbye!");
}
