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
         .read_resource::<Option<VirtualKeyCode>>()
         .with_query(<(Write<Transform>, Tagged<Pad>)>::query())
         .build(|_, world, key_pressed, query| {
        match **key_pressed{
            None => {},
            Some(key) => {println!("ae: {}", key as i32);}
        };
        for (mut transform, tag) in query.iter_mut(world) {
            match tag {
                Pad(Side::Left) => {
                    match **key_pressed {
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
                    match **key_pressed {
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
        print!("");
    })
}
