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
    pub email_address: String,
    pub name : String,
    pub number : String,
    pub number_extension : String,
    pub organization : String,
    pub role: String,
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
    pub related_contact_information: Vec<RelatedContactInformation>,
    pub site_type: MEFSiteType,
    pub postal_address: Vec<FieldedAddress>,
}