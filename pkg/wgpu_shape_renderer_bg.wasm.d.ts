/* tslint:disable */
/* eslint-disable */
export const memory: WebAssembly.Memory;
export function __wbg_orbitcamera_free(a: number): void;
export function __wbg_get_orbitcamera_distance(a: number): number;
export function __wbg_set_orbitcamera_distance(a: number, b: number): void;
export function __wbg_get_orbitcamera_pitch(a: number): number;
export function __wbg_set_orbitcamera_pitch(a: number, b: number): void;
export function __wbg_get_orbitcamera_yaw(a: number): number;
export function __wbg_set_orbitcamera_yaw(a: number, b: number): void;
export function __wbg_get_orbitcamera_target(a: number): number;
export function __wbg_set_orbitcamera_target(a: number, b: number): void;
export function __wbg_get_orbitcamera_up(a: number): number;
export function __wbg_set_orbitcamera_up(a: number, b: number): void;
export function __wbg_get_orbitcamera_bounds(a: number): number;
export function __wbg_set_orbitcamera_bounds(a: number, b: number): void;
export function __wbg_get_orbitcamera_aspect(a: number): number;
export function __wbg_set_orbitcamera_aspect(a: number, b: number): void;
export function __wbg_get_orbitcamera_fovy(a: number): number;
export function __wbg_set_orbitcamera_fovy(a: number, b: number): void;
export function __wbg_get_orbitcamera_znear(a: number): number;
export function __wbg_set_orbitcamera_znear(a: number, b: number): void;
export function __wbg_get_orbitcamera_zfar(a: number): number;
export function __wbg_set_orbitcamera_zfar(a: number, b: number): void;
export function __wbg_orbitcamerabounds_free(a: number): void;
export function __wbg_get_orbitcamerabounds_min_distance(a: number, b: number): void;
export function __wbg_set_orbitcamerabounds_min_distance(a: number, b: number, c: number): void;
export function __wbg_get_orbitcamerabounds_max_distance(a: number, b: number): void;
export function __wbg_set_orbitcamerabounds_max_distance(a: number, b: number, c: number): void;
export function __wbg_get_orbitcamerabounds_min_pitch(a: number): number;
export function __wbg_set_orbitcamerabounds_min_pitch(a: number, b: number): void;
export function __wbg_get_orbitcamerabounds_max_pitch(a: number): number;
export function __wbg_set_orbitcamerabounds_max_pitch(a: number, b: number): void;
export function __wbg_get_orbitcamerabounds_min_yaw(a: number, b: number): void;
export function __wbg_set_orbitcamerabounds_min_yaw(a: number, b: number, c: number): void;
export function __wbg_get_orbitcamerabounds_max_yaw(a: number, b: number): void;
export function __wbg_set_orbitcamerabounds_max_yaw(a: number, b: number, c: number): void;
export function __wbg_vector3_free(a: number): void;
export function __wbg_get_vector3_x(a: number): number;
export function __wbg_set_vector3_x(a: number, b: number): void;
export function __wbg_get_vector3_y(a: number): number;
export function __wbg_set_vector3_y(a: number, b: number): void;
export function __wbg_get_vector3_z(a: number): number;
export function __wbg_set_vector3_z(a: number, b: number): void;
export function __wbg_webglrenderer_free(a: number): void;
export function webglrenderer_create(a: number, b: number, c: number, d: number): number;
export function webglrenderer_update(a: number): void;
export function webglrenderer_render(a: number): void;
export function webglrenderer_resize(a: number, b: number, c: number): void;
export function webglrenderer_add_distance(a: number, b: number): void;
export function webglrenderer_add_pitch(a: number, b: number): void;
export function webglrenderer_add_yaw(a: number, b: number): void;
export function __wbg_state_free(a: number): void;
export function __wbg_get_state_width(a: number): number;
export function __wbg_set_state_width(a: number, b: number): void;
export function __wbg_get_state_height(a: number): number;
export function __wbg_set_state_height(a: number, b: number): void;
export function __wbg_get_state_camera(a: number): number;
export function __wbg_set_state_camera(a: number, b: number): void;
export function wgpu_render_bundle_set_pipeline(a: number, b: number): void;
export function wgpu_render_bundle_set_bind_group(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_set_vertex_buffer(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_set_push_constants(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_draw(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_bundle_draw_indexed(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function wgpu_render_bundle_draw_indirect(a: number, b: number, c: number): void;
export function wgpu_render_bundle_draw_indexed_indirect(a: number, b: number, c: number): void;
export function wgpu_compute_pass_set_pipeline(a: number, b: number): void;
export function wgpu_compute_pass_set_bind_group(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_compute_pass_set_push_constant(a: number, b: number, c: number, d: number): void;
export function wgpu_compute_pass_insert_debug_marker(a: number, b: number, c: number): void;
export function wgpu_compute_pass_push_debug_group(a: number, b: number, c: number): void;
export function wgpu_compute_pass_pop_debug_group(a: number): void;
export function wgpu_compute_pass_write_timestamp(a: number, b: number, c: number): void;
export function wgpu_compute_pass_begin_pipeline_statistics_query(a: number, b: number, c: number): void;
export function wgpu_compute_pass_end_pipeline_statistics_query(a: number): void;
export function wgpu_compute_pass_dispatch(a: number, b: number, c: number, d: number): void;
export function wgpu_compute_pass_dispatch_indirect(a: number, b: number, c: number): void;
export function wgpu_render_pass_set_bind_group(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_set_pipeline(a: number, b: number): void;
export function wgpu_render_pass_set_blend_constant(a: number, b: number): void;
export function wgpu_render_pass_set_vertex_buffer(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_set_scissor_rect(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_set_viewport(a: number, b: number, c: number, d: number, e: number, f: number, g: number): void;
export function wgpu_render_pass_set_stencil_reference(a: number, b: number): void;
export function wgpu_render_pass_draw(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_insert_debug_marker(a: number, b: number, c: number): void;
export function wgpu_render_pass_push_debug_group(a: number, b: number, c: number): void;
export function wgpu_render_pass_pop_debug_group(a: number): void;
export function wgpu_render_pass_draw_indexed(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function wgpu_render_pass_draw_indirect(a: number, b: number, c: number): void;
export function wgpu_render_pass_draw_indexed_indirect(a: number, b: number, c: number): void;
export function wgpu_render_pass_multi_draw_indirect(a: number, b: number, c: number, d: number): void;
export function wgpu_render_pass_multi_draw_indexed_indirect(a: number, b: number, c: number, d: number): void;
export function wgpu_render_pass_multi_draw_indirect_count(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function wgpu_render_pass_multi_draw_indexed_indirect_count(a: number, b: number, c: number, d: number, e: number, f: number): void;
export function wgpu_render_pass_set_push_constants(a: number, b: number, c: number, d: number, e: number): void;
export function wgpu_render_pass_write_timestamp(a: number, b: number, c: number): void;
export function wgpu_render_pass_begin_pipeline_statistics_query(a: number, b: number, c: number): void;
export function wgpu_render_pass_end_pipeline_statistics_query(a: number): void;
export function wgpu_render_bundle_pop_debug_group(a: number): void;
export function wgpu_render_bundle_insert_debug_marker(a: number, b: number): void;
export function wgpu_render_bundle_push_debug_group(a: number, b: number): void;
export function wgpu_render_pass_execute_bundles(a: number, b: number, c: number): void;
export function __wbindgen_malloc(a: number): number;
export function __wbindgen_realloc(a: number, b: number, c: number): number;
export function __wbindgen_add_to_stack_pointer(a: number): number;
export function __wbindgen_free(a: number, b: number): void;
export function __wbindgen_exn_store(a: number): void;
