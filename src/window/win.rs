use windows::{
  core::*,
  Win32::{
    Foundation::*,
    Graphics::Gdi::*,
    System::LibraryLoader::*,
    UI::WindowsAndMessaging::*,
  }
};
use std::ptr::null_mut;

//Todo implement RGB function which creates a u32 of type colorref 0x00BBGGRR
#[allow(non_snake_case)]
fn RGB(r: u8, g: u8, b: u8) -> u32 {
  (r as u32) << 16 | (g as u32) << 8 | b as u32
}


unsafe extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
  match msg {
    WM_DESTROY => {
      unsafe {
        PostQuitMessage(0);
        return LRESULT(0);
      }
    }
    WM_PAINT => {
      unsafe {
        let mut ps = PAINTSTRUCT::default();
        let hdc = BeginPaint(hwnd, &mut ps);

        let hbrush = CreateSolidBrush(COLORREF(RGB(0, 0, 0)));
        FillRect(hdc, &mut ps.rcPaint, hbrush);
      }
      return LRESULT(0);
    } 
    _=> unsafe {DefWindowProcW(hwnd, msg, wparam, lparam)}
  }
}


fn main() -> Result<()> {
  unsafe {
    let h_instance = GetModuleHandleW(None)?.into();   //creates the current handle
    let sztitle = w!("");           //It shows the title bar text
    let sz_window_class = w!("Window class");        //Window Class name
    let cursor = LoadCursorW(None, IDC_ARROW)?;

    let screen_width = GetSystemMetrics(SM_CXSCREEN);
    let screen_height = GetSystemMetrics(SM_CYSCREEN);

    let window_width = 800;
    let window_height = 600;

    let x = (screen_width - window_width) / 2;
    let y = (screen_height - window_height) / 2;

    //creating a window class to manage the style and what not
    let wc = WNDCLASSEXW {
      cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
      style: CS_HREDRAW  | CS_VREDRAW,
      lpfnWndProc: Some(window_proc),
      hInstance: h_instance,
      lpszClassName: sz_window_class,
      hCursor: cursor,
      ..Default::default()
    };

    RegisterClassExW(&wc);

    //hwnd is passed automatically so No need to pass
    //It creates the hwnd pointer with the details and that pointer is accessed by the system and works
    let _hwnd = CreateWindowExW(
      Default::default(), 
      sz_window_class, 
      sztitle, 
      WS_OVERLAPPEDWINDOW | WS_VISIBLE, //Creates what kind of window to be opened
      x, 
      y, 
      window_width,
      window_height, 
      None, 
      None, 
      Some(h_instance), 
      None,
    ).ok();

    SetWindowLongW(hwnd.unwrap(), GWL_EXSTYLE, WS_EX_DLGMODALFRAME.0 as _);

    let mut msg = MSG::default();

    //A hacctable is used to create shortcuts for the program.
    //let hacctable = LoadAcceleratorsW(h_instance.into(), lptablename);

    while GetMessageW(&mut msg, None, 0, 0) != FALSE {
      if TranslateAcceleratorW(msg.hwnd, HACCEL(null_mut()), &mut msg) == 0 {
        _ = TranslateMessage(&msg);
        DispatchMessageW(&msg);
      }
    }
    
  }
  return Ok(());
}

//The internal main function is called here and the window is created
pub fn create_window() {
  main().expect("Failed to create window");
}