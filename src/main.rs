mod assets;
mod data;
mod palette;
mod ui;

use gpui::{App, AppContext, Application, Bounds, WindowBounds, WindowOptions, px, size};
use gpui_component::{Root, theme::{Theme, ThemeMode}};

use ui::app::SuperTableApp;

#[cfg(target_os = "windows")]
use raw_window_handle::{HasWindowHandle, RawWindowHandle};

#[cfg(target_os = "windows")]
use windows_sys::Win32::{
    Foundation::HWND,
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{
        ICON_BIG, ICON_SMALL, IMAGE_ICON, LR_DEFAULTSIZE, LR_SHARED, LoadImageW, SendMessageW,
        WM_SETICON,
    },
};

fn open_main_window(cx: &mut App) {
    let bounds = Bounds::centered(None, size(px(1440.), px(920.)), cx);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            app_id: Some("com.supertable.desktop".to_string()),
            ..Default::default()
        },
        |window, cx| {
            #[cfg(target_os = "windows")]
            refresh_windows_taskbar_icon(window);

            let view: gpui::Entity<SuperTableApp> =
                cx.new(|cx: &mut gpui::Context<SuperTableApp>| SuperTableApp::new(window, cx));
            cx.new(|cx: &mut gpui::Context<Root>| Root::new(view, window, cx))
        },
    )
    .unwrap();
}

fn main() {
    Application::new().run(|cx: &mut App| {
        gpui_component::init(cx);
        Theme::change(ThemeMode::Dark, None, cx);

        open_main_window(cx);
        cx.activate(true);
    });
}

#[cfg(target_os = "windows")]
fn refresh_windows_taskbar_icon(window: &gpui::Window) {
    let Ok(handle) = window.window_handle() else {
        return;
    };

    let RawWindowHandle::Win32(handle) = handle.as_raw() else {
        return;
    };

    let hwnd = handle.hwnd.get() as HWND;
    let module = unsafe { GetModuleHandleW(std::ptr::null()) };

    if module.is_null() {
        return;
    }

    let big_icon = unsafe {
        LoadImageW(
            module,
            1 as *const u16,
            IMAGE_ICON,
            0,
            0,
            LR_DEFAULTSIZE | LR_SHARED,
        )
    };

    let small_icon = unsafe {
        LoadImageW(
            module,
            1 as *const u16,
            IMAGE_ICON,
            16,
            16,
            LR_SHARED,
        )
    };

    if !big_icon.is_null() {
        unsafe {
            SendMessageW(hwnd, WM_SETICON, ICON_BIG as usize, big_icon as isize);
        }
    }

    if !small_icon.is_null() {
        unsafe {
            SendMessageW(hwnd, WM_SETICON, ICON_SMALL as usize, small_icon as isize);
        }
    }
}
