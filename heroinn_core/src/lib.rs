use std::io::*;
use std::net::SocketAddr;

use heroinn_util::{*, protocol::{Server} , protocol::tcp::*, packet::Message};

pub struct HeroinnServer{
    tcp_server : Option<TcpServer>,
    protocol : HeroinnProtocol
}

impl HeroinnServer{

    fn cb_connection<CB: 'static + Fn(Message) + Send + Copy>(
        proto : HeroinnProtocol,
        data : Vec<u8>,
        peer_addr : SocketAddr,
        cb : CB ){
        
        let id = HeroinnClientMsgID::from(data[0]);

        match id{
            HeroinnClientMsgID::HostInfo => {
                let msg = Message::new(peer_addr , proto , &data).unwrap();
                cb(msg);
            },
            HeroinnClientMsgID::Unknow => {},
            HeroinnClientMsgID::Heartbeat => {
                let msg = Message::new(peer_addr , proto , &data).unwrap();
                cb(msg);
            },
        }
    }

    pub fn new<CB: 'static + Fn(Message) + Send + Copy>(
        protocol : HeroinnProtocol , 
        port : u16 , 
        cb_msg : CB) -> std::io::Result<Self>{
        match protocol{
            HeroinnProtocol::TCP => {
                match TcpServer::new(format!("0.0.0.0:{}" , port).as_str() , HeroinnServer::cb_connection , cb_msg){
                    Ok(tcp_server) => return Ok(Self{tcp_server: Some(tcp_server) , protocol}),
                    Err(e) => return Err(e),
                }
            },
        }
    }
    
    pub fn sendto(&mut self, peer_addr : SocketAddr, buf : & [u8]) -> Result<()>{
        match self.protocol{
            HeroinnProtocol::TCP => self.tcp_server.as_mut().unwrap().sendto(peer_addr, buf),
        }
    }

    pub fn local_addr(&self) -> Result<SocketAddr>{
        match self.protocol{
            HeroinnProtocol::TCP => self.tcp_server.as_ref().unwrap().local_addr(),
        }
    }

    pub fn proto(&self) -> HeroinnProtocol{
        self.protocol.clone()
    }

    pub fn close(&mut self){
        match self.protocol{
            HeroinnProtocol::TCP => self.tcp_server.as_mut().unwrap().close(),
        }
    }
}