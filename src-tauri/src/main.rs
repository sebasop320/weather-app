use tauri::{ Menu, Submenu, MenuItem, CustomMenuItem };

fn main() {
  let submenu_gear = Submenu::new(
    "Gear",
    Menu::new()
      .add_native_item(MenuItem::Copy)
      .add_native_item(MenuItem::Paste)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Zoom)
      .add_native_item(MenuItem::Separator)
      .add_native_item(MenuItem::Hide)
      .add_native_item(MenuItem::CloseWindow)
      .add_native_item(MenuItem::SelectAll)
      .add_native_item(MenuItem::Quit),
  );
  let close = CustomMenuItem::new("close".to_string(), "Close"); 
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let submenu_customer = Submenu::new(
    "More", 
    Menu::new()
      .add_item(close)
      .add_item(quit)
    );
  let menus = Menu::new()
    .add_submenu(submenu_gear)
    .add_submenu(submenu_customer);

  tauri::Builder::default()
    // add menu
    .menu(menus)
    // Listen for custom menu events
    .on_menu_event(|event| match event.menu_item_id() {
      "quit" => {
        std::process::exit(0);
      }
      "close" => {
        event.window().close().unwrap();
      }
      _ => {}
    })
    // Registration command
    
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}