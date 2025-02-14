import test from "ava";
import { fix } from "./dist/discord_unicode_fix.js";

test("strings without emojis", (t) => {
  t.is(fix(""), "");
  t.is(fix("sjdfejkfjiwef"), "sjdfejkfjiwef");
  t.is(fix("schön wűh"), "schön wűh");
});

test("okay emojis", (t) => {
  t.is(fix("\u{1F3F3}\u{200D}\u{1F308}"), "\u{1F3F3}\u{200D}\u{1F308}");
  t.is(
    fix("\u{1F3F3}\u{fe0f}\u{200D}\u{1F308}"),
    "\u{1F3F3}\u{fe0f}\u{200D}\u{1F308}",
  );
});

test("broken emojis", (t) => {
  t.is(fix("\u{1F635}\u{1F4AB}"), "\u{1F635}\u{200D}\u{1F4AB}"); // 😵💫 -> 😵‍💫
  t.is(fix("\u{1F3F3}\u{1F308}"), "\u{1F3F3}\u{FE0F}\u{200D}\u{1F308}"); // 🏳🌈 -> 🏳️‍🌈
  t.is(fix("\u{1F34B}\u{1F7E9}"), "\u{1F34B}\u{200D}\u{1F7E9}"); // 🍋🟩 -> 🍋‍🟩
});
