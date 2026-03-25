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

            ui.add_space(10.0);
            ui.separator();

            ui.label(
                egui::RichText::new("Color Picker").strong()
            );
            ui.color_edit_button_srgba(&mut self.current_color);  
            
            if ui.button("Clear Canvas").clicked(){
                self.pixels = [[egui::Color32::LIGHT_GRAY; 8]; 8];
            }

            if ui.button("Eraser").clicked(){
                self.current_color = egui::Color32::LIGHT_GRAY;
            }
        });


        egui::CentralPanel::default().show(ctx, |ui| {
              ui.heading("Your Canvas");


            for y in 0..8{
                ui.horizontal(|ui|{
                    for x in 0..8{
                        let pixel_color = self.pixels[y][x];

                        let button = egui::Button::new("")
                        .fill(pixel_color)
                        .min_size(egui::Vec2 { x: 32.0, y: 32.0 });

                        let button_response = ui.add(button);

                        let is_pointer_over_button = ui.rect_contains_pointer(button_response.rect);

                        if button_response.clicked()
                         || (is_pointer_over_button && ui.input(|i| i.pointer.primary_down()))
                        {
                                self.pixels[y][x] = self.current_color;
                        }
                    }

                });
            }

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