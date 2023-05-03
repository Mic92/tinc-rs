pub(crate) const PROT_MAJOR : u16 = 17;
pub(crate) const PROT_MINOR : u8 = 7;

/// Request numbers 
pub(crate) enum Request {
    // Guardian for allow_request
    All = -1,
    Id = 0,
    Metakey,
    Challenge,
    ChalReply,
    Ack,
    Status,
    Error,
    Termreq,
    Ping,
    Pong,
    AddSubnet,
    DelSubnet,
    AddEdge,
    DelEdge,
    KeyChanged,
    ReqKey,
    AnsKey,
    Packet,
    Control,
    ReqPubkey,
    AnsPubkey,
    SptpsPacket,
    UdpInfo,
    MtuInfo,
    /// Guardian for the highest request number
    Last,
}
