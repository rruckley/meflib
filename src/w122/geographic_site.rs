//! Geographite Site Module


use serde::{Deserialize,Serialize};

#[derive(Clone,Default,Debug, Deserialize, Serialize)]
pub enum MEFSiteType {
    #[default]
    Public,
    Private,
}

#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct RelatedContactInformation {
    email_address: String,
    name : String,
    number : String,
    number_extension : String,
    organization : String,
    role: String,
}

#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct FieldedAddress {
    city : String,
    country : String,
    locality: String,
}

#[derive(Clone,Default,Debug,Deserialize,Serialize)]
pub struct GeographicSite {
    pub id : String,
    pub href: String,
    pub name: String,
    pub description: String,
    pub company_name: String,
    pub customer_name: String,
    pub site_type: MEFSiteType,
    pub postal_address: Vec<FieldedAddress>,
}