import { useEffect, useRef } from 'react';
import {
    onImageUpdate,
    onTextUpdate,
    onHTMLUpdate,
    onRTFUpdate,
    onFilesUpdate,
    startListening,
} from "tauri-plugin-clipboard-api";
import { CopyItemsPatch } from '../api';

type PatchFunc = (patch: CopyItemsPatch) => void;

let counter = 100;

export function useListenClipboardEvents(onChange: PatchFunc) {
    const onChangeRef = useRef<PatchFunc>()
    onChangeRef.current = onChange;
    
    async function listen() {
        const unlistenText = await onTextUpdate((newText) => {
            console.log(`new text ${newText}`);
            if (onChangeRef.current) {
                onChangeRef.current({ newItem : {id: "" + counter++, displayName: "newitem" + counter}, removeIds: ["1"]})
            }
        });
        const unlistenHtml = await onHTMLUpdate((newHtml) => {
            console.log(`new html ${newHtml}`);
            if (onChangeRef.current) {
                onChangeRef.current({ newItem : {id: "" + counter++, displayName: "newitem" + counter}, removeIds: ["1"]})
            }
        });
        const unlistenImage = await onImageUpdate((b64Str) => {
            console.log(`new image ${b64Str}`);
            if (onChangeRef.current) {
                onChangeRef.current({ newItem : {id: "" + counter++, displayName: "newitem" + counter}, removeIds: ["1"]})
            }
        });
        const unlistenFiles = await onFilesUpdate((newFiles) => {
            console.log(`new files ${newFiles.join(", ")}`);
            if (onChangeRef.current) {
                onChangeRef.current({ newItem : {id: "" + counter++, displayName: "newitem" + counter}, removeIds: ["1"]})
            }
        });
        const unlistenRTF = await onRTFUpdate((newRTF) => {
            console.log(`new rtf ${newRTF}`);
            if (onChangeRef.current) {
                onChangeRef.current({ newItem : {id: "" + counter++, displayName: "newitem" + counter}, removeIds: ["1"]})
            }
        });
        const unlistenClipboard = await startListening();

        return async () => {
            unlistenText();
            unlistenHtml();
            unlistenImage();
            unlistenFiles();
            unlistenRTF();
            await unlistenClipboard();
        }
    }
    
    useEffect(() => {
        const promise = listen();
        return () => {
            promise.then(c => c());
        }
    }, []);
}
