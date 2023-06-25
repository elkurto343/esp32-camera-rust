use std::os::raw::c_int;

mod aithinker;
mod freenove;

use aithinker::AITHINKER_DVP_PINS;
use freenove::FREENOVE_DVP_PINS;

// Pin assignment for MIPI interface
#[derive(Debug)]
pub struct MipiPins {
    // Power down
    pub pwdn: c_int,
    // Reset
    pub rst: c_int,
    // Master clock
    pub xclk: c_int,
    // SDA two-wire line
    pub sda: c_int,
    // SCLK two-wire line
    pub scl: c_int,

    // Pixel data lines
    pub d9: c_int,
    pub d8: c_int,
    pub d7: c_int,
    pub d6: c_int,
    pub d5: c_int,
    pub d4: c_int,
}

// TODO: replace Clone + Copy trait and pass via reference
// Pin assignment for DVP interface
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DvpPins {
    // Power down
    pub pwdn: c_int,
    // Sensor reset
    pub rst: c_int,
    // Master clock
    pub xclk: c_int,
    // Pixel clock
    pub pclk: c_int,
    // Frame valid (active high: indicates active frame)
    pub vsync: c_int,
    // Pixels valid (active high: indicates active pixels)
    pub href: c_int,
    // SDA two-wire line
    pub sda: c_int,
    // SCLK two-wire line
    pub scl: c_int,

    // Pixel data lines
    pub d7: c_int,
    pub d6: c_int,
    pub d5: c_int,
    pub d4: c_int,
    pub d3: c_int,
    pub d2: c_int,
    pub d1: c_int,
    pub d0: c_int,
}

#[derive(Debug, PartialEq)]
pub enum Board {
    Freenove,
    AIThinker,
    Custom(DvpPins),
}

impl Board {
    pub fn dvp_pins(self) -> DvpPins {
        match self {
            Board::Freenove => FREENOVE_DVP_PINS,
            Board::AIThinker => AITHINKER_DVP_PINS,
            Board::Custom(dvp_pins) => dvp_pins,
        }
    }

    // pub fn camera_activity_led(&pins: gpio::Pins) -> Option<PinDriver> {
    //     match self {
    //         Board::Freenove => Some(PinDriver::output(&pins.gpio2).unwrap()),
    //         Board::AIThinker => None,
    //         // TODO:
    //         // Board::Custom(dvp_pins, cam_act_led) => cam_act_led,
    //         Board::Custom(_) => None,
    //     }
    // }

    pub fn from_env() -> Self {
        match env!("BOARD_MODEL") {
            "Freenove" => Board::Freenove,
            "AIThinker" => Board::AIThinker,
            // _ => Board::Custom(DvpPins {
            //     pwdn: env!("BOARD_CAM_PWDN").try_into().unwrap(),
            //     rst: env!("BOARD_CAM_RST").try_into().unwrap(),
            //     xclk: env!("BOARD_CAM_XCLK").try_into().unwrap(),
            //     pclk: env!("BOARD_CAM_PCLK").try_into().unwrap(),
            //     vsync: env!("BOARD_CAM_VSYNC").try_into().unwrap(),
            //     href: env!("BOARD_CAM_HREF").try_into().unwrap(),
            //     sda: env!("BOARD_CAM_SDA").try_into().unwrap(),
            //     scl: env!("BOARD_CAM_SCL").try_into().unwrap(),
            //     d7: env!("BOARD_CAM_D7").try_into().unwrap(),
            //     d6: env!("BOARD_CAM_D6").try_into().unwrap(),
            //     d5: env!("BOARD_CAM_D5").try_into().unwrap(),
            //     d4: env!("BOARD_CAM_D4").try_into().unwrap(),
            //     d3: env!("BOARD_CAM_D3").try_into().unwrap(),
            //     d2: env!("BOARD_CAM_D2").try_into().unwrap(),
            //     d1: env!("BOARD_CAM_D1").try_into().unwrap(),
            //     d0: env!("BOARD_CAM_D0").try_into().unwrap(),
            // }),
            _ => panic!("env var: invalid board specified"),
        }
    }
}
