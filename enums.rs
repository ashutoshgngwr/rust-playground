// sample console logging utility using enums

enum Log {
    Info(String),
    Debug(String),
    Warn(String),
    Error(String)
}

impl Log {
    pub fn print(&self) {
        match self {
            Log::Info(msg) => println!("info: {:?}", msg),
            Log::Debug(msg) => println!("debug: {:?}", msg),
            Log::Warn(msg) => println!("warn: {:?}", msg),
            Log::Error(msg) => println!("error: {:?}", msg)
        }
    }
}

fn main() {
    Log::Info(String::from("this is info message")).print();
    Log::Debug(String::from("this is debug message")).print();
    Log::Warn(String::from("this is warn message")).print();
    Log::Error(String::from("this is error message")).print();
}
