use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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

    pub fn collider(&self) -> Collider {
        let shapes = self
            .quads
            .iter()
            .map(|quad| quad.collider(self.height as f32))
            .collect();

        Collider::compound(shapes)
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
    fn collider(&self, y_shift: f32) -> (Vec2, f32, Collider) {
        let half_width = self.width as f32 / 2.0;
        let half_height = self.height as f32 / 2.0;
        let collider = Collider::cuboid(half_width, half_height);

        let position = Vec2::new(
            (self.x as f32) + half_width,
            (self.y as f32) - half_height - y_shift,
        );

        (position, 0.0, collider)
    }
}
