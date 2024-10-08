//! Convert from MEF to TMF Site object
use meflib::w122::geographic_site::GeographicSite as MEFSite;

fn main() {
    let mef = MEFSite {
        id : "mef123".to_string(),
        name: "MyMEFSite".into(),
        description : "This is a MEF site to be converted to TMF".into(),
        customer_name : "Orange Computers".into(),
        company_name: "The Property Group Pty Ltd".into(),
        ..Default::default()
    };

    dbg!(mef);
}