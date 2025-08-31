/* tslint:disable */
/* eslint-disable */
export function convert_date_format(date: Date, from_format: CalendarFormat, to_format: CalendarFormat): void;
export function generate_dates(start: Date, end: Date, format: CalendarFormat): void;
export enum CalendarFormat {
  BS = 0,
  AD = 1,
}
export type AppError = "LibraryUninitialized" | { DateParseError: { message: string } } | "MappingCorrupted";

export class Date {
  private constructor();
  free(): void;
  static new(year: number, month: number, day: number): Date;
  year: number;
  month: number;
  day: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly date_new: (a: number, b: number, c: number) => number;
  readonly convert_date_format: (a: number, b: number, c: number) => [number, number];
  readonly generate_dates: (a: number, b: number, c: number) => [number, number];
  readonly __wbg_date_free: (a: number, b: number) => void;
  readonly __wbg_get_date_year: (a: number) => number;
  readonly __wbg_set_date_year: (a: number, b: number) => void;
  readonly __wbg_get_date_month: (a: number) => number;
  readonly __wbg_set_date_month: (a: number, b: number) => void;
  readonly __wbg_get_date_day: (a: number) => number;
  readonly __wbg_set_date_day: (a: number, b: number) => void;
  readonly __wbindgen_export_0: WebAssembly.Table;
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
