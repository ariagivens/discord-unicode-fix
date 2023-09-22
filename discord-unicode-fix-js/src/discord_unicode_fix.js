import unicode_map from './unicode_map.csv';
import GraphemeSplitter from 'grapheme-splitter';
const splitter = new GraphemeSplitter();

/**
* @param {string} broken
* @returns {string}
*/
export function fix(broken) {
    let fixed = "";

    outer: while (broken.length > 0) {
        for ( const { expected, actual } of unicode_map.values() ) {
            if (broken.startsWith(actual)) {
                fixed += expected;
                broken = broken.slice(actual.length);
                continue outer;
            }
        }
        let grapheme = splitter.iterateGraphemes(broken).next().value;
        fixed += grapheme;
        broken = broken.slice(grapheme.length);
    }

    return fixed;
}