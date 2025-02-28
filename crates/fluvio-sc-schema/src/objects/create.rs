#![allow(clippy::assign_op_pattern)]

use std::fmt::Debug;

use dataplane::core::{Encoder, Decoder};
use dataplane::api::Request;
use fluvio_controlplane_metadata::derivedstream::DerivedStreamSpec;
use fluvio_protocol::Version;
use crate::topic::TopicSpec;
use crate::customspu::CustomSpuSpec;
use crate::smartmodule::SmartModuleSpec;
use crate::tableformat::TableFormatSpec;
use crate::spg::SpuGroupSpec;
use crate::connector::ManagedConnectorSpec;

use crate::{AdminPublicApiKey, CreatableAdminSpec, Status};

#[derive(Encoder, Decoder, Default, Debug)]
pub struct CreateRequest<S: CreatableAdminSpec> {
    pub request: S,
}

/// Every create request must have this parameters
#[derive(Encoder, Decoder, Default, Debug)]
pub struct CommonCreateRequest {
    pub name: String,
    pub dry_run: bool,
}

impl Request for ObjectApiCreateRequest {
    const API_KEY: u16 = AdminPublicApiKey::Create as u16;
    const DEFAULT_API_VERSION: i16 = 3;
    type Response = Status;
}

#[derive(Debug, Default, Encoder, Decoder)]
pub struct ObjectApiCreateRequest {
    pub common: CommonCreateRequest,
    pub request: ObjectCreateRequest,
}

#[derive(Debug)]
pub enum ObjectCreateRequest {
    Topic(TopicSpec),
    CustomSpu(CustomSpuSpec),
    SmartModule(SmartModuleSpec),
    ManagedConnector(ManagedConnectorSpec),
    SpuGroup(SpuGroupSpec),
    TableFormat(TableFormatSpec),
    DerivedStream(DerivedStreamSpec),
}

impl Default for ObjectCreateRequest {
    fn default() -> Self {
        Self::Topic(TopicSpec::default())
    }
}

impl ObjectCreateRequest {
    fn type_value(&self) -> u8 {
        match self {
            Self::Topic(_) => TopicSpec::CREATE_TYPE,
            Self::CustomSpu(_) => CustomSpuSpec::CREATE_TYPE,
            Self::SmartModule(_) => SmartModuleSpec::CREATE_TYPE,
            Self::ManagedConnector(_) => ManagedConnectorSpec::CREATE_TYPE,
            Self::SpuGroup(_) => SpuGroupSpec::CREATE_TYPE,
            Self::TableFormat(_) => TableFormatSpec::CREATE_TYPE,
            Self::DerivedStream(_) => DerivedStreamSpec::CREATE_TYPE,
        }
    }
}

impl Encoder for ObjectCreateRequest {
    fn write_size(&self, version: dataplane::core::Version) -> usize {
        let type_size = (0u8).write_size(version);

        type_size
            + match self {
                Self::Topic(s) => s.write_size(version),
                Self::CustomSpu(s) => s.write_size(version),
                Self::SmartModule(s) => s.write_size(version),
                Self::ManagedConnector(s) => s.write_size(version),
                Self::SpuGroup(s) => s.write_size(version),
                Self::TableFormat(s) => s.write_size(version),
                Self::DerivedStream(s) => s.write_size(version),
            }
    }

    fn encode<T>(
        &self,
        dest: &mut T,
        version: dataplane::core::Version,
    ) -> Result<(), std::io::Error>
    where
        T: dataplane::bytes::BufMut,
    {
        self.type_value().encode(dest, version)?;
        match self {
            Self::Topic(s) => s.encode(dest, version)?,
            Self::CustomSpu(s) => s.encode(dest, version)?,
            Self::ManagedConnector(s) => s.encode(dest, version)?,
            Self::SmartModule(s) => s.encode(dest, version)?,
            Self::SpuGroup(s) => s.encode(dest, version)?,
            Self::TableFormat(s) => s.encode(dest, version)?,
            Self::DerivedStream(s) => s.encode(dest, version)?,
        }

        Ok(())
    }
}

// We implement decode signature even thought this will be never called.
// RequestMessage use decode_object.  But in order to provide backward compatibility, we pretend
// to provide decode implementation but shoudl be never called
impl dataplane::core::Decoder for ObjectCreateRequest {
    fn decode<T>(&mut self, src: &mut T, version: Version) -> Result<(), std::io::Error>
    where
        T: dataplane::bytes::Buf,
    {
        let mut typ: u8 = 0;
        typ.decode(src, version)?;
        tracing::trace!("decoded type: {}", typ);

        match typ {
            TopicSpec::CREATE_TYPE => {
                tracing::trace!("detected topic");
                let mut request = TopicSpec::default();
                request.decode(src, version)?;
                *self = Self::Topic(request);
                Ok(())
            }

            TableFormatSpec::CREATE_TYPE => {
                tracing::trace!("detected table");
                let mut request = TableFormatSpec::default();
                request.decode(src, version)?;
                *self = Self::TableFormat(request);
                Ok(())
            }

            CustomSpuSpec::CREATE_TYPE => {
                tracing::trace!("detected custom spu");
                let mut request = CustomSpuSpec::default();
                request.decode(src, version)?;
                *self = Self::CustomSpu(request);
                Ok(())
            }

            SpuGroupSpec::CREATE_TYPE => {
                tracing::trace!("detected custom spu");
                let mut request = SpuGroupSpec::default();
                request.decode(src, version)?;
                *self = Self::SpuGroup(request);
                Ok(())
            }

            SmartModuleSpec::CREATE_TYPE => {
                tracing::trace!("detected smartmodule");
                let mut request = SmartModuleSpec::default();
                request.decode(src, version)?;
                *self = Self::SmartModule(request);
                Ok(())
            }

            ManagedConnectorSpec::CREATE_TYPE => {
                tracing::trace!("detected connector");
                let mut request = ManagedConnectorSpec::default();
                request.decode(src, version)?;
                *self = Self::ManagedConnector(request);
                Ok(())
            }

            DerivedStreamSpec::CREATE_TYPE => {
                tracing::trace!("detected derivedstream");
                let mut request = DerivedStreamSpec::default();
                request.decode(src, version)?;
                *self = Self::DerivedStream(request);
                Ok(())
            }

            // Unexpected type
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid create type {:#?}", typ),
            )),
        }
    }
}

/// Macro to convert create request
/// impl From<(CommonCreateRequest TopicSpec)> for ObjectApiCreateRequest {
/// fn from(req: (CommonCreateRequest TopicSpec)) -> Self {
///       ObjectApiCreateRequest {
///           common: req.0,
///           request: req.1
///       }
/// }
/// ObjectFrom!(WatchRequest, Topic);

macro_rules! CreateFrom {
    ($create:ty,$specTy:ident) => {
        impl From<(crate::objects::CommonCreateRequest, $create)>
            for crate::objects::ObjectApiCreateRequest
        {
            fn from(fr: (crate::objects::CommonCreateRequest, $create)) -> Self {
                crate::objects::ObjectApiCreateRequest {
                    common: fr.0,
                    request: crate::objects::ObjectCreateRequest::$specTy(fr.1),
                }
            }
        }
    };
}

pub(crate) use CreateFrom;
