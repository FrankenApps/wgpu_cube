/* tslint:disable */
/* eslint-disable */
/**
* An [OrbitCamera] only permits rotation of the eye on a spherical shell around a target.
*/
export class OrbitCamera {
  free(): void;
/**
* The aspect ratio of the camera.
*/
  aspect: number;
/**
* The bounds within which the camera can be moved.
*/
  bounds: OrbitCameraBounds;
/**
* The distance of the eye from the target.
*/
  distance: number;
/**
* The field of view of the camera.
*/
  fovy: number;
/**
* The pitch angle in radians.
*/
  pitch: number;
/**
* The target of the orbit camera.
*/
  target: Vector3;
/**
* The cameras' up vector.
*/
  up: Vector3;
/**
* The yaw angle in radians.
*/
  yaw: number;
/**
* The far clipping plane of the camera.
*/
  zfar: number;
/**
* The near clipping plane of the camera.
*/
  znear: number;
}
/**
* The boundaries for how an [OrbitCamera] can be rotated.
*/
export class OrbitCameraBounds {
  free(): void;
/**
* If set it is not possible to move the camera further from the target
* than the specified amount.
*/
  max_distance?: number;
/**
* The `min_pitch` can only be between `]0, PI / 2]` due to mathematical reasons.
*/
  max_pitch: number;
/**
* If set the yaw angle will be constrained. The constrain should be in the
* interval `[0, PI]`.
*/
  max_yaw?: number;
/**
* The minimum distance between the eye and the target.
* This should not be negative. In order to ensure this the minimum distance
* should never be smaller than [f32::EPSILON].
*/
  min_distance?: number;
/**
* The `min_pitch` can only be between `]-PI / 2, 0]` due to mathematical reasons.
*/
  min_pitch: number;
/**
* If set the yaw angle will be constrained. The constrain should be in the
* interval `[-PI, 0]`.
*/
  min_yaw?: number;
}
/**
* The state holds all data about the rendering cycle and the objects that are drawn to the screen.
*/
export class State {
  free(): void;
/**
* The camera used for rendering the scene.
*/
  camera: OrbitCamera;
/**
* The height of the wgpu renderer in pixels.
*/
  height: number;
/**
* The width of the wgpu renderer in pixels.
*/
  width: number;
}
/**
* A three-dimensional vector mainly used to pass data via `wasm-bindgen`.
*/
export class Vector3 {
  free(): void;
/**
*/
  x: number;
/**
*/
  y: number;
/**
*/
  z: number;
}
/**
* Renders to a canvas using the `wgpu` **WebGL2** backend.
*/
export class WebGLRenderer {
  free(): void;
/**
* Create a new [WebGLRenderer] instance for the given canvas id.
* @param {string} canvas_id
* @param {number} width
* @param {number} height
*/
  constructor(canvas_id: string, width: number, height: number);
/**
*/
  update(): void;
/**
*/
  render(): void;
/**
* @param {number} new_width
* @param {number} new_height
*/
  resize(new_width: number, new_height: number): void;
/**
* @param {number} delta
*/
  add_distance(delta: number): void;
/**
* @param {number} delta
*/
  add_pitch(delta: number): void;
/**
* @param {number} delta
*/
  add_yaw(delta: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_vector3_free: (a: number) => void;
  readonly __wbg_get_vector3_x: (a: number) => number;
  readonly __wbg_set_vector3_x: (a: number, b: number) => void;
  readonly __wbg_get_vector3_y: (a: number) => number;
  readonly __wbg_set_vector3_y: (a: number, b: number) => void;
  readonly __wbg_get_vector3_z: (a: number) => number;
  readonly __wbg_set_vector3_z: (a: number, b: number) => void;
  readonly __wbg_orbitcamera_free: (a: number) => void;
  readonly __wbg_get_orbitcamera_distance: (a: number) => number;
  readonly __wbg_set_orbitcamera_distance: (a: number, b: number) => void;
  readonly __wbg_get_orbitcamera_pitch: (a: number) => number;
  readonly __wbg_set_orbitcamera_pitch: (a: number, b: number) => void;
  readonly __wbg_get_orbitcamera_yaw: (a: number) => number;
  readonly __wbg_set_orbitcamera_yaw: (a: number, b: number) => void;
  readonly __wbg_get_orbitcamera_target: (a: number) => number;
  readonly __wbg_set_orbitcamera_target: (a: number, b: number) => void;
  readonly __wbg_get_orbitcamera_up: (a: number) => number;
  readonly __wbg_set_orbitcamera_up: (a: number, b: number) => void;
  readonly __wbg_get_orbitcamera_bounds: (a: number) => number;
  readonly __wbg_set_orbitcamera_bounds: (a: number, b: number) => void;
  readonly __wbg_get_orbitcamera_aspect: (a: number) => number;
  readonly __wbg_set_orbitcamera_aspect: (a: number, b: number) => void;
  readonly __wbg_get_orbitcamera_fovy: (a: number) => number;
  readonly __wbg_set_orbitcamera_fovy: (a: number, b: number) => void;
  readonly __wbg_get_orbitcamera_znear: (a: number) => number;
  readonly __wbg_set_orbitcamera_znear: (a: number, b: number) => void;
  readonly __wbg_get_orbitcamera_zfar: (a: number) => number;
  readonly __wbg_set_orbitcamera_zfar: (a: number, b: number) => void;
  readonly __wbg_orbitcamerabounds_free: (a: number) => void;
  readonly __wbg_get_orbitcamerabounds_min_distance: (a: number, b: number) => void;
  readonly __wbg_set_orbitcamerabounds_min_distance: (a: number, b: number, c: number) => void;
  readonly __wbg_get_orbitcamerabounds_max_distance: (a: number, b: number) => void;
  readonly __wbg_set_orbitcamerabounds_max_distance: (a: number, b: number, c: number) => void;
  readonly __wbg_get_orbitcamerabounds_min_pitch: (a: number) => number;
  readonly __wbg_set_orbitcamerabounds_min_pitch: (a: number, b: number) => void;
  readonly __wbg_get_orbitcamerabounds_max_pitch: (a: number) => number;
  readonly __wbg_set_orbitcamerabounds_max_pitch: (a: number, b: number) => void;
  readonly __wbg_get_orbitcamerabounds_min_yaw: (a: number, b: number) => void;
  readonly __wbg_set_orbitcamerabounds_min_yaw: (a: number, b: number, c: number) => void;
  readonly __wbg_get_orbitcamerabounds_max_yaw: (a: number, b: number) => void;
  readonly __wbg_set_orbitcamerabounds_max_yaw: (a: number, b: number, c: number) => void;
  readonly __wbg_state_free: (a: number) => void;
  readonly __wbg_get_state_width: (a: number) => number;
  readonly __wbg_set_state_width: (a: number, b: number) => void;
  readonly __wbg_get_state_height: (a: number) => number;
  readonly __wbg_set_state_height: (a: number, b: number) => void;
  readonly __wbg_get_state_camera: (a: number) => number;
  readonly __wbg_set_state_camera: (a: number, b: number) => void;
  readonly __wbg_webglrenderer_free: (a: number) => void;
  readonly webglrenderer_create: (a: number, b: number, c: number, d: number) => number;
  readonly webglrenderer_update: (a: number) => void;
  readonly webglrenderer_render: (a: number) => void;
  readonly webglrenderer_resize: (a: number, b: number, c: number) => void;
  readonly webglrenderer_add_distance: (a: number, b: number) => void;
  readonly webglrenderer_add_pitch: (a: number, b: number) => void;
  readonly webglrenderer_add_yaw: (a: number, b: number) => void;
  readonly wgpu_compute_pass_set_pipeline: (a: number, b: number) => void;
  readonly wgpu_compute_pass_set_bind_group: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_compute_pass_set_push_constant: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_compute_pass_insert_debug_marker: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_push_debug_group: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_pop_debug_group: (a: number) => void;
  readonly wgpu_compute_pass_write_timestamp: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_begin_pipeline_statistics_query: (a: number, b: number, c: number) => void;
  readonly wgpu_compute_pass_end_pipeline_statistics_query: (a: number) => void;
  readonly wgpu_compute_pass_dispatch_workgroups: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_compute_pass_dispatch_workgroups_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_bundle_set_pipeline: (a: number, b: number) => void;
  readonly wgpu_render_bundle_set_bind_group: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_set_vertex_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_set_push_constants: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_draw: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_draw_indexed: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_bundle_draw_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_bundle_draw_indexed_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_set_pipeline: (a: number, b: number) => void;
  readonly wgpu_render_pass_set_bind_group: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_set_vertex_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_set_push_constants: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_draw: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_draw_indexed: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_pass_draw_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_draw_indexed_indirect: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_multi_draw_indirect: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_render_pass_multi_draw_indexed_indirect: (a: number, b: number, c: number, d: number) => void;
  readonly wgpu_render_pass_multi_draw_indirect_count: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_pass_multi_draw_indexed_indirect_count: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly wgpu_render_pass_set_blend_constant: (a: number, b: number) => void;
  readonly wgpu_render_pass_set_scissor_rect: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_pass_set_viewport: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly wgpu_render_pass_set_stencil_reference: (a: number, b: number) => void;
  readonly wgpu_render_pass_insert_debug_marker: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_push_debug_group: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_pop_debug_group: (a: number) => void;
  readonly wgpu_render_pass_write_timestamp: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_begin_pipeline_statistics_query: (a: number, b: number, c: number) => void;
  readonly wgpu_render_pass_end_pipeline_statistics_query: (a: number) => void;
  readonly wgpu_render_pass_execute_bundles: (a: number, b: number, c: number) => void;
  readonly wgpu_render_bundle_set_index_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_pop_debug_group: (a: number) => void;
  readonly wgpu_render_bundle_insert_debug_marker: (a: number, b: number) => void;
  readonly wgpu_render_pass_set_index_buffer: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly wgpu_render_bundle_push_debug_group: (a: number, b: number) => void;
  readonly __wbindgen_export_0: (a: number) => number;
  readonly __wbindgen_export_1: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_export_2: (a: number, b: number) => void;
  readonly __wbindgen_export_3: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
