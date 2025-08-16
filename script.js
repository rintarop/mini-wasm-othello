import init, { OthelloGame } from './pkg/mini_wasm_othello.js';

let game;
let gameInProgress = true;
let playerColor = 1; // 1: 黒（先攻）, 2: 白（後攻）

async function run() {
    // WebAssemblyモジュールを初期化
    await init();
    
    const canvas = document.getElementById('game-canvas');
    
    // Canvasサイズを動的に設定
    function resizeCanvas() {
        const container = document.getElementById('game-container');
        const maxSize = Math.min(
            container.clientWidth - 40, // パディングを考慮
            window.innerHeight * 0.6,   // 画面高さの60%
            400                         // 最大400px
        );
        const size = Math.max(280, maxSize); // 最小280px
        
        canvas.width = size;
        canvas.height = size;
        canvas.style.width = size + 'px';
        canvas.style.height = size + 'px';
        
        // ゲームが存在する場合は再描画
        if (game) {
            game.draw_board();
        }
    }
    
    // 初期サイズ設定
    resizeCanvas();
    
    // ウィンドウリサイズ時にも対応
    window.addEventListener('resize', resizeCanvas);
    
    // ゲーム開始
    function startNewGame() {
        game = new OthelloGame(canvas);
        gameInProgress = true;
        
        // AI難易度を設定
        const difficultySelect = document.getElementById('ai-difficulty');
        const difficulty = parseInt(difficultySelect.value);
        game.set_ai_difficulty(difficulty);
        
        updateGameInfo(game);
        
        // プレイヤーが後攻（白）の場合、AIが最初に手を打つ
        if (playerColor === 2) {
            setTimeout(() => {
                if (gameInProgress && game.current_player === 1) {
                    game.make_ai_move();
                    updateGameInfo(game);
                }
            }, 1000);
        }
    }
    
    startNewGame();
    
    // クリックイベントを設定
    canvas.addEventListener('click', async (event) => {
        // ゲームが進行中でない、またはAIの番の場合は何もしない
        if (!gameInProgress || game.current_player !== playerColor) {
            return;
        }
        
        // プレイヤーの手を処理
        game.handle_click(event);
        updateGameInfo(game);
        
        // ゲーム終了チェック
        if (checkGameEnd()) {
            return;
        }
        
        // AIの番になったら少し待ってからAIの手を実行
        if (game.current_player !== playerColor) {
            setTimeout(() => {
                if (gameInProgress && game.current_player !== playerColor) {
                    const aiMoved = game.make_ai_move();
                    updateGameInfo(game);
                    
                    // AIが手を打てなかった場合の処理
                    if (!aiMoved && game.get_valid_moves_count() === 0) {
                        checkGameEnd();
                    }
                }
            }, 800); // 0.8秒後にAIが手を打つ
        }
    });
    
    // リセットボタン
    document.getElementById('reset-game').addEventListener('click', () => {
        startNewGame();
    });
    
    // 先攻・後攻切り替えボタン
    document.getElementById('toggle-turn').addEventListener('click', () => {
        playerColor = playerColor === 1 ? 2 : 1;
        updateGameModeText();
        startNewGame();
    });
    
    // AI難易度変更
    document.getElementById('ai-difficulty').addEventListener('change', (event) => {
        const difficulty = parseInt(event.target.value);
        if (game) {
            game.set_ai_difficulty(difficulty);
            console.log('AI難易度を変更:', game.get_ai_difficulty_description());
        }
    });
    
    // ゲーム終了チェック
    function checkGameEnd() {
        const validMoves = game.get_valid_moves_count();
        if (validMoves === 0) {
            // 現在のプレイヤーが手を打てない場合、相手に交代
            const currentPlayer = game.current_player;
            const nextPlayer = currentPlayer === 1 ? 2 : 1;
            
            // 相手も手を打てるかチェック（簡易的にゲーム終了とする）
            gameInProgress = false;
            showGameResult();
            return true;
        }
        return false;
    }
    
    // ゲーム結果表示
    function showGameResult() {
        const score = game.get_score();
        const blackScore = score[0];
        const whiteScore = score[1];
        
        let result;
        let playerScore, aiScore;
        
        if (playerColor === 1) { // プレイヤーが黒
            playerScore = blackScore;
            aiScore = whiteScore;
        } else { // プレイヤーが白
            playerScore = whiteScore;
            aiScore = blackScore;
        }
        
        if (playerScore > aiScore) {
            result = `あなたの勝利！ (あなた: ${playerScore}, AI: ${aiScore})`;
        } else if (aiScore > playerScore) {
            result = `AIの勝利... (あなた: ${playerScore}, AI: ${aiScore})`;
        } else {
            result = `引き分け (あなた: ${playerScore}, AI: ${aiScore})`;
        }
        
        setTimeout(() => {
            alert(result);
        }, 100);
    }
    
    // ゲーム情報を更新
    function updateGameInfo(game) {
        const score = game.get_score();
        document.getElementById('black-score').textContent = score[0];
        document.getElementById('white-score').textContent = score[1];
        
        const currentPlayer = game.current_player;
        let currentPlayerText;
        
        if (currentPlayer === playerColor) {
            const colorName = playerColor === 1 ? '黒' : '白';
            currentPlayerText = `${colorName}（あなた）`;
        } else {
            const colorName = playerColor === 1 ? '白' : '黒';
            currentPlayerText = `${colorName}（AI）`;
        }
        
        document.getElementById('current-player').textContent = currentPlayerText;
        
        // 現在のプレイヤーをハイライト
        const isPlayerBlack = playerColor === 1;
        const isCurrentPlayerBlack = currentPlayer === 1;
        
        if (isPlayerBlack) {
            document.getElementById('black-info').classList.toggle('current-player', isCurrentPlayerBlack);
            document.getElementById('white-info').classList.toggle('current-player', !isCurrentPlayerBlack);
        } else {
            document.getElementById('black-info').classList.toggle('current-player', isCurrentPlayerBlack);
            document.getElementById('white-info').classList.toggle('current-player', !isCurrentPlayerBlack);
        }
    }
    
    // ゲームモードテキストを更新
    function updateGameModeText() {
        const modeText = document.getElementById('game-mode-text');
        if (playerColor === 1) {
            modeText.innerHTML = 'あなたは黒（先攻）です。クリックして石を置いてください。<br>白はAIが自動で打ちます。';
        } else {
            modeText.innerHTML = 'あなたは白（後攻）です。クリックして石を置いてください。<br>黒はAIが自動で打ちます。';
        }
    }
    
    // 初期状態を更新
    updateGameModeText();
}

// ページが読み込まれたらゲームを開始
run();
