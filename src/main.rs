use eframe::egui;

struct PixelArtApp{
    pixels: [[egui::Color32; 8]; 8],
    current_color: egui::Color32
}

impl Default for PixelArtApp {
    fn default() -> Self {
        Self { pixels: [[egui::Color32::LIGHT_GRAY; 8]; 8], current_color: egui::Color32::RED }
    }
}

impl eframe::App for PixelArtApp{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("toolbox_panel").show(ctx, |ui| {
            ui.heading("ToolBox");

            // TODO: Add the color picker for `self.current_color` here.
            // TODO: Add the "Clear Canvas" button here.
        });


        egui::CentralPanel::default().show(ctx, |ui| {
              ui.heading("Your Canvas");


            // TODO: Use two `for` loops to draw the 8x8 grid of buttons here.
        });
    }
}



fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Pixel Art Maker",
        native_options,
        Box::new(|_cc| Ok(Box::new(PixelArtApp::default()))),
    )


}