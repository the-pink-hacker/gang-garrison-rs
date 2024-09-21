// Implemented off of: https://0fps.net/2012/06/30/meshing-in-a-minecraft-game/

use std::cmp::Ordering;

use bevy::prelude::*;

use crate::map::collision::BITMASK_BITS_PER_BYTE;

use super::WalkBitMask;

const TEMP_SHIFT_Y: f32 = 200.0;

pub struct WalkQuadMask {
    quads: Vec<Quad>,
}

impl WalkQuadMask {
    pub fn from_bits(walk_bit_mask: WalkBitMask) -> Self {
        let quads = walk_bit_mask
            .mask
            .into_iter()
            .flat_map(|byte| {
                (0..BITMASK_BITS_PER_BYTE)
                    .rev()
                    .map(move |bit_index| ((byte >> bit_index) & 1) != 0)
            })
            .enumerate()
            .flat_map(|(index, collidable)| {
                let x = (index % walk_bit_mask.width as usize) as u16;
                let y = walk_bit_mask.height - (index / walk_bit_mask.width as usize) as u16;
                collidable.then(|| Quad::square_unit(x, y))
            })
            .collect();

        Self { quads }
    }

    pub fn triangulate(self) -> WalkMeshMask {
        let (vertices, indices) = self
            .quads
            .into_iter()
            .enumerate()
            .map(|(quad_index, quad)| {
                let quad_index = quad_index as u32 * 4;
                (
                    quad.vertices(),
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

#[derive(Debug, PartialEq, Eq)]
struct Quad {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

impl Quad {
    const fn square_unit(x: u16, y: u16) -> Self {
        Self { x, y, w: 1, h: 1 }
    }

    fn vertices(&self) -> [Vec2; 4] {
        let (x, y, w, h) = self.into();
        let (x, y, w, h) = (x as f32, y as f32 - TEMP_SHIFT_Y, w as f32, h as f32);
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
        (value.x, value.y, value.w, value.h)
    }
}

impl Ord for Quad {
    fn cmp(&self, other: &Self) -> Ordering {
        let (x0, y0, w0, h0) = self.into();
        let (x1, y1, w1, h1) = other.into();

        if y0 != y1 {
            if y0 < y1 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else if x0 != x1 {
            if x0 < x1 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else if w0 != w1 {
            if w0 > w1 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else if h0 >= h1 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for Quad {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
pub struct WalkMeshMask {
    pub vertices: Vec<Vec2>,
    pub indices: Vec<[u32; 3]>,
}
