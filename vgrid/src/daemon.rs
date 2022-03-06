use std::cell::RefCell;
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
use std::str::FromStr;
use std::usize;
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::Foundation::{*};
use windows_sys::Win32::Graphics::Gdi::{MONITOR_DEFAULTTONEAREST, MonitorFromPoint};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::VK_LSHIFT;
use windows_sys::Win32::UI::Shell::{*};
use windows_sys::Win32::UI::WindowsAndMessaging::{*};

thread_local!(static LOCD: RefCell<Option<Daemon>> = RefCell::new(None));

pub struct Daemon {
    shift_down: bool,
    start_monitor: isize,
    start_window: isize,
    start_pos: POINT
}

const IDM_USER_QUIT: usize = 1;
const WM_USER_SHELLICON: u32 = WM_USER + 1;

impl Daemon {
    pub fn run_for_thread() {
        let vgrid_ico: Vec<u16> = OsString::from_str("viron-vgrid").unwrap().encode_wide().chain(Some(0)).into_iter().collect();
        let vgrid_class: Vec<u16> = OsString::from_str("VIRON_VGRID_CLASS").unwrap().encode_wide().chain(Some(0)).into_iter().collect();
        let vgrid_wnd: Vec<u16> = OsString::from_str("VIRON_VGRID_WND").unwrap().encode_wide().chain(Some(0)).into_iter().collect();
        let i18t_vgrid_monitor: Vec<u16> = OsString::from_str("VGrid Monitor").unwrap().encode_wide().chain(Some(0)).into_iter().collect();

        LOCD.with(|loc| { *loc.borrow_mut() = Some(Daemon {
            shift_down: false, start_monitor: 0, start_window: 0, start_pos: POINT { x: 0, y: 0 } }) });
        unsafe {

            let instance = GetModuleHandleW(std::ptr::null());
            assert_ne!(instance, 0);
            let icon = LoadImageW(instance, vgrid_ico.as_ptr(), IMAGE_ICON, 0, 0, LR_DEFAULTSIZE | LR_SHARED);
            assert_ne!(icon, 0);

            // Create an invisible message window.
            let mut class_ex: WNDCLASSEXW = std::mem::zeroed();
            class_ex.cbSize = std::mem::size_of::<WNDCLASSEXW>() as u32;
            class_ex.lpfnWndProc = Some(Daemon::window_proc);
            class_ex.hInstance = instance;
            class_ex.lpszClassName = vgrid_class.as_ptr();
            let mut r: i32 = RegisterClassExW(&class_ex) as i32;
            assert_ne!(r, 0);
            let hwnd = CreateWindowExW(0, vgrid_class.as_ptr(), vgrid_wnd.as_ptr(), 0, 0, 0, 0, 0, HWND_MESSAGE, 0, 0, std::ptr::null());
            assert_ne!(hwnd, 0);

            // Create sys tray icon.
            let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
            nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
            nid.hWnd = hwnd;
            nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP; // | NIF_SHOWTIP;
            nid.uCallbackMessage = WM_USER_SHELLICON;
            nid.hIcon = icon;
            nid.Anonymous.uVersion = NOTIFYICON_VERSION_4;
            nid.szTip[..i18t_vgrid_monitor.len()].copy_from_slice(&i18t_vgrid_monitor);
            r = Shell_NotifyIconW(NIM_ADD, &nid);
            assert_eq!(r, 1);

            // Create hooks
            let keyboard_hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(Daemon::low_level_keyboard_proc), instance, 0);
            assert_ne!(keyboard_hook, 0);
            let mouse_hook = SetWindowsHookExW(WH_MOUSE_LL, Some(Daemon::low_level_mouse_proc), instance, 0);
            assert_ne!(mouse_hook, 0);

            // Enter message loop
            let mut msg: MSG = std::mem::zeroed();
            r = GetMessageW(&mut msg, 0, 0, 0);
            assert_ne!(r, -1);
            while r != 0 {
                TranslateMessage(&mut msg);
                DispatchMessageW(&msg);
                r = GetMessageW(&mut msg, 0, 0, 0);
                assert_ne!(r, -1);
            }

            // Cleanup.
            r = UnhookWindowsHookEx(mouse_hook);
            assert_ne!(r, 0);
            r = UnhookWindowsHookEx(keyboard_hook);
            assert_ne!(r, 0);
            r = Shell_NotifyIconW(NIM_DELETE, &nid);
            assert_eq!(r, 1);
            r = DestroyWindow(hwnd);
            assert_ne!(r, 0);
            r = UnregisterClassW(vgrid_class.as_ptr(), instance);
            assert_ne!(r, 0);
        }
        LOCD.with(|loc| { *loc.borrow_mut() = None });
    }

    pub unsafe extern "system" fn window_proc(hwnd: HWND, u_msg: u32, w_param: usize, l_param: isize) -> LRESULT {
        match u_msg {
            WM_COMMAND => {
                if w_param & 0xffff == IDM_USER_QUIT {
                    PostQuitMessage(0);
                }
            },
            WM_USER_SHELLICON => {
                let cmd = (l_param & 0xffff) as u32;
                if cmd == WM_RBUTTONUP || cmd == WM_LBUTTONUP {
                    let mut pt: POINT = std::mem::zeroed();
                    let mut r = GetCursorPos(&mut pt);
                    assert_ne!(r, 0);
                    let h_pop_menu = CreatePopupMenu();
                    assert_ne!(h_pop_menu, 0);
                    let i18t_quit: Vec<u16> = OsString::from_str("Quit").unwrap().encode_wide().chain(Some(0)).into_iter().collect();
                    r = AppendMenuW(h_pop_menu, MF_BYPOSITION | MF_STRING, IDM_USER_QUIT, i18t_quit.as_ptr());
                    assert_ne!(r, 0);
                    r = SetForegroundWindow(hwnd);
                    assert_ne!(r, 0);
                    TrackPopupMenu(h_pop_menu, TPM_LEFTALIGN | TPM_LEFTBUTTON | TPM_BOTTOMALIGN, pt.x, pt.y, 0, hwnd, std::ptr::null());
                    assert_ne!(r, 0);
                }
            }
            _ => ()
        }
        DefWindowProcW(hwnd, u_msg, w_param, l_param)
    }

    pub unsafe extern "system" fn low_level_keyboard_proc(n_code: i32, w_param: usize, l_param: isize) -> LRESULT {
        const WM_KEYDOWN_S: usize = WM_KEYDOWN as usize;
        const WM_UP_S: usize = WM_KEYUP as usize;
        if n_code == HC_ACTION as i32 {
            match w_param {
                WM_KEYDOWN_S => {
                    let keyboard_struct: *mut KBDLLHOOKSTRUCT = l_param as *mut KBDLLHOOKSTRUCT;
                    if (*keyboard_struct).vkCode == VK_LSHIFT as u32 {
                        LOCD.with(|loc| { loc.borrow_mut().as_mut().unwrap().shift_down = true; });
                    }
                },
                WM_UP_S => {
                    let keyboard_struct: *mut KBDLLHOOKSTRUCT = l_param as *mut KBDLLHOOKSTRUCT;
                    if (*keyboard_struct).vkCode == VK_LSHIFT as u32 {
                        LOCD.with(|loc| { loc.borrow_mut().as_mut().unwrap().shift_down = false; });
                    }
                },
                _ => ()
            }
        }
        return CallNextHookEx(0, n_code, w_param, l_param);
    }

    pub unsafe extern "system" fn low_level_mouse_proc(n_code: i32, w_param: usize, l_param: isize) -> LRESULT {
        const WM_XBUTTONDOWN_S: usize = WM_XBUTTONDOWN as usize;
        const WM_XBUTTONUP_S: usize = WM_XBUTTONUP as usize;
        if n_code == HC_ACTION as i32 {
            match w_param {
                WM_XBUTTONDOWN_S => {
                    LOCD.with(|loc| {
                        let mut locdd_t = loc.borrow_mut();
                        let locdd = locdd_t.as_mut().unwrap();
                        let mouse_struct: *mut MSLLHOOKSTRUCT = l_param as *mut MSLLHOOKSTRUCT;
                        if ((*mouse_struct).mouseData >> 16) & 0xffff == XBUTTON2 && locdd.start_window == 0 && locdd.shift_down {
                            locdd.start_window = RealChildWindowFromPoint(GetDesktopWindow(), (*mouse_struct).pt);
                            if locdd.start_window != 0 {
                                // Ensure that we did not get the desktop window.
                                if locdd.start_window == GetDesktopWindow() || IsChild(GetShellWindow(), locdd.start_window) != 0{
                                    locdd.start_window = 0;
                                }
                            }
                            if locdd.start_window != 0 {
                                // Ensure we get a valid monitor.
                                locdd.start_pos = (*mouse_struct).pt;
                                locdd.start_monitor = MonitorFromPoint((*mouse_struct).pt, MONITOR_DEFAULTTONEAREST);
                                if locdd.start_monitor == 0 {
                                    locdd.start_window = 0;
                                }
                            }
                        }
                    });
                },
                WM_XBUTTONUP_S => {
                    LOCD.with(|loc| {
                        let mut locdd_t = loc.borrow_mut();
                        let locdd = locdd_t.as_mut().unwrap();
                        if locdd.start_window != 0 {
                            if !locdd.shift_down {
                                locdd.start_window = 0;
                            } else {
                                let mouse_struct: *mut MSLLHOOKSTRUCT = l_param as *mut MSLLHOOKSTRUCT;
                                if ((*mouse_struct).mouseData >> 16) & 0xffff == XBUTTON2 {
                                    if MonitorFromPoint((*mouse_struct).pt, MONITOR_DEFAULTTONEAREST) == locdd.start_monitor {

                                    }
                                    locdd.start_window = 0;
                                    println!("woot");
                                }
                            }
                        }
                    });
                },
                _ => ()
            }
        }
        return CallNextHookEx(0, n_code, w_param, l_param);
    }
}