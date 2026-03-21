use wry::WebViewBuilder;
use tao::{
    event_loop::{EventLoop, ControlFlow},
    window::{WindowBuilder, Icon},
};
use image::ImageReader;
use wry::Result;

use crate::structs::App;

pub fn launch(app: App) -> Result<()> 
{

    let event_loop = EventLoop::new();

    let icon = {
        let image_reader = ImageReader::open(app.icon_path)?
            .decode()
            .expect("Failed to decode image");

        let decoded_image = image_reader.into_rgba8();
        let (width, height) = decoded_image.dimensions();
        
        Icon::from_rgba(decoded_image.into_raw(), width, height)
            .expect("Failed to create icon from image data")
    };

    let window = WindowBuilder::new()
        .with_title(app.title)
        .with_window_icon(Some(icon))
        .build(&event_loop)
        .expect("Error : Failed to load webview.");

    let builder = WebViewBuilder::new();

    let _webview = builder
        .with_url(app.url)
        .build(&window);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            tao::event::Event::WindowEvent {
                event: tao::event::WindowEvent::CloseRequested,
                ..
            } => {
                println!("The application is closing.");
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });

}