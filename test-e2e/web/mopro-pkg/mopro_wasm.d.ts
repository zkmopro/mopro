/* tslint:disable */
/* eslint-disable */
export function generate_plonk_proof(srs_key: Uint8Array, proving_key: Uint8Array, input: any): any;
export function verify_plonk_proof(srs_key: Uint8Array, verifying_key: Uint8Array, proof: any, public_inputs: any): any;
export function generate_hyperplonk_proof(srs_key: Uint8Array, proving_key: Uint8Array, input: any): any;
export function verify_hyperplonk_proof(srs_key: Uint8Array, verifying_key: Uint8Array, proof: Uint8Array, public_inputs: Uint8Array): any;
export function generate_gemini_proof(srs_key: Uint8Array, proving_key: Uint8Array, input: any): any;
export function verify_gemini_proof(srs_key: Uint8Array, verifying_key: Uint8Array, proof: Uint8Array, public_inputs: Uint8Array): any;
export function initThreadPool(num_threads: number): Promise<any>;
export function wbg_rayon_start_worker(receiver: number): void;
export class wbg_rayon_PoolBuilder {
  private constructor();
  free(): void;
  mainJS(): string;
  numThreads(): number;
  receiver(): number;
  build(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly generate_plonk_proof: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly verify_plonk_proof: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly generate_hyperplonk_proof: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly verify_hyperplonk_proof: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly generate_gemini_proof: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly verify_gemini_proof: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly __wbg_wbg_rayon_poolbuilder_free: (a: number, b: number) => void;
  readonly wbg_rayon_poolbuilder_mainJS: (a: number) => number;
  readonly wbg_rayon_poolbuilder_numThreads: (a: number) => number;
  readonly wbg_rayon_poolbuilder_receiver: (a: number) => number;
  readonly wbg_rayon_poolbuilder_build: (a: number) => void;
  readonly initThreadPool: (a: number) => number;
  readonly wbg_rayon_start_worker: (a: number) => void;
  readonly memory: WebAssembly.Memory;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_thread_destroy: (a?: number, b?: number, c?: number) => void;
  readonly __wbindgen_start: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput, memory?: WebAssembly.Memory, thread_stack_size?: number }} module - Passing `SyncInitInput` directly is deprecated.
* @param {WebAssembly.Memory} memory - Deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput, memory?: WebAssembly.Memory, thread_stack_size?: number } | SyncInitInput, memory?: WebAssembly.Memory): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput>, memory?: WebAssembly.Memory, thread_stack_size?: number }} module_or_path - Passing `InitInput` directly is deprecated.
* @param {WebAssembly.Memory} memory - Deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput>, memory?: WebAssembly.Memory, thread_stack_size?: number } | InitInput | Promise<InitInput>, memory?: WebAssembly.Memory): Promise<InitOutput>;
