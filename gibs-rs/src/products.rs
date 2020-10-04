use crate::image;

/// https://wiki.earthdata.nasa.gov/display/GIBS/GIBS+Available+Imagery+Products#expand-CleanInfrared3Products
pub struct CleanInfrared {
    platform: Platform,
    instrument: String,
    image: image::Type,
    layer: String,
}

impl CleanInfrared {
    pub fn new(platform: Platform) -> Self {
        let layer = match platform {
            Platform::GoesEast => "GOES-East_ABI_Band13_Clean_Infrared",
            Platform::GoesWest => "GOES-West_ABI_Band13_Clean_Infrared",
            Platform::Himawari8 => "Himawari_AHI_Band3_Red_Visible_1km",
        };

        Self {
            platform,
            instrument: "ABI".into(),
            image: image::Type::PNG,
            layer: layer.into(),
        }
    }
}

pub enum Platform {
    GoesEast,
    GoesWest,
    Himawari8,
}
