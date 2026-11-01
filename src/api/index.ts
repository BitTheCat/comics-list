import { seriesApi } from './series';
import { publisherApi } from './publisher';
import { entryApi } from './entry';
import { configApi } from './config';

export const api = { ...seriesApi, ...publisherApi, ...entryApi, ...configApi };
