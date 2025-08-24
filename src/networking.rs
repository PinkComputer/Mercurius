

pub mod networking {
    // use chrono::{NaiveDate, NaiveDateTime};

    pub use std::{
        fmt,
        str::FromStr,
        str,
        //env,
    };

    //At the time I was unaware there was already an IP address type and I was too prideful to switch
    #[derive(Clone)]
    pub struct Ipv4 (u8,u8,u8,u8);

    //Creates the 
    impl fmt::Display for Ipv4 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}.{}.{}.{})", self.0, self.1, self.2, self.3)
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Ipv4ParseError; 
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
                .and_then(|s| s.strip_suffix(')')).map(|s| s.split('.'))
                .ok_or(Ipv4ParseError)?;

            if iter.clone().count() <= 1usize {
                return Err(Ipv4ParseError);
            }

            //The iterator is started/moved, unwrapped into a string, parsed into u8, and then unwrapped
            //again. Is this a good way to do it? Prolly not
            let u8_1_fromstr = iter.next().unwrap().parse::<u8>();
            let u8_2_fromstr = iter.next().unwrap().parse::<u8>();
            let u8_3_fromstr = iter.next().unwrap().parse::<u8>();
            let u8_4_fromstr = iter.next().expect("Failed at u8_4").parse::<u8>();
            
            
            if u8_1_fromstr.is_err() {
                return Err(Ipv4ParseError)
            }
             

            Ok(Ipv4(u8_1_fromstr.unwrap(),u8_2_fromstr.unwrap(),u8_3_fromstr.unwrap(),u8_4_fromstr.unwrap()))
         

        }
    }

    // May God strike you down FP.
    #[derive(Clone)]
    pub struct Ipv6 (u16,u16,u16,u16,u16,u16,u16,u16);

    impl fmt::Display for Ipv6 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x})", self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7)
        }
    }


    #[derive(Debug, PartialEq, Eq)]
    pub struct Ipv6ParseError; 
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
                .and_then(|s| s.strip_suffix(')')).map(|s| s.split(':'))
                .ok_or(Ipv6ParseError)?;
            //The iterator is started/moved, unwrapped into a string, parsed into u16, and then unwrapped
            //again. Is this a good way to do it? Prolly not
            //
            //([2603:6011:f73a:f397:c5f0:19d8:4c13:6a8b])
            let u16_1_fromstr = u16::from_str_radix(iter.next().unwrap(),16); 
            let u16_2_fromstr = u16::from_str_radix(iter.next().unwrap(),16);
            let u16_3_fromstr = u16::from_str_radix(iter.next().unwrap(),16);
            let u16_4_fromstr = u16::from_str_radix(iter.next().unwrap(),16); 
            let u16_5_fromstr = u16::from_str_radix(iter.next().unwrap(),16);
            let u16_6_fromstr = u16::from_str_radix(iter.next().unwrap(),16);
            let u16_7_fromstr = u16::from_str_radix(iter.next().unwrap(),16);
            let u16_8_fromstr = u16::from_str_radix(iter.next().unwrap(),16);
            
            /*
            if u16_1_fromstr.is_err() {
                return Err(FullIpParseError)
            }
            */




            Ok(Ipv6(u16_1_fromstr.unwrap(),u16_2_fromstr.unwrap(),u16_3_fromstr.unwrap(),u16_4_fromstr.unwrap(),u16_5_fromstr.unwrap(),u16_6_fromstr.unwrap(),u16_7_fromstr.unwrap(),u16_8_fromstr.unwrap()))
         

        }
    }




    pub struct Message {
        pub to: IpAdd,
        pub from: IpAdd,
        pub from_public: u8,
        pub to_public: u8,
        pub alias: String,
        pub message: String, 
       }

    impl Message {
        pub fn as_string(&self) -> String {
            format!("To: {0}, \n
    From:{1}, \n
    From_public:{2}, \n
    To_public:{3}, \n
    Alias:{4}, \n
    Message:{5} ",
    self.to, self.from, self.from_public, self.to_public, self.alias, self.message,)
        }
    }

    impl Message {
        pub fn pretty_print(&self) -> String {
            format!("{0}: {1}",self.alias, self.message,)
        }
    }


    impl fmt::Display for Message {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", Message::pretty_print(self))
        }
    }

    impl Message {
        //This has prolly already taken an hour but I am gonna just say development started at 8:55 PM
        //5/17/2025, time spent: 01:11:20~
        //That is the time needed to fix the single error, god help me
        pub fn from_request(s: &str) -> Message {
           
           let mut iter = s.split(",");

           let line = iter.next();
           let line_iter = line.expect("Iter is empty!").to_string().split_once(":").expect("Spliting failed!").1.to_owned();
           let to_ip_request  = IpAdd::from_str(line_iter.to_string().trim()).unwrap();

           let line = iter.next();
           let binding = line.expect("Iter is empty!").to_string();
           let line_iter = binding.split_once(":").expect("Spliting failed!").1;
           let from_ip_request  = IpAdd::from_str(line_iter.to_string().trim()).unwrap();

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
           let alias_request:String = line_iter.to_string();

           let line = iter.next();
           let binding = line.expect("Iter is empty!").to_string();
           let line_iter = binding.split_once(":").expect("Spliting failed!").1;
           let message_request:String = line_iter.to_string();

           Message {
               to: to_ip_request,
               from: from_ip_request,
               from_public: from_key_request,
               to_public: to_key_request,
               alias: alias_request,
               message: message_request
            }
        }
    }
    #[derive(Clone)]
    pub enum IpAdd {
        V4(Ipv4),
        V6(Ipv6)
    }

    impl fmt::Display for IpAdd {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self {
                IpAdd::V4(ip_v4) => write!(f, "({}.{}.{}.{})", ip_v4.0, ip_v4.1, ip_v4.2, ip_v4.3),
                IpAdd::V6(ip_v6) => write!(f, "({:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x})", ip_v6.0, ip_v6.1, ip_v6.2, ip_v6.3, ip_v6.4, ip_v6.5, ip_v6.6, ip_v6.7)
            }
        }
    }

    #[derive(Debug)]
    pub struct IpAddError; 
    impl fmt::Display for IpAddError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Invaild address: {}", (format!("{}", self)))
        }
    }


    impl IpAdd {
        pub fn from_str(s: &str) -> Result <Self, IpAddError> {
            let ipv4_result = Ipv4::from_str(s);
            let ipadd_str = match ipv4_result {
                Ok(ipadd_str) => IpAdd::V4(ipadd_str),
                Err(_error) => match Ipv6::from_str(s) {
                    Ok(ip) => IpAdd::V6(ip),
                    Err(e) => panic!("Cannot create string!: {e:?}")
                }
            };
            Ok(ipadd_str)
        }
    }


    #[derive(Clone)]
    pub struct FullIp {
        pub address: IpAdd,
        pub port: u16,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct FullIpParseError; 
    impl fmt::Display for FullIpParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Invaild address: {}", (format!("{}", self)))
        }
    }

    impl FullIp {
        pub fn from_str_ipv4(s: &str) -> Result<Self, <FullIp as FromStr>::Err>  {
            let mut iter = s
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')')).map(|s| s.split('.'))
                .ok_or(FullIpParseError)?;
            //The iterator is started/moved, unwrapped into a string, parsed into u8, and then unwrapped
            //again. Is this a good way to do it? Prolly not

            if iter.clone().count() <= 1usize {
                return Err(FullIpParseError);
            }

            let u8_1_fromstr = iter.next().unwrap().parse::<u8>();
            let u8_2_fromstr = iter.next().unwrap().parse::<u8>();
            let u8_3_fromstr = iter.next().unwrap().parse::<u8>();
            let u8_4_fromstr = iter.next().expect("Failed at u8_4").split_once(":").unwrap().0.parse();
            
            
            if u8_1_fromstr.is_err() {
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

        pub fn from_str_ipv6(s: &str) -> Result<Self, <FullIp as FromStr>::Err>  {
            // Example Ipv6 with port: 
            // ([2001:0db8:0000:0000:0000:8a2e:0370:7334]:7878)
            let mut iter = s
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .and_then(|s| s.strip_prefix('[')).map(|s| s.split(':'))
                .ok_or(FullIpParseError)?;
            //The iterator is started/moved, unwrapped into a string, parsed into u8, and then unwrapped
            //again. Is this a good way to do it? Prolly not
            let u16_1_fromstr = u16::from_str_radix(iter.next().unwrap(),16); 
            let u16_2_fromstr = u16::from_str_radix(iter.next().unwrap(),16);
            let u16_3_fromstr = u16::from_str_radix(iter.next().unwrap(),16);
            let u16_4_fromstr = u16::from_str_radix(iter.next().unwrap(),16); 
            let u16_5_fromstr = u16::from_str_radix(iter.next().unwrap(),16);
            let u16_6_fromstr = u16::from_str_radix(iter.next().unwrap(),16);
            let u16_7_fromstr = u16::from_str_radix(iter.next().unwrap(),16);
            let u16_8_fromstr = u16::from_str_radix(iter.next().expect("Failed at u16_8").strip_suffix(']').unwrap(),16);
            
            /*
            if u16_1_fromstr.is_err() {
                return Err(FullIpParseError)
            }
            */
            
            let ipadd = IpAdd::V6(Ipv6(u16_1_fromstr.unwrap(),u16_2_fromstr.unwrap(),u16_3_fromstr.unwrap(),u16_4_fromstr.unwrap(),u16_5_fromstr.unwrap(),u16_6_fromstr.unwrap(),u16_7_fromstr.unwrap(),u16_8_fromstr.unwrap()));

            //println!("{}", s);

            let port = s
                .strip_suffix(')')
                .and_then(|s| s.split(":").nth(8));
                
            
            let port_fromstring = port.expect("Port not parsed correctly!").parse::<u16>().map_err(|_| FullIpParseError)?;
            
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
        }
    }

    impl fmt::Display for FullIp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.address {
                IpAdd::V4(ip_v4) => write!(f, "({}.{}.{}.{}:{})", ip_v4.0, ip_v4.1, ip_v4.2, ip_v4.3, self.port),
                IpAdd::V6(ip_v6) => write!(f, "([{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}]:{})", ip_v6.0, ip_v6.1, ip_v6.2, ip_v6.3, ip_v6.4, ip_v6.5, ip_v6.6, ip_v6.7, self.port)
            }
        }
    }

    impl FullIp {
        pub fn connect_format(&self) -> String { 
            format!("{}", self).strip_prefix('(').expect("prefix").strip_suffix(')').expect("suffix").to_string()
        }
    }
}
