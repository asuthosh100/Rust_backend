// enum based request router 

enum Request {
    Ping,
    Echo(String),
    Upload {filename: String, bytes: usize},
}

fn handle_request(req: Request)-> String {

    match req {
        Request::Ping => "pong!".to_string(), 
        Request::Echo(text) => format!("Echo:{}", text), 
        Request::Upload{filename, bytes} => {
            format!("Filename is {} and the size of the file is {} bytes", filename, bytes)
        }
    }

}

fn main() {
    let r1 = Request::Ping;
    let r2 = Request::Echo("Hello!".to_string()); 
    let r3 = Request::Upload{
        filename : "nag.jpg".to_string(), 
        bytes : 2048
    };

    println!("{}", handle_request(r1)); 
    println!("{}", handle_request(r2)); 
    println!("{}", handle_request(r3)); 
}