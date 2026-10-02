#pragma once
// Wire-layout pins for shared_state.hpp: every field's type and fixed offset,
// checked at compile time. tests/test_shared_layout.cpp prints the manifest
// from these lists and compares it with tas_shared/shared_layout.txt, which
// the Rust pins (tas_shared/src/layout.rs) are checked against too. On any
// change: bump TAS_SHARED_VERSION and update all three.
#include <cstddef>
#include <type_traits>

#define TAS_LAYOUT_STRUCTS(X) \
    X(TasSegmentBoundary, 4, 4) \
    X(TasLogEntry, 128, 4) \
    X(TasSharedState, 1651444, 4)

#define TAS_LAYOUT_FIELDS(X) \
    X(TasSegmentBoundary, frame, uint32_t, 0) \
    X(TasLogEntry, sequence, uint32_t, 0) \
    X(TasLogEntry, severity, uint32_t, 4) \
    X(TasLogEntry, text, char[TAS_LOG_ENTRY_SIZE], 8) \
    X(TasSharedState, version, uint32_t, 0) \
    X(TasSharedState, command, uint32_t, 4) \
    X(TasSharedState, continue_from_frame, uint32_t, 8) \
    X(TasSharedState, mode, uint32_t, 12) \
    X(TasSharedState, recorded_count, uint32_t, 16) \
    X(TasSharedState, playback_pos, uint32_t, 20) \
    X(TasSharedState, player_x, float, 24) \
    X(TasSharedState, player_y, float, 28) \
    X(TasSharedState, player_z, float, 32) \
    X(TasSharedState, bb3b10_call_count, uint32_t, 36) \
    X(TasSharedState, handler_block_count, uint32_t, 40) \
    X(TasSharedState, frame_count, uint32_t, 44) \
    X(TasSharedState, bb3b10_block_count, uint32_t, 48) \
    X(TasSharedState, cycle_cave_hooked, uint32_t, 52) \
    X(TasSharedState, key_handler_cave_hooked, uint32_t, 56) \
    X(TasSharedState, observer_cave_hooked, uint32_t, 60) \
    X(TasSharedState, tick_cave_hooked, uint32_t, 64) \
    X(TasSharedState, replay_capture_hooked, uint32_t, 68) \
    X(TasSharedState, replay_ptr, uint32_t, 72) \
    X(TasSharedState, player_ptr, uint32_t, 76) \
    X(TasSharedState, playback_speed, float, 80) \
    X(TasSharedState, restart_state, uint32_t, 84) \
    X(TasSharedState, velocity_x, float, 88) \
    X(TasSharedState, velocity_y, float, 92) \
    X(TasSharedState, velocity_z, float, 96) \
    X(TasSharedState, tick_count, uint32_t, 100) \
    X(TasSharedState, segment_start_frame, uint32_t, 104) \
    X(TasSharedState, segment_count, uint32_t, 108) \
    X(TasSharedState, segment_boundaries, TasSegmentBoundary[TAS_MAX_SEGMENTS], 112) \
    X(TasSharedState, input_log, uint8_t[TAS_MAX_TICKS], 240) \
    X(TasSharedState, rec_coords, float[TAS_MAX_TICKS][3], 65776) \
    X(TasSharedState, play_coords, float[TAS_MAX_TICKS][3], 852208) \
    X(TasSharedState, log_write_seq, uint32_t, 1638640) \
    X(TasSharedState, log_ring, TasLogEntry[TAS_LOG_RING_SIZE], 1638644) \
    X(TasSharedState, cont_resume_speed, float, 1646836) \
    X(TasSharedState, game_in_game, uint32_t, 1646840) \
    X(TasSharedState, level_id, uint32_t, 1646844) \
    X(TasSharedState, race_time_cs, uint32_t, 1646848) \
    X(TasSharedState, race_start_ts, uint32_t, 1646852) \
    X(TasSharedState, test_arg4_override, uint32_t, 1646856) \
    X(TasSharedState, arg4_source, uint32_t, 1646860) \
    X(TasSharedState, cont_suppress_input, uint32_t, 1646864) \
    X(TasSharedState, level_epoch, uint32_t, 1646868) \
    X(TasSharedState, level_scan_epoch, uint32_t, 1646872) \
    X(TasSharedState, level_path, char[TAS_LEVEL_PATH_MAX], 1646876) \
    X(TasSharedState, level_path_gen, uint32_t, 1647004) \
    X(TasSharedState, level_ctx_seq, uint32_t, 1647008) \
    X(TasSharedState, arm_generation, uint32_t, 1647012) \
    X(TasSharedState, gate_index, uint32_t, 1647016) \
    X(TasSharedState, gate_align_rec, uint32_t, 1647020) \
    X(TasSharedState, capture_ok, uint32_t, 1647024) \
    X(TasSharedState, cont_splice_approved, uint32_t, 1647028) \
    X(TasSharedState, fpu_control_word, uint32_t, 1647032) \
    X(TasSharedState, renderer_id, uint32_t, 1647036) \
    X(TasSharedState, rider_character, uint32_t, 1647040) \
    X(TasSharedState, rider_stance, uint32_t, 1647044) \
    X(TasSharedState, rider_seq, uint32_t, 1647048) \
    X(TasSharedState, race_seq, uint32_t, 1647052) \
    X(TasSharedState, menu_screen, char[TAS_MENU_SCREEN_MAX], 1647056) \
    X(TasSharedState, menu_seq, uint32_t, 1647088) \
    X(TasSharedState, menu_doc, char[TAS_MENU_DOC_MAX], 1647092) \
    X(TasSharedState, menu_cmd_seq, uint32_t, 1651188) \
    X(TasSharedState, menu_cmd_kind, uint32_t, 1651192) \
    X(TasSharedState, menu_cmd_target, char[TAS_MENU_CMD_TARGET_MAX], 1651196) \
    X(TasSharedState, menu_cmd_screen, char[TAS_MENU_SCREEN_MAX], 1651260) \
    X(TasSharedState, menu_cmd_ack, uint32_t, 1651292) \
    X(TasSharedState, menu_cmd_result, uint32_t, 1651296) \
    X(TasSharedState, owner_request_seq, uint32_t, 1651300) \
    X(TasSharedState, owner_request_kind, uint32_t, 1651304) \
    X(TasSharedState, owner_request_pid, uint32_t, 1651308) \
    X(TasSharedState, owner_request_created_lo, uint32_t, 1651312) \
    X(TasSharedState, owner_request_created_hi, uint32_t, 1651316) \
    X(TasSharedState, owner_ack_seq, uint32_t, 1651320) \
    X(TasSharedState, owner_result, uint32_t, 1651324) \
    X(TasSharedState, owner_pid, uint32_t, 1651328) \
    X(TasSharedState, owner_generation, uint32_t, 1651332) \
    X(TasSharedState, game_call, uint32_t, 1651336) \
    X(TasSharedState, crash_seq, uint32_t, 1651340) \
    X(TasSharedState, crash_pid, uint32_t, 1651344) \
    X(TasSharedState, crash_code, uint32_t, 1651348) \
    X(TasSharedState, crash_address, uint32_t, 1651352) \
    X(TasSharedState, crash_game_call, uint32_t, 1651356) \
    X(TasSharedState, crash_thread_id, uint32_t, 1651360) \
    X(TasSharedState, crash_module_offset, uint32_t, 1651364) \
    X(TasSharedState, crash_module, char[TAS_CRASH_MODULE_MAX], 1651368) \
    X(TasSharedState, game_exit_clean, uint32_t, 1651400) \
    X(TasSharedState, race_clock_bits, uint32_t, 1651404) \
    X(TasSharedState, race_clock_flags, uint32_t, 1651408) \
    X(TasSharedState, race_ab_cs, uint32_t, 1651412) \
    X(TasSharedState, race_ab_bits, uint32_t, 1651416) \
    X(TasSharedState, input_model, uint32_t, 1651420) \
    X(TasSharedState, race_finish_seq, uint32_t, 1651424) \
    X(TasSharedState, race_finish_tick, uint32_t, 1651428) \
    X(TasSharedState, race_finish_mode, uint32_t, 1651432) \
    X(TasSharedState, race_finish_valid, uint32_t, 1651436) \
    X(TasSharedState, race_finish_time_bits, uint32_t, 1651440)

#define TAS_PIN_STRUCT(S, size, align) \
    static_assert(sizeof(S) == (size) && alignof(S) == (align), \
                  #S " size changed: bump TAS_SHARED_VERSION, update shared_layout.hpp and tas_shared");
#define TAS_PIN_FIELD(S, f, T, off) \
    static_assert(offsetof(S, f) == (off), \
                  #S "." #f " moved: bump TAS_SHARED_VERSION, update shared_layout.hpp and tas_shared"); \
    static_assert(std::is_same_v<std::remove_cv_t<decltype(S::f)>, T>, \
                  #S "." #f " changed type: bump TAS_SHARED_VERSION, update shared_layout.hpp and tas_shared");
TAS_LAYOUT_STRUCTS(TAS_PIN_STRUCT)
TAS_LAYOUT_FIELDS(TAS_PIN_FIELD)
#undef TAS_PIN_STRUCT
#undef TAS_PIN_FIELD
