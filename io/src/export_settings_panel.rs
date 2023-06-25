use crate::Snapshot;
use audio::exporter::Exporter;
use audio::{Command, Conn};
mod mid;

pub(super) fn set_exporter(
    c0: Vec<Command>,
    conn: &mut Conn,
    exporter: &Exporter,
) -> Option<Snapshot> {
    let c1 = vec![Command::SetExporter {
        exporter: Box::new(exporter.clone()),
    }];
    let snapshot = Some(Snapshot::from_commands(c0, &c1));
    conn.send(c1);
    snapshot
}
