use std::net::TcpStream;
//use std::io::{Error, Write, Read};

pub enum MessageType {
	CONNECT = 1,
	CONNACK = 2,
	PUBLISH = 3,
	PUBACK = 4,
	PUBREC = 5,
	PUBREL = 6,
	PUBCOMP = 7,
	SUBSCRIBE = 8,
	SUBACK = 9,
	UNSUBSCRIBE = 10,
	UNSUBACK = 11,
	PINGREQ = 12,
	PINGRESP = 13,
	DISCONNECT = 14
}

#[derive(Default)]
#[derive(Debug)]
pub struct Client<'a, 'b, 'c>{
	pub host: Option<&'a str>,
	pub id: String,
	pub user_name: Option<&'b str>,
	pub password: Option<&'c str>,
	pub keep_alive: u16,
	pub clean: bool,
	pub stream: Option<TcpStream>,
}



impl<'a, 'b, 'c> Client<'a, 'b, 'c>{

	pub fn new(id: &'a str) -> Self{
		Client{
			id: id.to_string(), 
			clean: true, 
			keep_alive: 5, 
			..Default::default()
		}
	}

	pub fn host(mut self, host: &'a str) -> Self{
		self.host = Some(host);
		self
	}

	pub fn username(mut self, user_name: &'b str) -> Self{
		self.user_name = Some(user_name);
		self
	}
}

pub fn test(){
	println!("{:?}", "Hello Mqtt");
	let client = Client::new("rtr").username("mosquitto");
	println!("{:?}", client);
}