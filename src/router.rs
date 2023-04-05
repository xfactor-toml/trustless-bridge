use crate::{error::Error, host::ChainID, prelude::Vec};
use codec::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
pub struct Request {
    pub nonce: u64,
    pub source_chain: ChainID,
    pub dest_chain: ChainID,
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub timeout_timestamp: u64,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
pub struct Response {
    pub request: Request,
    pub response: Vec<u8>,
}

pub enum RequestResponse {
    Request(Request),
    Response(Response),
}

pub trait IISMPRouter {
    /// Dispatch a request from a module to the ISMP router.
    /// If request source chain is the host, it should be committed in state as a sha256 hash
    fn dispatch(&self, request: Request) -> Result<(), Error>;

    /// Provide a response to a previously received request.
    /// If response source chain is the host, it should be committed in state as a sha256 hash
    fn write_response(&self, response: Response) -> Result<(), Error>;
}
