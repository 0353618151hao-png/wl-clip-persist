use std::ffi::CString;
use std::os::fd::OwnedFd;

use wayrs_client::core::ObjectId;
use wayrs_client::object::Proxy;
use wayrs_client::protocol::WlSeat;
use wayrs_client::{Connection, EventCtx};
use wayrs_protocols::ext_data_control_v1::*;
use wayrs_protocols::wlr_data_control_unstable_v1::*;

use crate::logger::log_seat_target;

pub(crate) trait DataControlOfferV1: Clone + Copy + Send + Sync + Proxy {
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

pub(crate) trait DataControlSourceV1: Clone + Copy + Send + Sync + Proxy {
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

pub(crate) trait DataControlDeviceV1<DataControlSource: DataControlSourceV1>:
    Clone + Copy + Send + Sync + Proxy
{
    const TYPE_NAME: &'static str;

    fn set_selection<D>(self, conn: &mut Connection<D>, source: Option<DataControlSource>);

    fn set_primary_selection<D>(self, conn: &mut Connection<D>, source: Option<DataControlSource>);

    fn destroy<D>(self, conn: &mut Connection<D>);
}

impl DataControlDeviceV1<ExtDataControlSourceV1> for ExtDataControlDeviceV1 {
    const TYPE_NAME: &'static str = "ext_data_control_device_v1";

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

impl DataControlDeviceV1<ZwlrDataControlSourceV1> for ZwlrDataControlDeviceV1 {
    const TYPE_NAME: &'static str = "zwlr_data_control_device_v1";

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

pub(crate) trait DataControlManagerV1<
    DataControlSource: DataControlSourceV1,
    DataControlDevice: DataControlDeviceV1<DataControlSource>,
>: Clone + Copy + Send + Sync + Proxy
{
    const TYPE_NAME: &'static str;

    fn create_data_source<D>(self, conn: &mut Connection<D>) -> DataControlSource;

    #[expect(dead_code)]
    fn create_data_source_with_cb<D>(
        self,
        conn: &mut Connection<D>,
        cb: impl FnMut(EventCtx<'_, D, DataControlSource>) + Send + 'static,
    ) -> DataControlSource;

    #[expect(dead_code)]
    fn get_data_device<D>(self, conn: &mut Connection<D>, seat: WlSeat) -> DataControlDevice;

    fn get_data_device_with_cb<D>(
        self,
        conn: &mut Connection<D>,
        seat: WlSeat,
        cb: impl FnMut(EventCtx<'_, D, DataControlDevice>) + Send + 'static,
    ) -> DataControlDevice;

    #[expect(dead_code)]
    fn destroy<D>(self, conn: &mut Connection<D>);
}

impl DataControlManagerV1<ExtDataControlSourceV1, ExtDataControlDeviceV1> for ExtDataControlManagerV1 {
    const TYPE_NAME: &'static str = "ext_data_control_manager_v1";

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

impl DataControlManagerV1<ZwlrDataControlSourceV1, ZwlrDataControlDeviceV1> for ZwlrDataControlManagerV1 {
    const TYPE_NAME: &'static str = "zwlr_data_control_manager_v1";

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

pub(crate) trait FromEvent<Event>
where
    Self: Sized,
{
    fn from(seat_name: u32, event: Event) -> Option<Self>;
}

#[derive(Debug)]
pub(crate) enum DataControlOfferEvent {
    Offer(CString),
}

impl FromEvent<ext_data_control_offer_v1::Event> for DataControlOfferEvent {
    #[inline(always)]
    fn from(seat_name: u32, event: ext_data_control_offer_v1::Event) -> Option<Self> {
        match event {
            ext_data_control_offer_v1::Event::Offer(cstring) => Some(Self::Offer(cstring)),
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

impl FromEvent<zwlr_data_control_offer_v1::Event> for DataControlOfferEvent {
    #[inline(always)]
    fn from(seat_name: u32, event: zwlr_data_control_offer_v1::Event) -> Option<Self> {
        match event {
            zwlr_data_control_offer_v1::Event::Offer(cstring) => Some(Self::Offer(cstring)),
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

#[derive(Debug)]
pub(crate) enum DataControlSourceEvent {
    Send(ext_data_control_source_v1::SendArgs),
    Cancelled,
}

impl FromEvent<ext_data_control_source_v1::Event> for DataControlSourceEvent {
    #[inline(always)]
    fn from(seat_name: u32, event: ext_data_control_source_v1::Event) -> Option<Self> {
        match event {
            ext_data_control_source_v1::Event::Send(send_args) => Some(Self::Send(send_args)),
            ext_data_control_source_v1::Event::Cancelled => Some(Self::Cancelled),
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

impl FromEvent<zwlr_data_control_source_v1::Event> for DataControlSourceEvent {
    #[inline(always)]
    fn from(seat_name: u32, event: zwlr_data_control_source_v1::Event) -> Option<Self> {
        match event {
            zwlr_data_control_source_v1::Event::Send(send_args) => {
                Some(Self::Send(ext_data_control_source_v1::SendArgs {
                    mime_type: send_args.mime_type,
                    fd: send_args.fd,
                }))
            }
            zwlr_data_control_source_v1::Event::Cancelled => Some(Self::Cancelled),
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

#[derive(Debug)]
pub(crate) enum DataControlDeviceEvent<DataControlOffer: DataControlOfferV1> {
    DataOffer(DataControlOffer),
    Selection(Option<ObjectId>),
    PrimarySelection(Option<ObjectId>),
    Finished,
}

impl FromEvent<ext_data_control_device_v1::Event> for DataControlDeviceEvent<ExtDataControlOfferV1> {
    #[inline(always)]
    fn from(seat_name: u32, event: ext_data_control_device_v1::Event) -> Option<Self> {
        match event {
            ext_data_control_device_v1::Event::DataOffer(ext_data_control_offer_v1) => {
                Some(Self::DataOffer(ext_data_control_offer_v1))
            }
            ext_data_control_device_v1::Event::Selection(object_id) => Some(Self::Selection(object_id)),
            ext_data_control_device_v1::Event::PrimarySelection(object_id) => Some(Self::PrimarySelection(object_id)),
            ext_data_control_device_v1::Event::Finished => Some(Self::Finished),
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

impl FromEvent<zwlr_data_control_device_v1::Event> for DataControlDeviceEvent<ZwlrDataControlOfferV1> {
    #[inline(always)]
    fn from(seat_name: u32, event: zwlr_data_control_device_v1::Event) -> Option<Self> {
        match event {
            zwlr_data_control_device_v1::Event::DataOffer(zwlr_data_control_offer_v1) => {
                Some(Self::DataOffer(zwlr_data_control_offer_v1))
            }
            zwlr_data_control_device_v1::Event::Selection(object_id) => Some(Self::Selection(object_id)),
            zwlr_data_control_device_v1::Event::PrimarySelection(object_id) => Some(Self::PrimarySelection(object_id)),
            zwlr_data_control_device_v1::Event::Finished => Some(Self::Finished),
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
