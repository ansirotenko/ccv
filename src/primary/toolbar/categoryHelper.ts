import { CopyCategory } from "../../api";

export function getCategoriesText(categoriesNumber: number, possibleCategories: CopyCategory[]) {
    if (categoriesNumber === (1 << possibleCategories.length) - 1) {
        return 'All';
    }
    if (categoriesNumber == 0) {
        return 'None';
    }

    return toCategoriesArray(categoriesNumber, possibleCategories)
        .map((c) => c.charAt(0))
        .sort((a, b) => a.localeCompare(b))
        .join(',');
}

export function toCategoriesNumber(initialCategories: CopyCategory[], possibleCategories: CopyCategory[]) {
    let categories = 0;
    for (let i = 0; i < possibleCategories.length; i++) {
        if (initialCategories.indexOf(possibleCategories[i]) !== -1) {
            categories = categories | (1 << i);
        }
    }
    return categories;
}

export function toCategoriesArray(categoriesNumber: number, possibleCategories: CopyCategory[]) {
    let selectedCategories: CopyCategory[] = [];
    let index = 0;
    while (categoriesNumber !== 0) {
        if (categoriesNumber % 2 === 1) {
            selectedCategories.push(possibleCategories[index]);
        }
        index++;
        categoriesNumber = categoriesNumber >> 1;
    }

    return selectedCategories;
}