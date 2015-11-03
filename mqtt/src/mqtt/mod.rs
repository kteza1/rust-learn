use std::net::TcpStream;
use std::io::{Error, Write, Read};

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
pub struct Client<'a>{
	pub host: &'a str,
	pub id: &'a str,
	pub user_name: &'a str,
	pub password: &'a str,
	pub keep_alive: u16,
	pub clean: bool,
	pub stream: Option<TcpStream>,
}

pub fn new(id: &str) -> Client{
	Client{host: "test.mosquitto.org", id: id, clean: true, keep_alive: 5, ..Default::default()}
}

impl<'a> Client<'a>{
	pub fn username(&'a mut self, user_name: &'a str) -> &'a mut Client{
		self.user_name = user_name;
		self
	}
}

pub fn test(){
	println!("{:?}", "Hello Mqtt");
	let client = self::new("rtr").username("openwrt");
	println!("{:?}", client);
}