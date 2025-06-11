//TO DO: Use hex crate to allow ipv6 to actually work
//       Figure out how to send messages remotely rather than keeping everything local
//       Pretty print the messages
//       Aliases (this requires a SMALLLLLLL message struct rewrite)

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    fmt,
    str::FromStr,
    str,
    io,
    //env,
    net::Shutdown,
    thread,
};

// use chrono::{NaiveDate, NaiveDateTime};

use hex;


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

// May God strike you down FP.
#[derive(Clone)]
struct Ipv6 (u16,u16,u16,u16,u16,u16,u16,u16);

impl fmt::Display for Ipv6 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x})", self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7)
    }
}


#[derive(Debug, PartialEq, Eq)]
struct Ipv6ParseError; 
impl fmt::Display for Ipv6ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invaild address: {}", (format!("{}", self)))
    }
}

impl FromStr for Ipv6 {
    type Err = Ipv6ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| Some(s.split(':')))
            .ok_or(Ipv6ParseError)?;
        //The iterator is started/moved, unwrapped into a string, parsed into u16, and then unwrapped
        //again. Is this a good way to do it? Prolly not
        let u16_1_fromstr: u16 = iter.next().unwrap().parse().unwrap(); 
        let u16_2_fromstr: u16 = iter.next().unwrap().parse().unwrap();
        let u16_3_fromstr: u16 = iter.next().unwrap().parse().unwrap();
        let u16_4_fromstr: u16 = iter.next().unwrap().parse().unwrap();
        let u16_5_fromstr: u16 = iter.next().unwrap().parse().unwrap(); 
        let u16_6_fromstr: u16 = iter.next().unwrap().parse().unwrap();
        let u16_7_fromstr: u16 = iter.next().unwrap().parse().unwrap();
        let u16_8_fromstr: u16 = iter.next().unwrap().parse().unwrap();


        Ok(Ipv6(u16_1_fromstr,u16_2_fromstr,u16_3_fromstr,u16_4_fromstr,u16_5_fromstr,u16_6_fromstr,u16_7_fromstr,u16_8_fromstr))
     

    }
}




struct Message {
    to: IpAdd,
    from: IpAdd,
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
           to: IpAdd::V4(to_ip_request),
           from: IpAdd::V4(from_ip_request),
           from_public: from_key_request,
           to_public: to_key_request,
           message: message_request
        };

       return result


    }
}
#[derive(Clone)]
enum IpAdd {
    V4(Ipv4),
    V6(Ipv6)
}

impl fmt::Display for IpAdd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            IpAdd::V4(Ip_v4) => write!(f, "({}.{}.{}.{})", Ip_v4.0, Ip_v4.1, Ip_v4.2, Ip_v4.3),
            IpAdd::V6(Ip_v6) => write!(f, "({}:{}:{}:{}:{}:{}:{}:{})", Ip_v6.0, Ip_v6.1, Ip_v6.2, Ip_v6.3, Ip_v6.4, Ip_v6.5, Ip_v6.6, Ip_v6.7)
        }
    }
}

#[derive(Clone)]
struct FullIp {
    address: IpAdd,
    port: u16,
}

#[derive(Debug, PartialEq, Eq)]
struct FullIpParseError; 
impl fmt::Display for FullIpParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invaild address: {}", (format!("{}", self)))
    }
}

impl FullIp {
    fn from_str_ipv4(s: &str) -> Result<Self, <FullIp as FromStr>::Err>  {
        let mut iter = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| Some(s.split('.')))
            .ok_or(FullIpParseError)?;
        //The iterator is started/moved, unwrapped into a string, parsed into u8, and then unwrapped
        //again. Is this a good way to do it? Prolly not

        if (iter.clone().count() <= 1usize) {
            return Err(FullIpParseError);
        }

        let u8_1_fromstr = iter.next().unwrap().parse::<u8>();
        let u8_2_fromstr = iter.next().unwrap().parse::<u8>();
        let u8_3_fromstr = iter.next().unwrap().parse::<u8>();
        let u8_4_fromstr = iter.next().expect("Failed at u8_4").split_once(":").unwrap().0.parse();
        
        
        if (u8_1_fromstr.is_err()) {
            return Err(FullIpParseError)
        }
        
        let ipadd = IpAdd::V4(Ipv4(u8_1_fromstr.unwrap(),u8_2_fromstr.unwrap(),u8_3_fromstr.unwrap(),u8_4_fromstr.unwrap()));
        let (_wrong, port) = s
            .strip_suffix(')')
            .and_then(|s| s.split_once(":"))
            .ok_or(FullIpParseError)?;
        let port_fromstring = port.parse::<u16>().map_err(|_| FullIpParseError)?;
        
        Ok(FullIp{address:ipadd, port:port_fromstring})
        

    }
}



impl FullIp {

    fn from_str_ipv6(s: &str) -> Result<Self, <FullIp as FromStr>::Err>  {
        // Example Ipv6 with port: 
        // ([2001:0db8:0000:0000:0000:8a2e:0370:7334]:7878)
        let mut iter = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.strip_prefix('['))
            .and_then(|s| Some(s.split(':')))
            .ok_or(FullIpParseError)?;
        //The iterator is started/moved, unwrapped into a string, parsed into u8, and then unwrapped
        //again. Is this a good way to do it? Prolly not
        let u16_1_fromstr = iter.next().unwrap().parse::<u16>(); 
        let u16_2_fromstr = iter.next().unwrap().parse::<u16>();
        let u16_3_fromstr = iter.next().unwrap().parse::<u16>();
        let u16_4_fromstr = iter.next().unwrap().parse::<u16>(); 
        let u16_5_fromstr = iter.next().unwrap().parse::<u16>();
        let u16_6_fromstr = iter.next().unwrap().parse::<u16>();
        let u16_7_fromstr = iter.next().unwrap().parse::<u16>();
        let u16_8_fromstr = iter.next().expect("Failed at u16_8").strip_suffix(']').unwrap().parse::<u16>();
        
        
        if (u16_1_fromstr.is_err()) {
            return Err(FullIpParseError)
        }
        

        let ipadd = IpAdd::V6(Ipv6(u16_1_fromstr.unwrap(),u16_2_fromstr.unwrap(),u16_3_fromstr.unwrap(),u16_4_fromstr.unwrap(),u16_5_fromstr.unwrap(),u16_6_fromstr.unwrap(),u16_7_fromstr.unwrap(),u16_8_fromstr.unwrap()));
        let port = s
            .strip_suffix(')')
            .and_then(|s| s.split(":").skip(5).next())
            .ok_or(FullIpParseError)?;
        let port_fromstring = port.parse::<u16>().map_err(|_| FullIpParseError)?;
        
        Ok(FullIp{address:ipadd, port:port_fromstring})
        

    }
}



impl FromStr for FullIp {
    type Err = FullIpParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ipv4_result = FullIp::from_str_ipv4(s);
        let full_ip_str = match ipv4_result {
            Ok(fullip) => fullip,
            Err(_error) => match FullIp::from_str_ipv6(s) {
                Ok(ip) => ip,
                Err(e) => panic!("Cannot create string!: {e:?}")
            }
        };
        Ok(full_ip_str)
        /*
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

        let ipadd = IpAdd(V4(Ipv4(u8_1_fromstr,u8_2_fromstr,u8_3_fromstr,u8_4_fromstr)));
        let (_wrong, port) = s
            .strip_suffix(')')
            .and_then(|s| s.split_once(":"))
            .ok_or(Ipv4ParseError)?;
        let port_fromstring = port.parse::<u16>().map_err(|_| Ipv4ParseError)?;
        
        Ok(FullIp{address:ipadd, port:port_fromstring})
        
        */
    }
}

impl fmt::Display for FullIp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.address {
            IpAdd::V4(Ip_v4) => write!(f, "({}.{}.{}.{}:{})", Ip_v4.0, Ip_v4.1, Ip_v4.2, Ip_v4.3, self.port),
            IpAdd::V6(Ip_v6) => write!(f, "([{}:{}:{}:{}:{}:{}:{}:{}]:{})", Ip_v6.0, Ip_v6.1, Ip_v6.2, Ip_v6.3, Ip_v6.4, Ip_v6.5, Ip_v6.6, Ip_v6.7, self.port)
        }
    }
}

impl FullIp {
    fn connect_format(&self) -> String { 
        return format!("{}", self).strip_prefix('(').expect("prefix").strip_suffix(')').expect("suffix").to_string();
    }
}

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    
    println!("   __  ___                     _        
  /  |/  /__ __________ ______(_)_ _____
 / /|_/ / -_) __/ __/ // / __/ / // (_-<
/_/  /_/\\__/_/  \\__/\\_,_/_/ /_/\\_,_/___/
                                        ");
    println!("Ver 1.0");
    println!("Live chat only");
    
    let mut local_ip = String::new();
    print!("Enter IP and port you would like connections to go to (ex:(127.0.0.1:7878) or ([2001:0db8:0:0:0:8a2e:0370:7334]:7878) ):");
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
            from: IpAdd::V4(Ipv4::from_str(&(format!("({})", stream.local_addr().unwrap().ip()))).unwrap()),
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipv4_support() {
        let _connection = TcpListener::bind(FullIp::connect_format(&(FullIp::from_str("(127.0.0.1:7878)").unwrap()))).expect("Ipv4 support failed!");
    }
    
    #[test]
    fn ipv6_support() {
        let _connection = TcpListener::bind(FullIp::connect_format(&(FullIp::from_str("([2001:0db8:0:0:0:8a2e:0370:7334]:7878)").unwrap()))).expect("Ipv6 support failed!");
    }
}
