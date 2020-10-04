use reqwest::Client;
use url::Url;
use std::fmt;

pub mod products;

// https://gibs.earthdata.nasa.gov/{service:wmts|wms|twms}/epsg{code:4326|3857|3413|3031}/{type:all|best|nrt|std}

/// WGS 84 / Geographic - EPSG:4326, WMTS version 1.0.0
pub static EPSG_4326: EPSG = EPSG(4326);
pub static GEOGRAPHIC: EPSG = EPSG_4326;
/// Web Mercator - EPSG:3857, WMTS version 1.0.0
pub static EPSG_3857: EPSG = EPSG(3857);
pub static WEB_MERCATOR: EPSG = EPSG_3857;
/// Arctic polar stereographic - EPSG:3413, WMTS version 1.0.0
pub static EPSG_3413: EPSG = EPSG(3413);
/// Antarctic polar stereographic - EPSG:3031, WMTS version 1.0.0
pub static EPSG_3031: EPSG = EPSG(3031);

pub trait Projection {}

pub trait Geographic: Projection {}
pub trait WebMercator: Projection {}

/// best - The "Best Available" imagery products.
/// std - Standard imagery products only.
/// nrt - Near Real-Time imagery products only.
/// all - All Best Available, Standard, and Near Real-Time imagery products.
pub enum Imagery {
    /// "all"
    All,
    /// best
    Best,
    /// nrt
    NearRealTime,
    /// std
    Standard,
}

impl fmt::Display for Imagery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self {
            Imagery::All => "all",
            Imagery::Best => "best",
            Imagery::NearRealTime => "nrt",
            Imagery::Standard => "std",
        };

        write!(f, "{}", type_str)
    }
}

pub enum Service {
    WMTS,
    WMS,
    TWMS,
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let service_str = match self {
            Service::WMTS => "wmts",
            Service::WMS => "wms",
            Service::TWMS => "twms",
        };

        write!(f, "{}", service_str)
    }
}

#[derive(Copy, Clone)]
pub struct EPSG(u16);

impl fmt::Display for EPSG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct GIBS {
    client: Client,
}

impl GIBS {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn get_url(&self, service: Service, epsg: EPSG, imagery_type: Imagery) -> Result<Url, url::ParseError> {
        // https://gibs.earthdata.nasa.gov/twms/epsg4326/best/twms.cgi?request=GetMap&layers=MODIS_Terra_CorrectedReflectance_TrueColor&srs=EPSG:4326&format=image/jpeg&styles=&time=2012-07-09&width=512&height=512&bbox=-18,27,-13.5,31.5
        let url_string = format!("https://gibs.earthdata.nasa.gov/{}/epsg{}/{}/{}.sgi", service, epsg, imagery_type, service);

        Url::parse(&url_string)
    }
}

pub mod image {
    pub struct Image {
        bytes: Vec<u8>,
        image_type: Type,
    }

    pub enum Type {
        JPEG,
        PNG,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_url_test() {
        let gibs = GIBS::new();

        let url_1 = gibs.get_url(Service::WMTS, GEOGRAPHIC, Imagery::Standard).expect("Should make");



        println!("{}", url_1);
    }
}
