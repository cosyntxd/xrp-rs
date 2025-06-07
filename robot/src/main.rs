use macroquad::prelude::*;
use xrp::{frc::physical::*, network::{send::XRPSendPacket, XRPConnection}, subsystem::{manager::{SubsystemManager, TRACKER}, Subsystem}};

fn window_conf() -> Conf {
    Conf {
        window_title: "Tank Steering".to_owned(),
        window_width: 800,
        window_height: 300,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut left_motor = Subsystem::new(Motor::new(0));
    let mut right_motor = Subsystem::new(Motor::new(1));
    let mut network = XRPConnection::new();
    let mut sequence = 0;
    let mut last_update = get_time();

    loop {
        let mut left_power = 0.0;
        let mut right_power = 0.0;

        if is_key_down(KeyCode::W) { left_power = 1.0; right_power = 1.0; }
        if is_key_down(KeyCode::S) { left_power = -1.0; right_power = -1.0; }
        if is_key_down(KeyCode::A) { left_power = -0.5; right_power = 0.5; }
        if is_key_down(KeyCode::D) { left_power = 0.5; right_power = -0.5; }

        // 20Hz
        if get_time() - last_update > 0.05 {
            left_motor.with_lock(|sub| sub.set_power(left_power));
            right_motor.with_lock(|sub| sub.set_power(-0.8 * right_power));
            
            let mut packet = XRPSendPacket::new(sequence, true);
            sequence += 1;
            
            unsafe {
                TRACKER.periodic_all();
                TRACKER.write_packet_all(&mut packet);
            }
            network.send(packet);
            
            last_update = get_time();
        }

        clear_background(BLACK);
        draw_text("WASD to drive", 20.0, 80.0, 120.0, WHITE);
        draw_text(&format!("L: {:.1}  R: {:.1}", left_power, right_power), 20.0, 170.0, 108.0, WHITE);

        next_frame().await;
    }
}