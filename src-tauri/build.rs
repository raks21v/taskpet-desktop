use std::{fs, io, path::Path};

const TRAY_ICON_PATH: &str = "icons/tray.ico";
const TRAY_ICON_SIZE: u32 = 16;

fn main() {
    write_generated_tray_icon(TRAY_ICON_PATH).expect("failed to generate TaskPets tray icon");
    tauri_build::build()
}

fn write_generated_tray_icon(path: impl AsRef<Path>) -> io::Result<()> {
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, generate_applybee_tray_icon())
}

fn generate_applybee_tray_icon() -> Vec<u8> {
    let width = TRAY_ICON_SIZE;
    let height = TRAY_ICON_SIZE;
    let mut xor_bitmap = Vec::with_capacity((width * height * 4) as usize);

    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = tray_pixel(x, y);
            xor_bitmap.extend_from_slice(&pixel);
        }
    }

    let and_mask_row_bytes = width.div_ceil(32) * 4;
    let and_mask = vec![0u8; (and_mask_row_bytes * height) as usize];
    let image_size = xor_bitmap.len() + and_mask.len();
    let dib_size = 40 + image_size;
    let icon_image_offset = 6 + 16;

    let mut ico = Vec::with_capacity(icon_image_offset + dib_size);

    // ICONDIR
    ico.extend_from_slice(&0u16.to_le_bytes());
    ico.extend_from_slice(&1u16.to_le_bytes());
    ico.extend_from_slice(&1u16.to_le_bytes());

    // ICONDIRENTRY
    ico.push(width as u8);
    ico.push(height as u8);
    ico.push(0);
    ico.push(0);
    ico.extend_from_slice(&1u16.to_le_bytes());
    ico.extend_from_slice(&32u16.to_le_bytes());
    ico.extend_from_slice(&(dib_size as u32).to_le_bytes());
    ico.extend_from_slice(&(icon_image_offset as u32).to_le_bytes());

    // BITMAPINFOHEADER
    ico.extend_from_slice(&40u32.to_le_bytes());
    ico.extend_from_slice(&(width as i32).to_le_bytes());
    ico.extend_from_slice(&((height * 2) as i32).to_le_bytes());
    ico.extend_from_slice(&1u16.to_le_bytes());
    ico.extend_from_slice(&32u16.to_le_bytes());
    ico.extend_from_slice(&0u32.to_le_bytes());
    ico.extend_from_slice(&(image_size as u32).to_le_bytes());
    ico.extend_from_slice(&0i32.to_le_bytes());
    ico.extend_from_slice(&0i32.to_le_bytes());
    ico.extend_from_slice(&0u32.to_le_bytes());
    ico.extend_from_slice(&0u32.to_le_bytes());

    ico.extend_from_slice(&xor_bitmap);
    ico.extend_from_slice(&and_mask);
    ico
}

fn tray_pixel(x: u32, y: u32) -> [u8; 4] {
    let center = (TRAY_ICON_SIZE - 1) as f32 / 2.0;
    let dx = x as f32 - center;
    let dy = y as f32 - center;
    let radius_squared = dx * dx + dy * dy;

    if radius_squared > 48.0 {
        return [0, 0, 0, 0];
    }

    if x == 5 || x == 10 {
        return [23, 43, 59, 255];
    }

    [74, 216, 255, 255]
}
