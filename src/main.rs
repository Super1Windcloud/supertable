mod assets;
mod data;
mod db;
mod i18n;
mod palette;
mod ui;

use gpui::{App, AppContext, Application, Bounds, WindowBounds, WindowOptions, px, size};
#[cfg(target_os = "macos")]
use gpui::{KeyBinding, Menu, MenuItem, SystemMenuType, actions};
use gpui_component::{
    Root,
    theme::{Theme, ThemeMode},
};

#[cfg(target_os = "macos")]
use cocoa::{
    appkit::NSApp,
    base::{id, nil},
    foundation::NSString,
};
#[cfg(target_os = "macos")]
use objc::{class, msg_send, sel, sel_impl};

#[cfg(target_os = "macos")]
use assets::app_icon_path;
use ui::app::SuperTableApp;

#[cfg(target_os = "macos")]
actions!(supertable_app, [Quit]);

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
    let bounds = Bounds::centered(None, size(px(1280.), px(800.)), cx);
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
        #[cfg(target_os = "macos")]
        configure_macos_app(cx);

        open_main_window(cx);
        cx.activate(true);
    });
}

#[cfg(target_os = "macos")]
fn configure_macos_app(cx: &mut App) {
    cx.on_action(quit);
    cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
    cx.set_menus(vec![Menu {
        name: "SuperTable".into(),
        items: vec![
            MenuItem::os_submenu("Services", SystemMenuType::Services),
            MenuItem::separator(),
            MenuItem::action("Quit SuperTable", Quit),
        ],
    }]);
    set_macos_dock_icon();
}

#[cfg(target_os = "macos")]
fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}

#[cfg(target_os = "macos")]
fn set_macos_dock_icon() {
    let Some(icon_path) = app_icon_path().to_str() else {
        return;
    };

    unsafe {
        let ns_image: id = msg_send![class!(NSImage), alloc];
        let ns_image: id =
            msg_send![ns_image, initWithContentsOfFile: NSString::alloc(nil).init_str(icon_path)];

        if ns_image != nil {
            let app = NSApp();
            let _: () = msg_send![app, setApplicationIconImage: ns_image];
        }
    }
}

#[cfg(target_os = "windows")]
fn refresh_windows_taskbar_icon(window: &gpui::Window) {
    let Ok(handle) = HasWindowHandle::window_handle(window) else {
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
