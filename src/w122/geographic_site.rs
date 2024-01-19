//! Geographite Site Module

#[derive(Clone,Default,Debug)]
pub enum MEFSiteType {
    #[default]
    Public,
    Private,
}

#[derive(Clone,Default,Debug)]
pub struct GeographicSite {
    id : String,
    href: String,
    name: String,
    description: String,
    company_name: String,
    customer_name: String,
    site_type: MEFSiteType,
}