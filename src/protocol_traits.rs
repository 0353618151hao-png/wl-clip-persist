use std::ffi::CString;
use std::os::fd::OwnedFd;

use wayrs_client::core::ObjectId;
use wayrs_client::object::Proxy;
use wayrs_client::protocol::WlSeat;
use wayrs_client::{Connection, EventCtx};
use wayrs_protocols::ext_data_control_v1::*;
use wayrs_protocols::wlr_data_control_unstable_v1::*;

use crate::logger::log_seat_target;

/// Trait that references the types that are used by a clipboard protocol.
pub(crate) trait DataControlV1: 'static {
    type DataControlOffer: DataControlOfferV1<Event: TryIntoGenericEvent<DataControlOfferEvent> + std::fmt::Debug>
        + std::fmt::Debug
        + 'static;
    type DataControlSource: DataControlSourceV1<Event: TryIntoGenericEvent<DataControlSourceEvent> + std::fmt::Debug>
        + std::fmt::Debug
        + 'static;
    type DataControlDevice: DataControlDeviceV1<
            DataControlSource = Self::DataControlSource,
            Event: TryIntoGenericEvent<DataControlDeviceEvent<Self::DataControlOffer>> + std::fmt::Debug,
        > + std::fmt::Debug
        + 'static;
    type DataControlManager: DataControlManagerV1<
            DataControlSource = Self::DataControlSource,
            DataControlDevice = Self::DataControlDevice,
            Event: std::fmt::Debug,
        > + std::fmt::Debug
        + 'static;
}

/// Types for the ext-data-control-v1 clipboard protocol.
pub(crate) struct ExtDataControlV1;

impl DataControlV1 for ExtDataControlV1 {
    type DataControlOffer = ExtDataControlOfferV1;
    type DataControlSource = ExtDataControlSourceV1;
    type DataControlDevice = ExtDataControlDeviceV1;
    type DataControlManager = ExtDataControlManagerV1;
}

/// Types for the wlr-data-control-unstable-v1 clipboard protocol.
pub(crate) struct ZwlrDataControlV1;

impl DataControlV1 for ZwlrDataControlV1 {
    type DataControlOffer = ZwlrDataControlOfferV1;
    type DataControlSource = ZwlrDataControlSourceV1;
    type DataControlDevice = ZwlrDataControlDeviceV1;
    type DataControlManager = ZwlrDataControlManagerV1;
}

/// Interface for interacting with a data control offer.
pub(crate) trait DataControlOfferV1: Clone + Copy + Send + Sync + Proxy {
    #[expect(dead_code)]
    const TYPE_NAME: &'static str;

    fn receive<D>(self, conn: &mut Connection<D>, mime_type: CString, fd: OwnedFd);

    fn destroy<D>(self, conn: &mut Connection<D>);
}

impl DataControlOfferV1 for ExtDataControlOfferV1 {
    const TYPE_NAME: &'static str = "ext_data_control_offer_v1";

    #[inline(always)]
    fn receive<D>(self, conn: &mut Connection<D>, mime_type: CString, fd: OwnedFd) {
        ExtDataControlOfferV1::receive(self, conn, mime_type, fd)
    }

    #[inline(always)]
    fn destroy<D>(self, conn: &mut Connection<D>) {
        ExtDataControlOfferV1::destroy(self, conn)
    }
}

impl DataControlOfferV1 for ZwlrDataControlOfferV1 {
    const TYPE_NAME: &'static str = "zwlr_data_control_offer_v1";

    #[inline(always)]
    fn receive<D>(self, conn: &mut Connection<D>, mime_type: CString, fd: OwnedFd) {
        ZwlrDataControlOfferV1::receive(self, conn, mime_type, fd)
    }

    #[inline(always)]
    fn destroy<D>(self, conn: &mut Connection<D>) {
        ZwlrDataControlOfferV1::destroy(self, conn)
    }
}

/// Interface for interacting with a data control source.
pub(crate) trait DataControlSourceV1: Clone + Copy + Send + Sync + Proxy {
    #[expect(dead_code)]
    const TYPE_NAME: &'static str;

    fn offer<D>(self, conn: &mut Connection<D>, mime_type: CString);

    fn destroy<D>(self, conn: &mut Connection<D>);
}

impl DataControlSourceV1 for ExtDataControlSourceV1 {
    const TYPE_NAME: &'static str = "ext_data_control_source_v1";

    #[inline(always)]
    fn offer<D>(self, conn: &mut Connection<D>, mime_type: CString) {
        ExtDataControlSourceV1::offer(self, conn, mime_type)
    }

    #[inline(always)]
    fn destroy<D>(self, conn: &mut Connection<D>) {
        ExtDataControlSourceV1::destroy(self, conn)
    }
}

impl DataControlSourceV1 for ZwlrDataControlSourceV1 {
    const TYPE_NAME: &'static str = "zwlr_data_control_source_v1";

    #[inline(always)]
    fn offer<D>(self, conn: &mut Connection<D>, mime_type: CString) {
        ZwlrDataControlSourceV1::offer(self, conn, mime_type)
    }

    #[inline(always)]
    fn destroy<D>(self, conn: &mut Connection<D>) {
        ZwlrDataControlSourceV1::destroy(self, conn)
    }
}

/// Interface for interacting with a data control device.
pub(crate) trait DataControlDeviceV1: Clone + Copy + Send + Sync + Proxy {
    const TYPE_NAME: &'static str;

    type DataControlSource: DataControlSourceV1;

    fn set_selection<D>(self, conn: &mut Connection<D>, source: Option<Self::DataControlSource>);

    fn set_primary_selection<D>(self, conn: &mut Connection<D>, source: Option<Self::DataControlSource>);

    fn destroy<D>(self, conn: &mut Connection<D>);
}

impl DataControlDeviceV1 for ExtDataControlDeviceV1 {
    const TYPE_NAME: &'static str = "ext_data_control_device_v1";

    type DataControlSource = ExtDataControlSourceV1;

    #[inline(always)]
    fn set_selection<D>(self, conn: &mut Connection<D>, source: Option<ExtDataControlSourceV1>) {
        ExtDataControlDeviceV1::set_selection(self, conn, source)
    }

    #[inline(always)]
    fn set_primary_selection<D>(self, conn: &mut Connection<D>, source: Option<ExtDataControlSourceV1>) {
        ExtDataControlDeviceV1::set_primary_selection(self, conn, source)
    }

    #[inline(always)]
    fn destroy<D>(self, conn: &mut Connection<D>) {
        ExtDataControlDeviceV1::destroy(self, conn)
    }
}

impl DataControlDeviceV1 for ZwlrDataControlDeviceV1 {
    const TYPE_NAME: &'static str = "zwlr_data_control_device_v1";

    type DataControlSource = ZwlrDataControlSourceV1;

    #[inline(always)]
    fn set_selection<D>(self, conn: &mut Connection<D>, source: Option<ZwlrDataControlSourceV1>) {
        ZwlrDataControlDeviceV1::set_selection(self, conn, source)
    }

    #[inline(always)]
    fn set_primary_selection<D>(self, conn: &mut Connection<D>, source: Option<ZwlrDataControlSourceV1>) {
        ZwlrDataControlDeviceV1::set_primary_selection(self, conn, source)
    }

    #[inline(always)]
    fn destroy<D>(self, conn: &mut Connection<D>) {
        ZwlrDataControlDeviceV1::destroy(self, conn)
    }
}

/// Interface for interacting with a data control manager.
pub(crate) trait DataControlManagerV1: Clone + Copy + Send + Sync + Proxy {
    #[expect(dead_code)]
    const TYPE_NAME: &'static str;

    type DataControlSource: DataControlSourceV1;
    type DataControlDevice: DataControlDeviceV1<DataControlSource = Self::DataControlSource>;

    fn create_data_source<D>(self, conn: &mut Connection<D>) -> Self::DataControlSource;

    #[expect(dead_code)]
    fn create_data_source_with_cb<D>(
        self,
        conn: &mut Connection<D>,
        cb: impl FnMut(EventCtx<'_, D, Self::DataControlSource>) + Send + 'static,
    ) -> Self::DataControlSource;

    #[expect(dead_code)]
    fn get_data_device<D>(self, conn: &mut Connection<D>, seat: WlSeat) -> Self::DataControlDevice;

    fn get_data_device_with_cb<D>(
        self,
        conn: &mut Connection<D>,
        seat: WlSeat,
        cb: impl FnMut(EventCtx<'_, D, Self::DataControlDevice>) + Send + 'static,
    ) -> Self::DataControlDevice;

    #[expect(dead_code)]
    fn destroy<D>(self, conn: &mut Connection<D>);
}

impl DataControlManagerV1 for ExtDataControlManagerV1 {
    const TYPE_NAME: &'static str = "ext_data_control_manager_v1";

    type DataControlSource = ExtDataControlSourceV1;
    type DataControlDevice = ExtDataControlDeviceV1;

    #[inline(always)]
    fn create_data_source<D>(self, conn: &mut Connection<D>) -> ExtDataControlSourceV1 {
        ExtDataControlManagerV1::create_data_source(self, conn)
    }

    #[inline(always)]
    fn create_data_source_with_cb<D>(
        self,
        conn: &mut Connection<D>,
        cb: impl FnMut(EventCtx<'_, D, ExtDataControlSourceV1>) + Send + 'static,
    ) -> ExtDataControlSourceV1 {
        ExtDataControlManagerV1::create_data_source_with_cb(self, conn, cb)
    }

    #[inline(always)]
    fn get_data_device<D>(self, conn: &mut Connection<D>, seat: WlSeat) -> ExtDataControlDeviceV1 {
        ExtDataControlManagerV1::get_data_device(self, conn, seat)
    }

    #[inline(always)]
    fn get_data_device_with_cb<D>(
        self,
        conn: &mut Connection<D>,
        seat: WlSeat,
        cb: impl FnMut(EventCtx<'_, D, ExtDataControlDeviceV1>) + Send + 'static,
    ) -> ExtDataControlDeviceV1 {
        ExtDataControlManagerV1::get_data_device_with_cb(self, conn, seat, cb)
    }

    #[inline(always)]
    fn destroy<D>(self, conn: &mut Connection<D>) {
        ExtDataControlManagerV1::destroy(self, conn)
    }
}

impl DataControlManagerV1 for ZwlrDataControlManagerV1 {
    const TYPE_NAME: &'static str = "zwlr_data_control_manager_v1";

    type DataControlSource = ZwlrDataControlSourceV1;
    type DataControlDevice = ZwlrDataControlDeviceV1;

    #[inline(always)]
    fn create_data_source<D>(self, conn: &mut Connection<D>) -> ZwlrDataControlSourceV1 {
        ZwlrDataControlManagerV1::create_data_source(self, conn)
    }

    #[inline(always)]
    fn create_data_source_with_cb<D>(
        self,
        conn: &mut Connection<D>,
        cb: impl FnMut(EventCtx<'_, D, ZwlrDataControlSourceV1>) + Send + 'static,
    ) -> ZwlrDataControlSourceV1 {
        ZwlrDataControlManagerV1::create_data_source_with_cb(self, conn, cb)
    }

    #[inline(always)]
    fn get_data_device<D>(self, conn: &mut Connection<D>, seat: WlSeat) -> ZwlrDataControlDeviceV1 {
        ZwlrDataControlManagerV1::get_data_device(self, conn, seat)
    }

    #[inline(always)]
    fn get_data_device_with_cb<D>(
        self,
        conn: &mut Connection<D>,
        seat: WlSeat,
        cb: impl FnMut(EventCtx<'_, D, ZwlrDataControlDeviceV1>) + Send + 'static,
    ) -> ZwlrDataControlDeviceV1 {
        ZwlrDataControlManagerV1::get_data_device_with_cb(self, conn, seat, cb)
    }

    #[inline(always)]
    fn destroy<D>(self, conn: &mut Connection<D>) {
        ZwlrDataControlManagerV1::destroy(self, conn)
    }
}

/// An attempted conversion that consumes `self` and tries to output a generic event.
pub(crate) trait TryIntoGenericEvent<GenericEvent> {
    /// Performs the conversion.
    ///
    /// The seat name is used for logging in case of an unsuccessful conversion.
    fn try_into_generic_event(self, seat_name: u32) -> Option<GenericEvent>;
}

/// A generic wrapper for data control offer events.
#[derive(Debug)]
pub(crate) enum DataControlOfferEvent {
    Offer(CString),
}

impl TryIntoGenericEvent<DataControlOfferEvent> for ext_data_control_offer_v1::Event {
    #[inline(always)]
    fn try_into_generic_event(self, seat_name: u32) -> Option<DataControlOfferEvent> {
        match self {
            ext_data_control_offer_v1::Event::Offer(cstring) => Some(DataControlOfferEvent::Offer(cstring)),
            fallback => {
                log::debug!(
                    target: &log_seat_target(seat_name),
                    "ext_data_control_offer_v1::Event: unhandled event: {:?}",
                    fallback,
                );
                None
            }
        }
    }
}

impl TryIntoGenericEvent<DataControlOfferEvent> for zwlr_data_control_offer_v1::Event {
    #[inline(always)]
    fn try_into_generic_event(self, seat_name: u32) -> Option<DataControlOfferEvent> {
        match self {
            zwlr_data_control_offer_v1::Event::Offer(cstring) => Some(DataControlOfferEvent::Offer(cstring)),
            fallback => {
                log::debug!(
                    target: &log_seat_target(seat_name),
                    "zwlr_data_control_offer_v1::Event: unhandled event: {:?}",
                    fallback,
                );
                None
            }
        }
    }
}

/// A generic wrapper for data control source events.
#[derive(Debug)]
pub(crate) enum DataControlSourceEvent {
    Send(ext_data_control_source_v1::SendArgs),
    Cancelled,
}

impl TryIntoGenericEvent<DataControlSourceEvent> for ext_data_control_source_v1::Event {
    #[inline(always)]
    fn try_into_generic_event(self, seat_name: u32) -> Option<DataControlSourceEvent> {
        match self {
            ext_data_control_source_v1::Event::Send(send_args) => Some(DataControlSourceEvent::Send(send_args)),
            ext_data_control_source_v1::Event::Cancelled => Some(DataControlSourceEvent::Cancelled),
            fallback => {
                log::debug!(
                    target: &log_seat_target(seat_name),
                    "ext_data_control_source_v1::Event: unhandled event: {:?}",
                    fallback
                );
                None
            }
        }
    }
}

impl TryIntoGenericEvent<DataControlSourceEvent> for zwlr_data_control_source_v1::Event {
    #[inline(always)]
    fn try_into_generic_event(self, seat_name: u32) -> Option<DataControlSourceEvent> {
        match self {
            zwlr_data_control_source_v1::Event::Send(send_args) => {
                Some(DataControlSourceEvent::Send(ext_data_control_source_v1::SendArgs {
                    mime_type: send_args.mime_type,
                    fd: send_args.fd,
                }))
            }
            zwlr_data_control_source_v1::Event::Cancelled => Some(DataControlSourceEvent::Cancelled),
            fallback => {
                log::debug!(
                    target: &log_seat_target(seat_name),
                    "zwlr_data_control_source_v1::Event: unhandled event: {:?}",
                    fallback
                );
                None
            }
        }
    }
}

/// A generic wrapper for data control device events.
#[derive(Debug)]
pub(crate) enum DataControlDeviceEvent<DataControlOffer: DataControlOfferV1> {
    DataOffer(DataControlOffer),
    Selection(Option<ObjectId>),
    PrimarySelection(Option<ObjectId>),
    Finished,
}

impl TryIntoGenericEvent<DataControlDeviceEvent<ExtDataControlOfferV1>> for ext_data_control_device_v1::Event {
    #[inline(always)]
    fn try_into_generic_event(self, seat_name: u32) -> Option<DataControlDeviceEvent<ExtDataControlOfferV1>> {
        match self {
            ext_data_control_device_v1::Event::DataOffer(ext_data_control_offer_v1) => {
                Some(DataControlDeviceEvent::DataOffer(ext_data_control_offer_v1))
            }
            ext_data_control_device_v1::Event::Selection(object_id) => {
                Some(DataControlDeviceEvent::Selection(object_id))
            }
            ext_data_control_device_v1::Event::PrimarySelection(object_id) => {
                Some(DataControlDeviceEvent::PrimarySelection(object_id))
            }
            ext_data_control_device_v1::Event::Finished => Some(DataControlDeviceEvent::Finished),
            fallback => {
                log::debug!(
                    target: &log_seat_target(seat_name),
                    "ext_data_control_device_v1::Event: unhandled event: {:?}",
                    fallback,
                );
                None
            }
        }
    }
}

impl TryIntoGenericEvent<DataControlDeviceEvent<ZwlrDataControlOfferV1>> for zwlr_data_control_device_v1::Event {
    #[inline(always)]
    fn try_into_generic_event(self, seat_name: u32) -> Option<DataControlDeviceEvent<ZwlrDataControlOfferV1>> {
        match self {
            zwlr_data_control_device_v1::Event::DataOffer(zwlr_data_control_offer_v1) => {
                Some(DataControlDeviceEvent::DataOffer(zwlr_data_control_offer_v1))
            }
            zwlr_data_control_device_v1::Event::Selection(object_id) => {
                Some(DataControlDeviceEvent::Selection(object_id))
            }
            zwlr_data_control_device_v1::Event::PrimarySelection(object_id) => {
                Some(DataControlDeviceEvent::PrimarySelection(object_id))
            }
            zwlr_data_control_device_v1::Event::Finished => Some(DataControlDeviceEvent::Finished),
            fallback => {
                log::debug!(
                    target: &log_seat_target(seat_name),
                    "zwlr_data_control_device_v1::Event: unhandled event: {:?}",
                    fallback,
                );
                None
            }
        }
    }
}
