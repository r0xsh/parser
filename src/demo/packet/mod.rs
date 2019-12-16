use bitstream_reader::BitRead;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

use crate::{Parse, ParserState, Result, Stream};

use self::consolecmd::ConsoleCmdPacket;
use self::datatable::DataTablePacket;
use self::message::MessagePacket;
use self::stop::StopPacket;
use self::stringtable::StringTablePacket;
use self::synctick::SyncTickPacket;
use self::usercmd::UserCmdPacket;

pub mod consolecmd;
pub mod datatable;
pub mod message;
pub mod stop;
pub mod stringtable;
pub mod synctick;
pub mod usercmd;

#[derive(Debug)]
pub enum Packet {
    Sigon(MessagePacket),
    Message(MessagePacket),
    SyncTick(SyncTickPacket),
    ConsoleCmd(ConsoleCmdPacket),
    UserCmd(UserCmdPacket),
    DataTables(DataTablePacket),
    Stop(StopPacket),
    StringTables(StringTablePacket),
}

#[derive(BitRead, TryFromPrimitive, Debug)]
#[discriminant_bits = 8]
#[repr(u8)]
pub enum PacketType {
    Sigon = 1,
    Message = 2,
    SyncTick = 3,
    ConsoleCmd = 4,
    UserCmd = 5,
    DataTables = 6,
    Stop = 7,
    StringTables = 8,
}

impl Parse for Packet {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let packet_type = PacketType::read(stream)?;
        Ok(match packet_type {
            PacketType::Sigon => Packet::Sigon(MessagePacket::parse(stream, state)?),
            PacketType::Message => Packet::Message(MessagePacket::parse(stream, state)?),
            PacketType::SyncTick => Packet::SyncTick(SyncTickPacket::parse(stream, state)?),
            PacketType::ConsoleCmd => Packet::ConsoleCmd(ConsoleCmdPacket::parse(stream, state)?),
            PacketType::UserCmd => Packet::UserCmd(UserCmdPacket::parse(stream, state)?),
            PacketType::DataTables => Packet::DataTables(DataTablePacket::parse(stream, state)?),
            PacketType::Stop => Packet::Stop(StopPacket::parse(stream, state)?),
            PacketType::StringTables => {
                Packet::StringTables(StringTablePacket::parse(stream, state)?)
            }
        })
    }
}
