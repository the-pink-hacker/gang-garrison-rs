use bevy::prelude::*;

use crate::map::collision::BITMASK_BITS_PER_BYTE;

use super::WalkBitMask;

pub struct WalkQuadMask {
    quads: Vec<Quad>,
    height: u16,
}

impl WalkQuadMask {
    pub fn from_bits(walk_bit_mask: WalkBitMask) -> Self {
        let mut mask = walk_bit_mask
            .mask
            .into_iter()
            .flat_map(|byte| {
                (0..BITMASK_BITS_PER_BYTE)
                    .rev()
                    .map(move |bit_index| ((byte >> bit_index) & 1) != 0)
            })
            .collect::<Vec<_>>();

        let width = walk_bit_mask.width as usize;
        let height = walk_bit_mask.height as usize;
        let mut quads = Vec::with_capacity(width * height);

        for quad_y in 0..height {
            let mut x_positions = 0..width;

            while let Some(quad_x) = x_positions.next() {
                let collidable = &mut mask[quad_x + quad_y * width];

                if !*collidable {
                    continue;
                }

                *collidable = false;

                // Count collidable tiles in x direction.
                let mut quad_width = 1;

                for x in x_positions.by_ref() {
                    let collidable = &mut mask[x + quad_y * width];
                    if !*collidable {
                        break;
                    }

                    quad_width += 1;
                    *collidable = false;
                }

                // Count collidable tiles in y direction.
                let mut quad_height = 1;

                'outer: for y in (quad_y + 1)..height {
                    let y_offset = y * width;

                    // Check for possible quad breakups.
                    if (quad_x != 0 && mask[quad_x - 1 + y_offset])
                        || (quad_x + quad_width as usize != width
                            && mask[quad_x + quad_width as usize + y_offset])
                    {
                        break;
                    }

                    // Scan if can merge.
                    for x in quad_x..(quad_x + quad_width as usize) {
                        if !mask[x + y_offset] {
                            // Failed to merge row.
                            break 'outer;
                        }
                    }

                    quad_height += 1;

                    for x in quad_x..(quad_x + quad_width as usize) {
                        // Already known to be collidable.
                        mask[x + y_offset] = false;
                    }
                }

                quads.push(Quad {
                    x: quad_x as u16,
                    y: (height - quad_y) as u16,
                    width: quad_width,
                    height: quad_height,
                });
            }
        }

        Self {
            quads,
            height: walk_bit_mask.height,
        }
    }

    pub fn triangulate(self) -> WalkMeshMask {
        let (vertices, indices) = self
            .quads
            .into_iter()
            .enumerate()
            .map(|(quad_index, quad)| {
                let quad_index = quad_index as u32 * 4;
                (
                    quad.vertices(self.height as f32),
                    [
                        [quad_index, quad_index + 1, quad_index + 2],
                        [quad_index + 1, quad_index + 2, quad_index + 3],
                    ],
                )
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        WalkMeshMask {
            vertices: vertices.into_flattened(),
            indices: indices.into_flattened(),
        }
    }
}

#[derive(Debug)]
struct Quad {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

impl Quad {
    fn vertices(&self, y_shift: f32) -> [Vec2; 4] {
        let (x, y, w, h) = self.into();
        let (x, y, w, h) = (x as f32, y as f32 - y_shift, w as f32, h as f32);
        [
            Vec2::new(x, y),
            Vec2::new(x + w, y),
            Vec2::new(x, y - h),
            Vec2::new(x + w, y - h),
        ]
    }
}

impl From<&Quad> for (u16, u16, u16, u16) {
    fn from(value: &Quad) -> Self {
        (value.x, value.y, value.width, value.height)
    }
}

#[derive(Debug)]
pub struct WalkMeshMask {
    pub vertices: Vec<Vec2>,
    pub indices: Vec<[u32; 3]>,
}
