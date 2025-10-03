use eframe::{egui::{self, ColorImage, TextureHandle}};
use morrish_chess::Game; // this chess engine is made by morrish
use morrish_chess::piece::Colour::*; 
use morrish_chess::piece::GameState::*; 

struct ChessGame {
    current_game: Game,                
    screen: Screen,                   // If the game is currently on language selection or game screen
    language: Option<String>,        
    special_mod: bool,               // some special meme mode (avoid it at all cost if mental health is a priority for you)
    chosen_piece: Option<[usize; 2]> // the position of the piece that the player has chosen by clicking
}

enum Screen  {
    LanguageSelect,
    SpecialModeAsk,
    GameScreen,
    GameIsOver
}

impl Default for Screen {
    fn default() -> Self {
        Screen::LanguageSelect //By default the program comes with language selection as it might be expected
    }
}
impl Default for ChessGame {
    fn default() -> Self {

        Self {
            current_game: Game::new(),
            screen: Screen::default(),
            special_mod: false, 
            language: None,
            chosen_piece: None
        }
    }
}

impl ChessGame {

    fn name() -> &'static str {
        "Legendary Chess"
    }
    
}


struct ChessBoardVisuals {
    white_pawn: TextureHandle, // this is needed in order to refer the necessary piece picture that is going to be uploaded
    black_pawn: TextureHandle,
    white_knight: TextureHandle,
    black_knight: TextureHandle,
    white_bishop: TextureHandle,
    black_bishop: TextureHandle,
    white_queen: TextureHandle,
    black_queen: TextureHandle,
    black_king: TextureHandle,
    white_king: TextureHandle,
    white_rook: TextureHandle,
    black_rook: TextureHandle
}

impl ChessBoardVisuals {

    fn new(ctx: &egui::Context, current_chess_game: &ChessGame) -> Self {

        if current_chess_game.special_mod {
            Self {
                white_pawn: load_texture(ctx, "images/special-mode/white_pawn.png"),
                black_pawn: load_texture(ctx, "images/special-mode/black_pawn.png"),
                white_knight: load_texture(ctx, "images/special-mode/white_knight.png"),
                black_knight: load_texture(ctx, "images/special-mode/black_knight.png"),
                white_bishop: load_texture(ctx, "images/special-mode/white_bishop.png"),
                black_bishop: load_texture(ctx, "images/special-mode/black_bishop.png"),
                white_king: load_texture(ctx, "images/special-mode/white_king.png"),
                black_king: load_texture(ctx, "images/special-mode/black_king.png"),
                white_queen: load_texture(ctx, "images/special-mode/white_queen.png"),
                black_queen: load_texture(ctx, "images/special-mode/black_queen.png"),
                white_rook: load_texture(ctx, "images/special-mode/white_rook.png"),
                black_rook: load_texture(ctx, "images/special-mode/black_rook.png"),
            }
        }

        else {
            Self {
                white_pawn: load_texture(ctx, "images/regular-mode/white_pawn.png"),
                black_pawn: load_texture(ctx, "images/regular-mode/black_pawn.png"),
                white_knight: load_texture(ctx, "images/regular-mode/white_knight.png"),
                black_knight: load_texture(ctx, "images/regular-mode/black_knight.png"),
                white_bishop: load_texture(ctx, "images/regular-mode/white_bishop.png"),
                black_bishop: load_texture(ctx, "images/regular-mode/black_bishop.png"),
                white_king: load_texture(ctx, "images/regular-mode/white_king.png"),
                black_king: load_texture(ctx, "images/regular-mode/black_king.png"),
                white_queen: load_texture(ctx, "images/regular-mode/white_queen.png"),
                black_queen: load_texture(ctx, "images/regular-mode/black_queen.png"),
                white_rook: load_texture(ctx, "images/regular-mode/white_rook.png"),
                black_rook: load_texture(ctx, "images/regular-mode/black_rook.png"),
            }
        }

    }
}

impl eframe::App for ChessGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        ctx.set_pixels_per_point(1.5);
        let visuals = ChessBoardVisuals::new(ctx, &self);

        egui::CentralPanel::default().show(ctx, |ui| {

            match self.screen {

                Screen::LanguageSelect => {

                    ui.heading("Welcome to this legendary chess game! || Agalar satranca hoş geldiniz!");
                    ui.label("Choose your language || Dil seçiniz");


                    ui.add_space(20.0);

                    if ui.button("English").clicked() {
                        self.language = Some("English".to_string());
                        self.screen = Screen::GameScreen;

                    };

                    if ui.button("Türkçe").clicked() {
                        self.language = Some("Türkçe".to_string());
                        self.screen = Screen::SpecialModeAsk;

                    }

                }

                Screen::SpecialModeAsk => {

                    ui.label("Aga mod?");


                    if ui.button("Olur").clicked() {
                        self.special_mod = true;
                        self.screen = Screen::GameScreen;

                    }

                    if ui.button("Yoğk").clicked() {
                        self.special_mod = false;
                        self.screen = Screen::GameScreen;

                    }
                }

                Screen::GameScreen => {

                    if self.language == Some("English".to_string()) {

                        if ui.button("Quit").clicked() {

                            std::process::exit(0);
                        }

                        ui.add_space(5.0);

                        if ui.button("Restart").clicked() {

                            self.current_game = Game::new();
                            self.chosen_piece = None;
                        }
                    }

                    else {

                        if ui.button("Çık").clicked() {

                            std::process::exit(0);
                        }

                        ui.add_space(5.0);

                        if ui.button("Yeniden Başlat").clicked() {

                            self.current_game = Game::new();
                            self.chosen_piece = None;
                        }
                    } 


                    ui.add_space(20.0);
                    
                    for row in 0..8 {
                        ui.horizontal( |ui| {


                            for col in 0..8 {

                                let is_light = (row + col) % 2 == 0; //if both of the conditions are satisfied the square shall be light, else dark

                                let mut color_of_the_square = if is_light {egui::Color32::LIGHT_GRAY} else {egui::Color32::DARK_GRAY};

                                    if let Some(coordinates) = self.chosen_piece {

                                        if coordinates[0] == row && coordinates[1] == col { // if we are currently on the square that has the piece the player has selected it will be shown as dark green

                                            color_of_the_square = egui::Color32::DARK_GREEN;
                                        }


                                    if Game::get_possible_moves(&mut self.current_game, position_converter(&coordinates[0], &coordinates[1]), false).contains(&position_converter(&row, &col)) { // to see which places the player could go we compare if a square is in the range of the piece by using the get_possible_moves function

                                        color_of_the_square = egui::Color32::LIGHT_GREEN;
                                    }
                                
                                }


                                let square = ui.add(
                                    egui::Button::new("") // we can create an empty button so that it becomes easier to detect if the piece on the square has been clicked on
                                    .fill(color_of_the_square)
                                    .min_size(egui::vec2(50.0, 50.0))
                                );

                                let piece_on_the_square = Game::piece_symbol(&self.current_game,  col as u32, (7 - row) as u32);
                                if piece_on_the_square.as_str() != "▢" { // if the referred position is not empty the game will show the relevant piece. Morrish's chess engine refers to characters below in order to show the color and the piece type of the piece
                            
                                    match piece_on_the_square.as_str() {
                                        "♟" => ui.put(square.rect, egui::Image::new(&visuals.white_pawn).fit_to_exact_size(square.rect.size())),
                                        "♙" => ui.put(square.rect, egui::Image::new(&visuals.black_pawn).fit_to_exact_size(square.rect.size())),
                                        "♘" => ui.put(square.rect, egui::Image::new(&visuals.black_knight).fit_to_exact_size(square.rect.size())),
                                        "♞" => ui.put(square.rect, egui::Image::new(&visuals.white_knight).fit_to_exact_size(square.rect.size())),
                                        "♝" => ui.put(square.rect, egui::Image::new(&visuals.white_bishop).fit_to_exact_size(square.rect.size())),
                                        "♗" => ui.put(square.rect, egui::Image::new(&visuals.black_bishop).fit_to_exact_size(square.rect.size())),
                                        "♜" => ui.put(square.rect, egui::Image::new(&visuals.white_rook).fit_to_exact_size(square.rect.size())),
                                        "♖" => ui.put(square.rect, egui::Image::new(&visuals.black_rook).fit_to_exact_size(square.rect.size())),
                                        "♛" => ui.put(square.rect, egui::Image::new(&visuals.white_queen).fit_to_exact_size(square.rect.size())),
                                        "♕" => ui.put(square.rect, egui::Image::new(&visuals.black_queen).fit_to_exact_size(square.rect.size())),
                                        "♚" => ui.put(square.rect, egui::Image::new(&visuals.white_king).fit_to_exact_size(square.rect.size())),
                                        "♔" => ui.put(square.rect, egui::Image::new(&visuals.black_king).fit_to_exact_size(square.rect.size())),
                                        _ => todo!()
                                    };

                                }


                                if square.clicked() {

                                    if let Some(coordinates) = self.chosen_piece {

                                        if Game::get_possible_moves(&mut self.current_game, position_converter(&coordinates[0],&coordinates[1]), false).contains(&position_converter(&row, &col)) { // if there is, of course, a piece chosen by the player and if the player clicks on another square in the range of the chosen piece the piece will move there

                                            Game::make_move(&mut self.current_game, position_converter(&coordinates[0], &coordinates[1]), position_converter(&row, &col));
                                            self.chosen_piece = None;
                                        }

                                        else {

                                            if is_this_colors_turn(&self.current_game, row, col) { // if the player clicks on another piece that belongs to his/her team the program will instead choose that piece

                                                self.chosen_piece = Some([row, col]);
                                            }
                                        }

                                    }
                                    else {

                                        if is_this_colors_turn(&self.current_game, row, col) { // if the player has not chosen a piece before and clicks on one of their pieces it will instead choose that piece

                                            self.chosen_piece = Some([row, col]);  
                                        }


                                    }
                                }


                            }


                        }

                        );
                    }

                    ui.add_space(20.0);

                    if self.language == Some("English".to_string()) {

                        if Game::get_turn(&self.current_game) == White {

                            ui.colored_label(egui::Color32::WHITE,"White's turn");
                        }

                        else {

                            ui.colored_label(egui::Color32::DARK_GRAY,"Black's turn");
                        }

                        ui.add_space(5.0);

                        let current_game_state = Game::get_game_state(&self.current_game);

                        match current_game_state {

                            InProgress => {ui.label("Current Game Status: In Progress...");} ,

                            Check => {ui.label("Current Game State: Check");},

                            GameOver => {
                                self.screen = Screen::GameIsOver;},
                        }

                    }

                    else {

                        if Game::get_turn(&self.current_game) == White {
                            
                            if self.special_mod {

                                ui.colored_label(egui::Color32::DARK_RED,"Sıra Türk Milleti'nde");
                            }

                            else {

                                ui.colored_label(egui::Color32::WHITE,"Sıra Beyazda");
                            }
                        }

                        else {

                            if self.special_mod {

                                ui.colored_label(egui::Color32::YELLOW,"Sıra AKP'de");
                            }

                            else {

                                ui.colored_label(egui::Color32::DARK_GRAY,"Sıra Siyahta");
                            }
                        }

                        ui.add_space(5.0);

                        let current_game_state = Game::get_game_state(&self.current_game);

                        match current_game_state {

                            InProgress => {ui.label("Mevcut Oyun Durumu: Devam Ediyor...");},

                            Check => {ui.label("Mevcut Oyun Durumu: Şah Çekildi!");},

                            GameOver => {self.screen = Screen::GameIsOver;}
                        }

                    }

                }

                Screen::GameIsOver => {

                    if self.language == Some("English".to_string()) {

                        ui.heading("GAME OVER");
                        
                        match Game::get_turn(&self.current_game) {

                            White => {ui.label("White Won!");},
                            Black => {ui.label("Black Won!");}
                        }

                        ui.add_space(5.0);

                        if ui.button("Quit").clicked() {

                            std::process::exit(0);
                        }

                        if ui.button("Restart").clicked() {

                            self.current_game = Game::new();
                            self.chosen_piece = None;
                            self.screen = Screen::GameScreen;
                        }

                    } 

                    else {

                        ui.heading("OYUN BİTTİ!");
                        if self.special_mod {


                            match Game::get_turn(&self.current_game) {

                                White => {ui.label("TÜRK MİLLETİ KAZANDI!");},
                                Black => {ui.label("Yine akp kazandı...");}
                            }

                        }

                        if !self.special_mod {

                            match Game::get_turn(&self.current_game) {

                                White => {ui.label("Beyaz kazandı!");},
                                Black => {ui.label("Siyah kazandı!");},
                            }
                        }

                        ui.add_space(5.0);

                        if ui.button("Çık").clicked() {

                            std::process::exit(0);
                        }

                        if ui.button("Yeniden Başlat").clicked() {

                            self.current_game = Game::new();
                            self.chosen_piece = None;
                            self.screen = Screen::GameScreen;
                        }

                    } 
                }   
            }

        });
    }
}

fn load_texture(ctx: &egui::Context, path: &str) -> TextureHandle {
    let img = image::open(path).expect("No file found with such name");
    let size = [img.width() as usize, img.height() as usize];
    let rgba = img.to_rgba8();
    let color_img = ColorImage::from_rgba_unmultiplied(size, &rgba);
    ctx.load_texture(path, color_img, Default::default())
}


fn position_converter(x: &usize, y: &usize) -> String 
{

    let row = (8 - x).to_string(); 
    
    // 97 is a's ASCII chart number, in Rust chars can also be modified as if they are numbers and there is a special chart (ASCII chart) for that. Morrish's chess engine uses an uncapitalized letter system for indicating positions
    let column: char = char::from_u32((y + 97) as u32).unwrap();
    

    let position: String = format!("{}{}", column, row);

    position
}

fn is_this_colors_turn(current_game: &Game, row: usize, col: usize)  -> bool { // instead of writing all long lines which are longer than my attention span we could just write it here and use it up there so that the code becomes easier to read

    let white_pieces = ["♟", "♞", "♝", "♜", "♛", "♚"];
    let black_pieces = ["♙", "♘", "♗", "♖", "♕", "♔",];

    if Game::get_turn(current_game) == White && white_pieces.contains(&Game::piece_symbol(current_game, col as u32, (7 - row) as u32).as_str()) {

        return true;
    }

    else if Game::get_turn(current_game) == Black && black_pieces.contains(&Game::piece_symbol(current_game, col as u32, (7 - row) as u32).as_str()) {

        return true;

    }

    false
}
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_resizable(true).with_inner_size((800.0, 1000.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        ChessGame::name(),
        options,
        Box::new(|_| Ok(Box::<ChessGame>::default())),
    )
}

