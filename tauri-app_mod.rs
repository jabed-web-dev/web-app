// File location: .cargo/registry/src/index.crates.io-xyz.../

// add all file -> use std::env;
// file: tauri-2.2.5/src/manager/mod.rs
// line: 359 
pub(crate) fn get_url(&self, https: bool) -> Cow<'_, Url> {
  let args: Vec<String> = env::args().collect();
  // Check for --url or -u
  if let Some(index) = args.iter().position(|arg| arg == "--url" || arg == "-u") {
    if let Some(url_arg) = args.get(index + 1) {
      if let Ok(url) = Url::parse(url_arg) {
        return Cow::Owned(url); // Return the parsed URL
      } else {
        eprintln!("Invalid URL provided via --url or -u: {}", url_arg);
      }
    } else {
      eprintln!("--url or -u argument provided without a value.");
    }
  }

  // Fallback if no URL is provided via command-line arguments
  match self.base_path() {
    Some(url) => Cow::Borrowed(url),
    _ => self.protocol_url(https),
  }
}

// file: tauri-runtime-2.3.0/src/webview.rs
// line: 432
#[must_use]
pub fn devtools(mut self, enabled: Option<bool>) -> Self {
  let args: Vec<String> = env::args().collect();
  let mut devtools_value = enabled; // Default to the provided value
  // Check for --devtools or -d
  if args.iter().any(|arg| arg == "--devtools" || arg == "-dt") {
    devtools_value = Some(true); // Enable devtools if the flag is present
  }

  self.devtools = devtools_value;
  self
}

// file: tauri-runtime-wry-2.3.0/src/lib.rs
// line: 770
builder = builder.title("Web App");
#[cfg(windows)]
{
  builder = builder.window_classname("WebApp Window");
}

// line: 877
fn position(mut self, x: f64, y: f64) -> Self {
  let args: Vec<String> = env::args().collect();
  let mut x_value = x; // Default to the provided x value
  let mut y_value = y; // Default to the provided y value
  let mut width = 0.0;
  let mut height = 0.0;
  if let Some(Size::Logical(logical_size)) = self.inner.window.inner_size {
    width = logical_size.width;
    height = logical_size.height;
  }

  // Check for --left or -x
  if let Some(index) = args.iter().position(|arg| arg == "--left" || arg == "-x") {
    if let Some(left_arg) = args.get(index + 1) {
      if let Ok(parsed_x) = left_arg.parse::<f64>() {
        x_value = parsed_x;
      } else {
        eprintln!("Invalid value for --left or -x: {}", left_arg);
      }
    } else {
      eprintln!("--left or -x argument provided without a value.");
    }
  }
  // Check for --top or -y
  if let Some(index) = args.iter().position(|arg| arg == "--top" || arg == "-y") {
    if let Some(top_arg) = args.get(index + 1) {
      if let Ok(parsed_y) = top_arg.parse::<f64>() {
        y_value = parsed_y;
      } else {
        eprintln!("Invalid value for --top or -y: {}", top_arg);
      }
    } else {
      eprintln!("--top or -y argument provided without a value.");
    }
  }

  if x_value < 0.0 || y_value < 0.0 {
    let event_loop = EventLoop::new();
    let monitor = event_loop.primary_monitor().unwrap();
    let logical_size = monitor.size().to_logical::<f64>(monitor.scale_factor());
    let screen_width = logical_size.width;
    let screen_height = logical_size.height;

    // Use logical size for positioning
    x_value = if x_value < 0.0 { (screen_width - width - x_value.abs()) } else { x_value };
    y_value = if y_value < 0.0 { (screen_height - height - 80.0 - y_value.abs()) } else { y_value };
  }

  self.inner = self.inner.with_position(TaoLogicalPosition::new(x_value, y_value));
  self
}

fn inner_size(mut self, width: f64, height: f64) -> Self {
  let args: Vec<String> = env::args().collect();
  let mut width_value = width;  // Default to the provided width
  let mut height_value = height; // Default to the provided height

  // Check for --width or -w
  if let Some(index) = args.iter().position(|arg| arg == "--width" || arg == "-w") {
    if let Some(width_arg) = args.get(index + 1) {
      if let Ok(parsed_width) = width_arg.parse::<f64>() {
        width_value = parsed_width;
      } else {
        eprintln!("Invalid value for --width or -w: {}", width_arg);
      }
    } else {
      eprintln!("--width or -w argument provided without a value.");
    }
  }
  // Check for --height or -h
  if let Some(index) = args.iter().position(|arg| arg == "--height" || arg == "-h") {
    if let Some(height_arg) = args.get(index + 1) {
      if let Ok(parsed_height) = height_arg.parse::<f64>() {
        height_value = parsed_height;
      } else {
        eprintln!("Invalid value for --height or -h: {}", height_arg);
      }
    } else {
      eprintln!("--height or -h argument provided without a value.");
    }
  }
  
  self.inner = self
    .inner
    .with_inner_size(TaoLogicalSize::new(width_value, height_value));
  self
}

fn title<S: Into<String>>(mut self, title: S) -> Self {
  let args: Vec<String> = env::args().collect();
  let mut title_value = title.into(); // Default to the provided title
  // Check for --title or -t
  if let Some(index) = args.iter().position(|arg| arg == "--title" || arg == "-t") {
    if let Some(title_arg) = args.get(index + 1) {
      title_value = title_arg.clone(); // Use the provided argument as the title
    } else {
      eprintln!("--title or -t argument provided without a value.");
    }
  }

  self.inner = self.inner.with_title(title_value);
  self
}

fn fullscreen(mut self, fullscreen: bool) -> Self {
  let args: Vec<String> = env::args().collect();
  let mut fullscreen_value = fullscreen; // Default to the provided value
  // Check for --fullscreen or -fs
  if args.iter().any(|arg| arg == "--fullscreen" || arg == "-fs") {
    fullscreen_value = true; // Enable fullscreen if the flag is present
  }

  self.inner = if fullscreen_value {
    self
      .inner
      .with_fullscreen(Some(Fullscreen::Borderless(None)))
  } else {
    self.inner.with_fullscreen(None)
  };
  self
}

fn always_on_top(mut self, always_on_top: bool) -> Self {
  let args: Vec<String> = env::args().collect();
  let mut always_on_top_value = always_on_top; // Default to the provided value
   // Check for --ontop or -ot
  if args.iter().any(|arg| arg == "--ontop" || arg == "-ot") {
    always_on_top_value = true; // Enable "always on top" if the flag is present
  }

  self.inner = self.inner.with_always_on_top(always_on_top_value);
  self
}

#[allow(unused_variables, unused_mut)]
fn theme(mut self, theme: Option<Theme>) -> Self {
  let args: Vec<String> = env::args().collect();
  let mut theme_value = theme; // Default to the provided theme
  // Check for --light flag
  if args.iter().any(|arg| arg == "--light" || arg == "-l") {
    theme_value = Some(Theme::Light); // Set theme to light
  }
  // Check for --dark flag
  if args.iter().any(|arg| arg == "--dark" || arg == "-d") {
    theme_value = Some(Theme::Dark); // Set theme to dark
  }

  self.inner = self.inner.with_theme(if let Some(t) = theme_value {
    match t {
      Theme::Dark => Some(TaoTheme::Dark),
      _ => Some(TaoTheme::Light),
    }
  } else {
    None
  });
  self
}

/* 
--url          -u
--title        -t
--width        -w
--height       -h
--left         -x
--top          -y
--light        -l
--dark         -d
--ontop        -ot
--devtools     -dt
--fullscreen   -fs
*/

// file: my-app/src-tauri/Cargo.toml
[dependencies]
tauri = { version = "2", features = ["devtools"] } // Enable devtools feature

// file: my-app/src-tauri/tauri.conf.json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "web-app",
  "version": "1.0.0",
  "identifier": "web-app",
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:5000"
  },
  "app": {
    "withGlobalTauri": false,
    "windows": [
      {
        "title": "Web App",
        "devtools": false,
        "center": true,
        "focus": true,
        "width": 800,
        "height": 600
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.ico",
      "icons/icon.icns"
    ]
  }
}
