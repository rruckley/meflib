//! Conversion Module
//! 


use tmflib::tmf674::geographic_site::GeographicSite as TMFSite;
use crate::w122::geographic_site::GeographicSite as MEFSite;
use std::convert::From;

impl From<MEFSite> for TMFSite {
    fn from(value: MEFSite) -> Self {
        let site = TMFSite::new(value.name);
        site
    }
    
}
