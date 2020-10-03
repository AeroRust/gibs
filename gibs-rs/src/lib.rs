use reqwest::Client;


// https://gibs.earthdata.nasa.gov/{service:wmts|wms|twms}/epsg{code:4326|3857|3413|3031}/{type:all|best|nrt|std}

/// WGS 84 / Geographic - EPSG:4326, WMTS version 1.0.0
pub static EPSG_4326: EPSG = EPSG(4326);
/// Web Mercator - EPSG:3857, WMTS version 1.0.0
pub static EPSG_3857: EPSG = EPSG(3857);
/// Arctic polar stereographic - EPSG:3413, WMTS version 1.0.0
pub static EPSG_3413: EPSG = EPSG(3413);
/// Antarctic polar stereographic - EPSG:3031, WMTS version 1.0.0
pub static EPSG_3031: EPSG = EPSG(3031);
/// best - The "Best Available" imagery products.
/// std - Standard imagery products only.
/// nrt - Near Real-Time imagery products only.
/// all - All Best Available, Standard, and Near Real-Time imagery products.
pub enum Type {
    /// "all"
    All,
    /// best
    Best,
    /// nrt
    NearRealTime,
    /// std
    Standard
}

pub enum Service {
    WMTS,
    WMS,
    TWMS
}

pub struct EPSG(u16);

pub struct GIBS {
    client: Client,
}

impl GIBS {
    fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    fn get() -> () {
        ()
    }
}

pub struct Image {
    bytes: Vec<u8>
}
