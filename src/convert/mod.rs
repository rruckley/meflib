//! Conversion Module
//! 


use tmflib::{tmf674::geographic_site_v4::GeographicSite as TMFSite, HasId};
// use tmflib::tmf629::customer::Customer;
use tmflib::tmf632::organization::Organization;
use tmflib::common::related_party::RelatedParty;
use crate::w122::geographic_site::GeographicSite as MEFSite;
use std::convert::From;


/// Whilst this function converts from a MEF payload into a TMF payload
/// it does not magically create records in any persistence layer. The 
/// resulting TMF payload can simply be used to create a new record that 
/// closely matches the input MEF payload. In the case of reference fields
/// such as related_party, an assumption is made that there is a matching
/// record already in existence at the end of that reference.
impl From<MEFSite> for TMFSite {
    fn from(value: MEFSite) -> Self {
        let mut site = TMFSite::new(value.name);
        site.description = Some(value.description.clone());
        // Copying the id across might violate uniqueness constraints
        site.id = Some(value.id.clone());
        site.generate_href();
        let org = Organization::from(value.customer_name);
        let party = RelatedParty::from(org);
        site.related_party = Some(vec![party]);
        site
    }
    
}
