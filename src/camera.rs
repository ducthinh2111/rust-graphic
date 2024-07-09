use crate::vec3::Vec3;

pub struct Camera {
    pin_hole: Vec3,
    screen: Screen
}

impl Camera {

    pub fn new(pin_hole: Vec3, screen: Screen) -> Self {
        Self { pin_hole, screen }
    }
    fn get_normal_vector(&self) -> Vec3 {
        Vec3::from_two_position(&self.screen.center_position, &self.pin_hole)
    }

    /// Projection based on the calculation of
    /// line equation: r = u + lambda * v
    /// and plane equation: ax + by + cz = 0
    ///
    /// u: World position
    ///
    /// v: Vector from world position to camera's pinhole
    ///
    /// Find lambda so that the result of the equation is the vector projected to the screen of the camera?????
    pub fn find_projection(&self, world_position: &Vec3) -> (f32, f32) {
        let line_equation = Self::find_vector_equation(world_position, &self.pin_hole);

        let plane_equation = Self::find_plane_equation_component(&self.get_normal_vector(), &self.screen.center_position);

        let lambda = Self::find_lambda(&line_equation, &plane_equation);

        let projected_position = Vec3::add(&line_equation.0, &line_equation.1.scale(lambda));

        let camera_perspective_projected_pos = Vec3::from_two_position(&self.screen.top_left, &projected_position);

        let projected_on_top = camera_perspective_projected_pos.project_on(&self.screen.top) / &self.screen.top.magnitude();
        let projected_on_left = camera_perspective_projected_pos.project_on(&self.screen.left) / &self.screen.left.magnitude();

        (projected_on_top, projected_on_left)
    }

    fn find_lambda(line_equation: &(Vec3, Vec3), plane_equation: &(f32, f32, f32, f32)) -> f32 {
        let a = plane_equation.0 * line_equation.1.x +
                     plane_equation.1 * line_equation.1.y +
                     plane_equation.2 * line_equation.1.z;

        let b = plane_equation.0 * line_equation.0.x +
                     plane_equation.1 * line_equation.0.y +
                     plane_equation.2 * line_equation.0.z;

        (plane_equation.3 - b) / a
    }

    fn find_vector_equation(position_vector: &Vec3, point: &Vec3) -> (Vec3, Vec3) {
        let position_vector_clone = Vec3::new(position_vector.x, position_vector.y, position_vector.z);
        (position_vector_clone, Vec3::from_two_position(position_vector, point))
    }

    fn find_plane_equation_component(normal_vector: &Vec3, point_in_plane: &Vec3) -> (f32, f32, f32, f32) {
        let constant =
            - point_in_plane.x * normal_vector.x +
            - point_in_plane.y * normal_vector.y +
            - point_in_plane.z * normal_vector.z;

        (normal_vector.x, normal_vector.y, normal_vector.z, constant * -1.0)
    }
}

pub struct Screen {
    top_left: Vec3,
    top_right: Vec3,
    bottom_left: Vec3,
    bottom_right: Vec3,
    center_position: Vec3,
    top: Vec3,
    left: Vec3
}

impl Screen {

    pub fn new(top_left: Vec3,
           top_right: Vec3,
           bottom_left: Vec3,
           bottom_right: Vec3) -> Self {

        let top = Vec3::from_two_position(&top_left, &top_right);
        let left = Vec3::from_two_position(&top_left, &bottom_left);
        let half_top = top.scale(0.5);
        let half_left = left.scale(0.5);
        let from_top_left_to_center = Vec3::add(&half_top, &half_left);

        // Position vector should start from origin
        let center_position = Vec3::add(&top_left, &from_top_left_to_center);

        Self {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
            center_position,
            top,
            left
        }
    }
}