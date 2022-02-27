use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows_sys::Win32::UI::Shell::NOTIFYICONDATAW;
use windows_sys::Win32::UI::WindowsAndMessaging::{AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, GetCursorPos, IMAGE_ICON, LoadImageW, LR_DEFAULTSIZE, LR_SHARED, MF_BYPOSITION, MF_STRING, PostQuitMessage, RegisterClassExW, SetForegroundWindow, TPM_BOTTOMALIGN, TPM_LEFTALIGN, TPM_LEFTBUTTON, TrackPopupMenu, WM_COMMAND, WM_LBUTTONUP, WM_RBUTTONUP, WM_USER, WNDCLASSEXW};

pub struct Daemon {
}

const VGRID_STR16: [u16; 12] = [118, 105, 114, 111, 110, 045, 118, 103, 114, 105, 100, 0]; // viron-vgrid
const VGRID_CLASS_STR16: [u16; 18] = [086, 073, 082, 079, 078, 095, 086, 071, 082, 073, 068, 095, 067, 076, 065, 083, 083, 0]; // VIRON_VGRID_CLASS
const VGRID_WND_STR16: [u16; 16] = [086, 073, 082, 079, 078, 095, 086, 071, 082, 073, 068, 095, 087, 078, 068, 0]; // VIRON_VGRID_WND
const I18T_QUIT_STR: [u16; 5] = [113, 117, 105, 116, 0]; // Quit
const IDM_USER_QUIT: usize = 1;
const WM_USER_SHELLICON: u32 = WM_USER + 1;

impl Daemon {
    pub fn new() {
        unsafe {
            let instance = GetModuleHandleW(std::ptr::null());
            assert_ne!(instance, 0);
            let icon = LoadImageW(instance, VGRID_STR16.as_ptr(), IMAGE_ICON, 0, 0, LR_DEFAULTSIZE | LR_SHARED);
            assert_ne!(icon, 0);
            let mut classEx: WNDCLASSEXW = std::mem::zeroed();
            classEx.cbSize = std::mem::size_of::<WNDCLASSEXW>() as u32;
            classEx.lpfnWndProc = Some(Daemon::window_proc);
            classEx.hInstance = instance;
            classEx.lpszClassName = VGRID_CLASS_STR16.as_ptr();
            let r = RegisterClassExW(&classEx);
            assert_ne!(r, 0);

            let nid: NOTIFYICONDATAW;
        }
    }

    pub unsafe extern "system" fn window_proc(hwnd: HWND, u_msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
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
                    let hPopMenu = CreatePopupMenu();
                    assert_ne!(hPopMenu, 0);
                    r = AppendMenuW(hPopMenu, MF_BYPOSITION | MF_STRING, IDM_USER_QUIT, I18T_QUIT_STR.as_ptr());
                    assert_ne!(r, 0);
                    r = SetForegroundWindow(hwnd);
                    assert_ne!(r, 0);
                    TrackPopupMenu(hPopMenu, TPM_LEFTALIGN | TPM_LEFTBUTTON | TPM_BOTTOMALIGN, pt.x, pt.y, 0, hwnd, std::ptr::null());
                    assert_ne!(r, 0);
                }
            }
            _ => ()
        }
        DefWindowProcW(hwnd, u_msg, w_param, l_param)
    }
}