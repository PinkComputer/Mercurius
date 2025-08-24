//TO DO: Actual packet writing
//       VOIP, maybe????
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    io,
    net::Shutdown,
    thread,
    //env,
    sync::mpsc,
    sync::mpsc::Sender,
};

mod networking;

use crate::networking::networking::*;

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
    print!("Enter IP and port you would like connections to go to (ex:(127.0.0.1:7878) or ([0:0:0:0:0:0:0:1]:7878)):");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut local_ip)
        .expect("Failed to read lines");
    
    

    let local_ip: FullIp = FullIp::from_str(local_ip.trim()).expect("Invalid address! Exiting...");
    let listener = TcpListener::bind(FullIp::connect_format(&local_ip)).expect("Listener could not be set up...");




    let mut remote_ip = String::new();
    print!("Enter IP and port you would like to connect to: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut remote_ip)
        .expect("Failed to read lines");

    
    let remote_ip: FullIp = FullIp::from_str(remote_ip.trim()).expect("Invalid address! Exiting...");

    //println!("{}", FullIp::connect_format(&remote_ip));
    
    let connection = TcpStream::connect(FullIp::connect_format(&remote_ip)).expect("Could not connect...");

    let mut alias = String::new();
    print!("What would you like your alias (username) to be? (Default is your IP): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut alias)
        .expect("Failed to read lines");
    
    if alias.trim() == "" {
        alias = format!("({})", connection.local_addr().unwrap().ip());
    } else {
        alias = alias.trim().to_string();
    }

    //connection.set_nonblocking(true).expect("set_nonblocking call failed");

    
    /*
    if let Ok(stream) = TcpStream::connect(FullIp::connect_format(&remote_ip)) {
        println!("Holy shit");
    } else {
        println!("Goddamn it");
    }
    */
    let (tx, rx) = mpsc::channel(); 
    let write_tx = tx.clone();
    let write_thread = thread::spawn( || {
        stream_writing(connection, remote_ip, alias, write_tx, local_ip);
    });


    let read_thread = thread::spawn( move || {
        let listener_copy = listener;
        tx.send(0).unwrap();
        for stream in listener_copy.incoming() {
            if rx.recv().unwrap() == 1 { 
                return
            }
            stream_reading(stream.unwrap());
        }
    });

    write_thread.join().unwrap();
    read_thread.join().unwrap();

    //let listener = TcpListener::bind("127.0.0.1:7879").unwrap();
    
    /*for stream in connection {
        //let stream = stream.unwrap();

        handle_connection(stream);
    
    }*/
    
}

fn stream_writing(mut stream: TcpStream, remote: FullIp, user_alias: String, channel: Sender<u16>, listener: FullIp) {
    //println!("Handling connection!");
    //let buf_reader = BufReader::new(&stream);
    /*let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    */

    println!("Enter messages (Enter nothing to exit): ");

    loop {
                
        print!("{user_alias}: ");
        io::stdout().flush().unwrap();
        let mut ent_message = String::new();

        io::stdin()
            .read_line(&mut ent_message)
            .expect("Failed to read line");

        if ent_message.trim() == ""{
            channel.send(1).unwrap();
            // Okay lemme explain this stupidity
            // Reader can't do the check for the shutdown signal until their is something in the
            // listener's buffer to actually read.
            // So we just send a space rq to force it to check for a shutdown signal
            let mut killer = TcpStream::connect(FullIp::connect_format(&listener)).expect("Could not connect...");
            killer.write(b" ");
            killer.shutdown(Shutdown::Both).expect("Reader killer shutdown failed?");
            stream.shutdown(Shutdown::Both).expect("Writing shutdown call failed?");
            return;
        }



        let response_message = Message {
            to: remote.address.clone(),
            from: (IpAdd::from_str(&(format!("({})", stream.local_addr().unwrap().ip()))).unwrap()),
            from_public: 1,
            to_public: 2,
            alias: user_alias.clone().trim().to_owned(),
            message: ent_message.trim().to_owned(),
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
            let _ = stream.write(line.to_string().as_bytes());
        }
        
        let _ = stream.shutdown(Shutdown::Write);

        stream = TcpStream::connect(FullIp::connect_format(&remote)).expect("Lost connection...");

        channel.send(0).unwrap();
        
     //stream.write_all(response_message.as_string().split("/n").next().unwrap().as_bytes());
     
    //println!("Request: {http_request:#?}");
    }
}


fn stream_reading(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let tcp_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    //println!("Request: {tcp_request:#?}");

    for message in tcp_request {

        println!("{}", Message::from_request(&message))
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ipv4_support() {
        let _listener = TcpListener::bind(FullIp::connect_format(&(FullIp::from_str("(127.0.0.1:7878)").unwrap()))).expect("Ipv4 support failed!");
    }
    
    #[test]
    fn ipv6_support() {
        let _listener = TcpListener::bind(FullIp::connect_format(&(FullIp::from_str("([0:0:0:0:0:0:0:1]:7878)").unwrap()))).expect("Ipv6 support failed!");
        let _connection = TcpStream::connect(FullIp::connect_format(&(FullIp::from_str("([0:0:0:0:0:0:0:1]:7878)").unwrap()))).expect("Ipv6 support failed!");
    }
}
