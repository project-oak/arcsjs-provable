import * as FontsByFamilyRecipe from "./FontsByFamilyRecipe.mjs";
import * as FullDemoRecipe from "./FullDemoRecipe.mjs";
import * as LocalFontsRecipe from "./LocalFontsRecipe.mjs";
import * as PhotosByDateRecipe from "./PhotosByDateRecipe.mjs";
import * as QuillFontPickerRecipe from "./QuillFontPickerRecipe.mjs";

export const all = {
    ...FontsByFamilyRecipe,
    ...FullDemoRecipe,
    ...LocalFontsRecipe,
    ...PhotosByDateRecipe,
    ...QuillFontPickerRecipe,
};

console.log(all);
