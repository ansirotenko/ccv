import { createContext } from 'react';
import DOMPurify from 'dompurify';
import { CopyCategory } from '../common/contract';

const highlightClass = 'highlight';

export const SearchContext = createContext<string | null>(null);

export const defaultQuery = '';
export const possibleCategories: CopyCategory[] = ['Files', 'Html', 'Image', 'Rtf', 'Text'];
export const defaultCategories: CopyCategory[] = possibleCategories;

export const copyItemSelectorClass = 'copy-item';

export function escapeSearch(search: string | null): string | null {
    if (!search) {
        return search;
    }
    return search.toLowerCase();
}

// TODO can be improved
export function htmlToHighlightedHtml(html: string | HTMLElement, search: string | null): string {
    const sanitizedHtml = DOMPurify.sanitize(html, { FORBID_ATTR: ['href'], FORBID_TAGS: ['script', 'iframe'] });

    if (!search) {
        return sanitizedHtml;
    }
    return sanitizedHtml.replace(/>([^<]+)</g, function (_, p1) {
        return `>${textToHighlightedHtml(makeUnsafeText(p1), search)}<`;
    });
}

export function textToHighlightedHtml(text: string, search: string | null): string {
    if (!search) {
        return makeSafeHtml(text);
    }
    const comparableText = text.toLowerCase();

    const ret: string[] = [];
    let start = 0;
    while (true) {
        const next = comparableText.indexOf(search, start);
        if (next === -1) {
            ret.push(makeSafeHtml(text.substring(start, text.length)));
            break;
        }

        ret.push(makeSafeHtml(text.substring(start, next)));
        ret.push(`<mark class="${highlightClass}">${makeSafeHtml(text.substring(next, next + search.length))}</mark>`);

        start = next + search.length;
        if (start >= text.length) {
            break;
        }
    }

    return ret.join('');
}

function makeSafeHtml(text: string): string {
    return text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;').replace(/'/g, '&#039;');
}

function makeUnsafeText(html: string): string {
    return html
        .replace(/&nbsp;/g, ' ')
        .replace(/&amp;/g, '&')
        .replace(/&lt;/g, '<')
        .replace(/&gt;/g, '>')
        .replace(/&quot;/g, '"')
        .replace(/&#039;/g, "'");
}
