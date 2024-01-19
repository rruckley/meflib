//! Geographite Site Module

#[derive(Clone,Default,Debug)]
pub enum MEFSiteType {
    #[default]
    Public,
    Private,
}

#[derive(Clone,Default,Debug)]
pub struct GeographicSite {
    pub id : String,
    pub href: String,
    pub name: String,
    pub description: String,
    pub company_name: String,
    pub customer_name: String,
    pub site_type: MEFSiteType,
}