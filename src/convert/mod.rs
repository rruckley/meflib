//! Conversion Module
//! 


use tmflib::{tmf674::geographic_site::GeographicSite as TMFSite, HasId};
use tmflib::tmf629::customer::Customer;
use tmflib::tmf632::organization::Organization;
use tmflib::common::related_party::RelatedParty;
use crate::w122::geographic_site::GeographicSite as MEFSite;
use std::convert::From;

impl From<MEFSite> for TMFSite {
    fn from(value: MEFSite) -> Self {
        let mut site = TMFSite::new(value.name);
        site.description = Some(value.description.clone());
        // Copying the id across might violate uniqueness constraints
        site.id = Some(value.id.clone());
        site.generate_href();
        let org = Organization::from(value.customer_name);
        let party = RelatedParty::from(org);
        // site.related_party.push(party)
        site
    }
    
}
