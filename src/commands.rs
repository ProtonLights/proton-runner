use dmx_output::DmxOutput;
use error::Error;

/// Bind value to range [1, 512]
fn bound(unbounded: u32) -> u32 {
    if unbounded < 1 { 1 }
    else if unbounded > 512 { 512 }
    else { unbounded }
}

pub fn range_on<D: DmxOutput>(dmx: &mut D, u_start: u32, u_end: u32) -> Result<(), Error> {
    let start = bound(u_start);
    let end = bound(u_end);

    let first_part = vec![0; start as usize - 1];
    let mut on_part = vec![255; (end - start + 1) as usize];
    let mut last_part = vec![0; 512 - end as usize];

    let mut data = first_part;
    data.append(&mut on_part);
    data.append(&mut last_part);

    dmx.send(&data)
}

pub fn range_off<D: DmxOutput>(dmx: &mut D, u_start: u32, u_end: u32) -> Result<(), Error> {
    let start = bound(u_start);
    let end = bound(u_end);

    let first_part = vec![255; start as usize - 1];
    let mut on_part = vec![0; (end - start + 1) as usize];
    let mut last_part = vec![255; 512 - end as usize];

    let mut data = first_part;
    data.append(&mut on_part);
    data.append(&mut last_part);

    dmx.send(&data)
}

pub fn all_on<D: DmxOutput>(dmx: &mut D) -> Result<(), Error> {
    range_on(dmx, 1, 512)
}

pub fn all_off<D: DmxOutput>(dmx: &mut D) -> Result<(), Error> {
    range_off(dmx, 1, 512)
}
