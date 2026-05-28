use global_hotkey::{
    GlobalHotKeyEvent, GlobalHotKeyManager,
    hotkey::{Code, HotKey, Modifiers},
};

use raylib::ffi::{ClearWindowState, ConfigFlags, SetWindowState};
use raylib::prelude::*;

use xcap::Monitor;

const SCREEN_WIDTH: i32 = 1920;
const SCREEN_HEIGHT: i32 = 1080;

const FPS: u32 = 240;

const ZOOM_SPEED: f32 = 0.1;

fn main() {
    let mut builder = raylib::init();

    builder.size(SCREEN_WIDTH, SCREEN_HEIGHT);
    builder.title("RSZoomer");

    let (mut rl, rl_thread) = builder.build();

    unsafe {
        SetWindowState(
            (ConfigFlags::FLAG_WINDOW_HIDDEN as u32)
                | (ConfigFlags::FLAG_WINDOW_TOPMOST as u32)
                | (ConfigFlags::FLAG_WINDOW_UNDECORATED as u32),
        );
    }

    let hotkey_manager = GlobalHotKeyManager::new().unwrap();
    let activate_hotkey = HotKey::new(Some(Modifiers::CONTROL), Code::F1);
    hotkey_manager.register(activate_hotkey).unwrap();
    let hotkey_receiver = GlobalHotKeyEvent::receiver();

    rl.set_target_fps(FPS);

    rl.set_exit_key(None);

    let mut screenshot_texture: Option<Texture2D> = None;

    let mut camera = Camera2D {
        target: Vector2 {
            x: (SCREEN_WIDTH as f32 / 2.0),
            y: (SCREEN_HEIGHT as f32 / 2.0),
        },
        offset: Vector2 {
            x: (SCREEN_WIDTH as f32 / 2.0),
            y: (SCREEN_HEIGHT as f32 / 2.0),
        },
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut target_zoom = 1.0;

    while !rl.window_should_close() {
        if let Ok(_event) = hotkey_receiver.try_recv() {
            let monitors = Monitor::all().unwrap();

            if let Some(primary_monitor) = monitors.first() {
                let img = primary_monitor.capture_image().unwrap();

                let img_width = img.width() as i32;
                let img_height = img.height() as i32;

                let raw_pixels = img.as_raw();

                let ffi_image = raylib::ffi::Image {
                    data: raw_pixels.as_ptr() as *mut std::ffi::c_void,
                    width: img_width,
                    height: img_height,
                    mipmaps: 1,
                    format: raylib::ffi::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32,
                };

                let raylib_image = unsafe { Image::from_raw(ffi_image) };

                let texture = rl
                    .load_texture_from_image(&rl_thread, &raylib_image)
                    .unwrap();

                std::mem::forget(raylib_image);
                let _ = img;

                screenshot_texture = Some(texture);

                unsafe {
                    ClearWindowState(ConfigFlags::FLAG_WINDOW_HIDDEN as u32);

                    rl.toggle_fullscreen();
                }
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            let _ = screenshot_texture.take();

            target_zoom = 1.0;
            camera.zoom = 1.0;

            camera.target = Vector2 {
                x: (SCREEN_WIDTH as f32 / 2.0),
                y: (SCREEN_HEIGHT as f32 / 2.0),
            };
            camera.offset = Vector2 {
                x: (SCREEN_WIDTH as f32 / 2.0),
                y: (SCREEN_HEIGHT as f32 / 2.0),
            };

            if rl.is_window_fullscreen() {
                rl.toggle_fullscreen();
            }

            unsafe {
                SetWindowState(ConfigFlags::FLAG_WINDOW_HIDDEN as u32);
            }
        }

        let wheel = rl.get_mouse_wheel_move();

        let mouse_screen_pos = rl.get_mouse_position();

        let mouse_world_pos = rl.get_screen_to_world2D(mouse_screen_pos, camera);

        if !rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
            if wheel != 0.0 {
                camera.offset = mouse_screen_pos;
                camera.target = mouse_world_pos;

                target_zoom += wheel * 0.25;

                if target_zoom < 1.0 {
                    target_zoom = 1.0;
                }

                if target_zoom > 15.0 {
                    target_zoom = 15.0;
                }
            }
        }

        camera.zoom += (target_zoom - camera.zoom) * ZOOM_SPEED;

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_delta = rl.get_mouse_delta();

            camera.target.x -= mouse_delta.x / camera.zoom;
            camera.target.y -= mouse_delta.y / camera.zoom;
        }

        let mut d = rl.begin_drawing(&rl_thread);

        d.clear_background(raylib::prelude::Color::BLACK);

        {
            let mut world = d.begin_mode2D(camera);

            if let Some(ref tex) = screenshot_texture {
                world.draw_texture_ex(tex, Vector2 { x: 0.0, y: 0.0 }, 0.0, 1.0, Color::WHITE);
            }
        }
    }
}
