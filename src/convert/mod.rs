//! Conversion Module
//! 


use tmflib::{tmf674::geographic_site::GeographicSite as TMFSite, HasId};
use crate::w122::geographic_site::GeographicSite as MEFSite;
use std::convert::From;

impl From<MEFSite> for TMFSite {
    fn from(value: MEFSite) -> Self {
        let mut site = TMFSite::new(value.name);
        site.description = Some(value.description.clone());
        site.
        // Copying the id across might violate uniqueness constraints
        site.id = Some(value.id.clone());
        site.generate_href();
        site
    }
    
}
