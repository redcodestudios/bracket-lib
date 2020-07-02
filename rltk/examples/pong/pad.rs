use rltk::{Rltk, VirtualKeyCode};

use legion::prelude::*;

use crate::component::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Side {
    Left,
    Right
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pad(pub Side);

fn move_up(transform: &mut Transform) {
    transform.y -= 1;
}

fn move_down(transform: &mut Transform) {
    transform.y += 1;
}

pub fn move_sys_build() -> Box<dyn Schedulable + 'static> {
     SystemBuilder::new("update_positions")
         .read_resource::<Rltk>()
         .with_query(<(Write<Transform>, Tagged<Pad>)>::query())
         .build(|_, world, ctx, query| {

        for (mut transform, tag) in query.iter_mut(world) {
            match tag {
                Pad(Side::Left) => {
                    match ctx.key {
                        Some(key) => {
                           match key {
                               VirtualKeyCode::W => move_up(&mut transform),
                               VirtualKeyCode::S => move_down(&mut transform),
                               _ => ()
                           } 
                        },
                        None => ()
                    }
                },
                Pad(Side::Right) => {
                    match ctx.key {
                        Some(key) => {
                           match key {
                               VirtualKeyCode::Up => move_up(&mut transform),
                               VirtualKeyCode::Down => move_down(&mut transform),
                                   _ => ()
                           } 
                        },
                        None => ()
                    } 
                },

                _ => ()
            }
        }
        println!("_");
    })
}
