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
@SYSCALL uint64_t sol_remaining_compute_units();

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
