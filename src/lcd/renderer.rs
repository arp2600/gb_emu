use super::pixel_iterator::PixelIterator;
use crate::bit_ops::BitGetSet;
use crate::memory::{locations::*, VideoMemory};
use crate::App;

pub struct Renderer {
    background_screen_buffer: [u8; 256 * 256],
    background_tile_map_cache: [u16; 32 * 32],
    background_tile_write_cache: [u64; 32 * 32],
    frame_count: u64,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            background_screen_buffer: [0; 256 * 256],
            background_tile_map_cache: [0; 32 * 32],
            background_tile_write_cache: [0; 32 * 32],
            frame_count: 0,
        }
    }

    pub fn draw_background(&mut self, vram: &mut VideoMemory) {
        self.frame_count += 1;

        let tile_data_start = vram.get_tile_data_select();
        let tile_map_start = vram.get_bg_tilemap_display_select();
        let mut update_indices = Vec::new();
        for (i, v) in self.background_tile_map_cache.iter_mut().enumerate() {
            let tile_index = u16::from(vram[tile_map_start as usize + i]);
            let tile_address = match tile_data_start {
                TILE_DATA_1 => {
                    let x = (tile_index as i8) as i16;
                    assert!(x >= -128 && x <= 127, "x = {}", x);
                    let x = ((x + 128) * 16) as u16;
                    tile_data_start + x
                }
                TILE_DATA_2 => tile_data_start + tile_index * 16,
                _ => unreachable!(),
            };

            let tile_index = usize::from((tile_address - 0x8000) / 16);
            let is_dirty = {
                let a = self.background_tile_write_cache[tile_index];
                let b = vram.tile_write_counts[tile_index];
                a != b
            };
            if *v != tile_address || is_dirty {
                *v = tile_address;
                self.background_tile_write_cache[tile_index] = vram.tile_write_counts[tile_index];
                update_indices.push(i);
            }
        }

        for i in update_indices.iter() {
            self.draw_background_tile(*i, vram);
        }
    }

    fn draw_background_tile(&mut self, tile_index: usize, vram: &VideoMemory) {
        let x = tile_index % 32;
        let y = tile_index / 32;

        let tile_map_cache = &mut self.background_tile_map_cache;
        let tile_address = tile_map_cache[usize::from(x + y * 32)];
        for yi in 0..8 {
            let tile_y_index = yi as u16;
            let line_address = tile_address + tile_y_index * 2;

            let pixels = vram.get_u16(line_address as usize);
            for (xi, pixel) in PixelIterator::new(pixels).enumerate() {
                let index = (x * 8 + xi) + y * 256 * 8 + yi * 256;
                self.background_screen_buffer[index] = pixel;
            }
        }
    }

    pub fn draw_line<T: App>(&self, vram: &VideoMemory, app: &mut T) {
        let mut line = [0; 160];

        self.draw_bg_line(vram, &mut line);
        draw_windows(vram, &mut line);
        draw_sprites(vram, &mut line);

        let bgp = vram.regs.bgp;
        let obp1 = vram.get_obp(0);
        let obp2 = vram.get_obp(2);
        let palette = create_combined_palette(bgp, obp1, obp2);
        for x in line.iter_mut() {
            *x = palette[*x as usize];
        }

        let ly = vram.regs.ly;
        app.draw_line(&line, ly);
    }

    fn draw_bg_line(&self, vram: &VideoMemory, line: &mut [u8; 160]) {
        let y = usize::from(vram.regs.ly.wrapping_add(vram.regs.scy));
        for x in 0..160 {
            let scx = vram.regs.scx;
            let i = y * 256 + usize::from((x as u8).wrapping_add(scx));
            line[x as usize] = self.background_screen_buffer[i];
        }
    }
}

fn create_combined_palette(bgp: u8, obp1: u8, obp2: u8) -> [u8; 12] {
    [
        3 - (bgp & 0b11),
        3 - ((bgp >> 2) & 0b11),
        3 - ((bgp >> 4) & 0b11),
        3 - ((bgp >> 6) & 0b11),
        3 - (obp1 & 0b11),
        3 - ((obp1 >> 2) & 0b11),
        3 - ((obp1 >> 4) & 0b11),
        3 - ((obp1 >> 6) & 0b11),
        3 - (obp2 & 0b11),
        3 - ((obp2 >> 2) & 0b11),
        3 - ((obp2 >> 4) & 0b11),
        3 - ((obp2 >> 6) & 0b11),
    ]
}

fn get_window_tile_index(x: u16, y: u16, vram: &VideoMemory) -> u16 {
    let tile_map = vram.get_window_tilemap_display_select();
    let i = tile_map + x + 32 * y;
    u16::from(vram[i as usize])
}

fn draw_windows(vram: &VideoMemory, line: &mut [u8; 160]) {
    if !vram.is_window_enabled() {
        return;
    }
    if vram.regs.ly < vram.regs.wy {
        return;
    }

    let ly = vram.regs.ly;

    // Look at each tile on the current line
    for x in 0..(256 / 8) {
        let y = u16::from((ly - vram.regs.wy) / 8);

        // Get the index of the tile data
        let tile_data_index = get_window_tile_index(x, y, vram);

        // Get the address of the tile
        let tile_data_start = vram.get_tile_data_select();
        let tile_address = match tile_data_start {
            TILE_DATA_1 => {
                let x = (tile_data_index as i8) as i16;
                assert!(x >= -128 && x <= 127, "x = {}", x);
                let x = ((x + 128) * 16) as u16;
                tile_data_start + x
            }
            TILE_DATA_2 => tile_data_start + tile_data_index * 16,
            _ => unreachable!(),
        };
        let tile_y_index = u16::from(ly % 8);
        let line_address = tile_address + tile_y_index * 2;

        let pixels = vram.get_u16(line_address as usize);
        for (i, pixel) in PixelIterator::new(pixels).enumerate() {
            let line_index = {
                let t = (x as u8 * 8) + i as u8;
                let wx = vram.regs.wx.wrapping_sub(7) as usize;
                t as usize + wx
            };

            if line_index < line.len() {
                line[line_index] = pixel;
            }
        }
    }
}

fn draw_sprites(vram: &VideoMemory, line: &mut [u8; 160]) {
    if !vram.are_sprites_enabled() {
        return;
    }

    for i in 0..40 {
        let sprite_height = vram.get_sprite_width();
        let oam_index = usize::from(SPRITE_ATTRIBUTE_TABLE + i * 4);
        let y = if sprite_height == 16 {
            vram[oam_index]
        } else {
            vram[oam_index].wrapping_sub(9)
        };
        let x = vram[oam_index + 1].wrapping_sub(8);
        if y >= vram.regs.ly && y < (vram.regs.ly + sprite_height) {
            let tile_num = {
                let x = vram[usize::from(oam_index + 2)] as u16;
                if vram.get_sprite_width() == 16 {
                    x & 0b11111110
                } else {
                    x
                }
            };
            let attributes = vram[usize::from(oam_index + 3)];

            let y_flip = attributes.get_bit(6);
            let x_flip = attributes.get_bit(5);
            let palette = attributes.get_bit(4) as u8;
            let tile_address = SPRITE_PATTERN_TABLE + tile_num * 16;
            let tile_y_index = {
                let y = u16::from(y - vram.regs.ly);
                if y_flip {
                    y
                } else {
                    u16::from(sprite_height) - 1 - y
                }
            };
            let line_address = tile_address + tile_y_index * 2;

            let pixels = vram.get_u16(line_address as usize);
            let bg_priority = attributes.get_bit(7);

            for (i, pixel) in PixelIterator::new(pixels).enumerate() {
                let index = if x_flip {
                    usize::from(x) + 7 - i
                } else {
                    usize::from(x) + i
                };

                if index < line.len() {
                    if pixel > 0 {
                        let pixel = pixel + 4 + 4 * palette;

                        if bg_priority && line[index] > 0 {
                            continue;
                        }

                        if x_flip {
                            line[index] = pixel;
                        } else {
                            line[index] = pixel;
                        }
                    }
                }
            }
        }
    }
}
