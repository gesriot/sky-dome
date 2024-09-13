use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
struct MotorSteps {
    h: i32,
    v: i32,
}

struct ScannerEnumerator {
    motor_range: MotorSteps,
    positions: Vec<MotorSteps>,
    current_index: usize,
}

impl ScannerEnumerator {
    fn new(motor_range: MotorSteps, initial_position: MotorSteps) -> Self {
        let mut se = ScannerEnumerator {
            motor_range,
            positions: Vec::new(),
            current_index: 0,
        };
        se.positions = se.optimize_circle_positions(se.generate_circle_positions(initial_position));
        se.current_index = se.find_closest_position_index(initial_position);
        se
    }

    fn current(&self) -> Option<MotorSteps> {
        if !self.positions.is_empty() && self.current_index < self.positions.len() {
            Some(self.positions[self.current_index])
        } else {
            None
        }
    }

    fn move_next(&mut self) -> bool {
        if self.positions.is_empty() {
            return false;
        }
        self.current_index = (self.current_index + 1) % self.positions.len();
        true
    }

    fn calculate_distance(&self, pos1: MotorSteps, pos2: MotorSteps) -> f64 {
        let dh = (pos1.h - pos2.h).abs().min(self.motor_range.h - (pos1.h - pos2.h).abs()) as f64;
        let dv = (pos1.v - pos2.v).abs() as f64;
        (dh.powi(2) + dv.powi(2)).sqrt()
    }

    fn optimize_circle_positions(&self, positions: Vec<MotorSteps>) -> Vec<MotorSteps> {
        let mut levels: std::collections::HashMap<i32, Vec<MotorSteps>> = std::collections::HashMap::new();
        for pos in positions {
            levels.entry(pos.v).or_insert(Vec::new()).push(pos);
        }

        let mut sorted_levels: Vec<i32> = levels.keys().cloned().collect();
        sorted_levels.sort();

        let mut snake_positions = Vec::new();
        for (i, &v) in sorted_levels.iter().enumerate() {
            let mut level_positions = levels.get(&v).unwrap().clone();
            level_positions.sort_by_key(|pos| pos.h);
            if i % 2 == 1 {
                level_positions.reverse();
            }
            snake_positions.extend(level_positions);
        }

        snake_positions
    }

    fn generate_circle_positions(&self, initial_position: MotorSteps) -> Vec<MotorSteps> {
        if initial_position.v < 0 || initial_position.v > self.motor_range.v / 2 {
            panic!("Initial position must be in the upper hemisphere");
        }

        let mut positions = Vec::new();

        let horizontal_fov_radians = deg2rad(FOV_H);
        let vertical_fov_radians = deg2rad(FOV_V);

        let max_vertical_steps = self.motor_range.v / 2;
        let vertical_step_size = (self.motor_range.v as f64 * vertical_fov_radians / (2.0 * PI)).max(1.0) as i32;

        for vertical_step in (0..=max_vertical_steps).step_by(vertical_step_size as usize) {
            let vertical_angle_radians = motor_to_rad(vertical_step, self.motor_range.v);
            let radius = vertical_angle_radians.cos();
            let circumference = 2.0 * PI * radius;
            let horizontal_steps_count = (circumference / horizontal_fov_radians).ceil().max(1.0) as i32;

            for i in 0..horizontal_steps_count {
                let horizontal_angle_radians = 2.0 * PI * i as f64 / horizontal_steps_count as f64;
                let mut horizontal_motor_units = rad_to_motor(horizontal_angle_radians, self.motor_range.h);

                horizontal_motor_units = (horizontal_motor_units + self.motor_range.h / 2) % self.motor_range.h - self.motor_range.h / 2;

                positions.push(MotorSteps { h: horizontal_motor_units, v: vertical_step });
            }
        }

        positions
    }

    fn find_closest_position_index(&self, initial_position: MotorSteps) -> usize {
        if self.positions.is_empty() {
            return 0;
        }

        let mut closest_index = 0;
        let mut min_distance = f64::MAX;

        for (i, &pos) in self.positions.iter().enumerate() {
            let distance = self.calculate_distance(initial_position, pos);
            if distance < min_distance {
                min_distance = distance;
                closest_index = i;
            }
        }

        closest_index
    }
}

fn deg2rad(deg: f64) -> f64 {
    deg * (PI / 180.0)
}

fn motor_to_rad(motor_units: i32, steps: i32) -> f64 {
    if steps <= 0 {
        panic!("Steps must be positive");
    }
    let half_steps = steps / 2;
    if motor_units < -half_steps || motor_units > half_steps {
        panic!("motorUnits must be in range [{}, {}]. Got: {}", -half_steps, half_steps, motor_units);
    }
    (PI * motor_units as f64) / half_steps as f64
}

fn rad_to_motor(radians: f64, steps: i32) -> i32 {
    if steps <= 0 {
        panic!("Steps must be positive");
    }
    let radians = (radians + PI) % (2.0 * PI) - PI;
    let half_steps = steps / 2;
    ((radians * half_steps as f64) / PI).round() as i32
}

const FOV_H: f64 = 34.16;
const FOV_V: f64 = 25.72;

fn main() {
    let motor_range = MotorSteps { h: 800, v: 800 };
    let initial_position = MotorSteps { h: 0, v: 0 };

    let mut scanner = ScannerEnumerator::new(motor_range, initial_position);

    for _ in 0..100 {
        if scanner.move_next() {
            if let Some(current) = scanner.current() {
                println!("Horizontal: {:5}, Vertical: {:5}", current.h, current.v);
            }
        }
    }
}
