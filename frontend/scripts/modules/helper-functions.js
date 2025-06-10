import { blocksCategories } from "./blocks-categories.js";

export const validateBlock = function() {
    return;
}

export const addContentWithCategory = function(block, content, contents) {
    let category = contents.find( obj =>  obj?.["name"] === blocksCategories[block.category]["name"].toUpperCase()) || null;
    console.log(category);

    if (!category) {
        contents.push({
            kind: "category",
            name: blocksCategories[block.category]["name"].toUpperCase(),
            colour: blocksCategories[block.category]["colour"],
            contents: [content],
        })
    }
    else {
        category.contents.push(content);
    }
}
