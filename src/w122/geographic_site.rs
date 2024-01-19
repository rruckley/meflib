//! Geographite Site Module

pub enum MEFSiteType {
    Public,
    Private,
}

pub struct GeographicSite {
    id : String,
    href: String,
    name: String,
    description: String,
    company_name: String,
    customer_name: String,
    site_type: MEFSiteType,
}