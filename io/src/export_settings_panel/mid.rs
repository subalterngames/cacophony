use super::{set_exporter, ExportSetting};
use crate::panel::*;
use crate::{edit_optional_string, edit_string};

const MID_FIELDS: [ExportSetting; 3] = [
    ExportSetting::Title,
    ExportSetting::Artist,
    ExportSetting::Copyright,
];

pub struct Mid {
    /// The current field.
    field: Index,
}

impl Default for Mid {
    fn default() -> Self {
        Self {
            field: Index::new(0, MID_FIELDS.len()),
        }
    }
}

impl Panel for Mid {
    fn update(
        &mut self,
        state: &mut State,
        conn: &mut Conn,
        input: &Input,
        tts: &mut TTS,
        text: &Text,
        _: &mut PathsState,
        exporter: &mut Exporter,
    ) -> Option<Snapshot> {
        if MID_FIELDS[self.field.get()].say(state, input, text, tts) {
            None
        } else if input.happened(&InputEvent::PreviousExportSetting) {
            self.field.increment(false);
            None
        } else if input.happened(&InputEvent::NextExportSetting) {
            self.field.increment(true);
            None
        } else if input.happened(&InputEvent::ToggleExportSettingBoolean)
            && MID_FIELDS[self.field.get()] == ExportSetting::Copyright
        {
            let c0 = vec![Command::SetExporter {
                exporter: Box::new(exporter.clone()),
            }];
            exporter.copyright = !exporter.copyright;
            set_exporter(c0, conn, exporter)
        } else if MID_FIELDS[self.field.get()] == ExportSetting::Title {
            edit_string(|e| &mut e.metadata.title, input, conn, state, exporter)
        } else if MID_FIELDS[self.field.get()] == ExportSetting::Artist {
            edit_optional_string(|e| &mut e.metadata.artist, input, conn, state, exporter)
        } else {
            None
        }
    }
}
