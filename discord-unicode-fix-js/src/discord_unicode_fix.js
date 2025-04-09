import unicode_map from "./unicode_map.csv";
const segmenter = new Intl.Segmenter(undefined, { granularity: "grapheme" });

/**
 * @param {string} broken
 * @returns {string}
 */
export function fix(broken) {
  let fixed = "";

  outer: while (broken.length > 0) {
    for (const { expected, actual } of unicode_map.values()) {
      if (broken.startsWith(actual)) {
        fixed += expected;
        broken = broken.slice(actual.length);
        continue outer;
      }
    }
    let grapheme = segmenter.segment(broken)[Symbol.iterator]().next()
      .value.segment;
    fixed += grapheme;
    broken = broken.slice(grapheme.length);
  }

  return fixed;
}
