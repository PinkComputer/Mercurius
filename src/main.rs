use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fmt,
    str::FromStr,
    str,
    io,
    env,
    net::Shutdown,
    thread,
};

// use chrono::{NaiveDate, NaiveDateTime};


//At the time I was unaware there was already an IP address type and I was too prideful to switch
#[derive(Clone)]
struct Ipv4 (u8,u8,u8,u8);

//Creates the 
impl fmt::Display for Ipv4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}.{}.{}.{})", self.0, self.1, self.2, self.3)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Ipv4ParseError; 
impl fmt::Display for Ipv4ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invaild address: {}", (format!("{}", self)))
    }
}

impl FromStr for Ipv4 {
    type Err = Ipv4ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| Some(s.split('.')))
            .ok_or(Ipv4ParseError)?;
        //The iterator is started/moved, unwrapped into a string, parsed into u8, and then unwrapped
        //again. Is this a good way to do it? Prolly not
        let u8_1_fromstr: u8 = iter.next().unwrap().parse().unwrap(); 
        let u8_2_fromstr: u8 = iter.next().unwrap().parse().unwrap();
        let u8_3_fromstr: u8 = iter.next().unwrap().parse().unwrap();
        let u8_4_fromstr: u8 = iter.next().unwrap().parse().unwrap();

        Ok(Ipv4(u8_1_fromstr,u8_2_fromstr,u8_3_fromstr,u8_4_fromstr))
        

    }
}
 

struct Message {
    to: Ipv4,
    from: Ipv4,
    from_public: u8,
    to_public: u8,
    message: String, 
   }
impl Message {
    fn as_string(&self) -> String {
        format!("To: {0}, \n
From: {1}, \n
From_public: {2}, \n
To_public: {3}, \n
Message: {4} ",
self.to, self.from, self.from_public, self.to_public, self.message,)
    }
}


impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Message::as_string(self))
    }
}

impl Message {
    //This has prolly already taken an hour but I am gonna just say development started at 8:55 PM
    //5/17/2025, time spent: 01:11:20~
    //That is the time needed to fix the single error, god help me
    fn from_request(s: &str) -> Message {
       
       let mut iter = s.split(",");

       let line = iter.next();
       let line_iter = line.expect("Iter is empty!").to_string().split_once(":").expect("Spliting failed!").1.to_owned();
       let to_ip_request: Ipv4  = Ipv4::from_str(line_iter.to_string().trim()).unwrap();

       let line = iter.next();
       let binding = line.expect("Iter is empty!").to_string();
       let line_iter = binding.split_once(":").expect("Spliting failed!").1;
       let from_ip_request: Ipv4  = Ipv4::from_str(line_iter.to_string().trim()).unwrap();

       let line = iter.next();
       let binding = line.expect("Iter is empty!").to_string();
       let line_iter = binding.split_once(":").expect("Spliting failed!").1;
       let from_key_request: u8  = line_iter.to_string().trim().parse().unwrap();

       let line = iter.next();
       let binding = line.expect("Iter is empty!").to_string();
       let line_iter = binding.split_once(":").expect("Spliting failed!").1;
       let to_key_request: u8  = line_iter.to_string().trim().parse().unwrap();

       let line = iter.next();
       let binding = line.expect("Iter is empty!").to_string();
       let line_iter = binding.split_once(":").expect("Spliting failed!").1;
       let message_request:String = line_iter.to_string();

       let result = Message {
           to: to_ip_request,
           from: from_ip_request,
           from_public: from_key_request,
           to_public: to_key_request,
           message: message_request
        };

       return result


    }
}

#[derive(Clone)]
struct FullIp {
    address: Ipv4,
    port: u16,
}


impl FromStr for FullIp {
    type Err = Ipv4ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| Some(s.split('.')))
            .ok_or(Ipv4ParseError)?;
        //The iterator is started/moved, unwrapped into a string, parsed into u8, and then unwrapped
        //again. Is this a good way to do it? Prolly not
        let u8_1_fromstr: u8 = iter.next().unwrap().parse().unwrap(); 
        let u8_2_fromstr: u8 = iter.next().unwrap().parse().unwrap();
        let u8_3_fromstr: u8 = iter.next().unwrap().parse().unwrap();
        let u8_4_fromstr: u8 = iter.next().expect("Failed at u8_4").split_once(":").unwrap().0.parse().unwrap();

        let ipadd = Ipv4(u8_1_fromstr,u8_2_fromstr,u8_3_fromstr,u8_4_fromstr);
        let (_wrong, port) = s
            .strip_suffix(')')
            .and_then(|s| s.split_once(":"))
            .ok_or(Ipv4ParseError)?;
        let port_fromstring = port.parse::<u16>().map_err(|_| Ipv4ParseError)?;
        
        Ok(FullIp{address:ipadd, port:port_fromstring})
        

    }
}

impl fmt::Display for FullIp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}.{}.{}.{}:{})", self.address.0, self.address.1, self.address.2, self.address.3, self.port)
    }
}

impl FullIp {
    fn connect_format(&self) -> String { 
        return format!("{}", self).strip_prefix('(').expect("prefix").strip_suffix(')').expect("suffix").to_string();
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    
    println!("   __  ___                     _        
  /  |/  /__ __________ ______(_)_ _____
 / /|_/ / -_) __/ __/ // / __/ / // (_-<
/_/  /_/\\__/_/  \\__/\\_,_/_/ /_/\\_,_/___/
                                        ");
    println!("Ver 1.0");
    println!("Live chat only");
    
    let mut local_ip = String::new();
    print!("Enter IP and port you would like connections to go to (ex:127.0.0.1:7878): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut local_ip)
        .expect("Failed to read lines");
    


    let local_ip: FullIp = FullIp::from_str(local_ip.trim()).expect("Invalid address! Exiting...");
    let mut listener = TcpListener::bind(FullIp::connect_format(&local_ip)).expect("Listener could not be set up...");




    let mut remote_ip = String::new();
    print!("Enter IP and port you would like to connect to: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut remote_ip)
        .expect("Failed to read lines");
    
    let remote_ip: FullIp = FullIp::from_str(remote_ip.trim()).expect("Invalid address! Exiting...");

    //println!("{}", FullIp::connect_format(&remote_ip));
    
    let mut connection = TcpStream::connect(FullIp::connect_format(&remote_ip)).expect("Could not connect...");

    //connection.set_nonblocking(true).expect("set_nonblocking call failed");

    
    /*
    if let Ok(stream) = TcpStream::connect(FullIp::connect_format(&remote_ip)) {
        println!("Holy shit");
    } else {
        println!("Goddamn it");
    }
    */
    let write_thread = thread::spawn( || {
        stream_writing(connection, remote_ip);
    });

    let read_thread = thread::spawn( move || {
        let listener_copy = listener;
        for stream in listener_copy.incoming() {
            stream_reading(stream.unwrap());
        }
    });

    write_thread.join().unwrap();

    //let listener = TcpListener::bind("127.0.0.1:7879").unwrap();
    
    /*for stream in connection {
        //let stream = stream.unwrap();

        handle_connection(stream);
    
    }*/
    
}

fn stream_writing(mut stream: TcpStream, remote: FullIp) {
    //println!("Handling connection!");
    //let buf_reader = BufReader::new(&stream);
    /*let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    */
    loop {
        print!("Enter Message (Enter nothing to exit): ");
        io::stdout().flush().unwrap();

        let mut ent_message = String::new();

        io::stdin()
            .read_line(&mut ent_message)
            .expect("Failed to read line");

        match ent_message.trim() {
            "" => {
                break;
            }
            _ => println!("Sending...")
        }



        let response_message = Message {
            to: remote.address.clone(),
            from: Ipv4::from_str(&(format!("({})", stream.local_addr().unwrap().ip()))).unwrap(),
            from_public: 1,
            to_public: 2,
            message: ent_message,
        };
     
     //let ip = "(192.0.0.1)";
     //let test = Ipv4::from_str(ip);
     //let test_2 = format!("{}", test.unwrap());
     //println!("{} \n \n", test);
    
     //println!("{}", response_message.as_string());
     
        let binding_message = response_message.as_string();
        let message_iter = binding_message.split("\n");
    
     //println!("{}", message_iter.clone().count());
        
        for line in message_iter{
            stream.write(line.to_string().as_bytes());
        }
        
        stream.shutdown(Shutdown::Write);

        stream = TcpStream::connect(FullIp::connect_format(&remote)).expect("Lost connection...");


        
     //stream.write_all(response_message.as_string().split("/n").next().unwrap().as_bytes());
     
    //println!("Request: {http_request:#?}");
    }
}


fn stream_reading(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let tcp_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    println!("Request: {tcp_request:#?}");

    for message in tcp_request {

        println!("Message: \n {}", format!("{}",Message::from_request(&message)))
    }


}
