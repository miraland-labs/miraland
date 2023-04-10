/**
 * The connection url to use when running unit tests against a live cluster
 */

export const MOCK_PORT = 9999;
export const url = process.env.TEST_LIVE
  ? 'http://localhost:8899/'
  : 'http://localhost:9999/';

export const wsUrl = process.env.TEST_LIVE
  ? 'ws://localhost:8900/'
  : 'ws://localhost:9999/';

//export const url = 'https://api.devnet-mln.miraland.top/';
//export const url = 'http://api.devnet.solana.com/';
