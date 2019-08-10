use std::collections::HashMap;

use crate::demo::gamevent::GameEventDefinition;
use crate::demo::message::gameevent::GameEventTypeId;
use crate::demo::message::packetentities::EntityId;
use crate::demo::message::stringtable::StringTableMeta;
use crate::demo::message::{Message, MessageType};
use crate::demo::packet::datatable::{SendTable, SendTableName, ServerClass};
use crate::demo::packet::stringtable::StringTableEntry;
use crate::demo::parser::analyser::Analyser;
use crate::demo::parser::handler::MessageHandler;
use crate::demo::sendprop::SendProp;
use crate::Stream;

#[derive(Default)]
pub struct DemoMeta {
    pub version: u16,
    pub game: String,
    pub interval_per_tick: f32,
}

pub struct ParserState {
    pub static_baselines: HashMap<u32, StaticBaseline>,
    pub event_definitions: HashMap<GameEventTypeId, GameEventDefinition>,
    pub string_tables: Vec<StringTableMeta>,
    pub entity_classes: HashMap<EntityId, ServerClass>,
    pub send_tables: HashMap<SendTableName, SendTable>,
    pub server_classes: Vec<ServerClass>,
    pub instance_baselines: [HashMap<EntityId, Vec<SendProp>>; 2],
    pub demo_meta: DemoMeta,
    analyser_handles: fn(message_type: MessageType) -> bool,
}

pub struct StaticBaseline {
    class_id: u32,
    raw: Stream,
    parsed: Option<Vec<SendProp>>,
}

impl StaticBaseline {
    fn new(class_id: u32, raw: Stream) -> Self {
        StaticBaseline {
            class_id,
            raw,
            parsed: None,
        }
    }
}

impl ParserState {
    pub fn new(analyser_handles: fn(message_type: MessageType) -> bool) -> Self {
        ParserState {
            static_baselines: HashMap::new(),
            event_definitions: HashMap::new(),
            string_tables: Vec::new(),
            entity_classes: HashMap::new(),
            send_tables: HashMap::new(),
            server_classes: Vec::new(),
            instance_baselines: [HashMap::new(), HashMap::new()],
            demo_meta: DemoMeta::default(),
            analyser_handles,
        }
    }

    pub fn handle_data_table(
        &mut self,
        send_tables: Vec<SendTable>,
        server_classes: Vec<ServerClass>,
    ) {
        for table in send_tables {
            self.send_tables.insert(table.name.clone(), table);
        }
        self.server_classes = server_classes
    }

    pub fn handle_string_table_meta(&mut self, table: StringTableMeta) {
        self.string_tables.push(table);
    }

    pub fn should_parse_message(&self, message_type: MessageType) -> bool {
        Self::does_handle(message_type) || (self.analyser_handles)(message_type)
    }
}

impl MessageHandler for ParserState {
    type Output = Self;

    fn does_handle(message_type: MessageType) -> bool {
        match message_type {
            MessageType::ServerInfo
            | MessageType::GameEventList
            | MessageType::CreateStringTable
            | MessageType::UpdateStringTable => true,
            _ => false,
        }
    }

    fn handle_message(&mut self, message: Message, _tick: u32) {
        match message {
            Message::ServerInfo(message) => {
                self.demo_meta.version = message.version;
                self.demo_meta.game = message.game;
                self.demo_meta.interval_per_tick = message.interval_per_tick
            }
            Message::GameEventList(message) => {
                self.event_definitions = message.event_list;
            }
            _ => {}
        }
    }

    fn handle_string_entry(&mut self, table: &String, _index: usize, entry: &StringTableEntry) {
        match table.as_str() {
            "instancebaseline" => match &entry.extra_data {
                Some(extra) => match entry.text().parse::<u32>() {
                    Ok(class_id) => {
                        let baseline = StaticBaseline::new(class_id, extra.data.clone());
                        self.static_baselines.insert(class_id, baseline);
                    }
                    Err(_) => {}
                },
                _ => unreachable!("missing baseline"),
            },
            _ => {}
        }
    }

    fn get_output(self, _state: ParserState) -> Self {
        self
    }
}
