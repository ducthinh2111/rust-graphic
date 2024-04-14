use std::num::NonZeroU32;
use std::rc::Rc;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

struct Gvector {
    x: f32,
    y: f32,
    z: f32
}

impl Gvector {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    fn subtract(point1: &Self, point2: &Self) -> Self {
        Self {
            x: point2.x - point1.x,
            y: point2.y - point1.y,
            z: point2.z - point1.z
        }
    }

    fn add(point1: &Self, point2: &Self) -> Self {
        Self {
            x: point1.x + point2.x,
            y: point1.y + point2.y,
            z: point1.z + point2.z
        }
    }

    fn scale(&self, scalar: f32) -> Self {
        Self {
            x: scalar * self.x,
            y: scalar * self.y,
            z: scalar * self.z,
        }
    }
}

struct Camera {
    pin_hole: Gvector,
    screen: Screen
}

impl Camera {
    fn get_center_point(&self) -> Gvector {
        let vector_horizontal = Gvector::subtract(&self.screen.top_left, &self.screen.top_right);
        let vector_vertical = Gvector::subtract(&self.screen.top_left, &self.screen.top_right);
        let half_vector_horizontal = vector_horizontal.scale(0.5);
        let half_vector_vertical = vector_vertical.scale(0.5);
        Gvector::add(&half_vector_horizontal, &half_vector_vertical)
    }

    fn center_vector(&self) -> Gvector {
        Gvector::subtract(&self.pin_hole, &self.get_center_point())
    }
}

struct Screen {
    top_left: Gvector,
    top_right: Gvector,
    bottom_left: Gvector,
    bottom_right: Gvector
}

fn find_projection(point: &Gvector, camera: &Camera) -> Gvector {
    let point_to_pin_hole = Gvector::subtract(&point, &camera.pin_hole);
    let lambda =
        - (
        camera.center_vector().x * camera.get_center_point().x - camera.center_vector().x * point.x +
        camera.center_vector().y * camera.get_center_point().y - camera.center_vector().y * point.y +
        camera.center_vector().z * camera.get_center_point().z - camera.center_vector().z * point.z
        ) / (
        camera.center_vector().x * point_to_pin_hole.x +
        camera.center_vector().y * point_to_pin_hole.y +
        camera.center_vector().z * point_to_pin_hole.z);

    Gvector {
        x: point.x + lambda * point_to_pin_hole.x,
        y: point.y + lambda * point_to_pin_hole.y,
        z: point.z + lambda * point_to_pin_hole.z,
    }

}

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

                let point = Gvector::new(0.5, 0.5, 2.0);
                let pin_hole = Gvector::new(1.0, 1.0, 1.0);
                let camera = Camera {
                    pin_hole,
                    length: 1.0
                };

                let x2 = 100;
                let y2 = 100;
                let x1 = 700;
                let y1 = 80;
                for index in 0..(width * height) {
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;

                    let current_x = index % width;
                    let current_y = index / width;

                    let is_point_on_line = is_point_on_line(x1,
                                                            y1,
                                                            x2,
                                                            y2,
                                                            current_x,
                                                            current_y);

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

fn is_point_on_line(
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
    current_x: u32,
    current_y: u32) -> bool {
    let mut first_x = x1;
    let mut first_y = y1;
    let mut second_x = x2;
    let mut second_y = y2;
    if x2 < x1 {
        first_x = x2;
        first_y = y2;
        second_x = x1;
        second_y = y1;
    }
    if current_x >= first_x && current_x <= second_x {
        let calculated_y = calculate_y(first_x, first_y, second_x, second_y, current_x);
        if current_y == calculated_y {
            true
        } else {
            false
        }
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