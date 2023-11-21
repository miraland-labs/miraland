#pragma once
/**
 * @brief Miraland logging utilities
 */

#include <mln/types.h>
#include <mln/string.h>
#include <mln/entrypoint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Prints a string to stdout
 */
/* DO NOT MODIFY THIS GENERATED FILE. INSTEAD CHANGE sdk/sbf/c/inc/mln/inc/compute_units.inc AND RUN `cargo run --bin gen-headers` */
#ifndef SOL_SBFV2
uint64_t sol_remaining_compute_units();
#else
typedef uint64_t(*sol_remaining_compute_units_pointer_type)();
static uint64_t sol_remaining_compute_units() {
  sol_remaining_compute_units_pointer_type sol_remaining_compute_units_pointer = (sol_remaining_compute_units_pointer_type) 3991886574;
  return sol_remaining_compute_units_pointer();
}
#endif

#ifdef SOL_TEST
/**
 * Stub functions when building tests
 */

uint64_t sol_remaining_compute_units() {
  return UINT64_MAX;
}
#endif

#ifdef __cplusplus
}
#endif

/**@}*/
