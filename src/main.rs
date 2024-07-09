use std::num::NonZeroU32;
use std::rc::Rc;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use crate::camera::{Camera, Screen};
use crate::vec3::Vec3;

mod vec3;
mod camera;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(WindowBuilder::new().build(&event_loop).unwrap());
    let context = softbuffer::Context::new(window.clone()).unwrap();
    let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::WindowEvent { window_id, event: WindowEvent::RedrawRequested } if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };
                surface.resize(
                    NonZeroU32::new(width).unwrap(),
                    NonZeroU32::new(height).unwrap(),
                ).unwrap();

                let mut buffer = surface.buffer_mut().unwrap();

                let screen = Screen::new(
                  Vec3::new(0.0, 0.0, 0.0),
                  Vec3::new(0.0, 20.0, 0.0),
                  Vec3::new(20.0, 0.0, 0.0),
                  Vec3::new(20.0, 20.0, 0.0)
                );

                let camera = Camera::new(Vec3::new(10.0, 10.0, 10.0), screen);

                let cube_vertex_1 = Vec3::new(5.0, 5.0, 20.0);
                let cube_vertex_2 = Vec3::new(5.0, 15.0, 20.0);
                let cube_vertex_3 = Vec3::new(15.0, 5.0, 20.0);
                let cube_vertex_4 = Vec3::new(15.0, 15.0, 20.0);

                let cube_vertex_5 = Vec3::new(5.0, 5.0, 30.0);
                let cube_vertex_6 = Vec3::new(5.0, 15.0, 30.0);
                let cube_vertex_7 = Vec3::new(15.0, 5.0, 30.0);
                let cube_vertex_8 = Vec3::new(15.0, 15.0, 40.0);

                let edge_1 = (&cube_vertex_1, &cube_vertex_2);
                let edge_2 = (&cube_vertex_3, &cube_vertex_4);
                let edge_3 = (&cube_vertex_1, &cube_vertex_3);
                let edge_4 = (&cube_vertex_2, &cube_vertex_4);

                let edge_5 = (&cube_vertex_5, &cube_vertex_6);
                let edge_6 = (&cube_vertex_7, &cube_vertex_8);
                let edge_7 = (&cube_vertex_5, &cube_vertex_7);
                let edge_8 = (&cube_vertex_6, &cube_vertex_8);

                let edge_9 = (&cube_vertex_1, &cube_vertex_5);
                let edge_10 = (&cube_vertex_2, &cube_vertex_6);
                let edge_11 = (&cube_vertex_3, &cube_vertex_7);
                let edge_12 = (&cube_vertex_4, &cube_vertex_8);

                let edges = vec![
                    edge_1, edge_2, edge_3, edge_4,
                    edge_5, edge_6, edge_7, edge_8,
                    edge_9, edge_10, edge_11, edge_12,
                ];

                let projected_edges : Vec<((f32, f32), (f32, f32))> = edges.iter()
                    .map(|edge| (camera.find_projection(edge.0), camera.find_projection(edge.1)))
                    .collect();

                for index in 0..(width * height) {
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;

                    let current_x = index % width;
                    let current_y = index / width;

                    let mut is_point_on_line = false;
                    for projected_edge in projected_edges.iter() {
                        let x1 = (projected_edge.0.0 * width as f32).round() as u32;
                        let y1 = (projected_edge.0.1 * height as f32).round() as u32;
                        let x2 = (projected_edge.1.0 * width as f32).round() as u32;
                        let y2 = (projected_edge.1.1 * height as f32).round() as u32;
                        if check_point_on_line(x1, y1, x2, y2, current_x, current_y) {
                            is_point_on_line = true;
                        }

                    }

                    if is_point_on_line {
                        red = 0;
                        green = 255;
                        blue = 0;
                    }

                    buffer[index as usize] = blue | (green << 8) | (red << 16);
                }

                buffer.present().unwrap();
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id
            } if window_id == window.id() => {
                elwt.exit();
            }
            _ => {}
        }
    }).unwrap();
}

fn check_point_on_line(x1: u32,
                       y1: u32,
                       x2: u32,
                       y2: u32,
                       current_x: u32,
                       current_y: u32) -> bool {

    if x1 <= x2 && (current_x < x1 || current_x > x2) {
        return false;
    }

    if x1 >= x2 && (current_x < x2 || current_x > x1) {
        return false;
    }

    if y1 <= y2 && (current_y < y1 || current_y > y2) {
        return false;
    }

    if y1 >= y2 && (current_y < y2 || current_y > y1) {
        return false;
    }

    if x1 == x2 {
        return true;
    }

    let calculated_y = calculate_y(x1, y1, x2, y2, current_x);
    if current_y == calculated_y {
        true
    } else {
        false
    }
}

fn calculate_y(x1: u32, y1: u32, x2: u32, y2: u32, x: u32) -> u32 {
    let x1 = x1 as f32;
    let y1 = y1 as f32;
    let x2 = x2 as f32;
    let y2 = y2 as f32;
    let slope = (y2 - y1) / (x2 - x1);
    let b: f32 = y1 - slope * x1;
    return (slope * x as f32 + b) as u32;
}