import init, { fix as fix_wasm } from './wasm_bindgen.js';
import string from '../resources/discord_unicode_fix_bg.wasm';
const binary = Buffer.from(string, "base64");

export async function fix(broken) {
    await init(binary);
    return fix_wasm(broken);
}