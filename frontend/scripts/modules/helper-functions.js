import { blocksCategories } from "./blocks-definitions/blocks-categories.js";

export const validateBlock = function() {
    return;
}

export const addContentWithCategory = function(block, content, contents) {
    let category = contents.find( obj =>  obj?.["name"] === block.category["name"].toUpperCase()) || null;

    if (!category) {
        contents.push({
            kind: "category",
            name: block.category["name"].toUpperCase(),
            colour: block.category["colour"],
            contents: [content],
        })
    }
    else {
        category.contents.push(content);
    }
}
