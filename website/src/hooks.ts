import type { Transport } from '@sveltejs/kit';
// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
import JSONbig from "json-bigint";

export const transport: Transport = {
    UserDto: {
        encode: (value) => JSONbig.stringify(value),
        decode: (str) => JSONbig.parse(str)
    }
};