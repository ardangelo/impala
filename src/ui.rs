use std::sync::atomic::Ordering;

use ratatui::Frame;

use crate::app::{App, FocusedBlock};

use crate::auth::Auth;

use crate::tui::Palette;

pub fn render(app: &mut App, palette: &Palette, frame: &mut Frame) {
    // Select mode
    if app.reset_mode {
        app.render(palette, frame);
    } else {
        // App
        app.adapter.render(palette, frame, app.focused_block);

        if app.focused_block == FocusedBlock::AdapterInfos {
            app.adapter.render_adapter(palette, frame);
        }

        // Auth Popup
        if app.authentication_required.load(Ordering::Relaxed) {
            app.focused_block = FocusedBlock::AuthKey;
            Auth.render(palette, frame, app.passkey_input.value());
        }

        // Access Point Popup
        if let Some(ap) = &app.adapter.device.access_point {
            if ap.ap_start.load(Ordering::Relaxed) {
                app.focused_block = FocusedBlock::AccessPointInput;
                ap.render_input(palette, frame);
            }
        }

        // Help
        if let FocusedBlock::Help = app.focused_block {
            app.help.render(palette, frame);
        }

        // Notifications
        for (index, notification) in app.notifications.iter().enumerate() {
            notification.render(index, palette, frame);
        }
    }
}
