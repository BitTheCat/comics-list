import { seriesApi } from './series';
import { publisherApi } from './publisher';
import { entryApi } from './entry';

export const api = { ...seriesApi, ...publisherApi, ...entryApi };
