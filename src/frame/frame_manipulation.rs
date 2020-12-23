use crate::Display;
use nuts::DomainState;

use crate::{quicksilver_compat::Rectangle, Domain, Frame, FrameHandle, NutsCheck};

struct FrameInitialization {
    div: div::PaneHandle,
    region: Rectangle,
}

pub(crate) struct FrameManipulator {
    _private: (),
}

impl FrameManipulator {
    pub(crate) fn init() {
        let fm = FrameManipulator { _private: () };
        let domain = Domain::Frame;
        let aid = nuts::new_domained_activity(fm, &domain);
        aid.private_domained_channel(Self::priv_init_frame);
    }
    pub(crate) fn init_frame<F: Frame>(f: &FrameHandle<F>) {
        nuts::send_to::<FrameManipulator, _>(FrameInitialization {
            div: f.div().clone(),
            region: f.region,
        });
    }
    fn priv_init_frame(&mut self, domain: &mut DomainState, init: FrameInitialization) {
        let display = Display::from_domain(domain);
        let region = init.region * display.game_to_browser_coordinates();
        init.div
            .reposition_and_resize(
                region.x() as u32,
                region.y() as u32,
                region.width() as u32,
                region.height() as u32,
            )
            .nuts_check();
    }
}
