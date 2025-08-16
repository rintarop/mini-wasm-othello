use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, Element};

// パニック時のスタックトレースを有効にする
#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

// AI難易度を表現する列挙型
#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum AiDifficulty {
    Easy = 1,    // 貪欲法
    Medium = 2,  // ミニマックス 3手先読み
    Hard = 3,    // ミニマックス + アルファベータ 5手先読み
    Expert = 4,  // ミニマックス + アルファベータ 7手先読み + 改良評価関数
}

// ボード状態を表現する構造体（AI計算用）
#[derive(Clone)]
struct BoardState {
    board: [[i8; 8]; 8],
    current_player: i8,
}

impl BoardState {
    // ボード上で手を実行
    fn make_move_on_board(&mut self, row: usize, col: usize, player: i8) {
        self.board[row][col] = player;

        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        for (dr, dc) in directions.iter() {
            if self.can_flip_in_direction_board(row, col, *dr, *dc, player) {
                self.flip_in_direction_board(row, col, *dr, *dc, player);
            }
        }
    }

    // ボード上で指定した方向に石をひっくり返せるかチェック
    fn can_flip_in_direction_board(&self, row: usize, col: usize, dr: i32, dc: i32, player: i8) -> bool {
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

    // ボード上で指定した方向の石をひっくり返す
    fn flip_in_direction_board(&mut self, row: usize, col: usize, dr: i32, dc: i32, player: i8) {
        let mut r = row as i32 + dr;
        let mut c = col as i32 + dc;

        while r >= 0 && r < 8 && c >= 0 && c < 8 {
            let current_piece = self.board[r as usize][c as usize];
            
            if current_piece == player {
                break;
            } else {
                self.board[r as usize][c as usize] = player;
            }
            
            r += dr;
            c += dc;
        }
    }
}

// オセロのゲーム状態を表現する構造体
#[wasm_bindgen]
pub struct OthelloGame {
    board: [[i8; 8]; 8], // 0: 空, 1: 黒, 2: 白
    current_player: i8,  // 1: 黒, 2: 白
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    ai_difficulty: AiDifficulty, // AI難易度
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
            ai_difficulty: AiDifficulty::Medium, // デフォルトは中級
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

    // AI難易度を設定
    pub fn set_ai_difficulty(&mut self, difficulty: AiDifficulty) {
        self.ai_difficulty = difficulty;
    }

    // AI難易度を取得
    #[wasm_bindgen(getter)]
    pub fn ai_difficulty(&self) -> AiDifficulty {
        self.ai_difficulty
    }

    // AI難易度の説明を取得
    pub fn get_ai_difficulty_description(&self) -> String {
        match self.ai_difficulty {
            AiDifficulty::Easy => "初級 (貪欲法)".to_string(),
            AiDifficulty::Medium => "中級 (ミニマックス 3手先読み)".to_string(),
            AiDifficulty::Hard => "上級 (アルファベータ 5手先読み)".to_string(),
            AiDifficulty::Expert => "エキスパート (アルファベータ 7手先読み + 高度評価)".to_string(),
        }
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

    // AI の手を取得（難易度に応じた戦略）
    pub fn get_ai_move(&self) -> Vec<i32> {
        match self.ai_difficulty {
            AiDifficulty::Easy => self.get_greedy_move(),
            AiDifficulty::Medium => self.get_minimax_move(3),
            AiDifficulty::Hard => self.get_alpha_beta_move(5),
            AiDifficulty::Expert => self.get_alpha_beta_move(7),
        }
    }

    // 貪欲法（簡単）
    fn get_greedy_move(&self) -> Vec<i32> {
        let valid_moves = self.get_valid_moves();
        if valid_moves.is_empty() {
            return vec![-1, -1];
        }

        let mut best_move = valid_moves[0];
        let mut best_score = self.calculate_basic_move_score(best_move.0, best_move.1);

        for &(row, col) in &valid_moves {
            let score = self.calculate_basic_move_score(row, col);
            if score > best_score {
                best_score = score;
                best_move = (row, col);
            }
        }

        vec![best_move.0 as i32, best_move.1 as i32]
    }

    // ミニマックス法
    fn get_minimax_move(&self, depth: u8) -> Vec<i32> {
        let valid_moves = self.get_valid_moves();
        if valid_moves.is_empty() {
            return vec![-1, -1];
        }

        let mut best_move = valid_moves[0];
        let mut best_score = i32::MIN;

        for &(row, col) in &valid_moves {
            let mut game_copy = self.clone_board();
            game_copy.make_move_on_board(row, col, self.current_player);
            let score = self.minimax(&game_copy, depth - 1, false, 3 - self.current_player);
            
            if score > best_score {
                best_score = score;
                best_move = (row, col);
            }
        }

        vec![best_move.0 as i32, best_move.1 as i32]
    }

    // アルファベータ法
    fn get_alpha_beta_move(&self, depth: u8) -> Vec<i32> {
        let valid_moves = self.get_valid_moves();
        if valid_moves.is_empty() {
            return vec![-1, -1];
        }

        let mut best_move = valid_moves[0];
        let mut best_score = i32::MIN;

        for &(row, col) in &valid_moves {
            let mut game_copy = self.clone_board();
            game_copy.make_move_on_board(row, col, self.current_player);
            let score = self.alpha_beta(&game_copy, depth - 1, i32::MIN, i32::MAX, false, 3 - self.current_player);
            
            if score > best_score {
                best_score = score;
                best_move = (row, col);
            }
        }

        vec![best_move.0 as i32, best_move.1 as i32]
    }

    // ボードをクローンして新しいゲーム状態を作成
    fn clone_board(&self) -> BoardState {
        BoardState {
            board: self.board,
            current_player: self.current_player,
        }
    }

    // ミニマックス法の実装
    fn minimax(&self, board: &BoardState, depth: u8, maximizing: bool, player: i8) -> i32 {
        if depth == 0 || self.is_terminal_state(board) {
            return self.evaluate_board(board);
        }

        let valid_moves = self.get_valid_moves_for_board(board, player);
        
        if valid_moves.is_empty() {
            // パスして相手のターン
            return self.minimax(board, depth - 1, !maximizing, 3 - player);
        }

        if maximizing {
            let mut max_eval = i32::MIN;
            for &(row, col) in &valid_moves {
                let mut new_board = board.clone();
                new_board.make_move_on_board(row, col, player);
                let eval = self.minimax(&new_board, depth - 1, false, 3 - player);
                max_eval = max_eval.max(eval);
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for &(row, col) in &valid_moves {
                let mut new_board = board.clone();
                new_board.make_move_on_board(row, col, player);
                let eval = self.minimax(&new_board, depth - 1, true, 3 - player);
                min_eval = min_eval.min(eval);
            }
            min_eval
        }
    }

    // アルファベータ法の実装
    fn alpha_beta(&self, board: &BoardState, depth: u8, mut alpha: i32, mut beta: i32, maximizing: bool, player: i8) -> i32 {
        if depth == 0 || self.is_terminal_state(board) {
            return self.evaluate_board(board);
        }

        let valid_moves = self.get_valid_moves_for_board(board, player);
        
        if valid_moves.is_empty() {
            // パスして相手のターン
            return self.alpha_beta(board, depth - 1, alpha, beta, !maximizing, 3 - player);
        }

        if maximizing {
            let mut max_eval = i32::MIN;
            for &(row, col) in &valid_moves {
                let mut new_board = board.clone();
                new_board.make_move_on_board(row, col, player);
                let eval = self.alpha_beta(&new_board, depth - 1, alpha, beta, false, 3 - player);
                max_eval = max_eval.max(eval);
                alpha = alpha.max(eval);
                if beta <= alpha {
                    break; // アルファベータカット
                }
            }
            max_eval
        } else {
            let mut min_eval = i32::MAX;
            for &(row, col) in &valid_moves {
                let mut new_board = board.clone();
                new_board.make_move_on_board(row, col, player);
                let eval = self.alpha_beta(&new_board, depth - 1, alpha, beta, true, 3 - player);
                min_eval = min_eval.min(eval);
                beta = beta.min(eval);
                if beta <= alpha {
                    break; // アルファベータカット
                }
            }
            min_eval
        }
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

    // ボード状態用の有効な手を取得
    fn get_valid_moves_for_board(&self, board: &BoardState, player: i8) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        for row in 0..8 {
            for col in 0..8 {
                if self.is_valid_move_for_board(board, row, col, player) {
                    moves.push((row, col));
                }
            }
        }
        moves
    }

    // ボード状態での有効な手チェック
    fn is_valid_move_for_board(&self, board: &BoardState, row: usize, col: usize, player: i8) -> bool {
        if board.board[row][col] != 0 {
            return false;
        }

        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];

        for (dr, dc) in directions.iter() {
            if board.can_flip_in_direction_board(row, col, *dr, *dc, player) {
                return true;
            }
        }

        false
    }

    // 終端状態かどうかをチェック
    fn is_terminal_state(&self, board: &BoardState) -> bool {
        // 盤面が満杯または両プレイヤーに有効な手がない
        let mut empty_count = 0;
        for row in 0..8 {
            for col in 0..8 {
                if board.board[row][col] == 0 {
                    empty_count += 1;
                }
            }
        }
        
        if empty_count == 0 {
            return true;
        }

        // 両プレイヤーに有効な手があるかチェック
        let has_moves_1 = !self.get_valid_moves_for_board(board, 1).is_empty();
        let has_moves_2 = !self.get_valid_moves_for_board(board, 2).is_empty();
        
        !has_moves_1 && !has_moves_2
    }

    // ボード状態を評価（改良版評価関数）
    fn evaluate_board(&self, board: &BoardState) -> i32 {
        let mut score = 0;

        // 石の数による評価
        let mut my_count = 0;
        let mut opponent_count = 0;
        
        // 位置による重み付け（戦略的）
        let position_weights = [
            [120, -20,  20,   5,   5,  20, -20, 120],
            [-20, -40,  -5,  -5,  -5,  -5, -40, -20],
            [ 20,  -5,  15,   3,   3,  15,  -5,  20],
            [  5,  -5,   3,   3,   3,   3,  -5,   5],
            [  5,  -5,   3,   3,   3,   3,  -5,   5],
            [ 20,  -5,  15,   3,   3,  15,  -5,  20],
            [-20, -40,  -5,  -5,  -5,  -5, -40, -20],
            [120, -20,  20,   5,   5,  20, -20, 120],
        ];

        for row in 0..8 {
            for col in 0..8 {
                match board.board[row][col] {
                    piece if piece == self.current_player => {
                        my_count += 1;
                        score += position_weights[row][col];
                    },
                    piece if piece != 0 => {
                        opponent_count += 1;
                        score -= position_weights[row][col];
                    },
                    _ => {}
                }
            }
        }

        // 序盤・中盤・終盤で評価を調整
        let total_pieces = my_count + opponent_count;
        
        if total_pieces < 20 {
            // 序盤: 位置を重視
            score *= 2;
        } else if total_pieces < 50 {
            // 中盤: モビリティ（機動性）を追加
            let my_mobility = self.get_valid_moves_for_board(board, self.current_player).len() as i32;
            let opponent_mobility = self.get_valid_moves_for_board(board, 3 - self.current_player).len() as i32;
            score += (my_mobility - opponent_mobility) * 10;
        } else {
            // 終盤: 石の数を重視
            score += (my_count - opponent_count) * 10;
        }

        // 安定性の評価（角と辺）
        score += self.evaluate_stability(board);

        score
    }

    // 安定性を評価（角と辺の制御）
    fn evaluate_stability(&self, board: &BoardState) -> i32 {
        let mut stability_score = 0;
        
        // 角の評価
        let corners = [(0, 0), (0, 7), (7, 0), (7, 7)];
        for &(row, col) in &corners {
            match board.board[row][col] {
                piece if piece == self.current_player => stability_score += 25,
                piece if piece != 0 => stability_score -= 25,
                _ => {}
            }
        }

        // 辺の評価
        for i in 0..8 {
            // 上辺と下辺
            if board.board[0][i] == self.current_player { stability_score += 5; }
            else if board.board[0][i] != 0 { stability_score -= 5; }
            
            if board.board[7][i] == self.current_player { stability_score += 5; }
            else if board.board[7][i] != 0 { stability_score -= 5; }
            
            // 左辺と右辺
            if board.board[i][0] == self.current_player { stability_score += 5; }
            else if board.board[i][0] != 0 { stability_score -= 5; }
            
            if board.board[i][7] == self.current_player { stability_score += 5; }
            else if board.board[i][7] != 0 { stability_score -= 5; }
        }

        stability_score
    }

    // 基本的な手の評価スコア（貪欲法用）
    fn calculate_basic_move_score(&self, row: usize, col: usize) -> i32 {
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
}
