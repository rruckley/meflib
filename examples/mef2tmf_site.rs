//! Convert from MEF to TMF Site object
use meflib::w122::geographic_site::GeographicSite as MEFSite;
use tmflib::tmf674::geographic_site_v4::GeographicSite as TMFSite;

fn main() {
    let mef = MEFSite {
        id : "mef123".to_string(),
        name: "MyMEFSite".into(),
        description : "This is a MEF site to be converted to TMF".into(),
        customer_name : "Orange Computers".into(),
        ..Default::default()
    };

    let tmf : TMFSite = TMFSite::from(mef);

    dbg!(tmf);
}