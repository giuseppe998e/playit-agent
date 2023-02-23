pub mod dec;
pub mod en;

#[cfg(test)]
mod en_dec {
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    use std::{
        io::Cursor,
        mem::size_of,
        net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4},
    };

    use rand::random;

    use super::{dec::AsyncMessageDecode, en::AsyncMessageEncode};
    use crate::{
        agent::AgentSession,
        control::{
            ControlRequest, ControlResponse, KeepAliveRequest, Ping, Pong, PortMappingFound,
            PortMappingRequest, PortMappingResponse, RegisterRequest, RegisterResponse,
            UdpChannelResponse,
        },
        hmac::HmacSign,
        socket::{Port, Protocol, Socket},
    };

    #[test]
    fn test_agentsession() {
        let mut buf = Vec::<u8>::with_capacity(size_of::<AgentSession>());
        let data = AgentSession {
            session_id: random(),
            account_id: random(),
            agent_id: random(),
        };

        // Encode
        assert!(matches!(aw!(data.clone().write_into(&mut buf)), Ok(_)));

        // Decode
        let mut buf_cursor = Cursor::new(buf);
        let dec_result = aw!(AgentSession::read_from(&mut buf_cursor));
        assert_eq!(data, dec_result.unwrap())
    }

    #[test]
    fn test_controlrequest() {
        let mut buf = Vec::<u8>::with_capacity(size_of::<ControlRequest>());
        let data = ControlRequest::KeepAlive(KeepAliveRequest {
            session_id: random(),
            account_id: random(),
            agent_id: random(),
        });

        // Encode
        assert!(matches!(aw!(data.clone().write_into(&mut buf)), Ok(_)));

        // Decode
        let mut buf_cursor = Cursor::new(buf);
        let dec_result = aw!(ControlRequest::read_from(&mut buf_cursor));
        assert_eq!(data, dec_result.unwrap())
    }

    #[test]
    fn test_controlresponse() {
        let mut buf = Vec::<u8>::with_capacity(size_of::<ControlResponse>());
        let data = ControlResponse::RequestQueued;

        // Encode
        assert!(matches!(aw!(data.clone().write_into(&mut buf)), Ok(_)));

        // Decode
        let mut buf_cursor = Cursor::new(buf);
        let dec_result = aw!(ControlResponse::read_from(&mut buf_cursor));
        assert_eq!(data, dec_result.unwrap())
    }

    #[test]
    fn test_control_ping() {
        let mut buf = Vec::<u8>::with_capacity(size_of::<Ping>());
        let data = Ping {
            now: random(),
            session: None,
        };

        // Encode
        assert!(matches!(aw!(data.clone().write_into(&mut buf)), Ok(_)));

        // Decode
        let mut buf_cursor = Cursor::new(buf);
        let dec_result = aw!(Ping::read_from(&mut buf_cursor));
        assert_eq!(data, dec_result.unwrap())
    }

    #[test]
    fn test_control_pong() {
        let mut buf = Vec::<u8>::with_capacity(size_of::<Pong>());
        let data = Pong {
            request_now: random(),
            server_now: random(),
            server_id: random(),
            data_center_id: random(),
            client_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, random())),
            tunnel_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::BROADCAST, random())),
            session_expire_at: random(),
        };

        // Encode
        assert!(matches!(aw!(data.clone().write_into(&mut buf)), Ok(_)));

        // Decode
        let mut buf_cursor = Cursor::new(buf);
        let dec_result = aw!(Pong::read_from(&mut buf_cursor));
        assert_eq!(data, dec_result.unwrap())
    }

    #[test]
    fn test_control_portmappingrequest() {
        let mut buf = Vec::<u8>::with_capacity(size_of::<PortMappingRequest>());
        let data = PortMappingRequest {
            session: AgentSession {
                session_id: random(),
                account_id: random(),
                agent_id: random(),
            },
            socket: Socket {
                ip: IpAddr::V4(Ipv4Addr::LOCALHOST),
                port: Port::Single(random()),
                proto: Protocol::Tcp,
            },
        };

        // Encode
        assert!(matches!(aw!(data.clone().write_into(&mut buf)), Ok(_)));

        // Decode
        let mut buf_cursor = Cursor::new(buf);
        let dec_result = aw!(PortMappingRequest::read_from(&mut buf_cursor));
        assert_eq!(data, dec_result.unwrap())
    }

    #[test]
    fn test_control_portmappingresponse() {
        let rand = random();

        let mut buf = Vec::<u8>::with_capacity(size_of::<PortMappingResponse>());
        let data = PortMappingResponse {
            socket: Socket {
                ip: IpAddr::V4(Ipv4Addr::BROADCAST),
                port: Port::Range(rand..=rand + 1),
                proto: Protocol::Udp,
            },
            found: None,
        };

        // Encode
        assert!(matches!(aw!(data.clone().write_into(&mut buf)), Ok(_)));

        // Decode
        let mut buf_cursor = Cursor::new(buf);
        let dec_result = aw!(PortMappingResponse::read_from(&mut buf_cursor));
        assert_eq!(data, dec_result.unwrap())
    }

    #[test]
    fn test_control_portmappingfound() {
        let mut buf = Vec::<u8>::with_capacity(size_of::<PortMappingFound>());
        let data = PortMappingFound::ToAgent(AgentSession {
            session_id: random(),
            account_id: random(),
            agent_id: random(),
        });

        // Encode
        assert!(matches!(aw!(data.clone().write_into(&mut buf)), Ok(_)));

        // Decode
        let mut buf_cursor = Cursor::new(buf);
        let dec_result = aw!(PortMappingFound::read_from(&mut buf_cursor));
        assert_eq!(data, dec_result.unwrap())
    }

    #[test]
    fn test_control_registerrequest() {
        let mut buf = Vec::<u8>::with_capacity(size_of::<RegisterRequest>());
        let data = RegisterRequest {
            account_id: random(),
            agent_id: random(),
            agent_version: random(),
            timestamp: random(),
            client_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, random())),
            tunnel_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::BROADCAST, random())),
            signature: Default::default(),
        };

        // Encode
        assert!(matches!(aw!(data.clone().write_into(&mut buf)), Ok(_)));

        // Decode
        let mut buf_cursor = Cursor::new(buf);
        let dec_result = aw!(RegisterRequest::read_from(&mut buf_cursor));
        assert_eq!(data, dec_result.unwrap())
    }

    #[test]
    fn test_control_registerresponse() {
        let mut buf = Vec::<u8>::with_capacity(size_of::<RegisterResponse>());
        let data = RegisterResponse {
            session: AgentSession {
                session_id: random(),
                account_id: random(),
                agent_id: random(),
            },
            expires_at: random(),
        };

        // Encode
        assert!(matches!(aw!(data.clone().write_into(&mut buf)), Ok(_)));

        // Decode
        let mut buf_cursor = Cursor::new(buf);
        let dec_result = aw!(RegisterResponse::read_from(&mut buf_cursor));
        assert_eq!(data, dec_result.unwrap())
    }

    #[test]
    fn test_control_udpchannelresponse() {
        let mut buf = Vec::<u8>::with_capacity(size_of::<UdpChannelResponse>());
        let data = UdpChannelResponse {
            tunnel_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, random())),
            token: Default::default(),
        };

        // Encode
        assert!(matches!(aw!(data.clone().write_into(&mut buf)), Ok(_)));

        // Decode
        let mut buf_cursor = Cursor::new(buf);
        let dec_result = aw!(UdpChannelResponse::read_from(&mut buf_cursor));
        assert_eq!(data, dec_result.unwrap())
    }

    // #[test]
    // fn test_control_udpchannelrequest() {
    //      // NOT_NEEDED alias of "AgentSession"
    // }

    // TODO randomic data
    #[test]
    fn test_hmacsign() {
        let mut buf = Vec::<u8>::with_capacity(size_of::<HmacSign<sha2::Sha256>>());
        let data = HmacSign::<sha2::Sha256>::default();

        // Encode
        assert!(matches!(aw!(data.clone().write_into(&mut buf)), Ok(_)));

        // Decode
        let mut buf_cursor = Cursor::new(buf);
        let dec_result = aw!(HmacSign::<sha2::Sha256>::read_from(&mut buf_cursor));
        assert_eq!(data, dec_result.unwrap())
    }

    #[test]
    fn test_socket() {
        let mut buf = Vec::<u8>::with_capacity(size_of::<Socket>());
        let data = Socket {
            ip: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            port: Port::Single(random()),
            proto: Protocol::Both,
        };

        // Encode
        assert!(matches!(aw!(data.clone().write_into(&mut buf)), Ok(_)));

        // Decode
        let mut buf_cursor = Cursor::new(buf);
        let dec_result = aw!(Socket::read_from(&mut buf_cursor));
        assert_eq!(data, dec_result.unwrap())
    }

    #[test]
    fn test_socket_port() {
        let mut buf = Vec::<u8>::with_capacity(size_of::<Port>());
        let data = Port::Single(random());

        // Encode
        assert!(matches!(aw!(data.clone().write_into(&mut buf)), Ok(_)));

        // Decode
        let mut buf_cursor = Cursor::new(buf);
        let dec_result = aw!(Port::read_from(&mut buf_cursor));
        assert_eq!(data, dec_result.unwrap())
    }

    #[test]
    fn test_socket_proto() {
        let mut buf = Vec::<u8>::with_capacity(size_of::<Protocol>());
        let data = Protocol::Both;

        // Encode
        assert!(matches!(aw!(data.clone().write_into(&mut buf)), Ok(_)));

        // Decode
        let mut buf_cursor = Cursor::new(buf);
        let dec_result = aw!(Protocol::read_from(&mut buf_cursor));
        assert_eq!(data, dec_result.unwrap())
    }
}
