use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, Element};

// パニック時のスタックトレースを有効にする
#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

// オセロのゲーム状態を表現する構造体
#[wasm_bindgen]
pub struct OthelloGame {
    board: [[i8; 8]; 8], // 0: 空, 1: 黒, 2: 白
    current_player: i8,  // 1: 黒, 2: 白
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl OthelloGame {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<OthelloGame, JsValue> {
        set_panic_hook();
        
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let mut game = OthelloGame {
            board: [[0; 8]; 8],
            current_player: 1, // 黒から開始
            canvas,
            context,
        };

        // 初期配置を設定
        game.board[3][3] = 2; // 白
        game.board[3][4] = 1; // 黒
        game.board[4][3] = 1; // 黒
        game.board[4][4] = 2; // 白

        game.draw_board()?;
        Ok(game)
    }

    // ボードを描画する
    pub fn draw_board(&self) -> Result<(), JsValue> {
        let canvas_width = self.canvas.width() as f64;
        let canvas_height = self.canvas.height() as f64;
        let size = canvas_width.min(canvas_height);
        let cell_size = size / 8.0;
        
        // ボードの背景を緑で塗りつぶし
        self.context.set_fill_style(&"#228B22".into());
        self.context.fill_rect(0.0, 0.0, size, size);

        // グリッドを描画
        self.context.set_stroke_style(&"#000".into());
        self.context.set_line_width(2.0);
        
        for i in 0..=8 {
            let pos = i as f64 * cell_size;
            self.context.begin_path();
            self.context.move_to(pos, 0.0);
            self.context.line_to(pos, size);
            self.context.stroke();
            
            self.context.begin_path();
            self.context.move_to(0.0, pos);
            self.context.line_to(size, pos);
            self.context.stroke();
        }

        // 石を描画
        for row in 0..8 {
            for col in 0..8 {
                if self.board[row][col] != 0 {
                    let x = col as f64 * cell_size + cell_size / 2.0;
                    let y = row as f64 * cell_size + cell_size / 2.0;
                    let radius = (cell_size * 0.4).min(25.0); // セルサイズに応じて調整、最大25px
                    
                    self.context.begin_path();
                    self.context.arc(x, y, radius, 0.0, 2.0 * std::f64::consts::PI)?;
                    
                    if self.board[row][col] == 1 {
                        self.context.set_fill_style(&"#000".into());
                    } else {
                        self.context.set_fill_style(&"#FFF".into());
                    }
                    self.context.fill();
                    
                    self.context.set_stroke_style(&"#000".into());
                    self.context.stroke();
                }
            }
        }

        Ok(())
    }

    // クリック処理
    pub fn handle_click(&mut self, event: MouseEvent) -> Result<(), JsValue> {
        let rect = self.canvas.dyn_ref::<Element>().unwrap().get_bounding_client_rect();
        let x = event.client_x() as f64 - rect.left();
        let y = event.client_y() as f64 - rect.top();
        
        let canvas_width = self.canvas.width() as f64;
        let canvas_height = self.canvas.height() as f64;
        let size = canvas_width.min(canvas_height);
        let cell_size = size / 8.0;
        
        let col = (x / cell_size) as usize;
        let row = (y / cell_size) as usize;
        
        if row < 8 && col < 8 && self.is_valid_move(row, col) {
            self.make_move(row, col);
            self.current_player = if self.current_player == 1 { 2 } else { 1 };
            self.draw_board()?;
        }
        
        Ok(())
    }

    // 有効な手かどうかをチェック
    fn is_valid_move(&self, row: usize, col: usize) -> bool {
        if self.board[row][col] != 0 {
            return false;
        }

        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        for (dr, dc) in directions.iter() {
            if self.can_flip_in_direction(row, col, *dr, *dc) {
                return true;
            }
        }

        false
    }

    // 指定した方向に石をひっくり返せるかチェック
    fn can_flip_in_direction(&self, row: usize, col: usize, dr: i32, dc: i32) -> bool {
        let mut r = row as i32 + dr;
        let mut c = col as i32 + dc;
        let mut found_opponent = false;

        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            let current_piece = self.board[r as usize][c as usize];
            
            if current_piece == 0 {
                return false;
            } else if current_piece == self.current_player {
                return found_opponent;
            } else {
                found_opponent = true;
            }
            
            r += dr;
            c += dc;
        }

        false
    }

    // 手を実行
    fn make_move(&mut self, row: usize, col: usize) {
        self.board[row][col] = self.current_player;

        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        for (dr, dc) in directions.iter() {
            if self.can_flip_in_direction(row, col, *dr, *dc) {
                self.flip_in_direction(row, col, *dr, *dc);
            }
        }
    }

    // 指定した方向の石をひっくり返す
    fn flip_in_direction(&mut self, row: usize, col: usize, dr: i32, dc: i32) {
        let mut r = row as i32 + dr;
        let mut c = col as i32 + dc;

        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            let current_piece = self.board[r as usize][c as usize];
            
            if current_piece == self.current_player {
                break;
            } else {
                self.board[r as usize][c as usize] = self.current_player;
            }
            
            r += dr;
            c += dc;
        }
    }

    // 現在のプレイヤーを取得
    #[wasm_bindgen(getter)]
    pub fn current_player(&self) -> i8 {
        self.current_player
    }

    // スコアを取得
    pub fn get_score(&self) -> Vec<i32> {
        let mut black_count = 0;
        let mut white_count = 0;
        
        for row in 0..8 {
            for col in 0..8 {
                match self.board[row][col] {
                    1 => black_count += 1,
                    2 => white_count += 1,
                    _ => {}
                }
            }
        }
        
        vec![black_count, white_count]
    }

    // 有効な手の一覧を取得（JavaScriptから呼び出し可能な形式）
    pub fn get_valid_moves_count(&self) -> usize {
        let mut count = 0;
        for row in 0..8 {
            for col in 0..8 {
                if self.is_valid_move(row, col) {
                    count += 1;
                }
            }
        }
        count
    }

    // 有効な手の一覧を取得（内部使用）
    fn get_valid_moves(&self) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        for row in 0..8 {
            for col in 0..8 {
                if self.is_valid_move(row, col) {
                    moves.push((row, col));
                }
            }
        }
        moves
    }

    // シンプルなAI（貪欲法）- 結果を別々の値で返す
    pub fn get_ai_move(&self) -> Vec<i32> {
        let valid_moves = self.get_valid_moves();
        if valid_moves.is_empty() {
            return vec![-1, -1]; // 無効な手を示す
        }

        let mut best_move = valid_moves[0];
        let mut best_score = 0;

        for &(row, col) in &valid_moves {
            let score = self.calculate_move_score(row, col);
            if score > best_score {
                best_score = score;
                best_move = (row, col);
            }
        }

        vec![best_move.0 as i32, best_move.1 as i32]
    }

    // 手の評価スコアを計算（改良版）
    fn calculate_move_score(&self, row: usize, col: usize) -> i32 {
        let mut score = 0;

        // 位置による重み付け（戦略的）
        let position_weights = [
            [100, -20,  10,   5,   5,  10, -20, 100],
            [-20, -50,  -2,  -2,  -2,  -2, -50, -20],
            [ 10,  -2,  -1,  -1,  -1,  -1,  -2,  10],
            [  5,  -2,  -1,  -1,  -1,  -1,  -2,   5],
            [  5,  -2,  -1,  -1,  -1,  -1,  -2,   5],
            [ 10,  -2,  -1,  -1,  -1,  -1,  -2,  10],
            [-20, -50,  -2,  -2,  -2,  -2, -50, -20],
            [100, -20,  10,   5,   5,  10, -20, 100],
        ];

        score += position_weights[row][col];

        // ひっくり返せる石の数を計算（少し重み付け）
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        for (dr, dc) in directions.iter() {
            score += self.count_flips_in_direction(row, col, *dr, *dc) * 2;
        }

        score
    }

    // 特定方向でひっくり返せる石の数を数える
    fn count_flips_in_direction(&self, row: usize, col: usize, dr: i32, dc: i32) -> i32 {
        let mut r = row as i32 + dr;
        let mut c = col as i32 + dc;
        let mut count = 0;

        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            let current_piece = self.board[r as usize][c as usize];
            
            if current_piece == 0 {
                return 0; // 空のマスに到達
            } else if current_piece == self.current_player {
                return count; // 自分の石に到達
            } else {
                count += 1; // 相手の石をカウント
            }
            
            r += dr;
            c += dc;
        }

        0 // 境界に到達
    }

    // AIの手を実行
    pub fn make_ai_move(&mut self) -> Result<bool, JsValue> {
        let ai_move = self.get_ai_move();
        if ai_move[0] >= 0 && ai_move[1] >= 0 {
            let row = ai_move[0] as usize;
            let col = ai_move[1] as usize;
            self.make_move(row, col);
            self.current_player = if self.current_player == 1 { 2 } else { 1 };
            self.draw_board()?;
            Ok(true)
        } else {
            Ok(false) // 有効な手がない
        }
    }

    // ゲームが終了しているかチェック
    pub fn is_game_over(&self) -> bool {
        // 盤面が満杯かチェック
        for row in 0..8 {
            for col in 0..8 {
                if self.board[row][col] == 0 {
                    // 空のマスがあるので、有効な手があるかチェック
                    if self.has_valid_moves_for_player(1) || self.has_valid_moves_for_player(2) {
                        return false;
                    }
                }
            }
        }
        true
    }

    // 指定したプレイヤーに有効な手があるかチェック
    fn has_valid_moves_for_player(&self, player: i8) -> bool {
        let original_player = self.current_player;
        
        for row in 0..8 {
            for col in 0..8 {
                if self.board[row][col] == 0 {
                    // 一時的にプレイヤーを変更して有効な手をチェック
                    if self.is_valid_move_for_player(row, col, player) {
                        return true;
                    }
                }
            }
        }
        false
    }

    // 特定のプレイヤーに対して有効な手かどうかをチェック
    fn is_valid_move_for_player(&self, row: usize, col: usize, player: i8) -> bool {
        if self.board[row][col] != 0 {
            return false;
        }

        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        for (dr, dc) in directions.iter() {
            if self.can_flip_in_direction_for_player(row, col, *dr, *dc, player) {
                return true;
            }
        }

        false
    }

    // 特定のプレイヤーに対して指定した方向に石をひっくり返せるかチェック
    fn can_flip_in_direction_for_player(&self, row: usize, col: usize, dr: i32, dc: i32, player: i8) -> bool {
        let mut r = row as i32 + dr;
        let mut c = col as i32 + dc;
        let mut found_opponent = false;

        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            let current_piece = self.board[r as usize][c as usize];
            
            if current_piece == 0 {
                return false;
            } else if current_piece == player {
                return found_opponent;
            } else {
                found_opponent = true;
            }
            
            r += dr;
            c += dc;
        }

        false
    }
}
