extern crate ipnetwork;
extern crate pnet;
extern crate zen;

use ipnetwork::IpNetwork;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use std::thread;
use zen::ether::Data;
use zen::net::Serializable;
use zen::*;

fn main() {

    let interface= net::default_interface();
    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("unhandled channel type"),
        Err(e) => panic!("An error occurred when creating the datalink channel {}", e),
    };


    let sender_mac = zen::net::MacAddr::local();
    let sender_ip = zen::net::IpAddr::local();

    let local_mac = sender_mac.clone();
    let handle = thread::spawn(move || loop {
        match rx.next() {
            Ok(packet) => {
                if packet[12] == 0x08 && packet[13] == 0x06 {
                    let packet_receive = ether::Packet::parse(&packet.to_vec());
                    match packet_receive {
                        Ok(ref p) => {
                            if p.dest_addr == local_mac {
                                eprintln!("p = {}", p);
                            }
                        }
                        Err(e) => eprintln!("e = {}", e),
                    }
                }
            }
            Err(e) => eprintln!("e = {:#?}", e),
        }
    });

    for i in 1..255 {
        let target_mac = net::MacAddr(0xff, 0xff, 0xff, 0xff, 0xff, 0xff);
        let target_ip = net::IpAddr(192, 168, 0, i as u8);

        let mut arp_packet = arp::Packet::new();
        arp_packet.set_operation(arp::Operation::Request);
        arp_packet.set_sender_mac(sender_mac.clone());
        arp_packet.set_sender_ip(sender_ip.clone());
        arp_packet.set_target_mac(target_mac);
        arp_packet.set_target_ip(target_ip);

        let mut ether_packet = ether::Packet::new();
        ether_packet.set_dest_addr(net::MacAddr(0xff, 0xff, 0xff, 0xff, 0xff, 0xff));
        ether_packet.set_src_addr(sender_mac.clone());
        ether_packet.set_packet_content(Data::ARP(arp_packet));
        let packet = ether_packet.to_hex();

        tx.send_to(&packet, None);
    }

    handle.join();
}
