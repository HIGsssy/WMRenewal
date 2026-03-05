use std::time::{Duration, Instant};

use anyhow::Result;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use tracing::info;

use wm_core::config::GameConfig;
use wm_game::state::GameState;
use wm_ui::events::UiEvent;
use wm_ui::font::FontCache;
use wm_ui::graphics::Graphics;
use wm_ui::screen::{ScreenAction, ScreenManager};
use wm_ui::texture_cache::TextureCache;
use wm_ui::widget::RenderContext;

/// Target frame time for ~25 FPS (matching original game).
const FRAME_DURATION: Duration = Duration::from_millis(40);

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    info!("WhoreMaster Renewal starting...");

    // Load configuration
    let resources = wm_core::resources_path();
    info!("Resources path: {:?}", resources);

    let config_path = resources.join("Data/config.xml");
    let config = if config_path.exists() {
        info!("Loading config from {:?}", config_path);
        wm_core::xml::load_config(&config_path)?
    } else {
        info!("Config file not found, using defaults");
        GameConfig::default()
    };

    // Create game state
    let mut game_state = GameState::new(config);
    info!("Game state initialized");

    // Initialize graphics (800x600, matching original)
    let mut graphics =
        Graphics::new("WhoreMaster Renewal", 800, 600).map_err(|e| anyhow::anyhow!("{}", e))?;
    info!("Graphics initialized (800x600)");

    // Initialize font cache — use bundled open-source DejaVu Sans font,
    // falling back to the legacy font path if the bundled font is missing.
    let bundled_font = std::path::PathBuf::from("assets/fonts/DejaVuSans.ttf");
    let legacy_font = resources.join("../Dependencies/fonts/bin/segoeui.ttf");
    let font_path = if bundled_font.exists() {
        bundled_font
    } else {
        legacy_font
    };
    let mut fonts =
        FontCache::new(&font_path).map_err(|e| anyhow::anyhow!("Font init failed: {}", e))?;
    info!("Font cache initialized");

    // Initialize texture cache
    let mut textures = TextureCache::new();

    // Initialize screen manager and push main menu
    let mut screen_mgr = ScreenManager::new();
    screen_mgr.push(
        Box::new(wm_ui::screen::main_menu::MainMenuScreen::new()),
        &mut game_state,
    );

    // Main game loop
    let mut event_pump = graphics
        .sdl_context
        .event_pump()
        .map_err(|e| anyhow::anyhow!("{}", e))?;

    let mut running = true;
    let mut mouse_x = 0i32;
    let mut mouse_y = 0i32;
    info!("Entering main loop");

    while running {
        let frame_start = Instant::now();

        // Poll SDL events
        for event in event_pump.poll_iter() {
            let ui_event = match event {
                Event::Quit { .. } => Some(UiEvent::Quit),
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => Some(UiEvent::Escape),
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => Some(UiEvent::KeyDown { keycode }),
                Event::MouseButtonDown { x, y, .. } => Some(UiEvent::MouseDown { x, y }),
                Event::MouseButtonUp { x, y, .. } => Some(UiEvent::MouseClick { x, y }),
                Event::MouseMotion { x, y, .. } => {
                    mouse_x = x;
                    mouse_y = y;
                    Some(UiEvent::MouseMove { x, y })
                }
                Event::MouseWheel { y, .. } => {
                    if y > 0 {
                        Some(UiEvent::MouseWheelUp)
                    } else if y < 0 {
                        Some(UiEvent::MouseWheelDown)
                    } else {
                        None
                    }
                }
                _ => None,
            };

            if let Some(ui_event) = ui_event {
                match &ui_event {
                    UiEvent::Quit | UiEvent::Escape => {
                        running = false;
                        break;
                    }
                    _ => {}
                }

                if !screen_mgr.is_empty() {
                    let action = screen_mgr.on_event(ui_event, &mut game_state);
                    if matches!(action, ScreenAction::Quit) {
                        running = false;
                        break;
                    }
                    screen_mgr.handle_action(action, &mut game_state);
                }
            }
        }

        if !running {
            break;
        }

        // Process current screen
        if !screen_mgr.is_empty() {
            let action = screen_mgr.process(&mut game_state);
            if matches!(action, ScreenAction::Quit) {
                running = false;
                continue;
            }
            screen_mgr.handle_action(action, &mut game_state);
        }

        // Render
        graphics.begin_frame();

        if !screen_mgr.is_empty() {
            let mut ctx = RenderContext {
                canvas: &mut graphics.canvas,
                textures: &mut textures,
                fonts: &mut fonts,
                texture_creator: &graphics.texture_creator,
                resources_path: &resources,
                mouse_x,
                mouse_y,
            };
            screen_mgr.widgets.draw_all(&mut ctx);
        }

        graphics.end_frame();

        // Frame rate cap (~25 FPS)
        let elapsed = frame_start.elapsed();
        if elapsed < FRAME_DURATION {
            std::thread::sleep(FRAME_DURATION - elapsed);
        }
    }

    // Clean up textures before graphics context drops
    textures.clear();

    info!("WhoreMaster Renewal shutting down");
    Ok(())
}
