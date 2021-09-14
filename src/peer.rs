use crate::prelude::*;

pub const INPORT: u16 = 36108;
pub const OUTPORT: u16 = 36107; //these are just different for testing, for actual use they should be merged

#[derive(Clone,Serialize,Deserialize)]
pub struct Peer {
	pub key: PublicKey,
	pub ip: IpAddr,
	//#[serde(serialize_with="se_instant",deserialize_with="de_instant")]
	pub last_online: SystemTime,
	#[serde(skip)]
	pub relative_key: [u8; 32],
}

impl Peer {
	pub fn punch(&self, socket: &UdpSocket) -> Result<()> {
		socket.send_to(&[], (self.ip, OUTPORT))?;
		Ok(())
	}
	
	pub fn send(&self, socket: &UdpSocket, packet: &Packet, key: &PublicKey) -> Result<()> {
		let bytes = packet.to_bytes(key);
		socket.send_to(&bytes, (self.ip, OUTPORT))?;
		Ok(())
	}
}

#[derive(Clone,Serialize,Deserialize)]
pub struct Friend {
	pub key: PublicKey,
	pub name: String,
	#[serde(skip)]
	pub relative_key: [u8; 32],
}

/*impl Friend {
	pub fn confirm_recved(&self, _hash: [u8; HASH_BYTES], _socket: &UdpSocket) {
		unimplemented!()
	}
	
	pub fn confirm_seen(&self, _hash: [u8; HASH_BYTES], _socket: &UdpSocket) {
		unimplemented!()	
	}
}*/
