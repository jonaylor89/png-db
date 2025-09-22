/* tslint:disable */
/* eslint-disable */
export class WebPngDatabase {
  free(): void;
  constructor(width: number, height: number, schema_json: string);
  static from_png_bytes(png_bytes: Uint8Array): WebPngDatabase;
  insert(x: number, y: number, data_json: string): void;
  query(where_clause: string): string;
  list_all(): string;
  to_png_bytes(): Uint8Array;
  get_schema(): string;
  get_dimensions(): Uint32Array;
  get_row_count(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_webpngdatabase_free: (a: number, b: number) => void;
  readonly webpngdatabase_new: (a: number, b: number, c: number, d: number) => [number, number, number];
  readonly webpngdatabase_from_png_bytes: (a: number, b: number) => [number, number, number];
  readonly webpngdatabase_insert: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly webpngdatabase_query: (a: number, b: number, c: number) => [number, number, number, number];
  readonly webpngdatabase_list_all: (a: number) => [number, number, number, number];
  readonly webpngdatabase_to_png_bytes: (a: number) => [number, number, number, number];
  readonly webpngdatabase_get_schema: (a: number) => [number, number, number, number];
  readonly webpngdatabase_get_dimensions: (a: number) => [number, number];
  readonly webpngdatabase_get_row_count: (a: number) => number;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
