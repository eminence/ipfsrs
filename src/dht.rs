// This file is generated. Do not edit
// @generated

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Message {
    // message fields
    field_type: ::std::option::Option<Message_MessageType>,
    clusterLevelRaw: ::std::option::Option<i32>,
    key: ::protobuf::SingularField<::std::string::String>,
    record: ::protobuf::SingularPtrField<Record>,
    closerPeers: ::protobuf::RepeatedField<Message_Peer>,
    providerPeers: ::protobuf::RepeatedField<Message_Peer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Message {
    pub fn new() -> Message {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message {
        static mut instance: ::protobuf::lazy::Lazy<Message> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message,
        };
        unsafe {
            instance.get(|| {
                Message {
                    field_type: ::std::option::Option::None,
                    clusterLevelRaw: ::std::option::Option::None,
                    key: ::protobuf::SingularField::none(),
                    record: ::protobuf::SingularPtrField::none(),
                    closerPeers: ::protobuf::RepeatedField::new(),
                    providerPeers: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .dht.pb.Message.MessageType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Message_MessageType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> Message_MessageType {
        self.field_type.unwrap_or(Message_MessageType::PUT_VALUE)
    }

    // optional int32 clusterLevelRaw = 10;

    pub fn clear_clusterLevelRaw(&mut self) {
        self.clusterLevelRaw = ::std::option::Option::None;
    }

    pub fn has_clusterLevelRaw(&self) -> bool {
        self.clusterLevelRaw.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clusterLevelRaw(&mut self, v: i32) {
        self.clusterLevelRaw = ::std::option::Option::Some(v);
    }

    pub fn get_clusterLevelRaw<'a>(&self) -> i32 {
        self.clusterLevelRaw.unwrap_or(0)
    }

    // optional string key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .dht.pb.Record record = 3;

    pub fn clear_record(&mut self) {
        self.record.clear();
    }

    pub fn has_record(&self) -> bool {
        self.record.is_some()
    }

    // Param is passed by value, moved
    pub fn set_record(&mut self, v: Record) {
        self.record = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_record<'a>(&'a mut self) -> &'a mut Record {
        if self.record.is_none() {
            self.record.set_default();
        };
        self.record.as_mut().unwrap()
    }

    // Take field
    pub fn take_record(&mut self) -> Record {
        self.record.take().unwrap_or_else(|| Record::new())
    }

    pub fn get_record<'a>(&'a self) -> &'a Record {
        self.record.as_ref().unwrap_or_else(|| Record::default_instance())
    }

    // repeated .dht.pb.Message.Peer closerPeers = 8;

    pub fn clear_closerPeers(&mut self) {
        self.closerPeers.clear();
    }

    // Param is passed by value, moved
    pub fn set_closerPeers(&mut self, v: ::protobuf::RepeatedField<Message_Peer>) {
        self.closerPeers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_closerPeers<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Message_Peer> {
        &mut self.closerPeers
    }

    // Take field
    pub fn take_closerPeers(&mut self) -> ::protobuf::RepeatedField<Message_Peer> {
        ::std::mem::replace(&mut self.closerPeers, ::protobuf::RepeatedField::new())
    }

    pub fn get_closerPeers<'a>(&'a self) -> &'a [Message_Peer] {
        &self.closerPeers
    }

    // repeated .dht.pb.Message.Peer providerPeers = 9;

    pub fn clear_providerPeers(&mut self) {
        self.providerPeers.clear();
    }

    // Param is passed by value, moved
    pub fn set_providerPeers(&mut self, v: ::protobuf::RepeatedField<Message_Peer>) {
        self.providerPeers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_providerPeers<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Message_Peer> {
        &mut self.providerPeers
    }

    // Take field
    pub fn take_providerPeers(&mut self) -> ::protobuf::RepeatedField<Message_Peer> {
        ::std::mem::replace(&mut self.providerPeers, ::protobuf::RepeatedField::new())
    }

    pub fn get_providerPeers<'a>(&'a self) -> &'a [Message_Peer] {
        &self.providerPeers
    }
}

impl ::protobuf::Message for Message {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.clusterLevelRaw = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.key.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.record.set_default();
                    try!(is.merge_message(tmp))
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.closerPeers));
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.providerPeers));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.clusterLevelRaw.iter() {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.key.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.record.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.closerPeers.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.providerPeers.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.clusterLevelRaw {
            try!(os.write_int32(10, v));
        };
        if let Some(v) = self.key.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.record.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.closerPeers.iter() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.providerPeers.iter() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Message>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Message {
    fn new() -> Message {
        Message::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "field_type",
                    Message::has_field_type,
                    Message::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "clusterLevelRaw",
                    Message::has_clusterLevelRaw,
                    Message::get_clusterLevelRaw,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    Message::has_key,
                    Message::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "record",
                    Message::has_record,
                    Message::get_record,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "closerPeers",
                    Message::get_closerPeers,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "providerPeers",
                    Message::get_providerPeers,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message>(
                    "Message",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_clusterLevelRaw();
        self.clear_key();
        self.clear_record();
        self.clear_closerPeers();
        self.clear_providerPeers();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Message {
    fn eq(&self, other: &Message) -> bool {
        self.field_type == other.field_type &&
        self.clusterLevelRaw == other.clusterLevelRaw &&
        self.key == other.key &&
        self.record == other.record &&
        self.closerPeers == other.closerPeers &&
        self.providerPeers == other.providerPeers &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Message_Peer {
    // message fields
    id: ::protobuf::SingularField<::std::string::String>,
    addrs: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    connection: ::std::option::Option<Message_ConnectionType>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Message_Peer {
    pub fn new() -> Message_Peer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Message_Peer {
        static mut instance: ::protobuf::lazy::Lazy<Message_Peer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Message_Peer,
        };
        unsafe {
            instance.get(|| {
                Message_Peer {
                    id: ::protobuf::SingularField::none(),
                    addrs: ::protobuf::RepeatedField::new(),
                    connection: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        self.id.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_id<'a>(&'a self) -> &'a str {
        match self.id.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated bytes addrs = 2;

    pub fn clear_addrs(&mut self) {
        self.addrs.clear();
    }

    // Param is passed by value, moved
    pub fn set_addrs(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.addrs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_addrs<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.addrs
    }

    // Take field
    pub fn take_addrs(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.addrs, ::protobuf::RepeatedField::new())
    }

    pub fn get_addrs<'a>(&'a self) -> &'a [::std::vec::Vec<u8>] {
        &self.addrs
    }

    // optional .dht.pb.Message.ConnectionType connection = 3;

    pub fn clear_connection(&mut self) {
        self.connection = ::std::option::Option::None;
    }

    pub fn has_connection(&self) -> bool {
        self.connection.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connection(&mut self, v: Message_ConnectionType) {
        self.connection = ::std::option::Option::Some(v);
    }

    pub fn get_connection<'a>(&self) -> Message_ConnectionType {
        self.connection.unwrap_or(Message_ConnectionType::NOT_CONNECTED)
    }
}

impl ::protobuf::Message for Message_Peer {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.id.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.addrs));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.connection = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.id.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.addrs.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.connection.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in self.addrs.iter() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.connection {
            try!(os.write_enum(3, v as i32));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Message_Peer>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Message_Peer {
    fn new() -> Message_Peer {
        Message_Peer::new()
    }

    fn descriptor_static(_: ::std::option::Option<Message_Peer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "id",
                    Message_Peer::has_id,
                    Message_Peer::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "addrs",
                    Message_Peer::get_addrs,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "connection",
                    Message_Peer::has_connection,
                    Message_Peer::get_connection,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Message_Peer>(
                    "Message_Peer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Message_Peer {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_addrs();
        self.clear_connection();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Message_Peer {
    fn eq(&self, other: &Message_Peer) -> bool {
        self.id == other.id &&
        self.addrs == other.addrs &&
        self.connection == other.connection &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Message_Peer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Message_MessageType {
    PUT_VALUE = 0,
    GET_VALUE = 1,
    ADD_PROVIDER = 2,
    GET_PROVIDERS = 3,
    FIND_NODE = 4,
    PING = 5,
}

impl ::protobuf::ProtobufEnum for Message_MessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Message_MessageType> {
        match value {
            0 => ::std::option::Option::Some(Message_MessageType::PUT_VALUE),
            1 => ::std::option::Option::Some(Message_MessageType::GET_VALUE),
            2 => ::std::option::Option::Some(Message_MessageType::ADD_PROVIDER),
            3 => ::std::option::Option::Some(Message_MessageType::GET_PROVIDERS),
            4 => ::std::option::Option::Some(Message_MessageType::FIND_NODE),
            5 => ::std::option::Option::Some(Message_MessageType::PING),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<Message_MessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Message_MessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Message_MessageType {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Message_ConnectionType {
    NOT_CONNECTED = 0,
    CONNECTED = 1,
    CAN_CONNECT = 2,
    CANNOT_CONNECT = 3,
}

impl ::protobuf::ProtobufEnum for Message_ConnectionType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Message_ConnectionType> {
        match value {
            0 => ::std::option::Option::Some(Message_ConnectionType::NOT_CONNECTED),
            1 => ::std::option::Option::Some(Message_ConnectionType::CONNECTED),
            2 => ::std::option::Option::Some(Message_ConnectionType::CAN_CONNECT),
            3 => ::std::option::Option::Some(Message_ConnectionType::CANNOT_CONNECT),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<Message_ConnectionType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Message_ConnectionType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Message_ConnectionType {
}

#[derive(Clone,Default)]
pub struct Record {
    // message fields
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    author: ::protobuf::SingularField<::std::string::String>,
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Record {
    pub fn new() -> Record {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Record {
        static mut instance: ::protobuf::lazy::Lazy<Record> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Record,
        };
        unsafe {
            instance.get(|| {
                Record {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    author: ::protobuf::SingularField::none(),
                    signature: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        self.key.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a str {
        match self.key.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a [u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional string author = 3;

    pub fn clear_author(&mut self) {
        self.author.clear();
    }

    pub fn has_author(&self) -> bool {
        self.author.is_some()
    }

    // Param is passed by value, moved
    pub fn set_author(&mut self, v: ::std::string::String) {
        self.author = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_author<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.author.is_none() {
            self.author.set_default();
        };
        self.author.as_mut().unwrap()
    }

    // Take field
    pub fn take_author(&mut self) -> ::std::string::String {
        self.author.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_author<'a>(&'a self) -> &'a str {
        match self.author.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes signature = 4;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.signature.is_none() {
            self.signature.set_default();
        };
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_signature<'a>(&'a self) -> &'a [u8] {
        match self.signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for Record {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.key.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.author.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.signature.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.key.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.author.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.signature.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.value.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.author.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.signature.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Record>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Record {
    fn new() -> Record {
        Record::new()
    }

    fn descriptor_static(_: ::std::option::Option<Record>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "key",
                    Record::has_key,
                    Record::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    Record::has_value,
                    Record::get_value,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "author",
                    Record::has_author,
                    Record::get_author,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "signature",
                    Record::has_signature,
                    Record::get_signature,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Record>(
                    "Record",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Record {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.clear_author();
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Record {
    fn eq(&self, other: &Record) -> bool {
        self.key == other.key &&
        self.value == other.value &&
        self.author == other.author &&
        self.signature == other.signature &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Record {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x09, 0x64, 0x68, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x06, 0x64, 0x68, 0x74,
    0x2e, 0x70, 0x62, 0x22, 0xed, 0x03, 0x0a, 0x07, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12,
    0x29, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1b, 0x2e,
    0x64, 0x68, 0x74, 0x2e, 0x70, 0x62, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x17, 0x0a, 0x0f, 0x63, 0x6c,
    0x75, 0x73, 0x74, 0x65, 0x72, 0x4c, 0x65, 0x76, 0x65, 0x6c, 0x52, 0x61, 0x77, 0x18, 0x0a, 0x20,
    0x01, 0x28, 0x05, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
    0x12, 0x1e, 0x0a, 0x06, 0x72, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0e, 0x2e, 0x64, 0x68, 0x74, 0x2e, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x63, 0x6f, 0x72, 0x64,
    0x12, 0x29, 0x0a, 0x0b, 0x63, 0x6c, 0x6f, 0x73, 0x65, 0x72, 0x50, 0x65, 0x65, 0x72, 0x73, 0x18,
    0x08, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x64, 0x68, 0x74, 0x2e, 0x70, 0x62, 0x2e, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x12, 0x2b, 0x0a, 0x0d, 0x70,
    0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x72, 0x50, 0x65, 0x65, 0x72, 0x73, 0x18, 0x09, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x14, 0x2e, 0x64, 0x68, 0x74, 0x2e, 0x70, 0x62, 0x2e, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x1a, 0x55, 0x0a, 0x04, 0x50, 0x65, 0x65, 0x72,
    0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05,
    0x61, 0x64, 0x64, 0x72, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0c, 0x12, 0x32, 0x0a, 0x0a, 0x63,
    0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x1e, 0x2e, 0x64, 0x68, 0x74, 0x2e, 0x70, 0x62, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x2e, 0x43, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x22,
    0x69, 0x0a, 0x0b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0d,
    0x0a, 0x09, 0x50, 0x55, 0x54, 0x5f, 0x56, 0x41, 0x4c, 0x55, 0x45, 0x10, 0x00, 0x12, 0x0d, 0x0a,
    0x09, 0x47, 0x45, 0x54, 0x5f, 0x56, 0x41, 0x4c, 0x55, 0x45, 0x10, 0x01, 0x12, 0x10, 0x0a, 0x0c,
    0x41, 0x44, 0x44, 0x5f, 0x50, 0x52, 0x4f, 0x56, 0x49, 0x44, 0x45, 0x52, 0x10, 0x02, 0x12, 0x11,
    0x0a, 0x0d, 0x47, 0x45, 0x54, 0x5f, 0x50, 0x52, 0x4f, 0x56, 0x49, 0x44, 0x45, 0x52, 0x53, 0x10,
    0x03, 0x12, 0x0d, 0x0a, 0x09, 0x46, 0x49, 0x4e, 0x44, 0x5f, 0x4e, 0x4f, 0x44, 0x45, 0x10, 0x04,
    0x12, 0x08, 0x0a, 0x04, 0x50, 0x49, 0x4e, 0x47, 0x10, 0x05, 0x22, 0x57, 0x0a, 0x0e, 0x43, 0x6f,
    0x6e, 0x6e, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x11, 0x0a, 0x0d,
    0x4e, 0x4f, 0x54, 0x5f, 0x43, 0x4f, 0x4e, 0x4e, 0x45, 0x43, 0x54, 0x45, 0x44, 0x10, 0x00, 0x12,
    0x0d, 0x0a, 0x09, 0x43, 0x4f, 0x4e, 0x4e, 0x45, 0x43, 0x54, 0x45, 0x44, 0x10, 0x01, 0x12, 0x0f,
    0x0a, 0x0b, 0x43, 0x41, 0x4e, 0x5f, 0x43, 0x4f, 0x4e, 0x4e, 0x45, 0x43, 0x54, 0x10, 0x02, 0x12,
    0x12, 0x0a, 0x0e, 0x43, 0x41, 0x4e, 0x4e, 0x4f, 0x54, 0x5f, 0x43, 0x4f, 0x4e, 0x4e, 0x45, 0x43,
    0x54, 0x10, 0x03, 0x22, 0x47, 0x0a, 0x06, 0x52, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x12, 0x0b, 0x0a,
    0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0e, 0x0a, 0x06, 0x61, 0x75, 0x74,
    0x68, 0x6f, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x69, 0x67,
    0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
