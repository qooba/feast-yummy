// This file is generated by rust-protobuf 3.1.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `feast/core/StreamFeatureView.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:feast.core.StreamFeatureView)
pub struct StreamFeatureView {
    // message fields
    ///  User-specified specifications of this feature view.
    // @@protoc_insertion_point(field:feast.core.StreamFeatureView.spec)
    pub spec: ::protobuf::MessageField<StreamFeatureViewSpec>,
    // @@protoc_insertion_point(field:feast.core.StreamFeatureView.meta)
    pub meta: ::protobuf::MessageField<super::FeatureView::FeatureViewMeta>,
    // special fields
    // @@protoc_insertion_point(special_field:feast.core.StreamFeatureView.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StreamFeatureView {
    fn default() -> &'a StreamFeatureView {
        <StreamFeatureView as ::protobuf::Message>::default_instance()
    }
}

impl StreamFeatureView {
    pub fn new() -> StreamFeatureView {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, StreamFeatureViewSpec>(
            "spec",
            |m: &StreamFeatureView| { &m.spec },
            |m: &mut StreamFeatureView| { &mut m.spec },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::FeatureView::FeatureViewMeta>(
            "meta",
            |m: &StreamFeatureView| { &m.meta },
            |m: &mut StreamFeatureView| { &mut m.meta },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StreamFeatureView>(
            "StreamFeatureView",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StreamFeatureView {
    const NAME: &'static str = "StreamFeatureView";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.spec)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.meta)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.spec.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.meta.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.spec.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.meta.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> StreamFeatureView {
        StreamFeatureView::new()
    }

    fn clear(&mut self) {
        self.spec.clear();
        self.meta.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StreamFeatureView {
        static instance: StreamFeatureView = StreamFeatureView {
            spec: ::protobuf::MessageField::none(),
            meta: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for StreamFeatureView {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StreamFeatureView").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StreamFeatureView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamFeatureView {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

///  Next available id: 17
#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:feast.core.StreamFeatureViewSpec)
pub struct StreamFeatureViewSpec {
    // message fields
    ///  Name of the feature view. Must be unique. Not updated.
    // @@protoc_insertion_point(field:feast.core.StreamFeatureViewSpec.name)
    pub name: ::std::string::String,
    ///  Name of Feast project that this feature view belongs to.
    // @@protoc_insertion_point(field:feast.core.StreamFeatureViewSpec.project)
    pub project: ::std::string::String,
    ///  List of names of entities associated with this feature view.
    // @@protoc_insertion_point(field:feast.core.StreamFeatureViewSpec.entities)
    pub entities: ::std::vec::Vec<::std::string::String>,
    ///  List of specifications for each feature defined as part of this feature view.
    // @@protoc_insertion_point(field:feast.core.StreamFeatureViewSpec.features)
    pub features: ::std::vec::Vec<super::Feature::FeatureSpecV2>,
    ///  List of specifications for each entity defined as part of this feature view.
    // @@protoc_insertion_point(field:feast.core.StreamFeatureViewSpec.entity_columns)
    pub entity_columns: ::std::vec::Vec<super::Feature::FeatureSpecV2>,
    ///  Description of the feature view.
    // @@protoc_insertion_point(field:feast.core.StreamFeatureViewSpec.description)
    pub description: ::std::string::String,
    ///  User defined metadata
    // @@protoc_insertion_point(field:feast.core.StreamFeatureViewSpec.tags)
    pub tags: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    ///  Owner of the feature view.
    // @@protoc_insertion_point(field:feast.core.StreamFeatureViewSpec.owner)
    pub owner: ::std::string::String,
    ///  Features in this feature view can only be retrieved from online serving
    ///  younger than ttl. Ttl is measured as the duration of time between
    ///  the feature's event timestamp and when the feature is retrieved
    ///  Feature values outside ttl will be returned as unset values and indicated to end user
    // @@protoc_insertion_point(field:feast.core.StreamFeatureViewSpec.ttl)
    pub ttl: ::protobuf::MessageField<::protobuf::well_known_types::duration::Duration>,
    ///  Batch/Offline DataSource where this view can retrieve offline feature data.
    // @@protoc_insertion_point(field:feast.core.StreamFeatureViewSpec.batch_source)
    pub batch_source: ::protobuf::MessageField<super::DataSource::DataSource>,
    ///  Streaming DataSource from where this view can consume "online" feature data.
    // @@protoc_insertion_point(field:feast.core.StreamFeatureViewSpec.stream_source)
    pub stream_source: ::protobuf::MessageField<super::DataSource::DataSource>,
    ///  Whether these features should be served online or not
    // @@protoc_insertion_point(field:feast.core.StreamFeatureViewSpec.online)
    pub online: bool,
    ///  Serialized function that is encoded in the streamfeatureview
    // @@protoc_insertion_point(field:feast.core.StreamFeatureViewSpec.user_defined_function)
    pub user_defined_function: ::protobuf::MessageField<super::OnDemandFeatureView::UserDefinedFunction>,
    ///  Mode of execution
    // @@protoc_insertion_point(field:feast.core.StreamFeatureViewSpec.mode)
    pub mode: ::std::string::String,
    ///  Aggregation definitions
    // @@protoc_insertion_point(field:feast.core.StreamFeatureViewSpec.aggregations)
    pub aggregations: ::std::vec::Vec<super::Aggregation::Aggregation>,
    ///  Timestamp field for aggregation
    // @@protoc_insertion_point(field:feast.core.StreamFeatureViewSpec.timestamp_field)
    pub timestamp_field: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:feast.core.StreamFeatureViewSpec.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a StreamFeatureViewSpec {
    fn default() -> &'a StreamFeatureViewSpec {
        <StreamFeatureViewSpec as ::protobuf::Message>::default_instance()
    }
}

impl StreamFeatureViewSpec {
    pub fn new() -> StreamFeatureViewSpec {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(16);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &StreamFeatureViewSpec| { &m.name },
            |m: &mut StreamFeatureViewSpec| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "project",
            |m: &StreamFeatureViewSpec| { &m.project },
            |m: &mut StreamFeatureViewSpec| { &mut m.project },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "entities",
            |m: &StreamFeatureViewSpec| { &m.entities },
            |m: &mut StreamFeatureViewSpec| { &mut m.entities },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "features",
            |m: &StreamFeatureViewSpec| { &m.features },
            |m: &mut StreamFeatureViewSpec| { &mut m.features },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "entity_columns",
            |m: &StreamFeatureViewSpec| { &m.entity_columns },
            |m: &mut StreamFeatureViewSpec| { &mut m.entity_columns },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "description",
            |m: &StreamFeatureViewSpec| { &m.description },
            |m: &mut StreamFeatureViewSpec| { &mut m.description },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "tags",
            |m: &StreamFeatureViewSpec| { &m.tags },
            |m: &mut StreamFeatureViewSpec| { &mut m.tags },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "owner",
            |m: &StreamFeatureViewSpec| { &m.owner },
            |m: &mut StreamFeatureViewSpec| { &mut m.owner },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, ::protobuf::well_known_types::duration::Duration>(
            "ttl",
            |m: &StreamFeatureViewSpec| { &m.ttl },
            |m: &mut StreamFeatureViewSpec| { &mut m.ttl },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DataSource::DataSource>(
            "batch_source",
            |m: &StreamFeatureViewSpec| { &m.batch_source },
            |m: &mut StreamFeatureViewSpec| { &mut m.batch_source },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DataSource::DataSource>(
            "stream_source",
            |m: &StreamFeatureViewSpec| { &m.stream_source },
            |m: &mut StreamFeatureViewSpec| { &mut m.stream_source },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "online",
            |m: &StreamFeatureViewSpec| { &m.online },
            |m: &mut StreamFeatureViewSpec| { &mut m.online },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::OnDemandFeatureView::UserDefinedFunction>(
            "user_defined_function",
            |m: &StreamFeatureViewSpec| { &m.user_defined_function },
            |m: &mut StreamFeatureViewSpec| { &mut m.user_defined_function },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "mode",
            |m: &StreamFeatureViewSpec| { &m.mode },
            |m: &mut StreamFeatureViewSpec| { &mut m.mode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "aggregations",
            |m: &StreamFeatureViewSpec| { &m.aggregations },
            |m: &mut StreamFeatureViewSpec| { &mut m.aggregations },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "timestamp_field",
            |m: &StreamFeatureViewSpec| { &m.timestamp_field },
            |m: &mut StreamFeatureViewSpec| { &mut m.timestamp_field },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<StreamFeatureViewSpec>(
            "StreamFeatureViewSpec",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for StreamFeatureViewSpec {
    const NAME: &'static str = "StreamFeatureViewSpec";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.name = is.read_string()?;
                },
                18 => {
                    self.project = is.read_string()?;
                },
                26 => {
                    self.entities.push(is.read_string()?);
                },
                34 => {
                    self.features.push(is.read_message()?);
                },
                42 => {
                    self.entity_columns.push(is.read_message()?);
                },
                50 => {
                    self.description = is.read_string()?;
                },
                58 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            10 => key = is.read_string()?,
                            18 => value = is.read_string()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.tags.insert(key, value);
                },
                66 => {
                    self.owner = is.read_string()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ttl)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.batch_source)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.stream_source)?;
                },
                96 => {
                    self.online = is.read_bool()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.user_defined_function)?;
                },
                114 => {
                    self.mode = is.read_string()?;
                },
                122 => {
                    self.aggregations.push(is.read_message()?);
                },
                130 => {
                    self.timestamp_field = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.project.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.project);
        }
        for value in &self.entities {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.features {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.entity_columns {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if !self.description.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.description);
        }
        for (k, v) in &self.tags {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if !self.owner.is_empty() {
            my_size += ::protobuf::rt::string_size(8, &self.owner);
        }
        if let Some(v) = self.ttl.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.batch_source.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.stream_source.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.online != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.user_defined_function.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.mode.is_empty() {
            my_size += ::protobuf::rt::string_size(14, &self.mode);
        }
        for value in &self.aggregations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if !self.timestamp_field.is_empty() {
            my_size += ::protobuf::rt::string_size(16, &self.timestamp_field);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.project.is_empty() {
            os.write_string(2, &self.project)?;
        }
        for v in &self.entities {
            os.write_string(3, &v)?;
        };
        for v in &self.features {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        for v in &self.entity_columns {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        if !self.description.is_empty() {
            os.write_string(6, &self.description)?;
        }
        for (k, v) in &self.tags {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::string_size(2, &v);
            os.write_raw_varint32(58)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_string(2, &v)?;
        };
        if !self.owner.is_empty() {
            os.write_string(8, &self.owner)?;
        }
        if let Some(v) = self.ttl.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.batch_source.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.stream_source.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if self.online != false {
            os.write_bool(12, self.online)?;
        }
        if let Some(v) = self.user_defined_function.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if !self.mode.is_empty() {
            os.write_string(14, &self.mode)?;
        }
        for v in &self.aggregations {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        if !self.timestamp_field.is_empty() {
            os.write_string(16, &self.timestamp_field)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> StreamFeatureViewSpec {
        StreamFeatureViewSpec::new()
    }

    fn clear(&mut self) {
        self.name.clear();
        self.project.clear();
        self.entities.clear();
        self.features.clear();
        self.entity_columns.clear();
        self.description.clear();
        self.tags.clear();
        self.owner.clear();
        self.ttl.clear();
        self.batch_source.clear();
        self.stream_source.clear();
        self.online = false;
        self.user_defined_function.clear();
        self.mode.clear();
        self.aggregations.clear();
        self.timestamp_field.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static StreamFeatureViewSpec {
        static instance: ::protobuf::rt::Lazy<StreamFeatureViewSpec> = ::protobuf::rt::Lazy::new();
        instance.get(StreamFeatureViewSpec::new)
    }
}

impl ::protobuf::MessageFull for StreamFeatureViewSpec {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("StreamFeatureViewSpec").unwrap()).clone()
    }
}

impl ::std::fmt::Display for StreamFeatureViewSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StreamFeatureViewSpec {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"feast/core/StreamFeatureView.proto\x12\nfeast.core\x1a\x1egoogle/pro\
    tobuf/duration.proto\x1a$feast/core/OnDemandFeatureView.proto\x1a\x1cfea\
    st/core/FeatureView.proto\x1a\x18feast/core/Feature.proto\x1a\x1bfeast/c\
    ore/DataSource.proto\x1a\x1cfeast/core/Aggregation.proto\"{\n\x11StreamF\
    eatureView\x125\n\x04spec\x18\x01\x20\x01(\x0b2!.feast.core.StreamFeatur\
    eViewSpecR\x04spec\x12/\n\x04meta\x18\x02\x20\x01(\x0b2\x1b.feast.core.F\
    eatureViewMetaR\x04meta\"\x98\x06\n\x15StreamFeatureViewSpec\x12\x12\n\
    \x04name\x18\x01\x20\x01(\tR\x04name\x12\x18\n\x07project\x18\x02\x20\
    \x01(\tR\x07project\x12\x1a\n\x08entities\x18\x03\x20\x03(\tR\x08entitie\
    s\x125\n\x08features\x18\x04\x20\x03(\x0b2\x19.feast.core.FeatureSpecV2R\
    \x08features\x12@\n\x0eentity_columns\x18\x05\x20\x03(\x0b2\x19.feast.co\
    re.FeatureSpecV2R\rentityColumns\x12\x20\n\x0bdescription\x18\x06\x20\
    \x01(\tR\x0bdescription\x12?\n\x04tags\x18\x07\x20\x03(\x0b2+.feast.core\
    .StreamFeatureViewSpec.TagsEntryR\x04tags\x12\x14\n\x05owner\x18\x08\x20\
    \x01(\tR\x05owner\x12+\n\x03ttl\x18\t\x20\x01(\x0b2\x19.google.protobuf.\
    DurationR\x03ttl\x129\n\x0cbatch_source\x18\n\x20\x01(\x0b2\x16.feast.co\
    re.DataSourceR\x0bbatchSource\x12;\n\rstream_source\x18\x0b\x20\x01(\x0b\
    2\x16.feast.core.DataSourceR\x0cstreamSource\x12\x16\n\x06online\x18\x0c\
    \x20\x01(\x08R\x06online\x12S\n\x15user_defined_function\x18\r\x20\x01(\
    \x0b2\x1f.feast.core.UserDefinedFunctionR\x13userDefinedFunction\x12\x12\
    \n\x04mode\x18\x0e\x20\x01(\tR\x04mode\x12;\n\x0caggregations\x18\x0f\
    \x20\x03(\x0b2\x17.feast.core.AggregationR\x0caggregations\x12'\n\x0ftim\
    estamp_field\x18\x10\x20\x01(\tR\x0etimestampField\x1a7\n\tTagsEntry\x12\
    \x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\
    \x01(\tR\x05value:\x028\x01B[\n\x10feast.proto.coreB\x16StreamFeatureVie\
    wProtoZ/github.com/feast-dev/feast/go/protos/feast/coreJ\xb7\x17\n\x06\
    \x12\x04\x11\0Y\x01\n\xc6\x04\n\x01\x0c\x12\x03\x11\0\x122\xbb\x04\n\x20\
    Copyright\x202020\x20The\x20Feast\x20Authors\n\n\x20Licensed\x20under\
    \x20the\x20Apache\x20License,\x20Version\x202.0\x20(the\x20\"License\");\
    \n\x20you\x20may\x20not\x20use\x20this\x20file\x20except\x20in\x20compli\
    ance\x20with\x20the\x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\
    \x20of\x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20https://www.apache\
    .org/licenses/LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\
    \x20law\x20or\x20agreed\x20to\x20in\x20writing,\x20software\n\x20distrib\
    uted\x20under\x20the\x20License\x20is\x20distributed\x20on\x20an\x20\"AS\
    \x20IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\
    \x20ANY\x20KIND,\x20either\x20express\x20or\x20implied.\n\x20See\x20the\
    \x20License\x20for\x20the\x20specific\x20language\x20governing\x20permis\
    sions\x20and\n\x20limitations\x20under\x20the\x20License.\n\n\n\x08\n\
    \x01\x02\x12\x03\x12\0\x13\n\x08\n\x01\x08\x12\x03\x14\0F\n\t\n\x02\x08\
    \x0b\x12\x03\x14\0F\n\x08\n\x01\x08\x12\x03\x15\07\n\t\n\x02\x08\x08\x12\
    \x03\x15\07\n\x08\n\x01\x08\x12\x03\x16\0)\n\t\n\x02\x08\x01\x12\x03\x16\
    \0)\n\t\n\x02\x03\0\x12\x03\x19\0(\n\t\n\x02\x03\x01\x12\x03\x1a\0.\n\t\
    \n\x02\x03\x02\x12\x03\x1b\0&\n\t\n\x02\x03\x03\x12\x03\x1c\0\"\n\t\n\
    \x02\x03\x04\x12\x03\x1d\0%\n\t\n\x02\x03\x05\x12\x03\x1e\0&\n\n\n\x02\
    \x04\0\x12\x04\x20\0$\x01\n\n\n\x03\x04\0\x01\x12\x03\x20\x08\x19\nB\n\
    \x04\x04\0\x02\0\x12\x03\"\x04#\x1a5\x20User-specified\x20specifications\
    \x20of\x20this\x20feature\x20view.\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x03\
    \"\x04\x19\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\"\x1a\x1e\n\x0c\n\x05\x04\
    \0\x02\0\x03\x12\x03\"!\"\n\x0b\n\x04\x04\0\x02\x01\x12\x03#\x04\x1d\n\
    \x0c\n\x05\x04\0\x02\x01\x06\x12\x03#\x04\x13\n\x0c\n\x05\x04\0\x02\x01\
    \x01\x12\x03#\x14\x18\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03#\x1b\x1c\n#\
    \n\x02\x04\x01\x12\x04'\0Y\x01\x1a\x17\x20Next\x20available\x20id:\x2017\
    \n\n\n\n\x03\x04\x01\x01\x12\x03'\x08\x1d\nE\n\x04\x04\x01\x02\0\x12\x03\
    )\x04\x14\x1a8\x20Name\x20of\x20the\x20feature\x20view.\x20Must\x20be\
    \x20unique.\x20Not\x20updated.\n\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03)\
    \x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03)\x0b\x0f\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03)\x12\x13\nG\n\x04\x04\x01\x02\x01\x12\x03,\x04\
    \x17\x1a:\x20Name\x20of\x20Feast\x20project\x20that\x20this\x20feature\
    \x20view\x20belongs\x20to.\n\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03,\
    \x04\n\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03,\x0b\x12\n\x0c\n\x05\x04\
    \x01\x02\x01\x03\x12\x03,\x15\x16\nK\n\x04\x04\x01\x02\x02\x12\x03/\x04!\
    \x1a>\x20List\x20of\x20names\x20of\x20entities\x20associated\x20with\x20\
    this\x20feature\x20view.\n\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03/\x04\
    \x0c\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03/\r\x13\n\x0c\n\x05\x04\x01\
    \x02\x02\x01\x12\x03/\x14\x1c\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03/\
    \x1f\x20\n\\\n\x04\x04\x01\x02\x03\x12\x032\x04(\x1aO\x20List\x20of\x20s\
    pecifications\x20for\x20each\x20feature\x20defined\x20as\x20part\x20of\
    \x20this\x20feature\x20view.\n\n\x0c\n\x05\x04\x01\x02\x03\x04\x12\x032\
    \x04\x0c\n\x0c\n\x05\x04\x01\x02\x03\x06\x12\x032\r\x1a\n\x0c\n\x05\x04\
    \x01\x02\x03\x01\x12\x032\x1b#\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x032&\
    '\n[\n\x04\x04\x01\x02\x04\x12\x035\x04.\x1aN\x20List\x20of\x20specifica\
    tions\x20for\x20each\x20entity\x20defined\x20as\x20part\x20of\x20this\
    \x20feature\x20view.\n\n\x0c\n\x05\x04\x01\x02\x04\x04\x12\x035\x04\x0c\
    \n\x0c\n\x05\x04\x01\x02\x04\x06\x12\x035\r\x1a\n\x0c\n\x05\x04\x01\x02\
    \x04\x01\x12\x035\x1b)\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x035,-\n/\n\
    \x04\x04\x01\x02\x05\x12\x038\x04\x1b\x1a\"\x20Description\x20of\x20the\
    \x20feature\x20view.\n\n\x0c\n\x05\x04\x01\x02\x05\x05\x12\x038\x04\n\n\
    \x0c\n\x05\x04\x01\x02\x05\x01\x12\x038\x0b\x16\n\x0c\n\x05\x04\x01\x02\
    \x05\x03\x12\x038\x19\x1a\n$\n\x04\x04\x01\x02\x06\x12\x03;\x04\x20\x1a\
    \x17\x20User\x20defined\x20metadata\n\n\x0c\n\x05\x04\x01\x02\x06\x06\
    \x12\x03;\x04\x16\n\x0c\n\x05\x04\x01\x02\x06\x01\x12\x03;\x17\x1b\n\x0c\
    \n\x05\x04\x01\x02\x06\x03\x12\x03;\x1e\x1f\n)\n\x04\x04\x01\x02\x07\x12\
    \x03>\x04\x15\x1a\x1c\x20Owner\x20of\x20the\x20feature\x20view.\n\n\x0c\
    \n\x05\x04\x01\x02\x07\x05\x12\x03>\x04\n\n\x0c\n\x05\x04\x01\x02\x07\
    \x01\x12\x03>\x0b\x10\n\x0c\n\x05\x04\x01\x02\x07\x03\x12\x03>\x13\x14\n\
    \xb2\x02\n\x04\x04\x01\x02\x08\x12\x03D\x04%\x1a\xa4\x02\x20Features\x20\
    in\x20this\x20feature\x20view\x20can\x20only\x20be\x20retrieved\x20from\
    \x20online\x20serving\n\x20younger\x20than\x20ttl.\x20Ttl\x20is\x20measu\
    red\x20as\x20the\x20duration\x20of\x20time\x20between\n\x20the\x20featur\
    e's\x20event\x20timestamp\x20and\x20when\x20the\x20feature\x20is\x20retr\
    ieved\n\x20Feature\x20values\x20outside\x20ttl\x20will\x20be\x20returned\
    \x20as\x20unset\x20values\x20and\x20indicated\x20to\x20end\x20user\n\n\
    \x0c\n\x05\x04\x01\x02\x08\x06\x12\x03D\x04\x1c\n\x0c\n\x05\x04\x01\x02\
    \x08\x01\x12\x03D\x1d\x20\n\x0c\n\x05\x04\x01\x02\x08\x03\x12\x03D#$\nZ\
    \n\x04\x04\x01\x02\t\x12\x03G\x04!\x1aM\x20Batch/Offline\x20DataSource\
    \x20where\x20this\x20view\x20can\x20retrieve\x20offline\x20feature\x20da\
    ta.\n\n\x0c\n\x05\x04\x01\x02\t\x06\x12\x03G\x04\x0e\n\x0c\n\x05\x04\x01\
    \x02\t\x01\x12\x03G\x0f\x1b\n\x0c\n\x05\x04\x01\x02\t\x03\x12\x03G\x1e\
    \x20\n[\n\x04\x04\x01\x02\n\x12\x03I\x04\"\x1aN\x20Streaming\x20DataSour\
    ce\x20from\x20where\x20this\x20view\x20can\x20consume\x20\"online\"\x20f\
    eature\x20data.\n\n\x0c\n\x05\x04\x01\x02\n\x06\x12\x03I\x04\x0e\n\x0c\n\
    \x05\x04\x01\x02\n\x01\x12\x03I\x0f\x1c\n\x0c\n\x05\x04\x01\x02\n\x03\
    \x12\x03I\x1f!\nD\n\x04\x04\x01\x02\x0b\x12\x03L\x04\x15\x1a7\x20Whether\
    \x20these\x20features\x20should\x20be\x20served\x20online\x20or\x20not\n\
    \n\x0c\n\x05\x04\x01\x02\x0b\x05\x12\x03L\x04\x08\n\x0c\n\x05\x04\x01\
    \x02\x0b\x01\x12\x03L\t\x0f\n\x0c\n\x05\x04\x01\x02\x0b\x03\x12\x03L\x12\
    \x14\nK\n\x04\x04\x01\x02\x0c\x12\x03O\x043\x1a>\x20Serialized\x20functi\
    on\x20that\x20is\x20encoded\x20in\x20the\x20streamfeatureview\n\n\x0c\n\
    \x05\x04\x01\x02\x0c\x06\x12\x03O\x04\x17\n\x0c\n\x05\x04\x01\x02\x0c\
    \x01\x12\x03O\x18-\n\x0c\n\x05\x04\x01\x02\x0c\x03\x12\x03O02\n\x20\n\
    \x04\x04\x01\x02\r\x12\x03R\x04\x15\x1a\x13\x20Mode\x20of\x20execution\n\
    \n\x0c\n\x05\x04\x01\x02\r\x05\x12\x03R\x04\n\n\x0c\n\x05\x04\x01\x02\r\
    \x01\x12\x03R\x0b\x0f\n\x0c\n\x05\x04\x01\x02\r\x03\x12\x03R\x12\x14\n&\
    \n\x04\x04\x01\x02\x0e\x12\x03U\x04+\x1a\x19\x20Aggregation\x20definitio\
    ns\n\n\x0c\n\x05\x04\x01\x02\x0e\x04\x12\x03U\x04\x0c\n\x0c\n\x05\x04\
    \x01\x02\x0e\x06\x12\x03U\r\x18\n\x0c\n\x05\x04\x01\x02\x0e\x01\x12\x03U\
    \x19%\n\x0c\n\x05\x04\x01\x02\x0e\x03\x12\x03U(*\n.\n\x04\x04\x01\x02\
    \x0f\x12\x03X\x04\x20\x1a!\x20Timestamp\x20field\x20for\x20aggregation\n\
    \n\x0c\n\x05\x04\x01\x02\x0f\x05\x12\x03X\x04\n\n\x0c\n\x05\x04\x01\x02\
    \x0f\x01\x12\x03X\x0b\x1a\n\x0c\n\x05\x04\x01\x02\x0f\x03\x12\x03X\x1d\
    \x1fb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(6);
            deps.push(::protobuf::well_known_types::duration::file_descriptor().clone());
            deps.push(super::OnDemandFeatureView::file_descriptor().clone());
            deps.push(super::FeatureView::file_descriptor().clone());
            deps.push(super::Feature::file_descriptor().clone());
            deps.push(super::DataSource::file_descriptor().clone());
            deps.push(super::Aggregation::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(StreamFeatureView::generated_message_descriptor_data());
            messages.push(StreamFeatureViewSpec::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
