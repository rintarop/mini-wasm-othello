/* tslint:disable */
/* eslint-disable */
export enum AiDifficulty {
  Easy = 1,
  Medium = 2,
  Hard = 3,
  Expert = 4,
}
export class OthelloGame {
  free(): void;
  constructor(canvas: HTMLCanvasElement);
  draw_board(): void;
  handle_click(event: MouseEvent): void;
  set_ai_difficulty(difficulty: AiDifficulty): void;
  get_ai_difficulty_description(): string;
  get_score(): Int32Array;
  get_valid_moves_count(): number;
  get_ai_move(): Int32Array;
  make_ai_move(): boolean;
  is_game_over(): boolean;
  readonly current_player: number;
  readonly ai_difficulty: AiDifficulty;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_othellogame_free: (a: number, b: number) => void;
  readonly othellogame_new: (a: any) => [number, number, number];
  readonly othellogame_draw_board: (a: number) => [number, number];
  readonly othellogame_handle_click: (a: number, b: any) => [number, number];
  readonly othellogame_current_player: (a: number) => number;
  readonly othellogame_set_ai_difficulty: (a: number, b: number) => void;
  readonly othellogame_ai_difficulty: (a: number) => number;
  readonly othellogame_get_ai_difficulty_description: (a: number) => [number, number];
  readonly othellogame_get_score: (a: number) => [number, number];
  readonly othellogame_get_valid_moves_count: (a: number) => number;
  readonly othellogame_get_ai_move: (a: number) => [number, number];
  readonly othellogame_make_ai_move: (a: number) => [number, number, number];
  readonly othellogame_is_game_over: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
