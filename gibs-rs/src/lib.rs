use log::warn;
use opencv::prelude::VideoWriterTrait;
use reqwest::Client;
use std::fmt;
use url::Url;

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
    image_width: i32,
    image_height: i32,
}

impl GIBS {
    pub fn new(image_height: i32, image_width: i32) -> Self {
        Self {
            client: Client::new(),
            image_height: image_height,
            image_width: image_width,
        }
    }

    pub fn get_url(
        &self,
        service: Service,
        epsg: EPSG,
        imagery_type: Imagery,
    ) -> Result<Url, url::ParseError> {
        // https://gibs.earthdata.nasa.gov/twms/epsg4326/best/twms.cgi?request=GetMap&layers=MODIS_Terra_CorrectedReflectance_TrueColor&srs=EPSG:4326&format=image/jpeg&styles=&time=2012-07-09&width=512&height=512&bbox=-18,27,-13.5,31.5
        let url_string = format!(
            "https://gibs.earthdata.nasa.gov/{}/epsg{}/{}/{}.sgi",
            service, epsg, imagery_type, service
        );

        Url::parse(&url_string)
    }

    /// Processes images to a single video file. Converted video file name is returned.
    // ToDo: Create proper error handling,
    // Use unique ID for generating converted video files,
    // Set custom video writer having filename, width and height of images.
    pub fn process_images(&self, images: Vec<image::Image>) -> Result<String, String> {
        let uuid = uuid::Uuid::new_v4();
        let file_name = format!("{}.mp4", uuid.to_string());

        let video_writer = match opencv::videoio::VideoWriter::new(
            file_name,
            opencv::videoio::VideoWriter::fourcc('M', 'J', 'P', 'G'),
            opencv::core::Size::new(self.image_width, self.image_height),
            true,
        ) {
            Ok(e) => e,

            Err(e) => {
                warn!("Error creating Opencv videowriter, error: {}", e);
                return "Error creating Opencv videowriter.";
            }
        };

        for image in images.iter() {
            match video_writer.write(image) {
                Ok(_) => continue,

                Err(e) => {
                    warn!("Error writing to video writer, error: {}", e);
                    return "Error writing to video writer.";
                }
            }
        }

        match video_writer.release() {
            Ok(_) => {}

            Err(e) => {
                warn!("Error closing video writer, error: {}", e);
                return "Error closing video writer.";
            }
        };

        Ok(file_name)
    }
}

pub mod image {
    pub struct Image {
        bytes: Vec<f64>,
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
        let gibs = GIBS::new(500, 500);

        let url_1 = gibs
            .get_url(Service::WMTS, GEOGRAPHIC, Imagery::Standard)
            .expect("Should make");

        println!("{}", url_1);
    }
}
