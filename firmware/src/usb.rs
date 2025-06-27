use core::sync::atomic::{AtomicBool, Ordering};

use defmt::*;
use embassy_usb::{
    class::hid::{self, HidReaderWriter, ReportId, RequestHandler},
    control::OutResponse,
    driver::Driver,
    Builder, Handler,
};
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

pub fn setup_usb_device<'d, D: Driver<'d>>(
    driver: D,
    data: &'d mut UsbDeviceData<'d>,
) -> UsbDevice<'d, D> {
    let mut usb_config = embassy_usb::Config::new(0xc0de, 0xcafe);
    usb_config.manufacturer = Some("DJ_Laser");
    usb_config.product = Some("N16 Remote");
    usb_config.serial_number = Some("DJ_Laser-16.01");

    let mut builder = Builder::new(
        driver,
        usb_config,
        &mut data.config_descriptor,
        &mut data.bos_descriptor,
        &mut data.msos_descriptor,
        &mut data.control_buf,
    );

    builder.handler(&mut data.device_handler);

    let config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: Some(&mut data.request_handler),
        poll_ms: 60,
        max_packet_size: 64,
    };

    let hid: HidReaderWriter<'_, D, 1, 8> =
        HidReaderWriter::<_, 1, 8>::new(&mut builder, &mut data.state, config);

    let usb_device = builder.build();

    UsbDevice {
        device: usb_device,
        hid,
    }
}

pub struct UsbDevice<'d, D: Driver<'d>> {
    device: embassy_usb::UsbDevice<'d, D>,
    hid: HidReaderWriter<'d, D, 1, 8>,
}

pub struct UsbDeviceData<'d> {
    config_descriptor: [u8; 256],
    bos_descriptor: [u8; 256],
    msos_descriptor: [u8; 256],
    control_buf: [u8; 64],

    state: hid::State<'d>,
    device_handler: UsbDeviceHandler,
    request_handler: UsbRequestHandler,
}

impl<'d> UsbDeviceData<'d> {
    pub fn new() -> Self {
        Self {
            config_descriptor: [0; 256],
            bos_descriptor: [0; 256],
            msos_descriptor: [0; 256],
            control_buf: [0; 64],

            state: hid::State::new(),
            device_handler: UsbDeviceHandler::new(),
            request_handler: UsbRequestHandler {},
        }
    }
}

struct UsbRequestHandler {}

impl RequestHandler for UsbRequestHandler {
    fn get_report(&mut self, id: ReportId, _buf: &mut [u8]) -> Option<usize> {
        info!("Get report for {:?}", id);
        None
    }

    fn set_report(&mut self, id: ReportId, data: &[u8]) -> OutResponse {
        info!("Set report for {:?}: {=[u8]}", id, data);
        OutResponse::Accepted
    }

    fn set_idle_ms(&mut self, id: Option<ReportId>, dur: u32) {
        info!("Set idle rate for {:?} to {:?}", id, dur);
    }

    fn get_idle_ms(&mut self, id: Option<ReportId>) -> Option<u32> {
        info!("Get idle rate for {:?}", id);
        None
    }
}

struct UsbDeviceHandler {
    configured: AtomicBool,
}

impl UsbDeviceHandler {
    fn new() -> Self {
        UsbDeviceHandler {
            configured: AtomicBool::new(false),
        }
    }
}

impl Handler for UsbDeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);
        if enabled {
            info!("Device enabled");
        } else {
            info!("Device disabled");
        }
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
        info!("Bus reset, the Vbus current limit is 100mA");
    }

    fn addressed(&mut self, addr: u8) {
        self.configured.store(false, Ordering::Relaxed);
        info!("USB address set to: {}", addr);
    }

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
        if configured {
            info!(
                "Device configured, it may now draw up to the configured current limit from Vbus."
            )
        } else {
            info!("Device is no longer configured, the Vbus current limit is 100mA.");
        }
    }
}
