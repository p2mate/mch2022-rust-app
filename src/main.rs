use core::fmt::Debug;
use embedded_graphics::prelude::*;
use embedded_hal::digital::v2::OutputPin;
use esp_idf_hal::gpio;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use esp_idf_sys as _;
use tinytga::DynamicTga; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

const ILI9341_POWERA: u8 = 0xCB; // Power control A register
const ILI9341_POWERB: u8 = 0xCF; // Power control B register
const ILI9341_DTCA: u8 = 0xE8; // Driver timing control A
const ILI9341_DTCB: u8 = 0xEA; // Driver timing control B
const ILI9341_POWER_SEQ: u8 = 0xED; // Power on sequence register
const ILI9341_3GAMMA_EN: u8 = 0xF2; // 3 Gamma enable register
const ILI9341_PRC: u8 = 0xF7; // Pump ratio control register
const ILI9341_LCMCTRL: u8 = 0xC0; // LCM Control
const ILI9341_POWER2: u8 = 0xC1; // Power Control 2 register
const ILI9341_VCOM1: u8 = 0xC5; // VCOM Control 1 register
const ILI9341_VCOM2: u8 = 0xC7; // VCOM Control 2 register
const ILI9341_MADCTL: u8 = 0x36; // Memory Data Access Control
const ILI9341_COLMOD: u8 = 0x3A; // Interface Pixel Format
const ILI9341_FRMCTR1: u8 = 0xB1; // Frame Rate Control (In Normal Mode)
const ILI9341_DFC: u8 = 0xB6; // Display Function Control register
const ILI9341_PVGAMCTRL: u8 = 0xE0; // Positive Voltage Gamma control
const ILI9341_NVGAMCTRL: u8 = 0xE1; // Negative Voltage Gamma control
const ILI9341_GAMSET: u8 = 0x26; // Display Invert On Gamma

struct Ili9341Command {
    cmd: u8,
    data: Vec<u8>,
}

fn hello_world_lcd(
    dc: gpio::Gpio33<gpio::Unknown>,
    rst: gpio::Gpio25<gpio::Unknown>,
    spi: spi::SPI3,
    sclk: gpio::Gpio18<gpio::Unknown>,
    sdo: gpio::Gpio23<gpio::Unknown>,
    cs: gpio::Gpio32<gpio::Unknown>,
) -> anyhow::Result<()> {
    use esp_idf_hal::delay;

    dbg!();
    let config = <spi::config::Config as Default>::default().baudrate(40.MHz().into());
    dbg!();

    let init_sequence = [
        Ili9341Command {
            cmd: ILI9341_POWERB,
            data: Vec::from([0x00, 0xC1, 0x30]),
        },
        Ili9341Command {
            cmd: ILI9341_POWER_SEQ,
            data: Vec::from([0x64, 0x03, 0x12, 0x81]),
        },
        Ili9341Command {
            cmd: ILI9341_DTCA,
            data: Vec::from([0x85, 0x00, 0x78]),
        },
        Ili9341Command {
            cmd: ILI9341_POWERA,
            data: Vec::from([0x39, 0x2C, 0x00, 0x34, 0x02]),
        },
        Ili9341Command {
            cmd: ILI9341_PRC,
            data: Vec::from([0x20]),
        },
        Ili9341Command {
            cmd: ILI9341_DTCB,
            data: Vec::from([0x00, 0x00]),
        },
        Ili9341Command {
            cmd: ILI9341_LCMCTRL,
            data: Vec::from([0x23]),
        },
        Ili9341Command {
            cmd: ILI9341_POWER2,
            data: Vec::from([0x10]),
        },
        Ili9341Command {
            cmd: ILI9341_VCOM1,
            data: Vec::from([0x3e, 0x28]),
        },
        Ili9341Command {
            cmd: ILI9341_VCOM2,
            data: Vec::from([0x86]),
        },
        Ili9341Command {
            cmd: ILI9341_MADCTL,
            data: Vec::from([0x48]),
        },
        Ili9341Command {
            cmd: ILI9341_COLMOD,
            data: Vec::from([0x55]),
        },
        Ili9341Command {
            cmd: ILI9341_FRMCTR1,
            data: Vec::from([0x00, 0x18]),
        },
        Ili9341Command {
            cmd: ILI9341_DFC,
            data: Vec::from([0x08, 0x82, 0x27]),
        },
        Ili9341Command {
            cmd: ILI9341_3GAMMA_EN,
            data: Vec::from([0x00]),
        },
        Ili9341Command {
            cmd: ILI9341_GAMSET,
            data: Vec::from([0x01]),
        },
        Ili9341Command {
            cmd: ILI9341_PVGAMCTRL,
            data: Vec::from([
                0x0F, 0x31, 0x2B, 0x0C, 0x0E, 0x08, 0x4E, 0xF1, 0x37, 0x07, 0x10, 0x03, 0x0E, 0x09,
                0x00,
            ]),
        },
        Ili9341Command {
            cmd: ILI9341_NVGAMCTRL,
            data: Vec::from([
                0x00, 0x0E, 0x14, 0x03, 0x11, 0x07, 0x31, 0xC1, 0x48, 0x08, 0x0F, 0x0C, 0x31, 0x36,
                0x0F,
            ]),
        },
    ];

    let spi_master = spi::Master::<spi::SPI3, _, _, _, _>::new(
        spi,
        spi::Pins {
            sclk,
            sdo,
            sdi: Option::<gpio::Gpio21<gpio::Unknown>>::None,
            cs: Option::<gpio::Gpio21<gpio::Unknown>>::None,
        },
        config,
    )?;
    let mut di =
        display_interface_spi::SPIInterface::new(spi_master, dc.into_output()?, cs.into_output()?);

    for cmd in init_sequence {
        use display_interface::DataFormat;
        use display_interface::WriteOnlyDataCommand;

        di.send_commands(DataFormat::U8(&[cmd.cmd])).unwrap();
        di.send_data(DataFormat::U8(&cmd.data)).unwrap();
    }

    dbg!();
    let reset = rst.into_output()?;
    dbg!();
    let mut display = ili9341::Ili9341::new(
        di,
        reset,
        &mut delay::Ets,
        MCH2022BadgeOrientation::Landscape,
        ili9341::DisplaySize240x320,
    )
    .map_err(|e| anyhow::anyhow!("Display error: {:?}", e))?;
    dbg!();
    led_draw(&mut display).map_err(|e| anyhow::anyhow!("Display error: {:?}", e))?;
    dbg!();
    Ok(())
}

fn led_draw<D>(display: &mut D) -> anyhow::Result<()>
where
    D: DrawTarget + Dimensions,
    D::Color: From<embedded_graphics::pixelcolor::Rgb565>,
    <D as embedded_graphics::draw_target::DrawTarget>::Color:
        From<embedded_graphics::pixelcolor::Gray8>,
    <D as embedded_graphics::draw_target::DrawTarget>::Color:
        From<embedded_graphics::pixelcolor::Rgb555>,
    <D as embedded_graphics::draw_target::DrawTarget>::Color:
        From<embedded_graphics::pixelcolor::Rgb888>,
    <D as embedded_graphics::draw_target::DrawTarget>::Error: Debug,
{
    use embedded_graphics::{
        image::Image,
        mono_font::{ascii::FONT_10X20, MonoTextStyle},
        pixelcolor::*,
        primitives::*,
        text::*,
    };

    display.clear(Rgb565::BLACK.into()).expect("Display error");
    let border = Rectangle::new(display.bounding_box().top_left, display.bounding_box().size)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK.into())
                .stroke_color(Rgb565::MAGENTA.into())
                .stroke_width(1)
                .build(),
        );
    border.draw(display).expect("Display error");
    dbg!(border.bounding_box());

    let rust_pride = include_bytes!("../rust-pride.tga");
    let tga = DynamicTga::from_slice(rust_pride).expect("Display error");
    let mut velocity = (1i32, 1i32);
    let mut image = Image::new(&tga, Point::new(2, 2));
    let mut image_erase = Rectangle::new(image.bounding_box().top_left, image.bounding_box().size)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::BLACK.into())
                .build(),
        );

    //let ohm_sign = include_bytes!("../mini_ohm.tga");
    //let ohm = DynamicTga::from_slice(ohm_sign).expect("Display error");
    //let ohm_image = Image::new(
    //    &ohm,
    //    Point::new(
    //        display.bounding_box().center().x - (ohm.size().width / 2) as i32,
    //        display.bounding_box().center().y - (ohm.size().height / 2) as i32,
    //    ),
    //);
    //ohm_image.draw(display).expect("Display error");
    //let text = Text::new(
    //    "p2mate",
    //    Point::new(display.bounding_box().center().x - 20, display.bounding_box().center().y - 5),
    //    MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE.into()),
    //);
    //    let text_bb = Rectangle::new(text.bounding_box().top_left, text.bounding_box().size)
    //        .into_styled(
    //            PrimitiveStyleBuilder::new()
    //                .fill_color(Rgb565::BLACK.into())
    //                .build(),
    //        );
    //text.draw(display).expect("Display error");

    //  let yoffset = display.bounding_box().center().y -8;
    //  let xoffset = display.bounding_box().center().x;
    //  let triangle = Triangle::new(
    //      Point::new(xoffset, 16 + yoffset),
    //      Point::new(xoffset + 16, 16 + yoffset),
    //      Point::new(xoffset, yoffset)
    //  ).into_styled(   PrimitiveStyleBuilder::new()
    //                  .fill_color(Rgb565::MAGENTA.into())
    //                  .build());
    //  triangle.draw(display).expect("Display error");
    loop {
        use embedded_hal::blocking::delay::DelayMs;
        use esp_idf_hal::delay;

        let mut delay = delay::Ets;

        image.translate_mut(Point::new(velocity.0, velocity.1));
        image_erase.translate_mut(Point::new(velocity.0, velocity.1));
        let image_bb = image.bounding_box();

        if image_bb.top_left.x < (border.bounding_box().top_left.x + 2)
            || image_bb.bottom_right().unwrap().x
                > (border.bounding_box().bottom_right().unwrap().x as i32 - 2)
        {
            velocity.0 = -velocity.0;
            image.translate_mut(Point::new(velocity.0, velocity.1));
            image_erase.translate_mut(Point::new(velocity.0, velocity.1));
        }

        if image_bb.top_left.y < (border.bounding_box().top_left.y as i32 + 2)
            || image_bb.bottom_right().unwrap().y
                > (border.bounding_box().bottom_right().unwrap().y as i32 - 2)
        {
            velocity.1 = -velocity.1;
            image.translate_mut(Point::new(velocity.0, velocity.1));
            image_erase.translate_mut(Point::new(velocity.0, velocity.1));
        }
        image.draw(display).expect("Display error");
        delay.delay_ms(0u32);
        image_erase.draw(display).expect("Display error");
        //     let overlap = image.bounding_box().intersection(&text.bounding_box());
        //  if !overlap.is_zero_sized() {
        //      let mut clipped_display = display.clipped(&overlap);
        //        text_bb.draw(&mut clipped_display).expect("Display error");
        //      text.draw(&mut clipped_display).expect("Display error");
        //  }
        //    let overlap = image.bounding_box().intersection(&triangle.bounding_box());
        //    if !overlap.is_zero_sized() {
        //        let mut clipped_display = display.clipped(&overlap);
        //        triangle.draw(&mut clipped_display).expect("Display error");
        //    }
     //   let overlap = image.bounding_box().intersection(&ohm_image.bounding_box());
     //   if !overlap.is_zero_sized() {
     //       let mut clipped_display = display.clipped(&overlap);
     //       ohm_image.draw(&mut clipped_display).expect("Display error");
     //   }
        

    }
}

pub enum MCH2022BadgeOrientation {
    Portrait,
    PortraitFlipped,
    Landscape,
    LandscapeFlipped,
}

impl ili9341::Mode for MCH2022BadgeOrientation {
    fn mode(&self) -> u8 {
        match self {
            Self::Portrait => 0 | 8,
            Self::Landscape => 0x20 | 8,
            Self::PortraitFlipped => 0x80 | 0x40 | 8,
            Self::LandscapeFlipped => 0x80 | 0x20 | 8,
        }
    }

    fn is_landscape(&self) -> bool {
        matches!(self, Self::Landscape | Self::LandscapeFlipped)
    }
}

fn main() -> anyhow::Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Greetings from Ferris to MCH!");
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;
    let mut mode = pins.gpio26.into_output()?;
    mode.set_low()?;

    hello_world_lcd(
        pins.gpio33,
        pins.gpio25,
        peripherals.spi3,
        pins.gpio18,
        pins.gpio23,
        pins.gpio32,
    )?;

    Ok(())
}
