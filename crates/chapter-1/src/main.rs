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

impl Work {
    fn try_from(value: String) -> Result<Self, String> {
        let words: Vec<&str> = value.split_ascii_whitespace().collect::<Vec<_>>();
        match words.as_slice() {
            ["add", a_str, b_str] => {
                let a = a_str.parse::<u32>().map_err(|err| format!("{}", err))?;
                let b = b_str.parse::<u32>().map_err(|err| format!("{}", err))?;
                Ok(Work::Add(a, b))
            }
            ["ping"] => Ok(Work::Ping),
            ["help"] => Ok(Work::Help),
            ["?"] => Ok(Work::Help),
            _ => anyhow::bail!("unsupported or malformed command string '{}'", value),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
